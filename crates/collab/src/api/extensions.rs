use crate::db::ExtensionVersionConstraints;
use crate::{AppState, Error, Result, db::NewExtensionVersion};
use anyhow::Context as _;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    response::Redirect,
    routing::get,
};
use collections::{BTreeSet, HashMap};
use rpc::{ExtensionApiManifest, ExtensionProvides, GetExtensionsResponse};
use semver::Version as SemanticVersion;
use serde::Deserialize;
use std::str::FromStr;
use std::{sync::Arc, time::Duration};
use time::PrimitiveDateTime;
use util::{ResultExt, maybe};

pub fn router() -> Router {
    Router::new()
        .route("/extensions", get(get_extensions))
        .route("/extensions/updates", get(get_extension_updates))
        .route("/extensions/:extension_id", get(get_extension_versions))
        .route(
            "/extensions/:extension_id/download",
            get(download_latest_extension),
        )
        .route(
            "/extensions/:extension_id/:version/download",
            get(download_extension),
        )
}

#[derive(Debug, Deserialize)]
struct GetExtensionsParams {
    filter: Option<String>,
    /// A comma-delimited list of features that the extension must provide.
    ///
    /// For example:
    /// - `themes`
    /// - `themes,icon-themes`
    /// - `languages,language-servers`
    #[serde(default)]
    provides: Option<String>,
    #[serde(default)]
    max_schema_version: i32,
}

async fn get_extensions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionsParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let provides_filter = params.provides.map(|provides| {
        provides
            .split(',')
            .map(|value| value.trim())
            .filter_map(|value| ExtensionProvides::from_str(value).ok())
            .collect::<BTreeSet<_>>()
    });

    let mut extensions = app
        .db
        .get_extensions(
            params.filter.as_deref(),
            provides_filter.as_ref(),
            params.max_schema_version,
            1_000,
        )
        .await?;

    if let Some(filter) = params.filter.as_deref() {
        let extension_id = filter.to_lowercase();
        let mut exact_match = None;
        extensions.retain(|extension| {
            if extension.id.as_ref() == extension_id {
                exact_match = Some(extension.clone());
                false
            } else {
                true
            }
        });
        if exact_match.is_none() {
            exact_match = app
                .db
                .get_extensions_by_ids(&[&extension_id], None)
                .await?
                .first()
                .cloned();
        }

        if let Some(exact_match) = exact_match {
            extensions.insert(0, exact_match);
        }
    };

    if let Some(query) = params.filter.as_deref() {
        let count = extensions.len();
        tracing::info!(query, count, "extension_search")
    }

    Ok(Json(GetExtensionsResponse { data: extensions }))
}

#[derive(Debug, Deserialize)]
struct GetExtensionUpdatesParams {
    ids: String,
    min_schema_version: i32,
    max_schema_version: i32,
    min_wasm_api_version: semver::Version,
    max_wasm_api_version: semver::Version,
}

async fn get_extension_updates(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionUpdatesParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let constraints = ExtensionVersionConstraints {
        schema_versions: params.min_schema_version..=params.max_schema_version,
        wasm_api_versions: params.min_wasm_api_version..=params.max_wasm_api_version,
    };

    let extension_ids = params.ids.split(',').map(|s| s.trim()).collect::<Vec<_>>();

    let extensions = app
        .db
        .get_extensions_by_ids(&extension_ids, Some(&constraints))
        .await?;

    Ok(Json(GetExtensionsResponse { data: extensions }))
}

#[derive(Debug, Deserialize)]
struct GetExtensionVersionsParams {
    extension_id: String,
}

async fn get_extension_versions(
    Extension(app): Extension<Arc<AppState>>,
    Path(params): Path<GetExtensionVersionsParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let extension_versions = app.db.get_extension_versions(&params.extension_id).await?;

    Ok(Json(GetExtensionsResponse {
        data: extension_versions,
    }))
}

#[derive(Debug, Deserialize)]
struct DownloadLatestExtensionPathParams {
    extension_id: String,
}

#[derive(Debug, Deserialize)]
struct DownloadLatestExtensionQueryParams {
    min_schema_version: Option<i32>,
    max_schema_version: Option<i32>,
    min_wasm_api_version: Option<SemanticVersion>,
    max_wasm_api_version: Option<SemanticVersion>,
}

async fn download_latest_extension(
    Extension(app): Extension<Arc<AppState>>,
    Path(params): Path<DownloadLatestExtensionPathParams>,
    Query(query): Query<DownloadLatestExtensionQueryParams>,
) -> Result<Redirect> {
    let constraints = maybe!({
        let min_schema_version = query.min_schema_version?;
        let max_schema_version = query.max_schema_version?;
        let min_wasm_api_version = query.min_wasm_api_version?;
        let max_wasm_api_version = query.max_wasm_api_version?;

        Some(ExtensionVersionConstraints {
            schema_versions: min_schema_version..=max_schema_version,
            wasm_api_versions: min_wasm_api_version..=max_wasm_api_version,
        })
    });

    let extension = app
        .db
        .get_extension(&params.extension_id, constraints.as_ref())
        .await?
        .context("unknown extension")?;
    download_extension(
        Extension(app),
        Path(DownloadExtensionParams {
            extension_id: params.extension_id,
            version: extension.manifest.version.to_string(),
        }),
    )
    .await
}

#[derive(Debug, Deserialize)]
struct DownloadExtensionParams {
    extension_id: String,
    version: String,
}

