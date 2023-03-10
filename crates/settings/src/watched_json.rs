use fs::Fs;
use futures::StreamExt;
use gpui::{executor, MutableAppContext};
use postage::sink::Sink as _;
use postage::{prelude::Stream, watch};
use serde::Deserialize;

use std::{path::Path, sync::Arc, time::Duration};
use theme::ThemeRegistry;
use util::ResultExt;

use crate::{parse_json_with_comments, KeymapFileContent, Settings, SettingsFileContent};

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

    pub fn current(&self) -> T {
        self.0.borrow().clone()
    }
}

pub fn watch_files(
    defaults: Settings,
    settings_file: WatchedJsonFile<SettingsFileContent>,
    theme_registry: Arc<ThemeRegistry>,
    keymap_file: WatchedJsonFile<KeymapFileContent>,
    cx: &mut MutableAppContext,
) {
    watch_settings_file(defaults, settings_file, theme_registry, cx);
    watch_keymap_file(keymap_file, cx);
}

pub(crate) fn watch_settings_file(
    defaults: Settings,
    mut file: WatchedJsonFile<SettingsFileContent>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut MutableAppContext,
) {
    settings_updated(&defaults, file.0.borrow().clone(), &theme_registry, cx);
    cx.spawn(|mut cx| async move {
        while let Some(content) = file.0.recv().await {
            cx.update(|cx| settings_updated(&defaults, content, &theme_registry, cx));
        }
    })
    .detach();
}

fn keymap_updated(content: KeymapFileContent, cx: &mut MutableAppContext) {
    cx.clear_bindings();
    KeymapFileContent::load_defaults(cx);
    content.add_to_cx(cx).log_err();
}

fn settings_updated(
    defaults: &Settings,
    content: SettingsFileContent,
    theme_registry: &Arc<ThemeRegistry>,
    cx: &mut MutableAppContext,
) {
    let mut settings = defaults.clone();
    settings.set_user_settings(content, theme_registry, cx.font_cache());
    cx.set_global(settings);
    cx.refresh_windows();
}

fn watch_keymap_file(mut file: WatchedJsonFile<KeymapFileContent>, cx: &mut MutableAppContext) {
    cx.spawn(|mut cx| async move {
        let mut settings_subscription = None;
        while let Some(content) = file.0.recv().await {
            cx.update(|cx| {
                let old_base_keymap = cx.global::<Settings>().base_keymap;
                keymap_updated(content.clone(), cx);
                settings_subscription = Some(cx.observe_global::<Settings, _>(move |cx| {
                    let settings = cx.global::<Settings>();
                    if settings.base_keymap != old_base_keymap {
                        keymap_updated(content.clone(), cx);
                    }
                }));
            });
        }
    })
    .detach();
}
