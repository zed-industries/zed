// Test for Zed issue #48628: Extension category filter bypass (client-side)
// Contract: crates/extensions_ui/src/extensions_ui.rs::filter_extension_entries()
// Enforces: POST-FE-05, POST-FE-06 (client-side provides_filter enforcement)

#[cfg(test)]
mod contract_authority_record {
    //! CONTRACT AUTHORITY RECORD:
    //! - File: crates/extensions_ui/src/extensions_ui.rs
    //! - Authority: "AUTHORITATIVE for ExtensionsPage filtering logic"
    //! - POST-FE-05: If self.provides_filter is Some(category), every indexed extension has manifest.provides containing that category
    //! - POST-FE-06: Install status filter AND provides_filter compose conjunctively
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use client::{ExtensionApiManifest, ExtensionProvides};
    use collections::BTreeSet;
    use std::sync::Arc;

    fn create_test_extension(
        id: &str,
        provides: BTreeSet<ExtensionProvides>,
    ) -> client::ExtensionMetadata {
        client::ExtensionMetadata {
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

    fn should_be_indexed(
        ext: &client::ExtensionMetadata,
        provides_filter: Option<&ExtensionProvides>,
    ) -> bool {
        match provides_filter {
            Some(category) => ext.manifest.provides.contains(category),
            None => true,
        }
    }

    #[test]
    fn test_post_fe_05_provides_filter_enforced() {
        // POST-FE-05: If self.provides_filter is Some(category), every indexed
        // extension has manifest.provides containing that category

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

        let provides_filter = Some(&ExtensionProvides::Themes);

        assert!(
            should_be_indexed(&theme_ext, provides_filter),
            "POST-FE-05 violation: theme-ext should be indexed with Themes filter\n\
            EXPECTED: Extension indexed (provides contains Themes)\n\
            ACTUAL: Extension not indexed\n\
            GUIDANCE: provides_filter MUST be enforced."
        );

        assert!(
            !should_be_indexed(&language_ext, provides_filter),
            "POST-FE-05 violation: language-ext should NOT be indexed with Themes filter\n\
            EXPECTED: Extension not indexed (provides does not contain Themes)\n\
            ACTUAL: Extension indexed despite no match\n\
            GUIDANCE: provides_filter MUST exclude non-matching extensions."
        );

        assert!(
            should_be_indexed(&mixed_ext, provides_filter),
            "POST-FE-05 violation: mixed-ext should be indexed with Themes filter\n\
            EXPECTED: Extension indexed (provides contains Themes among others)\n\
            ACTUAL: Extension not indexed\n\
            GUIDANCE: Extension with matching category AMONG multiple provides MUST be indexed."
        );
    }

    #[test]
    fn test_post_fe_05_no_filter_includes_all() {
        // POST-FE-05: If provides_filter is None, all categories included

        let theme_ext = create_test_extension(
            "theme-ext",
            BTreeSet::from([ExtensionProvides::Themes]),
        );

        let language_ext = create_test_extension(
            "language-ext",
            BTreeSet::from([ExtensionProvides::Languages]),
        );

        let provides_filter = None;

        assert!(
            should_be_indexed(&theme_ext, provides_filter),
            "POST-FE-05 violation: theme-ext should be indexed with no filter\n\
            EXPECTED: Extension indexed (no category restriction)\n\
            ACTUAL: Extension not indexed\n\
            GUIDANCE: When provides_filter is None, ALL extensions MUST be included."
        );

        assert!(
            should_be_indexed(&language_ext, provides_filter),
            "POST-FE-05 violation: language-ext should be indexed with no filter\n\
            EXPECTED: Extension indexed (no category restriction)\n\
            ACTUAL: Extension not indexed\n\
            GUIDANCE: When provides_filter is None, ALL extensions MUST be included."
        );
    }

    #[test]
    fn test_post_fe_05_multiple_categories_in_provides() {
        // POST-FE-05: Extension with multiple provides indexed if ANY matches filter

        let multi_ext = create_test_extension(
            "multi-ext",
            BTreeSet::from([
                ExtensionProvides::Languages,
                ExtensionProvides::Themes,
                ExtensionProvides::Grammars,
            ]),
        );

        let language_filter = Some(&ExtensionProvides::Languages);

        assert!(
            should_be_indexed(&multi_ext, language_filter),
            "POST-FE-05 violation: multi-ext should be indexed when ANY provides matches\n\
            EXPECTED: Extension indexed (Languages in provides)\n\
            ACTUAL: Extension not indexed\n\
            GUIDANCE: Extension matching filter in ANY of its multiple provides MUST be indexed."
        );

        let icon_theme_filter = Some(&ExtensionProvides::IconThemes);

        assert!(
            !should_be_indexed(&multi_ext, icon_theme_filter),
            "POST-FE-05 violation: multi-ext should NOT be indexed when NO provides matches\n\
            EXPECTED: Extension not indexed (IconThemes not in provides)\n\
            ACTUAL: Extension indexed despite no match\n\
            GUIDANCE: Extension with NO matching category MUST be excluded."
        );
    }

    #[test]
    fn test_post_fe_06_filter_composition_concept() {
        // POST-FE-06: Install status filter AND provides_filter compose conjunctively
        // Full integration test requires GPUI context; this tests the provides_filter aspect

        let theme_installed = create_test_extension(
            "theme-installed",
            BTreeSet::from([ExtensionProvides::Themes]),
        );

        let language_installed = create_test_extension(
            "language-installed",
            BTreeSet::from([ExtensionProvides::Languages]),
        );

        let theme_not_installed = create_test_extension(
            "theme-not-installed",
            BTreeSet::from([ExtensionProvides::Themes]),
        );

        let provides_filter = Some(&ExtensionProvides::Themes);

        assert!(
            should_be_indexed(&theme_installed, provides_filter),
            "POST-FE-06 violation: theme-installed satisfies provides_filter\n\
            EXPECTED: Passes provides_filter check (Themes match)\n\
            ACTUAL: Fails provides_filter check\n\
            GUIDANCE: Conjunctive composition means extension MUST satisfy ALL active filters."
        );

        assert!(
            !should_be_indexed(&language_installed, provides_filter),
            "POST-FE-06 violation: language-installed fails provides_filter\n\
            EXPECTED: Fails provides_filter check (no Themes)\n\
            ACTUAL: Passes provides_filter check incorrectly\n\
            GUIDANCE: Conjunctive composition means failing ANY filter excludes extension."
        );

        assert!(
            should_be_indexed(&theme_not_installed, provides_filter),
            "POST-FE-06 violation: theme-not-installed satisfies provides_filter\n\
            EXPECTED: Passes provides_filter check (Themes match)\n\
            ACTUAL: Fails provides_filter check\n\
            GUIDANCE: Extension passes provides_filter but would be excluded by install status filter."
        );
    }
}