async fn download_extension(
    Extension(app): Extension<Arc<AppState>>,
    Path(params): Path<DownloadExtensionParams>,
) -> Result<Redirect> {
    let Some((blob_store_client, bucket)) = app
        .blob_store_client
        .clone()
        .zip(app.config.blob_store_bucket.clone())
    else {
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let DownloadExtensionParams {
        extension_id,
        version,
    } = params;

    let version_exists = app
        .db
        .record_extension_download(&extension_id, &version)
        .await?;

    if !version_exists {
        Err(Error::http(
            StatusCode::NOT_FOUND,
            "unknown extension version".into(),
        ))?;
    }

    let url = blob_store_client
        .get_object()
        .bucket(bucket)
        .key(format!(
            "extensions/{extension_id}/{version}/archive.tar.gz"
        ))
        .presigned(PresigningConfig::expires_in(EXTENSION_DOWNLOAD_URL_LIFETIME).unwrap())
        .await
        .context("creating presigned extension download url")?;

    Ok(Redirect::temporary(url.uri()))
}

const EXTENSION_FETCH_INTERVAL: Duration = Duration::from_secs(5 * 60);
const EXTENSION_DOWNLOAD_URL_LIFETIME: Duration = Duration::from_secs(3 * 60);

pub fn fetch_extensions_from_blob_store_periodically(app_state: Arc<AppState>) {
    let Some(blob_store_client) = app_state.blob_store_client.clone() else {
        log::info!("no blob store client");
        return;
    };
    let Some(blob_store_bucket) = app_state.config.blob_store_bucket.clone() else {
        log::info!("no blob store bucket");
        return;
    };

    let executor = app_state.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                fetch_extensions_from_blob_store(
                    &blob_store_client,
                    &blob_store_bucket,
                    &app_state,
                )
                .await
                .log_err();
                executor.sleep(EXTENSION_FETCH_INTERVAL).await;
            }
        }
    });
}

async fn fetch_extensions_from_blob_store(
    blob_store_client: &aws_sdk_s3::Client,
    blob_store_bucket: &String,
    app_state: &Arc<AppState>,
) -> anyhow::Result<()> {
    log::info!("fetching extensions from blob store");

    let mut next_marker = None;
    let mut published_versions = HashMap::<String, Vec<String>>::default();

    loop {
        let list = blob_store_client
            .list_objects()
            .bucket(blob_store_bucket)
            .prefix("extensions/")
            .set_marker(next_marker.clone())
            .send()
            .await?;
        let objects = list.contents.unwrap_or_default();
        log::info!("fetched {} object(s) from blob store", objects.len());

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
            if parts.next() == Some("manifest.json") {
                published_versions
                    .entry(extension_id.to_owned())
                    .or_default()
                    .push(version.to_owned());
            }
        }

        if let (Some(true), Some(last_object)) = (list.is_truncated, objects.last()) {
            next_marker.clone_from(&last_object.key);
        } else {
            break;
        }
    }

    log::info!("found {} published extensions", published_versions.len());

    let known_versions = app_state.db.get_known_extension_versions().await?;

    let mut new_versions = HashMap::<&str, Vec<NewExtensionVersion>>::default();
    let empty = Vec::new();
    for (extension_id, published_versions) in &published_versions {
        let known_versions = known_versions.get(extension_id).unwrap_or(&empty);

        for published_version in published_versions {
            if known_versions
                .binary_search_by_key(&published_version, |known_version| known_version)
                .is_err()
                && let Some(extension) = fetch_extension_manifest(
                    blob_store_client,
                    blob_store_bucket,
                    extension_id,
                    published_version,
                )
                .await
                .log_err()
            {
                new_versions
                    .entry(extension_id)
                    .or_default()
                    .push(extension);
            }
        }
    }

    app_state
        .db
        .insert_extension_versions(&new_versions)
        .await?;

    log::info!(
        "fetched {} new extensions from blob store",
        new_versions.values().map(|v| v.len()).sum::<usize>()
    );

    Ok(())
}

async fn fetch_extension_manifest(
    blob_store_client: &aws_sdk_s3::Client,
    blob_store_bucket: &String,
    extension_id: &str,
    version: &str,
) -> anyhow::Result<NewExtensionVersion> {
    let object = blob_store_client
        .get_object()
        .bucket(blob_store_bucket)
        .key(format!("extensions/{extension_id}/{version}/manifest.json"))
        .send()
        .await?;
    let manifest_bytes = object
        .body
        .collect()
        .await
        .map(|data| data.into_bytes())
        .with_context(|| {
            format!("failed to download manifest for extension {extension_id} version {version}")
        })?
        .to_vec();
    let manifest =
        serde_json::from_slice::<ExtensionApiManifest>(&manifest_bytes).with_context(|| {
            format!(
                "invalid manifest for extension {extension_id} version {version}: {}",
                String::from_utf8_lossy(&manifest_bytes)
            )
        })?;
    let published_at = object.last_modified.with_context(|| {
        format!("missing last modified timestamp for extension {extension_id} version {version}")
    })?;
    let published_at = time::OffsetDateTime::from_unix_timestamp_nanos(published_at.as_nanos())?;
    let published_at = PrimitiveDateTime::new(published_at.date(), published_at.time());
    let version = semver::Version::parse(&manifest.version).with_context(|| {
        format!("invalid version for extension {extension_id} version {version}")
    })?;
    Ok(NewExtensionVersion {
        name: manifest.name,
        version,
        description: manifest.description.unwrap_or_default(),
        authors: manifest.authors,
        repository: manifest.repository,
        schema_version: manifest.schema_version.unwrap_or(0),
        wasm_api_version: manifest.wasm_api_version,
        provides: manifest.provides,
        published_at,
    })
}
