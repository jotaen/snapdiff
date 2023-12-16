use std::collections::{HashMap};
use file::File;
use snapshot_1::Snapshot1;
use crate::{file, result, snapshot_1, stats};
use crate::file::ContentsHash;
use crate::snapshot::Snapshot;

#[derive(Debug)]
pub struct Snapshot2 {
    snap_1: Snapshot1,
    snap_2_remainder: HashMap<ContentsHash, Vec<File>>,
}

impl Snapshot for Snapshot2 {
    fn add(&mut self, f2: File) {
        let res = self.snap_1.digest(&f2);
        if !res {
            if !self.snap_2_remainder.contains_key(&f2.checksum) {
                self.snap_2_remainder.insert(f2.checksum, vec![]);
            }
            self.snap_2_remainder.get_mut(&f2.checksum).unwrap().push(f2);
        }
    }

    fn total(&self) -> stats::Stats {
        return self.snap_1.result.total_snap_2;
    }
}

impl Snapshot2 {
    pub fn new(source_snap: Snapshot1) -> Snapshot2 {
        return Snapshot2 {
            snap_1: source_snap,
            snap_2_remainder: HashMap::new(),
        };
    }

    pub fn conclude(&mut self) -> result::Result {
        let (mut result, mut snap_1_remainder) = self.snap_1.conclude();
        self.snap_2_remainder.retain(|checksum, fs| {
            if !snap_1_remainder.contains_key(checksum) {
                for f1 in fs {
                    result.added.record_file(&f1);
                }
                return true;
            }
            let f1 = snap_1_remainder.get_mut(checksum).unwrap().remove(0);
            result.moved.record_file(&f1);
            if snap_1_remainder.get(checksum).unwrap().is_empty() {
                snap_1_remainder.remove(checksum);
            }
            return false;
        });

        for (_, fs) in snap_1_remainder {
            for f1 in fs {
                result.deleted.record_file(&f1);
            }
        }

        return result;
    }
}
