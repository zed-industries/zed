//! Cross-platform file system notification library
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! notify = "6.1.1"
//! ```
//!
//! If you want debounced events (or don't need them in-order), see [notify-debouncer-mini](https://docs.rs/notify-debouncer-mini/latest/notify_debouncer_mini/)
//! or [notify-debouncer-full](https://docs.rs/notify-debouncer-full/latest/notify_debouncer_full/).
//!
//! ## Features
//!
//! List of compilation features, see below for details
//!
//! - `serde` for serialization of events
//! - `macos_fsevent` enabled by default, for fsevent backend on macos
//! - `macos_kqueue` for kqueue backend on macos
//! - `crossbeam-channel` enabled by default, see below
//!
//! ### Serde
//!
//! Events are serializable via [serde](https://serde.rs) if the `serde` feature is enabled:
//!
//! ```toml
//! notify = { version = "6.1.1", features = ["serde"] }
//! ```
//!
//! ### Crossbeam-Channel & Tokio
//!
//! By default crossbeam-channel is used internally by notify. Which also allows the [Watcher] to be sync.
//! This can [cause issues](https://github.com/notify-rs/notify/issues/380) when used inside tokio.
//!
//! You can disable crossbeam-channel, letting notify fallback to std channels via
//!
//! ```toml
//! notify = { version = "6.1.1", default-features = false, features = ["macos_kqueue"] }
//! // Alternatively macos_fsevent instead of macos_kqueue
//! ```
//! Note the `macos_kqueue` requirement here, otherwise no native backend is available on macos.
//!
//! # Known Problems
//! 
//! ### Network filesystems
//! 
//! Network mounted filesystems like NFS may not emit any events for notify to listen to.
//! This applies especially to WSL programs watching windows paths ([issue #254](https://github.com/notify-rs/notify/issues/254)).
//! 
//! A workaround is the [PollWatcher] backend.
//!
//! ### Docker with Linux on MacOS M1
//!
//! Docker on macos M1 [throws](https://github.com/notify-rs/notify/issues/423) `Function not implemented (os error 38)`.
//! You have to manually use the [PollWatcher], as the native backend isn't available inside the emulation.
//!
//! ### MacOS, FSEvents and unowned files
//!
//! Due to the inner security model of FSEvents (see [FileSystemEventSecurity](https://developer.apple.com/library/mac/documentation/Darwin/Conceptual/FSEvents_ProgGuide/FileSystemEventSecurity/FileSystemEventSecurity.html)),
//! some events cannot be observed easily when trying to follow files that do not
//! belong to you. In this case, reverting to the pollwatcher can fix the issue,
//! with a slight performance cost.
//!
//! ### Editor Behaviour
//!
//! If you rely on precise events (Write/Delete/Create..), you will notice that the actual events
//! can differ a lot between file editors. Some truncate the file on save, some create a new one and replace the old one.
//! See also [this](https://github.com/notify-rs/notify/issues/247) and [this](https://github.com/notify-rs/notify/issues/113#issuecomment-281836995) issues for example.
//!
//! ### Parent folder deletion
//!
//! If you want to receive an event for a deletion of folder `b` for the path `/a/b/..`, you will have to watch its parent `/a`.
//! See [here](https://github.com/notify-rs/notify/issues/403) for more details.
//!
//! ### Pseudo Filesystems like /proc, /sys
//!
//! Some filesystems like `/proc` and `/sys` on *nix do not emit change events or use correct file change dates.
//! To circumvent that problem you can use the [PollWatcher] with the `compare_contents` option.
//!
//! ### Linux: Bad File Descriptor / No space left on device
//!
//! This may be the case of running into the max-files watched limits of your user or system.
//! (Files also includes folders.) Note that for recursive watched folders each file and folder inside counts towards the limit.
//!
//! You may increase this limit in linux via
//! ```sh
//! sudo sysctl fs.inotify.max_user_instances=8192 # example number
//! sudo sysctl fs.inotify.max_user_watches=524288 # example number
//! sudo sysctl -p
//! ```
//!
//! Note that the [PollWatcher] is not restricted by this limitation, so it may be an alternative if your users can't increase the limit.
//!
//! ### Watching large directories
//! 
//! When watching a very large amount of files, notify may fail to receive all events.
//! For example the linux backend is documented to not be a 100% reliable source. See also issue [#412](https://github.com/notify-rs/notify/issues/412).
//! 
//! # Examples
//!
//! For more examples visit the [examples folder](https://github.com/notify-rs/notify/tree/main/examples) in the repository.
//!
//! ```rust
//! # use std::path::Path;
//! use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};
//!
//! fn main() -> Result<()> {
//!     // Automatically select the best implementation for your platform.
//!     let mut watcher = notify::recommended_watcher(|res| {
//!         match res {
//!            Ok(event) => println!("event: {:?}", event),
//!            Err(e) => println!("watch error: {:?}", e),
//!         }
//!     })?;
//!
//!     // Add a path to be watched. All files and directories at that path and
//!     // below will be monitored for changes.
//! #     #[cfg(not(any(
//! #     target_os = "freebsd",
//! #     target_os = "openbsd",
//! #     target_os = "dragonflybsd",
//! #     target_os = "netbsd")))]
//! #     { // "." doesn't exist on BSD for some reason in CI
//!     watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
//! #     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With different configurations
//!
//! It is possible to create several watchers with different configurations or implementations that
//! all call the same event function. This can accommodate advanced behaviour or work around limits.
//!
//! ```rust
//! # use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
//! # use std::path::Path;
//! #
//! # fn main() -> Result<()> {
//!       fn event_fn(res: Result<notify::Event>) {
//!           match res {
//!              Ok(event) => println!("event: {:?}", event),
//!              Err(e) => println!("watch error: {:?}", e),
//!           }
//!       }
//!
//!       let mut watcher1 = notify::recommended_watcher(event_fn)?;
//!       // we will just use the same watcher kind again here
//!       let mut watcher2 = notify::recommended_watcher(event_fn)?;
//! #     #[cfg(not(any(
//! #     target_os = "freebsd",
//! #     target_os = "openbsd",
//! #     target_os = "dragonflybsd",
//! #     target_os = "netbsd")))]
//! #     { // "." doesn't exist on BSD for some reason in CI
//! #     watcher1.watch(Path::new("."), RecursiveMode::Recursive)?;
//! #     watcher2.watch(Path::new("."), RecursiveMode::Recursive)?;
//! #     }
//!       // dropping the watcher1/2 here (no loop etc) will end the program
//! #
//! #     Ok(())
//! # }
//! ```

