mod dir_iter;
mod file;
mod format;
mod progress;
mod result;
mod snapper;
mod snapshot;
mod snapshot_1;
mod snapshot_2;
mod stats;

use crate::snapper::{Config, Snapper};
use crate::snapshot_1::Snapshot1;
use crate::snapshot_2::Snapshot2;
use std::process;
use std::thread::available_parallelism;
use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong args arity!");
        process::exit(1);
    }

    let root1 = path::Path::new(&args[1]);
    let root2 = path::Path::new(&args[2]);
    let snapper = Snapper::new(Config {
        worker: available_parallelism().unwrap().get(),
        chunk_size: 1024 * 1024 * 10, // ~10MB
    });

    let snap1 = {
        let snap1 = Snapshot1::new();
        snapper.process("Snap 1", root1, snap1)
    };

    let result = {
        let snap2 = Snapshot2::new(snap1);
        snapper.process("Snap 2", root2, snap2).conclude()
    };

    println!("{}", result.serialize());
}
