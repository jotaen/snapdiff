use crate::checksum::CheckSum;
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
    snap_2_remainder: HashMap<CheckSum, Vec<File>>,
    report: Report,
}

impl Snapshot for Snapshot2 {
    fn add(&mut self, f2: File) {
        self.report.total_snap_2.record(&f2);
        self.snap_1
            .digest(&f2)
            .map(|(c, f1)| match c {
                Comparison::Identical => {
                    self.report.identical.record(&f2);
                }
                Comparison::Modified => {
                    self.report.modified_snap_1.record(&f1);
                    self.report.modified_snap_2.record(&f2);
                }
            })
            .unwrap_or_else(|| {
                if !self.snap_2_remainder.contains_key(&f2.check_sum) {
                    self.snap_2_remainder.insert(f2.check_sum, vec![]);
                }
                self.snap_2_remainder
                    .get_mut(&f2.check_sum)
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
            report: Report::new(),
        };
    }

    pub fn conclude(&mut self) -> Report {
        let (total1, mut snap_1_remainder) = self.snap_1.conclude();
        self.report.total_snap_1 = total1;
        self.snap_2_remainder.retain(|checksum, fs| {
            if !snap_1_remainder.contains_key(checksum) {
                for f1 in fs {
                    self.report.added.record(&f1);
                }
                return true;
            }
            let f1 = snap_1_remainder.get_mut(checksum).unwrap().remove(0);
            self.report.moved.record(&f1);
            if snap_1_remainder.get(checksum).unwrap().is_empty() {
                snap_1_remainder.remove(checksum);
            }
            return false;
        });

        for (_, fs) in snap_1_remainder {
            for f1 in fs {
                self.report.deleted.record(&f1);
            }
        }

        return std::mem::replace(&mut self.report, Report::new());
    }
}
