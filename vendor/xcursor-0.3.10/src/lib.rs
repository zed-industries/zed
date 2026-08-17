//! A crate to load cursor themes, and parse XCursor files.

use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

/// A module implementing XCursor file parsing.
pub mod parser;

/// A cursor theme.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CursorTheme {
    theme: CursorThemeIml,
    /// Global search path for themes.
    search_paths: Vec<PathBuf>,
}

impl CursorTheme {
    /// Search for a theme with the given name in the given search paths,
    /// and returns an XCursorTheme which represents it. If no inheritance
    /// can be determined, then the themes inherits from the "default" theme.
    pub fn load(name: &str) -> Self {
        let search_paths = theme_search_paths(SearchPathsEnvironment::get());

        let theme = CursorThemeIml::load(name, &search_paths);

        CursorTheme {
            theme,
            search_paths,
        }
    }

    /// Try to load an icon from the theme.
    /// If the icon is not found within this theme's
    /// directories, then the function looks at the
    /// theme from which this theme is inherited.
    pub fn load_icon(&self, icon_name: &str) -> Option<PathBuf> {
        let mut walked_themes = HashSet::new();

        self.theme
            .load_icon_with_depth(icon_name, &self.search_paths, &mut walked_themes)
            .map(|(pathbuf, _)| pathbuf)
    }

