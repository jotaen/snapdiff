use crate::file::ContentsHash;
use crate::snapshot::Snapshot;
use crate::snapshot_1::Comparison;
use crate::{file, report, snapshot_1};
use file::File;
use report::Report;
use snapshot_1::Snapshot1;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Snapshot2 {
    snap_1: Snapshot1,
    snap_2_remainder: HashMap<ContentsHash, Vec<File>>,
    result: Report,
}

impl Snapshot for Snapshot2 {
    fn add(&mut self, f2: File) {
        self.result.total_snap_2.record_file(&f2);
        self.snap_1
            .digest(&f2)
            .map(|(c, f1)| match c {
                Comparison::Identical => {
                    self.result.identical.record_file(&f2);
                }
                Comparison::Modified => {
                    self.result.modified_snap_1.record_file(&f1);
                    self.result.modified_snap_2.record_file(&f2);
                }
            })
            .unwrap_or_else(|| {
                if !self.snap_2_remainder.contains_key(&f2.checksum) {
                    self.snap_2_remainder.insert(f2.checksum, vec![]);
                }
                self.snap_2_remainder
                    .get_mut(&f2.checksum)
                    .unwrap()
                    .push(f2);
            });
    }
}

impl Snapshot2 {
    pub fn new(source_snap: Snapshot1) -> Snapshot2 {
        return Snapshot2 {
            snap_1: source_snap,
            snap_2_remainder: HashMap::new(),
            result: Report::new(),
        };
    }

    pub fn conclude(&mut self) -> Report {
        let (total1, mut snap_1_remainder) = self.snap_1.conclude();
        self.result.total_snap_1 = total1;
        self.snap_2_remainder.retain(|checksum, fs| {
            if !snap_1_remainder.contains_key(checksum) {
                for f1 in fs {
                    self.result.added.record_file(&f1);
                }
                return true;
            }
            let f1 = snap_1_remainder.get_mut(checksum).unwrap().remove(0);
            self.result.moved.record_file(&f1);
            if snap_1_remainder.get(checksum).unwrap().is_empty() {
                snap_1_remainder.remove(checksum);
            }
            return false;
        });

        for (_, fs) in snap_1_remainder {
            for f1 in fs {
                self.result.deleted.record_file(&f1);
            }
        }

        return self.result;
    }
}
