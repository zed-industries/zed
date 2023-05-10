use crate::{
    settings_store::parse_json_with_comments, settings_store::SettingsStore, KeymapFileContent,
    Setting, Settings, DEFAULT_SETTINGS_ASSET_PATH,
};
use anyhow::Result;
use assets::Assets;
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui::{executor::Background, AppContext, AssetSource};
use std::{
    borrow::Cow,
    io::ErrorKind,
    path::{Path, PathBuf},
    str,
    sync::Arc,
    time::Duration,
};
use util::{paths, ResultExt};

pub fn register_setting<T: Setting>(cx: &mut AppContext) {
    cx.update_global::<SettingsStore, _, _>(|store, cx| {
        store.register_setting::<T>(cx);
    });
}

pub fn get_setting<'a, T: Setting>(path: Option<&Path>, cx: &'a AppContext) -> &'a T {
    cx.global::<SettingsStore>().get(path)
}

pub fn default_settings() -> Cow<'static, str> {
    match Assets.load(DEFAULT_SETTINGS_ASSET_PATH).unwrap() {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_settings() -> String {
    let mut value =
        parse_json_with_comments::<serde_json::Value>(default_settings().as_ref()).unwrap();
    util::merge_non_null_json_value_into(
        serde_json::json!({
            "buffer_font_family": "Courier",
            "buffer_font_features": {},
            "default_buffer_font_size": 14,
            "preferred_line_length": 80,
            "theme": theme::EMPTY_THEME_NAME,
        }),
        &mut value,
    );
    serde_json::to_string(&value).unwrap()
}

pub fn watch_config_file(
    executor: Arc<Background>,
    fs: Arc<dyn Fs>,
    path: PathBuf,
) -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded();
    executor
        .spawn(async move {
            let events = fs.watch(&path, Duration::from_millis(100)).await;
            futures::pin_mut!(events);
            loop {
                if let Ok(contents) = fs.load(&path).await {
                    if !tx.unbounded_send(contents).is_ok() {
                        break;
                    }
                }
                if events.next().await.is_none() {
                    break;
                }
            }
        })
        .detach();
    rx
}

pub fn handle_keymap_file_changes(
    mut user_keymap_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    cx.spawn(move |mut cx| async move {
        let mut settings_subscription = None;
        while let Some(user_keymap_content) = user_keymap_file_rx.next().await {
            if let Ok(keymap_content) =
                parse_json_with_comments::<KeymapFileContent>(&user_keymap_content)
            {
                cx.update(|cx| {
                    cx.clear_bindings();
                    KeymapFileContent::load_defaults(cx);
                    keymap_content.clone().add_to_cx(cx).log_err();
                });

                let mut old_base_keymap = cx.read(|cx| cx.global::<Settings>().base_keymap.clone());
                drop(settings_subscription);
                settings_subscription = Some(cx.update(|cx| {
                    cx.observe_global::<Settings, _>(move |cx| {
                        let settings = cx.global::<Settings>();
                        if settings.base_keymap != old_base_keymap {
                            old_base_keymap = settings.base_keymap.clone();

                            cx.clear_bindings();
                            KeymapFileContent::load_defaults(cx);
                            keymap_content.clone().add_to_cx(cx).log_err();
                        }
                    })
                    .detach();
                }));
            }
        }
    })
    .detach();
}

pub fn handle_settings_file_changes(
    mut user_settings_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    let user_settings_content = cx.background().block(user_settings_file_rx.next()).unwrap();
    cx.update_global::<SettingsStore, _, _>(|store, cx| {
        store
            .set_user_settings(&user_settings_content, cx)
            .log_err();

        // TODO - remove the Settings global, use the SettingsStore instead.
        store.register_setting::<Settings>(cx);
        cx.set_global(store.get::<Settings>(None).clone());
    });
    cx.spawn(move |mut cx| async move {
        while let Some(user_settings_content) = user_settings_file_rx.next().await {
            cx.update(|cx| {
                cx.update_global::<SettingsStore, _, _>(|store, cx| {
                    store
                        .set_user_settings(&user_settings_content, cx)
                        .log_err();

                    // TODO - remove the Settings global, use the SettingsStore instead.
                    cx.set_global(store.get::<Settings>(None).clone());
                });
            });
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
                    return Ok(Settings::initial_user_settings_content(&Assets).to_string());
                }
            }
            return Err(err);
        }
    }
}