    /// Try to load an icon from the theme, returning it with its inheritance
    /// depth.
    ///
    /// If the icon is not found within this theme's directories, then the
    /// function looks at the theme from which this theme is inherited. The
    /// second element of the returned tuple indicates how many levels of
    /// inheritance were traversed before the icon was found.
    pub fn load_icon_with_depth(&self, icon_name: &str) -> Option<(PathBuf, usize)> {
        let mut walked_themes = HashSet::new();

        self.theme
            .load_icon_with_depth(icon_name, &self.search_paths, &mut walked_themes)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CursorThemeIml {
    /// Theme name.
    name: String,
    /// Directories where the theme is presented and corresponding names of inherited themes.
    /// `None` if theme inherits nothing.
    data: Vec<(PathBuf, Option<String>)>,
}

impl CursorThemeIml {
    /// The implementation of cursor theme loading.
    fn load(name: &str, search_paths: &[PathBuf]) -> Self {
        let mut data = Vec::new();

        // Find directories where this theme is presented.
        for mut path in search_paths.iter().cloned() {
            path.push(name);
            if path.is_dir() {
                let data_dir = path.clone();

                path.push("index.theme");
                let inherits = if let Some(inherits) = theme_inherits(&path) {
                    Some(inherits)
                } else if name != "default" {
                    Some(String::from("default"))
                } else {
                    None
                };

                data.push((data_dir, inherits));
            }
        }

        CursorThemeIml {
            name: name.to_owned(),
            data,
        }
    }

    /// The implementation of cursor icon loading.
    fn load_icon_with_depth(
        &self,
        icon_name: &str,
        search_paths: &[PathBuf],
        walked_themes: &mut HashSet<String>,
    ) -> Option<(PathBuf, usize)> {
        for data in &self.data {
            let mut icon_path = data.0.clone();
            icon_path.push("cursors");
            icon_path.push(icon_name);
            if icon_path.is_file() {
                return Some((icon_path, 0));
            }
        }

        // We've processed all based theme files. Traverse inherited themes, marking this theme
        // as already visited to avoid infinite recursion.
        walked_themes.insert(self.name.clone());

        for data in &self.data {
            // Get inherited theme name, if any.
            let inherits = match data.1.as_ref() {
                Some(inherits) => inherits,
                None => continue,
            };

            // We've walked this theme, avoid rebuilding.
            if walked_themes.contains(inherits) {
                continue;
            }

            let inherited_theme = CursorThemeIml::load(inherits, search_paths);

            match inherited_theme.load_icon_with_depth(icon_name, search_paths, walked_themes) {
                Some((icon_path, depth)) => return Some((icon_path, depth + 1)),
                None => continue,
            }
        }

        None
    }
}

#[derive(Default)]
struct SearchPathsEnvironment {
    home: Option<String>,
    xcursor_path: Option<String>,
    xdg_data_home: Option<String>,
    xdg_data_dirs: Option<String>,
}

impl SearchPathsEnvironment {
    fn get() -> Self {
        SearchPathsEnvironment {
            home: env::var("HOME").ok().filter(|x| !x.is_empty()),
            xcursor_path: env::var("XCURSOR_PATH").ok().filter(|x| !x.is_empty()),
            xdg_data_home: env::var("XDG_DATA_HOME").ok().filter(|x| !x.is_empty()),
            xdg_data_dirs: env::var("XDG_DATA_DIRS").ok().filter(|x| !x.is_empty()),
        }
    }
}

/// Get the list of paths where the themes have to be searched, according to the XDG Icon Theme
/// specification. If `XCURSOR_PATH` is set, it will override the default search paths.
fn theme_search_paths(environment: SearchPathsEnvironment) -> Vec<PathBuf> {
    let home_dir = environment
        .home
        .as_ref()
        .map(|home| Path::new(home.as_str()));

    if let Some(xcursor_path) = environment.xcursor_path {
        return xcursor_path
            .split(':')
            .flat_map(|entry| {
                if entry.is_empty() {
                    return None;
                }
                expand_home_dir(PathBuf::from(entry), home_dir)
            })
            .collect();
    }

    // The order is following other XCursor loading libs, like libwayland-cursor.
    let mut paths = Vec::new();

    if let Some(xdg_data_home) = environment.xdg_data_home {
        paths.extend(expand_home_dir(PathBuf::from(xdg_data_home), home_dir));
    } else if let Some(home_dir) = home_dir {
        paths.push(home_dir.join(".local/share/icons"))
    }

    if let Some(home_dir) = home_dir {
        paths.push(home_dir.join(".icons"));
    }

    if let Some(xdg_data_dirs) = environment.xdg_data_dirs {
        paths.extend(xdg_data_dirs.split(':').flat_map(|entry| {
            if entry.is_empty() {
                return None;
            }
            let mut entry = expand_home_dir(PathBuf::from(entry), home_dir)?;
            entry.push("icons");
            Some(entry)
        }))
    } else {
        paths.push(PathBuf::from("/usr/local/share/icons"));
        paths.push(PathBuf::from("/usr/share/icons"));
    }

    paths.push(PathBuf::from("/usr/share/pixmaps"));

    if let Some(home_dir) = home_dir {
        paths.push(home_dir.join(".cursors"));
    }

    paths.push(PathBuf::from("/usr/share/cursors/xorg-x11"));

    paths
}

/// If the first component of the path is `~`, replaces it with the home dir. If no home dir is
/// present, returns `None`.
fn expand_home_dir(path: PathBuf, home_dir: Option<&Path>) -> Option<PathBuf> {
    let mut components = path.iter();
    if let Some(first_component) = components.next() {
        if first_component == "~" {
            if let Some(home_dir) = home_dir {
                let mut path = home_dir.to_path_buf();
                for component in components {
                    path.push(component);
                }
                return Some(path);
            } else {
                return None;
            }
        }
    }
    Some(path)
}

/// Load the specified index.theme file, and returns a `Some` with
/// the value of the `Inherits` key in it.
/// Returns `None` if the file cannot be read for any reason,
/// if the file cannot be parsed, or if the `Inherits` key is omitted.
fn theme_inherits(file_path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(file_path).ok()?;

    parse_theme(&content)
}

/// Parse the content of the `index.theme` and return the `Inherits` value.
fn parse_theme(content: &str) -> Option<String> {
    const PATTERN: &str = "Inherits";

    let is_xcursor_space_or_separator =
        |&ch: &char| -> bool { ch.is_whitespace() || ch == ';' || ch == ',' };

    for line in content.lines() {
        // Line should start with `Inherits`, otherwise go to the next line.
        if !line.starts_with(PATTERN) {
            continue;
        }

        // Skip the `Inherits` part and trim the leading white spaces.
        let mut chars = line.get(PATTERN.len()..).unwrap().trim_start().chars();

        // If the next character after leading white spaces isn't `=` go the next line.
        if Some('=') != chars.next() {
            continue;
        }

        // Skip XCursor spaces/separators.
        let result: String = chars
            .skip_while(is_xcursor_space_or_separator)
            .take_while(|ch| !is_xcursor_space_or_separator(ch))
            .collect();

        if !result.is_empty() {
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_parse_theme() {
        let theme_name = String::from("XCURSOR_RS");

        let theme = format!("Inherits={}", theme_name.clone());

        assert_eq!(parse_theme(&theme), Some(theme_name.clone()));

        let theme = format!(" Inherits={}", theme_name.clone());

        assert_eq!(parse_theme(&theme), None);

        let theme = format!(
            "[THEME name]\nInherits   = ,;\t\t{};;;;Tail\n\n",
            theme_name.clone()
        );

        assert_eq!(parse_theme(&theme), Some(theme_name.clone()));

        let theme = format!("Inherits;=;{}", theme_name.clone());

        assert_eq!(parse_theme(&theme), None);

        let theme = format!("Inherits = {}\n\nInherits=OtherTheme", theme_name.clone());

        assert_eq!(parse_theme(&theme), Some(theme_name.clone()));

        let theme = format!(
            "Inherits = ;;\nSome\tgarbage\nInherits={}",
            theme_name.clone()
        );

        assert_eq!(parse_theme(&theme), Some(theme_name.clone()));
    }

    #[test]
    fn test_expand_home_dir() {
        let home = Path::new("/home/user");

        let result = expand_home_dir("~".into(), Some(home));
        assert_eq!(result, Some("/home/user".into()));

        let result = expand_home_dir("~/.icons".into(), Some(home));
        assert_eq!(result, Some("/home/user/.icons".into()));

        let result = expand_home_dir("~/.local/share/icons".into(), Some(home));
        assert_eq!(result, Some("/home/user/.local/share/icons".into()));

        let result = expand_home_dir("~/.icons".into(), None);
        assert_eq!(result, None);

        let path: PathBuf = "/usr/share/icons".into();
        let result = expand_home_dir(path.clone(), Some(home));
        assert_eq!(result, Some(path));

        let path: PathBuf = "".into();
        let result = expand_home_dir(path.clone(), Some(home));
        assert_eq!(result, Some(path));

        // ~ in the middle of path should not expand
        let path: PathBuf = "/some/path/~/icons".into();
        let result = expand_home_dir(path.clone(), Some(home));
        assert_eq!(result, Some(path));
    }

    #[test]
    fn test_theme_search_paths() {
        assert_eq!(
            theme_search_paths(SearchPathsEnvironment {
                home: Some("/home/user".to_string()),
                xdg_data_home: Some("/home/user/.data".to_string()),
                xdg_data_dirs: Some("/opt/share::/usr/local/share:~/custom/share".to_string()),
                ..Default::default()
            }),
            vec![
                PathBuf::from("/home/user/.data"),
                PathBuf::from("/home/user/.icons"),
                PathBuf::from("/opt/share/icons"),
                PathBuf::from("/usr/local/share/icons"),
                PathBuf::from("/home/user/custom/share/icons"),
                PathBuf::from("/usr/share/pixmaps"),
                PathBuf::from("/home/user/.cursors"),
                PathBuf::from("/usr/share/cursors/xorg-x11"),
            ]
        );

        // XCURSOR_PATH overrides all other paths
        assert_eq!(
            theme_search_paths(SearchPathsEnvironment {
                home: Some("/home/user".to_string()),
                xcursor_path: Some("~/custom/xcursor/icons:/absolute-path/icons".to_string()),
                ..Default::default()
            }),
            vec![
                PathBuf::from("/home/user/custom/xcursor/icons"),
                PathBuf::from("/absolute-path/icons")
            ]
        );

        // no home causes tilde paths to be omitted
        assert_eq!(
            theme_search_paths(SearchPathsEnvironment {
                xdg_data_home: Some("~/.data".to_string()),
                ..Default::default()
            }),
            vec![
                PathBuf::from("/usr/local/share/icons"),
                PathBuf::from("/usr/share/icons"),
                PathBuf::from("/usr/share/pixmaps"),
                PathBuf::from("/usr/share/cursors/xorg-x11"),
            ]
        );
    }
}
