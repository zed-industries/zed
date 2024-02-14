use crate::{db::NewExtensionVersion, executor::Executor, AppState, Result};
use anyhow::{anyhow, Context as _};
use axum::{extract::Query, routing::get, Extension, Json, Router};
use collections::{HashMap, HashSet};
use serde::{ser::Error as _, Deserialize, Serialize, Serializer};
use std::{sync::Arc, time::Duration};
use time::{
    format_description::well_known::{
        iso8601::{Config, EncodedConfig, TimePrecision},
        Iso8601,
    },
    OffsetDateTime, PrimitiveDateTime,
};
use util::ResultExt;

pub fn router() -> Router {
    Router::new().route("/extensions", get(get_extensions))
}

#[derive(Debug, Serialize)]
struct ExtensionJson {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub repository: String,
    #[serde(serialize_with = "serialize_iso8601")]
    pub published_at: OffsetDateTime,
}

const SERDE_CONFIG: EncodedConfig = Config::DEFAULT
    .set_year_is_six_digits(false)
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();

pub fn serialize_iso8601<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime
        .format(&Iso8601::<SERDE_CONFIG>)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

#[derive(Debug, Deserialize)]
struct GetExtensionsParams {
    filter: Option<String>,
}

#[derive(Debug, Serialize)]
struct GetExtensionsResponse {
    pub data: Vec<ExtensionJson>,
}

#[derive(Deserialize)]
struct ExtensionManifest {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    repository: String,
}

async fn get_extensions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionsParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let extensions = app.db.get_extensions(params.filter.as_deref(), 30).await?;

    Ok(Json(GetExtensionsResponse {
        data: extensions
            .into_iter()
            .map(|(extension, version)| ExtensionJson {
                id: extension.external_id,
                name: extension.name,
                version: version.version,
                authors: version
                    .authors
                    .split(',')
                    .map(|author| author.trim().to_string())
                    .collect::<Vec<_>>(),
                repository: version.repository,
                published_at: version.published_at.assume_utc(),
            })
            .collect(),
    }))
}

const EXTENSION_FETCH_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub fn fetch_extensions_periodically(app_state: Arc<AppState>, executor: Executor) {
    let Some(blob_store_client) = app_state.blob_store_client.clone() else {
        return;
    };
    let Some(blob_store_bucket) = app_state.config.blob_store_bucket.clone() else {
        return;
    };

    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                fetch_extensions(&blob_store_client, &blob_store_bucket, &app_state)
                    .await
                    .log_err();
                executor.sleep(EXTENSION_FETCH_INTERVAL).await;
            }
        }
    });
}

async fn fetch_extensions(
    blob_store_client: &aws_sdk_s3::Client,
    blob_store_bucket: &String,
    app_state: &Arc<AppState>,
) -> anyhow::Result<()> {
    let list = blob_store_client
        .list_objects()
        .bucket(blob_store_bucket)
        .prefix("extensions/")
        .send()
        .await?;

    let objects = list
        .contents
        .ok_or_else(|| anyhow!("missing bucket contents"))?;

    let mut published_versions = HashMap::<&str, Vec<&str>>::default();
    for object in &objects {
        let Some(key) = object.key.as_ref() else {
            continue;
        };
        let mut parts = key.split('/');
        let Some(_) = parts.next().filter(|part| *part == "extensions") else {
            continue;
        };
        let Some(extension_id) = parts.next() else {
            continue;
        };
        let Some(version) = parts.next() else {
            continue;
        };
        published_versions
            .entry(extension_id)
            .or_default()
            .push(version);
    }

    let known_versions = app_state.db.get_known_extension_versions().await?;

    let mut new_versions = HashMap::<&str, Vec<NewExtensionVersion>>::default();
    let empty = HashSet::default();
    for (extension_id, published_versions) in published_versions {
        let known_versions = known_versions.get(extension_id).unwrap_or(&empty);

        for published_version in published_versions {
            if !known_versions.contains(published_version) {
                let object = blob_store_client
                    .get_object()
                    .bucket(blob_store_bucket)
                    .key(format!(
                        "extensions/{extension_id}/{published_version}/manifest.json"
                    ))
                    .send()
                    .await?;
                let manifest_bytes = object
                    .body
                    .collect()
                    .await
                    .map(|data| data.into_bytes())
                    .with_context(|| format!("failed to download manifest for extension {extension_id} version {published_version}"))?
                    .to_vec();
                let manifest = serde_json::from_slice::<ExtensionManifest>(&manifest_bytes)
                    .with_context(|| format!("invalid manifest for extension {extension_id} version {published_version}"))?;

                let published_at = object.last_modified.ok_or_else(|| anyhow!("missing last modified timestamp for extension {extension_id} version {published_version}"))?;
                let published_at =
                    time::OffsetDateTime::from_unix_timestamp_nanos(published_at.as_nanos())?;
                let published_at = PrimitiveDateTime::new(published_at.date(), published_at.time());

                let version = semver::Version::parse(&manifest.version).with_context(|| {
                    format!(
                        "invalid version for extension {extension_id} version {published_version}"
                    )
                })?;

                new_versions
                    .entry(extension_id)
                    .or_default()
                    .push(NewExtensionVersion {
                        name: manifest.name,
                        version,
                        description: manifest.description,
                        authors: manifest.authors,
                        repository: manifest.repository,
                        published_at,
                    });
            }
        }
    }

    app_state
        .db
        .insert_extension_versions(&new_versions)
        .await?;

    Ok(())
}
