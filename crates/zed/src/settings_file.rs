use futures::StreamExt;
use gpui::{executor, MutableAppContext};
use postage::sink::Sink as _;
use postage::{prelude::Stream, watch};
use project::Fs;
use serde::Deserialize;
use settings::{parse_json_with_comments, KeymapFileContent, Settings, SettingsFileContent};
use std::{path::Path, sync::Arc, time::Duration};
use theme::ThemeRegistry;
use util::ResultExt;

#[derive(Clone)]
pub struct WatchedJsonFile<T>(pub watch::Receiver<T>);

impl<T> WatchedJsonFile<T>
where
    T: 'static + for<'de> Deserialize<'de> + Clone + Default + Send + Sync,
{
    pub async fn new(
        fs: Arc<dyn Fs>,
        executor: &executor::Background,
        path: impl Into<Arc<Path>>,
    ) -> Self {
        let path = path.into();
        let settings = Self::load(fs.clone(), &path).await.unwrap_or_default();
        let mut events = fs.watch(&path, Duration::from_millis(500)).await;
        let (mut tx, rx) = watch::channel_with(settings);
        executor
            .spawn(async move {
                while events.next().await.is_some() {
                    if let Some(settings) = Self::load(fs.clone(), &path).await {
                        if tx.send(settings).await.is_err() {
                            break;
                        }
                    }
                }
            })
            .detach();
        Self(rx)
    }

    ///Loads the given watched JSON file. In the special case that the file is
    ///empty (ignoring whitespace) or is not a file, this will return T::default()
    async fn load(fs: Arc<dyn Fs>, path: &Path) -> Option<T> {
        if !fs.is_file(path).await {
            return Some(T::default());
        }

        fs.load(path).await.log_err().and_then(|data| {
            if data.trim().is_empty() {
                Some(T::default())
            } else {
                parse_json_with_comments(&data).log_err()
            }
        })
    }
}

pub fn watch_settings_file(
    defaults: Settings,
    mut file: WatchedJsonFile<SettingsFileContent>,
    theme_registry: Arc<ThemeRegistry>,
    internal: bool,
    cx: &mut MutableAppContext,
) {
    settings_updated(
        &defaults,
        file.0.borrow().clone(),
        &theme_registry,
        internal,
        cx,
    );
    cx.spawn(|mut cx| async move {
        while let Some(content) = file.0.recv().await {
            cx.update(|cx| settings_updated(&defaults, content, &theme_registry, internal, cx));
        }
    })
    .detach();
}

pub fn keymap_updated(content: KeymapFileContent, cx: &mut MutableAppContext) {
    cx.clear_bindings();
    settings::KeymapFileContent::load_defaults(cx);
    content.add_to_cx(cx).log_err();
}

pub fn settings_updated(
    defaults: &Settings,
    content: SettingsFileContent,
    theme_registry: &Arc<ThemeRegistry>,
    internal: bool,
    cx: &mut MutableAppContext,
) {
    let mut settings = defaults.clone();
    settings.set_user_settings(content, theme_registry, cx.font_cache(), internal);
    cx.set_global(settings);
    cx.refresh_windows();
}

pub fn watch_keymap_file(mut file: WatchedJsonFile<KeymapFileContent>, cx: &mut MutableAppContext) {
    cx.spawn(|mut cx| async move {
        while let Some(content) = file.0.recv().await {
            cx.update(|cx| keymap_updated(content, cx));
        }
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use super::*;
    use project::FakeFs;
    use settings::{EditorSettings, SoftWrap};

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
                ThemeRegistry::new((), font_cache, false),
                false,
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
