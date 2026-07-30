/// Derives a Fluent message identifier from an English source string.
///
/// The identifier is the source string reduced to lower-kebab-case. Deriving the
/// key from the English text (rather than hand-writing one at every call site)
/// keeps call sites readable and lets the English literal double as the
/// fallback that is displayed when no translation is available.
///
/// Fluent identifiers must match `[a-zA-Z][a-zA-Z0-9_-]*`, so any run of
/// characters that is not ASCII alphanumeric collapses into a single `-`, and a
/// key that would not start with a letter is prefixed with `t-`.
///
/// Fluent placeables contribute their variable name to the key, which keeps keys
/// for parameterized messages stable and distinct:
/// `"Renaming {$count} files"` becomes `renaming-count-files`.
pub fn derive_key(source: &str) -> String {
    let mut key = String::with_capacity(source.len());
    let mut pending_separator = false;

    for ch in source.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_separator && !key.is_empty() {
                key.push('-');
            }
            pending_separator = false;
            key.push(ch.to_ascii_lowercase());
        } else {
            pending_separator = true;
        }
    }

    if key.is_empty() {
        // A source string with no ASCII alphanumerics (e.g. "…") has no
        // meaningful key; give it a valid, stable one.
        return "t".to_owned();
    }

    if !key.starts_with(|ch: char| ch.is_ascii_alphabetic()) {
        key.insert_str(0, "t-");
    }

    key
}

/// Derives a key and leaks it so it can be memoized in a `'static` slot.
///
/// The number of distinct call sites is bounded by the size of the binary, so
/// each key is leaked at most once and the total is bounded.
pub fn derive_key_leaked(source: &str) -> &'static str {
    Box::leak(derive_key(source).into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::derive_key;

    #[test]
    fn derives_kebab_case_keys() {
        assert_eq!(derive_key("Zoom In"), "zoom-in");
        assert_eq!(derive_key("Reset All Zoom"), "reset-all-zoom");
        assert_eq!(derive_key("Toggle Left Dock"), "toggle-left-dock");
    }

    #[test]
    fn collapses_punctuation_runs() {
        assert_eq!(derive_key("Save As…"), "save-as");
        assert_eq!(
            derive_key("Copy Permalink to Line"),
            "copy-permalink-to-line"
        );
        assert_eq!(derive_key("  Open   Recent  "), "open-recent");
        assert_eq!(derive_key("Find/Replace"), "find-replace");
    }

    #[test]
    fn placeables_contribute_their_variable_name() {
        assert_eq!(
            derive_key("Renaming {$count} files"),
            "renaming-count-files"
        );
        assert_eq!(derive_key("{$name}'s cursor"), "name-s-cursor");
    }

    #[test]
    fn produces_valid_fluent_identifiers() {
        // Must start with a letter.
        assert_eq!(derive_key("3 files changed"), "t-3-files-changed");
        assert_eq!(derive_key("…"), "t");
        assert_eq!(derive_key(""), "t");

        for source in [
            "Zoom In",
            "Save As…",
            "3 files changed",
            "…",
            "",
            "Renaming {$count} files",
        ] {
            let key = derive_key(source);
            assert!(
                key.starts_with(|ch: char| ch.is_ascii_alphabetic()),
                "{key:?} must start with a letter"
            );
            assert!(
                key.chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'),
                "{key:?} must only contain identifier characters"
            );
        }
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(derive_key("Zoom In"), derive_key("Zoom In"));
        // Strings differing only in punctuation or case collide by design; the
        // extraction tool reports these so they can be disambiguated.
        assert_eq!(derive_key("Zoom In"), derive_key("zoom in"));
    }
}
