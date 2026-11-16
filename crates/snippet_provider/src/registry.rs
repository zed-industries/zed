use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use collections::{HashMap, IndexMap};
use gpui::{App, Global, ReadGlobal, UpdateGlobal};
use parking_lot::RwLock;
use util::ResultExt;

use crate::{Snippet, SnippetKind, file_stem_to_key};

struct GlobalSnippetRegistry(Arc<SnippetRegistry>);

impl Global for GlobalSnippetRegistry {}

#[derive(Default)]
pub struct SnippetRegistry {
    snippets: RwLock<HashMap<SnippetKind, IndexMap<PathBuf, Vec<Arc<Snippet>>>>>,
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
        let new_snippets = crate::file_to_snippets(snippets_in_file, file_path)
            .filter_map(Result::log_err)
            .collect();

        self.snippets
            .write()
            .entry(kind)
            .or_default()
            .insert(file_path.to_path_buf(), new_snippets);

        Ok(())
    }

    pub fn get_snippets(&self, kind: &SnippetKind) -> Vec<Arc<Snippet>> {
        self.snippets
            .read()
            .get(kind)
            .into_iter()
            .flat_map(|snippets_by_file| snippets_by_file.values().flatten().cloned())
            .collect()
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
                Path::new("first_extension/ruby.json"),
                r#"{"Snippet One": {"prefix": "one", "body": "snippet_one"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("second_extension/ruby.json"),
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
    fn test_register_snippets_same_file_replaces_previous_contents() -> Result<()> {
        let registry = SnippetRegistry::new();
        let file_path = Path::new("extension/snippets/ruby.json");
        registry.register_snippets(
            file_path,
            r#"{"Hello": {"prefix": "hello", "body": "Hello ${1:world}"}}"#,
        )?;
        registry.register_snippets(
            file_path,
            r#"{"Hello": {"prefix": "hello", "body": "Hello ${1:Zed}"}}"#,
        )?;

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            1,
            "Registering the same file again should replace its snippets"
        );
        assert_eq!(snippets[0].body, "Hello ${1:Zed}");
        Ok(())
    }

    #[test]
    fn test_register_snippets_same_first_prefix_preserved() -> Result<()> {
        let registry = SnippetRegistry::new();
        registry.register_snippets(
            Path::new("first_extension/ruby.json"),
            r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
        )?;
        registry.register_snippets(
            Path::new("second_extension/ruby.json"),
            r#"{"Different Name": {"prefix": "for", "body": "${2:iter}.each do |${1:item}|\n$0\nend"}}"#,
        )?;

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            2,
            "Snippets sharing the same first prefix should both be preserved"
        );
        assert_eq!(snippets[0].name, "For Loop");
        assert_eq!(snippets[0].prefix, vec!["for".to_owned()]);
        assert_eq!(snippets[1].name, "Different Name");
        assert_eq!(snippets[1].prefix, vec!["for".to_owned()]);
        Ok(())
    }

    #[test]
    fn test_register_snippets_same_first_prefix_with_extra_prefixes_preserved() -> Result<()> {
        let registry = SnippetRegistry::new();
        registry.register_snippets(
            Path::new("first_extension/ruby.json"),
            r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
        )?;
        registry.register_snippets(
            Path::new("second_extension/ruby.json"),
            r#"{"For Loop Alt": {"prefix": ["for", "floop"], "body": "${2:iter}.each do |${1:item}|\n$0\nend"}}"#,
        )?;

        let snippets = registry.get_snippets(&Some("ruby".to_owned()));
        assert_eq!(
            snippets.len(),
            2,
            "Snippets sharing the same first prefix should be preserved along with their extra prefixes"
        );
        assert_eq!(snippets[0].name, "For Loop");
        assert_eq!(snippets[0].prefix, vec!["for".to_owned()]);
        assert_eq!(snippets[1].name, "For Loop Alt");
        assert_eq!(
            snippets[1].prefix,
            vec!["for".to_owned(), "floop".to_owned()]
        );
        Ok(())
    }

    #[test]
    fn test_register_snippets_different_first_prefixes_not_deduplicated() {
        let registry = SnippetRegistry::new();
        registry
            .register_snippets(
                Path::new("first_extension/ruby.json"),
                r#"{"For Loop": {"prefix": "for", "body": "for ${1:i} in ${2:iter} do\n$0\nend"}}"#,
            )
            .unwrap();
        registry
            .register_snippets(
                Path::new("second_extension/ruby.json"),
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
