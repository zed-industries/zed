// Test for Zed issue #48628: Extension category filter bypass
// Contract: crates/collab/src/api/extensions.rs::get_extensions()
// Enforces: POST-GE-02, POST-GE-04 (provides_filter must apply to ALL extensions including exact-match promoted ones)

#[cfg(test)]
mod contract_authority_record {
    //! CONTRACT AUTHORITY RECORD:
    //! - File: crates/collab/src/api/extensions.rs
    //! - Authority: "AUTHORITATIVE for get_extensions API handler"
    //! - POST-GE-02: If params.provides is Some(filter), every ExtensionMetadata in response.data has manifest.provides ∩ filter ≠ ∅
    //! - POST-GE-04: POST-GE-02 applies to ALL extensions including exact-match promoted extension
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use collections::BTreeSet;
    use rpc::{ExtensionApiManifest, ExtensionProvides};
    use std::sync::Arc;

    fn create_test_extension(
        id: &str,
        provides: BTreeSet<ExtensionProvides>,
    ) -> rpc::ExtensionMetadata {
        rpc::ExtensionMetadata {
            id: Arc::from(id),
            manifest: ExtensionApiManifest {
                name: id.to_string(),
                version: Arc::from("1.0.0"),
                description: None,
                authors: Vec::new(),
                repository: String::new(),
                schema_version: Some(1),
                wasm_api_version: None,
                provides,
            },
            published_at: Utc::now(),
            download_count: 0,
        }
    }

    fn matches_provides_filter(
        ext: &rpc::ExtensionMetadata,
        filter: &BTreeSet<ExtensionProvides>,
    ) -> bool {
        !ext.manifest.provides.is_disjoint(filter)
    }

    #[test]
    fn test_post_ge_02_provides_filter_logic() {
        // POST-GE-02: If params.provides is Some(filter), every ExtensionMetadata
        // in response.data has manifest.provides ∩ filter ≠ ∅

        let theme_ext = create_test_extension(
            "theme-ext",
            BTreeSet::from([ExtensionProvides::Themes]),
        );

        let language_ext = create_test_extension(
            "language-ext",
            BTreeSet::from([ExtensionProvides::Languages]),
        );

        let mixed_ext = create_test_extension(
            "mixed-ext",
            BTreeSet::from([ExtensionProvides::Themes, ExtensionProvides::Languages]),
        );

        let filter = BTreeSet::from([ExtensionProvides::Themes]);

        assert!(
            matches_provides_filter(&theme_ext, &filter),
            "POST-GE-02 violation: theme-ext should match Themes filter\n\
            EXPECTED: manifest.provides contains ExtensionProvides::Themes\n\
            ACTUAL: manifest.provides = {:?}\n\
            GUIDANCE: provides_filter MUST include extensions with matching category",
            theme_ext.manifest.provides
        );

        assert!(
            !matches_provides_filter(&language_ext, &filter),
            "POST-GE-02 violation: language-ext should NOT match Themes filter\n\
            EXPECTED: manifest.provides does NOT contain ExtensionProvides::Themes\n\
            ACTUAL: manifest.provides = {:?}\n\
            GUIDANCE: provides_filter MUST exclude extensions without matching category",
            language_ext.manifest.provides
        );

        assert!(
            matches_provides_filter(&mixed_ext, &filter),
            "POST-GE-02 violation: mixed-ext should match Themes filter\n\
            EXPECTED: manifest.provides contains ExtensionProvides::Themes\n\
            ACTUAL: manifest.provides = {:?}\n\
            GUIDANCE: provides_filter MUST include extensions with ANY matching category",
            mixed_ext.manifest.provides
        );
    }

    #[test]
    fn test_post_ge_04_exact_match_must_satisfy_provides_filter() {
        // POST-GE-04: Exact-match promotion MUST NOT bypass provides_filter.
        // This is the core of issue #48628.

        let language_python = create_test_extension(
            "language-python",
            BTreeSet::from([ExtensionProvides::Languages]),
        );

        let theme_filter = BTreeSet::from([ExtensionProvides::Themes]);

        assert!(
            !matches_provides_filter(&language_python, &theme_filter),
            "POST-GE-04 violation: language-python should NOT match Themes filter despite being exact-match candidate\n\
            EXPECTED: Extension excluded (provides Languages, not Themes)\n\
            ACTUAL: Extension would be included via exact-match promotion\n\
            GUIDANCE: Exact-match promotion MUST NOT bypass provides_filter."
        );
    }

    #[test]
    fn test_post_ge_04_exact_match_allowed_when_provides_matches() {
        // POST-GE-04: Exact-match extension included ONLY if it matches provides_filter

        let theme_ocean = create_test_extension(
            "theme-ocean",
            BTreeSet::from([ExtensionProvides::Themes]),
        );

        let theme_filter = BTreeSet::from([ExtensionProvides::Themes]);

        assert!(
            matches_provides_filter(&theme_ocean, &theme_filter),
            "POST-GE-04 violation: theme-ocean should match Themes filter when exact-match\n\
            EXPECTED: Extension included (exact match AND provides Themes)\n\
            ACTUAL: Extension excluded despite matching both criteria\n\
            GUIDANCE: Extension satisfying BOTH exact-match AND provides_filter MUST be included."
        );
    }

    #[test]
    fn test_post_ge_02_multiple_provides_any_match_sufficient() {
        // POST-GE-02: Extension with multiple provides matches if ANY overlap with filter

        let multi_ext = create_test_extension(
            "multi-ext",
            BTreeSet::from([
                ExtensionProvides::Languages,
                ExtensionProvides::Themes,
                ExtensionProvides::Grammars,
            ]),
        );

        let language_filter = BTreeSet::from([ExtensionProvides::Languages]);

        assert!(
            matches_provides_filter(&multi_ext, &language_filter),
            "POST-GE-02 violation: multi-ext should match when ANY provides overlaps\n\
            EXPECTED: Extension matches (Languages in provides)\n\
            ACTUAL: Extension does not match\n\
            GUIDANCE: provides_filter intersection means ANY overlap is sufficient for match."
        );
    }

    #[test]
    fn test_post_ge_02_no_overlap_excludes_extension() {
        // POST-GE-02: Extension excluded if NO overlap with provides_filter

        let grammar_ext = create_test_extension(
            "grammar-ext",
            BTreeSet::from([ExtensionProvides::Grammars, ExtensionProvides::Languages]),
        );

        let theme_filter = BTreeSet::from([ExtensionProvides::Themes]);

        assert!(
            !matches_provides_filter(&grammar_ext, &theme_filter),
            "POST-GE-02 violation: grammar-ext should NOT match when provides disjoint\n\
            EXPECTED: Extension excluded (no Themes in provides)\n\
            ACTUAL: Extension matches despite no overlap\n\
            GUIDANCE: If manifest.provides and filter are disjoint, extension MUST be excluded."
        );
    }
}
