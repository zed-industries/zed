use std::{
    io::{self, Write},
    path::PathBuf,
    sync::{Mutex, atomic::AtomicU64},
};

use crate::{SCOPE_STRING_SEP_CHAR, Scope};

/// Whether stdout output is enabled.
static mut ENABLED_SINKS_STDOUT: bool = false;

/// Is Some(file) if file output is enabled.
static ENABLED_SINKS_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
static mut SINK_FILE_PATH: Option<&'static PathBuf> = None;
static mut SINK_FILE_PATH_ALT: Option<&'static PathBuf> = None;
/// Atomic counter for the size of the log file in bytes.
// TODO: make non-atomic if writing single threaded
static SINK_FILE_SIZE_BYTES: AtomicU64 = AtomicU64::new(0);
/// Maximum size of the log file before it will be rotated, in bytes.
const SINK_FILE_SIZE_BYTES_MAX: u64 = 1024 * 1024; // 1 MB

pub fn init_stdout_output() {
    unsafe {
        ENABLED_SINKS_STDOUT = true;
    }
}

pub fn init_file_output(
    path: &'static PathBuf,
    path_alt: Option<&'static PathBuf>,
) -> io::Result<()> {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let mut enabled_sinks_file = ENABLED_SINKS_FILE
        .try_lock()
        .expect("Log file lock is available during init");
    *enabled_sinks_file = Some(file);
    unsafe {
        SINK_FILE_PATH = Some(path);
        SINK_FILE_PATH_ALT = path_alt;
    }
    Ok(())
}

const LEVEL_OUTPUT_STRINGS: [&str; 5] = [
    //
    "ERROR", //
    "WARN ", //
    "INFO ", //
    "DEBUG", //
    "TRACE", //
];

pub fn submit(record: Record) {
    if unsafe { ENABLED_SINKS_STDOUT } {
        let mut stdout = std::io::stdout().lock();
        _ = writeln!(
            &mut stdout,
            "{} {} [{}] {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
            LEVEL_OUTPUT_STRINGS[record.level as usize],
            ScopeFmt(record.scope),
            record.message
        );
    }
    let mut file = ENABLED_SINKS_FILE.lock().unwrap_or_else(|handle| {
        ENABLED_SINKS_FILE.clear_poison();
        handle.into_inner()
    });
    if let Some(file) = file.as_mut() {
        _ = writeln!(
            file,
            "{} {} [{}] {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
            LEVEL_OUTPUT_STRINGS[record.level as usize],
            ScopeFmt(record.scope),
            record.message
        );
    }
}

pub fn flush() {
    _ = std::io::stdout().lock().flush();
}

struct ScopeFmt(Scope);

impl std::fmt::Display for ScopeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str(self.0[0])?;
        for scope in &self.0[1..] {
            if !scope.is_empty() {
                f.write_char(SCOPE_STRING_SEP_CHAR)?;
            }
            f.write_str(scope)?;
        }
        Ok(())
    }
}

pub struct Record<'a> {
    pub scope: Scope,
    pub level: log::Level,
    pub message: &'a std::fmt::Arguments<'a>,
}
