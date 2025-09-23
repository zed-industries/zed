use std::sync::Arc;

use futures::StreamExt;
use settings::{DEFAULT_KEYMAP_PATH, KeymapFile, SettingsStore, watch_config_file};
use settings_ui::open_settings_editor;
use ui::BorrowAppContext;

fn main() {
    let app = gpui::Application::new().with_assets(assets::Assets);

    let fs = Arc::new(fs::RealFs::new(None, app.background_executor()));
    let mut user_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::settings_file().clone(),
    );
    zlog::init();
    zlog::init_output_stderr();

    app.run(move |cx| {
        <dyn fs::Fs>::set_global(fs.clone(), cx);
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        language::init(cx);
        editor::init(cx);
        menu::init();

        let keybindings =
            KeymapFile::load_asset_allow_partial_failure(DEFAULT_KEYMAP_PATH, cx).unwrap();
        cx.bind_keys(keybindings.into_iter());
        cx.spawn(async move |cx| {
            while let Some(content) = user_settings_file_rx.next().await {
                cx.update(|cx| {
                    cx.update_global(|store: &mut SettingsStore, cx| {
                        store.set_user_settings(&content, cx).unwrap()
                    })
                })
                .ok();
            }
        })
        .detach();

        open_settings_editor(cx).unwrap();
        cx.activate(true);
    });
}
