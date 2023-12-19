use crate::error::Error;
use std::io::{BufWriter, Write};
use std::{fs, io, path};

pub const SNP1: &str = "Snap 1";
pub const SNP2: &str = "Snap 2";

pub trait Printer {
    fn print(&mut self, text: String);
    fn colours(&self) -> &Colours;
}

#[derive(Debug, Copy, Clone)]
pub struct Colours {
    pub blank: &'static str,
    pub reset: &'static str,
    pub bold: &'static str,
    pub light: &'static str,
    pub gray: &'static str,
    pub dark: &'static str,
    pub blue: &'static str,
    pub green: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub brown: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub struct TerminalPrinter {
    pub colours: Colours,
}

const WITH_COLOURS: Colours = Colours {
    blank: "",
    reset: "\x1b[0m",
    bold: "\x1b[1m",
    light: "\x1b[38;5;253m",
    gray: "\x1b[38;5;246m",
    dark: "\x1b[38;5;237m",
    blue: "\x1b[38;5;039m",
    green: "\x1b[38;5;082m",
    red: "\x1b[38;5;202m",
    yellow: "\x1b[38;5;220m",
    brown: "\x1b[38;5;094m",
};

const NO_COLOURS: Colours = Colours {
    blank: "",
    reset: "",
    bold: "",
    light: "",
    gray: "",
    dark: "",
    blue: "",
    green: "",
    red: "",
    yellow: "",
    brown: "",
};

impl TerminalPrinter {
    pub fn new() -> TerminalPrinter {
        return TerminalPrinter {
            colours: WITH_COLOURS,
        };
    }

    pub fn new_plain() -> TerminalPrinter {
        return TerminalPrinter {
            colours: NO_COLOURS,
        };
    }
}

impl Printer for TerminalPrinter {
    fn print(&mut self, text: String) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    fn colours(&self) -> &Colours {
        return &self.colours;
    }
}

#[derive(Debug)]
pub struct FilePrinter {
    target_file: fs::File,
}

impl FilePrinter {
    pub fn new(p: &path::Path) -> Result<FilePrinter, Error> {
        let target_file = fs::File::create(&p)?;
        return Ok(FilePrinter { target_file });
    }
}

impl Printer for FilePrinter {
    fn print(&mut self, text: String) {
        let mut buffer = BufWriter::new(&self.target_file);
        write!(buffer, "{}", text).expect("failed to write to report file");
    }

    fn colours(&self) -> &Colours {
        return &NO_COLOURS;
    }
}

#[allow(dead_code)]
pub struct MockPrinter {
    sink: String,
}

impl Printer for MockPrinter {
    fn print(&mut self, text: String) {
        self.sink.push_str(&text);
    }

    fn colours(&self) -> &Colours {
        return &NO_COLOURS;
    }
}
