//! Filtering for log records.
//!
//! This module contains the log filtering used by `env_logger` to match records.
//! You can use the `Filter` type in your own logger implementation to use the same
//! filter parsing and matching as `env_logger`. For more details about the format
//! for directive strings see [Enabling Logging].
//!
//! ## Using `env_logger` in your own logger
//!
//! You can use `env_logger`'s filtering functionality with your own logger.
//! Call [`Builder::parse`] to parse directives from a string when constructing
//! your logger. Call [`Filter::matches`] to check whether a record should be
//! logged based on the parsed filters when log records are received.
//!
//! ```
//! extern crate log;
//! extern crate env_logger;
//! use env_logger::filter::Filter;
//! use log::{Log, Metadata, Record};
//!
//! struct MyLogger {
//!     filter: Filter
//! }
//!
//! impl MyLogger {
//!     fn new() -> MyLogger {
//!         use env_logger::filter::Builder;
//!         let mut builder = Builder::new();
//!
//!         // Parse a directives string from an environment variable
//!         if let Ok(ref filter) = std::env::var("MY_LOG_LEVEL") {
//!            builder.parse(filter);
//!         }
//!
//!         MyLogger {
//!             filter: builder.build()
//!         }
//!     }
//! }
//!
//! impl Log for MyLogger {
//!     fn enabled(&self, metadata: &Metadata) -> bool {
//!         self.filter.enabled(metadata)
//!     }
//!
//!     fn log(&self, record: &Record) {
//!         // Check if the record is matched by the filter
//!         if self.filter.matches(record) {
//!             println!("{:?}", record);
//!         }
//!     }
//!
//!     fn flush(&self) {}
//! }
//! ```
//!
//! [Enabling Logging]: ../index.html#enabling-logging
//! [`Builder::parse`]: struct.Builder.html#method.parse
//! [`Filter::matches`]: struct.Filter.html#method.matches

use log::{Level, LevelFilter, Metadata, Record};
use std::env;
use std::fmt;
use std::mem;

#[cfg(feature = "regex")]
#[path = "regex.rs"]
mod inner;

#[cfg(not(feature = "regex"))]
#[path = "string.rs"]
mod inner;

/// A builder for a log filter.
///
/// It can be used to parse a set of directives from a string before building
/// a [`Filter`] instance.
///
/// ## Example
///
/// ```
/// # #[macro_use] extern crate log;
/// # use std::env;
/// use env_logger::filter::Builder;
///
/// let mut builder = Builder::new();
///
/// // Parse a logging filter from an environment variable.
/// if let Ok(rust_log) = env::var("RUST_LOG") {
///     builder.parse(&rust_log);
/// }
///
/// let filter = builder.build();
/// ```
///
/// [`Filter`]: struct.Filter.html
pub struct Builder {
    directives: Vec<Directive>,
    filter: Option<inner::Filter>,
    built: bool,
}

impl Builder {
    /// Initializes the filter builder with defaults.
    pub fn new() -> Builder {
        Builder {
            directives: Vec::new(),
            filter: None,
            built: false,
        }
    }

    /// Initializes the filter builder from an environment.
    pub fn from_env(env: &str) -> Builder {
        let mut builder = Builder::new();

        if let Ok(s) = env::var(env) {
            builder.parse(&s);
        }

        builder
    }

    /// Insert the directive replacing any directive with the same name.
    fn insert_directive(&mut self, mut directive: Directive) {
        if let Some(pos) = self
            .directives
            .iter()
            .position(|d| d.name == directive.name)
        {
            mem::swap(&mut self.directives[pos], &mut directive);
        } else {
            self.directives.push(directive);
        }
    }

    /// Adds a directive to the filter for a specific module.
    pub fn filter_module(&mut self, module: &str, level: LevelFilter) -> &mut Self {
        self.filter(Some(module), level)
    }

