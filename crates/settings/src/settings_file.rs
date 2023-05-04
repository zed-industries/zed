use crate::{update_settings_file, watched_json::WatchedJsonFile, Settings, SettingsFileContent};
use anyhow::Result;
use assets::Assets;
use fs::Fs;
use gpui::AppContext;
use std::{io::ErrorKind, ops::Range, path::Path, sync::Arc};

// TODO: Switch SettingsFile to open a worktree and buffer for synchronization
//       And instant updates in the Zed editor
#[derive(Clone)]
pub struct SettingsFile {
    path: &'static Path,
    settings_file_content: WatchedJsonFile<SettingsFileContent>,
    fs: Arc<dyn Fs>,
}

impl SettingsFile {
    pub fn new(
        path: &'static Path,
        settings_file_content: WatchedJsonFile<SettingsFileContent>,
        fs: Arc<dyn Fs>,
    ) -> Self {
        SettingsFile {
            path,
            settings_file_content,
            fs,
        }
    }

    async fn load_settings(path: &Path, fs: &Arc<dyn Fs>) -> Result<String> {
        match fs.load(path).await {
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

    pub fn update_unsaved(
        text: &str,
        cx: &AppContext,
        update: impl FnOnce(&mut SettingsFileContent),
    ) -> Vec<(Range<usize>, String)> {
        let this = cx.global::<SettingsFile>();
        let tab_size = cx.global::<Settings>().tab_size(Some("JSON"));
        let current_file_content = this.settings_file_content.current();
        update_settings_file(&text, current_file_content, tab_size, update)
    }

    pub fn update(
        cx: &mut AppContext,
        update: impl 'static + Send + FnOnce(&mut SettingsFileContent),
    ) {
        let this = cx.global::<SettingsFile>();
        let tab_size = cx.global::<Settings>().tab_size(Some("JSON"));
        let current_file_content = this.settings_file_content.current();
        let fs = this.fs.clone();
        let path = this.path.clone();

        cx.background()
            .spawn(async move {
                let old_text = SettingsFile::load_settings(path, &fs).await?;
                let edits = update_settings_file(&old_text, current_file_content, tab_size, update);
                let mut new_text = old_text;
                for (range, replacement) in edits.into_iter().rev() {
                    new_text.replace_range(range, &replacement);
                }
                fs.atomic_write(path.to_path_buf(), new_text).await?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        watch_files, watched_json::watch_settings_file, EditorSettings, Settings, SoftWrap,
    };
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
        let font_cache = cx.font_cache();

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

        let settings_file =
            WatchedJsonFile::new(fs.clone(), &executor, "/settings.json".as_ref()).await;
        let keymaps_file =
            WatchedJsonFile::new(fs.clone(), &executor, "/keymap.json".as_ref()).await;

        let default_settings = cx.read(Settings::test);

        cx.update(|cx| {
            cx.add_global_action(|_: &A, _cx| {});
            cx.add_global_action(|_: &B, _cx| {});
            cx.add_global_action(|_: &ActivatePreviousPane, _cx| {});
            cx.add_global_action(|_: &ActivatePrevItem, _cx| {});
            watch_files(
                default_settings,
                settings_file,
                ThemeRegistry::new((), font_cache),
                keymaps_file,
                cx,
            )
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

    #[gpui::test]
    async fn test_watch_settings_files(cx: &mut gpui::TestAppContext) {
        let executor = cx.background();
        let fs = FakeFs::new(executor.clone());
        let font_cache = cx.font_cache();

        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "buffer_font_size": 24,
                "soft_wrap": "editor_width",
                "tab_size": 8,
                "language_overrides": {
                    "Markdown": {
                        "tab_size": 2,
                        "preferred_line_length": 100,
                        "soft_wrap": "preferred_line_length"
                    }
                }
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        let source = WatchedJsonFile::new(fs.clone(), &executor, "/settings.json".as_ref()).await;

        let default_settings = cx.read(Settings::test).with_language_defaults(
            "JavaScript",
            EditorSettings {
                tab_size: Some(2.try_into().unwrap()),
                ..Default::default()
            },
        );
        cx.update(|cx| {
            watch_settings_file(
                default_settings.clone(),
                source,
                ThemeRegistry::new((), font_cache),
                cx,
            )
        });

        cx.foreground().run_until_parked();
        let settings = cx.read(|cx| cx.global::<Settings>().clone());
        assert_eq!(settings.buffer_font_size, 24.0);

        assert_eq!(settings.soft_wrap(None), SoftWrap::EditorWidth);
        assert_eq!(
            settings.soft_wrap(Some("Markdown")),
            SoftWrap::PreferredLineLength
        );
        assert_eq!(
            settings.soft_wrap(Some("JavaScript")),
            SoftWrap::EditorWidth
        );

        assert_eq!(settings.preferred_line_length(None), 80);
        assert_eq!(settings.preferred_line_length(Some("Markdown")), 100);
        assert_eq!(settings.preferred_line_length(Some("JavaScript")), 80);

        assert_eq!(settings.tab_size(None).get(), 8);
        assert_eq!(settings.tab_size(Some("Markdown")).get(), 2);
        assert_eq!(settings.tab_size(Some("JavaScript")).get(), 8);

        fs.save(
            "/settings.json".as_ref(),
            &"(garbage)".into(),
            Default::default(),
        )
        .await
        .unwrap();
        // fs.remove_file("/settings.json".as_ref(), Default::default())
        //     .await
        //     .unwrap();

        cx.foreground().run_until_parked();
        let settings = cx.read(|cx| cx.global::<Settings>().clone());
        assert_eq!(settings.buffer_font_size, 24.0);

        assert_eq!(settings.soft_wrap(None), SoftWrap::EditorWidth);
        assert_eq!(
            settings.soft_wrap(Some("Markdown")),
            SoftWrap::PreferredLineLength
        );
        assert_eq!(
            settings.soft_wrap(Some("JavaScript")),
            SoftWrap::EditorWidth
        );

        assert_eq!(settings.preferred_line_length(None), 80);
        assert_eq!(settings.preferred_line_length(Some("Markdown")), 100);
        assert_eq!(settings.preferred_line_length(Some("JavaScript")), 80);

        assert_eq!(settings.tab_size(None).get(), 8);
        assert_eq!(settings.tab_size(Some("Markdown")).get(), 2);
        assert_eq!(settings.tab_size(Some("JavaScript")).get(), 8);

        fs.remove_file("/settings.json".as_ref(), Default::default())
            .await
            .unwrap();
        cx.foreground().run_until_parked();
        let settings = cx.read(|cx| cx.global::<Settings>().clone());
        assert_eq!(settings.buffer_font_size, default_settings.buffer_font_size);
    }
}
