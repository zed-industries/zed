use crate::{settings_store::SettingsStore, KeymapFile, Settings};
use anyhow::Result;
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui::{AppContext, BackgroundExecutor};
use std::{io::ErrorKind, path::PathBuf, str, sync::Arc, time::Duration};
use util::{paths, ResultExt};

pub const EMPTY_THEME_NAME: &'static str = "empty-theme";

#[cfg(any(test, feature = "test-support"))]
pub fn test_settings() -> String {
    let mut value = crate::settings_store::parse_json_with_comments::<serde_json::Value>(
        crate::default_settings().as_ref(),
    )
    .unwrap();
    util::merge_non_null_json_value_into(
        serde_json::json!({
            "buffer_font_family": "Courier",
            "buffer_font_features": {},
            "buffer_font_size": 14,
            "theme": EMPTY_THEME_NAME,
        }),
        &mut value,
    );
    value.as_object_mut().unwrap().remove("languages");
    serde_json::to_string(&value).unwrap()
}

pub fn watch_config_file(
    executor: &BackgroundExecutor,
    fs: Arc<dyn Fs>,
    path: PathBuf,
) -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded();
    executor
        .spawn(async move {
            let events = fs.watch(&path, Duration::from_millis(100)).await;
            futures::pin_mut!(events);

            let contents = fs.load(&path).await.unwrap_or_default();
            if tx.unbounded_send(contents).is_err() {
                return;
            }

            loop {
                if events.next().await.is_none() {
                    break;
                }

                if let Ok(contents) = fs.load(&path).await {
                    if !tx.unbounded_send(contents).is_ok() {
                        break;
                    }
                }
            }
        })
        .detach();
    rx
}

pub fn handle_settings_file_changes(
    mut user_settings_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    let user_settings_content = cx
        .background_executor()
        .block(user_settings_file_rx.next())
        .unwrap();
    cx.update_global(|store: &mut SettingsStore, cx| {
        store
            .set_user_settings(&user_settings_content, cx)
            .log_err();
    });
    cx.spawn(move |mut cx| async move {
        while let Some(user_settings_content) = user_settings_file_rx.next().await {
            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                store
                    .set_user_settings(&user_settings_content, cx)
                    .log_err();
                cx.refresh();
            });
            if result.is_err() {
                break; // App dropped
            }
        }
    })
    .detach();
}

async fn load_settings(fs: &Arc<dyn Fs>) -> Result<String> {
    match fs.load(&paths::SETTINGS).await {
        result @ Ok(_) => result,
        Err(err) => {
            if let Some(e) = err.downcast_ref::<std::io::Error>() {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(crate::initial_user_settings_content().to_string());
                }
            }
            return Err(err);
        }
    }
}

pub fn update_settings_file<T: Settings>(
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
    update: impl 'static + Send + FnOnce(&mut T::FileContent),
) {
    cx.spawn(|cx| async move {
        let old_text = load_settings(&fs).await?;
        let new_text = cx.read_global(|store: &SettingsStore, _cx| {
            store.new_text_for_update::<T>(old_text, update)
        })?;
        fs.atomic_write(paths::SETTINGS.clone(), new_text).await?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

pub fn load_default_keymap(cx: &mut AppContext) {
    for path in ["keymaps/default.json", "keymaps/vim.json"] {
        KeymapFile::load_asset(path, cx).unwrap();
    }

    // todo!()
    // if let Some(asset_path) = settings::get::<BaseKeymap>(cx).asset_path() {
    //     KeymapFile::load_asset(asset_path, cx).unwrap();
    // }
}

pub fn handle_keymap_file_changes(
    mut user_keymap_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    cx.spawn(move |cx| async move {
        //  let mut settings_subscription = None;
        while let Some(user_keymap_content) = user_keymap_file_rx.next().await {
            if let Some(keymap_content) = KeymapFile::parse(&user_keymap_content).log_err() {
                cx.update(|cx| reload_keymaps(cx, &keymap_content)).ok();

                // todo!()
                // let mut old_base_keymap = cx.read(|cx| *settings::get::<BaseKeymap>(cx));
                // drop(settings_subscription);
                // settings_subscription = Some(cx.update(|cx| {
                //     cx.observe_global::<SettingsStore, _>(move |cx| {
                //         let new_base_keymap = *settings::get::<BaseKeymap>(cx);
                //         if new_base_keymap != old_base_keymap {
                //             old_base_keymap = new_base_keymap.clone();
                //             reload_keymaps(cx, &keymap_content);
                //         }
                //     })
                // }));
            }
        }
    })
    .detach();
}

fn reload_keymaps(cx: &mut AppContext, keymap_content: &KeymapFile) {
    // todo!()
    // cx.clear_bindings();
    load_default_keymap(cx);
    keymap_content.clone().add_to_cx(cx).log_err();
    // cx.set_menus(menus::menus());
}