pub fn update_settings_file<T: Setting>(
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
    update: impl 'static + Send + FnOnce(&mut T::FileContent),
) {
    cx.spawn(|cx| async move {
        let old_text = cx
            .background()
            .spawn({
                let fs = fs.clone();
                async move { load_settings(&fs).await }
            })
            .await?;

        let edits = cx.read(|cx| cx.global::<SettingsStore>().update::<T>(&old_text, update));

        let mut new_text = old_text;
        for (range, replacement) in edits.into_iter().rev() {
            new_text.replace_range(range, &replacement);
        }

        cx.background()
            .spawn(async move { fs.atomic_write(paths::SETTINGS.clone(), new_text).await })
            .await?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{actions, elements::*, Action, Entity, TestAppContext, View, ViewContext};
    use theme::ThemeRegistry;

    struct TestView;

    impl Entity for TestView {
        type Event = ();
    }

    impl View for TestView {
        fn ui_name() -> &'static str {
            "TestView"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }

    #[gpui::test]
    async fn test_base_keymap(cx: &mut gpui::TestAppContext) {
        let executor = cx.background();
        let fs = FakeFs::new(executor.clone());

        actions!(test, [A, B]);
        // From the Atom keymap
        actions!(workspace, [ActivatePreviousPane]);
        // From the JetBrains keymap
        actions!(pane, [ActivatePrevItem]);

        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "Atom"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": "test::A"
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.update(|cx| {
            let mut store = SettingsStore::default();
            store.set_default_settings(&test_settings(), cx).unwrap();
            cx.set_global(store);
            cx.set_global(ThemeRegistry::new(Assets, cx.font_cache().clone()));
            cx.add_global_action(|_: &A, _cx| {});
            cx.add_global_action(|_: &B, _cx| {});
            cx.add_global_action(|_: &ActivatePreviousPane, _cx| {});
            cx.add_global_action(|_: &ActivatePrevItem, _cx| {});

            let settings_rx = watch_config_file(
                executor.clone(),
                fs.clone(),
                PathBuf::from("/settings.json"),
            );
            let keymap_rx =
                watch_config_file(executor.clone(), fs.clone(), PathBuf::from("/keymap.json"));

            handle_keymap_file_changes(keymap_rx, cx);
            handle_settings_file_changes(settings_rx, cx);
        });

        cx.foreground().run_until_parked();

        let (window_id, _view) = cx.add_window(|_| TestView);

        // Test loading the keymap base at all
        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &A), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the users keymap, while retaining the base keymap
        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": "test::B"
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &B), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the base, while retaining the users keymap
        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "JetBrains"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &B), ("[", &ActivatePrevItem)],
            line!(),
        );
    }

    fn assert_key_bindings_for<'a>(
        window_id: usize,
        cx: &TestAppContext,
        actions: Vec<(&'static str, &'a dyn Action)>,
        line: u32,
    ) {
        for (key, action) in actions {
            // assert that...
            assert!(
                cx.available_actions(window_id, 0)
                    .into_iter()
                    .any(|(_, bound_action, b)| {
                        // action names match...
                        bound_action.name() == action.name()
                    && bound_action.namespace() == action.namespace()
                    // and key strokes contain the given key
                    && b.iter()
                        .any(|binding| binding.keystrokes().iter().any(|k| k.key == key))
                    }),
                "On {} Failed to find {} with key binding {}",
                line,
                action.name(),
                key
            );
        }
    }
}
