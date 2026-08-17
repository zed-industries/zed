// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//!
//! `simplelog` provides a series of logging facilities, that can be easily combined.
//!
//! - `SimpleLogger` (very basic logger that logs to stdout)
//! - `TermLogger` (advanced terminal logger, that splits to stdout/err and has color support) (can be excluded on unsupported platforms)
//! - `WriteLogger` (logs to a given struct implementing `Write`, e.g. a file)
//! - `CombinedLogger` (can be used to form combinations of the above loggers)
//! - `TestLogger` (specialized logger for tests. Uses print!() / println!() for tests to be able to capture the output)
//!
//! Only one Logger should be initialized of the start of your program
//! through the `Logger::init(...)` method. For the actual calling syntax
//! take a look at the documentation of the specific implementation(s) you wanna use.
//!

#![deny(missing_docs, rust_2018_idioms)]

mod config;
mod loggers;

pub use self::config::{
    format_description, Config, ConfigBuilder, FormatItem, LevelPadding, TargetPadding,
    ThreadLogMode, ThreadPadding,
};
#[cfg(feature = "test")]
pub use self::loggers::TestLogger;
pub use self::loggers::{CombinedLogger, SimpleLogger, WriteLogger};
#[cfg(feature = "termcolor")]
pub use self::loggers::{TermLogger, TerminalMode};
#[cfg(feature = "termcolor")]
pub use termcolor::{Color, ColorChoice};

pub use log::{Level, LevelFilter};

use log::Log;
#[cfg(all(test, not(feature = "paris")))]
use log::*;

#[cfg(feature = "paris")]
pub(crate) mod paris_macros;
#[cfg(feature = "paris")]
#[doc(hidden)]
pub mod __private {
    pub use log;
    pub use paris;
}

/// Trait to have a common interface to obtain the Level of Loggers
///
/// Necessary for CombinedLogger to calculate
/// the lowest used Level.
///
pub trait SharedLogger: Log {
    /// Returns the set Level for this Logger
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let logger = SimpleLogger::new(LevelFilter::Info, Config::default());
    /// println!("{}", logger.level());
    /// # }
    /// ```
    fn level(&self) -> LevelFilter;

    /// Inspect the config of a running Logger
    ///
    /// An Option is returned, because some Logger may not contain a Config
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let logger = SimpleLogger::new(LevelFilter::Info, Config::default());
    /// println!("{:?}", logger.config());
    /// # }
    /// ```
    fn config(&self) -> Option<&Config>;

