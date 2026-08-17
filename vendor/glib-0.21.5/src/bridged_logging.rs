// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{gstr, log as glib_log, log_structured_array, translate::*, LogField};

// rustdoc-stripper-ignore-next
/// Enumeration of the possible formatting behaviours for a
/// [`GlibLogger`](struct.GlibLogger.html).
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlibLoggerFormat {
    // rustdoc-stripper-ignore-next
    /// A simple format, writing only the message on output.
    Plain,
    // rustdoc-stripper-ignore-next
    /// A simple format, writing file, line and message on output.
    LineAndFile,
    // rustdoc-stripper-ignore-next
    /// A logger using glib structured logging.
    Structured,
}

// rustdoc-stripper-ignore-next
/// Enumeration of the possible domain handling behaviours for a
/// [`GlibLogger`](struct.GlibLogger.html).
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlibLoggerDomain {
    // rustdoc-stripper-ignore-next
    /// Logs will have no domain specified.
    None,
    // rustdoc-stripper-ignore-next
    /// Logs will use the `target` of the log crate as a domain; this allows
    /// Rust code, like `warn!(target: "my-domain", "...");` to log to the glib
    /// logger using the specified domain.
    CrateTarget,
    // rustdoc-stripper-ignore-next
    /// Logs will use the crate path as the log domain.
    CratePath,
}

// rustdoc-stripper-ignore-next
/// An implementation of a [`log`](https://crates.io/crates/log) compatible
/// logger which logs over glib logging facilities.
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
///
/// Use this if you want to use glib as the main logging output in your application,
/// and want to route all logging happening through the log crate to glib logging.
/// If you want the opposite, see
/// [`rust_log_handler`](fn.rust_log_handler.html).
///
/// NOTE: This should never be used when
/// [`rust_log_handler`](fn.rust_log_handler.html) has
/// been registered as a default glib log handler, otherwise a stack overflow
/// will occur.
///
/// Example:
///
/// ```no_compile
/// static glib_logger: glib::GlibLogger = glib::GlibLogger::new(
///     glib::GlibLoggerFormat::Plain,
///     glib::GlibLoggerDomain::CrateTarget,
/// );
///
/// log::set_logger(&glib_logger);
/// log::set_max_level(log::LevelFilter::Debug);
///
/// log::info!("This line will get logged by glib");
/// ```
#[derive(Debug)]
pub struct GlibLogger {
    format: GlibLoggerFormat,
    domain: GlibLoggerDomain,
}

impl GlibLogger {
    // rustdoc-stripper-ignore-next
    /// Creates a new instance of [`GlibLogger`](struct.GlibLogger.html).
    /// See documentation of [`GlibLogger`](struct.GlibLogger.html) for more
    /// information.
    ///
    /// Example:
    ///
    /// ```no_compile
    /// static glib_logger: glib::GlibLogger = glib::GlibLogger::new(
    ///     glib::GlibLoggerFormat::Plain,
    ///     glib::GlibLoggerDomain::CrateTarget,
    /// );
    ///
    /// log::set_logger(&glib_logger);
    /// log::set_max_level(log::LevelFilter::Debug);
    ///
    /// log::info!("This line will get logged by glib");
    /// ```
    pub const fn new(format: GlibLoggerFormat, domain: GlibLoggerDomain) -> Self {
        Self { format, domain }
    }

    fn level_to_glib(level: rs_log::Level) -> crate::LogLevel {
        match level {
            // Errors are mapped to critical to avoid automatic termination
            rs_log::Level::Error => crate::LogLevel::Critical,
            rs_log::Level::Warn => crate::LogLevel::Warning,
            rs_log::Level::Info => crate::LogLevel::Info,
            rs_log::Level::Debug => crate::LogLevel::Debug,
            // There is no equivalent to trace level in glib
            rs_log::Level::Trace => crate::LogLevel::Debug,
        }
    }

    #[doc(alias = "g_log")]
    fn write_log(domain: Option<&str>, level: rs_log::Level, message: &std::fmt::Arguments<'_>) {
        unsafe {
            use std::fmt::Write;

            let mut message_builder = crate::GStringBuilder::default();
            if write!(&mut message_builder, "{}", message).is_err() {
                return;
            }
            let message = message_builder.into_string();

            crate::ffi::g_log(
                domain.to_glib_none().0,
                GlibLogger::level_to_glib(level).into_glib(),
                b"%s\0".as_ptr() as *const _,
                ToGlibPtr::<*const std::os::raw::c_char>::to_glib_none(&message).0,
            );
        }
    }

