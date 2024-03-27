use super::ips_file::IpsFile;
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
use rpc::ExtensionMetadata;
use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};
use telemetry_events::{
    ActionEvent, AppEvent, AssistantEvent, CallEvent, CopilotEvent, CpuEvent, EditEvent,
    EditorEvent, Event, EventRequestBody, EventWrapper, ExtensionEvent, MemoryEvent, SettingEvent,
};
use util::SemanticVersion;

pub fn router() -> Router {
    Router::new()
        .route("/telemetry/events", post(post_events))
        .route("/telemetry/crashes", post(post_crash))
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

pub struct CloudflareIpCountryHeader(String);

impl Header for CloudflareIpCountryHeader {
    fn name() -> &'static HeaderName {
        static CLOUDFLARE_IP_COUNTRY_HEADER: OnceLock<HeaderName> = OnceLock::new();
        CLOUDFLARE_IP_COUNTRY_HEADER.get_or_init(|| HeaderName::from_static("cf-ipcountry"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let country_code = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        Ok(Self(country_code.to_string()))
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
    static CRASH_REPORTS_BUCKET: &str = "zed-crash-reports";

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
        description = %description,
        backtrace = %summary,
        "crash report");

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

pub async fn post_events(
    Extension(app): Extension<Arc<AppState>>,
    TypedHeader(ZedChecksumHeader(checksum)): TypedHeader<ZedChecksumHeader>,
    country_code_header: Option<TypedHeader<CloudflareIpCountryHeader>>,
    body: Bytes,
) -> Result<()> {
    let Some(clickhouse_client) = app.clickhouse_client.clone() else {
        Err(Error::Http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let Some(checksum_seed) = app.config.zed_client_checksum_seed.as_ref() else {
        return Err(Error::Http(
            StatusCode::INTERNAL_SERVER_ERROR,
            "events not enabled".into(),
        ))?;
    };

    let mut summer = Sha256::new();
    summer.update(checksum_seed);
    summer.update(&body);
    summer.update(checksum_seed);

    if &checksum != &summer.finalize()[..] {
        return Err(Error::Http(
            StatusCode::BAD_REQUEST,
            "invalid checksum".into(),
        ))?;
    }

    let request_body: telemetry_events::EventRequestBody =
        serde_json::from_slice(&body).map_err(|err| {
            log::error!("can't parse event json: {err}");
            Error::Internal(anyhow!(err))
        })?;

    let mut to_upload = ToUpload::default();
    let Some(last_event) = request_body.events.last() else {
        return Err(Error::Http(StatusCode::BAD_REQUEST, "no events".into()))?;
    };
    let country_code = country_code_header.map(|h| h.0 .0);

    let first_event_at = chrono::Utc::now()
        - chrono::Duration::milliseconds(last_event.milliseconds_since_first_event);

    for wrapper in &request_body.events {
        match &wrapper.event {
            Event::Editor(event) => to_upload.editor_events.push(EditorEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
                country_code.clone(),
            )),
            Event::Copilot(event) => to_upload.copilot_events.push(CopilotEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
                country_code.clone(),
            )),
            Event::Call(event) => to_upload.call_events.push(CallEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::Assistant(event) => {
                to_upload
                    .assistant_events
                    .push(AssistantEventRow::from_event(
                        event.clone(),
                        &wrapper,
                        &request_body,
                        first_event_at,
                    ))
            }
            Event::Cpu(event) => to_upload.cpu_events.push(CpuEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::Memory(event) => to_upload.memory_events.push(MemoryEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::App(event) => to_upload.app_events.push(AppEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::Setting(event) => to_upload.setting_events.push(SettingEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::Edit(event) => to_upload.edit_events.push(EditEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
            )),
            Event::Action(event) => to_upload.action_events.push(ActionEventRow::from_event(
                event.clone(),
                &wrapper,
                &request_body,
                first_event_at,
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
                        &wrapper,
                        &request_body,
                        metadata,
                        first_event_at,
                    ))
            }
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
    copilot_events: Vec<CopilotEventRow>,
    assistant_events: Vec<AssistantEventRow>,
    call_events: Vec<CallEventRow>,
    cpu_events: Vec<CpuEventRow>,
    memory_events: Vec<MemoryEventRow>,
    app_events: Vec<AppEventRow>,
    setting_events: Vec<SettingEventRow>,
    extension_events: Vec<ExtensionEventRow>,
    edit_events: Vec<EditEventRow>,
    action_events: Vec<ActionEventRow>,
}

impl ToUpload {
    pub async fn upload(&self, clickhouse_client: &clickhouse::Client) -> anyhow::Result<()> {
        const EDITOR_EVENTS_TABLE: &str = "editor_events";
        Self::upload_to_table(EDITOR_EVENTS_TABLE, &self.editor_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{EDITOR_EVENTS_TABLE}'"))?;

        const COPILOT_EVENTS_TABLE: &str = "copilot_events";
        Self::upload_to_table(
            COPILOT_EVENTS_TABLE,
            &self.copilot_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{COPILOT_EVENTS_TABLE}'"))?;

        const ASSISTANT_EVENTS_TABLE: &str = "assistant_events";
        Self::upload_to_table(
            ASSISTANT_EVENTS_TABLE,
            &self.assistant_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{ASSISTANT_EVENTS_TABLE}'"))?;

        const CALL_EVENTS_TABLE: &str = "call_events";
        Self::upload_to_table(CALL_EVENTS_TABLE, &self.call_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{CALL_EVENTS_TABLE}'"))?;

        const CPU_EVENTS_TABLE: &str = "cpu_events";
        Self::upload_to_table(CPU_EVENTS_TABLE, &self.cpu_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{CPU_EVENTS_TABLE}'"))?;

        const MEMORY_EVENTS_TABLE: &str = "memory_events";
        Self::upload_to_table(MEMORY_EVENTS_TABLE, &self.memory_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{MEMORY_EVENTS_TABLE}'"))?;

        const APP_EVENTS_TABLE: &str = "app_events";
        Self::upload_to_table(APP_EVENTS_TABLE, &self.app_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{APP_EVENTS_TABLE}'"))?;

        const SETTING_EVENTS_TABLE: &str = "setting_events";
        Self::upload_to_table(
            SETTING_EVENTS_TABLE,
            &self.setting_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{SETTING_EVENTS_TABLE}'"))?;

        const EXTENSION_EVENTS_TABLE: &str = "extension_events";
        Self::upload_to_table(
            EXTENSION_EVENTS_TABLE,
            &self.extension_events,
            clickhouse_client,
        )
        .await
        .with_context(|| format!("failed to upload to table '{EXTENSION_EVENTS_TABLE}'"))?;

        const EDIT_EVENTS_TABLE: &str = "edit_events";
        Self::upload_to_table(EDIT_EVENTS_TABLE, &self.edit_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{EDIT_EVENTS_TABLE}'"))?;

        const ACTION_EVENTS_TABLE: &str = "action_events";
        Self::upload_to_table(ACTION_EVENTS_TABLE, &self.action_events, clickhouse_client)
            .await
            .with_context(|| format!("failed to upload to table '{ACTION_EVENTS_TABLE}'"))?;

        Ok(())
    }

    async fn upload_to_table<T: clickhouse::Row + Serialize + std::fmt::Debug>(
        table: &str,
        rows: &[T],
        clickhouse_client: &clickhouse::Client,
    ) -> anyhow::Result<()> {
        if !rows.is_empty() {
            let mut insert = clickhouse_client.insert(table)?;

            for event in rows {
                insert.write(event).await?;
            }

            insert.end().await?;

            let event_count = rows.len();
            log::info!(
                "wrote {event_count} {event_specifier} to '{table}'",
                event_specifier = if event_count == 1 { "event" } else { "events" }
            );
        }

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

    serializer.serialize_u16(((country_code[0] as u16) << 8) + country_code[1] as u16)
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct EditorEventRow {
    pub installation_id: String,
    pub operation: String,
    pub app_version: String,
    pub file_extension: String,
    pub os_name: String,
    pub os_version: String,
    pub release_channel: String,
    pub signed_in: bool,
    pub vim_mode: bool,
    #[serde(serialize_with = "serialize_country_code")]
    pub country_code: String,
    pub region_code: String,
    pub city: String,
    pub time: i64,
    pub copilot_enabled: bool,
    pub copilot_enabled_for_language: bool,
    pub historical_event: bool,
    pub architecture: String,
    pub is_staff: Option<bool>,
    pub session_id: Option<String>,
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
}

impl EditorEventRow {
    fn from_event(
        event: EditorEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        country_code: Option<String>,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone().unwrap_or_default(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone().unwrap_or_default(),
            session_id: body.session_id.clone(),
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
            historical_event: false,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct CopilotEventRow {
    pub installation_id: String,
    pub suggestion_id: String,
    pub suggestion_accepted: bool,
    pub app_version: String,
    pub file_extension: String,
    pub os_name: String,
    pub os_version: String,
    pub release_channel: String,
    pub signed_in: bool,
    #[serde(serialize_with = "serialize_country_code")]
    pub country_code: String,
    pub region_code: String,
    pub city: String,
    pub time: i64,
    pub is_staff: Option<bool>,
    pub session_id: Option<String>,
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
}

impl CopilotEventRow {
    fn from_event(
        event: CopilotEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
        country_code: Option<String>,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
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
            suggestion_id: event.suggestion_id.unwrap_or_default(),
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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
    release_channel: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: i64,

    // AssistantEventRow
    conversation_id: String,
    kind: String,
    model: String,
}

impl AssistantEventRow {
    fn from_event(
        event: AssistantEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            conversation_id: event.conversation_id.unwrap_or_default(),
            kind: event.kind.to_string(),
            model: event.model,
        }
    }
}

#[derive(Debug, clickhouse::Row, Serialize)]
pub struct CpuEventRow {
    pub installation_id: Option<String>,
    pub is_staff: Option<bool>,
    pub usage_as_percentage: f32,
    pub core_count: u32,
    pub app_version: String,
    pub release_channel: String,
    pub time: i64,
    pub session_id: Option<String>,
    // pub normalized_cpu_usage: f64, MATERIALIZED
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
}

impl CpuEventRow {
    fn from_event(
        event: CpuEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
    release_channel: String,

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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
    release_channel: String,

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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
    release_channel: String,

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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
    release_channel: String,

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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
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
pub struct EditEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
    release_channel: String,

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
}

impl EditEventRow {
    fn from_event(
        event: EditEvent,
        wrapper: &EventWrapper,
        body: &EventRequestBody,
        first_event_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        let period_start = time - chrono::Duration::milliseconds(event.duration);
        let period_end = time;

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            period_start: period_start.timestamp_millis(),
            period_end: period_end.timestamp_millis(),
            environment: event.environment,
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
    release_channel: String,

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
    ) -> Self {
        let semver = body.semver();
        let time =
            first_event_at + chrono::Duration::milliseconds(wrapper.milliseconds_since_first_event);

        Self {
            app_version: body.app_version.clone(),
            major: semver.map(|s| s.major as i32),
            minor: semver.map(|s| s.minor as i32),
            patch: semver.map(|s| s.patch as i32),
            release_channel: body.release_channel.clone().unwrap_or_default(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.timestamp_millis(),
            source: event.source,
            action: event.action,
        }
    }
}
