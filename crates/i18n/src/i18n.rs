use std::sync::Arc;

use anyhow::{Context as _, Result};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use gpui::{App, AssetSource, Global, SharedString};
use settings::SettingsStore;
use unic_langid::LanguageIdentifier;

pub use fluent::FluentArgs as TranslationArgs;

struct GlobalI18n {
    bundle: Arc<FluentBundle<FluentResource>>,
    fallback_bundle: Arc<FluentBundle<FluentResource>>,
    #[allow(dead_code)]
    locale: LanguageIdentifier,
}

impl Global for GlobalI18n {}

pub fn init(cx: &mut App) {
    let fallback_locale = english_locale();
    let fallback_bundle = Arc::new(
        load_bundle(&fallback_locale, cx.asset_source().as_ref())
            .expect("English locale file (locales/en.ftl) must be loadable"),
    );

    let locale = resolve_locale(cx);
    let bundle = load_locale_bundle(&locale, &fallback_bundle, cx.asset_source().as_ref());

    cx.set_global(GlobalI18n {
        bundle,
        fallback_bundle,
        locale,
    });

    let mut previous_locale_setting = read_locale_setting(cx);
    cx.observe_global::<SettingsStore>(move |cx| {
        let current_locale_setting = read_locale_setting(cx);
        if current_locale_setting != previous_locale_setting {
            previous_locale_setting = current_locale_setting;
            reload_locale(cx);
        }
    })
    .detach();
}

pub fn t(cx: &App, message_id: &str) -> SharedString {
    t_with_args(cx, message_id, None)
}

pub fn t_with_args(cx: &App, message_id: &str, args: Option<&FluentArgs<'_>>) -> SharedString {
    let i18n = cx.global::<GlobalI18n>();
    if let Some(result) = format_message(&i18n.bundle, message_id, args) {
        return result.into();
    }
    if let Some(result) = format_message(&i18n.fallback_bundle, message_id, args) {
        return result.into();
    }
    log::warn!("Missing translation for message ID '{message_id}'");
    SharedString::from(message_id.to_string())
}

pub trait Localize {
    fn t(&self, message_id: &str) -> SharedString;
    fn t_with_args(&self, message_id: &str, args: &FluentArgs<'_>) -> SharedString;
}

impl Localize for App {
    fn t(&self, message_id: &str) -> SharedString {
        t(self, message_id)
    }

    fn t_with_args(&self, message_id: &str, args: &FluentArgs<'_>) -> SharedString {
        t_with_args(self, message_id, Some(args))
    }
}

fn english_locale() -> LanguageIdentifier {
    "en".parse().expect("'en' is a valid locale identifier")
}

fn resolve_locale(cx: &App) -> LanguageIdentifier {
    let locale_setting = read_locale_setting(cx);

    if locale_setting == "system" || locale_setting.is_empty() {
        sys_locale::get_locale()
            .and_then(|locale| locale.parse::<LanguageIdentifier>().ok())
            .unwrap_or_else(english_locale)
    } else {
        locale_setting.parse::<LanguageIdentifier>().unwrap_or_else(|_| {
            log::warn!(
                "Invalid locale setting '{locale_setting}', falling back to English"
            );
            english_locale()
        })
    }
}

fn read_locale_setting(cx: &App) -> String {
    let store = cx.global::<SettingsStore>();

    if let Some(user_settings) = store.raw_user_settings() {
        if let Some(ref locale) = user_settings.content.locale {
            return locale.clone();
        }
    }

    store
        .raw_default_settings()
        .locale
        .clone()
        .unwrap_or_else(|| "system".to_string())
}

fn load_locale_bundle(
    locale: &LanguageIdentifier,
    fallback_bundle: &Arc<FluentBundle<FluentResource>>,
    asset_source: &dyn AssetSource,
) -> Arc<FluentBundle<FluentResource>> {
    let fallback_locale = english_locale();
    if locale.language == fallback_locale.language {
        return fallback_bundle.clone();
    }

    match load_bundle(locale, asset_source) {
        Ok(bundle) => Arc::new(bundle),
        Err(error) => {
            log::error!("Failed to load locale '{locale}': {error:#}. Using English.");
            fallback_bundle.clone()
        }
    }
}

fn load_bundle(
    locale: &LanguageIdentifier,
    asset_source: &dyn AssetSource,
) -> Result<FluentBundle<FluentResource>> {
    let path = format!("locales/{}.ftl", locale.language);
    let data = asset_source
        .load(&path)?
        .with_context(|| format!("Locale file not found: {path}"))?;
    let source = std::str::from_utf8(&data)
        .context("Locale file is not valid UTF-8")?
        .to_string();
    let resource = FluentResource::try_new(source)
        .map_err(|(_, errors)| anyhow::anyhow!("Fluent parse errors: {errors:?}"))?;
    let mut bundle = FluentBundle::new(vec![locale.clone()]);
    bundle
        .add_resource(resource)
        .map_err(|errors| anyhow::anyhow!("Fluent bundle errors: {errors:?}"))?;
    Ok(bundle)
}

fn reload_locale(cx: &mut App) {
    let locale = resolve_locale(cx);
    let asset_source = cx.asset_source().clone();
    let fallback_bundle = cx.global::<GlobalI18n>().fallback_bundle.clone();
    let bundle = load_locale_bundle(&locale, &fallback_bundle, asset_source.as_ref());

    cx.update_global::<GlobalI18n, _>(|i18n, _| {
        i18n.bundle = bundle;
        i18n.fallback_bundle = fallback_bundle;
        i18n.locale = locale;
    });
    cx.refresh_windows();
}

fn format_message(
    bundle: &FluentBundle<FluentResource>,
    message_id: &str,
    args: Option<&FluentArgs<'_>>,
) -> Option<String> {
    let message = bundle.get_message(message_id)?;
    let pattern = message.value()?;
    let mut errors = vec![];
    let result = bundle.format_pattern(pattern, args, &mut errors);
    if !errors.is_empty() {
        log::warn!("Fluent formatting errors for '{message_id}': {errors:?}");
    }
    Some(result.into_owned())
}
