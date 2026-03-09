use std::borrow::Cow;

use log::{Level, Log, Record};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct LogRecord<'a> {
    pub level: usize,
    pub module_path: Option<Cow<'a, str>>,
    pub file: Option<Cow<'a, str>>,
    pub line: Option<u32>,
    pub message: Cow<'a, str>,
}

impl<'a> LogRecord<'a> {
    pub fn new(record: &'a Record<'a>) -> Self {
        Self {
            level: serialize_level(record.level()),
            module_path: record.module_path().map(Cow::Borrowed),
            file: record.file().map(Cow::Borrowed),
            line: record.line(),
            message: Cow::Owned(record.args().to_string()),
        }
    }

    pub fn log(&'a self, logger: &dyn Log) {
        if let Some(level) = deserialize_level(self.level) {
            logger.log(
                &log::Record::builder()
                    .module_path(self.module_path.as_deref())
                    .target("remote_server")
                    .args(format_args!("{}", self.message))
                    .file(self.file.as_deref())
                    .line(self.line)
                    .level(level)
                    .build(),
            )
        }
    }
}

fn serialize_level(level: Level) -> usize {
    match level {
        Level::Error => 1,
        Level::Warn => 2,
        Level::Info => 3,
        Level::Debug => 4,
        Level::Trace => 5,
    }
}

fn deserialize_level(level: usize) -> Option<Level> {
    match level {
        1 => Some(Level::Error),
        2 => Some(Level::Warn),
        3 => Some(Level::Info),
        4 => Some(Level::Debug),
        5 => Some(Level::Trace),
        _ => None,
    }
}
