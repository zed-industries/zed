use std::collections::BTreeSet;
use std::sync::Arc;

use rpc::ExtensionProvides;

use crate::test_both_dbs;
use collab::db::Database;
use collab::db::ExtensionVersionConstraints;
use collab::db::{NewExtensionVersion, queries::extensions::convert_time_to_chrono};
use rpc::ExtensionMetadata;
test_both_dbs!(
    test_extensions,
    test_extensions_postgres,
    test_extensions_sqlite
);

test_both_dbs!(
    test_agent_servers_filter,
    test_agent_servers_filter_postgres,
    test_agent_servers_filter_sqlite
);

async fn test_agent_servers_filter(db: &Arc<Database>) {
    // No extensions initially
    let versions = db.get_known_extension_versions().await.unwrap();
    assert!(versions.is_empty());

    // Shared timestamp
    let t0 = time::OffsetDateTime::from_unix_timestamp_nanos(0).unwrap();
    let t0 = time::PrimitiveDateTime::new(t0.date(), t0.time());

    // Insert two extensions, only one provides AgentServers
    db.insert_extension_versions(
        &[
            (
                "ext_agent_servers",
                vec![NewExtensionVersion {
                    name: "Agent Servers Provider".into(),
                    version: semver::Version::parse("1.0.0").unwrap(),
                    description: "has agent servers".into(),
                    authors: vec!["author".into()],
                    repository: "org/agent-servers".into(),
                    schema_version: 1,
                    wasm_api_version: None,
                    provides: BTreeSet::from_iter([ExtensionProvides::AgentServers]),
                    published_at: t0,
                }],
            ),
            (
                "ext_plain",
                vec![NewExtensionVersion {
                    name: "Plain Extension".into(),
                    version: semver::Version::parse("0.1.0").unwrap(),
                    description: "no agent servers".into(),
                    authors: vec!["author2".into()],
                    repository: "org/plain".into(),
                    schema_version: 1,
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

    // Filter by AgentServers provides
    let provides_filter = BTreeSet::from_iter([ExtensionProvides::AgentServers]);

    let filtered = db
        .get_extensions(None, Some(&provides_filter), 1, 10)
        .await
        .unwrap();

    // Expect only the extension that declared AgentServers
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id.as_ref(), "ext_agent_servers");
}

async fn test_extensions(db: &Arc<Database>) {
    let versions = db.get_known_extension_versions().await.unwrap();
    assert!(versions.is_empty());

    let extensions = db.get_extensions(None, None, 1, 5).await.unwrap();
    assert!(extensions.is_empty());

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
                        wasm_api_version: None,
                        provides: BTreeSet::default(),
                        published_at: t0,
                    },
                    NewExtensionVersion {
                        name: "Extension One".into(),
                        version: semver::Version::parse("0.0.2").unwrap(),
                        description: "a good extension".into(),
                        authors: vec!["max".into(), "marshall".into()],
                        repository: "ext1/repo".into(),
                        schema_version: 1,
                        wasm_api_version: None,
                        provides: BTreeSet::default(),
                        published_at: t0,
                    },
                ],
            ),
            (
                "ext2",
                vec![NewExtensionVersion {
                    name: "Extension Two".into(),
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

    let versions = db.get_known_extension_versions().await.unwrap();
    assert_eq!(
        versions,
        [
            ("ext1".into(), vec!["0.0.1".into(), "0.0.2".into()]),
            ("ext2".into(), vec!["0.2.0".into()])
        ]
        .into_iter()
        .collect()
    );

    // The latest version of each extension is returned.
    let extensions = db.get_extensions(None, None, 1, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext1".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension One".into(),
                    version: "0.0.2".into(),
                    authors: vec!["max".into(), "marshall".into()],
                    description: Some("a good extension".into()),
                    repository: "ext1/repo".into(),
                    schema_version: Some(1),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 0,
            },
            ExtensionMetadata {
                id: "ext2".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension Two".into(),
                    version: "0.2.0".into(),
                    authors: vec!["marshall".into()],
                    description: Some("a great extension".into()),
                    repository: "ext2/repo".into(),
                    schema_version: Some(0),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 0
            },
        ]
    );

    // Extensions with too new of a schema version are excluded.
    let extensions = db.get_extensions(None, None, 0, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[ExtensionMetadata {
            id: "ext2".into(),
            manifest: rpc::ExtensionApiManifest {
                name: "Extension Two".into(),
                version: "0.2.0".into(),
                authors: vec!["marshall".into()],
                description: Some("a great extension".into()),
                repository: "ext2/repo".into(),
                schema_version: Some(0),
                wasm_api_version: None,
                provides: BTreeSet::default(),
            },
            published_at: t0_chrono,
            download_count: 0
        },]
    );

    // Record extensions being downloaded.
    for _ in 0..7 {
        assert!(db.record_extension_download("ext2", "0.0.2").await.unwrap());
    }

    for _ in 0..3 {
        assert!(db.record_extension_download("ext1", "0.0.1").await.unwrap());
    }

    for _ in 0..2 {
        assert!(db.record_extension_download("ext1", "0.0.2").await.unwrap());
    }

    // Record download returns false if the extension does not exist.
    assert!(
        !db.record_extension_download("no-such-extension", "0.0.2")
            .await
            .unwrap()
    );

    // Extensions are returned in descending order of total downloads.
    let extensions = db.get_extensions(None, None, 1, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext2".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension Two".into(),
                    version: "0.2.0".into(),
                    authors: vec!["marshall".into()],
                    description: Some("a great extension".into()),
                    repository: "ext2/repo".into(),
                    schema_version: Some(0),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 7
            },
            ExtensionMetadata {
                id: "ext1".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension One".into(),
                    version: "0.0.2".into(),
                    authors: vec!["max".into(), "marshall".into()],
                    description: Some("a good extension".into()),
                    repository: "ext1/repo".into(),
                    schema_version: Some(1),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 5,
            },
        ]
    );

    // Add more extensions, including a new version of `ext1`, and backfilling
    // an older version of `ext2`.
    db.insert_extension_versions(
        &[
            (
                "ext1",
                vec![NewExtensionVersion {
                    name: "Extension One".into(),
                    version: semver::Version::parse("0.0.3").unwrap(),
                    description: "a real good extension".into(),
                    authors: vec!["max".into(), "marshall".into()],
                    repository: "ext1/repo".into(),
                    schema_version: 1,
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                    published_at: t0,
                }],
            ),
            (
                "ext2",
                vec![NewExtensionVersion {
                    name: "Extension Two".into(),
                    version: semver::Version::parse("0.1.0").unwrap(),
                    description: "an old extension".into(),
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

    let versions = db.get_known_extension_versions().await.unwrap();
    assert_eq!(
        versions,
        [
            (
                "ext1".into(),
                vec!["0.0.1".into(), "0.0.2".into(), "0.0.3".into()]
            ),
            ("ext2".into(), vec!["0.1.0".into(), "0.2.0".into()])
        ]
        .into_iter()
        .collect()
    );

    let extensions = db.get_extensions(None, None, 1, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext2".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension Two".into(),
                    version: "0.2.0".into(),
                    authors: vec!["marshall".into()],
                    description: Some("a great extension".into()),
                    repository: "ext2/repo".into(),
                    schema_version: Some(0),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 7
            },
            ExtensionMetadata {
                id: "ext1".into(),
                manifest: rpc::ExtensionApiManifest {
                    name: "Extension One".into(),
                    version: "0.0.3".into(),
                    authors: vec!["max".into(), "marshall".into()],
                    description: Some("a real good extension".into()),
                    repository: "ext1/repo".into(),
                    schema_version: Some(1),
                    wasm_api_version: None,
                    provides: BTreeSet::default(),
                },
                published_at: t0_chrono,
                download_count: 5,
            },
        ]
    );
}

test_both_dbs!(
    test_extensions_by_id,
    test_extensions_by_id_postgres,
    test_extensions_by_id_sqlite
);

async fn test_extensions_by_id(db: &Arc<Database>) {
    let versions = db.get_known_extension_versions().await.unwrap();
    assert!(versions.is_empty());

    let extensions = db.get_extensions(None, None, 1, 5).await.unwrap();
    assert!(extensions.is_empty());

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
            manifest: rpc::ExtensionApiManifest {
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
