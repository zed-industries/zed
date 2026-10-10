use std::sync::Arc;
use std::{path::Path, str};

use gpui::{App, SharedString};
use settings_content::FolderIndicator;
use theme::{GlobalTheme, IconTheme, ThemeRegistry};
use util::paths::PathExt;

#[derive(Debug)]
pub struct FileIcons {
    icon_theme: Arc<IconTheme>,
}

/// What a panel draws ahead of a directory's name, in render order.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FolderIndicators {
    pub chevron: Option<SharedString>,
    pub icon: Option<SharedString>,
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

        if let Some(mut typ) = path.file_name().and_then(|typ| typ.to_str()) {
            // check if file name is in suffixes
            // e.g. catch file named `eslint.config.js` instead of `.eslint.config.js`
            let maybe_path = get_icon_from_suffix(typ);
            if maybe_path.is_some() {
                return maybe_path;
            }

            // check if stem (before first dot) is in stems
            // e.g. catch file named `Dockerfile.prod` or `Podfile.lock`
            if let Some((stem, _)) = typ.split_once('.') {
                let maybe_path = get_icon_from_suffix(stem);
                if maybe_path.is_some() {
                    return maybe_path;
                }
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

    /// Resolves what a panel should draw ahead of a directory's name. Shared by every
    /// panel that exposes a `folder_indicator` setting so they stay in agreement.
    pub fn get_folder_indicators(
        indicator: FolderIndicator,
        expanded: bool,
        path: &Path,
        cx: &App,
    ) -> FolderIndicators {
        let chevron = indicator
            .shows_chevron()
            .then(|| Self::get_chevron_icon(expanded, cx))
            .flatten();
        let icon = indicator
            .shows_icon()
            .then(|| Self::get_folder_icon(expanded, path, cx))
            .flatten();

        FolderIndicators { chevron, icon }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[gpui::test]
    fn test_folder_indicators_per_setting(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            theme::init(theme::LoadThemes::JustBase, cx);
        });

        let path = PathBuf::from("src");

        cx.update(|cx| {
            let icon_only =
                FileIcons::get_folder_indicators(FolderIndicator::Icon, false, &path, cx);
            assert_eq!(icon_only.chevron, None);
            assert_eq!(
                icon_only.icon,
                Some("icons/file_icons/folder.svg".into()),
                "`icon` should draw the folder icon and no chevron"
            );

            let chevron_only =
                FileIcons::get_folder_indicators(FolderIndicator::Chevron, false, &path, cx);
            assert_eq!(
                chevron_only.chevron,
                Some("icons/file_icons/chevron_right.svg".into())
            );
            assert_eq!(
                chevron_only.icon, None,
                "`chevron` should draw the chevron and no folder icon"
            );

            let both = FileIcons::get_folder_indicators(FolderIndicator::Both, false, &path, cx);
            assert_eq!(
                both.chevron,
                Some("icons/file_icons/chevron_right.svg".into())
            );
            assert_eq!(both.icon, Some("icons/file_icons/folder.svg".into()));
        });
    }

    #[gpui::test]
    fn test_folder_indicators_reflect_expanded_state(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            theme::init(theme::LoadThemes::JustBase, cx);
        });

        let path = PathBuf::from("src");

        cx.update(|cx| {
            let collapsed =
                FileIcons::get_folder_indicators(FolderIndicator::Both, false, &path, cx);
            assert_eq!(
                collapsed.chevron,
                Some("icons/file_icons/chevron_right.svg".into())
            );
            assert_eq!(collapsed.icon, Some("icons/file_icons/folder.svg".into()));

            let expanded = FileIcons::get_folder_indicators(FolderIndicator::Both, true, &path, cx);
            assert_eq!(
                expanded.chevron,
                Some("icons/file_icons/chevron_down.svg".into())
            );
            assert_eq!(
                expanded.icon,
                Some("icons/file_icons/folder_open.svg".into())
            );
        });
    }

    #[gpui::test]
    fn test_folder_indicator_default_is_icon(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            theme::init(theme::LoadThemes::JustBase, cx);
        });

        let path = PathBuf::from("src");

        cx.update(|cx| {
            let default =
                FileIcons::get_folder_indicators(FolderIndicator::default(), false, &path, cx);
            assert_eq!(
                default,
                FileIcons::get_folder_indicators(FolderIndicator::Icon, false, &path, cx),
                "the default must stay `icon` so existing users see no change"
            );
        });
    }
}
