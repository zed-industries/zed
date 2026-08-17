//! These strategies simply provide the user’s configuration, data, and cache directories, without knowing about the application specifically.

use std::path::{Path, PathBuf};

/// Provides configuration, data, and cache directories of the current user.
pub trait BaseStrategy: Sized {
    /// The error type returned by `new`.
    type CreationError: std::error::Error;

    /// Base strategies are constructed without knowledge of the application.
    fn new() -> Result<Self, Self::CreationError>;

    /// Gets the home directory of the current user.
    fn home_dir(&self) -> &Path;

    /// Gets the user’s configuration directory.
    fn config_dir(&self) -> PathBuf;

    /// Gets the user’s data directory.
    fn data_dir(&self) -> PathBuf;

    /// Gets the user’s cache directory.
    fn cache_dir(&self) -> PathBuf;

    /// Gets the user’s state directory.
    /// State directory may not exist for all conventions.
    fn state_dir(&self) -> Option<PathBuf>;

    /// Gets the user’s runtime directory.
    /// Runtime directory may not exist for all conventions.
    fn runtime_dir(&self) -> Option<PathBuf>;
}

macro_rules! create_strategies {
    ($native: ty, $base: ty) => {
        /// Returns the current OS’s native [`BaseStrategy`](trait.BaseStrategy.html).
        /// This uses the [`Windows`](struct.Windows.html) strategy on Windows, [`Apple`](struct.Apple.html) on macOS & iOS, and [`Xdg`](struct.Xdg.html) everywhere else.
        /// This is the convention used by most GUI applications.
        pub fn choose_native_strategy() -> Result<$native, <$native as BaseStrategy>::CreationError>
        {
            <$native>::new()
        }

        /// Returns the current OS’s default [`BaseStrategy`](trait.BaseStrategy.html).
        /// This uses the [`Windows`](struct.Windows.html) strategy on Windows, and [`Xdg`](struct.Xdg.html) everywhere else.
        /// This is the convention used by most CLI applications.
        pub fn choose_base_strategy() -> Result<$base, <$base as BaseStrategy>::CreationError> {
            <$base>::new()
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        create_strategies!(Windows, Windows);
    } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        create_strategies!(Apple, Xdg);
    } else {
        create_strategies!(Xdg, Xdg);
    }
}

mod apple;
mod windows;
mod xdg;

pub use apple::Apple;
pub use windows::Windows;
pub use xdg::Xdg;