    fn write_log_structured(
        domain: Option<&str>,
        level: rs_log::Level,
        file: Option<&str>,
        line: Option<u32>,
        func: Option<&str>,
        message: &str,
    ) {
        // Write line number into a static array to avoid allocating its string
        // representation. 16 bytes allow 10^15 lines, which should be more than
        // sufficient.
        let mut line_buffer = [0u8; 16];
        let line = {
            use std::io::{Cursor, Write};
            let mut c = Cursor::new(line_buffer.as_mut_slice());
            match line {
                Some(lineno) => write!(&mut c, "{lineno}").ok(),
                None => write!(&mut c, "<unknown line>").ok(),
            };
            let pos = c.position() as usize;
            &line_buffer[..pos]
        };
        let glib_level = GlibLogger::level_to_glib(level);
        let fields = [
            LogField::new(gstr!("PRIORITY"), glib_level.priority().as_bytes()),
            LogField::new(
                gstr!("CODE_FILE"),
                file.unwrap_or("<unknown file>").as_bytes(),
            ),
            LogField::new(gstr!("CODE_LINE"), line),
            LogField::new(
                gstr!("CODE_FUNC"),
                func.unwrap_or("<unknown module path>").as_bytes(),
            ),
            LogField::new(gstr!("MESSAGE"), message.as_bytes()),
            LogField::new(gstr!("GLIB_DOMAIN"), domain.unwrap_or("default").as_bytes()),
        ];
        log_structured_array(glib_level, &fields);
    }
}

impl rs_log::Log for GlibLogger {
    fn enabled(&self, _: &rs_log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &rs_log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let domain = match &self.domain {
            GlibLoggerDomain::None => None,
            GlibLoggerDomain::CrateTarget => Some(record.metadata().target()),
            GlibLoggerDomain::CratePath => record.module_path(),
        };

        match self.format {
            GlibLoggerFormat::Plain => {
                GlibLogger::write_log(domain, record.level(), record.args());
            }
            GlibLoggerFormat::LineAndFile => {
                match (record.file(), record.line()) {
                    (Some(file), Some(line)) => {
                        GlibLogger::write_log(
                            domain,
                            record.level(),
                            &format_args!("{}:{}: {}", file, line, record.args()),
                        );
                    }
                    (Some(file), None) => {
                        GlibLogger::write_log(
                            domain,
                            record.level(),
                            &format_args!("{}: {}", file, record.args()),
                        );
                    }
                    _ => {
                        GlibLogger::write_log(domain, record.level(), record.args());
                    }
                };
            }
            GlibLoggerFormat::Structured => {
                let args = record.args();
                let message = if let Some(s) = args.as_str() {
                    s
                } else {
                    &args.to_string()
                };
                GlibLogger::write_log_structured(
                    domain,
                    record.level(),
                    record.file(),
                    record.line(),
                    record.module_path(),
                    message,
                );
            }
        };
    }

    fn flush(&self) {}
}

// rustdoc-stripper-ignore-next
/// Provides a glib log handler which routes all logging messages to the
/// [`log crate`](https://crates.io/crates/log).
///
/// In order to use this function, `glib` must be built with the `log` feature
/// enabled.
///
/// Use this function if you want to use the log crate as the main logging
/// output in your application, and want to route all logging happening in
/// glib to the log crate. If you want the opposite, use [`GlibLogger`](struct.GlibLogger.html).
///
/// NOTE: This should never be used when [`GlibLogger`](struct.GlibLogger.html) is
/// registered as a logger, otherwise a stack overflow will occur.
///
/// ```no_run
/// glib::log_set_default_handler(glib::rust_log_handler);
/// ```
pub fn rust_log_handler(domain: Option<&str>, level: glib_log::LogLevel, message: &str) {
    let level = match level {
        glib_log::LogLevel::Error | glib_log::LogLevel::Critical => rs_log::Level::Error,
        glib_log::LogLevel::Warning => rs_log::Level::Warn,
        glib_log::LogLevel::Message | glib_log::LogLevel::Info => rs_log::Level::Info,
        glib_log::LogLevel::Debug => rs_log::Level::Debug,
    };

    rs_log::log!(target: domain.unwrap_or("<null>"), level, "{}", message);
}

// rustdoc-stripper-ignore-next
/// A macro which behaves exactly as `log::error!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// glib::error!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(docsrs, feature = "log_macros"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::rs_log::log!(target: $target, $crate::rs_log::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::rs_log::log!(target: G_LOG_DOMAIN, $crate::rs_log::Level::Error, $($arg)+);
    )
}

// rustdoc-stripper-ignore-next
/// A macro which behaves exactly as `log::warn!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// glib::warn!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(docsrs, feature = "log_macros"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::rs_log::log!(target: $target, $crate::rs_log::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::rs_log::log!(target: G_LOG_DOMAIN, $crate::rs_log::Level::Warn, $($arg)+);
    )
}

// rustdoc-stripper-ignore-next
/// A macro which behaves exactly as `log::info!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// glib::info!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(docsrs, feature = "log_macros"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::rs_log::log!(target: $target, $crate::rs_log::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::rs_log::log!(target: G_LOG_DOMAIN, $crate::rs_log::Level::Info, $($arg)+);
    )
}

// rustdoc-stripper-ignore-next
/// A macro which behaves exactly as `log::debug!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// glib::debug!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(docsrs, feature = "log_macros"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::rs_log::log!(target: $target, $crate::rs_log::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::rs_log::log!(target: G_LOG_DOMAIN, $crate::rs_log::Level::Debug, $($arg)+);
    )
}

// rustdoc-stripper-ignore-next
/// A macro which behaves exactly as `log::trace!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// glib::trace!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(docsrs, feature = "log_macros"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::rs_log::log!(target: $target, $crate::rs_log::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::rs_log::log!(target: G_LOG_DOMAIN, $crate::rs_log::Level::Trace, $($arg)+);
    )
}
