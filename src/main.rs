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

use crate::cli::Cli;
use crate::error::Error;
use crate::snapper::{Config, Snapper};
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use clap::Parser;
use std::process;
use std::thread::available_parallelism;

fn run() -> Result<(), Error> {
    let args = Cli::parse();

    let root1 = args.snap1()?;
    let root2 = args.snap2()?;

    let snapper = Snapper::new(Config {
        worker: available_parallelism().unwrap().get(),
        chunk_size: 1024 * 1024 * 10, // ~10MB
    });

    let snap1 = {
        let snap1 = Snapshot1::new();
        snapper.process("Snap 1", root1, snap1)?
    };

    let result = {
        let snap2 = Snapshot2::new(snap1);
        snapper.process("Snap 2", root2, snap2)?.conclude()
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