#![deny(missing_docs)]

pub use config::{Config, RecursiveMode};
pub use error::{Error, ErrorKind, Result};
pub use event::{Event, EventKind};
use std::path::Path;

#[allow(dead_code)]
#[cfg(feature = "crossbeam-channel")]
pub(crate) type Receiver<T> = crossbeam_channel::Receiver<T>;
#[allow(dead_code)]
#[cfg(not(feature = "crossbeam-channel"))]
pub(crate) type Receiver<T> = std::sync::mpsc::Receiver<T>;

#[allow(dead_code)]
#[cfg(feature = "crossbeam-channel")]
pub(crate) type Sender<T> = crossbeam_channel::Sender<T>;
#[allow(dead_code)]
#[cfg(not(feature = "crossbeam-channel"))]
pub(crate) type Sender<T> = std::sync::mpsc::Sender<T>;

// std limitation
#[allow(dead_code)]
#[cfg(feature = "crossbeam-channel")]
pub(crate) type BoundSender<T> = crossbeam_channel::Sender<T>;
#[allow(dead_code)]
#[cfg(not(feature = "crossbeam-channel"))]
pub(crate) type BoundSender<T> = std::sync::mpsc::SyncSender<T>;

#[allow(dead_code)]
#[inline]
pub(crate) fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    #[cfg(feature = "crossbeam-channel")]
    return crossbeam_channel::unbounded();
    #[cfg(not(feature = "crossbeam-channel"))]
    return std::sync::mpsc::channel();
}