    /// Adds a directive to the filter for all modules.
    pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self {
        self.filter(None, level)
    }

    /// Adds a directive to the filter.
    ///
    /// The given module (if any) will log at most the specified level provided.
    /// If no module is provided then the filter will apply to all log messages.
    pub fn filter(&mut self, module: Option<&str>, level: LevelFilter) -> &mut Self {
        self.insert_directive(Directive {
            name: module.map(|s| s.to_string()),
            level,
        });
        self
    }

    /// Parses the directives string.
    ///
    /// See the [Enabling Logging] section for more details.
    ///
    /// [Enabling Logging]: ../index.html#enabling-logging
    pub fn parse(&mut self, filters: &str) -> &mut Self {
        let (directives, filter) = parse_spec(filters);

        self.filter = filter;

        for directive in directives {
            self.insert_directive(directive);
        }
        self
    }

    /// Build a log filter.
    pub fn build(&mut self) -> Filter {
        assert!(!self.built, "attempt to re-use consumed builder");
        self.built = true;

        let mut directives = Vec::new();
        if self.directives.is_empty() {
            // Adds the default filter if none exist
            directives.push(Directive {
                name: None,
                level: LevelFilter::Error,
            });
        } else {
            // Consume directives.
            directives = mem::take(&mut self.directives);
            // Sort the directives by length of their name, this allows a
            // little more efficient lookup at runtime.
            directives.sort_by(|a, b| {
                let alen = a.name.as_ref().map(|a| a.len()).unwrap_or(0);
                let blen = b.name.as_ref().map(|b| b.len()).unwrap_or(0);
                alen.cmp(&blen)
            });
        }

        Filter {
            directives: mem::take(&mut directives),
            filter: mem::take(&mut self.filter),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.built {
            f.debug_struct("Filter").field("built", &true).finish()
        } else {
            f.debug_struct("Filter")
                .field("filter", &self.filter)
                .field("directives", &self.directives)
                .finish()
        }
    }
}

#[derive(Debug)]
struct Directive {
    name: Option<String>,
    level: LevelFilter,
}

/// A log filter.
///
/// This struct can be used to determine whether or not a log record
/// should be written to the output.
/// Use the [`Builder`] type to parse and construct a `Filter`.
///
/// [`Builder`]: struct.Builder.html
pub struct Filter {
    directives: Vec<Directive>,
    filter: Option<inner::Filter>,
}

impl Filter {
    /// Returns the maximum `LevelFilter` that this filter instance is
    /// configured to output.
    ///
    /// # Example
    ///
    /// ```rust
    /// use log::LevelFilter;
    /// use env_logger::filter::Builder;
    ///
    /// let mut builder = Builder::new();
    /// builder.filter(Some("module1"), LevelFilter::Info);
    /// builder.filter(Some("module2"), LevelFilter::Error);
    ///
    /// let filter = builder.build();
    /// assert_eq!(filter.filter(), LevelFilter::Info);
    /// ```
    pub fn filter(&self) -> LevelFilter {
        self.directives
            .iter()
            .map(|d| d.level)
            .max()
            .unwrap_or(LevelFilter::Off)
    }

    /// Checks if this record matches the configured filter.
    pub fn matches(&self, record: &Record) -> bool {
        if !self.enabled(record.metadata()) {
            return false;
        }

        if let Some(filter) = self.filter.as_ref() {
            if !filter.is_match(&record.args().to_string()) {
                return false;
            }
        }

        true
    }

    /// Determines if a log message with the specified metadata would be logged.
    pub fn enabled(&self, metadata: &Metadata) -> bool {
        let level = metadata.level();
        let target = metadata.target();

        enabled(&self.directives, level, target)
    }
}

impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter")
            .field("filter", &self.filter)
            .field("directives", &self.directives)
            .finish()
    }
}

