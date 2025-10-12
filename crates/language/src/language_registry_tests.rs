use super::*;
use gpui::{App, TestAppContext};
use std::sync::Arc;

#[gpui::test]
async fn test_version_increments_on_language_add(cx: &mut TestAppContext) {
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    // Record initial version
    let initial_version = registry.version();

    // Create a simple language
    let language = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("TestLang"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["test".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));

    // Add language to registry - this should increment version
    registry.add(language.clone());

    // Version should increment after adding language
    let after_add_version = registry.version();
    assert!(
        after_add_version > initial_version,
        "Version should increment when language is added (initial: {}, after: {})",
        initial_version,
        after_add_version
    );
}

#[gpui::test]
async fn test_multiple_languages_increment_version(cx: &mut TestAppContext) {
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    let initial_version = registry.version();

    // Add first language
    let lang1 = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Language1"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["lang1".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));
    registry.add(lang1);
    let version_after_first = registry.version();

    // Add second language
    let lang2 = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Language2"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["lang2".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));
    registry.add(lang2);
    let version_after_second = registry.version();

    // Each addition should increment the version
    assert!(
        version_after_first > initial_version,
        "Version should increment after first language"
    );
    assert!(
        version_after_second > version_after_first,
        "Version should increment after second language"
    );
}

#[gpui::test]
fn test_language_registry_version_observable(cx: &mut App) {
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

    let initial_version = registry.version();

    // Add a language
    let language = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Test"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["test".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));
    registry.add(language);

    let new_version = registry.version();
    assert!(
        new_version > initial_version,
        "Version should be observable and increment"
    );
}

#[gpui::test]
async fn test_extension_grammar_injection_scenario(cx: &mut TestAppContext) {
    // This test simulates the exact scenario from the bug report:
    // 1. Extension with outer grammar loads first
    // 2. Inner grammar loads later (simulating async loading)
    // 3. Version increments with each load (the fix)
    // 4. This allows SyntaxMap to detect changes and resolve pending injections

    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    // Simulate markdown block grammar (outer) loading first
    let markdown_outer = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Markdown"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["md".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));
    registry.add(markdown_outer.clone());
    let version_after_outer = registry.version();

    // At this point, if markdown has injection queries for markdown-inline,
    // it would create a pending injection because markdown-inline isn't loaded yet

    // Simulate async loading of markdown-inline (inner) grammar
    // Before the fix: version wouldn't increment → injection stays pending
    // After the fix: version increments → SyntaxMap rechecks → injection resolves
    let markdown_inner = Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Markdown Inline"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["md-inline".to_string()],
                first_line_pattern: None,
            },
            ..Default::default()
        },
        None,
    ));

    // Add the inner language - with the fix, this MUST increment version
    registry.add(markdown_inner.clone());
    let version_after_inner = registry.version();

    // THE FIX: Version must increment so SyntaxMap can detect the change
    assert!(
        version_after_inner > version_after_outer,
        "Version MUST increment when extension grammar loads (outer: {}, inner: {}), \
         enabling SyntaxMap to resolve pending injections",
        version_after_outer,
        version_after_inner
    );

    // This version increment is what triggers SyntaxMap to recheck pending injections
    // Without it, injections remain pending forever and the injected grammar is never applied
}

#[gpui::test]
async fn test_version_monotonically_increases(cx: &mut TestAppContext) {
    // Verify that version always increases, never decreases or stays the same
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    let mut previous_version = registry.version();

    for i in 0..5 {
        let language = Arc::new(Language::new(
            LanguageConfig {
                name: LanguageName::new(&format!("TestLang{}", i)),
                matcher: LanguageMatcher {
                    path_suffixes: vec![format!("test{}", i)],
                    first_line_pattern: None,
                },
                ..Default::default()
            },
            None,
        ));

        registry.add(language);
        let current_version = registry.version();

        assert!(
            current_version > previous_version,
            "Version should monotonically increase (iteration {}: {} -> {})",
            i,
            previous_version,
            current_version
        );

        previous_version = current_version;
    }
}
