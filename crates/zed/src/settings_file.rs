use futures::{stream, StreamExt};
use gpui::{executor, FontCache};
use postage::sink::Sink as _;
use postage::{prelude::Stream, watch};
use project::Fs;
use settings::{Settings, SettingsFileContent};
use std::{path::Path, sync::Arc, time::Duration};
use theme::ThemeRegistry;
use util::ResultExt;

#[derive(Clone)]
pub struct SettingsFile(watch::Receiver<SettingsFileContent>);

impl SettingsFile {
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

    async fn load(fs: Arc<dyn Fs>, path: &Path) -> Option<SettingsFileContent> {
        if fs.is_file(&path).await {
            fs.load(&path)
                .await
                .log_err()
                .and_then(|data| serde_json::from_str(&data).log_err())
        } else {
            Some(SettingsFileContent::default())
        }
    }
}

pub fn settings_from_files(
    defaults: Settings,
    sources: Vec<SettingsFile>,
    theme_registry: Arc<ThemeRegistry>,
    font_cache: Arc<FontCache>,
) -> impl futures::stream::Stream<Item = Settings> {
    stream::select_all(sources.iter().enumerate().map(|(i, source)| {
        let mut rx = source.0.clone();
        // Consume the initial item from all of the constituent file watches but one.
        // This way, the stream will yield exactly one item for the files' initial
        // state, and won't return any more items until the files change.
        if i > 0 {
            rx.try_recv().ok();
        }
        rx
    }))
    .map(move |_| {
        let mut settings = defaults.clone();
        for source in &sources {
            settings.merge(&*source.0.borrow(), &theme_registry, &font_cache);
        }
        settings
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use project::FakeFs;
    use settings::SoftWrap;

    #[gpui::test]
    async fn test_settings_from_files(cx: &mut gpui::TestAppContext) {
        let executor = cx.background();
        let fs = FakeFs::new(executor.clone());

        fs.save(
            "/settings1.json".as_ref(),
            &r#"
            {
                "buffer_font_size": 24,
                "soft_wrap": "editor_width",
                "language_overrides": {
                    "Markdown": {
                        "preferred_line_length": 100,
                        "soft_wrap": "preferred_line_length"
                    }
                }
            }
            "#
            .into(),
        )
        .await
        .unwrap();

        let source1 = SettingsFile::new(fs.clone(), &executor, "/settings1.json".as_ref()).await;
        let source2 = SettingsFile::new(fs.clone(), &executor, "/settings2.json".as_ref()).await;
        let source3 = SettingsFile::new(fs.clone(), &executor, "/settings3.json".as_ref()).await;

        let mut settings_rx = settings_from_files(
            cx.read(Settings::test),
            vec![source1, source2, source3],
            ThemeRegistry::new((), cx.font_cache()),
            cx.font_cache(),
        );

        let settings = settings_rx.next().await.unwrap();
        let md_settings = settings.language_overrides.get("Markdown").unwrap();
        assert_eq!(settings.soft_wrap, SoftWrap::EditorWidth);
        assert_eq!(settings.buffer_font_size, 24.0);
        assert_eq!(settings.tab_size, 4);
        assert_eq!(md_settings.soft_wrap, Some(SoftWrap::PreferredLineLength));
        assert_eq!(md_settings.preferred_line_length, Some(100));

        fs.save(
            "/settings2.json".as_ref(),
            &r#"
            {
                "tab_size": 2,
                "soft_wrap": "none",
                "language_overrides": {
                    "Markdown": {
                        "preferred_line_length": 120
                    }
                }
            }
            "#
            .into(),
        )
        .await
        .unwrap();

        let settings = settings_rx.next().await.unwrap();
        let md_settings = settings.language_overrides.get("Markdown").unwrap();
        assert_eq!(settings.soft_wrap, SoftWrap::None);
        assert_eq!(settings.buffer_font_size, 24.0);
        assert_eq!(settings.tab_size, 2);
        assert_eq!(md_settings.soft_wrap, Some(SoftWrap::PreferredLineLength));
        assert_eq!(md_settings.preferred_line_length, Some(120));

        fs.remove_file("/settings2.json".as_ref(), Default::default())
            .await
            .unwrap();

        let settings = settings_rx.next().await.unwrap();
        assert_eq!(settings.tab_size, 4);
    }
}
