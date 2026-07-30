use std::{collections::HashMap, sync::Arc};

use fluent_bundle::{FluentArgs, FluentResource, concurrent::FluentBundle};
use parking_lot::RwLock;
use unic_langid::LanguageIdentifier;

/// The locale the English literals in the source tree are written in. It is
/// always available as the final fallback and never needs a catalog.
pub const DEFAULT_LOCALE: &str = "en-US";

type Bundle = FluentBundle<Arc<FluentResource>>;

static STATE: RwLock<Option<State>> = RwLock::new(None);

struct State {
    /// The locale that was requested, before negotiation.
    requested: LanguageIdentifier,
    /// Catalogs keyed by the locale they were loaded for.
    catalogs: HashMap<LanguageIdentifier, Bundle>,
    /// Locales to consult, in order, when resolving a key.
    resolution_order: Vec<LanguageIdentifier>,
    /// Bumped whenever resolution results could change, so that the UI layer can
    /// tell when it needs to re-render.
    generation: usize,
}

impl State {
    fn new() -> Self {
        Self {
            requested: default_locale(),
            catalogs: HashMap::default(),
            resolution_order: Vec::new(),
            generation: 0,
        }
    }

    /// Recomputes which locales to consult, in which order, for the requested
    /// locale. `en-US` is never in the order: a miss falls through to the
    /// English literal at the call site.
    fn renegotiate(&mut self) {
        let available = self.catalogs.keys().cloned().collect::<Vec<_>>();
        let default = default_locale();

        self.resolution_order = fluent_langneg::negotiate_languages(
            std::slice::from_ref(&self.requested),
            &available,
            Some(&default),
            fluent_langneg::NegotiationStrategy::Filtering,
        )
        .into_iter()
        .filter(|locale| **locale != default)
        .cloned()
        .collect();

        self.generation += 1;
    }
}

fn default_locale() -> LanguageIdentifier {
    DEFAULT_LOCALE
        .parse()
        .expect("DEFAULT_LOCALE is a valid language identifier")
}

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut guard = STATE.write();
    f(guard.get_or_insert_with(State::new))
}

/// Adds Fluent source to the catalog for `locale`.
///
/// Calling this more than once for the same locale merges the messages;
/// later definitions override earlier ones, which is what lets a user-supplied
/// catalog override the bundled one.
pub fn add_ftl(locale: &LanguageIdentifier, source: String) -> Result<(), Vec<String>> {
    let resource = match FluentResource::try_new(source) {
        Ok(resource) => resource,
        Err((resource, errors)) => {
            // A partially-valid catalog is still worth loading: the messages that
            // parsed are usable, and the ones that did not fall back to English.
            let errors = errors.iter().map(|error| error.to_string()).collect();
            log::warn!("i18n: catalog for {locale} has parse errors: {errors:?}");
            return with_state(|state| {
                insert_resource(state, locale, resource);
                state.renegotiate();
                Err(errors)
            });
        }
    };

    with_state(|state| {
        insert_resource(state, locale, resource);
        state.renegotiate();
    });

    Ok(())
}

fn insert_resource(state: &mut State, locale: &LanguageIdentifier, resource: FluentResource) {
    let bundle = state.catalogs.entry(locale.clone()).or_insert_with(|| {
        let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
        // Fluent wraps placeables in Unicode bidi isolation marks by default,
        // which render as stray glyphs in a UI that does its own bidi handling.
        bundle.set_use_isolating(false);
        bundle
    });

    // Overriding rather than erroring on duplicate keys is what lets a
    // user-supplied catalog take precedence over the bundled one.
    bundle.add_resource_overriding(Arc::new(resource));
}

/// Sets the active locale, returning `true` if resolution results may have
/// changed and the UI should be re-rendered.
///
/// The locale does not need to have a catalog loaded: keys simply fall back to
/// the English literals at their call sites.
pub fn set_locale(locale: LanguageIdentifier) -> bool {
    with_state(|state| {
        if state.requested == locale {
            return false;
        }
        state.requested = locale;
        state.renegotiate();
        true
    })
}

/// The locale that was requested, before negotiation against loaded catalogs.
pub fn current_locale() -> LanguageIdentifier {
    with_state(|state| state.requested.clone())
}

/// The locales that have a catalog loaded, plus the default locale.
pub fn available_locales() -> Vec<LanguageIdentifier> {
    with_state(|state| {
        let mut locales = vec![default_locale()];
        locales.extend(state.catalogs.keys().cloned());
        locales.sort_by_cached_key(|locale| locale.to_string());
        locales.dedup();
        locales
    })
}

/// A counter that changes whenever resolution results could change.
///
/// The UI layer observes this to decide when localized text needs re-rendering,
/// which is what makes switching locales at runtime take effect without
/// rebuilding view trees by hand.
pub fn generation() -> usize {
    with_state(|state| state.generation)
}

