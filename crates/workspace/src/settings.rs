use anyhow::Result;
use futures::{stream, SinkExt, StreamExt as _};
use gpui::{
    executor,
    font_cache::{FamilyId, FontCache},
};
use language::Language;
use parking_lot::Mutex;
use postage::{prelude::Stream, watch};
use project::Fs;
use serde::Deserialize;
use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};
use theme::{Theme, ThemeRegistry};
use util::ResultExt;

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    pub soft_wrap: SoftWrap,
    pub preferred_line_length: u32,
    pub language_overrides: HashMap<Arc<str>, LanguageOverride>,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LanguageOverride {
    pub tab_size: Option<usize>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

#[derive(Clone)]
pub struct SettingsFile(watch::Receiver<SettingsFileContent>);

#[derive(Clone, Debug, Default, Deserialize)]
struct SettingsFileContent {
    #[serde(default)]
    buffer_font_family: Option<String>,
    #[serde(default)]
    buffer_font_size: Option<f32>,
    #[serde(flatten)]
    editor: LanguageOverride,
    #[serde(default)]
    language_overrides: HashMap<Arc<str>, LanguageOverride>,
    #[serde(default)]
    theme: Option<String>,
}

impl SettingsFile {
    pub async fn new(
        fs: Arc<dyn Fs>,
        executor: &executor::Background,
        path: impl Into<Arc<Path>>,
    ) -> Self {
        let path = path.into();
        let settings = Self::load(fs.clone(), &path).await.unwrap_or_default();
        let mut events = fs.watch(&path, Duration::from_millis(500)).await;
        let (mut tx, mut rx) = watch::channel_with(settings);
        rx.recv().await;
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

impl Settings {
    pub fn from_files(
        defaults: Self,
        sources: Vec<SettingsFile>,
        executor: Arc<executor::Background>,
        theme_registry: Arc<ThemeRegistry>,
        font_cache: Arc<FontCache>,
    ) -> (Arc<Mutex<watch::Sender<Self>>>, watch::Receiver<Self>) {
        let (tx, mut rx) = watch::channel_with(defaults.clone());
        let tx = Arc::new(Mutex::new(tx));
        executor
            .spawn({
                let tx = tx.clone();
                async move {
                    let mut stream =
                        stream::select_all(sources.iter().map(|source| source.0.clone()));
                    while stream.next().await.is_some() {
                        let mut settings = defaults.clone();
                        for source in &sources {
                            settings.merge(&*source.0.borrow(), &theme_registry, &font_cache);
                        }
                        *tx.lock().borrow_mut() = settings;
                    }
                }
            })
            .detach();
        rx.try_recv().ok();
        (tx, rx)
    }

    pub fn new(
        buffer_font_family: &str,
        font_cache: &FontCache,
        theme: Arc<Theme>,
    ) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&[buffer_font_family])?,
            buffer_font_size: 15.,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
            theme,
        })
    }

    pub fn with_overrides(
        mut self,
        language_name: impl Into<Arc<str>>,
        overrides: LanguageOverride,
    ) -> Self {
        self.language_overrides
            .insert(language_name.into(), overrides);
        self
    }

    pub fn tab_size(&self, language: Option<&Arc<Language>>) -> usize {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.tab_size)
            .unwrap_or(self.tab_size)
    }

    pub fn soft_wrap(&self, language: Option<&Arc<Language>>) -> SoftWrap {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.soft_wrap)
            .unwrap_or(self.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&Arc<Language>>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.preferred_line_length)
            .unwrap_or(self.preferred_line_length)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
            theme: gpui::fonts::with_font_cache(cx.font_cache().clone(), || Default::default()),
        }
    }

    fn merge(
        &mut self,
        data: &SettingsFileContent,
        theme_registry: &ThemeRegistry,
        font_cache: &FontCache,
    ) {
        if let Some(value) = &data.buffer_font_family {
            if let Some(id) = font_cache.load_family(&[value]).log_err() {
                self.buffer_font_family = id;
            }
        }
        if let Some(value) = &data.theme {
            if let Some(theme) = theme_registry.get(value).log_err() {
                self.theme = theme;
            }
        }

        merge(&mut self.buffer_font_size, data.buffer_font_size);
        merge(&mut self.soft_wrap, data.editor.soft_wrap);
        merge(&mut self.tab_size, data.editor.tab_size);
        merge(
            &mut self.preferred_line_length,
            data.editor.preferred_line_length,
        );

        for (language_name, settings) in &data.language_overrides {
            let target = self
                .language_overrides
                .entry(language_name.clone())
                .or_default();

            merge_option(&mut target.tab_size, settings.tab_size);
            merge_option(&mut target.soft_wrap, settings.soft_wrap);
            merge_option(
                &mut target.preferred_line_length,
                settings.preferred_line_length,
            );
        }
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

fn merge_option<T: Copy>(target: &mut Option<T>, value: Option<T>) {
    if value.is_some() {
        *target = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postage::prelude::Stream;
    use project::FakeFs;

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

        let (_, mut settings_rx) = Settings::from_files(
            cx.read(Settings::test),
            vec![source1, source2, source3],
            cx.background(),
            ThemeRegistry::new((), cx.font_cache()),
            cx.font_cache(),
        );

        let settings = settings_rx.recv().await.unwrap();
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

        let settings = settings_rx.recv().await.unwrap();
        let md_settings = settings.language_overrides.get("Markdown").unwrap();
        assert_eq!(settings.soft_wrap, SoftWrap::None);
        assert_eq!(settings.buffer_font_size, 24.0);
        assert_eq!(settings.tab_size, 2);
        assert_eq!(md_settings.soft_wrap, Some(SoftWrap::PreferredLineLength));
        assert_eq!(md_settings.preferred_line_length, Some(120));

        fs.remove_file("/settings2.json".as_ref(), Default::default())
            .await
            .unwrap();

        let settings = settings_rx.recv().await.unwrap();
        assert_eq!(settings.tab_size, 4);
    }
}
