use crate::api::CloudflareIpCountryHeader;
use crate::{AppState, Error, Result};
use anyhow::anyhow;
use axum::{
    Extension, Router, TypedHeader,
    body::Bytes,
    headers::Header,
    http::{HeaderName, StatusCode},
    routing::post,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};
use telemetry_events::{Event, EventRequestBody};
use util::ResultExt;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/telemetry/events", post(post_events))
        .route("/telemetry/crashes", post(post_panic))
        .route("/telemetry/panics", post(post_panic))
        .route("/telemetry/hangs", post(post_panic))
}

pub struct ZedChecksumHeader(Vec<u8>);

impl Header for ZedChecksumHeader {
    fn name() -> &'static HeaderName {
        static ZED_CHECKSUM_HEADER: OnceLock<HeaderName> = OnceLock::new();
        ZED_CHECKSUM_HEADER.get_or_init(|| HeaderName::from_static("x-zed-checksum"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let checksum = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        let bytes = hex::decode(checksum).map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(bytes))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

pub async fn post_panic() -> Result<()> {
    // as of v0.201.x crash/panic reporting is now done via Sentry.
    // The endpoint returns OK to avoid spurious errors for old clients.
    Ok(())
}

pub async fn post_events(
    Extension(app): Extension<Arc<AppState>>,
    TypedHeader(ZedChecksumHeader(checksum)): TypedHeader<ZedChecksumHeader>,
    country_code_header: Option<TypedHeader<CloudflareIpCountryHeader>>,
    body: Bytes,
) -> Result<()> {
    let Some(expected) = calculate_json_checksum(app.clone(), &body) else {
        return Err(Error::http(
            StatusCode::INTERNAL_SERVER_ERROR,
            "events not enabled".into(),
        ))?;
    };

    let checksum_matched = checksum == expected;

    let request_body: telemetry_events::EventRequestBody =
        serde_json::from_slice(&body).map_err(|err| {
            log::error!("can't parse event json: {err}");
            Error::Internal(anyhow!(err))
        })?;

    let Some(last_event) = request_body.events.last() else {
        return Err(Error::http(StatusCode::BAD_REQUEST, "no events".into()))?;
    };
    let country_code = country_code_header.map(|h| h.to_string());

    let first_event_at = chrono::Utc::now()
        - chrono::Duration::milliseconds(last_event.milliseconds_since_first_event);

    if let Some(kinesis_client) = app.kinesis_client.clone()
        && let Some(stream) = app.config.kinesis_stream.clone()
    {
        let mut request = kinesis_client.put_records().stream_name(stream);
        let mut has_records = false;
        for row in for_snowflake(
            request_body.clone(),
            first_event_at,
            country_code.clone(),
            checksum_matched,
        ) {
            if let Some(data) = serde_json::to_vec(&row).log_err() {
                request = request.records(
                    aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                        .partition_key(request_body.system_id.clone().unwrap_or_default())
                        .data(data.into())
                        .build()
                        .unwrap(),
                );
                has_records = true;
            }
        }
        if has_records {
            request.send().await.log_err();
        }
    };

    Ok(())
}

pub fn calculate_json_checksum(app: Arc<AppState>, json: &impl AsRef<[u8]>) -> Option<Vec<u8>> {
    let checksum_seed = app.config.zed_client_checksum_seed.as_ref()?;

    let mut summer = Sha256::new();
    summer.update(checksum_seed);
    summer.update(json);
    summer.update(checksum_seed);
    Some(summer.finalize().into_iter().collect())
}

fn for_snowflake(
    body: EventRequestBody,
    first_event_at: chrono::DateTime<chrono::Utc>,
    country_code: Option<String>,
    checksum_matched: bool,
) -> impl Iterator<Item = SnowflakeRow> {
    body.events.into_iter().map(move |event| {
        let timestamp =
            first_event_at + Duration::milliseconds(event.milliseconds_since_first_event);
        let (event_type, mut event_properties) = match &event.event {
            Event::Flexible(e) => (
                e.event_type.clone(),
                serde_json::to_value(&e.event_properties).unwrap(),
            ),
        };

        if let serde_json::Value::Object(ref mut map) = event_properties {
            map.insert("app_version".to_string(), body.app_version.clone().into());
            map.insert("os_name".to_string(), body.os_name.clone().into());
            map.insert("os_version".to_string(), body.os_version.clone().into());
            map.insert("architecture".to_string(), body.architecture.clone().into());
            map.insert(
                "release_channel".to_string(),
                body.release_channel.clone().into(),
            );
            map.insert("signed_in".to_string(), event.signed_in.into());
            map.insert("checksum_matched".to_string(), checksum_matched.into());
            if let Some(country_code) = country_code.as_ref() {
                map.insert("country".to_string(), country_code.clone().into());
            }
        }

        // NOTE: most amplitude user properties are read out of our event_properties
        // dictionary. See https://app.amplitude.com/data/zed/Zed/sources/detail/production/falcon%3A159998
        // for how that is configured.
        let user_properties = body.is_staff.map(|is_staff| {
            serde_json::json!({
                "is_staff": is_staff,
            })
        });

        SnowflakeRow {
            time: timestamp,
            user_id: body.metrics_id.clone(),
            device_id: body.system_id.clone(),
            event_type,
            event_properties,
            user_properties,
            insert_id: Some(Uuid::new_v4().to_string()),
        }
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnowflakeRow {
    pub time: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    pub event_type: String,
    pub event_properties: serde_json::Value,
    pub user_properties: Option<serde_json::Value>,
    pub insert_id: Option<String>,
}

impl SnowflakeRow {
    pub fn new(
        event_type: impl Into<String>,
        metrics_id: Option<Uuid>,
        is_staff: bool,
        system_id: Option<String>,
        event_properties: serde_json::Value,
    ) -> Self {
        Self {
            time: chrono::Utc::now(),
            event_type: event_type.into(),
            device_id: system_id,
            user_id: metrics_id.map(|id| id.to_string()),
            insert_id: Some(uuid::Uuid::new_v4().to_string()),
            event_properties,
            user_properties: Some(json!({"is_staff": is_staff})),
        }
    }

    pub async fn write(
        self,
        client: &Option<aws_sdk_kinesis::Client>,
        stream: &Option<String>,
    ) -> anyhow::Result<()> {
        let Some((client, stream)) = client.as_ref().zip(stream.as_ref()) else {
            return Ok(());
        };
        let row = serde_json::to_vec(&self)?;
        client
            .put_record()
            .stream_name(stream)
            .partition_key(&self.user_id.unwrap_or_default())
            .data(row.into())
            .send()
            .await?;
        Ok(())
    }
}
