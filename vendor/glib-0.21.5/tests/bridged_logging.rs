#![cfg(feature = "log")]

use std::sync::{Arc, Mutex};

use glib::LogLevel;
use rs_log::Log;

#[derive(Debug, PartialEq, Eq)]
struct LoggedEvent {
    level: LogLevel,
    fields: Vec<(String, Option<String>)>,
}

fn setup_log_collector() -> Arc<Mutex<Vec<LoggedEvent>>> {
    let events = Arc::new(Mutex::new(Vec::new()));
    let event_writer = events.clone();
    glib::log_set_writer_func(move |level, fields| {
        let fields = fields
            .iter()
            .map(|field| {
                (
                    field.key().to_string(),
                    field.value_str().map(|s| s.to_owned()),
                )
            })
            .collect();
        event_writer
            .lock()
            .unwrap()
            .push(LoggedEvent { level, fields });
        glib::LogWriterOutput::Handled
    });
    events
}

/// Test the glib Rust logger with different formats.
///
/// We put everything into one test because we can only set the log writer func once.
#[test]
fn glib_logger_formats() {
    let events = setup_log_collector();

    let record = rs_log::RecordBuilder::new()
        .target("test_target")
        .level(rs_log::Level::Info)
        .args(format_args!("test message"))
        .file(Some("/path/to/a/test/file.rs"))
        .line(Some(42))
        .module_path(Some("foo::bar"))
        .build();

    glib::GlibLogger::new(
        glib::GlibLoggerFormat::Plain,
        glib::GlibLoggerDomain::CrateTarget,
    )
    .log(&record);
    let event = events.lock().unwrap().pop().unwrap();
    assert_eq!(
        event,
        LoggedEvent {
            level: glib::LogLevel::Info,
            fields: vec![
                ("GLIB_OLD_LOG_API".to_string(), Some("1".to_string())),
                ("MESSAGE".to_string(), Some("test message".to_string())),
                ("PRIORITY".to_string(), Some("6".to_string())),
                ("GLIB_DOMAIN".to_string(), Some("test_target".to_string()))
            ]
        }
    );
    events.lock().unwrap().clear();

    glib::GlibLogger::new(
        glib::GlibLoggerFormat::LineAndFile,
        glib::GlibLoggerDomain::CrateTarget,
    )
    .log(&record);
    let event = events.lock().unwrap().pop().unwrap();
    assert_eq!(
        event,
        LoggedEvent {
            level: glib::LogLevel::Info,
            fields: vec![
                ("GLIB_OLD_LOG_API".to_string(), Some("1".to_string())),
                (
                    "MESSAGE".to_string(),
                    Some("/path/to/a/test/file.rs:42: test message".to_string())
                ),
                ("PRIORITY".to_string(), Some("6".to_string())),
                ("GLIB_DOMAIN".to_string(), Some("test_target".to_string()))
            ]
        }
    );

    glib::GlibLogger::new(
        glib::GlibLoggerFormat::Structured,
        glib::GlibLoggerDomain::CrateTarget,
    )
    .log(&record);
    let event = events.lock().unwrap().pop().unwrap();
    assert_eq!(
        event,
        LoggedEvent {
            level: glib::LogLevel::Info,
            fields: vec![
                ("PRIORITY".to_string(), Some("6".to_string())),
                (
                    "CODE_FILE".to_string(),
                    Some("/path/to/a/test/file.rs".to_string())
                ),
                ("CODE_LINE".to_string(), Some("42".to_string())),
                ("CODE_FUNC".to_string(), Some("foo::bar".to_string())),
                ("MESSAGE".to_string(), Some("test message".to_string())),
                ("GLIB_DOMAIN".to_string(), Some("test_target".to_string()))
            ]
        }
    );

    // Structured logging without location fields
    glib::GlibLogger::new(
        glib::GlibLoggerFormat::Structured,
        glib::GlibLoggerDomain::CrateTarget,
    )
    .log(
        &rs_log::RecordBuilder::new()
            .target("test_target")
            .level(rs_log::Level::Info)
            .args(format_args!("test message"))
            .build(),
    );
    let event = events.lock().unwrap().pop().unwrap();
    assert_eq!(
        event,
        LoggedEvent {
            level: glib::LogLevel::Info,
            fields: vec![
                ("PRIORITY".to_string(), Some("6".to_string())),
                ("CODE_FILE".to_string(), Some("<unknown file>".to_string())),
                ("CODE_LINE".to_string(), Some("<unknown line>".to_string())),
                (
                    "CODE_FUNC".to_string(),
                    Some("<unknown module path>".to_string())
                ),
                ("MESSAGE".to_string(), Some("test message".to_string())),
                ("GLIB_DOMAIN".to_string(), Some("test_target".to_string()))
            ]
        }
    );
}
