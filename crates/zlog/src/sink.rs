use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::{SCOPE_STRING_SEP_CHAR, Scope};

/// Whether stdout output is enabled.
static mut ENABLED_SINKS_STDOUT: bool = false;

/// Is Some(file) if file output is enabled.
static ENABLED_SINKS_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
static SINK_FILE_PATH: OnceLock<&'static PathBuf> = OnceLock::new();
static SINK_FILE_PATH_ROTATE: OnceLock<&'static PathBuf> = OnceLock::new();
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
    path_rotate: Option<&'static PathBuf>,
) -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let mut enabled_sinks_file = ENABLED_SINKS_FILE
        .try_lock()
        .expect("Log file lock is available during init");

    SINK_FILE_PATH
        .set(path)
        .expect("Init file output should only be called once");
    if let Some(path_rotate) = path_rotate {
        SINK_FILE_PATH_ROTATE
            .set(path_rotate)
            .expect("Init file output should only be called once");
    }

    let size_bytes = file.metadata().map_or(0, |metadata| metadata.len());
    if size_bytes >= SINK_FILE_SIZE_BYTES_MAX {
        rotate_log_file(&mut file);
    } else {
        SINK_FILE_SIZE_BYTES.store(size_bytes, Ordering::Relaxed);
    }

    *enabled_sinks_file = Some(file);
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
        struct SizedWriter<'a> {
            file: &'a mut std::fs::File,
            written: u64,
        }
        impl<'a> io::Write for SizedWriter<'a> {
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
                "{} {} [{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
                LEVEL_OUTPUT_STRINGS[record.level as usize],
                ScopeFmt(record.scope),
                record.message
            );
            SINK_FILE_SIZE_BYTES.fetch_add(writer.written, Ordering::Relaxed) + writer.written
        };
        if file_size_bytes > SINK_FILE_SIZE_BYTES_MAX {
            rotate_log_file(file);
            #[cfg(debug_assertions)]
            println!(
                "Log file rotated at {} bytes now = {}",
                file_size_bytes,
                file.metadata().map_or(-1, |meta| meta.len() as i128)
            );
        }
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

fn rotate_log_file(file: &mut fs::File) {
    if let Err(err) = file.flush() {
        eprintln!(
            "Failed to flush log file before rotating, some logs may be lost: {}",
            err
        );
    }
    let mut rotate_error: Option<anyhow::Error> = Some(anyhow::anyhow!(
        "Failed to copy log file for unknown reason"
    ));
    if let Some(path) = SINK_FILE_PATH.get() {
        if let Some(path_rotate) = SINK_FILE_PATH_ROTATE.get() {
            rotate_error = fs::copy(path, path_rotate)
                .err()
                .map(|err| anyhow::anyhow!(err));
        } else {
            rotate_error.replace(anyhow::anyhow!("No rotation log file path configured"));
        }
    } else {
        // should never happen, but a panic here doesn't make sense
        rotate_error.replace(anyhow::anyhow!("No log file path configured"));
    }
    if let Some(err) = rotate_error {
        eprintln!(
            "Log file rotation failed. Truncating log file anyways: {}",
            err,
        );
    }
    _ = file.set_len(0);

    // SAFETY: It is safe to set size to 0 even if set_len fails as
    // according to the documentation, it only fails if:
    // - the file is not writeable: should never happen,
    // - the size would cause an overflow (implementation specific): 0 should never cause an overflow
    SINK_FILE_SIZE_BYTES.store(0, Ordering::Relaxed);
}
