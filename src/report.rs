use crate::format::{dec, dec_signed};
use crate::printer::{Colours, Printer, TerminalPrinter, SNP1, SNP2};
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

impl Report {
    pub fn new() -> Report {
        return Report {
            total_snap_1: Stats::new(),
            total_snap_2: Stats::new(),
            identical: Stats::new(),
            moved: Stats::new_with_file_storage(),
            added: Stats::new_with_file_storage(),
            deleted: Stats::new_with_file_storage(),
            modified_snap_1: Stats::new(),
            modified_snap_2: Stats::new_with_file_storage(),
        };
    }

    pub fn detailed_list(&self, printer: &mut dyn Printer) {
        printer.print(format!(
            "#sn1 {} ({} files)\n",
            self.total_snap_1.count.size, self.total_snap_1.count.files
        ));
        printer.print(format!(
            "#sn2 {} ({} files)\n",
            self.total_snap_2.count.size, self.total_snap_2.count.files
        ));
        printer.print(format!(
            "=idn {} ({} files)\n",
            self.identical.count.size, self.identical.count.files
        ));
        for f in self.moved.files().unwrap() {
            printer.print(format!(">mvd {} {}\n", f.size, f.path.display()));
        }
        for f in self.added.files().unwrap() {
            printer.print(format!("+add {} {}\n", f.size, f.path.display()));
        }
        for f in self.deleted.files().unwrap() {
            printer.print(format!("-del {} {}\n", f.size, f.path.display()));
        }
        for f in self.modified_snap_2.files().unwrap() {
            printer.print(format!("*mdf {} {}\n", f.size, f.path.display()));
        }
    }

    pub fn summary(&self, mut printer: TerminalPrinter) {
        let files = vec![
            "FILES".to_string(),
            dec(self.total_snap_1.count.files as i128),
            dec(self.total_snap_2.count.files as i128),
            dec(self.identical.count.files as i128),
            dec(self.moved.count.files as i128),
            dec(self.added.count.files as i128),
            dec(self.deleted.count.files as i128),
            dec(self.modified_snap_2.count.files as i128),
        ];
        let size = vec![
            "BYTES".to_string(),
            dec(self.total_snap_1.count.size as i128),
            dec(self.total_snap_2.count.size as i128),
            dec(self.identical.count.size as i128),
            dec(self.moved.count.size as i128),
            dec(self.added.count.size as i128),
            dec(self.deleted.count.size as i128),
            dec(self.modified_snap_2.count.size as i128),
        ];
        let longest_size = size.iter().map(|s| s.len()).max().unwrap();
        let longest_file_count = files.iter().map(|s| s.len()).max().unwrap();
        let byte_markers = {
            let markers = "T   G   M   K   B";
            markers[markers.len() - longest_size..].to_string()
        };
        let modified_delta = {
            let delta =
                self.modified_snap_2.count.size as i128 - self.modified_snap_1.count.size as i128;
            if delta == 0 {
                "±0".to_string()
            } else {
                dec_signed(delta)
            }
        };
        let Colours {
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
        } = printer.colours;
        printer.print(format!(
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
        ));
    }
}
