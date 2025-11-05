use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use crate::{SCOPE_STRING_SEP_CHAR, Scope};

// ANSI color escape codes for log levels
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_MAGENTA: &str = "\x1b[35m";

/// Is Some(file) if file output is enabled.
static ENABLED_SINKS_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
static SINK_FILE_PATH: OnceLock<&'static PathBuf> = OnceLock::new();
static SINK_FILE_PATH_ROTATE: OnceLock<&'static PathBuf> = OnceLock::new();

// NB: Since this can be accessed in tests, we probably should stick to atomics here.
/// Whether stdout output is enabled.
static ENABLED_SINKS_STDOUT: AtomicBool = AtomicBool::new(false);
/// Whether stderr output is enabled.
static ENABLED_SINKS_STDERR: AtomicBool = AtomicBool::new(false);
/// Atomic counter for the size of the log file in bytes.
static SINK_FILE_SIZE_BYTES: AtomicU64 = AtomicU64::new(0);
/// Maximum size of the log file before it will be rotated, in bytes.
const SINK_FILE_SIZE_BYTES_MAX: u64 = 1024 * 1024; // 1 MB

pub struct Record<'a> {
    pub scope: Scope,
    pub level: log::Level,
    pub message: &'a std::fmt::Arguments<'a>,
    pub module_path: Option<&'a str>,
}

pub fn init_output_stdout() {
    // Use atomics here instead of just a `static mut`, since in the context
    // of tests these accesses can be multi-threaded.
    ENABLED_SINKS_STDOUT.store(true, Ordering::Release);
}

pub fn init_output_stderr() {
    ENABLED_SINKS_STDERR.store(true, Ordering::Release);
}

pub fn init_output_file(
    path: &'static PathBuf,
    path_rotate: Option<&'static PathBuf>,
) -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    SINK_FILE_PATH
        .set(path)
        .expect("Init file output should only be called once");
    if let Some(path_rotate) = path_rotate {
        SINK_FILE_PATH_ROTATE
            .set(path_rotate)
            .expect("Init file output should only be called once");
    }

    let mut enabled_sinks_file = ENABLED_SINKS_FILE
        .try_lock()
        .expect("Log file lock is available during init");

    let size_bytes = file.metadata().map_or(0, |metadata| metadata.len());
    if size_bytes >= SINK_FILE_SIZE_BYTES_MAX {
        rotate_log_file(&mut file, Some(path), path_rotate, &SINK_FILE_SIZE_BYTES);
    } else {
        SINK_FILE_SIZE_BYTES.store(size_bytes, Ordering::Release);
    }

    *enabled_sinks_file = Some(file);

    Ok(())
}

const LEVEL_OUTPUT_STRINGS: [&str; 6] = [
    "     ", // nop: ERROR = 1
    "ERROR", //
    "WARN ", //
    "INFO ", //
    "DEBUG", //
    "TRACE", //
];

// Colors for different log levels
static LEVEL_ANSI_COLORS: [&str; 6] = [
    "",           // nop
    ANSI_RED,     // Error: Red
    ANSI_YELLOW,  // Warn: Yellow
    ANSI_GREEN,   // Info: Green
    ANSI_BLUE,    // Debug: Blue
    ANSI_MAGENTA, // Trace: Magenta
];

// PERF: batching
pub fn submit(record: Record) {
    if ENABLED_SINKS_STDOUT.load(Ordering::Acquire) {
        let mut stdout = std::io::stdout().lock();
        _ = writeln!(
            &mut stdout,
            "{} {ANSI_BOLD}{}{}{ANSI_RESET} {} {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
            LEVEL_ANSI_COLORS[record.level as usize],
            LEVEL_OUTPUT_STRINGS[record.level as usize],
            SourceFmt {
                scope: record.scope,
                module_path: record.module_path,
                ansi: true,
            },
            record.message
        );
    } else if ENABLED_SINKS_STDERR.load(Ordering::Acquire) {
        let mut stdout = std::io::stderr().lock();
        _ = writeln!(
            &mut stdout,
            "{} {ANSI_BOLD}{}{}{ANSI_RESET} {} {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
            LEVEL_ANSI_COLORS[record.level as usize],
            LEVEL_OUTPUT_STRINGS[record.level as usize],
            SourceFmt {
                scope: record.scope,
                module_path: record.module_path,
                ansi: true,
            },
            record.message
        );
    }
    let mut file = ENABLED_SINKS_FILE.lock().unwrap_or_else(|handle| {
        ENABLED_SINKS_FILE.clear_poison();
        handle.into_inner()
    });
    if let Some(file) = file.as_mut() {
        struct SizedWriter<'a> {
            file: &'a mut std::fs::File,
            written: u64,
        }
        impl io::Write for SizedWriter<'_> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.file.write(buf)?;
                self.written += buf.len() as u64;
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                self.file.flush()
            }
        }
        let file_size_bytes = {
            let mut writer = SizedWriter { file, written: 0 };
            _ = writeln!(
                &mut writer,
                "{} {} {} {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
                LEVEL_OUTPUT_STRINGS[record.level as usize],
                SourceFmt {
                    scope: record.scope,
                    module_path: record.module_path,
                    ansi: false,
                },
                record.message
            );
            SINK_FILE_SIZE_BYTES.fetch_add(writer.written, Ordering::AcqRel) + writer.written
        };
        if file_size_bytes > SINK_FILE_SIZE_BYTES_MAX {
            rotate_log_file(
                file,
                SINK_FILE_PATH.get(),
                SINK_FILE_PATH_ROTATE.get(),
                &SINK_FILE_SIZE_BYTES,
            );
        }
    }
}

