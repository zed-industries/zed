use std::collections::BTreeSet;
use std::sync::Arc;

use cloud_api_types::{ExtensionMetadata, ExtensionProvides};
use collab::db::Database;
use collab::db::ExtensionVersionConstraints;
use collab::db::{NewExtensionVersion, queries::extensions::convert_time_to_chrono};

use crate::test_both_dbs;

test_both_dbs!(
    test_extensions_by_id,
    test_extensions_by_id_postgres,
    test_extensions_by_id_sqlite
);

async fn test_extensions_by_id(db: &Arc<Database>) {
    let versions = db.get_known_extension_versions().await.unwrap();
    assert!(versions.is_empty());

    let t0 = time::OffsetDateTime::from_unix_timestamp_nanos(0).unwrap();
    let t0 = time::PrimitiveDateTime::new(t0.date(), t0.time());

    let t0_chrono = convert_time_to_chrono(t0);

    db.insert_extension_versions(
        &[
            (
                "ext1",
                vec![
                    NewExtensionVersion {
                        name: "Extension 1".into(),
                        version: semver::Version::parse("0.0.1").unwrap(),
                        description: "an extension".into(),
                        authors: vec!["max".into()],
                        repository: "ext1/repo".into(),
                        schema_version: 1,
                        wasm_api_version: Some("0.0.4".into()),
                        provides: BTreeSet::from_iter([
                            ExtensionProvides::Grammars,
                            ExtensionProvides::Languages,
                        ]),
                        published_at: t0,
                    },
                    NewExtensionVersion {
                        name: "Extension 1".into(),
                        version: semver::Version::parse("0.0.2").unwrap(),
                        description: "a good extension".into(),
                        authors: vec!["max".into()],
                        repository: "ext1/repo".into(),
                        schema_version: 1,
                        wasm_api_version: Some("0.0.4".into()),
                        provides: BTreeSet::from_iter([
                            ExtensionProvides::Grammars,
                            ExtensionProvides::Languages,
                            ExtensionProvides::LanguageServers,
                        ]),
                        published_at: t0,
                    },
                    NewExtensionVersion {
                        name: "Extension 1".into(),
                        version: semver::Version::parse("0.0.3").unwrap(),
                        description: "a real good extension".into(),
                        authors: vec!["max".into(), "marshall".into()],
                        repository: "ext1/repo".into(),
                        schema_version: 1,
                        wasm_api_version: Some("0.0.5".into()),
                        provides: BTreeSet::from_iter([
                            ExtensionProvides::Grammars,
                            ExtensionProvides::Languages,
                            ExtensionProvides::LanguageServers,
                        ]),
                        published_at: t0,
                    },
                ],
            ),
            (
                "ext2",
                vec![NewExtensionVersion {
                    name: "Extension 2".into(),
                    version: semver::Version::parse("0.2.0").unwrap(),
                    description: "a great extension".into(),
                    authors: vec!["marshall".into()],
                    repository: "ext2/repo".into(),
                    schema_version: 0,
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                    published_at: t0,
                }],
            ),
        ]
        .into_iter()
        .collect(),
    )
    .await
    .unwrap();

    let extensions = db
        .get_extensions_by_ids(
            &["ext1"],
            Some(&ExtensionVersionConstraints {
                schema_versions: 1..=1,
                wasm_api_versions: "0.0.1".parse().unwrap()..="0.0.4".parse().unwrap(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(
        extensions,
        &[ExtensionMetadata {
            id: "ext1".into(),
            manifest: cloud_api_types::ExtensionApiManifest {
                name: "Extension 1".into(),
                version: "0.0.2".into(),
                authors: vec!["max".into()],
                description: Some("a good extension".into()),
                repository: "ext1/repo".into(),
                schema_version: Some(1),
                wasm_api_version: Some("0.0.4".into()),
                provides: BTreeSet::from_iter([
                    ExtensionProvides::Grammars,
                    ExtensionProvides::Languages,
                    ExtensionProvides::LanguageServers,
                ]),
            },
            published_at: t0_chrono,
            download_count: 0,
        }]
    );
}
