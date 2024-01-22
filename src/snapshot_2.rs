use crate::snapshot::{FilesByChecksums, Snapshot};
use crate::snapshot_1::Comparison;
use crate::{file, report, snapshot_1};
use file::File;
use report::Report;
use snapshot_1::Snapshot1;

#[derive(Debug)]
pub struct Snapshot2 {
    snap_1: Snapshot1,
    snap_2_remainder: FilesByChecksums,
    report: Report,
}

impl Snapshot for Snapshot2 {
    // For each added file, compare it with snapshot 1. If it’s not
    // known in snapshot 1, store it in an internal remainder list.
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
                self.snap_2_remainder.add(f2);
            });
    }
}

impl Snapshot2 {
    pub fn new(source_snap: Snapshot1) -> Snapshot2 {
        return Snapshot2 {
            snap_1: source_snap,
            snap_2_remainder: FilesByChecksums::new(),
            report: Report::new(),
        };
    }

    // Processes all remaining files, both from snapshot 1 and snapshot 2.
    pub fn conclude(&mut self) -> Report {
        let (total1, mut snap_1_remainder) = self.snap_1.conclude();
        self.report.total_snap_1 = total1;

        for (checksum, fs) in self.snap_2_remainder.drain() {
            let f1 = snap_1_remainder.withdraw(&checksum);
            if f1.is_none() {
                for f1 in fs {
                    self.report.added.record(&f1);
                }
                continue;
            }
            self.report.moved.record(&f1.unwrap());
        }

        for (_, fs) in snap_1_remainder.drain() {
            for f1 in fs {
                self.report.deleted.record(&f1);
            }
        }

        return std::mem::replace(&mut self.report, Report::new());
    }
}
