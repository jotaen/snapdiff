use crate::Error;
use clap::Parser;
use std::{fs, path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    snap1_path: String,
    snap2_path: String,
}

impl Cli {
    pub fn snap1(&self) -> Result<&path::Path, Error> {
        return self.get_snap(&self.snap1_path);
    }

    pub fn snap2(&self) -> Result<&path::Path, Error> {
        return self.get_snap(&self.snap2_path);
    }

    pub fn get_snap<'a>(&'a self, s: &'a String) -> Result<&path::Path, Error> {
        let m = fs::metadata(s);
        if !m.is_ok() {
            return Err(format!("cannot open directory: {}", s));
        }
        if !m.unwrap().is_dir() {
            return Err(format!("not a directory: {}", s));
        }
        return Ok(path::Path::new(s));
    }
}
