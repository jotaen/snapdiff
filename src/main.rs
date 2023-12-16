mod file;
mod stats;
mod snapshot_1;
mod dir_iter;
mod fs_scan;
mod snapshot_2;
mod result;
mod snapshot;
mod progress;
mod format;

use std::{env, path};
use std::process;
use std::thread::available_parallelism;
use crate::fs_scan::{Config, FsScan};
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong args arity!");
        process::exit(1);
    }

    let root1 = path::Path::new(&args[1]);
    let root2 = path::Path::new(&args[2]);
    let fs_scan = FsScan::new(Config{
        worker: available_parallelism().unwrap().get(),
        chunk_size: 1024 * 1024 * 10, // ~10MB
    });

    let snap1 = {
        let snap1 = Snapshot1::new();
        fs_scan.traverse("snap 1", root1, snap1)
    };

    let result = {
        let snap2 = Snapshot2::new(snap1);
        fs_scan.traverse("snap 2", root2, snap2).conclude()
    };

    println!("{}", result.serialize());
}