/// Parse a logging specification string (e.g: "crate1,crate2::mod3,crate3::x=error/foo")
/// and return a vector with log directives.
fn parse_spec(spec: &str) -> (Vec<Directive>, Option<inner::Filter>) {
    let mut dirs = Vec::new();

    let mut parts = spec.split('/');
    let mods = parts.next();
    let filter = parts.next();
    if parts.next().is_some() {
        eprintln!(
            "warning: invalid logging spec '{}', \
             ignoring it (too many '/'s)",
            spec
        );
        return (dirs, None);
    }
    if let Some(m) = mods {
        for s in m.split(',').map(|ss| ss.trim()) {
            if s.is_empty() {
                continue;
            }
            let mut parts = s.split('=');
            let (log_level, name) =
                match (parts.next(), parts.next().map(|s| s.trim()), parts.next()) {
                    (Some(part0), None, None) => {
                        // if the single argument is a log-level string or number,
                        // treat that as a global fallback
                        match part0.parse() {
                            Ok(num) => (num, None),
                            Err(_) => (LevelFilter::max(), Some(part0)),
                        }
                    }
                    (Some(part0), Some(""), None) => (LevelFilter::max(), Some(part0)),
                    (Some(part0), Some(part1), None) => match part1.parse() {
                        Ok(num) => (num, Some(part0)),
                        _ => {
                            eprintln!(
                                "warning: invalid logging spec '{}', \
                                 ignoring it",
                                part1
                            );
                            continue;
                        }
                    },
                    _ => {
                        eprintln!(
                            "warning: invalid logging spec '{}', \
                             ignoring it",
                            s
                        );
                        continue;
                    }
                };
            dirs.push(Directive {
                name: name.map(|s| s.to_string()),
                level: log_level,
            });
        }
    }

    let filter = filter.and_then(|filter| match inner::Filter::new(filter) {
        Ok(re) => Some(re),
        Err(e) => {
            eprintln!("warning: invalid regex filter - {}", e);
            None
        }
    });

    (dirs, filter)
}

