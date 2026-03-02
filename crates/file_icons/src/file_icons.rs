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
        // TODO: Associate a type with the languages and have the file's language
        //       override these associations

        // Check if file is a Helm/Helmfile configuration by verifying:
        // 1. File is in a templates/ subdirectory with Chart.yaml in parent directory
        // 2. File is in a helmfile.d/ subdirectory (Helmfile modular configuration)
        let path_str = path.to_str();
        let extension = path.extension().and_then(|e| e.to_str());

        // Check for helmfile.d directory pattern (Helmfile modular configuration)
        if path_str.map_or(false, |p| p.contains("/helmfile.d/"))
            && (extension == Some("yaml") || extension == Some("yml"))
        {
            if let Some(helm_icon) = this.get_icon_for_type("helm", cx) {
                return Some(helm_icon);
            }
        }

        // Check for templates directory pattern (Helm Charts)
        if path_str.map_or(false, |p| p.contains("/templates/"))
            && (extension == Some("yaml") || extension == Some("yml") || extension == Some("tpl"))
        {
            // Walk up the directory tree to find Chart.yaml
            let mut current = path.parent();
            while let Some(dir) = current {
                let chart_yaml = dir.join("Chart.yaml");
                let chart_yml = dir.join("Chart.yml");
                if chart_yaml.exists() || chart_yml.exists() {
                    if let Some(helm_icon) = this.get_icon_for_type("helm", cx) {
                        return Some(helm_icon);
                    }
                    break;
                }
                current = dir.parent();
            }
        }

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
