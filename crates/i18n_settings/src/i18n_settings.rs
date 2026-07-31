//! Connects the [`i18n`] catalog to Zed's settings and asset bundle.
//!
//! This is kept out of the [`i18n`] crate so that crates which only need to mark
//! strings for localization do not take on a dependency on `gpui` or `settings`.

use gpui::{App, AssetSource};
use i18n::LanguageIdentifier;
use settings::{Settings, SettingsStore};

/// The directory inside the asset bundle that holds the bundled catalogs, laid
/// out as `i18n/<locale>/<name>.ftl`.
const CATALOG_DIR: &str = "i18n";

/// The locale Zed renders its interface in.
#[derive(Clone, Debug, PartialEq)]
pub struct UiLanguageSettings {
    /// The configured locale. Not necessarily one that has a catalog: without
    /// one, the interface stays in English.
    pub locale: LanguageIdentifier,
}

impl Settings for UiLanguageSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let configured = content
            .ui_language
            .as_ref()
            .map(|language| language.0.as_ref())
            .expect("ui_language must have a default in default.json");

        let locale = configured.parse().unwrap_or_else(|error| {
            log::warn!(
                "i18n: {configured:?} is not a valid language tag ({error}); \
                 falling back to {}",
                i18n::DEFAULT_LOCALE
            );
            default_locale()
        });

        Self { locale }
    }
}

fn default_locale() -> LanguageIdentifier {
    i18n::DEFAULT_LOCALE
        .parse()
        .expect("DEFAULT_LOCALE is a valid language tag")
}

/// Loads the bundled catalogs and keeps the active locale in sync with settings.
pub fn init(assets: Box<dyn AssetSource>, cx: &mut App) {
    UiLanguageSettings::register(cx);
    load_bundled_catalogs(assets.as_ref());
    apply_configured_locale(cx);

    cx.observe_global::<SettingsStore>(|cx| {
        if apply_configured_locale(cx) {
            // Localized text resolves when it is read, so a re-render is all a
            // locale change needs; view trees do not have to be rebuilt.
            cx.refresh_windows();
        }
    })
    .detach();
}

/// Points the catalog at the configured locale, returning whether it changed.
fn apply_configured_locale(cx: &mut App) -> bool {
    let locale = UiLanguageSettings::get_global(cx).locale.clone();
    i18n::set_locale(locale)
}

fn load_bundled_catalogs(assets: &dyn AssetSource) {
    let paths = match assets.list(CATALOG_DIR) {
        Ok(paths) => paths,
        Err(error) => {
            log::warn!("i18n: could not list bundled catalogs: {error}");
            return;
        }
    };

    for path in paths {
        if !path.ends_with(".ftl") {
            continue;
        }

        let Some(locale) = locale_from_path(&path) else {
            log::warn!("i18n: skipping catalog at {path:?}: no locale in its path");
            continue;
        };

        let source = match assets.load(&path) {
            Ok(Some(bytes)) => match String::from_utf8(bytes.into_owned()) {
                Ok(source) => source,
                Err(error) => {
                    log::warn!("i18n: catalog at {path:?} is not valid UTF-8: {error}");
                    continue;
                }
            },
            Ok(None) => continue,
            Err(error) => {
                log::warn!("i18n: could not load catalog at {path:?}: {error}");
                continue;
            }
        };

        // Parse errors are reported by `add_ftl`; the messages that did parse are
        // still loaded, so one bad entry does not cost a whole language.
        let _ = i18n::add_ftl(&locale, source);
    }
}

/// Loads the bundled catalogs, returning any parse errors per catalog path.
///
/// Only used by tests, which assert that the catalogs shipped in the binary parse
/// cleanly rather than silently degrading to English at runtime.
#[cfg(test)]
fn load_bundled_catalogs_reporting_errors(assets: &dyn AssetSource) -> Vec<(String, Vec<String>)> {
    let mut failures = Vec::new();

    for path in assets
        .list(CATALOG_DIR)
        .expect("catalogs should be listable")
    {
        if !path.ends_with(".ftl") {
            continue;
        }
        let locale = locale_from_path(&path).expect("catalog path should name a locale");
        let bytes = assets
            .load(&path)
            .expect("catalog should load")
            .expect("catalog should exist");
        let source = String::from_utf8(bytes.into_owned()).expect("catalog should be UTF-8");

        if let Err(errors) = i18n::add_ftl(&locale, source) {
            failures.push((path.to_string(), errors));
        }
    }

    failures
}