    /// Returns the logger as a Log trait object
    fn as_log(self: Box<Self>) -> Box<dyn Log>;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn test() {
        let mut i = 0;

        CombinedLogger::init({
            let mut vec = Vec::new();
            let mut conf_builder = ConfigBuilder::new();

            let conf_thread_name = ConfigBuilder::new()
                .set_time_level(LevelFilter::Off)
                .set_thread_level(LevelFilter::Error)
                .set_thread_mode(ThreadLogMode::Names)
                .build();

            vec.push(WriteLogger::new(
                LevelFilter::Error,
                conf_thread_name,
                File::create("thread_naming.log").unwrap(),
            ) as Box<dyn SharedLogger>);

            for elem in vec![
                LevelFilter::Off,
                LevelFilter::Trace,
                LevelFilter::Debug,
                LevelFilter::Info,
                LevelFilter::Warn,
                LevelFilter::Error,
            ] {
                let conf = conf_builder
                    .set_location_level(elem)
                    .set_target_level(elem)
                    .set_max_level(elem)
                    .set_time_level(elem)
                    .build();
                i += 1;

                //error
                vec.push(
                    SimpleLogger::new(LevelFilter::Error, conf.clone()) as Box<dyn SharedLogger>
                );
                #[cfg(feature = "termcolor")]
                vec.push(TermLogger::new(
                    LevelFilter::Error,
                    conf.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ) as Box<dyn SharedLogger>);
                vec.push(WriteLogger::new(
                    LevelFilter::Error,
                    conf.clone(),
                    File::create(&format!("error_{}.log", i)).unwrap(),
                ) as Box<dyn SharedLogger>);
                #[cfg(feature = "test")]
                vec.push(TestLogger::new(LevelFilter::Error, conf.clone()));

                //warn
                vec.push(
                    SimpleLogger::new(LevelFilter::Warn, conf.clone()) as Box<dyn SharedLogger>
                );
                #[cfg(feature = "termcolor")]
                vec.push(TermLogger::new(
                    LevelFilter::Warn,
                    conf.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ) as Box<dyn SharedLogger>);
                vec.push(WriteLogger::new(
                    LevelFilter::Warn,
                    conf.clone(),
                    File::create(&format!("warn_{}.log", i)).unwrap(),
                ) as Box<dyn SharedLogger>);
                #[cfg(feature = "test")]
                vec.push(TestLogger::new(LevelFilter::Warn, conf.clone()));

                //info
                vec.push(
                    SimpleLogger::new(LevelFilter::Info, conf.clone()) as Box<dyn SharedLogger>
                );
                #[cfg(feature = "termcolor")]
                vec.push(TermLogger::new(
                    LevelFilter::Info,
                    conf.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ) as Box<dyn SharedLogger>);
                vec.push(WriteLogger::new(
                    LevelFilter::Info,
                    conf.clone(),
                    File::create(&format!("info_{}.log", i)).unwrap(),
                ) as Box<dyn SharedLogger>);
                #[cfg(feature = "test")]
                vec.push(TestLogger::new(LevelFilter::Info, conf.clone()));

                //debug
                vec.push(
                    SimpleLogger::new(LevelFilter::Debug, conf.clone()) as Box<dyn SharedLogger>
                );
                #[cfg(feature = "termcolor")]
                vec.push(TermLogger::new(
                    LevelFilter::Debug,
                    conf.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ) as Box<dyn SharedLogger>);
                vec.push(WriteLogger::new(
                    LevelFilter::Debug,
                    conf.clone(),
                    File::create(&format!("debug_{}.log", i)).unwrap(),
                ) as Box<dyn SharedLogger>);
                #[cfg(feature = "test")]
                vec.push(TestLogger::new(LevelFilter::Debug, conf.clone()));

                //trace
                vec.push(
                    SimpleLogger::new(LevelFilter::Trace, conf.clone()) as Box<dyn SharedLogger>
                );
                #[cfg(feature = "termcolor")]
                vec.push(TermLogger::new(
                    LevelFilter::Trace,
                    conf.clone(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ) as Box<dyn SharedLogger>);
                vec.push(WriteLogger::new(
                    LevelFilter::Trace,
                    conf.clone(),
                    File::create(&format!("trace_{}.log", i)).unwrap(),
                ) as Box<dyn SharedLogger>);
                #[cfg(feature = "test")]
                vec.push(TestLogger::new(LevelFilter::Trace, conf.clone()));
            }

            vec
        })
        .unwrap();

        error!("Test Error");
        warn!("Test Warning");
        info!("Test Information");
        debug!("Test Debug");
        trace!("Test Trace");

        let mut thread_naming = String::new();
        File::open("thread_naming.log")
            .unwrap()
            .read_to_string(&mut thread_naming)
            .unwrap();

        if let Some(name) = std::thread::current().name() {
            assert!(thread_naming.contains(&format!("({})", name)));
        }

        for j in 1..i {
            let mut error = String::new();
            File::open(&format!("error_{}.log", j))
                .unwrap()
                .read_to_string(&mut error)
                .unwrap();
            let mut warn = String::new();
            File::open(&format!("warn_{}.log", j))
                .unwrap()
                .read_to_string(&mut warn)
                .unwrap();
            let mut info = String::new();
            File::open(&format!("info_{}.log", j))
                .unwrap()
                .read_to_string(&mut info)
                .unwrap();
            let mut debug = String::new();
            File::open(&format!("debug_{}.log", j))
                .unwrap()
                .read_to_string(&mut debug)
                .unwrap();
            let mut trace = String::new();
            File::open(&format!("trace_{}.log", j))
                .unwrap()
                .read_to_string(&mut trace)
                .unwrap();

            assert!(error.contains("Test Error"));
            assert!(!error.contains("Test Warning"));
            assert!(!error.contains("Test Information"));
            assert!(!error.contains("Test Debug"));
            assert!(!error.contains("Test Trace"));

            assert!(warn.contains("Test Error"));
            assert!(warn.contains("Test Warning"));
            assert!(!warn.contains("Test Information"));
            assert!(!warn.contains("Test Debug"));
            assert!(!warn.contains("Test Trace"));

            assert!(info.contains("Test Error"));
            assert!(info.contains("Test Warning"));
            assert!(info.contains("Test Information"));
            assert!(!info.contains("Test Debug"));
            assert!(!info.contains("Test Trace"));

            assert!(debug.contains("Test Error"));
            assert!(debug.contains("Test Warning"));
            assert!(debug.contains("Test Information"));
            assert!(debug.contains("Test Debug"));
            assert!(!debug.contains("Test Trace"));

            assert!(trace.contains("Test Error"));
            assert!(trace.contains("Test Warning"));
            assert!(trace.contains("Test Information"));
            assert!(trace.contains("Test Debug"));
            assert!(trace.contains("Test Trace"));
        }
    }
}
