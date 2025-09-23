use std::{sync::Arc, time::Instant};

use futures::StreamExt;
use settings::{DEFAULT_KEYMAP_PATH, KeymapFile, SettingsStore, watch_config_file};
use settings_ui::open_settings_editor;
use ui::BorrowAppContext;

fn main() {
    let now = Instant::now();
    let app = gpui::Application::new().with_assets(assets::Assets);
    dbg!(now.elapsed());

    let fs = Arc::new(fs::RealFs::new(None, app.background_executor()));
    let mut user_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::settings_file().clone(),
    );
    dbg!(now.elapsed());
    zlog::init();
    zlog::init_output_stderr();
    dbg!(now.elapsed());

    app.run(move |cx| {
        dbg!(now.elapsed());
        <dyn fs::Fs>::set_global(fs.clone(), cx);
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        language::init(cx);
        editor::init(cx);
        menu::init();
        dbg!(now.elapsed());

        let keybindings =
            KeymapFile::load_asset_allow_partial_failure(DEFAULT_KEYMAP_PATH, cx).unwrap();
        cx.bind_keys(keybindings.into_iter());
        dbg!(now.elapsed());
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

        dbg!(now.elapsed());
        let handle = open_settings_editor(cx).unwrap();
        dbg!(now.elapsed());
        cx.spawn(async move |cx| {
            handle
                .update(cx, |_, window, _cx| {
                    window.activate_window();
                    dbg!(now.elapsed());
                })
                .unwrap();
        })
        .detach();
    });
}
