use crate::base_strategy;
use crate::base_strategy::BaseStrategy;
use std::path::{Path, PathBuf};

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern. The specification is available [here](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid).
///
/// This initial example removes all the relevant environment variables to show the strategy’s use of the:
/// - (on Windows) SHGetFolderPathW API.
/// - (on non-Windows) Windows default directories.
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Windows;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::remove_var("USERPROFILE");
/// std::env::remove_var("APPDATA");
/// std::env::remove_var("LOCALAPPDATA");
///
/// let app_strategy = Windows::new(AppStrategyArgs {
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
///     Ok(Path::new("AppData/Roaming/Acme Corp/Frobnicator Plus/config"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/Acme Corp/Frobnicator Plus/data"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Local/Acme Corp/Frobnicator Plus/cache"))
/// );
/// assert_eq!(
///     app_strategy.state_dir(),
///     None
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
/// use etcetera::app_strategy::Windows;
/// use std::path::Path;
///
/// let home_path = if cfg!(windows) {
///     "C:\\my_home_location\\".to_string()
/// } else {
///     etcetera::home_dir().unwrap().to_string_lossy().to_string()
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
///
/// std::env::set_var("USERPROFILE", &home_path);
/// std::env::set_var("APPDATA", data_path);
/// std::env::set_var("LOCALAPPDATA", cache_path);
///
/// let app_strategy = Windows::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Acme Corp".to_string(),
///     app_name: "Frobnicator Plus".to_string(),
/// }).unwrap();
///
/// assert_eq!(
///     app_strategy.home_dir(),
///     Path::new(&home_path)
/// );
/// assert_eq!(
///     app_strategy.config_dir(),
///     Path::new(&format!("{}/Acme Corp/Frobnicator Plus/config", data_path))
/// );
/// assert_eq!(
///     app_strategy.data_dir(),
///     Path::new(&format!("{}/Acme Corp/Frobnicator Plus/data", data_path))
/// );
/// assert_eq!(
///     app_strategy.cache_dir(),
///     Path::new(&format!("{}/Acme Corp/Frobnicator Plus/cache", cache_path))
/// );
/// assert_eq!(
///     app_strategy.state_dir(),
///     None
/// );
/// assert_eq!(
///     app_strategy.runtime_dir(),
///     None
/// );
/// ```

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Windows {
    base_strategy: base_strategy::Windows,
    author_app_name_path: PathBuf,
}

macro_rules! dir_method {
    ($self: ident, $base_strategy_method: ident, $subfolder_name: expr) => {{
        let mut path = $self.base_strategy.$base_strategy_method();
        path.push(&$self.author_app_name_path);
        path.push($subfolder_name);

        path
    }};
}

impl super::AppStrategy for Windows {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        Ok(Self {
            base_strategy: base_strategy::Windows::new()?,
            author_app_name_path: PathBuf::from(args.author).join(args.app_name),
        })
    }

    fn home_dir(&self) -> &Path {
        self.base_strategy.home_dir()
    }

    fn config_dir(&self) -> PathBuf {
        dir_method!(self, config_dir, "config")
    }

    fn data_dir(&self) -> PathBuf {
        dir_method!(self, data_dir, "data")
    }

    fn cache_dir(&self) -> PathBuf {
        dir_method!(self, cache_dir, "cache")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        None
    }
}
