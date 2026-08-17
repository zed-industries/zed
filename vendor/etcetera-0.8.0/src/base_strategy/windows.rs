use std::path::{Path, PathBuf};

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern. The specification is available [here](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid).
///
/// This initial example removes all the relevant environment variables to show the strategy’s use of the:
/// - (on Windows) SHGetFolderPathW API.
/// - (on non-Windows) Windows default directories.
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Windows;
/// use std::path::Path;
///
/// // Remove the environment variables that the strategy reads from.
/// std::env::remove_var("USERPROFILE");
/// std::env::remove_var("APPDATA");
/// std::env::remove_var("LOCALAPPDATA");
///
/// let base_strategy = Windows::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     base_strategy.home_dir(),
///     &home_dir
/// );
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/"))
/// );
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/"))
/// );
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Local/"))
/// );
/// assert_eq!(
///     base_strategy.state_dir(),
///     None
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
/// use etcetera::base_strategy::Windows;
/// use std::path::Path;
///
/// let home_path = if cfg!(windows) {
///     "C:\\foo\\".to_string()
/// } else {
///     etcetera::home_dir().unwrap().to_string_lossy().to_string()
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
///
/// std::env::set_var("USERPROFILE", &home_path);
/// std::env::set_var("APPDATA", data_path);
/// std::env::set_var("LOCALAPPDATA", cache_path);
///
/// let base_strategy = Windows::new().unwrap();
///
/// assert_eq!(
///     base_strategy.home_dir(),
///     Path::new(&home_path)
/// );
/// assert_eq!(
///     base_strategy.config_dir(),
///     Path::new(data_path)
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
///     base_strategy.state_dir(),
///     None
/// );
/// assert_eq!(
///     base_strategy.runtime_dir(),
///     None
/// );
/// ```

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Windows {
    home_dir: PathBuf,
}

// Ref: https://github.com/rust-lang/cargo/blob/home-0.5.5/crates/home/src/windows.rs
// We should keep this code in sync with the above.
impl Windows {
    fn dir_inner(env: &'static str) -> Option<PathBuf> {
        std::env::var_os(env)
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .or_else(|| Self::dir_crt(env))
    }

    #[cfg(all(windows, target_vendor = "uwp"))]
    fn dir_crt(env: &'static str) -> Option<PathBuf> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        use windows_sys::Win32::Foundation::{MAX_PATH, S_OK};
        use windows_sys::Win32::UI::Shell::{SHGetFolderPathW, CSIDL_APPDATA, CSIDL_LOCAL_APPDATA};

        let csidl = match env {
            "APPDATA" => CSIDL_APPDATA,
            "LOCALAPPDATA" => CSIDL_LOCAL_APPDATA,
            _ => return None,
        };

        extern "C" {
            fn wcslen(buf: *const u16) -> usize;
        }

        unsafe {
            let mut path: Vec<u16> = Vec::with_capacity(MAX_PATH as usize);
            match SHGetFolderPathW(0, csidl, 0, 0, path.as_mut_ptr()) {
                S_OK => {
                    let len = wcslen(path.as_ptr());
                    path.set_len(len);
                    let s = OsString::from_wide(&path);
                    Some(PathBuf::from(s))
                }
                _ => None,
            }
        }
    }

    #[cfg(not(all(windows, target_vendor = "uwp")))]
    fn dir_crt(_env: &'static str) -> Option<PathBuf> {
        None
    }
}

impl super::BaseStrategy for Windows {
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
        self.data_dir()
    }

    fn data_dir(&self) -> PathBuf {
        Self::dir_inner("APPDATA").unwrap_or_else(|| self.home_dir.join("AppData").join("Roaming"))
    }

    fn cache_dir(&self) -> PathBuf {
        Self::dir_inner("LOCALAPPDATA")
            .unwrap_or_else(|| self.home_dir.join("AppData").join("Local"))
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        None
    }
}
