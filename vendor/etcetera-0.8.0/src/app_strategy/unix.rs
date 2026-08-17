use std::path::{Path, PathBuf};

/// This strategy has no standard or official specification. It has arisen over time through hundreds of Unixy tools. Vim and Cargo are notable examples whose configuration/data/cache directory layouts are similar to those created by this strategy.
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Unix;
/// use std::path::Path;
///
/// let app_strategy = Unix::new(AppStrategyArgs {
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
///     Ok(Path::new(".frobnicator-plus/"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".frobnicator-plus/data/"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".frobnicator-plus/cache/"))
/// );
/// assert_eq!(
///     app_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".frobnicator-plus/state/"))
/// );
/// assert_eq!(
///     app_strategy.runtime_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".frobnicator-plus/runtime/"))
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unix {
    // This is `.frobnicator-plus` in the above example.
    home_dir: PathBuf,
    unixy_name: String,
}

impl super::AppStrategy for Unix {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        Ok(Self {
            home_dir: crate::home_dir()?,
            unixy_name: format!(".{}", args.unixy_name()),
        })
    }

    fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    fn config_dir(&self) -> PathBuf {
        self.home_dir.join(&self.unixy_name)
    }

    fn data_dir(&self) -> PathBuf {
        self.home_dir.join(&self.unixy_name).join("data/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.home_dir.join(&self.unixy_name).join("cache/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        Some(self.home_dir.join(&self.unixy_name).join("state/"))
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        Some(self.home_dir.join(&self.unixy_name).join("runtime/"))
    }
}
