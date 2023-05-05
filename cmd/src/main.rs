#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
use std::fmt::Display;

use clap::Parser;
use log::{debug, error};
use rio_rt::runitime as rio;
use radicle_term as term;

mod cmd;
mod gen;

use cmd::cmd_args::{Cli, Commands};
use gen::{csv_method::CSVCodeGen, CodeGenMethod};

#[derive(Debug, Clone)]
struct DispachError {
    cause: String,
}

impl DispachError {
    pub fn new(cause: &str) -> Self {
        DispachError {
            cause: cause.to_owned(),
        }
    }
}

impl Display for DispachError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl From<std::io::Error> for DispachError {
    fn from(value: std::io::Error) -> Self {
        DispachError {
            cause: format!("{value}"),
        }
    }
}

impl From<crate::gen::CodeGenError> for DispachError {
    fn from(value: crate::gen::CodeGenError) -> Self {
        DispachError {
            cause: value.to_string(),
        }
    }
}

async fn dispach_cmd(args: &Cli) -> Result<(), DispachError> {
    match &args.command {
        Commands::Generate {
            bolt,
            to: result_path,
        } => {
            if let Err(err) = async_std::fs::metadata(bolt).await {
                return Err(DispachError::new(format!("error: {err}").as_str()));
            }
            let file_content = async_std::fs::read_to_string(bolt).await?;
            let lang = args.lang.clone().unwrap();
            let generator = CSVCodeGen {
                lang: lang.to_owned(),
            };
            let result = generator.generate(file_content.as_str()).await?;
            term::success!(
                "Generate {lang} to {}",
                result_path.as_os_str().to_str().unwrap()
            );
            if let Err(err) = async_std::fs::write(result_path, result).await {
                return Err(DispachError::from(err));
            }
            Ok(())
        }
        Commands::Decode { from } => {
            use fundamentals::bolt::bolt1::Init;
            use fundamentals::core::FromWire;
            use std::io::BufReader;
            let bytes = hex::decode(from).expect("error while decoding hex to bytes");
            let mut reader = BufReader::new(bytes.as_slice());
            let init = Init::from_wire(&mut reader);
            term::success!("{:?}", init);
            Ok(())
        }
    }
}

fn main() {
    env_logger::init();
    debug!("running lncodegen.rs");
    let args = Cli::parse();
    rio::block_on(async move {
        if let Err(err) = dispach_cmd(&args).await {
            error!("runtime error: {err}");
            panic!("runtime error: {err}");
        }
    });
    rio::wait();
}
