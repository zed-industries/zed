//! Localization of Zed's user interface.
//!
//! # How a string gets localized
//!
//! Wrap the English literal at its call site in [`t!`]:
//!
//! ```ignore
//! MenuItem::action(t!("Zoom In"), zed_actions::IncreaseBufferFontSize::default())
//! ```
//!
//! [`t!`] derives a catalog key from the English text (`Zoom In` becomes
//! `zoom-in`, see [`derive_key`]) and returns a [`LocalizedString`], which
//! resolves against the active locale when it is read rather than when it is
//! constructed. Because [`LocalizedString`] converts into
//! [`SharedString`](gpui_shared_string::SharedString), call sites that already
//! take `impl Into<SharedString>` accept `t!(..)` without a signature change.
//!
//! # Resolving late
//!
//! Resolution happens on read so that changing the locale takes effect on the
//! next render, without rebuilding view trees by hand. A view that stores a
//! [`LocalizedString`] and resolves it in `render` picks up the new locale
//! automatically; the UI layer watches [`generation`] to know when to re-render.
//!
//! # Falling back
//!
//! A key with no translation resolves to the English literal from its call site.
//! Nothing is ever blank because a catalog is incomplete, so call sites can be
//! migrated to [`t!`] a crate at a time and catalogs can be translated
//! incrementally.

mod catalog;
mod key;
mod localized;

#[cfg(any(test, feature = "test-support"))]
pub use catalog::lock_for_test;
pub use catalog::{
    DEFAULT_LOCALE, DISPLAY_NAME_KEY, add_ftl, available_locales, current_locale, display_label,
    display_name, generation, reset, set_locale,
};
pub use key::{derive_key, derive_key_leaked};
pub use localized::{ArgValue, LocalizedString};

pub use unic_langid::LanguageIdentifier;

/// Marks a user-facing English literal for localization.
///
/// The catalog key is derived from the literal by [`derive_key`] and memoized per
/// call site, so resolving costs a catalog lookup and no string processing.
///
/// ```ignore
/// // Without placeables:
/// Label::new(t!("Zoom In"))
///
/// // With placeables, which the catalog can also use for plural selection:
/// Label::new(t!("Renaming {$count} files", count = files.len()))
/// ```
///
/// The literal must be a string literal, not a runtime value: it is both the key
/// source and the fallback, and it is what the extraction tool scans for when it
/// builds the English catalog.
///
/// # Overriding the derived key
///
/// Because the key is derived from the literal, two literals that differ only in
/// case or punctuation derive the same key and would have to share one
/// translation — `"Open"` and `"Open…"` both derive `open`. When both spellings
/// have to stay, one of them names its key explicitly:
///
/// ```ignore
/// // The File menu keeps the derived `open`…
/// MenuItem::action(t!("Open…"), workspace::Open)
/// // …so the button that confirms an already-chosen project takes its own key,
/// // and can be translated without the menu's trailing ellipsis.
/// Button::new("open", t!(key = "open-action", "Open"))
/// ```
///
/// Prefer the derived key: an explicit one is a second thing to keep in sync.
/// Reach for this only to break a collision that `script/i18n-coverage` reports.
#[macro_export]
macro_rules! t {
    // The trailing comma is optional here because rustfmt adds one whenever the
    // literal is long enough to wrap onto its own line.
    ($source:literal $(,)?) => {{
        static KEY: ::std::sync::OnceLock<&'static str> = ::std::sync::OnceLock::new();
        $crate::LocalizedString::from_parts(
            *KEY.get_or_init(|| $crate::derive_key_leaked($source)),
            $source,
        )
    }};
    ($source:literal, $($name:ident = $value:expr),+ $(,)?) => {{
        static KEY: ::std::sync::OnceLock<&'static str> = ::std::sync::OnceLock::new();
        $crate::LocalizedString::with_args(
            *KEY.get_or_init(|| $crate::derive_key_leaked($source)),
            $source,
            ::std::vec![
                $((::std::stringify!($name), $crate::ArgValue::from($value))),+
            ],
        )
    }};
    (key = $key:literal, $source:literal $(,)?) => {
        $crate::LocalizedString::from_parts($key, $source)
    };
    (key = $key:literal, $source:literal, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::LocalizedString::with_args(
            $key,
            $source,
            ::std::vec![
                $((::std::stringify!($name), $crate::ArgValue::from($value))),+
            ],
        )
    };
}

