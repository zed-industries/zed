//! Configuration types

use notify_types::event::EventKindMask;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

/// Indicates whether only the provided directory or its sub-directories as well should be watched
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum RecursiveMode {
    /// Watch all sub-directories as well, including directories created after installing the watch
    Recursive,

    /// Watch only the provided directory
    NonRecursive,
}

impl RecursiveMode {
    pub(crate) fn is_recursive(&self) -> bool {
        match *self {
            RecursiveMode::Recursive => true,
            RecursiveMode::NonRecursive => false,
        }
    }
}

/// Controls how path separators are represented in emitted event paths on Windows.
///
/// This applies to [`ReadDirectoryChangesWatcher`](crate::ReadDirectoryChangesWatcher).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum WindowsPathSeparatorStyle {
    /// Infer separator style from each watched input path.
    ///
    /// For example, watching `C:/tmp` yields paths with `/`, while watching `C:\tmp`
    /// yields paths with `\`.
    #[default]
    Auto,

    /// Always use `/` in emitted event paths.
    Slash,

    /// Always use `\` in emitted event paths.
    Backslash,
}

/// Watcher Backend configuration
///
/// This contains multiple settings that may relate to only one specific backend,
/// such as to correctly configure each backend regardless of what is selected during runtime.
///
/// ```rust
/// # use std::time::Duration;
/// # use notify::Config;
/// let config = Config::default()
///     .with_poll_interval(Duration::from_secs(2))
///     .with_compare_contents(true);
/// ```
///
/// Some options can be changed during runtime, others have to be set when creating the watcher backend.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Config {
    /// See [Config::with_poll_interval]
    poll_interval: Option<Duration>,

    /// See [Config::with_compare_contents]
    compare_contents: bool,

    follow_symlinks: bool,

    /// See [Config::with_event_kinds]
    event_kinds: EventKindMask,

    /// See [Config::with_windows_path_separator_style]
    windows_path_separator_style: WindowsPathSeparatorStyle,

    /// See [Config::with_fsevent_latency]
    fsevent_latency: Duration,
}

