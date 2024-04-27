extern crate core;

use ityfuzz::logger;

#[cfg(feature = "sui_support")]
pub mod r#move;

use clap::{Parser, Subcommand};
// use evm::{evm_main, EvmArgs};

use ityfuzz::evm::{evm_main, EvmArgs};

#[cfg(feature = "sui_support")]
use crate::r#move::{move_main, MoveArgs};

pub fn init_sentry() {
    let _guard = sentry::init((
        "https://96f3517bd77346ea835d28f956a84b9d@o4504503751344128.ingest.sentry.io/4504503752523776",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
enum Commands {
    Evm(EvmArgs),
    #[cfg(feature = "sui_support")]
    Move(MoveArgs),
}

fn main() {
    init_sentry();
    logger::init();

    let args = Cli::parse();
    match args.command {
        Commands::Evm(args) => {
            evm_main(args);
        }
        #[cfg(feature = "sui_support")]
        Commands::Move(args) => {
            move_main(args);
        }
    }
}
