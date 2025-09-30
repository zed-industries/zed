use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionHostProxy, ExtensionThemeProxy};
use fs::Fs;
use gpui::{App, BackgroundExecutor, SharedString, Task};
use theme::{GlobalTheme, ThemeRegistry};

pub fn init(
    extension_host_proxy: Arc<ExtensionHostProxy>,
    theme_registry: Arc<ThemeRegistry>,
    executor: BackgroundExecutor,
) {
    extension_host_proxy.register_theme_proxy(ThemeRegistryProxy {
        theme_registry,
        executor,
    });
}

struct ThemeRegistryProxy {
    theme_registry: Arc<ThemeRegistry>,
    executor: BackgroundExecutor,
}

impl ExtensionThemeProxy for ThemeRegistryProxy {
    fn set_extensions_loaded(&self) {
        self.theme_registry.set_extensions_loaded();
    }

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

    fn reload_current_theme(&self, cx: &mut App) {
        GlobalTheme::reload_theme(cx)
    }

    fn list_icon_theme_names(
        &self,
        icon_theme_path: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<Vec<String>>> {
        self.executor.spawn(async move {
            let icon_theme_family = theme::read_icon_theme(&icon_theme_path, fs).await?;
            Ok(icon_theme_family
                .themes
                .into_iter()
                .map(|theme| theme.name)
                .collect())
        })
    }

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>) {
        self.theme_registry.remove_icon_themes(&icon_themes);
    }

    fn load_icon_theme(
        &self,
        icon_theme_path: PathBuf,
        icons_root_dir: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<()>> {
        let theme_registry = self.theme_registry.clone();
        self.executor.spawn(async move {
            theme_registry
                .load_icon_theme(&icon_theme_path, &icons_root_dir, fs)
                .await
        })
    }

    fn reload_current_icon_theme(&self, cx: &mut App) {
        GlobalTheme::reload_icon_theme(cx)
    }
}