/// Looks `key` up in the negotiated catalogs, formatting it with `args`.
///
/// Returns `None` when no catalog defines `key`, in which case the caller falls
/// back to the English literal from the call site.
pub fn lookup(key: &str, args: Option<&FluentArgs<'_>>) -> Option<String> {
    let guard = STATE.read();
    let state = guard.as_ref()?;

    for locale in &state.resolution_order {
        // Skip rather than give up, so that one locale without a catalog cannot
        // cut the fallback chain short.
        let Some(bundle) = state.catalogs.get(locale) else {
            continue;
        };
        let Some(message) = bundle.get_message(key) else {
            continue;
        };
        let Some(pattern) = message.value() else {
            continue;
        };

        let mut errors = Vec::new();
        let formatted = bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            log::warn!("i18n: errors formatting {key:?} for {locale}: {errors:?}");
        }
        return Some(formatted.into_owned());
    }

    None
}

/// Drops every loaded catalog and returns to the default locale.
pub fn reset() {
    *STATE.write() = None;
}

/// Serializes tests that touch the process-global catalog.
///
/// Tests within a crate run on multiple threads, so any test that loads a catalog
/// or changes the locale must hold this for the duration.
#[cfg(test)]
pub(crate) fn lock_for_test() -> parking_lot::MutexGuard<'static, ()> {
    static TEST_LOCK: parking_lot::Mutex<()> = parking_lot::Mutex::new(());
    TEST_LOCK.lock()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> LanguageIdentifier {
        tag.parse().unwrap()
    }

    #[test]
    fn misses_when_no_catalog_is_loaded() {
        let _guard = lock_for_test();
        reset();
        assert_eq!(lookup("zoom-in", None), None);
        assert_eq!(current_locale(), locale("en-US"));
        assert_eq!(available_locales(), vec![locale("en-US")]);
    }

    #[test]
    fn resolves_from_a_loaded_catalog() {
        let _guard = lock_for_test();
        reset();
        add_ftl(&locale("zh-CN"), "zoom-in = 放大\n".to_owned()).unwrap();

        // Not active yet, so nothing resolves.
        assert_eq!(lookup("zoom-in", None), None);

        assert!(set_locale(locale("zh-CN")));
        assert_eq!(lookup("zoom-in", None).as_deref(), Some("放大"));
        // A key with no translation still misses, so callers show English.
        assert_eq!(lookup("zoom-out", None), None);

        // Switching back to the default locale stops resolving.
        assert!(set_locale(locale("en-US")));
        assert_eq!(lookup("zoom-in", None), None);
    }

    #[test]
    fn falls_back_across_the_negotiated_chain() {
        let _guard = lock_for_test();
        reset();
        add_ftl(
            &locale("zh"),
            "zoom-in = 放大\nzoom-out = 缩小\n".to_owned(),
        )
        .unwrap();
        add_ftl(&locale("zh-CN"), "zoom-in = 放大一点\n".to_owned()).unwrap();

        set_locale(locale("zh-CN"));
        // The region-specific catalog wins where it defines a key...
        assert_eq!(lookup("zoom-in", None).as_deref(), Some("放大一点"));
        // ...and the base-language catalog covers the rest.
        assert_eq!(lookup("zoom-out", None).as_deref(), Some("缩小"));
    }

    #[test]
    fn formats_placeables() {
        let _guard = lock_for_test();
        reset();
        add_ftl(
            &locale("zh-CN"),
            "renaming-count-files = 正在重命名 {$count} 个文件\n".to_owned(),
        )
        .unwrap();
        set_locale(locale("zh-CN"));

        let mut args = FluentArgs::new();
        args.set("count", 3);
        assert_eq!(
            lookup("renaming-count-files", Some(&args)).as_deref(),
            Some("正在重命名 3 个文件")
        );
    }

    #[test]
    fn preserves_the_line_breaks_in_a_block_value() {
        let _guard = lock_for_test();
        reset();
        // A quoted Fluent value cannot contain a line break, so a message whose
        // English literal has them is written as a block value instead. Catalogs
        // rely on the lines coming back joined by newlines.
        add_ftl(
            &locale("zh-CN"),
            "format-on-save =\n    On：格式化整个缓冲区。\n    Off：不格式化。\n".to_owned(),
        )
        .unwrap();
        set_locale(locale("zh-CN"));

        assert_eq!(
            lookup("format-on-save", None).as_deref(),
            Some("On：格式化整个缓冲区。\nOff：不格式化。")
        );
    }

    #[test]
    fn tolerates_a_partially_invalid_catalog() {
        let _guard = lock_for_test();
        reset();
        // The second line is not valid Fluent; the first should still load.
        let result = add_ftl(&locale("zh-CN"), "zoom-in = 放大\n= broken\n".to_owned());
        assert!(result.is_err(), "parse errors should be reported");

        set_locale(locale("zh-CN"));
        assert_eq!(lookup("zoom-in", None).as_deref(), Some("放大"));
    }

    #[test]
    fn tracks_generation_across_changes() {
        let _guard = lock_for_test();
        reset();
        let start = generation();

        add_ftl(&locale("zh-CN"), "zoom-in = 放大\n".to_owned()).unwrap();
        let after_load = generation();
        assert!(after_load > start, "loading a catalog bumps the generation");

        assert!(set_locale(locale("zh-CN")));
        let after_switch = generation();
        assert!(after_switch > after_load, "switching locale bumps it");

        // Setting the same locale again is a no-op.
        assert!(!set_locale(locale("zh-CN")));
        assert_eq!(generation(), after_switch);
    }
}
