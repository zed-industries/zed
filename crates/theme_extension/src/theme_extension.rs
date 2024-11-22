use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionChangeListeners, OnThemeExtensionChange};
use fs::Fs;
use gpui::{AppContext, BackgroundExecutor, SharedString, Task};
use theme::{ThemeRegistry, ThemeSettings};

pub fn init(
    extension_change_listeners: Arc<ExtensionChangeListeners>,
    theme_registry: Arc<ThemeRegistry>,
    executor: BackgroundExecutor,
) {
    extension_change_listeners.register_theme_listener(ExtensionThemeListener {
        theme_registry,
        executor,
    });
}

struct ExtensionThemeListener {
    theme_registry: Arc<ThemeRegistry>,
    executor: BackgroundExecutor,
}

impl OnThemeExtensionChange for ExtensionThemeListener {
    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        self.executor.spawn(async move {
            let themes = theme::read_user_theme(&theme_path, fs).await?;
            Ok(themes.themes.into_iter().map(|theme| theme.name).collect())
        })
    }

    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        self.theme_registry.remove_user_themes(&themes);
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let theme_registry = self.theme_registry.clone();
        self.executor
            .spawn(async move { theme_registry.load_user_theme(&theme_path, fs).await })
    }

    fn reload_current_theme(&self, cx: &mut AppContext) {
        ThemeSettings::reload_current_theme(cx)
    }
}
