#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir {
    pub prefix: DirPrefix,
    pub salt: String,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CacheDir {
    pub prefix: DirPrefix,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Include {
    pub prefix: DirPrefix,
    pub ignore_missing: bool,
    pub path: String,
}

/// This element contains a directory name where will be mapped as the path 'as-path' in cached information. This is useful if the directory name is an alias (via a bind mount or symlink) to another directory in the system for which cached font information is likely to exist.

/// 'salt' property affects to determine cache filename as same as [`Dir`] element.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemapDir {
    pub prefix: DirPrefix,
    pub as_path: String,
    pub salt: String,
    pub path: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DirPrefix {
    Default,
    Cwd,
    Xdg,
    Relative,
}

pub enum PrefixBehavior {
    Config,
    Cwd,
    Xdg,
    Relative,
}

parse_enum! {
    DirPrefix,
    (Default, "default"),
    (Cwd, "cwd"),
    (Xdg, "xdg"),
    (Relative, "relative"),
}

impl Default for DirPrefix {
    fn default() -> Self {
        DirPrefix::Default
    }
}

/// Get the location to user home directory.
///
/// This implementation follows `FcConfigHome` function of freedesktop.org's
/// Fontconfig library.
#[allow(unused_mut, clippy::let_and_return)]
fn config_home() -> Result<String, std::env::VarError> {
    let mut home = std::env::var("HOME");

    #[cfg(target_os = "windows")]
    {
        home = home.or_else(|_| std::env::var("USERPROFILE"));
    }

    home
}

/// Given a relative path to a config file, this function returns
/// the complete file name to load.
///
/// This is a simplified version of `FcConfigGetFilename` from the Fontconfig
/// library.
fn config_get_file_name(p: &std::path::PathBuf) -> std::path::PathBuf {
    if cfg!(target_os = "windows") {
        // TODO: get config file path properly for Windows
        return p.clone();
    } else {
        std::path::Path::new("/etc/fonts").join(p)
    }
}

fn expand_tilde(path: &String) -> std::path::PathBuf {
    let parsed_path = std::path::Path::new(path);
    if let Ok(stripped_path) = parsed_path.strip_prefix("~") {
        let home = config_home().unwrap_or("/".to_string());
        std::path::Path::new(&home).join(stripped_path)
    } else {
        parsed_path.into()
    }
}

macro_rules! define_calculate_path {
    ($ty:ident, $xdg_env:expr, $xdg_fallback:expr, $default_prefix_behavior:expr) => {
        impl $ty {
            /// Environment variable name which used `xdg` prefix
            pub const XDG_ENV: &'static str = $xdg_env;
            /// Fallback path when `XDG_ENV` is not exists
            pub const XDG_FALLBACK_PATH: &'static str = $xdg_fallback;
            const DEFAULT_PREFIX_BEHAVIOR: PrefixBehavior = $default_prefix_behavior;

            fn get_prefix_behavior(prefix: DirPrefix) -> PrefixBehavior {
                match prefix {
                    DirPrefix::Default => Self::DEFAULT_PREFIX_BEHAVIOR,
                    DirPrefix::Cwd => PrefixBehavior::Cwd,
                    DirPrefix::Xdg => PrefixBehavior::Xdg,
                    DirPrefix::Relative => PrefixBehavior::Relative,
                }
            }

            /// Calculate actual path
            pub fn calculate_path<P: AsRef<std::path::Path> + ?Sized>(
                &self,
                config_file_path: &P,
            ) -> std::path::PathBuf {
                let expanded_path = expand_tilde(&self.path);

                if expanded_path.is_absolute() {
                    return expanded_path;
                }

                let prefix = Self::get_prefix_behavior(self.prefix);

                match prefix {
                    PrefixBehavior::Config => config_get_file_name(&expanded_path),
                    PrefixBehavior::Cwd => std::path::Path::new(".").join(expanded_path),
                    PrefixBehavior::Relative => match config_file_path.as_ref().parent() {
                        Some(parent) => parent.join(expanded_path),
                        None => std::path::Path::new(".").join(expanded_path),
                    },
                    PrefixBehavior::Xdg => {
                        let xdg_path =
                            std::env::var($xdg_env).unwrap_or_else(|_| $xdg_fallback.into());
                        expand_tilde(&xdg_path).join(expanded_path)
                    }
                }
            }
        }
    };
}

define_calculate_path!(Dir, "XDG_DATA_HOME", "~/.local/share", PrefixBehavior::Cwd);
define_calculate_path!(CacheDir, "XDG_CACHE_HOME", "~/.cache", PrefixBehavior::Cwd);
define_calculate_path!(
    Include,
    "XDG_CONFIG_HOME",
    "~/.config",
    PrefixBehavior::Config
);
define_calculate_path!(
    RemapDir,
    "XDG_CONFIG_HOME",
    "~/.config",
    PrefixBehavior::Cwd
);
