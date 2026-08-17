// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the FileLogger Implementation

use super::logging::try_log;
use crate::{Config, SharedLogger};
use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::Write;
use std::sync::Mutex;

/// The WriteLogger struct. Provides a Logger implementation for structs implementing `Write`, e.g. File
pub struct WriteLogger<W: Write + Send + 'static> {
    level: LevelFilter,
    config: Config,
    writable: Mutex<W>,
}

impl<W: Write + Send + 'static> WriteLogger<W> {
    /// init function. Globally initializes the WriteLogger as the one and only used log facility.
    ///
    /// Takes the desired `Level`, `Config` and `Write` struct as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # use std::fs::File;
    /// # fn main() {
    /// let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("my_rust_bin.log").unwrap());
    /// # }
    /// ```
    pub fn init(log_level: LevelFilter, config: Config, writable: W) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        set_boxed_logger(WriteLogger::new(log_level, config, writable))
    }

    /// allows to create a new logger, that can be independently used, no matter what is globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired `Level`, `Config` and `Write` struct as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # use std::fs::File;
    /// # fn main() {
    /// let file_logger = WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_bin.log").unwrap());
    /// # }
    /// ```
    pub fn new(log_level: LevelFilter, config: Config, writable: W) -> Box<WriteLogger<W>> {
        Box::new(WriteLogger {
            level: log_level,
            config,
            writable: Mutex::new(writable),
        })
    }
}

impl<W: Write + Send + 'static> Log for WriteLogger<W> {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let mut write_lock = self.writable.lock().unwrap();
            let _ = try_log(&self.config, record, &mut *write_lock);
        }
    }

    fn flush(&self) {
        let _ = self.writable.lock().unwrap().flush();
    }
}

impl<W: Write + Send + 'static> SharedLogger for WriteLogger<W> {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config> {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}
