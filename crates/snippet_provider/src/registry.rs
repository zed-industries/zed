use std::{path::Path, sync::Arc};

use anyhow::Result;
use collections::HashMap;
use gpui::{App, Global, ReadGlobal, UpdateGlobal};
use parking_lot::RwLock;
use util::ResultExt;

use crate::{Snippet, SnippetKind, file_stem_to_key};

struct GlobalSnippetRegistry(Arc<SnippetRegistry>);

impl Global for GlobalSnippetRegistry {}

#[derive(Default)]
pub struct SnippetRegistry {
    snippets: RwLock<HashMap<SnippetKind, Vec<Arc<Snippet>>>>,
}

impl SnippetRegistry {
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalSnippetRegistry::global(cx).0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Arc<Self>> {
        cx.try_global::<GlobalSnippetRegistry>()
            .map(|registry| registry.0.clone())
    }

    pub fn init_global(cx: &mut App) {
        GlobalSnippetRegistry::set_global(cx, GlobalSnippetRegistry(Arc::new(Self::new())))
    }

    pub fn new() -> Self {
        Self {
            snippets: RwLock::new(HashMap::default()),
        }
    }

    pub fn register_snippets(&self, file_path: &Path, contents: &str) -> Result<()> {
        let snippets_in_file: crate::format::VsSnippetsFile =
            serde_json_lenient::from_str(contents)?;
        let kind = file_path
            .file_stem()
            .and_then(|stem| stem.to_str().and_then(file_stem_to_key));
        let new_snippets =
            crate::file_to_snippets(snippets_in_file, file_path).filter_map(Result::log_err);
        let mut snippets = self.snippets.write();
        let existing = snippets.entry(kind).or_default();
        for snippet in new_snippets {
            // Only the first prefix is currently used to trigger a snippet completion, so
            // dedup on it alone for now rather than the full prefix list.
            if !existing
                .iter()
                .any(|s| s.prefix.first() == snippet.prefix.first())
            {
                existing.push(snippet);
            }
        }

        Ok(())
    }

    pub fn get_snippets(&self, kind: &SnippetKind) -> Vec<Arc<Snippet>> {
        self.snippets.read().get(kind).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_register_snippets_single_language() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("rust.json"),
                r#"{"Hello World": {"prefix": "hello", "body": "Hello, ${1:World}!"}}"#,
            )
            .unwrap();

        let snippets = registry.get_snippets(&Some("rust".to_owned()));
        assert_eq!(snippets.len(), 1);
        assert_eq!(snippets[0].name, "Hello World");
        assert_eq!(snippets[0].prefix, vec!["hello".to_owned()]);
    }

    #[test]
    fn test_register_snippets_two_extensions_same_language() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"Snippet One": {"prefix": "one", "body": "snippet_one"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"Snippet Two": {"prefix": "two", "body": "snippet_two"}}"#,
            )
            .unwrap();

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            2,
            "Snippets from both extensions should be present"
        );
        let names: Vec<&str> = snippets.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"Snippet One"));
        assert!(names.contains(&"Snippet Two"));
    }

    #[test]
    fn test_register_snippets_same_first_prefix_deduplicated() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"Different Name": {"prefix": "for", "body": "${2:iter}.each do |${1:item}|\n$0\nend"}}"#,
            )
            .unwrap();

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            1,
            "Snippets sharing the same first prefix should be deduplicated, keeping the first"
        );
        assert_eq!(snippets[0].name, "For Loop");
    }

    #[test]
    fn test_register_snippets_extra_prefixes_ignored_for_dedup() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"For Loop Alt": {"prefix": ["for", "floop"], "body": "${2:iter}.each do |${1:item}|\n$0\nend"}}"#,
            )
            .unwrap();

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            1,
            "Only the first prefix is currently used to trigger completions, \
            so a matching first prefix is a duplicate regardless of any extra prefixes"
        );
    }

    #[test]
    fn test_register_snippets_different_first_prefixes_not_deduplicated() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("ruby.json"),
                r#"{"While Loop": {"prefix": "while", "body": "while ${1:cond}\n$0\nend"}}"#,
            )
            .unwrap();

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            2,
            "Snippets with different first prefixes should not be deduplicated"
        );
    }

    #[test]
    fn test_register_global_snippets() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("snippets.json"),
                r#"{"Global Snippet": {"prefix": "global", "body": "a global snippet"}}"#,
            )
            .unwrap();

        let global_snippets = registry.get_snippets(&None);
        assert_eq!(global_snippets.len(), 1);
        assert_eq!(global_snippets[0].name, "Global Snippet");

        let miskeyed = registry.get_snippets(&Some("snippets".to_owned()));
        assert!(
            miskeyed.is_empty(),
            "Should not be stored under the language key 'snippets'"
        );
    }

    #[test]
    fn test_register_snippets_different_languages_are_isolated() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("rust.json"),
                r#"{"Rust Snippet": {"prefix": "rsnip", "body": "fn ${1:name}() {}"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("python.json"),
                r#"{"Python Snippet": {"prefix": "pysnip", "body": "def ${1:name}():"}}"#,
            )
            .unwrap();

        let rust_snippets = registry.get_snippets(&Some("rust".to_owned()));
        assert_eq!(rust_snippets.len(), 1);
        assert_eq!(rust_snippets[0].name, "Rust Snippet");

        let python_snippets = registry.get_snippets(&Some("python".to_owned()));
        assert_eq!(python_snippets.len(), 1);
        assert_eq!(python_snippets[0].name, "Python Snippet");
    }

    #[test]
    fn test_get_snippets_unknown_language_returns_empty() {
        let registry = SnippetRegistry::new();
        assert!(
            registry
                .get_snippets(&Some("nonexistent".to_owned()))
                .is_empty()
        );
        assert!(registry.get_snippets(&None).is_empty());
    }
}
