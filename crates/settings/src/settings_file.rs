use crate::{settings_store::SettingsStore, Settings};
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui::{AppContext, BackgroundExecutor, ReadGlobal, UpdateGlobal};
use std::{path::PathBuf, sync::Arc, time::Duration};
use util::ResultExt;

pub const EMPTY_THEME_NAME: &str = "empty-theme";

#[cfg(any(test, feature = "test-support"))]
pub fn test_settings() -> String {
    let mut value = crate::settings_store::parse_json_with_comments::<serde_json::Value>(
        crate::default_settings().as_ref(),
    )
    .unwrap();
    util::merge_non_null_json_value_into(
        serde_json::json!({
            "ui_font_family": "Courier",
            "ui_font_features": {},
            "ui_font_size": 14,
            "ui_font_fallback": [],
            "buffer_font_family": "Courier",
            "buffer_font_features": {},
            "buffer_font_size": 14,
            "buffer_font_fallback": [],
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
            let (events, _) = fs.watch(&path, Duration::from_millis(100)).await;
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
                    if tx.unbounded_send(contents).is_err() {
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
    settings_changed: impl Fn(Option<anyhow::Error>, &mut AppContext) + 'static,
) {
    let user_settings_content = cx
        .background_executor()
        .block(user_settings_file_rx.next())
        .unwrap();
    SettingsStore::update_global(cx, |store, cx| {
        store
            .set_user_settings(&user_settings_content, cx)
            .log_err();
    });
    cx.spawn(move |cx| async move {
        while let Some(user_settings_content) = user_settings_file_rx.next().await {
            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                let result = store.set_user_settings(&user_settings_content, cx);
                if let Err(err) = &result {
                    log::error!("Failed to load user settings: {err}");
                }
                settings_changed(result.err(), cx);
                cx.refresh();
            });
            if result.is_err() {
                break; // App dropped
            }
        }
    })
    .detach();
}

pub fn update_settings_file<T: Settings>(
    fs: Arc<dyn Fs>,
    cx: &AppContext,
    update: impl 'static + Send + FnOnce(&mut T::FileContent, &AppContext),
) {
    SettingsStore::global(cx).update_settings_file::<T>(fs, update);
}
