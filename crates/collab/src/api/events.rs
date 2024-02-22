use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::Bytes, headers::Header, http::HeaderName, routing::post, Extension, Router, TypedHeader,
};
use hyper::StatusCode;
use lazy_static::lazy_static;
use serde::Serialize;
use sha2::{Digest, Sha256};
use telemetry_events::{
    ActionEvent, AppEvent, AssistantEvent, CallEvent, CopilotEvent, CpuEvent, EditEvent,
    EditorEvent, Event, EventRequestBody, EventWrapper, MemoryEvent, SettingEvent,
};

use crate::{AppState, Error, Result};

pub fn router() -> Router {
    Router::new().route("/events", post(post_events))
}

lazy_static! {
    static ref ZED_CHECKSUM_HEADER: HeaderName = HeaderName::from_static("x-zed-checksum");
    static ref CLOUDFLARE_IP_COUNTRY_HEADER: HeaderName = HeaderName::from_static("cf-ipcountry");
}

pub struct ZedChecksumHeader(Vec<u8>);

impl Header for ZedChecksumHeader {
    fn name() -> &'static HeaderName {
        &ZED_CHECKSUM_HEADER
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

pub struct CloudflareIpCountryHeader(Option<String>);

impl Header for CloudflareIpCountryHeader {
    fn name() -> &'static HeaderName {
        &CLOUDFLARE_IP_COUNTRY_HEADER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        if let Some(header) = values.next() {
            Ok(Self(Some(
                header
                    .to_str()
                    .map_err(|_| axum::headers::Error::invalid())?
                    .to_string(),
            )))
        } else {
            Ok(Self(None))
        }
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

pub async fn post_events(
    Extension(app): Extension<Arc<AppState>>,
    TypedHeader(ZedChecksumHeader(checksum)): TypedHeader<ZedChecksumHeader>,
    TypedHeader(CloudflareIpCountryHeader(country_code)): TypedHeader<CloudflareIpCountryHeader>,
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

    if &checksum[..] != &summer.finalize()[..] {
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
    edit_events: Vec<EditEventRow>,
    action_events: Vec<ActionEventRow>,
}

impl ToUpload {
    pub async fn upload(&self, clickhouse_client: &clickhouse::Client) -> anyhow::Result<()> {
        Self::upload_to_table("editor_events", &self.editor_events, clickhouse_client).await?;
        Self::upload_to_table("copilot_events", &self.copilot_events, clickhouse_client).await?;
        Self::upload_to_table(
            "assistant_events",
            &self.assistant_events,
            clickhouse_client,
        )
        .await?;
        Self::upload_to_table("call_events", &self.call_events, clickhouse_client).await?;
        Self::upload_to_table("cpu_events", &self.cpu_events, clickhouse_client).await?;
        Self::upload_to_table("memory_events", &self.memory_events, clickhouse_client).await?;
        Self::upload_to_table("app_events", &self.app_events, clickhouse_client).await?;
        Self::upload_to_table("setting_events", &self.setting_events, clickhouse_client).await?;
        Self::upload_to_table("edit_events", &self.edit_events, clickhouse_client).await?;
        Self::upload_to_table("action_events", &self.action_events, clickhouse_client).await?;
        Ok(())
    }

    async fn upload_to_table<T: clickhouse::Row + Serialize>(
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
        }

        Ok(())
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct EditorEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

    // EditorEventRow
    operation: String,
    file_extension: Option<String>,
    signed_in: bool,
    vim_mode: bool,
    copilot_enabled: bool,
    copilot_enabled_for_language: bool,
    country_code: Option<String>,
    region_code: Option<String>,
    city: Option<String>,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            operation: event.operation,
            file_extension: event.file_extension,
            signed_in: wrapper.signed_in,
            vim_mode: event.vim_mode,
            copilot_enabled: event.copilot_enabled,
            copilot_enabled_for_language: event.copilot_enabled_for_language,
            country_code,
            region_code: None,
            city: None,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct CopilotEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

    // CopilotEventRow
    suggestion_id: Option<String>,
    suggestion_accepted: bool,
    file_extension: Option<String>,
    signed_in: bool,
    country_code: Option<String>,
    region_code: Option<String>,
    city: Option<String>,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            file_extension: event.file_extension,
            signed_in: wrapper.signed_in,
            country_code,
            region_code: None,
            city: None,
            suggestion_id: event.suggestion_id,
            suggestion_accepted: event.suggestion_accepted,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct CallEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
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
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

    // AssistantEventRow
    conversation_id: Option<String>,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            conversation_id: event.conversation_id,
            kind: event.kind.to_string(),
            model: event.model,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct CpuEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

    // CpuEventRow
    usage_as_percentage: f32,
    core_count: u32,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            usage_as_percentage: event.usage_as_percentage,
            core_count: event.core_count,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct MemoryEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            memory_in_bytes: event.memory_in_bytes,
            virtual_memory_in_bytes: event.virtual_memory_in_bytes,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct AppEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            operation: event.operation,
        }
    }
}

#[derive(Serialize, clickhouse::Row)]
pub struct SettingEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            setting: event.setting,
            value: event.value,
        }
    }
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct EditEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,

    // EditEventRow
    period_start: String,
    period_end: String,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            period_start: period_start.to_rfc3339(),
            period_end: period_end.to_rfc3339(),
            environment: event.environment,
        }
    }
}

#[derive(Serialize, clickhouse::Row)]
pub struct ActionEventRow {
    // AppInfoBase
    app_version: String,
    major: Option<usize>,
    minor: Option<usize>,
    patch: Option<usize>,
    release_channel: Option<String>,

    // SystemInfoBase
    os_name: String,
    os_version: Option<String>,
    architecture: String,

    // ClientEventBase
    installation_id: Option<String>,
    session_id: Option<String>,
    is_staff: Option<bool>,
    time: String,
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
            major: semver.map(|s| s.major),
            minor: semver.map(|s| s.minor),
            patch: semver.map(|s| s.patch),
            release_channel: body.release_channel.clone(),
            os_name: body.os_name.clone(),
            os_version: body.os_version.clone(),
            architecture: body.architecture.clone(),
            installation_id: body.installation_id.clone(),
            session_id: body.session_id.clone(),
            is_staff: body.is_staff,
            time: time.to_rfc3339(),
            source: event.source,
            action: event.action,
        }
    }
}