/// Extracts the locale from a bundled catalog path such as
/// `i18n/zh-CN/zed.ftl`.
fn locale_from_path(path: &str) -> Option<LanguageIdentifier> {
    path.strip_prefix(CATALOG_DIR)?
        .trim_start_matches('/')
        .split('/')
        .next()
        .filter(|segment| !segment.is_empty())?
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::{load_bundled_catalogs, locale_from_path};
    use i18n::t;

    /// Serializes the tests that touch the process-global catalog, since tests in
    /// a crate run on multiple threads.
    fn lock_catalog() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Covers the whole chain that a running Zed uses: enumerate the bundled
    /// catalogs out of the asset source, parse them, negotiate the locale,
    /// resolve a string through `t!`, and hand the result to a menu that takes
    /// `impl Into<SharedString>`.
    #[test]
    fn loads_the_bundled_simplified_chinese_catalog() {
        let _guard = lock_catalog();
        i18n::reset();
        load_bundled_catalogs(&assets::Assets);

        assert!(
            i18n::available_locales()
                .iter()
                .any(|locale| *locale == "zh-CN"),
            "zh-CN should be discovered from the asset bundle, found {:?}",
            i18n::available_locales()
        );

        i18n::set_locale("zh-CN".parse().unwrap());

        assert_eq!(t!("Quit Zed").resolve().as_ref(), "退出 Zed");
        assert_eq!(t!("Zoom In").resolve().as_ref(), "放大");
        assert_eq!(t!("Save As…").resolve().as_ref(), "另存为…");
        // A string the catalog does not define stays in English.
        assert_eq!(
            t!("Not A Real Menu Item").resolve().as_ref(),
            "Not A Real Menu Item"
        );

        // Existing call sites take `impl Into<SharedString>`, so `t!` has to
        // localize without any signature change. This is what the app menus rely
        // on.
        let quit = gpui::MenuItem::action(t!("Quit Zed"), gpui::NoAction);
        let gpui::MenuItem::Action { name, .. } = quit else {
            panic!("expected an action item");
        };
        assert_eq!(name.as_ref(), "退出 Zed");

        assert_eq!(gpui::Menu::new(t!("File")).name.as_ref(), "文件");
        assert_eq!(gpui::Menu::new(t!("Selection")).name.as_ref(), "选择");

        // Switching back to the source locale stops consulting the catalog.
        i18n::set_locale("en-US".parse().unwrap());
        assert_eq!(t!("Quit Zed").resolve().as_ref(), "Quit Zed");

        i18n::reset();
    }

    #[test]
    fn reads_the_locale_out_of_a_catalog_path() {
        assert_eq!(
            locale_from_path("i18n/zh-CN/zed.ftl").map(|l| l.to_string()),
            Some("zh-CN".to_owned())
        );
        assert_eq!(
            locale_from_path("i18n/ja/menus.ftl").map(|l| l.to_string()),
            Some("ja".to_owned())
        );
    }

    /// A catalog with a syntax error degrades silently to English at runtime, so
    /// the bundled ones are checked here instead.
    #[test]
    fn bundled_catalogs_parse_cleanly() {
        let _guard = lock_catalog();
        i18n::reset();
        let failures = super::load_bundled_catalogs_reporting_errors(&assets::Assets);
        i18n::reset();

        assert!(
            failures.is_empty(),
            "bundled catalogs have parse errors: {failures:#?}"
        );
    }

    #[test]
    fn rejects_paths_without_a_usable_locale() {
        assert_eq!(locale_from_path("i18n/zed.ftl"), None);
        assert_eq!(locale_from_path("i18n/"), None);
        assert_eq!(locale_from_path("i18n"), None);
        assert_eq!(locale_from_path("themes/one/one.json"), None);
    }
}
