use std::sync::Arc;

use futures::StreamExt;
use settings::{SettingsStore, default_settings, watch_config_file};
use settings_ui::open_settings_editor;
use ui::BorrowAppContext;

fn main() {
    let app = gpui::Application::new().with_assets(assets::Assets);

    let fs = Arc::new(fs::RealFs::new(None, app.background_executor()));
    let user_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::settings_file().clone(),
    );
    let global_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::global_settings_file().clone(),
    );

    app.run(|cx| {
        let store = SettingsStore::new(cx, &default_settings());
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);

        cx.spawn(async move |cx| {
            let mut settings_streams = futures::stream::select(
                global_settings_file_rx.map(|content| (content, false)),
                user_settings_file_rx.map(|content| (content, true)),
            );

            while let Some((content, is_user)) = settings_streams.next().await {
                cx.update(|cx| {
                    cx.update_global(|store: &mut SettingsStore, cx| {
                        if is_user {
                            store.set_user_settings(&content, cx).unwrap()
                        } else {
                            store.set_global_settings(&content, cx).unwrap()
                        };
                        cx.refresh_windows();
                    })
                })
                .ok();
            }
        })
        .detach();

        let handle = open_settings_editor(cx).unwrap();
        cx.spawn(async move |cx| {
            handle
                .update(cx, |_, window, _cx| {
                    window.activate_window();
                })
                .unwrap();
        })
        .detach();
    });
}
