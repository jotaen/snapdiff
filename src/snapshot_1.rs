use crate::snapshot::{FilesByChecksums, Snapshot};
use crate::snapshot_1::Comparison::{Identical, Modified};
use crate::{file, stats};
use file::File;
use stats::Stats;
use std::collections::HashMap;
use std::path;

#[derive(Debug)]
pub struct Snapshot1 {
    files_by_path: HashMap<path::PathBuf, File>,
    total: Stats,
}

impl Snapshot for Snapshot1 {
    fn add(&mut self, f1: File) {
        self.total.record(&f1);
        if self.files_by_path.contains_key(&f1.path) {
            panic!("Added duplicate file")
        }
        self.files_by_path.insert(f1.path.clone(), f1);
    }
}

pub enum Comparison {
    Identical,
    Modified,
}

impl Snapshot1 {
    pub fn new() -> Snapshot1 {
        return Snapshot1 {
            files_by_path: HashMap::new(),
            total: Stats::new(),
        };
    }

    // Processes a file from snapshot 2, and checks whether there is
    // a matching file in snapshot 1. If so, the file is removed from
    // the internal lookup table.
    pub fn digest(&mut self, f2: &File) -> Option<(Comparison, File)> {
        return self.files_by_path.remove(&f2.path).map(|f1| {
            if f1.equals(&f2) {
                (Identical, f1)
            } else {
                (Modified, f1)
            }
        });
    }

    pub fn total(&self) -> &Stats {
        return &self.total;
    }

    // Processes the remainder of the internal lookup table, and puts
    // all remaining files into a lookup table “by checksums”.
    pub fn conclude(&mut self) -> (Stats, FilesByChecksums) {
        let mut files_by_hash: FilesByChecksums = FilesByChecksums::new();

        for (_, f) in self.files_by_path.drain() {
            files_by_hash.add(f);
        }
        return (
            std::mem::replace(&mut self.total, Stats::new()),
            files_by_hash,
        );
    }
}
