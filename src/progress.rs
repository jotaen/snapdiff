use crate::dir_iter::ScanStats;
use crate::file::SizeBytes;
use crate::format::term::*;
use crate::format::{dec, duration_human, percent, size_human};
use std::io;
use std::io::Write;
use std::time::Instant;

#[derive(Debug)]
pub struct Progress {
    display_name: &'static str,
    initialised: Instant,
    last_trigger: Instant,
    bytes_since_last_trigger: SizeBytes,
    files_count: u64,
    size: SizeBytes,
    expected_files_count: u64,
    expected_size: SizeBytes,
}

impl Progress {
    pub fn new(display_name: &'static str) -> Progress {
        let init = Instant::now();
        return Progress {
            display_name,
            initialised: init,
            last_trigger: init,
            bytes_since_last_trigger: 0,
            files_count: 0,
            size: 0,
            expected_files_count: 0,
            expected_size: 0,
        };
    }

    pub fn scan_start(&self) {
        print!("{GRY}{}:  Scanning...{RST}", self.display_name);
        io::stdout().flush().unwrap();
    }

    pub fn scan_done(&mut self, s: &ScanStats) {
        self.expected_files_count = s.scheduled_files_count;
        self.expected_size = s.scheduled_size;
        let file_count = dec(s.scheduled_files_count as i128);
        let skipped_info = if s.skipped_files > 0 || s.skipped_folders > 0 {
            format!(
                "   (skipped {} files, {} dirs)",
                s.skipped_files, s.skipped_folders,
            )
        } else {
            "".to_string()
        };
        print!(
            "\r{GRY}{}:    Indexed:  {: >f$} files  {: >7}{}{RST}\n",
            self.display_name,
            file_count,
            size_human(s.scheduled_size),
            skipped_info,
            f = file_count.len(),
        );
        io::stdout().flush().unwrap();
    }

    pub fn process_inc(&mut self, files_added: u64, bytes_added: SizeBytes) {
        self.files_count += files_added;
        self.size += bytes_added;
        self.bytes_since_last_trigger += bytes_added;
        let elapsed_ms = self.last_trigger.elapsed().as_millis();
        if self.initialised != self.last_trigger && elapsed_ms < 666 {
            return;
        }
        let rate = if elapsed_ms != 0 {
            let s = (1000 * self.bytes_since_last_trigger as u128 / elapsed_ms) as SizeBytes;
            format!("[{: >7}/s]", size_human(s))
        } else {
            "".to_string()
        };
        self.print_process(rate);
        self.last_trigger = Instant::now();
        self.bytes_since_last_trigger = 0;
    }

    pub fn process_done(&self) {
        self.print_process("           ".to_string());
        print!("\n");
    }

    fn print_process(&self, rate: String) {
        let indent = " ".repeat(self.display_name.len() + 2);
        print!(
            "\r{}{GRY}Processing:  {: >f$} files  {: >7}   {: >5}  {: >4}   {}{RST} ",
            indent,
            dec(self.files_count as i128),
            size_human(self.size),
            percent(self.size, self.expected_size),
            duration_human(self.initialised.elapsed().as_secs()),
            rate,
            f = dec(self.expected_files_count as i128).len(),
        );
        io::stdout().flush().unwrap();
    }
}
