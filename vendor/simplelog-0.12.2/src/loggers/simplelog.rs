// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the SimpleLogger Implementation

use super::logging::try_log;
use crate::{Config, SharedLogger};
use log::{
    set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError,
};
use std::io::{stderr, stdout};
use std::sync::Mutex;

/// The SimpleLogger struct. Provides a very basic Logger implementation
pub struct SimpleLogger {
    level: LevelFilter,
    config: Config,
    output_lock: Mutex<()>,
}

impl SimpleLogger {
    /// init function. Globally initializes the SimpleLogger as the one and only used log facility.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn init(log_level: LevelFilter, config: Config) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        set_boxed_logger(SimpleLogger::new(log_level, config))
    }

    /// allows to create a new logger, that can be independently used, no matter what is globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let simple_logger = SimpleLogger::new(LevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn new(log_level: LevelFilter, config: Config) -> Box<SimpleLogger> {
        Box::new(SimpleLogger {
            level: log_level,
            config,
            output_lock: Mutex::new(()),
        })
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let _lock = self.output_lock.lock().unwrap();

            match record.level() {
                Level::Error => {
                    let stderr = stderr();
                    let mut stderr_lock = stderr.lock();
                    let _ = try_log(&self.config, record, &mut stderr_lock);
                }
                _ => {
                    let stdout = stdout();
                    let mut stdout_lock = stdout.lock();
                    let _ = try_log(&self.config, record, &mut stdout_lock);
                }
            }
        }
    }

    fn flush(&self) {
        use std::io::Write;
        let _ = stdout().flush();
    }
}

impl SharedLogger for SimpleLogger {
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
