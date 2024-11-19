use super::ips_file::IpsFile;
use crate::api::CloudflareIpCountryHeader;
use crate::clickhouse::write_to_table;
use crate::{api::slack, AppState, Error, Result};
use anyhow::{anyhow, Context};
use aws_sdk_s3::primitives::ByteStream;
use axum::{
    body::Bytes,
    headers::Header,
    http::{HeaderMap, HeaderName, StatusCode},
    routing::post,
    Extension, Router, TypedHeader,
};
use chrono::Duration;
use rpc::ExtensionMetadata;
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};
use telemetry_events::{
    ActionEvent, AppEvent, AssistantEvent, CallEvent, CpuEvent, EditEvent, EditorEvent, Event,
    EventRequestBody, EventWrapper, ExtensionEvent, InlineCompletionEvent, MemoryEvent, Panic,
    ReplEvent, SettingEvent,
};
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
    let panic = report.panic;

    if panic.os_name == "Linux" && panic.os_version == Some("1.0.0".to_string()) {
        return Err(Error::http(
            StatusCode::BAD_REQUEST,
            "invalid os version".into(),
        ))?;
    }

    tracing::error!(
        service = "client",
        version = %panic.app_version,
        os_name = %panic.os_name,
        os_version = %panic.os_version.clone().unwrap_or_default(),
        installation_id = %panic.installation_id.clone().unwrap_or_default(),
        description = %panic.payload,
        backtrace = %panic.backtrace.join("\n"),
        "panic report"
    );

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
                        slack::Text::markdown(format!(
                            "*OS:*\n{} {}",
                            panic.os_name,
                            panic.os_version.unwrap_or_default()
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

    let mut to_upload = ToUpload::default();
    let Some(last_event) = request_body.events.last() else {
        return Err(Error::http(StatusCode::BAD_REQUEST, "no events".into()))?;
    };
    let country_code = country_code_header.map(|h| h.to_string());

    let first_event_at = chrono::Utc::now()
        - chrono::Duration::milliseconds(last_event.milliseconds_since_first_event);

    if let Some(kinesis_client) = app.kinesis_client.clone() {
        if let Some(stream) = app.config.kinesis_stream.clone() {
            let mut request = kinesis_client.put_records().stream_name(stream);
            for row in for_snowflake(request_body.clone(), first_event_at) {
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

    let Some(clickhouse_client) = app.clickhouse_client.clone() else {
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let first_event_at = chrono::Utc::now()
        - chrono::Duration::milliseconds(last_event.milliseconds_since_first_event);

    for wrapper in &request_body.events {
        match &wrapper.event {
            Event::Editor(event) => to_upload.editor_events.push(EditorEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                country_code.clone(),
                checksum_matched,
            )),
            Event::InlineCompletion(event) => {
                to_upload
                    .inline_completion_events
                    .push(InlineCompletionEventRow::from_event(
                        event.clone(),
                        wrapper,
                        &request_body,
                        first_event_at,
                        country_code.clone(),
                        checksum_matched,
                    ))
            }
            Event::Call(event) => to_upload.call_events.push(CallEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Assistant(event) => {
                to_upload
                    .assistant_events
                    .push(AssistantEventRow::from_event(
                        event.clone(),
                        wrapper,
                        &request_body,
                        first_event_at,
                        checksum_matched,
                    ))
            }
            Event::Cpu(event) => to_upload.cpu_events.push(CpuEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Memory(event) => to_upload.memory_events.push(MemoryEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::App(event) => to_upload.app_events.push(AppEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Setting(event) => to_upload.setting_events.push(SettingEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Edit(event) => to_upload.edit_events.push(EditEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Action(event) => to_upload.action_events.push(ActionEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
            Event::Extension(event) => {
                let metadata = app
                    .db
                    .get_extension_version(&event.extension_id, &event.version)
                    .await?;
                to_upload
                    .extension_events
                    .push(ExtensionEventRow::from_event(
                        event.clone(),
                        wrapper,
                        &request_body,
                        metadata,
                        first_event_at,
                        checksum_matched,
                    ))
            }
            Event::Repl(event) => to_upload.repl_events.push(ReplEventRow::from_event(
                event.clone(),
                wrapper,
                &request_body,
                first_event_at,
                checksum_matched,
            )),
        }
    }

    to_upload
        .upload(&clickhouse_client)
        .await
        .map_err(|err| Error::Internal(anyhow!(err)))?;

    Ok(())
}

#[derive(Default)]
struct ToUpload {
    editor_events: Vec<EditorEventRow>,
    inline_completion_events: Vec<InlineCompletionEventRow>,
    assistant_events: Vec<AssistantEventRow>,
    call_events: Vec<CallEventRow>,
    cpu_events: Vec<CpuEventRow>,
    memory_events: Vec<MemoryEventRow>,
    app_events: Vec<AppEventRow>,
    setting_events: Vec<SettingEventRow>,
    extension_events: Vec<ExtensionEventRow>,
    edit_events: Vec<EditEventRow>,
    action_events: Vec<ActionEventRow>,
    repl_events: Vec<ReplEventRow>,
}

impl ToUpload {
    pub async fn upload(&self, clickhouse_client: &clickhouse::Client) -> anyhow::Result<()> {
        const EDITOR_EVENTS_TABLE: &str = "editor_events";
        write_to_table(EDITOR_EVENTS_TABLE, &self.editor_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{EDITOR_EVENTS_TABLE}'"))?;

        const INLINE_COMPLETION_EVENTS_TABLE: &str = "inline_completion_events";
        write_to_table(
            INLINE_COMPLETION_EVENTS_TABLE,
            &self.inline_completion_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{INLINE_COMPLETION_EVENTS_TABLE}'"))?;

        const ASSISTANT_EVENTS_TABLE: &str = "assistant_events";
        write_to_table(
            ASSISTANT_EVENTS_TABLE,
            &self.assistant_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{ASSISTANT_EVENTS_TABLE}'"))?;

        const CALL_EVENTS_TABLE: &str = "call_events";
        write_to_table(CALL_EVENTS_TABLE, &self.call_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{CALL_EVENTS_TABLE}'"))?;

        const CPU_EVENTS_TABLE: &str = "cpu_events";
        write_to_table(CPU_EVENTS_TABLE, &self.cpu_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{CPU_EVENTS_TABLE}'"))?;

        const MEMORY_EVENTS_TABLE: &str = "memory_events";
        write_to_table(MEMORY_EVENTS_TABLE, &self.memory_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{MEMORY_EVENTS_TABLE}'"))?;

        const APP_EVENTS_TABLE: &str = "app_events";
        write_to_table(APP_EVENTS_TABLE, &self.app_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{APP_EVENTS_TABLE}'"))?;

        const SETTING_EVENTS_TABLE: &str = "setting_events";
        write_to_table(
            SETTING_EVENTS_TABLE,
            &self.setting_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{SETTING_EVENTS_TABLE}'"))?;

        const EXTENSION_EVENTS_TABLE: &str = "extension_events";
        write_to_table(
            EXTENSION_EVENTS_TABLE,
            &self.extension_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{EXTENSION_EVENTS_TABLE}'"))?;

        const EDIT_EVENTS_TABLE: &str = "edit_events";
        write_to_table(EDIT_EVENTS_TABLE, &self.edit_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{EDIT_EVENTS_TABLE}'"))?;

        const ACTION_EVENTS_TABLE: &str = "action_events";
        write_to_table(ACTION_EVENTS_TABLE, &self.action_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{ACTION_EVENTS_TABLE}'"))?;

        const REPL_EVENTS_TABLE: &str = "repl_events";
        write_to_table(REPL_EVENTS_TABLE, &self.repl_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{REPL_EVENTS_TABLE}'"))?;

        Ok(())
    }
}

pub fn serialize_country_code<S>(country_code: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if country_code.len() != 2 {
        use serde::ser::Error;
        return Err(S::Error::custom(
            "country_code must be exactly 2 characters",
        ));
    }

    let country_code = country_code.as_bytes();

    serializer.serialize_u16(((country_code[1] as u16) << 8) + country_code[0] as u16)
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct EditorEventRow {
    system_id: String,
    installation_id: String,
    session_id: Option<String>,
    metrics_id: String,
    operation: String,
    app_version: String,
    file_extension: String,
    os_name: String,
    os_version: String,
    release_channel: String,
    signed_in: bool,
    vim_mode: bool,
    #[serde(serialize_with = "serialize_country_code")]
    country_code: String,
    region_code: String,
    city: String,
    time: i64,
    copilot_enabled: bool,
    copilot_enabled_for_language: bool,
    architecture: String,
    is_staff: Option<bool>,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    is_via_ssh: bool,
}

impl EditorEventRow {
    fn from_event(
        event: EditorEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        country_code: Option<String>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            architecture: body.architecture.clone(),
            system_id: body.system_id.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone().unwrap_or_default(),
            session_id: body.session_id.clone(),
            metrics_id: body.metrics_id.clone().unwrap_or_default(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            operation: event.operation,
            file_extension: event.file_extension.unwrap_or_default(),
            signed_in: wrapper.signed_in,
            vim_mode: event.vim_mode,
            copilot_enabled: event.copilot_enabled,
            copilot_enabled_for_language: event.copilot_enabled_for_language,
            country_code: country_code.unwrap_or("XX".to_string()),
            region_code: "".to_string(),
            city: "".to_string(),
            is_via_ssh: event.is_via_ssh,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct InlineCompletionEventRow {
    installation_id: String,
    session_id: Option<String>,
    provider: String,
    suggestion_accepted: bool,
    app_version: String,
    file_extension: String,
    os_name: String,
    os_version: String,
    release_channel: String,
    signed_in: bool,
    #[serde(serialize_with = "serialize_country_code")]
    country_code: String,
    region_code: String,
    city: String,
    time: i64,
    is_staff: Option<bool>,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
}

impl InlineCompletionEventRow {
    fn from_event(
        event: InlineCompletionEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        country_code: Option<String>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone().unwrap_or_default(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            file_extension: event.file_extension.unwrap_or_default(),
            signed_in: wrapper.signed_in,
            country_code: country_code.unwrap_or("XX".to_string()),
            region_code: "".to_string(),
            city: "".to_string(),
            provider: event.provider,
            suggestion_accepted: event.suggestion_accepted,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct CallEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    release_channel: String,
    os_name: String,
    os_version: String,
    checksum_matched: bool,

    // ClientEventBase
    installation_id: String,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // CallEventRow
    operation: String,
    room_id: Option<u64>,
    channel_id: Option<u64>,
}

impl CallEventRow {
    fn from_event(
        event: CallEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone().unwrap_or_default(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            operation: event.operation,
            room_id: event.room_id,
            channel_id: event.channel_id,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct AssistantEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // AssistantEventRow
    conversation_id: String,
    kind: String,
    phase: String,
    model: String,
    response_latency_in_ms: Option<i64>,
    error_message: Option<String>,
}

impl AssistantEventRow {
    fn from_event(
        event: AssistantEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            conversation_id: event.conversation_id.unwrap_or_default(),
            kind: event.kind.to_string(),
            phase: event.phase.to_string(),
            model: event.model,
            response_latency_in_ms: event
                .response_latency
                .map(|latency| latency.as_millis() as i64),
            error_message: event.error_message,
        }
    }
}

#[derive(Debug, clickhouse::Row, Serialize)]
pub struct CpuEventRow {
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    usage_as_percentage: f32,
    core_count: u32,
    app_version: String,
    release_channel: String,
    os_name: String,
    os_version: String,
    time: i64,
    // pub normalized_cpu_usage: f64, MATERIALIZED
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
}

impl CpuEventRow {
    fn from_event(
        event: CpuEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            usage_as_percentage: event.usage_as_percentage,
            core_count: event.core_count,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct MemoryEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // MemoryEventRow
    memory_in_bytes: u64,
    virtual_memory_in_bytes: u64,
}

impl MemoryEventRow {
    fn from_event(
        event: MemoryEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            memory_in_bytes: event.memory_in_bytes,
            virtual_memory_in_bytes: event.virtual_memory_in_bytes,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct AppEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // AppEventRow
    operation: String,
}

impl AppEventRow {
    fn from_event(
        event: AppEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            operation: event.operation,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct SettingEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,
    // SettingEventRow
    setting: String,
    value: String,
}

impl SettingEventRow {
    fn from_event(
        event: SettingEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            checksum_matched,
            patch: semver.map(|v| v.patch() as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            setting: event.setting,
            value: event.value,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct ExtensionEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // ExtensionEventRow
    extension_id: Arc<str>,
    extension_version: Arc<str>,
    dev: bool,
    schema_version: Option<i32>,
    wasm_api_version: Option<String>,
}

impl ExtensionEventRow {
    fn from_event(
        event: ExtensionEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        extension_metadata: Option<ExtensionMetadata>,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            extension_id: event.extension_id,
            extension_version: event.version,
            dev: extension_metadata.is_none(),
            schema_version: extension_metadata
                .as_ref()
                .and_then(|metadata| metadata.manifest.schema_version),
            wasm_api_version: extension_metadata.as_ref().and_then(|metadata| {
                metadata
                    .manifest
                    .wasm_api_version
                    .as_ref()
                    .map(|version| version.to_string())
            }),
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct ReplEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // ReplEventRow
    kernel_language: String,
    kernel_status: String,
    repl_session_id: String,
}

impl ReplEventRow {
    fn from_event(
        event: ReplEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            kernel_language: event.kernel_language,
            kernel_status: event.kernel_status,
            repl_session_id: event.repl_session_id,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct EditEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    // Note: This column name has a typo in the ClickHouse table.
    #[serde(rename = "sesssion_id")]
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // EditEventRow
    period_start: i64,
    period_end: i64,
    environment: String,
    is_via_ssh: bool,
}

impl EditEventRow {
    fn from_event(
        event: EditEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        let period_start = time - chrono::Duration::milliseconds(event.duration);
        let period_end = time;

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            period_start: period_start.timestamp_millis(),
            period_end: period_end.timestamp_millis(),
            environment: event.environment,
            is_via_ssh: event.is_via_ssh,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct ActionEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    checksum_matched: bool,
    release_channel: String,
    os_name: String,
    os_version: String,

    // ClientEventBase
    installation_id: Option<String>,
    // Note: This column name has a typo in the ClickHouse table.
    #[serde(rename = "sesssion_id")]
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,
    // ActionEventRow
    source: String,
    action: String,
}

impl ActionEventRow {
    fn from_event(
        event: ActionEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        checksum_matched: bool,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|v| v.major() as i32),
            minor: semver.map(|v| v.minor() as i32),
            patch: semver.map(|v| v.patch() as i32),
            checksum_matched,
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            source: event.source,
            action: event.action,
        }
    }
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
) -> impl Iterator<Item = SnowflakeRow> {
    body.events.into_iter().map(move |event| {
        let timestamp =
            first_event_at + Duration::milliseconds(event.milliseconds_since_first_event);
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
                    "Inline Completion {}",
                    if e.suggestion_accepted {
                        "Accepted"
                    } else {
                        "Discarded"
                    }
                ),
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
                    "enable camera" => "Camera Enabled".to_string(),
                    "disable camera" => "Camera Disabled".to_string(),
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
            Event::Cpu(e) => (
                "System CPU Sampled".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::Memory(e) => (
                "System Memory Sampled".to_string(),
                serde_json::to_value(e).unwrap(),
            ),
            Event::App(e) => {
                let mut properties = json!({});
                let event_type = match e.operation.trim() {
                    "extensions: install extension" => "Extension Installed".to_string(),
                    "open" => "App Opened".to_string(),
                    "project search: open" => "Project Search Opened".to_string(),
                    "first open" => {
                        properties["is_first_open"] = json!(true);
                        "App First Opened".to_string()
                    }
                    "extensions: uninstall extension" => "Extension Uninstalled".to_string(),
                    "welcome page: close" => "Welcome Page Closed".to_string(),
                    "open project" => {
                        properties["is_first_time"] = json!(false);
                        "Project Opened".to_string()
                    }
                    "welcome page: install cli" => "CLI Installed".to_string(),
                    "project diagnostics: open" => "Project Diagnostics Opened".to_string(),
                    "extensions page: open" => "Extensions Page Opened".to_string(),
                    "welcome page: change theme" => "Welcome Theme Changed".to_string(),
                    "welcome page: toggle metric telemetry" => {
                        properties["enabled"] = json!(false);
                        "Welcome Telemetry Toggled".to_string()
                    }
                    "welcome page: change keymap" => "Keymap Changed".to_string(),
                    "welcome page: toggle vim" => {
                        properties["enabled"] = json!(false);
                        "Welcome Vim Mode Toggled".to_string()
                    }
                    "welcome page: sign in to copilot" => "Welcome Copilot Signed In".to_string(),
                    "welcome page: toggle diagnostic telemetry" => {
                        "Welcome Telemetry Toggled".to_string()
                    }
                    "welcome page: open" => "Welcome Page Opened".to_string(),
                    "close" => "App Closed".to_string(),
                    "markdown preview: open" => "Markdown Preview Opened".to_string(),
                    "welcome page: open extensions" => "Extensions Page Opened".to_string(),
                    "open node project" | "open pnpm project" | "open yarn project" => {
                        properties["project_type"] = json!("node");
                        properties["is_first_time"] = json!(false);
                        "Project Opened".to_string()
                    }
                    "repl sessions: open" => "REPL Session Started".to_string(),
                    "welcome page: toggle helix" => {
                        properties["enabled"] = json!(false);
                        "Helix Mode Toggled".to_string()
                    }
                    "welcome page: edit settings" => {
                        properties["changed_settings"] = json!([]);
                        "Settings Edited".to_string()
                    }
                    "welcome page: view docs" => "Documentation Viewed".to_string(),
                    "open ssh project" => {
                        properties["is_first_time"] = json!(false);
                        "SSH Project Opened".to_string()
                    }
                    "create ssh server" => "SSH Server Created".to_string(),
                    "create ssh project" => "SSH Project Created".to_string(),
                    "first open for release channel" => {
                        properties["is_first_for_channel"] = json!(true);
                        "App First Opened For Release Channel".to_string()
                    }
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
        }

        let user_properties = Some(serde_json::json!({
            "is_staff": body.is_staff,
        }));

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

#[derive(Serialize, Deserialize)]
struct SnowflakeRow {
    pub time: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    pub event_type: String,
    pub event_properties: serde_json::Value,
    pub user_properties: Option<serde_json::Value>,
    pub insert_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SnowflakeData {
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    pub installation_id: Option<String>,
    /// Identifier unique to each logged in Zed user (randomly generated on first sign in)
    /// Identifier unique to each Zed session (differs for each time you open Zed)
    pub session_id: Option<String>,
    pub metrics_id: Option<String>,
    /// True for Zed staff, otherwise false
    pub is_staff: Option<bool>,
    /// Zed version number
    pub app_version: String,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    /// Zed release channel (stable, preview, dev)
    pub release_channel: Option<String>,
    pub signed_in: bool,

    #[serde(flatten)]
    pub editor_event: Option<EditorEvent>,
    #[serde(flatten)]
    pub inline_completion_event: Option<InlineCompletionEvent>,
    #[serde(flatten)]
    pub call_event: Option<CallEvent>,
    #[serde(flatten)]
    pub assistant_event: Option<AssistantEvent>,
    #[serde(flatten)]
    pub cpu_event: Option<CpuEvent>,
    #[serde(flatten)]
    pub memory_event: Option<MemoryEvent>,
    #[serde(flatten)]
    pub app_event: Option<AppEvent>,
    #[serde(flatten)]
    pub setting_event: Option<SettingEvent>,
    #[serde(flatten)]
    pub extension_event: Option<ExtensionEvent>,
    #[serde(flatten)]
    pub edit_event: Option<EditEvent>,
    #[serde(flatten)]
    pub repl_event: Option<ReplEvent>,
    #[serde(flatten)]
    pub action_event: Option<ActionEvent>,
}
