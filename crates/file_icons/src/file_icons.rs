use std::sync::Arc;
use std::{path::Path, str};

use gpui::{App, SharedString};
use theme::{GlobalTheme, IconTheme, ThemeRegistry};
use util::paths::PathExt;

#[derive(Debug)]
pub struct FileIcons {
    icon_theme: Arc<IconTheme>,
}

impl FileIcons {
    pub fn get(cx: &App) -> Self {
        Self {
            icon_theme: GlobalTheme::icon_theme(cx).clone(),
        }
    }

    pub fn get_icon(path: &Path, cx: &App) -> Option<SharedString> {
        let this = Self::get(cx);

        let get_icon_from_suffix = |suffix: &str| -> Option<SharedString> {
            this.icon_theme
                .file_stems
                .get(suffix)
                .or_else(|| this.icon_theme.file_suffixes.get(suffix))
                .and_then(|typ| this.get_icon_for_type(typ, cx))
        };
        if let Some(mut typ) = path.file_name().and_then(|typ| typ.to_str()) {
            // check if file name is in suffixes
            // e.g. catch file named `eslint.config.js` instead of `.eslint.config.js`
            let maybe_path = get_icon_from_suffix(typ);
            if maybe_path.is_some() {
                return maybe_path;
            }

            // check if suffix based on first dot is in suffixes
            // e.g. consider `module.js` as suffix to angular's module file named `auth.module.js`
            while let Some((_, suffix)) = typ.split_once('.') {
                let maybe_path = get_icon_from_suffix(suffix);
                if maybe_path.is_some() {
                    return maybe_path;
                }
                typ = suffix;
            }
        }

        // handle cases where the file extension is made up of multiple important
        // parts (e.g Component.stories.tsx) that refer to an alternative icon style
        if let Some(suffix) = path.multiple_extensions() {
            let maybe_path = get_icon_from_suffix(suffix.as_str());
            if maybe_path.is_some() {
                return maybe_path;
            }
        }

        // primary case: check if the files extension or the hidden file name
        // matches some icon path
        if let Some(suffix) = path.extension_or_hidden_file_name() {
            let maybe_path = get_icon_from_suffix(suffix);
            if maybe_path.is_some() {
                return maybe_path;
            }
        }

        // this _should_ only happen when the file is hidden (has leading '.')
        // and is not a "special" file we have an icon (e.g. not `.eslint.config.js`)
        // that should be caught above. In the remaining cases, we want to check
        // for a normal supported extension e.g. `.data.json` -> `json`
        let extension = path.extension().and_then(|ext| ext.to_str());
        if let Some(extension) = extension {
            let maybe_path = get_icon_from_suffix(extension);
            if maybe_path.is_some() {
                return maybe_path;
            }
        }
        this.get_icon_for_type("default", cx)
    }

    /// Resolves a file icon based on a language name.
    ///
    /// Maps the language name to an icon theme key (e.g., "Rust" → "rust",
    /// "C++" → "cpp") and looks it up in the icon theme. Returns `None` if
    /// no matching icon is found.
    pub fn get_icon_for_language(language_name: &str, cx: &App) -> Option<SharedString> {
        let icon_key = Self::language_name_to_icon_key(language_name);
        let this = Self::get(cx);
        this.get_icon_for_type(&icon_key, cx)
    }

    fn language_name_to_icon_key(language_name: &str) -> String {
        match language_name {
            "C++" => "cpp".to_string(),
            "C#" => "csharp".to_string(),
            "F#" => "fsharp".to_string(),
            "Shell Script" => "terminal".to_string(),
            "TSX" => "react".to_string(),
            "JSONC" => "json".to_string(),
            _ => language_name.to_lowercase(),
        }
    }

    fn default_icon_theme(cx: &App) -> Option<Arc<IconTheme>> {
        let theme_registry = ThemeRegistry::global(cx);
        theme_registry.default_icon_theme().ok()
    }

    pub fn get_icon_for_type(&self, typ: &str, cx: &App) -> Option<SharedString> {
        fn get_icon_for_type(icon_theme: &Arc<IconTheme>, typ: &str) -> Option<SharedString> {
            icon_theme
                .file_icons
                .get(typ)
                .map(|icon_definition| icon_definition.path.clone())
        }

        get_icon_for_type(GlobalTheme::icon_theme(cx), typ).or_else(|| {
            Self::default_icon_theme(cx).and_then(|icon_theme| get_icon_for_type(&icon_theme, typ))
        })
    }

    pub fn get_folder_icon(expanded: bool, path: &Path, cx: &App) -> Option<SharedString> {
        fn get_folder_icon(
            icon_theme: &Arc<IconTheme>,
            path: &Path,
            expanded: bool,
        ) -> Option<SharedString> {
            let name = path.file_name()?.to_str()?.trim();
            if name.is_empty() {
                return None;
            }

            let directory_icons = icon_theme.named_directory_icons.get(name)?;

            if expanded {
                directory_icons.expanded.clone()
            } else {
                directory_icons.collapsed.clone()
            }
        }

        get_folder_icon(GlobalTheme::icon_theme(cx), path, expanded)
            .or_else(|| {
                Self::default_icon_theme(cx)
                    .and_then(|icon_theme| get_folder_icon(&icon_theme, path, expanded))
            })
            .or_else(|| {
                // If we can't find a specific folder icon for the folder at the given path, fall back to the generic folder
                // icon.
                Self::get_generic_folder_icon(expanded, cx)
            })
    }

