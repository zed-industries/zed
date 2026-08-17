use crate::base_strategy;
use crate::base_strategy::BaseStrategy;
use std::path::{Path, PathBuf};

/// This strategy implements the [XDG Base Directories Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html). It is the most common on Linux, but is increasingly being adopted elsewhere.
///
/// This initial example removes all the XDG environment variables to show the strategy’s use of the XDG default directories.
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Xdg;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::remove_var("XDG_DATA_HOME");
/// std::env::remove_var("XDG_CACHE_HOME");
/// std::env::remove_var("XDG_STATE_HOME");
/// std::env::remove_var("XDG_RUNTIME_DIR");
///
/// let app_strategy = Xdg::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Acme Corp".to_string(),
///     app_name: "Frobnicator Plus".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     app_strategy.home_dir(),
///     &home_dir
/// );
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".config/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".local/share/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".cache/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.runtime_dir(),
///     None
/// );
/// ```
///
/// This next example gives the environment variables values:
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Xdg;
/// use std::path::Path;
///
/// // We need to conditionally set these to ensure that they are absolute paths both on Windows and other systems.
/// let config_path = if cfg!(windows) {
///     "C:\\my_config_location\\"
/// } else {
///     "/my_config_location/"
/// };
/// let data_path = if cfg!(windows) {
///     "C:\\my_data_location\\"
/// } else {
///     "/my_data_location/"
/// };
/// let cache_path = if cfg!(windows) {
///     "C:\\my_cache_location\\"
/// } else {
///     "/my_cache_location/"
/// };
/// let state_path = if cfg!(windows) {
///     "C:\\my_state_location\\"
/// } else {
///     "/my_state_location/"
/// };
/// let runtime_path = if cfg!(windows) {
///     "C:\\my_runtime_location\\"
/// } else {
///     "/my_runtime_location/"
/// };
///
/// std::env::set_var("XDG_CONFIG_HOME", config_path);
/// std::env::set_var("XDG_DATA_HOME", data_path);
/// std::env::set_var("XDG_CACHE_HOME", cache_path);
/// std::env::set_var("XDG_STATE_HOME", state_path);
/// std::env::set_var("XDG_RUNTIME_DIR", runtime_path);
///
/// let app_strategy = Xdg::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Acme Corp".to_string(),
///     app_name: "Frobnicator Plus".to_string(),
/// }).unwrap();
///
/// assert_eq!(
///     app_strategy.config_dir(),
///     Path::new(&format!("{}/frobnicator-plus/", config_path))
/// );
/// assert_eq!(
///     app_strategy.data_dir(),
///     Path::new(&format!("{}/frobnicator-plus/", data_path))
/// );
/// assert_eq!(
///     app_strategy.cache_dir(),
///     Path::new(&format!("{}/frobnicator-plus/", cache_path))
/// );
/// assert_eq!(
///     app_strategy.state_dir().unwrap(),
///     Path::new(&format!("{}/frobnicator-plus/", state_path))
/// );
/// assert_eq!(
///     app_strategy.runtime_dir().unwrap(),
///     Path::new(&format!("{}/frobnicator-plus/", runtime_path))
/// );
/// ```
///
/// The XDG spec requires that when the environment variables’ values are not absolute paths, their values should be ignored. This example exemplifies this behaviour:
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Xdg;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::set_var("XDG_CONFIG_HOME", "relative_path/");
/// std::env::set_var("XDG_DATA_HOME", "./another_one/");
/// std::env::set_var("XDG_CACHE_HOME", "yet_another/");
/// std::env::set_var("XDG_STATE_HOME", "./and_another");
/// std::env::set_var("XDG_RUNTIME_DIR", "relative_path/");
///
/// let app_strategy = Xdg::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Acme Corp".to_string(),
///     app_name: "Frobnicator Plus".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// // We still get the default values.
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".config/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".local/share/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".cache/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state/frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.runtime_dir(),
///     None
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Xdg {
    base_strategy: base_strategy::Xdg,
    unixy_name: String,
}

impl super::AppStrategy for Xdg {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        Ok(Self {
            base_strategy: base_strategy::Xdg::new()?,
            unixy_name: args.unixy_name(),
        })
    }

    fn home_dir(&self) -> &Path {
        self.base_strategy.home_dir()
    }

    fn config_dir(&self) -> PathBuf {
        self.base_strategy.config_dir().join(&self.unixy_name)
    }

    fn data_dir(&self) -> PathBuf {
        self.base_strategy.data_dir().join(&self.unixy_name)
    }

    fn cache_dir(&self) -> PathBuf {
        self.base_strategy.cache_dir().join(&self.unixy_name)
    }

    fn state_dir(&self) -> Option<PathBuf> {
        Some(
            self.base_strategy
                .state_dir()
                .unwrap()
                .join(&self.unixy_name),
        )
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        self.base_strategy
            .runtime_dir()
            .map(|runtime_dir| runtime_dir.join(&self.unixy_name))
    }
}
