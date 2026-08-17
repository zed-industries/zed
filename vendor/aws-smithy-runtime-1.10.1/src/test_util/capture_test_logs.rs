/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::env;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::subscriber::DefaultGuard;
use tracing::Level;
use tracing_subscriber::fmt::TestWriter;

/// A guard that resets log capturing upon being dropped.
#[derive(Debug)]
pub struct LogCaptureGuard(#[allow(dead_code)] DefaultGuard);

/// Enables output of test logs to stdout
///
/// The `env` filter can be configured to set the log level for different modules
pub fn show_filtered_test_logs(filter: &str) -> LogCaptureGuard {
    let (mut writer, _rx) = Tee::stdout();
    writer.loud();

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(Mutex::new(writer))
        .finish();
    let guard = tracing::subscriber::set_default(subscriber);
    LogCaptureGuard(guard)
}

/// Enables output of test logs to stdout at trace level by default.
///
/// The env filter can be changed with the `RUST_LOG` environment variable.
#[must_use]
pub fn show_test_logs() -> LogCaptureGuard {
    let (mut writer, _rx) = Tee::stdout();
    writer.loud();

    let env_var = env::var("RUST_LOG").ok();
    let env_filter = env_var.as_deref().unwrap_or("trace");
    eprintln!(
        "Enabled verbose test logging with env filter {env_filter:?}. \
        You can change the env filter with the RUST_LOG environment variable."
    );

    show_filtered_test_logs(env_filter)
}

/// Capture logs from this test.
///
/// The logs will be captured until the `DefaultGuard` is dropped.
///
/// *Why use this instead of traced_test?*
/// This captures _all_ logs, not just logs produced by the current crate.
#[must_use] // log capturing ceases the instant the `DefaultGuard` is dropped
pub fn capture_test_logs() -> (LogCaptureGuard, Rx) {
    // it may be helpful to upstream this at some point
    let (mut writer, rx) = Tee::stdout();
    if env::var("VERBOSE_TEST_LOGS").is_ok() {
        eprintln!("Enabled verbose test logging.");
        writer.loud();
    } else {
        eprintln!("To see full logs from this test set VERBOSE_TEST_LOGS=true");
    }
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(Mutex::new(writer))
        .finish();
    let guard = tracing::subscriber::set_default(subscriber);
    (LogCaptureGuard(guard), rx)
}

/// Receiver for the captured logs.
pub struct Rx(Arc<Mutex<Vec<u8>>>);
impl Rx {
    /// Returns the captured logs as a string.
    ///
    /// # Panics
    /// This will panic if the logs are not valid UTF-8.
    pub fn contents(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

struct Tee<W> {
    buf: Arc<Mutex<Vec<u8>>>,
    quiet: bool,
    inner: W,
}

impl Tee<TestWriter> {
    fn stdout() -> (Self, Rx) {
        let buf: Arc<Mutex<Vec<u8>>> = Default::default();
        (
            Tee {
                buf: buf.clone(),
                quiet: true,
                inner: TestWriter::new(),
            },
            Rx(buf),
        )
    }
}

impl<W> Tee<W> {
    fn loud(&mut self) {
        self.quiet = false;
    }
}

impl<W> Write for Tee<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.lock().unwrap().extend_from_slice(buf);
        if !self.quiet {
            self.inner.write_all(buf)?;
            Ok(buf.len())
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