    fn get_generic_folder_icon(expanded: bool, cx: &App) -> Option<SharedString> {
        fn get_generic_folder_icon(
            icon_theme: &Arc<IconTheme>,
            expanded: bool,
        ) -> Option<SharedString> {
            if expanded {
                icon_theme.directory_icons.expanded.clone()
            } else {
                icon_theme.directory_icons.collapsed.clone()
            }
        }

        get_generic_folder_icon(GlobalTheme::icon_theme(cx), expanded).or_else(|| {
            Self::default_icon_theme(cx)
                .and_then(|icon_theme| get_generic_folder_icon(&icon_theme, expanded))
        })
    }

    pub fn get_chevron_icon(expanded: bool, cx: &App) -> Option<SharedString> {
        fn get_chevron_icon(icon_theme: &Arc<IconTheme>, expanded: bool) -> Option<SharedString> {
            if expanded {
                icon_theme.chevron_icons.expanded.clone()
            } else {
                icon_theme.chevron_icons.collapsed.clone()
            }
        }

        get_chevron_icon(GlobalTheme::icon_theme(cx), expanded).or_else(|| {
            Self::default_icon_theme(cx)
                .and_then(|icon_theme| get_chevron_icon(&icon_theme, expanded))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[gpui::test]
    fn test_get_icon_for_known_extensions(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let rust_icon = FileIcons::get_icon(Path::new("main.rs"), cx);
            assert!(rust_icon.is_some(), "Rust files should have an icon");

            let python_icon = FileIcons::get_icon(Path::new("script.py"), cx);
            assert!(python_icon.is_some(), "Python files should have an icon");

            let js_icon = FileIcons::get_icon(Path::new("app.js"), cx);
            assert!(js_icon.is_some(), "JavaScript files should have an icon");

            let ts_icon = FileIcons::get_icon(Path::new("index.ts"), cx);
            assert!(ts_icon.is_some(), "TypeScript files should have an icon");
        });
    }

    #[gpui::test]
    fn test_get_icon_for_unknown_extension_returns_default(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let icon = FileIcons::get_icon(Path::new("file.xyz_unknown_ext"), cx);
            let default_icon = FileIcons::get(cx).get_icon_for_type("default", cx);
            assert_eq!(icon, default_icon);
        });
    }

    #[gpui::test]
    fn test_get_icon_for_language_known_languages(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let rust_icon = FileIcons::get_icon_for_language("Rust", cx);
            assert!(
                rust_icon.is_some(),
                "Rust language should resolve to an icon"
            );

            let python_icon = FileIcons::get_icon_for_language("Python", cx);
            assert!(
                python_icon.is_some(),
                "Python language should resolve to an icon"
            );

            let go_icon = FileIcons::get_icon_for_language("Go", cx);
            assert!(go_icon.is_some(), "Go language should resolve to an icon");
        });
    }

    #[gpui::test]
    fn test_get_icon_for_language_special_cases(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let cpp_icon = FileIcons::get_icon_for_language("C++", cx);
            assert!(cpp_icon.is_some(), "C++ should map to cpp icon");

            let csharp_icon = FileIcons::get_icon_for_language("C#", cx);
            assert!(csharp_icon.is_some(), "C# should map to csharp icon");

            let fsharp_icon = FileIcons::get_icon_for_language("F#", cx);
            assert!(fsharp_icon.is_some(), "F# should map to fsharp icon");

            let shell_icon = FileIcons::get_icon_for_language("Shell Script", cx);
            assert!(
                shell_icon.is_some(),
                "Shell Script should map to terminal icon"
            );

            let tsx_icon = FileIcons::get_icon_for_language("TSX", cx);
            assert!(tsx_icon.is_some(), "TSX should map to react icon");
        });
    }

    #[gpui::test]
    fn test_get_icon_for_language_matches_extension_icon(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let icon_by_language = FileIcons::get_icon_for_language("Rust", cx);
            let icon_by_extension = FileIcons::get_icon(Path::new("test.rs"), cx);
            assert_eq!(
                icon_by_language, icon_by_extension,
                "Language-based and extension-based icons should match for Rust"
            );

            let icon_by_language = FileIcons::get_icon_for_language("Python", cx);
            let icon_by_extension = FileIcons::get_icon(Path::new("test.py"), cx);
            assert_eq!(
                icon_by_language, icon_by_extension,
                "Language-based and extension-based icons should match for Python"
            );
        });
    }

    #[gpui::test]
    fn test_get_icon_for_language_unknown_returns_none(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let icon = FileIcons::get_icon_for_language("NonExistentLanguage", cx);
            assert!(icon.is_none(), "Unknown language should return None");
        });
    }

    #[test]
    fn test_language_name_to_icon_key() {
        assert_eq!(FileIcons::language_name_to_icon_key("Rust"), "rust");
        assert_eq!(FileIcons::language_name_to_icon_key("Python"), "python");
        assert_eq!(
            FileIcons::language_name_to_icon_key("JavaScript"),
            "javascript"
        );
        assert_eq!(FileIcons::language_name_to_icon_key("C++"), "cpp");
        assert_eq!(FileIcons::language_name_to_icon_key("C#"), "csharp");
        assert_eq!(FileIcons::language_name_to_icon_key("F#"), "fsharp");
        assert_eq!(
            FileIcons::language_name_to_icon_key("Shell Script"),
            "terminal"
        );
        assert_eq!(FileIcons::language_name_to_icon_key("TSX"), "react");
        assert_eq!(FileIcons::language_name_to_icon_key("JSONC"), "json");
        assert_eq!(FileIcons::language_name_to_icon_key("Go"), "go");
        assert_eq!(
            FileIcons::language_name_to_icon_key("TypeScript"),
            "typescript"
        );
    }
}
