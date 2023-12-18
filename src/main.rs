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

use crate::cli::Cli;
use crate::dir_iter::DirIterator;
use crate::error::Error;
use crate::printer::{SNP1, SNP2};
use crate::progress::Progress;
use crate::snapper::Snapper;
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use std::process;

fn run() -> Result<(), Error> {
    let cli = Cli::new_from_env()?;

    let snap1 = {
        let mut progress1 = Progress::new(cli.terminal_printer, SNP1, None);
        let dir_it1 = DirIterator::scan(cli.workers1, &cli.snap1, &mut progress1)?;
        let snapper1 = Snapper::new(cli.workers1, cli.ctrl_c.clone());
        let snap1 = Snapshot1::new();
        snapper1.process(dir_it1, snap1, progress1)?
    };

    let report = {
        let mut progress2 = Progress::new(
            cli.terminal_printer,
            SNP2,
            Some(snap1.total().files_count()),
        );
        let dir_it2 = DirIterator::scan(cli.workers2, &cli.snap2, &mut progress2)?;
        let snapper2 = Snapper::new(cli.workers2, cli.ctrl_c.clone());
        let snap2 = Snapshot2::new(snap1);
        snapper2.process(dir_it2, snap2, progress2)?.conclude()
    };

    report.summary(cli.terminal_printer);
    report.detailed_list(cli.file_printer);
    return Ok(());
}

fn main() {
    let status = run();
    if !status.is_ok() {
        println!("{}", status.unwrap_err());
        process::exit(1);
    }
}
