#[cfg(feature = "termcolor")]
use log::Level;
use log::LevelFilter;

use std::borrow::Cow;
#[cfg(feature = "termcolor")]
use termcolor::Color;
pub use time::{format_description::FormatItem, macros::format_description, UtcOffset};

#[derive(Debug, Clone, Copy)]
/// Padding to be used for logging the level
pub enum LevelPadding {
    /// Add spaces on the left side
    Left,
    /// Add spaces on the right side
    Right,
    /// Do not pad the level
    Off,
}

#[derive(Debug, Clone, Copy)]
/// Padding to be used for logging the thread id/name
pub enum ThreadPadding {
    /// Add spaces on the left side, up to usize many
    Left(usize),
    /// Add spaces on the right side, up to usize many
    Right(usize),
    /// Do not pad the thread id/name
    Off,
}

#[derive(Debug, Clone, Copy)]
/// Padding to be used for logging the thread id/name
pub enum TargetPadding {
    /// Add spaces on the left side, up to usize many
    Left(usize),
    /// Add spaces on the right side, up to usize many
    Right(usize),
    /// Do not pad the thread id/name
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Mode for logging the thread name or id or both.
pub enum ThreadLogMode {
    /// Log thread ids only
    IDs,
    /// Log the thread names only
    Names,
    /// If this thread is named, log the name. Otherwise, log the thread id.
    Both,
}

#[derive(Debug, Clone)]
pub(crate) enum TimeFormat {
    Rfc2822,
    Rfc3339,
    Custom(&'static [time::format_description::FormatItem<'static>]),
}

/// Configuration for the Loggers
///
/// All loggers print the message in the following form:
/// `00:00:00 [LEVEL] crate::module: [lib.rs::100] your_message`
/// Every space delimited part except the actual message is optional.
///
/// Pass this struct to your logger to change when these information shall
/// be logged.
///
/// Construct using [`Default`](Config::default) or using [`ConfigBuilder`]
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) time: LevelFilter,
    pub(crate) level: LevelFilter,
    pub(crate) level_padding: LevelPadding,
    pub(crate) thread: LevelFilter,
    pub(crate) thread_log_mode: ThreadLogMode,
    pub(crate) thread_padding: ThreadPadding,
    pub(crate) target: LevelFilter,
    pub(crate) target_padding: TargetPadding,
    pub(crate) location: LevelFilter,
    pub(crate) time_format: TimeFormat,
    pub(crate) time_offset: UtcOffset,
    pub(crate) filter_allow: Cow<'static, [Cow<'static, str>]>,
    pub(crate) filter_ignore: Cow<'static, [Cow<'static, str>]>,
    #[cfg(feature = "termcolor")]
    pub(crate) level_color: [Option<Color>; 6],
    pub(crate) write_log_enable_colors: bool,
}

/// Builder for the Logger Configurations (`Config`)
///
/// All loggers print the message in the following form:
/// `00:00:00 [LEVEL] crate::module: [lib.rs::100] your_message`
/// Every space delimited part except the actual message is optional.
///
/// Use this struct to create a custom `Config` changing when these information shall
/// be logged. Every part can be enabled for a specific Level and is then
/// automatically enable for all lower levels as well.
///
/// The Result is that the logging gets more detailed the more verbose it gets.
/// E.g. to have one part shown always use `Level::Error`. But if you
/// want to show the source line only on `Trace` use that.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ConfigBuilder(Config);

impl ConfigBuilder {
    /// Create a new default ConfigBuilder
    pub fn new() -> ConfigBuilder {
        ConfigBuilder(Config::default())
    }

    /// Set at which level and above (more verbose) the level itself shall be logged (default is Error)
    pub fn set_max_level(&mut self, level: LevelFilter) -> &mut ConfigBuilder {
        self.0.level = level;
        self
    }

    /// Set at which level and  above (more verbose) the current time shall be logged (default is Error)
    pub fn set_time_level(&mut self, time: LevelFilter) -> &mut ConfigBuilder {
        self.0.time = time;
        self
    }

