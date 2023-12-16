use std::collections::{HashMap};
use std::path;
use file::File;
use crate::{file, result, stats};
use crate::file::{ContentsHash};
use crate::snapshot::Snapshot;

#[derive(Debug)]
pub struct Snapshot1 {
    files_by_path: HashMap<path::PathBuf, File>,
    pub result: result::Result,
}

impl Snapshot for Snapshot1 {
    fn add(&mut self, f1: File) {
        self.result.total_snap_1.record_file(&f1);
        if self.files_by_path.contains_key(&f1.path) {
            panic!("Added duplicate file")
        }
        self.files_by_path.insert(f1.path.clone(), f1);
    }

    fn total(&self) -> stats::Stats {
        return self.result.total_snap_1;
    }
}

impl Snapshot1 {
    pub fn new() -> Snapshot1 {
        return Snapshot1 {
            result: result::Result::new(),
            files_by_path: HashMap::new(),
        };
    }

    pub fn digest(&mut self, f2: &File) -> bool {
        self.result.total_snap_2.record_file(&f2);
        let was_digested = self.files_by_path.get(&f2.path)
            .map(|f1| {
                if f2.checksum == f1.checksum {
                    self.result.identical.record_file(f2);
                    true
                } else {
                    self.result.modified.record(f2, f1);
                    true
                }
            }).unwrap_or(false);
        if was_digested != false {
            self.files_by_path.remove(&f2.path);
        }
        return was_digested;
    }

    pub fn conclude(&mut self) -> (result::Result, HashMap<ContentsHash, Vec<File>>) {
        let mut files_by_hash: HashMap<ContentsHash, Vec<File>> = HashMap::new();

        for (_, f) in self.files_by_path.drain() {
            if !files_by_hash.contains_key(&f.checksum) {
                files_by_hash.insert(f.checksum, vec![]);
            }
            files_by_hash.get_mut(&f.checksum).unwrap().push(f);
        }
        return (self.result, files_by_hash);
    }
}
