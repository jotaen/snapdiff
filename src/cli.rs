use crate::Error;
use clap::Parser;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

    fn get_snap<'a>(&'a self, s: &'a String) -> Result<&path::Path, Error> {
        let m = fs::metadata(s).map_err(|e| {
            return Error::from(format!("cannot open directory: {}", s), e.to_string());
        })?;
        if !m.is_dir() {
            return Err(Error::new(format!("not a directory: {}", s)));
        }
        return Ok(path::Path::new(s));
    }

    pub fn report_file(&self) -> PathBuf {
        return tempfile::NamedTempFile::new()
            .map(|f| {
                return f.path().to_path_buf();
            })
            .unwrap();
    }
}

pub type CtrlCSignal = Arc<AtomicBool>;

pub fn handle_ctrl_c() -> CtrlCSignal {
    let ctrlc_arc = Arc::new(AtomicBool::new(false));
    let r = ctrlc_arc.clone();
    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    return ctrlc_arc;
}
