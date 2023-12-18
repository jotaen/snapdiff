use crate::file::SizeBytes;
use crate::format::{dec, duration_human, percent, size_human};
use crate::printer::TerminalPrinter;
use crate::report::ScanStats;
use std::io;
use std::io::Write;
use std::time::Instant;

#[derive(Debug)]
pub struct Progress {
    printer: TerminalPrinter,
    display_name: &'static str,
    initialised: Instant,
    last_trigger: Instant,
    bytes_since_last_trigger: SizeBytes,
    current_files_count: u64,
    current_size: SizeBytes,
    expected_files_count: u64,
    expected_size: SizeBytes,
    previous_files_count: Option<u64>,
}

impl Progress {
    pub fn new(
        printer: TerminalPrinter,
        display_name: &'static str,
        previous_files_count: Option<u64>,
    ) -> Progress {
        let init = Instant::now();
        return Progress {
            printer,
            display_name,
            initialised: init,
            last_trigger: init,
            bytes_since_last_trigger: 0,
            current_files_count: 0,
            current_size: 0,
            expected_files_count: 0,
            expected_size: 0,
            previous_files_count,
        };
    }

    pub fn scan_start(&mut self) {
        let TerminalPrinter {
            gray: gry,
            reset: rst,
            ..
        } = self.printer;
        self.printer
            .print(format!("{gry}{}: Indexing...{rst}", self.display_name));
    }

    pub fn scan_done(&mut self, s: &ScanStats) {
        self.expected_files_count = s.scheduled_files_count;
        self.expected_size = s.scheduled_size;
        let skipped_info = if s.skipped_files > 0 || s.skipped_folders > 0 {
            format!(
                "   (skipped {} files, {} dirs)",
                s.skipped_files, s.skipped_folders,
            )
        } else {
            "".to_string()
        };
        let TerminalPrinter {
            gray: gry,
            reset: rst,
            ..
        } = self.printer;
        self.printer.print(format!(
            "\r{gry}{}: Indexed:     {: >f$} files  {: >7}{}{rst}\n",
            self.display_name,
            dec(self.expected_files_count as i128),
            size_human(s.scheduled_size),
            skipped_info,
            f = self.files_display_length(),
        ));
        io::stdout().flush().unwrap();
    }

    pub fn process_inc(&mut self, files_added: u64, bytes_added: SizeBytes) {
        self.current_files_count += files_added;
        self.current_size += bytes_added;
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

    pub fn process_done(&mut self) {
        self.print_process("           ".to_string());
        self.printer.print("\n".to_string());
    }

    fn print_process(&mut self, rate: String) {
        let indent = " ".repeat(self.display_name.len() + 2);
        let TerminalPrinter {
            gray: gry,
            reset: rst,
            ..
        } = self.printer;
        self.printer.print(format!(
            "\r{}{gry}Processing:  {: >f$} files  {: >7}   {: >5}    {: >3}   {}{rst} ",
            indent,
            dec(self.current_files_count as i128),
            size_human(self.current_size),
            percent(self.current_size, self.expected_size),
            duration_human(self.initialised.elapsed().as_secs()),
            rate,
            f = self.files_display_length(),
        ));
        io::stdout().flush().unwrap();
    }

    fn files_display_length(&self) -> usize {
        let extra_padding = 1000;
        let count = self
            .previous_files_count
            .map(|c| c + extra_padding)
            .unwrap_or(self.expected_files_count + extra_padding);
        return dec(count as i128).len();
    }
}
