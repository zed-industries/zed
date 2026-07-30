use gpui_shared_string::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

static NEXT_LANGUAGE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct LanguageId(usize);

impl LanguageId {
    pub fn new() -> Self {
        Self(NEXT_LANGUAGE_ID.fetch_add(1, SeqCst))
    }
}

impl Default for LanguageId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct LanguageName(pub SharedString);

impl LanguageName {
    pub fn new(s: &str) -> Self {
        Self(SharedString::new(s))
    }

    pub fn new_static(s: &'static str) -> Self {
        Self(SharedString::new_static(s))
    }

    pub fn from_proto(s: String) -> Self {
        Self(SharedString::from(s))
    }

    pub fn to_proto(&self) -> String {
        self.0.to_string()
    }

    pub fn lsp_id(&self) -> String {
        match self.0.as_ref() {
            "Plain Text" => "plaintext".to_string(),
            language_name => language_name.to_lowercase(),
        }
    }

    /// Identifier used to name and look up this language's snippet file.
    ///
    /// Path separators are stripped from the `lsp_id` because the snippet file
    /// is stored as a flat file directly under the snippets directory; a `/`
    /// (or `\`) would otherwise place it in a subdirectory that is never
    /// scanned, making the snippet impossible to use.
    pub fn snippet_scope_id(&self) -> String {
        self.lsp_id().replace(['/', '\\'], "")
    }
}

impl From<LanguageName> for SharedString {
    fn from(value: LanguageName) -> Self {
        value.0
    }
}

impl From<SharedString> for LanguageName {
    fn from(value: SharedString) -> Self {
        LanguageName(value)
    }
}

impl AsRef<str> for LanguageName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for LanguageName {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl PartialEq<str> for LanguageName {
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
    }
}

impl PartialEq<&str> for LanguageName {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_ref() == *other
    }
}

impl std::fmt::Display for LanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for LanguageName {
    fn from(str: &'static str) -> Self {
        Self(SharedString::new_static(str))
    }
}

impl From<LanguageName> for String {
    fn from(value: LanguageName) -> Self {
        let value: &str = &value.0;
        Self::from(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snippet_scope_id_strips_path_separators() {
        // A `/` or `\` in the language name would route the snippet file into a
        // subdirectory that is never scanned (see issue #59620).
        assert_eq!(LanguageName::new("PL/X").snippet_scope_id(), "plx");
        assert_eq!(LanguageName::new("a\\b").snippet_scope_id(), "ab");
        // Names without separators are unchanged beyond `lsp_id`'s lowercasing.
        assert_eq!(LanguageName::new("Rust").snippet_scope_id(), "rust");
        assert_eq!(
            LanguageName::new("Shell Script").snippet_scope_id(),
            "shell script"
        );
        assert_eq!(
            LanguageName::new("Plain Text").snippet_scope_id(),
            "plaintext"
        );
    }
}
