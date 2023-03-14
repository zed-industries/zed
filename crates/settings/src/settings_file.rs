use crate::{update_settings_file, watched_json::WatchedJsonFile, SettingsFileContent};
use anyhow::Result;
use assets::Assets;
use fs::Fs;
use gpui::{AssetSource, MutableAppContext};
use std::{io::ErrorKind, path::Path, sync::Arc};

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
                        return Ok(std::str::from_utf8(
                            Assets
                                .load("settings/initial_user_settings.json")
                                .unwrap()
                                .as_ref(),
                        )
                        .unwrap()
                        .to_string());
                    }
                }
                return Err(err);
            }
        }
    }

    pub fn update(
        cx: &mut MutableAppContext,
        update: impl 'static + Send + FnOnce(&mut SettingsFileContent),
    ) {
        let this = cx.global::<SettingsFile>();

        let current_file_content = this.settings_file_content.current();

        let fs = this.fs.clone();
        let path = this.path.clone();

        cx.background()
            .spawn(async move {
                let old_text = SettingsFile::load_settings(path, &fs).await?;

                let new_text = update_settings_file(old_text, current_file_content, update);

                fs.atomic_write(path.to_path_buf(), new_text).await?;

                Ok(()) as Result<()>
            })
            .detach_and_log_err(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        watch_files, watched_json::watch_settings_file, EditorSettings, Settings, SoftWrap,
    };
    use fs::FakeFs;
    use gpui::{actions, Action};
    use theme::ThemeRegistry;

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

        // Test loading the keymap base at all
        cx.update(|cx| {
            assert_keybindings_for(
                cx,
                vec![("backspace", &A), ("k", &ActivatePreviousPane)],
                line!(),
            );
        });

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

        cx.update(|cx| {
            assert_keybindings_for(
                cx,
                vec![("backspace", &B), ("k", &ActivatePreviousPane)],
                line!(),
            );
        });

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

        cx.update(|cx| {
            assert_keybindings_for(
                cx,
                vec![("backspace", &B), ("[", &ActivatePrevItem)],
                line!(),
            );
        });
    }

    fn assert_keybindings_for<'a>(
        cx: &mut MutableAppContext,
        actions: Vec<(&'static str, &'a dyn Action)>,
        line: u32,
    ) {
        for (key, action) in actions {
            // assert that...
            assert!(
                cx.available_actions(0, 0).any(|(_, bound_action, b)| {
                    // action names match...
                    bound_action.name() == action.name()
                    && bound_action.namespace() == action.namespace()
                    // and key strokes contain the given key
                    && b.iter()
                        .any(|binding| binding.keystrokes().iter().any(|k| k.key == key))
                }),
                "On {} Failed to find {} with keybinding {}",
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
