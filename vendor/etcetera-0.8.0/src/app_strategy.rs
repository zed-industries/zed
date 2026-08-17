//! These strategies require you to provide some information on your application, and they will in turn locate the configuration/data/cache directory specifically for your application.

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

/// The arguments to the creator method of an [`AppStrategy`](trait.AppStrategy.html).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AppStrategyArgs {
    /// The top level domain of the application, e.g. `com`, `org`, or `io.github`.
    pub top_level_domain: String,
    /// The name of the author of the application.
    pub author: String,
    /// The application’s name. This should be capitalised if appropriate.
    pub app_name: String,
}

impl AppStrategyArgs {
    /// Constructs a bunde identifier from an `AppStrategyArgs`.
    ///
    /// ```
    /// use etcetera::app_strategy::AppStrategyArgs;
    ///
    /// let strategy_args = AppStrategyArgs {
    ///     top_level_domain: "org".to_string(),
    ///     author: "Acme Corp".to_string(),
    ///     app_name: "Frobnicator Plus".to_string(),
    /// };
    ///
    /// assert_eq!(strategy_args.bundle_id().replace(' ', ""), "org.acmecorp.FrobnicatorPlus".to_string());
    /// ```
    pub fn bundle_id(&self) -> String {
        format!(
            "{}.{}.{}",
            self.top_level_domain,
            self.author.to_lowercase(),
            self.app_name
        )
    }

    /// Returns a ‘unixy’ version of the application’s name, akin to what would usually be used as a binary name.
    ///
    /// ```
    /// use etcetera::app_strategy::AppStrategyArgs;
    ///
    /// let strategy_args = AppStrategyArgs {
    ///     top_level_domain: "org".to_string(),
    ///     author: "Acme Corp".to_string(),
    ///     app_name: "Frobnicator Plus".to_string(),
    /// };
    ///
    /// assert_eq!(strategy_args.unixy_name(), "frobnicator-plus".to_string());
    /// ```
    pub fn unixy_name(&self) -> String {
        self.app_name.to_lowercase().replace(' ', "-")
    }
}

macro_rules! in_dir_method {
    ($self: ident, $path_extra: expr, $dir_method_name: ident) => {{
        let mut path = $self.$dir_method_name();
        path.push(Path::new(&$path_extra));
        path
    }};
    (opt: $self: ident, $path_extra: expr, $dir_method_name: ident) => {{
        let mut path = $self.$dir_method_name()?;
        path.push(Path::new(&$path_extra));
        Some(path)
    }};
}

/// Allows applications to retrieve the paths of configuration, data, and cache directories specifically for them.
pub trait AppStrategy: Sized {
    /// The error type returned by `new`.
    type CreationError: std::error::Error;

    /// The constructor requires access to some basic information about your application.
    fn new(args: AppStrategyArgs) -> Result<Self, Self::CreationError>;

    /// Gets the home directory of the current user.
    fn home_dir(&self) -> &Path;

    /// Gets the configuration directory for your application.
    fn config_dir(&self) -> PathBuf;

    /// Gets the data directory for your application.
    fn data_dir(&self) -> PathBuf;

    /// Gets the cache directory for your application.
    fn cache_dir(&self) -> PathBuf;

    /// Gets the state directory for your application.
    /// State directory may not to exist for all conventions.
    fn state_dir(&self) -> Option<PathBuf>;

    /// Gets the runtime directory for your application.
    /// Runtime directory may not to exist for all conventions.
    fn runtime_dir(&self) -> Option<PathBuf>;

    /// Constructs a path inside your application’s configuration directory to which a path of your choice has been appended.
    fn in_config_dir<P: AsRef<OsStr>>(&self, path: P) -> PathBuf {
        in_dir_method!(self, path, config_dir)
    }

    /// Constructs a path inside your application’s data directory to which a path of your choice has been appended.
    fn in_data_dir<P: AsRef<OsStr>>(&self, path: P) -> PathBuf {
        in_dir_method!(self, path, data_dir)
    }

    /// Constructs a path inside your application’s cache directory to which a path of your choice has been appended.
    fn in_cache_dir<P: AsRef<OsStr>>(&self, path: P) -> PathBuf {
        in_dir_method!(self, path, cache_dir)
    }

    /// Constructs a path inside your application’s state directory to which a path of your choice has been appended.
    fn in_state_dir<P: AsRef<OsStr>>(&self, path: P) -> Option<PathBuf> {
        in_dir_method!(opt: self, path, state_dir)
    }

    /// Constructs a path inside your application’s runtime directory to which a path of your choice has been appended.
    fn in_runtime_dir<P: AsRef<OsStr>>(&self, path: P) -> Option<PathBuf> {
        in_dir_method!(opt: self, path, runtime_dir)
    }
}

macro_rules! create_strategies {
    ($native: ty, $app: ty) => {
        /// Returns the current OS’s native [`AppStrategy`](trait.AppStrategy.html).
        /// This uses the [`Windows`](struct.Windows.html) strategy on Windows, [`Apple`](struct.Apple.html) on macOS & iOS, and [`Xdg`](struct.Xdg.html) everywhere else.
        /// This is the convention used by most GUI applications.
        pub fn choose_native_strategy(
            args: AppStrategyArgs,
        ) -> Result<$native, <$native as AppStrategy>::CreationError> {
            <$native>::new(args)
        }

        /// Returns the current OS’s default [`AppStrategy`](trait.AppStrategy.html).
        /// This uses the [`Windows`](struct.Windows.html) strategy on Windows, and [`Xdg`](struct.Xdg.html) everywhere else.
        /// This is the convention used by most CLI applications.
        pub fn choose_app_strategy(
            args: AppStrategyArgs,
        ) -> Result<$app, <$app as AppStrategy>::CreationError> {
            <$app>::new(args)
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
mod unix;
mod windows;
mod xdg;

pub use apple::Apple;
pub use unix::Unix;
pub use windows::Windows;
pub use xdg::Xdg;
