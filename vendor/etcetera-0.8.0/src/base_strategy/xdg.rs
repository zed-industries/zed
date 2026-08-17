use std::path::Path;
use std::path::PathBuf;

/// This strategy implements the [XDG Base Directories Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html). It is the most common on Linux, but is increasingly being adopted elsewhere.
///
/// This initial example removes all the XDG environment variables to show the strategy’s use of the XDG default directories.
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::remove_var("XDG_DATA_HOME");
/// std::env::remove_var("XDG_CACHE_HOME");
/// std::env::remove_var("XDG_STATE_HOME");
/// std::env::remove_var("XDG_RUNTIME_DIR");
///
/// let base_strategy = Xdg::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     base_strategy.home_dir(),
///     &home_dir
/// );
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".config/"))
/// );
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".local/share/"))
/// );
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".cache/"))
/// );
/// assert_eq!(
///     base_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state"))
/// );
/// assert_eq!(
///     base_strategy.runtime_dir(),
///     None
/// );
/// ```
///
/// This next example gives the environment variables values:
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// // We need to conditionally set these to ensure that they are absolute paths both on Windows and other systems.
/// let config_path = if cfg!(windows) {
///     "C:\\foo\\"
/// } else {
///     "/foo/"
/// };
/// let data_path = if cfg!(windows) {
///     "C:\\bar\\"
/// } else {
///     "/bar/"
/// };
/// let cache_path = if cfg!(windows) {
///     "C:\\baz\\"
/// } else {
///     "/baz/"
/// };
/// let state_path = if cfg!(windows) {
///     "C:\\foobar\\"
/// } else {
///     "/foobar/"
/// };
/// let runtime_path = if cfg!(windows) {
///     "C:\\qux\\"
/// } else {
///     "/qux/"
/// };
///
/// std::env::set_var("XDG_CONFIG_HOME", config_path);
/// std::env::set_var("XDG_DATA_HOME", data_path);
/// std::env::set_var("XDG_CACHE_HOME", cache_path);
/// std::env::set_var("XDG_STATE_HOME", state_path);
/// std::env::set_var("XDG_RUNTIME_DIR", runtime_path);
///
/// let base_strategy = Xdg::new().unwrap();
///
/// assert_eq!(
///     base_strategy.config_dir(),
///     Path::new(config_path)
/// );
/// assert_eq!(
///     base_strategy.data_dir(),
///     Path::new(data_path)
/// );
/// assert_eq!(
///     base_strategy.cache_dir(),
///     Path::new(cache_path)
/// );
/// assert_eq!(
///     base_strategy.state_dir().unwrap(),
///     Path::new(state_path)
/// );
/// assert_eq!(
///     base_strategy.runtime_dir().unwrap(),
///     Path::new(runtime_path)
/// );
/// ```
///
/// The XDG spec requires that when the environment variables’ values are not absolute paths, their values should be ignored. This example exemplifies this behaviour:
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::set_var("XDG_CONFIG_HOME", "foo/");
/// std::env::set_var("XDG_DATA_HOME", "bar/");
/// std::env::set_var("XDG_CACHE_HOME", "baz/");
/// std::env::set_var("XDG_STATE_HOME", "foobar/");
/// std::env::set_var("XDG_RUNTIME_DIR", "qux/");
///
/// let base_strategy = Xdg::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// // We still get the default values.
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".config/"))
/// );
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".local/share/"))
/// );
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".cache/"))
/// );
/// assert_eq!(
///     base_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state/"))
/// );
/// assert_eq!(
///     base_strategy.runtime_dir(),
///     None
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Xdg {
    home_dir: PathBuf,
}

impl Xdg {
    fn env_var_or_none(env_var: &str) -> Option<PathBuf> {
        std::env::var(env_var).ok().and_then(|path| {
            let path = PathBuf::from(path);

            // Return None if the path obtained from the environment variable isn’t absolute.
            if path.is_absolute() {
                Some(path)
            } else {
                None
            }
        })
    }

    fn env_var_or_default(&self, env_var: &str, default: impl AsRef<Path>) -> PathBuf {
        Self::env_var_or_none(env_var).unwrap_or_else(|| self.home_dir.join(default))
    }
}

impl super::BaseStrategy for Xdg {
    type CreationError = crate::HomeDirError;

    fn new() -> Result<Self, Self::CreationError> {
        Ok(Self {
            home_dir: crate::home_dir()?,
        })
    }

    fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    fn config_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_CONFIG_HOME", ".config/")
    }

    fn data_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_DATA_HOME", ".local/share/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_CACHE_HOME", ".cache/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        Some(self.env_var_or_default("XDG_STATE_HOME", ".local/state/"))
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        Self::env_var_or_none("XDG_RUNTIME_DIR")
    }
}
