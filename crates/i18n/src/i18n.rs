use collections::HashMap;
use gpui::{App, Global, SharedString};
use rust_embed::RustEmbed;
use serde::Deserialize;
use settings::{RegisterSetting, Settings, SettingsContent};

pub use settings::LocaleContent;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "i18n/*"]
#[exclude = "*.DS_Store"]
struct I18nAssets;

pub struct I18n {
    locale: LocaleContent,
    translations: HashMap<String, String>,
}

impl Global for I18n {}

impl I18n {
    fn new(locale: LocaleContent) -> Self {
        let translations = load_translations(locale);
        Self {
            locale,
            translations,
        }
    }

    pub fn locale(&self) -> LocaleContent {
        self.locale
    }
}

fn locale_file_name(locale: LocaleContent) -> &'static str {
    match locale {
        LocaleContent::En => "i18n/en.json",
        LocaleContent::ZhCn => "i18n/zh-CN.json",
    }
}

fn load_translations(locale: LocaleContent) -> HashMap<String, String> {
    let file_name = locale_file_name(locale);
    let content = match I18nAssets::get(file_name) {
        Some(file) => file.data,
        None => {
            return HashMap::default();
        }
    };

    let content_str = match std::str::from_utf8(&content) {
        Ok(s) => s,
        Err(_) => {
            return HashMap::default();
        }
    };

    serde_json::from_str::<HashMap<String, String>>(content_str).unwrap_or_default()
}

pub fn init(cx: &mut App) {
    let locale = I18nSettings::get_global(cx).locale;
    let i18n = I18n::new(locale);
    cx.set_global(i18n);
}

pub fn set_locale(locale: LocaleContent, cx: &mut App) {
    let i18n = cx.global_mut::<I18n>();
    if i18n.locale != locale {
        let translations = load_translations(locale);
        i18n.locale = locale;
        i18n.translations = translations;
    }
}

pub fn t(key: &str, cx: &App) -> SharedString {
    let i18n = match cx.try_global::<I18n>() {
        Some(i18n) => i18n,
        None => return SharedString::new(key.to_owned()),
    };

    if i18n.locale == LocaleContent::En {
        return SharedString::new(key.to_owned());
    }

    if let Some(translated) = i18n.translations.get(key) {
        SharedString::from(translated.clone())
    } else {
        SharedString::new(key.to_owned())
    }
}

#[derive(Deserialize, RegisterSetting)]
pub struct I18nSettings {
    pub locale: LocaleContent,
}

impl Settings for I18nSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        Self {
            locale: content.locale.unwrap(),
        }
    }
}
