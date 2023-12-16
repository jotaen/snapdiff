use crate::stats;
use crate::util::dec_format;

#[derive(Debug, Copy, Clone)]
pub struct Result {
    pub total_snap_1: stats::Stats,
    pub total_snap_2: stats::Stats,
    pub identical: stats::Stats,
    pub moved: stats::Stats,
    pub added: stats::Stats,
    pub deleted: stats::Stats,
    pub modified: stats::StatsDelta,
}

impl Result {
    pub fn new() -> Result {
        return Result {
            total_snap_1: stats::Stats::new(),
            total_snap_2: stats::Stats::new(),
            identical: stats::Stats::new(),
            moved: stats::Stats::new(),
            added: stats::Stats::new(),
            deleted: stats::Stats::new(),
            modified: stats::StatsDelta::new(),
        }
    }

    pub fn serialize(&self) -> String {
        let files = vec![
            "FILES".to_string(),
            dec_format(self.total_snap_1.files_count() as i128),
            dec_format(self.total_snap_2.files_count() as i128),
            dec_format(self.identical.files_count() as i128),
            dec_format(self.moved.files_count() as i128),
            dec_format(self.added.files_count() as i128),
            dec_format(self.deleted.files_count() as i128),
            dec_format(self.modified.files_count() as i128),
        ];
        let size = vec![
            "BYTES".to_string(),
            dec_format(self.total_snap_1.size() as i128),
            dec_format(self.total_snap_2.size() as i128),
            dec_format(self.identical.size() as i128),
            dec_format(self.moved.size() as i128),
            dec_format(self.added.size() as i128),
            dec_format(self.deleted.size() as i128),
            dec_format(self.modified.diff()),
        ];
        let longest_size = size.iter().map(|s| {s.len()}).max().unwrap();
        let longest_file_count = files.iter().map(|s| {s.len()}).max().unwrap();
        return format!("
                        {: >f$}     {: >b$}

TOTAL       Before      {: >f$}     {: >b$}
            After       {: >f$}     {: >b$}

OF WHICH    Identical   {: >f$}     {: >b$}
            Moved       {: >f$}     {: >b$}
            Added       {: >f$}     {: >b$}
            Deleted     {: >f$}     {: >b$}
            Modified    {: >f$}     {: >b$} (+{} / -{})
",
                       files[0], size[0], files[1], size[1], files[2], size[2],
                       files[3], size[3], files[4], size[4], files[5], size[5],
                       files[6], size[6], files[7], size[7],
                       dec_format(self.modified.gain() as i128), dec_format(self.modified.loss() as i128),
                       b = longest_size, f = longest_file_count,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::result;

    #[test]
    fn dec_format() {
        assert_eq!(result::dec_format(-123_456_789), "-123.456.789");
        assert_eq!(result::dec_format(-12_345), "-12.345");
        assert_eq!(result::dec_format(-1), "-1");
        assert_eq!(result::dec_format(0), "0");
        assert_eq!(result::dec_format(1), "1");
        assert_eq!(result::dec_format(543), "543");
        assert_eq!(result::dec_format(987), "987");
        assert_eq!(result::dec_format(1_234), "1.234");
        assert_eq!(result::dec_format(9_876), "9.876");
        assert_eq!(result::dec_format(12_345), "12.345");
        assert_eq!(result::dec_format(98_765), "98.765");
        assert_eq!(result::dec_format(123_456_789), "123.456.789");
    }
}