    /// Set at which level and above (more verbose) the thread id shall be logged. (default is Debug)
    pub fn set_thread_level(&mut self, thread: LevelFilter) -> &mut ConfigBuilder {
        self.0.thread = thread;
        self
    }

    /// Set at which level and above (more verbose) the target shall be logged. (default is Debug)
    pub fn set_target_level(&mut self, target: LevelFilter) -> &mut ConfigBuilder {
        self.0.target = target;
        self
    }

    /// Set how the thread should be padded
    pub fn set_target_padding(&mut self, padding: TargetPadding) -> &mut ConfigBuilder {
        self.0.target_padding = padding;
        self
    }

    /// Set at which level and above (more verbose) a source code reference shall be logged (default is Trace)
    pub fn set_location_level(&mut self, location: LevelFilter) -> &mut ConfigBuilder {
        self.0.location = location;
        self
    }

    /// Set how the levels should be padded, when logging (default is Off)
    pub fn set_level_padding(&mut self, padding: LevelPadding) -> &mut ConfigBuilder {
        self.0.level_padding = padding;
        self
    }

    /// Set how the thread should be padded
    pub fn set_thread_padding(&mut self, padding: ThreadPadding) -> &mut ConfigBuilder {
        self.0.thread_padding = padding;
        self
    }

    /// Set the mode for logging the thread
    pub fn set_thread_mode(&mut self, mode: ThreadLogMode) -> &mut ConfigBuilder {
        self.0.thread_log_mode = mode;
        self
    }

    /// Set the color used for printing the level (if the logger supports it),
    /// or None to use the default foreground color
    #[cfg(feature = "termcolor")]
    pub fn set_level_color(&mut self, level: Level, color: Option<Color>) -> &mut ConfigBuilder {
        self.0.level_color[level as usize] = color;
        self
    }

