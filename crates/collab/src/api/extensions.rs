use crate::{
    db::{ExtensionMetadata, NewExtensionVersion},
    executor::Executor,
    AppState, Error, Result,
};
use anyhow::{anyhow, Context as _};
use aws_sdk_s3::presigning::PresigningConfig;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Redirect,
    routing::get,
    Extension, Json, Router,
};
use collections::HashMap;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use time::PrimitiveDateTime;
use util::ResultExt;

pub fn router() -> Router {
    Router::new()
        .route("/extensions", get(get_extensions))
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
}

#[derive(Debug, Deserialize)]
struct DownloadLatestExtensionParams {
    extension_id: String,
}

#[derive(Debug, Deserialize)]
struct DownloadExtensionParams {
    extension_id: String,
    version: String,
}

#[derive(Debug, Serialize)]
struct GetExtensionsResponse {
    pub data: Vec<ExtensionMetadata>,
}

#[derive(Deserialize)]
struct ExtensionManifest {
    name: String,
    version: String,
    description: Option<String>,
    authors: Vec<String>,
    repository: String,
}

async fn get_extensions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionsParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let extensions = app.db.get_extensions(params.filter.as_deref(), 500).await?;
    Ok(Json(GetExtensionsResponse { data: extensions }))
}

async fn download_latest_extension(
    Extension(app): Extension<Arc<AppState>>,
    Path(params): Path<DownloadLatestExtensionParams>,
) -> Result<Redirect> {
    let extension = app
        .db
        .get_extension(&params.extension_id)
        .await?
        .ok_or_else(|| anyhow!("unknown extension"))?;
    download_extension(
        Extension(app),
        Path(DownloadExtensionParams {
            extension_id: params.extension_id,
            version: extension.version,
        }),
    )
    .await
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
        Err(Error::Http(
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
        Err(Error::Http(
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
        .map_err(|e| anyhow!("failed to create presigned extension download url {e}"))?;

    Ok(Redirect::temporary(url.uri()))
}

const EXTENSION_FETCH_INTERVAL: Duration = Duration::from_secs(5 * 60);
const EXTENSION_DOWNLOAD_URL_LIFETIME: Duration = Duration::from_secs(3 * 60);

pub fn fetch_extensions_from_blob_store_periodically(app_state: Arc<AppState>, executor: Executor) {
    let Some(blob_store_client) = app_state.blob_store_client.clone() else {
        log::info!("no blob store client");
        return;
    };
    let Some(blob_store_bucket) = app_state.config.blob_store_bucket.clone() else {
        log::info!("no blob store bucket");
        return;
    };

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

    let list = blob_store_client
        .list_objects()
        .bucket(blob_store_bucket)
        .prefix("extensions/")
        .send()
        .await?;

    let objects = list.contents.unwrap_or_default();

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
        if parts.next() == Some("manifest.json") {
            published_versions
                .entry(extension_id)
                .or_default()
                .push(version);
        }
    }

    let known_versions = app_state.db.get_known_extension_versions().await?;

    let mut new_versions = HashMap::<&str, Vec<NewExtensionVersion>>::default();
    let empty = Vec::new();
    for (extension_id, published_versions) in published_versions {
        let known_versions = known_versions.get(extension_id).unwrap_or(&empty);

        for published_version in published_versions {
            if known_versions
                .binary_search_by_key(&published_version, String::as_str)
                .is_err()
            {
                if let Some(extension) = fetch_extension_manifest(
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
) -> Result<NewExtensionVersion, anyhow::Error> {
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
        serde_json::from_slice::<ExtensionManifest>(&manifest_bytes).with_context(|| {
            format!(
                "invalid manifest for extension {extension_id} version {version}: {}",
                String::from_utf8_lossy(&manifest_bytes)
            )
        })?;
    let published_at = object.last_modified.ok_or_else(|| {
        anyhow!("missing last modified timestamp for extension {extension_id} version {version}")
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
        published_at,
    })
}
