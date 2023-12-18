use crate::checksum::CheckSum;
use crate::file::File;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

pub trait Snapshot {
    fn add(&mut self, f1: File);
}

#[derive(Debug)]
pub struct FilesByChecksums {
    map: HashMap<CheckSum, Vec<File>>,
}

impl FilesByChecksums {
    pub fn new() -> FilesByChecksums {
        return FilesByChecksums {
            map: HashMap::new(),
        };
    }

    pub fn add(&mut self, f: File) {
        if !self.map.contains_key(&f.check_sum) {
            self.map.insert(f.check_sum, vec![]);
        }
        self.map.get_mut(&f.check_sum).unwrap().push(f);
    }

    pub fn withdraw(&mut self, checksum: &CheckSum) -> Option<File> {
        if !self.map.contains_key(checksum) {
            return None;
        }
        let f = self.map.get_mut(checksum).unwrap().remove(0);
        if self.map.get(checksum).unwrap().is_empty() {
            self.map.remove(checksum);
        }
        return Some(f);
    }

    pub fn drain(&mut self) -> Drain<'_, CheckSum, Vec<File>> {
        return self.map.drain();
    }
}

#[cfg(test)]
mod tests {
    use crate::file::File;
    use crate::snapshot::Snapshot;
    use crate::{snapshot_1, snapshot_2};
    use snapshot_1::Snapshot1;
    use snapshot_2::Snapshot2;

    #[test]
    fn test_identical_files() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/identical-1", "identical-1"));
        s1.add(File::from_strings("/identical-2", "identical-2"));

        let mut s2 = Snapshot2::new(s1);
        s2.add(File::from_strings("/identical-1", "identical-1"));
        s2.add(File::from_strings("/identical-2", "identical-2"));

        let res = s2.conclude();
        assert_eq!(res.identical.files_count(), 2);
        assert_eq!(res.identical.size(), 22);
        assert_eq!(res.total_snap_1.files_count(), 2);
        assert_eq!(res.total_snap_1.size(), 22);
        assert_eq!(res.total_snap_2.files_count(), 2);
        assert_eq!(res.total_snap_2.size(), 22);
    }

    #[test]
    fn test_modified_files() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/modified-1", "modified-1"));
        s1.add(File::from_strings("/modified-2", "modified-2"));

        let mut s2 = Snapshot2::new(s1);
        s2.add(File::from_strings("/modified-1", "modif"));
        s2.add(File::from_strings("/modified-2", "modified-2222"));

        let res = s2.conclude();
        assert_eq!(res.modified_snap_1.files_count(), 2);
        assert_eq!(res.modified_snap_1.size(), 20);
        assert_eq!(res.modified_snap_2.files_count(), 2);
        assert_eq!(res.modified_snap_2.size(), 18);
        assert_eq!(res.total_snap_1.files_count(), 2);
        assert_eq!(res.total_snap_2.files_count(), 2);
    }

    #[test]
    fn test_moved_files() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/moved-1", "moved-1"));
        s1.add(File::from_strings("/moved-2", "moved-2"));

        let mut s2 = Snapshot2::new(s1);
        s2.add(File::from_strings("/moved-1111", "moved-1"));
        s2.add(File::from_strings("/moved-2222", "moved-2"));

        let res = s2.conclude();
        assert_eq!(res.moved.files_count(), 2);
        assert_eq!(res.total_snap_1.files_count(), 2);
        assert_eq!(res.total_snap_2.files_count(), 2);
    }

    #[test]
    fn test_added_files() {
        let s1 = Snapshot1::new();

        let mut s2 = Snapshot2::new(s1);
        s2.add(File::from_strings("/added-1", "added"));
        s2.add(File::from_strings("/added-2", "added"));

        let res = s2.conclude();
        assert_eq!(res.added.files_count(), 2);
        assert_eq!(res.total_snap_1.files_count(), 0);
        assert_eq!(res.total_snap_2.files_count(), 2);
    }

    #[test]
    fn test_deleted_files() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/deleted-1", "deleted-1"));
        s1.add(File::from_strings("/deleted-2", "deleted-2"));

        let mut s2 = Snapshot2::new(s1);

        let res = s2.conclude();
        assert_eq!(res.deleted.files_count(), 2);
        assert_eq!(res.total_snap_1.files_count(), 2);
        assert_eq!(res.total_snap_2.files_count(), 0);
    }

    #[test]
    fn test_compare_file() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/identical", "identical"));
        s1.add(File::from_strings("/modified", "modified"));
        s1.add(File::from_strings("/moved-1", "moved"));
        s1.add(File::from_strings("/deleted", "deleted"));

        let mut s2 = Snapshot2::new(s1);

        s2.add(File::from_strings("/identical", "identical"));
        s2.add(File::from_strings("/modified", "MODIFIED"));
        s2.add(File::from_strings("/moved-2", "moved"));
        s2.add(File::from_strings("/added", "added"));

        let res = s2.conclude();
        assert_eq!(res.identical.files_count(), 1);
        assert_eq!(res.modified_snap_1.files_count(), 1);
        assert_eq!(res.modified_snap_2.files_count(), 1);
        assert_eq!(res.moved.files_count(), 1);
        assert_eq!(res.deleted.files_count(), 1);
        assert_eq!(res.added.files_count(), 1);
        assert_eq!(res.total_snap_1.files_count(), 4);
        assert_eq!(res.total_snap_2.files_count(), 4);
    }

    #[test]
    fn test_duplicate_files() {
        let mut s1 = Snapshot1::new();
        s1.add(File::from_strings("/b", "1"));
        s1.add(File::from_strings("/c", "1"));

        let mut s2 = Snapshot2::new(s1);
        s2.add(File::from_strings("/a", "1"));
        s2.add(File::from_strings("/b", "1"));

        let res = s2.conclude();
        assert_eq!(res.moved.files_count(), 1);
        assert_eq!(res.identical.files_count(), 1);
    }
}
