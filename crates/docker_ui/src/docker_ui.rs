mod docker_settings;

pub use docker_settings::DockerSettings;

use gpui::App;

pub fn init(_cx: &mut App) {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use settings::Settings;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    fn docker_settings_resolve_from_defaults(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let settings = DockerSettings::get_global(cx);
            assert_eq!(settings.poll_interval_seconds, 5);
            assert!(settings.endpoints.is_empty());
        });
    }
}