    /// Sets the time format to a custom representation.
    ///
    /// The easiest way to satisfy the static lifetime of the argument is to directly use the
    /// re-exported [`time::macros::format_description`] macro.
    ///
    /// *Note*: The default time format is "[hour]:[minute]:[second]".
    ///
    /// The syntax for the format_description macro can be found in the
    /// [`time` crate book](https://time-rs.github.io/book/api/format-description.html).
    ///
    /// # Usage
    ///
    /// ```
    /// # use simplelog::{ConfigBuilder, format_description};
    /// let config = ConfigBuilder::new()
    ///     .set_time_format_custom(format_description!("[hour]:[minute]:[second].[subsecond]"))
    ///     .build();
    /// ```
    pub fn set_time_format_custom(
        &mut self,
        time_format: &'static [FormatItem<'static>],
    ) -> &mut ConfigBuilder {
        self.0.time_format = TimeFormat::Custom(time_format);
        self
    }

    /// Set time format string to use rfc2822.
    pub fn set_time_format_rfc2822(&mut self) -> &mut ConfigBuilder {
        self.0.time_format = TimeFormat::Rfc2822;
        self
    }

    /// Set time format string to use rfc3339.
    pub fn set_time_format_rfc3339(&mut self) -> &mut ConfigBuilder {
        self.0.time_format = TimeFormat::Rfc3339;
        self
    }

    /// Set offset used for logging time (default is UTC)
    pub fn set_time_offset(&mut self, offset: UtcOffset) -> &mut ConfigBuilder {
        self.0.time_offset = offset;
        self
    }

    /// Sets the offset used to the current local time offset
    /// (overriding values previously set by [`ConfigBuilder::set_time_offset`]).
    ///
    /// This function may fail if the offset cannot be determined soundly.
    /// This may be the case, when the program is multi-threaded by the time of calling this function.
    /// One can opt-out of this behavior by setting `RUSTFLAGS="--cfg unsound_local_offset"`.
    /// Doing so is not recommended, completely untested and may cause unexpected segfaults.
    #[cfg(feature = "local-offset")]
    pub fn set_time_offset_to_local(&mut self) -> Result<&mut ConfigBuilder, &mut ConfigBuilder> {
        match UtcOffset::current_local_offset() {
            Ok(offset) => {
                self.0.time_offset = offset;
                Ok(self)
            }
            Err(_) => Err(self),
        }
    }

    /// set if you want to write colors in the logfile (default is Off)
    #[cfg(feature = "ansi_term")]
    pub fn set_write_log_enable_colors(&mut self, local: bool) -> &mut ConfigBuilder {
        self.0.write_log_enable_colors = local;
        self
    }

    /// Add allowed module filters.
    /// If any are specified, only records from modules starting with one of these entries will be printed
    ///
    /// For example, `add_filter_allow_str("tokio::uds")` would allow only logging from the `tokio` crates `uds` module.
    pub fn add_filter_allow_str(&mut self, filter_allow: &'static str) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_allow);
        list.push(Cow::Borrowed(filter_allow));
        self.0.filter_allow = Cow::Owned(list);
        self
    }

    /// Add allowed module filters.
    /// If any are specified, only records from modules starting with one of these entries will be printed
    ///
    /// For example, `add_filter_allow(format!("{}{}","tokio", "uds"))` would allow only logging from the `tokio` crates `uds` module.
    pub fn add_filter_allow(&mut self, filter_allow: String) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_allow);
        list.push(Cow::Owned(filter_allow));
        self.0.filter_allow = Cow::Owned(list);
        self
    }

    /// Clear allowed module filters.
    /// If none are specified, nothing is filtered out
    pub fn clear_filter_allow(&mut self) -> &mut ConfigBuilder {
        self.0.filter_allow = Cow::Borrowed(&[]);
        self
    }

    /// Add denied module filters.
    /// If any are specified, records from modules starting with one of these entries will be ignored
    ///
    /// For example, `add_filter_ignore_str("tokio::uds")` would deny logging from the `tokio` crates `uds` module.
    pub fn add_filter_ignore_str(&mut self, filter_ignore: &'static str) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_ignore);
        list.push(Cow::Borrowed(filter_ignore));
        self.0.filter_ignore = Cow::Owned(list);
        self
    }

    /// Add denied module filters.
    /// If any are specified, records from modules starting with one of these entries will be ignored
    ///
    /// For example, `add_filter_ignore(format!("{}{}","tokio", "uds"))` would deny logging from the `tokio` crates `uds` module.
    pub fn add_filter_ignore(&mut self, filter_ignore: String) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_ignore);
        list.push(Cow::Owned(filter_ignore));
        self.0.filter_ignore = Cow::Owned(list);
        self
    }

    /// Clear ignore module filters.
    /// If none are specified, nothing is filtered
    pub fn clear_filter_ignore(&mut self) -> &mut ConfigBuilder {
        self.0.filter_ignore = Cow::Borrowed(&[]);
        self
    }

    /// Build new `Config`
    pub fn build(&mut self) -> Config {
        self.0.clone()
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            time: LevelFilter::Error,
            level: LevelFilter::Error,
            level_padding: LevelPadding::Off,
            thread: LevelFilter::Debug,
            thread_log_mode: ThreadLogMode::IDs,
            thread_padding: ThreadPadding::Off,
            target: LevelFilter::Debug,
            target_padding: TargetPadding::Off,
            location: LevelFilter::Trace,
            time_format: TimeFormat::Custom(format_description!("[hour]:[minute]:[second]")),
            time_offset: UtcOffset::UTC,
            filter_allow: Cow::Borrowed(&[]),
            filter_ignore: Cow::Borrowed(&[]),
            write_log_enable_colors: false,

            #[cfg(feature = "termcolor")]
            level_color: [
                None,                // Default foreground
                Some(Color::Red),    // Error
                Some(Color::Yellow), // Warn
                Some(Color::Blue),   // Info
                Some(Color::Cyan),   // Debug
                Some(Color::White),  // Trace
            ],
        }
    }
}
