use crate::{settings_store::SettingsStore, Settings};
use collections::HashMap;
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui::{App, BackgroundExecutor, ReadGlobal, UpdateGlobal};
use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
    time::Duration,
};

pub const EMPTY_THEME_NAME: &str = "empty-theme";

#[cfg(any(test, feature = "test-support"))]
pub fn test_settings() -> String {
    let mut value = crate::utils::parse_json_with_comments::<serde_json::Value>(
        crate::default_settings().as_ref(),
    )
    .unwrap();
    #[cfg(not(target_os = "windows"))]
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
    #[cfg(target_os = "windows")]
    util::merge_non_null_json_value_into(
        serde_json::json!({
            "ui_font_family": "Courier New",
            "ui_font_features": {},
            "ui_font_size": 14,
            "ui_font_fallback": [],
            "buffer_font_family": "Courier New",
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
    cx: &mut App,
    settings_changed: impl Fn(Option<anyhow::Error>, &mut App) + 'static,
) {
    let user_settings_content = cx
        .background_executor()
        .block(user_settings_file_rx.next())
        .unwrap();
    SettingsStore::update_global(cx, |store, cx| {
        let result = store.set_user_settings(&user_settings_content, cx);
        if let Err(err) = &result {
            log::error!("Failed to load user settings: {err}");
        }
        settings_changed(result.err(), cx);
    });
    cx.spawn(move |cx| async move {
        while let Some(user_settings_content) = user_settings_file_rx.next().await {
            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                let result = store.set_user_settings(&user_settings_content, cx);
                if let Err(err) = &result {
                    log::error!("Failed to load user settings: {err}");
                }
                settings_changed(result.err(), cx);
                cx.refresh_windows();
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
    cx: &App,
    update: impl 'static + Send + FnOnce(&mut T::FileContent, &App),
) {
    SettingsStore::global(cx).update_settings_file::<T>(fs, update);
}

fn migrate_settings_for_type<T: Settings>(fs: Arc<dyn Fs>, cx: &mut App) {
    let type_id = std::any::type_name::<T>();
    update_settings_file::<T>(fs, cx, |settings, _| {
        let user_settings = match settings.raw_user_settings.as_object_mut() {
            Some(settings) => settings,
            None => return,
        };
        if let Some(replacements) = SETTINGS_STRING_REPLACE.get(type_id) {
            for (old_key, new_key) in replacements.iter() {
                if let Some(value) = user_settings.remove(*old_key) {
                    user_settings.insert(new_key.to_string(), value);
                }
            }
        }
        if let Some(replacements) = SETTINGS_NESTED_STRING_REPLACE.get(type_id) {
            for (parent_key, (old_key, new_key)) in replacements.iter() {
                if let Some(parent_value) = user_settings.get_mut(*parent_key) {
                    if let Some(child_value) = parent_value.as_object_mut() {
                        if let Some(value) = child_value.remove(*old_key) {
                            child_value.insert(new_key.to_string(), value);
                        }
                    }
                }
            }
        }
    });
}

#[rustfmt::skip]
static SETTINGS_STRING_REPLACE: LazyLock<HashMap<&'static str, Vec<(&'static str, &'static str)>>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("EditorSettings", vec![
            ("show_inline_completions_in_menu", "show_edit_predictions_in_menu")
        ]),
        ("LanguageSettings", vec![
            ("show_inline_completions", "show_edit_predictions"),
            ("inline_completions_disabled_in", "edit_predictions_disabled_in")
        ]),
        ("AllLanguageSettings", vec![
            ("inline_completions", "edit_predictions")
        ])
    ])
});

#[rustfmt::skip]
static SETTINGS_NESTED_STRING_REPLACE: LazyLock<HashMap<&'static str, Vec<(&'static str, (&'static str, &'static str))>>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("AllLanguageSettings", vec![
            ("features", ("inline_completion_provider", "edit_prediction_provider"))
        ])
    ])
});