impl Config {
    /// For the [`PollWatcher`](crate::PollWatcher) backend.
    ///
    /// Interval between each re-scan attempt. This can be extremely expensive for large
    /// file trees so it is recommended to measure and tune accordingly.
    ///
    /// The default poll frequency is 30 seconds.
    ///
    /// This will enable automatic polling, overwriting [`with_manual_polling()`](Config::with_manual_polling).
    #[must_use]
    pub fn with_poll_interval(mut self, dur: Duration) -> Self {
        // TODO: v7.0 break signature to option
        self.poll_interval = Some(dur);
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn poll_interval(&self) -> Option<Duration> {
        // Changed Signature to Option
        self.poll_interval
    }

    /// For the [`PollWatcher`](crate::PollWatcher) backend.
    ///
    /// Disable automatic polling. Requires calling [`crate::PollWatcher::poll()`] manually.
    ///
    /// This will disable automatic polling, overwriting [`with_poll_interval()`](Config::with_poll_interval).
    #[must_use]
    pub fn with_manual_polling(mut self) -> Self {
        self.poll_interval = None;
        self
    }

    /// For the [`PollWatcher`](crate::PollWatcher) backend.
    ///
    /// Optional feature that will evaluate the contents of changed files to determine if
    /// they have indeed changed using a fast hashing algorithm.  This is especially important
    /// for pseudo filesystems like those on Linux under /sys and /proc which are not obligated
    /// to respect any other filesystem norms such as modification timestamps, file sizes, etc.
    /// By enabling this feature, performance will be significantly impacted as all files will
    /// need to be read and hashed at each `poll_interval`.
    ///
    /// This can't be changed during runtime. Off by default.
    #[must_use]
    pub fn with_compare_contents(mut self, compare_contents: bool) -> Self {
        self.compare_contents = compare_contents;
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn compare_contents(&self) -> bool {
        self.compare_contents
    }

    /// For the [INotifyWatcher](crate::INotifyWatcher), [KqueueWatcher](crate::KqueueWatcher),
    /// and [PollWatcher](crate::PollWatcher).
    ///
    /// Determine if symbolic links should be followed when recursively watching a directory.
    ///
    /// This can't be changed during runtime. On by default.
    #[must_use]
    pub fn with_follow_symlinks(mut self, follow_symlinks: bool) -> Self {
        self.follow_symlinks = follow_symlinks;
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn follow_symlinks(&self) -> bool {
        self.follow_symlinks
    }

    /// Filter which event kinds are monitored.
    ///
    /// This allows you to control which types of filesystem events are delivered
    /// to your event handler. On backends that support kernel-level filtering
    /// (inotify), the mask is translated to native flags for optimal
    /// performance. On other backends (kqueue, Windows, FSEvents, PollWatcher),
    /// filtering is applied in userspace.
    ///
    /// The default is [`EventKindMask::ALL`], which includes all events.
    /// Use [`EventKindMask::CORE`] to exclude access events.
    ///
    /// This can't be changed during runtime.
    ///
    /// # Example
    ///
    /// ```rust
    /// use notify::{Config, EventKindMask};
    ///
    /// // Only monitor file creation and deletion
    /// let config = Config::default()
    ///     .with_event_kinds(EventKindMask::CREATE | EventKindMask::REMOVE);
    ///
    /// // Monitor everything including access events
    /// let config_all = Config::default()
    ///     .with_event_kinds(EventKindMask::ALL);
    /// ```
    #[must_use]
    pub fn with_event_kinds(mut self, event_kinds: EventKindMask) -> Self {
        self.event_kinds = event_kinds;
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn event_kinds(&self) -> EventKindMask {
        self.event_kinds
    }

    /// For the [`ReadDirectoryChangesWatcher`](crate::ReadDirectoryChangesWatcher) backend.
    ///
    /// Controls path separator normalization for emitted event paths on Windows.
    ///
    /// The default is [`WindowsPathSeparatorStyle::Auto`], which preserves the
    /// separator style implied by each watched input path.
    ///
    /// This can't be changed during runtime.
    #[must_use]
    pub fn with_windows_path_separator_style(mut self, style: WindowsPathSeparatorStyle) -> Self {
        self.windows_path_separator_style = style;
        self
    }

    /// Returns current setting.
    #[must_use]
    pub fn windows_path_separator_style(&self) -> WindowsPathSeparatorStyle {
        self.windows_path_separator_style
    }

    /// For the [`FsEventWatcher`](crate::FsEventWatcher) backend.
    ///
    /// The latency passed to `FSEventStreamCreate`: how long the FSEvents service waits
    /// after hearing about an event from the kernel before invoking the callback. A larger
    /// value lets the system coalesce bursts of changes into fewer callbacks at the cost of
    /// higher delivery latency.
    ///
    /// This watcher is created with `kFSEventStreamCreateFlagNoDefer`, so the first event in
    /// an idle period is delivered immediately; only subsequent events get coalesced within
    /// the latency window.
    ///
    /// The default is [`Duration::ZERO`].
    ///
    /// This can't be changed during runtime.
    #[must_use]
    pub fn with_fsevent_latency(mut self, latency: Duration) -> Self {
        self.fsevent_latency = latency;
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn fsevent_latency(&self) -> Duration {
        self.fsevent_latency
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Some(Duration::from_secs(30)),
            compare_contents: false,
            follow_symlinks: true,
            event_kinds: EventKindMask::ALL,
            windows_path_separator_style: WindowsPathSeparatorStyle::Auto,
            fsevent_latency: Duration::ZERO,
        }
    }
}

/// Single watch backend configuration
///
/// This contains some settings that may relate to only one specific backend,
/// such as to correctly configure each backend regardless of what is selected during runtime.
#[derive(Debug)]
pub struct WatchPathConfig {
    recursive_mode: RecursiveMode,
}

impl WatchPathConfig {
    /// Creates new instance with provided [`RecursiveMode`]
    #[must_use]
    pub fn new(recursive_mode: RecursiveMode) -> Self {
        Self { recursive_mode }
    }

    /// Set [`RecursiveMode`] for the watch
    #[must_use]
    pub fn with_recursive_mode(mut self, recursive_mode: RecursiveMode) -> Self {
        self.recursive_mode = recursive_mode;
        self
    }

    /// Returns current setting
    #[must_use]
    pub fn recursive_mode(&self) -> RecursiveMode {
        self.recursive_mode
    }
}

/// An operation to apply to a watcher
///
/// See [`Watcher::update_paths`] for more information
#[derive(Debug)]
pub enum PathOp {
    /// Path should be watched
    Watch(PathBuf, WatchPathConfig),

    /// Path should be unwatched
    Unwatch(PathBuf),
}

impl PathOp {
    /// Watch the path with [`RecursiveMode::Recursive`]
    #[must_use]
    pub fn watch_recursive<P: Into<PathBuf>>(path: P) -> Self {
        Self::Watch(path.into(), WatchPathConfig::new(RecursiveMode::Recursive))
    }

    /// Watch the path with [`RecursiveMode::NonRecursive`]
    #[must_use]
    pub fn watch_non_recursive<P: Into<PathBuf>>(path: P) -> Self {
        Self::Watch(
            path.into(),
            WatchPathConfig::new(RecursiveMode::NonRecursive),
        )
    }

    /// Unwatch the path
    #[must_use]
    pub fn unwatch<P: Into<PathBuf>>(path: P) -> Self {
        Self::Unwatch(path.into())
    }

    /// Returns the path associated with this operation.
    #[must_use]
    pub fn as_path(&self) -> &Path {
        match self {
            PathOp::Watch(p, _) => p,
            PathOp::Unwatch(p) => p,
        }
    }

    /// Returns the path associated with this operation.
    #[must_use]
    pub fn into_path(self) -> PathBuf {
        match self {
            PathOp::Watch(p, _) => p,
            PathOp::Unwatch(p) => p,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify_types::event::EventKindMask;

    #[test]
    fn config_default_event_kinds_is_all() {
        let config = Config::default();
        assert_eq!(config.event_kinds(), EventKindMask::ALL);
    }

    #[test]
    fn config_with_event_kinds() {
        let mask = EventKindMask::CREATE | EventKindMask::REMOVE;
        let config = Config::default().with_event_kinds(mask);
        assert_eq!(config.event_kinds(), mask);
    }

    #[test]
    fn config_with_all_events_includes_access() {
        let config = Config::default().with_event_kinds(EventKindMask::ALL);
        assert!(config.event_kinds().intersects(EventKindMask::ALL_ACCESS));
    }

    #[test]
    fn config_with_empty_mask() {
        let config = Config::default().with_event_kinds(EventKindMask::empty());
        assert!(config.event_kinds().is_empty());
    }

    #[test]
    fn event_kind_mask_default_matches_config_default() {
        // Verify cross-crate consistency: both defaults should be ALL
        assert_eq!(EventKindMask::default(), Config::default().event_kinds());
        assert_eq!(EventKindMask::default(), EventKindMask::ALL);
    }

    #[test]
    fn config_default_windows_separator_style_is_auto() {
        let config = Config::default();
        assert_eq!(
            config.windows_path_separator_style(),
            WindowsPathSeparatorStyle::Auto
        );
    }

    #[test]
    fn config_with_windows_separator_style() {
        let config =
            Config::default().with_windows_path_separator_style(WindowsPathSeparatorStyle::Slash);
        assert_eq!(
            config.windows_path_separator_style(),
            WindowsPathSeparatorStyle::Slash
        );
    }

    #[test]
    fn config_default_fsevent_latency_is_zero() {
        assert_eq!(Config::default().fsevent_latency(), Duration::ZERO);
    }

    #[test]
    fn config_with_fsevent_latency() {
        let latency = Duration::from_millis(250);
        let config = Config::default().with_fsevent_latency(latency);
        assert_eq!(config.fsevent_latency(), latency);
    }
}