pub fn flush() {
    if ENABLED_SINKS_STDOUT.load(Ordering::Acquire) {
        _ = std::io::stdout().lock().flush();
    }
    let mut file = ENABLED_SINKS_FILE.lock().unwrap_or_else(|handle| {
        ENABLED_SINKS_FILE.clear_poison();
        handle.into_inner()
    });
    if let Some(file) = file.as_mut()
        && let Err(err) = file.flush()
    {
        eprintln!("Failed to flush log file: {}", err);
    }
}

struct SourceFmt<'a> {
    scope: Scope,
    module_path: Option<&'a str>,
    ansi: bool,
}

impl std::fmt::Display for SourceFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('[')?;
        if self.ansi {
            f.write_str(ANSI_BOLD)?;
        }
        // NOTE: if no longer prefixing scopes with their crate name, check if scope[0] is empty
        if (self.scope[1].is_empty() && self.module_path.is_some()) || self.scope[0].is_empty() {
            f.write_str(self.module_path.unwrap_or("?"))?;
        } else {
            f.write_str(self.scope[0])?;
            for subscope in &self.scope[1..] {
                if subscope.is_empty() {
                    break;
                }
                f.write_char(SCOPE_STRING_SEP_CHAR)?;
                f.write_str(subscope)?;
            }
        }
        if self.ansi {
            f.write_str(ANSI_RESET)?;
        }
        f.write_char(']')?;
        Ok(())
    }
}

fn rotate_log_file<PathRef>(
    file: &mut fs::File,
    path: Option<PathRef>,
    path_rotate: Option<PathRef>,
    atomic_size: &AtomicU64,
) where
    PathRef: AsRef<std::path::Path>,
{
    if let Err(err) = file.flush() {
        eprintln!(
            "Failed to flush log file before rotating, some logs may be lost: {}",
            err
        );
    }
    let rotation_error = match (path, path_rotate) {
        (Some(_), None) => Some(anyhow::anyhow!("No rotation log file path configured")),
        (None, _) => Some(anyhow::anyhow!("No log file path configured")),
        (Some(path), Some(path_rotate)) => fs::copy(path, path_rotate)
            .err()
            .map(|err| anyhow::anyhow!(err)),
    };
    if let Some(err) = rotation_error {
        eprintln!("Log file rotation failed. Truncating log file anyways: {err}",);
    }
    _ = file.set_len(0);

    // SAFETY: It is safe to set size to 0 even if set_len fails as
    // according to the documentation, it only fails if:
    // - the file is not writeable: should never happen,
    // - the size would cause an overflow (implementation specific): 0 should never cause an overflow
    atomic_size.store(0, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_log_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let log_file_path = temp_dir.path().join("log.txt");
        let rotation_log_file_path = temp_dir.path().join("log_rotated.txt");

        let mut file = fs::File::create(&log_file_path).unwrap();
        let contents = String::from("Hello, world!");
        file.write_all(contents.as_bytes()).unwrap();

        let size = AtomicU64::new(contents.len() as u64);

        rotate_log_file(
            &mut file,
            Some(&log_file_path),
            Some(&rotation_log_file_path),
            &size,
        );

        assert!(log_file_path.exists());
        assert_eq!(log_file_path.metadata().unwrap().len(), 0);
        assert!(rotation_log_file_path.exists());
        assert_eq!(
            std::fs::read_to_string(&rotation_log_file_path).unwrap(),
            contents,
        );
        assert_eq!(size.load(Ordering::Acquire), 0);
    }

    /// Regression test, ensuring that if log level values change we are made aware
    #[test]
    fn test_log_level_names() {
        assert_eq!(LEVEL_OUTPUT_STRINGS[log::Level::Error as usize], "ERROR");
        assert_eq!(LEVEL_OUTPUT_STRINGS[log::Level::Warn as usize], "WARN ");
        assert_eq!(LEVEL_OUTPUT_STRINGS[log::Level::Info as usize], "INFO ");
        assert_eq!(LEVEL_OUTPUT_STRINGS[log::Level::Debug as usize], "DEBUG");
        assert_eq!(LEVEL_OUTPUT_STRINGS[log::Level::Trace as usize], "TRACE");
    }
}
