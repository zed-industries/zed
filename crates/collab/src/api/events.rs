use super::ips_file::IpsFile;
use crate::api::CloudflareIpCountryHeader;
use crate::{AppState, Error, Result, api::slack};
use anyhow::anyhow;
use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Extension, Router, TypedHeader,
    body::Bytes,
    headers::Header,
    http::{HeaderMap, HeaderName, StatusCode},
    routing::post,
};
use chrono::Duration;
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};
use telemetry_events::{Event, EventRequestBody, Panic};
use util::ResultExt;
use uuid::Uuid;

const CRASH_REPORTS_BUCKET: &str = "zed-crash-reports";

pub fn router() -> Router {
    Router::new()
        .route("/telemetry/events", post(post_events))
        .route("/telemetry/crashes", post(post_crash))
        .route("/telemetry/panics", post(post_panic))
        .route("/telemetry/hangs", post(post_hang))
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

pub async fn post_crash(
    Extension(app): Extension<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<()> {
    let report = IpsFile::parse(&body)?;
    let version_threshold = SemanticVersion::new(0, 123, 0);

    let bundle_id = &report.header.bundle_id;
    let app_version = &report.app_version();

    if bundle_id == "dev.zed.Zed-Dev" {
        log::error!("Crash uploads from {} are ignored.", bundle_id);
        return Ok(());
    }

    if app_version.is_none() || app_version.unwrap() < version_threshold {
        log::error!(
            "Crash uploads from {} are ignored.",
            report.header.app_version
        );
        return Ok(());
    }
    let app_version = app_version.unwrap();

    if let Some(blob_store_client) = app.blob_store_client.as_ref() {
        let response = blob_store_client
            .head_object()
            .bucket(CRASH_REPORTS_BUCKET)
            .key(report.header.incident_id.clone() + ".ips")
            .send()
            .await;

        if response.is_ok() {
            log::info!("We've already uploaded this crash");
            return Ok(());
        }

        blob_store_client
            .put_object()
            .bucket(CRASH_REPORTS_BUCKET)
            .key(report.header.incident_id.clone() + ".ips")
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .body(ByteStream::from(body.to_vec()))
            .send()
            .await
            .map_err(|e| log::error!("Failed to upload crash: {}", e))
            .ok();
    }

    let recent_panic_on: Option<i64> = headers
        .get("x-zed-panicked-on")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok());

    let installation_id = headers
        .get("x-zed-installation-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let mut recent_panic = None;

    if let Some(recent_panic_on) = recent_panic_on {
        let crashed_at = match report.timestamp() {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("Can't parse {}: {}", report.header.timestamp, e);
                None
            }
        };
        if crashed_at.is_some_and(|t| (t.timestamp_millis() - recent_panic_on).abs() <= 30000) {
            recent_panic = headers.get("x-zed-panic").and_then(|h| h.to_str().ok());
        }
    }

    let description = report.description(recent_panic);
    let summary = report.backtrace_summary();

    tracing::error!(
        service = "client",
        version = %report.header.app_version,
        os_version = %report.header.os_version,
        bundle_id = %report.header.bundle_id,
        incident_id = %report.header.incident_id,
        installation_id = %installation_id,
        description = %description,
        backtrace = %summary,
        "crash report"
    );

    if let Some(kinesis_client) = app.kinesis_client.clone() {
        if let Some(stream) = app.config.kinesis_stream.clone() {
            let properties = json!({
                "app_version": report.header.app_version,
                "os_version": report.header.os_version,
                "os_name": "macOS",
                "bundle_id": report.header.bundle_id,
                "incident_id": report.header.incident_id,
                "installation_id": installation_id,
                "description": description,
                "backtrace": summary,
            });
            let row = SnowflakeRow::new(
                "Crash Reported",
                None,
                false,
                Some(installation_id),
                properties,
            );
            let data = serde_json::to_vec(&row)?;
            kinesis_client
                .put_record()
                .stream_name(stream)
                .partition_key(row.insert_id.unwrap_or_default())
                .data(data.into())
                .send()
                .await
                .log_err();
        }
    }

    if let Some(slack_panics_webhook) = app.config.slack_panics_webhook.clone() {
        let payload = slack::WebhookBody::new(|w| {
            w.add_section(|s| s.text(slack::Text::markdown(description)))
                .add_section(|s| {
                    s.add_field(slack::Text::markdown(format!(
                        "*Version:*\n{} ({})",
                        bundle_id, app_version
                    )))
                    .add_field({
                        let hostname = app.config.blob_store_url.clone().unwrap_or_default();
                        let hostname = hostname.strip_prefix("https://").unwrap_or_else(|| {
                            hostname.strip_prefix("http://").unwrap_or_default()
                        });

                        slack::Text::markdown(format!(
                            "*Incident:*\n<https://{}.{}/{}.ips|{}…>",
                            CRASH_REPORTS_BUCKET,
                            hostname,
                            report.header.incident_id,
                            report
                                .header
                                .incident_id
                                .chars()
                                .take(8)
                                .collect::<String>(),
                        ))
                    })
                })
                .add_rich_text(|r| r.add_preformatted(|p| p.add_text(summary)))
        });
        let payload_json = serde_json::to_string(&payload).map_err(|err| {
            log::error!("Failed to serialize payload to JSON: {err}");
            Error::Internal(anyhow!(err))
        })?;

        reqwest::Client::new()
            .post(slack_panics_webhook)
            .header("Content-Type", "application/json")
            .body(payload_json)
            .send()
            .await
            .map_err(|err| {
                log::error!("Failed to send payload to Slack: {err}");
                Error::Internal(anyhow!(err))
            })?;
    }

    Ok(())
}

pub async fn post_hang(
    Extension(app): Extension<Arc<AppState>>,
    TypedHeader(ZedChecksumHeader(checksum)): TypedHeader<ZedChecksumHeader>,
    body: Bytes,
) -> Result<()> {
    let Some(expected) = calculate_json_checksum(app.clone(), &body) else {
        return Err(Error::http(
            StatusCode::INTERNAL_SERVER_ERROR,
            "events not enabled".into(),
        ))?;
    };

    if checksum != expected {
        return Err(Error::http(
            StatusCode::BAD_REQUEST,
            "invalid checksum".into(),
        ))?;
    }

    let incident_id = Uuid::new_v4().to_string();

    // dump JSON into S3 so we can get frame offsets if we need to.
    if let Some(blob_store_client) = app.blob_store_client.as_ref() {
        blob_store_client
            .put_object()
            .bucket(CRASH_REPORTS_BUCKET)
            .key(incident_id.clone() + ".hang.json")
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .body(ByteStream::from(body.to_vec()))
            .send()
            .await
            .map_err(|e| log::error!("Failed to upload crash: {}", e))
            .ok();
    }

    let report: telemetry_events::HangReport = serde_json::from_slice(&body).map_err(|err| {
        log::error!("can't parse report json: {err}");
        Error::Internal(anyhow!(err))
    })?;

    let mut backtrace = "Possible hang detected on main thread:".to_string();
    let unknown = "<unknown>".to_string();
    for frame in report.backtrace.iter() {
        backtrace.push_str(&format!("\n{}", frame.symbols.first().unwrap_or(&unknown)));
    }

    tracing::error!(
        service = "client",
        version = %report.app_version.unwrap_or_default().to_string(),
        os_name = %report.os_name,
        os_version = report.os_version.unwrap_or_default().to_string(),
        incident_id = %incident_id,
        installation_id = %report.installation_id.unwrap_or_default(),
        backtrace = %backtrace,
        "hang report");

    Ok(())
}

pub async fn post_panic(
    Extension(app): Extension<Arc<AppState>>,
    TypedHeader(ZedChecksumHeader(checksum)): TypedHeader<ZedChecksumHeader>,
    body: Bytes,
) -> Result<()> {
    let Some(expected) = calculate_json_checksum(app.clone(), &body) else {
        return Err(Error::http(
            StatusCode::INTERNAL_SERVER_ERROR,
            "events not enabled".into(),
        ))?;
    };

    if checksum != expected {
        return Err(Error::http(
            StatusCode::BAD_REQUEST,
            "invalid checksum".into(),
        ))?;
    }

    let report: telemetry_events::PanicRequest = serde_json::from_slice(&body)
        .map_err(|_| Error::http(StatusCode::BAD_REQUEST, "invalid json".into()))?;
    let incident_id = uuid::Uuid::new_v4().to_string();
    let panic = report.panic;

    if panic.os_name == "Linux" && panic.os_version == Some("1.0.0".to_string()) {
        return Err(Error::http(
            StatusCode::BAD_REQUEST,
            "invalid os version".into(),
        ))?;
    }

    if let Some(blob_store_client) = app.blob_store_client.as_ref() {
        let response = blob_store_client
            .head_object()
            .bucket(CRASH_REPORTS_BUCKET)
            .key(incident_id.clone() + ".json")
            .send()
            .await;

        if response.is_ok() {
            log::info!("We've already uploaded this crash");
            return Ok(());
        }

        blob_store_client
            .put_object()
            .bucket(CRASH_REPORTS_BUCKET)
            .key(incident_id.clone() + ".json")
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .body(ByteStream::from(body.to_vec()))
            .send()
            .await
            .map_err(|e| log::error!("Failed to upload crash: {}", e))
            .ok();
    }

    let backtrace = panic.backtrace.join("\n");

    tracing::error!(
        service = "client",
        version = %panic.app_version,
        os_name = %panic.os_name,
        os_version = %panic.os_version.clone().unwrap_or_default(),
        incident_id = %incident_id,
        installation_id = %panic.installation_id.clone().unwrap_or_default(),
        description = %panic.payload,
        backtrace = %backtrace,
        "panic report"
    );

    if let Some(kinesis_client) = app.kinesis_client.clone() {
        if let Some(stream) = app.config.kinesis_stream.clone() {
            let properties = json!({
                "app_version": panic.app_version,
                "os_name": panic.os_name,
                "os_version": panic.os_version,
                "incident_id": incident_id,
                "installation_id": panic.installation_id,
                "description": panic.payload,
                "backtrace": backtrace,
            });
            let row = SnowflakeRow::new(
                "Panic Reported",
                None,
                false,
                panic.installation_id.clone(),
                properties,
            );
            let data = serde_json::to_vec(&row)?;
            kinesis_client
                .put_record()
                .stream_name(stream)
                .partition_key(row.insert_id.unwrap_or_default())
                .data(data.into())
                .send()
                .await
                .log_err();
        }
    }

    let backtrace = if panic.backtrace.len() > 25 {
        let total = panic.backtrace.len();
        format!(
            "{}\n   and {} more",
            panic
                .backtrace
                .iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
            total - 20
        )
    } else {
        panic.backtrace.join("\n")
    };

    if !report_to_slack(&panic) {
        return Ok(());
    }

    let backtrace_with_summary = panic.payload + "\n" + &backtrace;

    if let Some(slack_panics_webhook) = app.config.slack_panics_webhook.clone() {
        let payload = slack::WebhookBody::new(|w| {
            w.add_section(|s| s.text(slack::Text::markdown("Panic request".to_string())))
                .add_section(|s| {
                    s.add_field(slack::Text::markdown(format!(
                        "*Version:*\n {} ",
                        panic.app_version
                    )))
                    .add_field({
                        let hostname = app.config.blob_store_url.clone().unwrap_or_default();
                        let hostname = hostname.strip_prefix("https://").unwrap_or_else(|| {
                            hostname.strip_prefix("http://").unwrap_or_default()
                        });

                        slack::Text::markdown(format!(
                            "*{} {}:*\n<https://{}.{}/{}.json|{}…>",
                            panic.os_name,
                            panic.os_version.unwrap_or_default(),
                            CRASH_REPORTS_BUCKET,
                            hostname,
                            incident_id,
                            incident_id.chars().take(8).collect::<String>(),
                        ))
                    })
                })
                .add_rich_text(|r| r.add_preformatted(|p| p.add_text(backtrace_with_summary)))
        });
        let payload_json = serde_json::to_string(&payload).map_err(|err| {
            log::error!("Failed to serialize payload to JSON: {err}");
            Error::Internal(anyhow!(err))
        })?;

        reqwest::Client::new()
            .post(slack_panics_webhook)
            .header("Content-Type", "application/json")
            .body(payload_json)
            .send()
            .await
            .map_err(|err| {
                log::error!("Failed to send payload to Slack: {err}");
                Error::Internal(anyhow!(err))
            })?;
    }

    Ok(())
}

fn report_to_slack(panic: &Panic) -> bool {
    // Panics on macOS should make their way to Slack as a crash report,
    // so we don't need to send them a second time via this channel.
    if panic.os_name == "macOS" {
        return false;
    }

    if panic.payload.contains("ERROR_SURFACE_LOST_KHR") {
        return false;
    }

    if panic.payload.contains("ERROR_INITIALIZATION_FAILED") {
        return false;
    }

    if panic
        .payload
        .contains("GPU has crashed, and no debug information is available")
    {
        return false;
    }

    true
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

    if let Some(kinesis_client) = app.kinesis_client.clone() {
        if let Some(stream) = app.config.kinesis_stream.clone() {
            let mut request = kinesis_client.put_records().stream_name(stream);
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
                }
            }
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
    body.events.into_iter().flat_map(move |event| {
        let timestamp =
            first_event_at + Duration::milliseconds(event.milliseconds_since_first_event);
        // We will need to double check, but I believe all of the events that
        // are being transformed here are now migrated over to use the
        // telemetry::event! macro, as of this commit so this code can go away
        // when we feel enough users have upgraded past this point.
        let (event_type, mut event_properties) = match &event.event {
            Event::Editor(e) => (
                match e.operation.as_str() {
                    "open" => "Editor Opened".to_string(),
                    "save" => "Editor Saved".to_string(),
                    _ => format!("Unknown Editor Event: {}", e.operation),
                },
                serde_json::to_value(e).unwrap(),
            ),
            Event::InlineCompletion(e) => (
                format!(
                    "Edit Prediction {}",
                    if e.suggestion_accepted {
                        "Accepted"
                    } else {
                        "Discarded"
                    }
                ),
                serde_json::to_value(e).unwrap(),
            ),
            Event::InlineCompletionRating(e) => (
                "Edit Prediction Rated".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Call(e) => {
                let event_type = match e.operation.trim() {
                    "unshare project" => "Project Unshared".to_string(),
                    "open channel notes" => "Channel Notes Opened".to_string(),
                    "share project" => "Project Shared".to_string(),
                    "join channel" => "Channel Joined".to_string(),
                    "hang up" => "Call Ended".to_string(),
                    "accept incoming" => "Incoming Call Accepted".to_string(),
                    "invite" => "Participant Invited".to_string(),
                    "disable microphone" => "Microphone Disabled".to_string(),
                    "enable microphone" => "Microphone Enabled".to_string(),
                    "enable screen share" => "Screen Share Enabled".to_string(),
                    "disable screen share" => "Screen Share Disabled".to_string(),
                    "decline incoming" => "Incoming Call Declined".to_string(),
                    _ => format!("Unknown Call Event: {}", e.operation),
                };

                (event_type, serde_json::to_value(e).unwrap())
            }
            Event::Assistant(e) => (
                match e.phase {
                    telemetry_events::AssistantPhase::Response => "Assistant Responded".to_string(),
                    telemetry_events::AssistantPhase::Invoked => "Assistant Invoked".to_string(),
                    telemetry_events::AssistantPhase::Accepted => {
                        "Assistant Response Accepted".to_string()
                    }
                    telemetry_events::AssistantPhase::Rejected => {
                        "Assistant Response Rejected".to_string()
                    }
                },
                serde_json::to_value(e).unwrap(),
            ),
            Event::Cpu(_) | Event::Memory(_) => return None,
            Event::App(e) => {
                let mut properties = json!({});
                let event_type = match e.operation.trim() {
                    // App
                    "open" => "App Opened".to_string(),
                    "first open" => "App First Opened".to_string(),
                    "first open for release channel" => {
                        "App First Opened For Release Channel".to_string()
                    }
                    "close" => "App Closed".to_string(),

                    // Project
                    "open project" => "Project Opened".to_string(),
                    "open node project" => {
                        properties["project_type"] = json!("node");
                        "Project Opened".to_string()
                    }
                    "open pnpm project" => {
                        properties["project_type"] = json!("pnpm");
                        "Project Opened".to_string()
                    }
                    "open yarn project" => {
                        properties["project_type"] = json!("yarn");
                        "Project Opened".to_string()
                    }

                    // SSH
                    "create ssh server" => "SSH Server Created".to_string(),
                    "create ssh project" => "SSH Project Created".to_string(),
                    "open ssh project" => "SSH Project Opened".to_string(),

                    // Welcome Page
                    "welcome page: change keymap" => "Welcome Keymap Changed".to_string(),
                    "welcome page: change theme" => "Welcome Theme Changed".to_string(),
                    "welcome page: close" => "Welcome Page Closed".to_string(),
                    "welcome page: edit settings" => "Welcome Settings Edited".to_string(),
                    "welcome page: install cli" => "Welcome CLI Installed".to_string(),
                    "welcome page: open" => "Welcome Page Opened".to_string(),
                    "welcome page: open extensions" => "Welcome Extensions Page Opened".to_string(),
                    "welcome page: sign in to copilot" => "Welcome Copilot Signed In".to_string(),
                    "welcome page: toggle diagnostic telemetry" => {
                        "Welcome Diagnostic Telemetry Toggled".to_string()
                    }
                    "welcome page: toggle metric telemetry" => {
                        "Welcome Metric Telemetry Toggled".to_string()
                    }
                    "welcome page: toggle vim" => "Welcome Vim Mode Toggled".to_string(),
                    "welcome page: view docs" => "Welcome Documentation Viewed".to_string(),

                    // Extensions
                    "extensions page: open" => "Extensions Page Opened".to_string(),
                    "extensions: install extension" => "Extension Installed".to_string(),
                    "extensions: uninstall extension" => "Extension Uninstalled".to_string(),

                    // Misc
                    "markdown preview: open" => "Markdown Preview Opened".to_string(),
                    "project diagnostics: open" => "Project Diagnostics Opened".to_string(),
                    "project search: open" => "Project Search Opened".to_string(),
                    "repl sessions: open" => "REPL Session Started".to_string(),

                    // Feature Upsell
                    "feature upsell: toggle vim" => {
                        properties["source"] = json!("Feature Upsell");
                        "Vim Mode Toggled".to_string()
                    }
                    _ => e
                        .operation
                        .strip_prefix("feature upsell: viewed docs (")
                        .and_then(|s| s.strip_suffix(')'))
                        .map_or_else(
                            || format!("Unknown App Event: {}", e.operation),
                            |docs_url| {
                                properties["url"] = json!(docs_url);
                                properties["source"] = json!("Feature Upsell");
                                "Documentation Viewed".to_string()
                            },
                        ),
                };
                (event_type, properties)
            }
            Event::Setting(e) => (
                "Settings Changed".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Extension(e) => (
                "Extension Loaded".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Edit(e) => (
                "Editor Edited".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Action(e) => (
                "Action Invoked".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Repl(e) => (
                "Kernel Status Changed".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
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
        let user_properties = Some(serde_json::json!({
            "is_staff": body.is_staff,
        }));

        Some(SnowflakeRow {
            time: timestamp,
            user_id: body.metrics_id.clone(),
            device_id: body.system_id.clone(),
            event_type,
            event_properties,
            user_properties,
            insert_id: Some(Uuid::new_v4().to_string()),
        })
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
