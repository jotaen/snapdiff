mod checksum;
mod cli;
mod dir_iter;
mod error;
mod file;
mod format;
mod printer;
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
use crate::printer::{SNP1, SNP2};
use crate::progress::Progress;
use crate::snapper::Snapper;
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use clap::Parser;
use std::process;

fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let ctrl_c = handle_ctrl_c();

    let (num_workers1, num_workers2) = cli.num_workers();

    let snap1 = {
        let mut progress1 = Progress::new(cli.printer(), SNP1, None);
        let dir_it1 = DirIterator::scan(num_workers1, cli.snap1()?, &mut progress1)?;
        let snapper1 = Snapper::new(num_workers1, ctrl_c.clone());
        let snap1 = Snapshot1::new();
        snapper1.process(dir_it1, snap1, progress1)?
    };

    let report = {
        let mut progress2 = Progress::new(cli.printer(), SNP2, Some(snap1.total().files_count()));
        let dir_it2 = DirIterator::scan(num_workers2, cli.snap2()?, &mut progress2)?;
        let snapper2 = Snapper::new(num_workers2, ctrl_c.clone());
        let snap2 = Snapshot2::new(snap1);
        snapper2.process(dir_it2, snap2, progress2)?.conclude()
    };

    println!("{}", report.summary(cli.printer()));
    return Ok(());
}

fn main() {
    let status = run();
    if !status.is_ok() {
        println!("{}", status.unwrap_err());
        process::exit(1);
    }
}
