use crate::filter::Filter;
use crate::printer::{FilePrinter, TerminalPrinter};
use crate::Error;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::{fs, path};

pub struct Cli {
    pub snap1_root: path::PathBuf,
    pub snap2_root: path::PathBuf,
    pub filters: Filter,
    pub workers1: usize,
    pub workers2: usize,
    pub terminal_printer: TerminalPrinter,
    pub file_printer: Option<FilePrinter>,
    pub ctrl_c: CtrlCSignal,
}

pub struct CtrlCSignal(Arc<AtomicBool>);

impl Clone for CtrlCSignal {
    fn clone(&self) -> Self {
        return CtrlCSignal(Arc::clone(&self.0));
    }
}

impl CtrlCSignal {
    pub fn has_triggered(&self) -> bool {
        return self.0.load(Ordering::SeqCst);
    }
}

/// snapdiff compares two snapshots of a directory tree, captured at different points
/// in time. That way, it gives a high-level insight into how the directory tree has
/// evolved over time. It summarises the difference between both snapshots based on
/// the following categories:
/// - Identical: both snapshots contain a file at the same path with the same contents.
/// - Moved:     both snapshots contain a file with the same contents, but at different
///              paths.
/// - Added:     the second snapshot contains a file whose path or contents is not
///              present in the first snapshot.
/// - Deleted:   the first snapshot contains a file whose path or contents is not
///              present in the second snapshot.
/// - Modified:  both snapshots contain a file at the same path, but with different
///              contents.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {
    /// Path to the first snapshot (the older one).
    #[arg(verbatim_doc_comment)]
    snap1_path: String,

    /// Path to the second snapshot (the more recent one).
    #[arg(verbatim_doc_comment)]
    snap2_path: String,

    /// Print a detailed report to a file. The report lists
    /// all captured file names (one per line, for all but
    /// identical files).
    #[arg(long = "report", short = 'r', verbatim_doc_comment)]
    report_file: Option<String>,

    /// Include files or folders whose name start with a dot,
    /// instead of ignoring them (which is the default). For
    /// dot-folders, it ignores the entire (sub-)directory
    /// tree, with all files and folders it may contain.
    #[arg(
        long = "include-dot-paths",
        short = 'd',
        default_value_t = false,
        verbatim_doc_comment
    )]
    include_dot_paths: bool,

    /// Include symlinks, instead of ignoring them (which is
    /// the default). If symlinks are included, it counts one
    /// file per symlink, without increasing the byte count.
    /// If the symlink target had been changed between snapshots,
    /// it counts the symlink file as modified.
    #[arg(
        long = "include-symlinks",
        short = 's',
        default_value_t = false,
        verbatim_doc_comment
    )]
    include_symlinks: bool,

    /// Number of CPU cores to utilise. A value of `0` means
    /// that all available cores are maxed out (which is the
    /// default). The value can be distinguished for each
    /// snapshot side via a colon, e.g. `1:4`.
    #[arg(
        long = "workers",
        alias = "worker",
        value_delimiter = ':',
        verbatim_doc_comment
    )]
    workers: Option<Vec<usize>>,

    /// Disable output colouring.
    #[arg(
        long = "no-color",
        alias = "no-colour",
        default_value_t = false,
        verbatim_doc_comment
    )]
    no_color: bool,
}

impl Cli {
    pub fn new_from_env() -> Result<Cli, Error> {
        let args = Args::parse();
        let (workers1, workers2) = num_workers(args.workers);
        return Ok(Cli {
            snap1_root: get_snap(&args.snap1_path)?,
            snap2_root: get_snap(&args.snap2_path)?,
            filters: Filter::new(args.include_symlinks, args.include_dot_paths),
            workers1,
            workers2,
            terminal_printer: if args.no_color {
                TerminalPrinter::new_plain()
            } else {
                TerminalPrinter::new()
            },
            file_printer: if args.report_file.is_none() {
                None
            } else {
                let f = &args.report_file.unwrap();
                let p = path::Path::new(f);
                if p.exists() {
                    return Err(Error::new(format!(
                        "report file already exists: {}",
                        p.display()
                    )));
                }
                Some(FilePrinter::new(p)?)
            },
            ctrl_c: {
                let ctrl_c = Arc::new(AtomicBool::new(false));
                let c_arc = Arc::clone(&ctrl_c);
                ctrlc::set_handler(move || {
                    c_arc.store(true, Ordering::SeqCst);
                })
                .map_err(|e| {
                    Error::from("failed to register ^C handler".to_string(), e.to_string())
                })?;
                CtrlCSignal(ctrl_c)
            },
        });
    }
}

fn num_workers(ws: Option<Vec<usize>>) -> (usize, usize) {
    let cores = thread::available_parallelism().unwrap().get();
    return ws
        .map(|ws| {
            if ws.len() == 1 {
                return (ws[0], ws[0]);
            }
            return (ws[0], ws[1]);
        })
        .map(|(mut w1, mut w2)| {
            if w1 == 0 {
                w1 = cores;
            }
            if w2 == 0 {
                w2 = cores;
            }
            return (w1, w2);
        })
        .unwrap_or_else(|| {
            return (cores, cores);
        });
}

fn get_snap(s: &String) -> Result<path::PathBuf, Error> {
    let m = fs::metadata(s).map_err(|e| {
        return Error::from(format!("cannot open directory: {}", s), e.to_string());
    })?;
    if !m.is_dir() {
        return Err(Error::new(format!("not a directory: {}", s)));
    }
    return Ok(path::Path::new(s).to_path_buf());
}