// Check whether a level and target are enabled by the set of directives.
fn enabled(directives: &[Directive], level: Level, target: &str) -> bool {
    // Search for the longest match, the vector is assumed to be pre-sorted.
    for directive in directives.iter().rev() {
        match directive.name {
            Some(ref name) if !target.starts_with(&**name) => {}
            Some(..) | None => return level <= directive.level,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use log::{Level, LevelFilter};

    use super::{enabled, parse_spec, Builder, Directive, Filter};

    fn make_logger_filter(dirs: Vec<Directive>) -> Filter {
        let mut logger = Builder::new().build();
        logger.directives = dirs;
        logger
    }

    #[test]
    fn filter_info() {
        let logger = Builder::new().filter(None, LevelFilter::Info).build();
        assert!(enabled(&logger.directives, Level::Info, "crate1"));
        assert!(!enabled(&logger.directives, Level::Debug, "crate1"));
    }

    #[test]
    fn filter_beginning_longest_match() {
        let logger = Builder::new()
            .filter(Some("crate2"), LevelFilter::Info)
            .filter(Some("crate2::mod"), LevelFilter::Debug)
            .filter(Some("crate1::mod1"), LevelFilter::Warn)
            .build();
        assert!(enabled(&logger.directives, Level::Debug, "crate2::mod1"));
        assert!(!enabled(&logger.directives, Level::Debug, "crate2"));
    }

    // Some of our tests are only correct or complete when they cover the full
    // universe of variants for log::Level. In the unlikely event that a new
    // variant is added in the future, this test will detect the scenario and
    // alert us to the need to review and update the tests. In such a
    // situation, this test will fail to compile, and the error message will
    // look something like this:
    //
    //     error[E0004]: non-exhaustive patterns: `NewVariant` not covered
    //        --> src/filter/mod.rs:413:15
    //         |
    //     413 |         match level_universe {
    //         |               ^^^^^^^^^^^^^^ pattern `NewVariant` not covered
    #[test]
    fn ensure_tests_cover_level_universe() {
        let level_universe: Level = Level::Trace; // use of trace variant is arbitrary
        match level_universe {
            Level::Error | Level::Warn | Level::Info | Level::Debug | Level::Trace => (),
        }
    }

    #[test]
    fn parse_default() {
        let logger = Builder::new().parse("info,crate1::mod1=warn").build();
        assert!(enabled(&logger.directives, Level::Warn, "crate1::mod1"));
        assert!(enabled(&logger.directives, Level::Info, "crate2::mod2"));
    }

    #[test]
    fn parse_default_bare_level_off_lc() {
        let logger = Builder::new().parse("off").build();
        assert!(!enabled(&logger.directives, Level::Error, ""));
        assert!(!enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_off_uc() {
        let logger = Builder::new().parse("OFF").build();
        assert!(!enabled(&logger.directives, Level::Error, ""));
        assert!(!enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_error_lc() {
        let logger = Builder::new().parse("error").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(!enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_error_uc() {
        let logger = Builder::new().parse("ERROR").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(!enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_warn_lc() {
        let logger = Builder::new().parse("warn").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_warn_uc() {
        let logger = Builder::new().parse("WARN").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(!enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_info_lc() {
        let logger = Builder::new().parse("info").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_info_uc() {
        let logger = Builder::new().parse("INFO").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(!enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_debug_lc() {
        let logger = Builder::new().parse("debug").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_debug_uc() {
        let logger = Builder::new().parse("DEBUG").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(enabled(&logger.directives, Level::Debug, ""));
        assert!(!enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_trace_lc() {
        let logger = Builder::new().parse("trace").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(enabled(&logger.directives, Level::Debug, ""));
        assert!(enabled(&logger.directives, Level::Trace, ""));
    }

    #[test]
    fn parse_default_bare_level_trace_uc() {
        let logger = Builder::new().parse("TRACE").build();
        assert!(enabled(&logger.directives, Level::Error, ""));
        assert!(enabled(&logger.directives, Level::Warn, ""));
        assert!(enabled(&logger.directives, Level::Info, ""));
        assert!(enabled(&logger.directives, Level::Debug, ""));
        assert!(enabled(&logger.directives, Level::Trace, ""));
    }

    // In practice, the desired log level is typically specified by a token
    // that is either all lowercase (e.g., 'trace') or all uppercase (.e.g,
    // 'TRACE'), but this tests serves as a reminder that
    // log::Level::from_str() ignores all case variants.
    #[test]
    fn parse_default_bare_level_debug_mixed() {
        {
            let logger = Builder::new().parse("Debug").build();
            assert!(enabled(&logger.directives, Level::Error, ""));
            assert!(enabled(&logger.directives, Level::Warn, ""));
            assert!(enabled(&logger.directives, Level::Info, ""));
            assert!(enabled(&logger.directives, Level::Debug, ""));
            assert!(!enabled(&logger.directives, Level::Trace, ""));
        }
        {
            let logger = Builder::new().parse("debuG").build();
            assert!(enabled(&logger.directives, Level::Error, ""));
            assert!(enabled(&logger.directives, Level::Warn, ""));
            assert!(enabled(&logger.directives, Level::Info, ""));
            assert!(enabled(&logger.directives, Level::Debug, ""));
            assert!(!enabled(&logger.directives, Level::Trace, ""));
        }
        {
            let logger = Builder::new().parse("deBug").build();
            assert!(enabled(&logger.directives, Level::Error, ""));
            assert!(enabled(&logger.directives, Level::Warn, ""));
            assert!(enabled(&logger.directives, Level::Info, ""));
            assert!(enabled(&logger.directives, Level::Debug, ""));
            assert!(!enabled(&logger.directives, Level::Trace, ""));
        }
        {
            let logger = Builder::new().parse("DeBuG").build(); // LaTeX flavor!
            assert!(enabled(&logger.directives, Level::Error, ""));
            assert!(enabled(&logger.directives, Level::Warn, ""));
            assert!(enabled(&logger.directives, Level::Info, ""));
            assert!(enabled(&logger.directives, Level::Debug, ""));
            assert!(!enabled(&logger.directives, Level::Trace, ""));
        }
    }

    #[test]
    fn match_full_path() {
        let logger = make_logger_filter(vec![
            Directive {
                name: Some("crate2".to_string()),
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Warn,
            },
        ]);
        assert!(enabled(&logger.directives, Level::Warn, "crate1::mod1"));
        assert!(!enabled(&logger.directives, Level::Info, "crate1::mod1"));
        assert!(enabled(&logger.directives, Level::Info, "crate2"));
        assert!(!enabled(&logger.directives, Level::Debug, "crate2"));
    }

    #[test]
    fn no_match() {
        let logger = make_logger_filter(vec![
            Directive {
                name: Some("crate2".to_string()),
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Warn,
            },
        ]);
        assert!(!enabled(&logger.directives, Level::Warn, "crate3"));
    }

    #[test]
    fn match_beginning() {
        let logger = make_logger_filter(vec![
            Directive {
                name: Some("crate2".to_string()),
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Warn,
            },
        ]);
        assert!(enabled(&logger.directives, Level::Info, "crate2::mod1"));
    }

    #[test]
    fn match_beginning_longest_match() {
        let logger = make_logger_filter(vec![
            Directive {
                name: Some("crate2".to_string()),
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate2::mod".to_string()),
                level: LevelFilter::Debug,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Warn,
            },
        ]);
        assert!(enabled(&logger.directives, Level::Debug, "crate2::mod1"));
        assert!(!enabled(&logger.directives, Level::Debug, "crate2"));
    }

    #[test]
    fn match_default() {
        let logger = make_logger_filter(vec![
            Directive {
                name: None,
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Warn,
            },
        ]);
        assert!(enabled(&logger.directives, Level::Warn, "crate1::mod1"));
        assert!(enabled(&logger.directives, Level::Info, "crate2::mod2"));
    }

    #[test]
    fn zero_level() {
        let logger = make_logger_filter(vec![
            Directive {
                name: None,
                level: LevelFilter::Info,
            },
            Directive {
                name: Some("crate1::mod1".to_string()),
                level: LevelFilter::Off,
            },
        ]);
        assert!(!enabled(&logger.directives, Level::Error, "crate1::mod1"));
        assert!(enabled(&logger.directives, Level::Info, "crate2::mod2"));
    }

    #[test]
    fn parse_spec_valid() {
        let (dirs, filter) = parse_spec("crate1::mod1=error,crate1::mod2,crate2=debug");
        assert_eq!(dirs.len(), 3);
        assert_eq!(dirs[0].name, Some("crate1::mod1".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Error);

        assert_eq!(dirs[1].name, Some("crate1::mod2".to_string()));
        assert_eq!(dirs[1].level, LevelFilter::max());

        assert_eq!(dirs[2].name, Some("crate2".to_string()));
        assert_eq!(dirs[2].level, LevelFilter::Debug);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_invalid_crate() {
        // test parse_spec with multiple = in specification
        let (dirs, filter) = parse_spec("crate1::mod1=warn=info,crate2=debug");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate2".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Debug);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_invalid_level() {
        // test parse_spec with 'noNumber' as log level
        let (dirs, filter) = parse_spec("crate1::mod1=noNumber,crate2=debug");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate2".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Debug);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_string_level() {
        // test parse_spec with 'warn' as log level
        let (dirs, filter) = parse_spec("crate1::mod1=wrong,crate2=warn");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate2".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Warn);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_empty_level() {
        // test parse_spec with '' as log level
        let (dirs, filter) = parse_spec("crate1::mod1=wrong,crate2=");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate2".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::max());
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_empty_level_isolated() {
        // test parse_spec with "" as log level (and the entire spec str)
        let (dirs, filter) = parse_spec(""); // should be ignored
        assert_eq!(dirs.len(), 0);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_blank_level_isolated() {
        // test parse_spec with a white-space-only string specified as the log
        // level (and the entire spec str)
        let (dirs, filter) = parse_spec("     "); // should be ignored
        assert_eq!(dirs.len(), 0);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_blank_level_isolated_comma_only() {
        // The spec should contain zero or more comma-separated string slices,
        // so a comma-only string should be interpreted as two empty strings
        // (which should both be treated as invalid, so ignored).
        let (dirs, filter) = parse_spec(","); // should be ignored
        assert_eq!(dirs.len(), 0);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_blank_level_isolated_comma_blank() {
        // The spec should contain zero or more comma-separated string slices,
        // so this bogus spec should be interpreted as containing one empty
        // string and one blank string. Both should both be treated as
        // invalid, so ignored.
        let (dirs, filter) = parse_spec(",     "); // should be ignored
        assert_eq!(dirs.len(), 0);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_blank_level_isolated_blank_comma() {
        // The spec should contain zero or more comma-separated string slices,
        // so this bogus spec should be interpreted as containing one blank
        // string and one empty string. Both should both be treated as
        // invalid, so ignored.
        let (dirs, filter) = parse_spec("     ,"); // should be ignored
        assert_eq!(dirs.len(), 0);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_global() {
        // test parse_spec with no crate
        let (dirs, filter) = parse_spec("warn,crate2=debug");
        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs[0].name, None);
        assert_eq!(dirs[0].level, LevelFilter::Warn);
        assert_eq!(dirs[1].name, Some("crate2".to_string()));
        assert_eq!(dirs[1].level, LevelFilter::Debug);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_global_bare_warn_lc() {
        // test parse_spec with no crate, in isolation, all lowercase
        let (dirs, filter) = parse_spec("warn");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, None);
        assert_eq!(dirs[0].level, LevelFilter::Warn);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_global_bare_warn_uc() {
        // test parse_spec with no crate, in isolation, all uppercase
        let (dirs, filter) = parse_spec("WARN");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, None);
        assert_eq!(dirs[0].level, LevelFilter::Warn);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_global_bare_warn_mixed() {
        // test parse_spec with no crate, in isolation, mixed case
        let (dirs, filter) = parse_spec("wArN");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, None);
        assert_eq!(dirs[0].level, LevelFilter::Warn);
        assert!(filter.is_none());
    }

    #[test]
    fn parse_spec_valid_filter() {
        let (dirs, filter) = parse_spec("crate1::mod1=error,crate1::mod2,crate2=debug/abc");
        assert_eq!(dirs.len(), 3);
        assert_eq!(dirs[0].name, Some("crate1::mod1".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Error);

        assert_eq!(dirs[1].name, Some("crate1::mod2".to_string()));
        assert_eq!(dirs[1].level, LevelFilter::max());

        assert_eq!(dirs[2].name, Some("crate2".to_string()));
        assert_eq!(dirs[2].level, LevelFilter::Debug);
        assert!(filter.is_some() && filter.unwrap().to_string() == "abc");
    }

    #[test]
    fn parse_spec_invalid_crate_filter() {
        let (dirs, filter) = parse_spec("crate1::mod1=error=warn,crate2=debug/a.c");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate2".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::Debug);
        assert!(filter.is_some() && filter.unwrap().to_string() == "a.c");
    }

    #[test]
    fn parse_spec_empty_with_filter() {
        let (dirs, filter) = parse_spec("crate1/a*c");
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, Some("crate1".to_string()));
        assert_eq!(dirs[0].level, LevelFilter::max());
        assert!(filter.is_some() && filter.unwrap().to_string() == "a*c");
    }
}
