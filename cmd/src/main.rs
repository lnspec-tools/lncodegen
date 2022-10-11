#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
use clap::Parser;
use log::{debug, info, trace};
use rio_rt::runitime as rio;

mod cmd;
mod gen;

use cmd::cmd_args::{Cli, Commands};
use gen::{csv_method::CSVCodeGen, CodeGenMethod};

async fn dispach_cmd(args: &Cli) {
    match &args.command {
        Commands::Gen {
            bolt,
            to: result_path,
        } => {
            info!(
                "generate {} of bolt {} in {} ",
                args.lang,
                bolt,
                result_path.as_os_str().to_str().unwrap()
            );
            // TODO: read bolt file
            let generator = CSVCodeGen {
                lang: args.lang.to_owned(),
            };
            let result = generator.generate("").await;
            trace!("result from generator {}", result.unwrap());
            // TODO: write code file
        }
    }
}

fn main() {
    env_logger::init();
    debug!("running lncodegen.rs");
    let args = Cli::parse();
    rio::block_on(async move {
        dispach_cmd(&args).await;
    });
    rio::wait();
}