#[allow(dead_code)]
#[inline]
pub(crate) fn bounded<T>(cap: usize) -> (BoundSender<T>, Receiver<T>) {
    #[cfg(feature = "crossbeam-channel")]
    return crossbeam_channel::bounded(cap);
    #[cfg(not(feature = "crossbeam-channel"))]
    return std::sync::mpsc::sync_channel(cap);
}

#[cfg(all(target_os = "macos", not(feature = "macos_kqueue")))]
pub use crate::fsevent::FsEventWatcher;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use crate::inotify::INotifyWatcher;
#[cfg(any(
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonflybsd",
    all(target_os = "macos", feature = "macos_kqueue")
))]
pub use crate::kqueue::KqueueWatcher;
pub use null::NullWatcher;
pub use poll::PollWatcher;
#[cfg(target_os = "windows")]
pub use windows::ReadDirectoryChangesWatcher;

#[cfg(all(target_os = "macos", not(feature = "macos_kqueue")))]
pub mod fsevent;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod inotify;
#[cfg(any(
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "dragonflybsd",
    target_os = "netbsd",
    all(target_os = "macos", feature = "macos_kqueue")
))]
pub mod kqueue;
#[cfg(target_os = "windows")]
pub mod windows;

pub mod event;
pub mod null;
pub mod poll;

mod config;
mod error;

/// The set of requirements for watcher event handling functions.
///
/// # Example implementation
///
/// ```no_run
/// use notify::{Event, Result, EventHandler};
///
/// /// Prints received events
/// struct EventPrinter;
///
/// impl EventHandler for EventPrinter {
///     fn handle_event(&mut self, event: Result<Event>) {
///         if let Ok(event) = event {
///             println!("Event: {:?}", event);
///         }
///     }
/// }
/// ```
pub trait EventHandler: Send + 'static {
    /// Handles an event.
    fn handle_event(&mut self, event: Result<Event>);
}

impl<F> EventHandler for F
where
    F: FnMut(Result<Event>) + Send + 'static,
{
    fn handle_event(&mut self, event: Result<Event>) {
        (self)(event);
    }
}

#[cfg(feature = "crossbeam-channel")]
impl EventHandler for crossbeam_channel::Sender<Result<Event>> {
    fn handle_event(&mut self, event: Result<Event>) {
        let _ = self.send(event);
    }
}

impl EventHandler for std::sync::mpsc::Sender<Result<Event>> {
    fn handle_event(&mut self, event: Result<Event>) {
        let _ = self.send(event);
    }
}

/// Watcher kind enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WatcherKind {
    /// inotify backend (linux)
    Inotify,
    /// FS-Event backend (mac)
    Fsevent,
    /// KQueue backend (bsd,optionally mac)
    Kqueue,
    /// Polling based backend (fallback)
    PollWatcher,
    /// Windows backend
    ReadDirectoryChangesWatcher,
    /// Fake watcher for testing
    NullWatcher,
}

/// Type that can deliver file activity notifications
///
/// Watcher is implemented per platform using the best implementation available on that platform.
/// In addition to such event driven implementations, a polling implementation is also provided
/// that should work on any platform.
pub trait Watcher {
    /// Create a new watcher with an initial Config.
    fn new<F: EventHandler>(event_handler: F, config: config::Config) -> Result<Self>
    where
        Self: Sized;
    /// Begin watching a new path.
    ///
    /// If the `path` is a directory, `recursive_mode` will be evaluated. If `recursive_mode` is
    /// `RecursiveMode::Recursive` events will be delivered for all files in that tree. Otherwise
    /// only the directory and its immediate children will be watched.
    ///
    /// If the `path` is a file, `recursive_mode` will be ignored and events will be delivered only
    /// for the file.
    ///
    /// On some platforms, if the `path` is renamed or removed while being watched, behaviour may
    /// be unexpected. See discussions in [#165] and [#166]. If less surprising behaviour is wanted
    /// one may non-recursively watch the _parent_ directory as well and manage related events.
    ///
    /// [#165]: https://github.com/notify-rs/notify/issues/165
    /// [#166]: https://github.com/notify-rs/notify/issues/166
    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()>;