#[cfg(test)]
mod tests {
    use crate as i18n;

    #[test]
    fn derives_the_key_from_the_literal() {
        let _guard = crate::catalog::lock_for_test();
        i18n::reset();

        let zoom_in = t!("Zoom In");
        assert_eq!(zoom_in.key(), "zoom-in");
        assert_eq!(zoom_in.fallback(), "Zoom In");
        assert_eq!(zoom_in.resolve().as_ref(), "Zoom In");
    }

    #[test]
    fn resolves_against_the_active_locale() {
        let _guard = crate::catalog::lock_for_test();
        i18n::reset();

        i18n::add_ftl(
            &"zh-CN".parse().unwrap(),
            "zoom-in = 放大\nrenaming-count-files = 正在重命名 {$count} 个文件\n".to_owned(),
        )
        .unwrap();
        i18n::set_locale("zh-CN".parse().unwrap());

        assert_eq!(t!("Zoom In").resolve().as_ref(), "放大");
        assert_eq!(
            t!("Renaming {$count} files", count = 4).resolve().as_ref(),
            "正在重命名 4 个文件"
        );
        // Untranslated keys still show their English literal.
        assert_eq!(t!("Zoom Out").resolve().as_ref(), "Zoom Out");

        i18n::reset();
    }

    #[test]
    fn accepts_varied_argument_types() {
        let _guard = crate::catalog::lock_for_test();
        i18n::reset();

        let owned = String::from("main.rs");
        assert_eq!(
            t!("{$name} has {$count} changes", name = owned, count = 2usize)
                .resolve()
                .as_ref(),
            "main.rs has 2 changes"
        );
        assert_eq!(
            t!("{$ratio} done", ratio = 0.5).resolve().as_ref(),
            "0.5 done"
        );
    }

    #[test]
    fn an_explicit_key_overrides_the_derived_one() {
        let _guard = crate::catalog::lock_for_test();
        i18n::reset();

        // `"Open"` would otherwise derive `open`, which the File menu's `"Open…"`
        // already owns.
        let open = t!(key = "open-action", "Open");
        assert_eq!(open.key(), "open-action");
        assert_eq!(open.fallback(), "Open");

        i18n::add_ftl(
            &"zh-CN".parse().unwrap(),
            "open = 打开…\nopen-action = 打开\nrenamed-count = 已重命名 {$count} 个\n".to_owned(),
        )
        .unwrap();
        i18n::set_locale("zh-CN".parse().unwrap());

        assert_eq!(t!("Open…").resolve().as_ref(), "打开…");
        assert_eq!(t!(key = "open-action", "Open").resolve().as_ref(), "打开");
        assert_eq!(
            t!(key = "renamed-count", "Renamed {$count} files", count = 2)
                .resolve()
                .as_ref(),
            "已重命名 2 个"
        );

        i18n::reset();
    }

    #[test]
    fn accepts_a_trailing_comma_after_the_source() {
        // rustfmt puts a long literal on its own line and adds a trailing comma,
        // so both argument-less forms have to tolerate one.
        assert_eq!(
            t!("Edit and save files directly in the results multibuffer!",).key(),
            "edit-and-save-files-directly-in-the-results-multibuffer"
        );
        assert_eq!(t!(key = "open-action", "Open",).key(), "open-action");
    }

    #[test]
    fn memoizes_the_key_per_call_site() {
        let _guard = crate::catalog::lock_for_test();
        i18n::reset();

        // Resolving repeatedly must keep returning the same key, which exercises
        // the `OnceLock` path after it has been initialized.
        for _ in 0..3 {
            assert_eq!(t!("Toggle Left Dock").key(), "toggle-left-dock");
        }
    }
}
