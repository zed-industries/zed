use super::Database;
use crate::{
    db::{ExtensionMetadata, NewExtensionVersion},
    test_both_dbs,
};
use std::sync::Arc;
use time::{OffsetDateTime, PrimitiveDateTime};

test_both_dbs!(
    test_extensions,
    test_extensions_postgres,
    test_extensions_sqlite
);

async fn test_extensions(db: &Arc<Database>) {
    let versions = db.get_known_extension_versions().await.unwrap();
    assert!(versions.is_empty());

    let extensions = db.get_extensions(None, 5).await.unwrap();
    assert!(extensions.is_empty());

    let t0 = OffsetDateTime::from_unix_timestamp_nanos(0).unwrap();
    let t0 = PrimitiveDateTime::new(t0.date(), t0.time());

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
                        published_at: t0,
                    },
                    NewExtensionVersion {
                        name: "Extension One".into(),
                        version: semver::Version::parse("0.0.2").unwrap(),
                        description: "a good extension".into(),
                        authors: vec!["max".into(), "marshall".into()],
                        repository: "ext1/repo".into(),
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
    let extensions = db.get_extensions(None, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext1".into(),
                name: "Extension One".into(),
                version: "0.0.2".into(),
                authors: vec!["max".into(), "marshall".into()],
                description: "a good extension".into(),
                repository: "ext1/repo".into(),
                published_at: t0,
                download_count: 0,
            },
            ExtensionMetadata {
                id: "ext2".into(),
                name: "Extension Two".into(),
                version: "0.2.0".into(),
                authors: vec!["marshall".into()],
                description: "a great extension".into(),
                repository: "ext2/repo".into(),
                published_at: t0,
                download_count: 0
            },
        ]
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
    assert!(!db
        .record_extension_download("no-such-extension", "0.0.2")
        .await
        .unwrap());

    // Extensions are returned in descending order of total downloads.
    let extensions = db.get_extensions(None, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext2".into(),
                name: "Extension Two".into(),
                version: "0.2.0".into(),
                authors: vec!["marshall".into()],
                description: "a great extension".into(),
                repository: "ext2/repo".into(),
                published_at: t0,
                download_count: 7
            },
            ExtensionMetadata {
                id: "ext1".into(),
                name: "Extension One".into(),
                version: "0.0.2".into(),
                authors: vec!["max".into(), "marshall".into()],
                description: "a good extension".into(),
                repository: "ext1/repo".into(),
                published_at: t0,
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

    let extensions = db.get_extensions(None, 5).await.unwrap();
    assert_eq!(
        extensions,
        &[
            ExtensionMetadata {
                id: "ext2".into(),
                name: "Extension Two".into(),
                version: "0.2.0".into(),
                authors: vec!["marshall".into()],
                description: "a great extension".into(),
                repository: "ext2/repo".into(),
                published_at: t0,
                download_count: 7
            },
            ExtensionMetadata {
                id: "ext1".into(),
                name: "Extension One".into(),
                version: "0.0.3".into(),
                authors: vec!["max".into(), "marshall".into()],
                description: "a real good extension".into(),
                repository: "ext1/repo".into(),
                published_at: t0,
                download_count: 5,
            },
        ]
    );
}
