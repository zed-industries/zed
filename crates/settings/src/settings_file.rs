use crate::{settings_store::SettingsStore, Settings};
use anyhow::{Context, Result};
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui::{AppContext, BackgroundExecutor, UpdateGlobal};
use std::{io::ErrorKind, path::PathBuf, sync::Arc, time::Duration};
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
    match fs.load(paths::settings_file()).await {
        result @ Ok(_) => result,
        Err(err) => {
            if let Some(e) = err.downcast_ref::<std::io::Error>() {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(crate::initial_user_settings_content().to_string());
                }
            }
            Err(err)
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
        let initial_path = paths::settings_file().as_path();
        if fs.is_file(initial_path).await {
            let resolved_path = fs.canonicalize(initial_path).await.with_context(|| {
                format!("Failed to canonicalize settings path {:?}", initial_path)
            })?;

            fs.atomic_write(resolved_path.clone(), new_text)
                .await
                .with_context(|| format!("Failed to write settings to file {:?}", resolved_path))?;
        } else {
            fs.atomic_write(initial_path.to_path_buf(), new_text)
                .await
                .with_context(|| format!("Failed to write settings to file {:?}", initial_path))?;
        }

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
