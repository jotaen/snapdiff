use crate::file::SizeBytes;
use crate::format::{dec, dec_signed};
use crate::printer::{Printer, SNP1, SNP2};
use crate::stats;
use stats::Stats;

#[derive(Debug)]
pub struct Report {
    pub total_snap_1: Stats,
    pub total_snap_2: Stats,
    pub identical: Stats,
    pub moved: Stats,
    pub added: Stats,
    pub deleted: Stats,
    pub modified_snap_1: Stats,
    pub modified_snap_2: Stats,
}

#[derive(Debug)]
pub struct ScanStats {
    pub scheduled_files_count: u64,
    pub scheduled_size: SizeBytes,
    pub skipped_folders: u64,
    pub skipped_files: u64,
}

impl Report {
    pub fn new() -> Report {
        return Report {
            total_snap_1: Stats::new(),
            total_snap_2: Stats::new(),
            identical: Stats::new(),
            moved: Stats::new(),
            added: Stats::new(),
            deleted: Stats::new(),
            modified_snap_1: Stats::new(),
            modified_snap_2: Stats::new(),
        };
    }

    pub fn summary(&self, printer: Printer) -> String {
        let files = vec![
            "FILES".to_string(),
            dec(self.total_snap_1.files_count() as i128),
            dec(self.total_snap_2.files_count() as i128),
            dec(self.identical.files_count() as i128),
            dec(self.moved.files_count() as i128),
            dec(self.added.files_count() as i128),
            dec(self.deleted.files_count() as i128),
            dec(self.modified_snap_2.files_count() as i128),
        ];
        let size = vec![
            "BYTES".to_string(),
            dec(self.total_snap_1.size() as i128),
            dec(self.total_snap_2.size() as i128),
            dec(self.identical.size() as i128),
            dec(self.moved.size() as i128),
            format!("+{}", dec(self.added.size() as i128)),
            format!("-{}", dec(self.deleted.size() as i128)),
            dec(self.modified_snap_2.size() as i128),
        ];
        let longest_size = size.iter().map(|s| s.len()).max().unwrap();
        let longest_file_count = files.iter().map(|s| s.len()).max().unwrap();
        let byte_markers = {
            let markers = "T   G   M   K   B";
            markers[markers.len() - longest_size..].to_string()
        };
        let modified_delta = {
            let delta = self.modified_snap_2.size() as i128 - self.modified_snap_1.size() as i128;
            if delta == 0 {
                "±0".to_string()
            } else {
                dec_signed(delta)
            }
        };
        let Printer {
            blank: ___,
            dark: drk,
            yellow: ylw,
            brown: brn,
            light: lgt,
            blue: blu,
            green: grn,
            red,
            reset: rst,
            bold: bld,
            ..
        } = printer;
        return format!(
            "
{bld}            {___}{___}            {: >f$}     {: >b$}{rst}
{bld}            {rst}{drk}            {: >f$}     {: >b$}{rst}
{bld}TOTAL       {rst}{lgt}{SNP1}      {: >f$}     {: >b$}{rst}
{bld}            {rst}{lgt}{SNP2}      {: >f$}     {: >b$}{rst}
{bld}            {rst}{lgt}
{bld}OF WHICH    {rst}{blu}Identical   {: >f$}     {: >b$}{rst}
{bld}            {rst}{blu}Moved       {: >f$}     {: >b$}{rst}
{bld}            {rst}{grn}Added       {: >f$}     {: >b$}{rst}
{bld}            {rst}{red}Deleted     {: >f$}     {: >b$}{rst}
{bld}            {rst}{ylw}Modified    {: >f$}     {: >b$}{brn} ({}){rst}
",
            files[0],
            size[0],
            "".to_string(),
            byte_markers,
            files[1],
            size[1],
            files[2],
            size[2],
            files[3],
            size[3],
            files[4],
            size[4],
            files[5],
            size[5],
            files[6],
            size[6],
            files[7],
            size[7],
            modified_delta,
            b = longest_size,
            f = longest_file_count,
        );
    }
}
