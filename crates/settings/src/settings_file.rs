use crate::{watched_json::WatchedJsonFile, write_top_level_setting, SettingsFileContent};
use anyhow::Result;
use fs::Fs;
use gpui::MutableAppContext;
use serde_json::Value;
use std::{path::Path, sync::Arc};

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

    pub fn update(cx: &mut MutableAppContext, update: impl FnOnce(&mut SettingsFileContent)) {
        let this = cx.global::<SettingsFile>();

        let current_file_content = this.settings_file_content.current();
        let mut new_file_content = current_file_content.clone();

        update(&mut new_file_content);

        let fs = this.fs.clone();
        let path = this.path.clone();

        cx.background()
            .spawn(async move {
                // Unwrap safety: These values are all guarnteed to be well formed, and we know
                // that they will deserialize to our settings object. All of the following unwraps
                // are therefore safe.
                let tmp = serde_json::to_value(current_file_content).unwrap();
                let old_json = tmp.as_object().unwrap();

                let new_tmp = serde_json::to_value(new_file_content).unwrap();
                let new_json = new_tmp.as_object().unwrap();

                // Find changed fields
                let mut diffs = vec![];
                for (key, old_value) in old_json.iter() {
                    let new_value = new_json.get(key).unwrap();
                    if old_value != new_value {
                        if matches!(
                            new_value,
                            &Value::Null | &Value::Object(_) | &Value::Array(_)
                        ) {
                            unimplemented!(
                                "We only support updating basic values at the top level"
                            );
                        }

                        let new_json = serde_json::to_string_pretty(new_value)
                            .expect("Could not serialize new json field to string");

                        diffs.push((key, new_json));
                    }
                }

                // Have diffs, rewrite the settings file now.
                let mut content = fs.load(path).await?;

                for (key, new_value) in diffs {
                    content = write_top_level_setting(content, key, &new_value)
                }

                fs.atomic_write(path.to_path_buf(), content).await?;

                Ok(()) as Result<()>
            })
            .detach_and_log_err(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{watched_json::watch_settings_file, EditorSettings, Settings, SoftWrap};
    use fs::FakeFs;
    use theme::ThemeRegistry;

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
