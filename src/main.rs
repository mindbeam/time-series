use std::path::{Path, PathBuf};
use store::Store;
use structopt::StructOpt;

mod cmd;
mod module;
mod store;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "mbcli")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Run,
    Export {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Import {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    env_logger::init();

    let store = Store::open(Path::new(".")).unwrap();

    match opt.cmd {
        Command::Run => cmd::run::run(store)?,
        Command::Export { file } => cmd::export::export(store, file)?,
        Command::Import { file } => cmd::import::import(store, file)?,
    };

    Ok(())
}

// TODO:
// [ ] JSON Parsing
// [ ] Reparse historical data (bincode?)
// [ ] Low-road plotters implementation (local png export?)
// [ ] Evaluate Arrow
// [ ] Auto-reconnect
// [ ] Run on Raspberry Pi
// [ ] Data export protocol
// [ ] React + Observable + WASM + Websocket
// [ ] Mindbase integration