    /// Stop watching a path.
    ///
    /// # Errors
    ///
    /// Returns an error in the case that `path` has not been watched or if removing the watch
    /// fails.
    fn unwatch(&mut self, path: &Path) -> Result<()>;

    /// Configure the watcher at runtime.
    ///
    /// See the [`Config`](config/enum.Config.html) enum for all configuration options.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` on success.
    /// - `Ok(false)` if the watcher does not support or implement the option.
    /// - `Err(notify::Error)` on failure.
    fn configure(&mut self, _option: Config) -> Result<bool> {
        Ok(false)
    }

    /// Returns the watcher kind, allowing to perform backend-specific tasks
    fn kind() -> WatcherKind
    where
        Self: Sized;
}

/// The recommended `Watcher` implementation for the current platform
#[cfg(any(target_os = "linux", target_os = "android"))]
pub type RecommendedWatcher = INotifyWatcher;
/// The recommended `Watcher` implementation for the current platform
#[cfg(all(target_os = "macos", not(feature = "macos_kqueue")))]
pub type RecommendedWatcher = FsEventWatcher;
/// The recommended `Watcher` implementation for the current platform
#[cfg(target_os = "windows")]
pub type RecommendedWatcher = ReadDirectoryChangesWatcher;
/// The recommended `Watcher` implementation for the current platform
#[cfg(any(
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonflybsd",
    all(target_os = "macos", feature = "macos_kqueue")
))]
pub type RecommendedWatcher = KqueueWatcher;
/// The recommended `Watcher` implementation for the current platform
#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonflybsd"
)))]
pub type RecommendedWatcher = PollWatcher;

/// Convenience method for creating the `RecommendedWatcher` for the current platform in
/// _immediate_ mode.
///
/// See [`Watcher::new_immediate`](trait.Watcher.html#tymethod.new_immediate).
pub fn recommended_watcher<F>(event_handler: F) -> Result<RecommendedWatcher>
where
    F: EventHandler,
{
    // All recommended watchers currently implement `new`, so just call that.
    RecommendedWatcher::new(event_handler, Config::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_safe() {
        let _watcher: &dyn Watcher = &NullWatcher;
    }

    #[test]
    fn test_debug_impl() {
        macro_rules! assert_debug_impl {
            ($t:ty) => {{
                trait NeedsDebug: std::fmt::Debug {}
                impl NeedsDebug for $t {}
            }};
        }

        assert_debug_impl!(Config);
        assert_debug_impl!(Error);
        assert_debug_impl!(ErrorKind);
        assert_debug_impl!(event::AccessKind);
        assert_debug_impl!(event::AccessMode);
        assert_debug_impl!(event::CreateKind);
        assert_debug_impl!(event::DataChange);
        assert_debug_impl!(event::EventAttributes);
        assert_debug_impl!(event::Flag);
        assert_debug_impl!(event::MetadataKind);
        assert_debug_impl!(event::ModifyKind);
        assert_debug_impl!(event::RemoveKind);
        assert_debug_impl!(event::RenameMode);
        assert_debug_impl!(Event);
        assert_debug_impl!(EventKind);
        assert_debug_impl!(NullWatcher);
        assert_debug_impl!(PollWatcher);
        assert_debug_impl!(RecommendedWatcher);
        assert_debug_impl!(RecursiveMode);
        assert_debug_impl!(WatcherKind);
    }
}
