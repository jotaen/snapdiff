mod checksum;
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
use crate::snapper::Snapper;
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use clap::Parser;
use std::process;

fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let ctrl_c = handle_ctrl_c();

    let snap1 = {
        let snapper1 = Snapper::new("Snap 1", cli.num_workers(), ctrl_c.clone());
        let snap1 = Snapshot1::new();
        let dir_it1 = DirIterator::scan(cli.snap1()?)?;
        snapper1.process(dir_it1, snap1)?
    };

    let report = {
        let snapper2 = Snapper::new("Snap 2", cli.num_workers(), ctrl_c.clone());
        let snap2 = Snapshot2::new(snap1);
        let dir_it2 = DirIterator::scan(cli.snap2()?)?;
        snapper2.process(dir_it2, snap2)?.conclude()
    };

    println!("{}", report.summary());
    return Ok(());
}

fn main() {
    let status = run();
    if !status.is_ok() {
        println!("{}", status.unwrap_err());
        process::exit(1);
    }
}
