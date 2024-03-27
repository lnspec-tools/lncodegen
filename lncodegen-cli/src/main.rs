use std::fs;

use clap::Parser;
use radicle_term as term;

mod cmd;
mod gen;

use crate::cmd::cmd_args::{Cli, Commands};
use crate::gen::csv_method::CSVCodeGen;
use crate::gen::CodeGenMethod;

fn dispach_cmd(args: &Cli) -> anyhow::Result<()> {
    match &args.command {
        Commands::Generate {
            bolt,
            to: result_path,
        } => {
            fs::metadata(bolt)?;
            let file_content = fs::read_to_string(bolt)?;
            let lang = args.lang.clone().unwrap();
            let generator = CSVCodeGen {
                lang: lang.to_owned(),
            };
            let result = generator.generate(file_content.as_str())?;
            term::success!(
                "Generate {lang} to {}",
                result_path.as_os_str().to_str().unwrap()
            );
            fs::write(result_path, result)?;
            Ok(())
        }
        Commands::Decode { from } => {
            use fundamentals::bolt::bolt1::Init;
            use fundamentals::core::FromWire;
            use std::io::BufReader;

            let bytes = hex::decode(from)?;
            let mut reader = BufReader::new(bytes.as_slice());
            let init = Init::from_wire(&mut reader)?;
            term::success!("{:#?}", init);
            Ok(())
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Cli::parse();
    match dispach_cmd(&args) {
        Ok(_) => {}
        Err(err) => term::error(format!("{err}")),
    }
    Ok(())
}
