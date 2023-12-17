mod cli;
mod dir_iter;
mod error;
mod file;
mod format;
mod progress;
mod report;
mod snapper;
mod snapshot;
mod snapshot_1;
mod snapshot_2;
mod stats;

use crate::cli::{handle_ctrl_c, Cli};
use crate::dir_iter::DirIterator;
use crate::error::Error;
use crate::snapper::{Config, Snapper};
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use clap::Parser;
use std::process;
use std::thread::available_parallelism;

fn run() -> Result<(), Error> {
    let args = Cli::parse();
    let ctrl_c = handle_ctrl_c();

    let config = Config {
        worker: available_parallelism().unwrap().get(),
        chunk_size: 1024 * 1024 * 10, // ~10MB
    };

    let snap1 = {
        let snapper1 = Snapper::new("Snap 1", config, ctrl_c.clone());
        let snap1 = Snapshot1::new();
        let dir_it1 = DirIterator::scan(args.snap1()?, config.chunk_size)?;
        snapper1.process(dir_it1, snap1)?
    };

    let result = {
        let snapper2 = Snapper::new("Snap 2", config, ctrl_c.clone());
        let snap2 = Snapshot2::new(snap1);
        let dir_it2 = DirIterator::scan(args.snap2()?, config.chunk_size)?;
        snapper2.process(dir_it2, snap2)?.conclude()
    };

    println!("{}", result.serialize());
    return Ok(());
}

fn main() {
    let status = run();
    if !status.is_ok() {
        println!("{}", status.unwrap_err());
        process::exit(1);
    }
}
