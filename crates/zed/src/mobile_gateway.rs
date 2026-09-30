use acp_thread::{
    AcpThreadEvent, AgentThreadEntry, AssistantMessageChunk, ThreadStatus, ToolCallStatus,
};
use agent_client_protocol::schema::v1 as acp;
use agent_ui::{
    Agent, AgentPanel, AgentPanelEvent, AgentThreadSource, ConversationView, CreateThreadOptions,
    StateChange,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use futures::{
    AsyncReadExt, AsyncWriteExt, StreamExt,
    channel::mpsc::{self, Receiver, Sender},
};
use gpui::{
    App, AsyncApp, Entity, Global, ReadGlobal as _, Subscription, Task, WeakEntity, WindowHandle,
};
use language_model::LanguageModelRegistry;
use rand::RngCore;
use remote::RemoteConnectionOptions;
use serde::Deserialize;
use serde_json::{Value, json};
use settings::{
    MobileGatewayPairedDeviceSettingsContent, Settings, SettingsContent, SettingsStore,
};
use sha2::{Digest, Sha256};
use smol::net::{TcpListener, TcpStream};
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use workspace::{
    AppState, MultiWorkspace, OpenOptions, SerializedWorkspaceLocation, Workspace, WorkspaceDb,
};

const MAX_HEADERS: usize = 8 * 1024;
const MAX_BODY: usize = 64 * 1024;
const MAX_SNAPSHOT: usize = 2 * 1024 * 1024;
const PAIRING_TICKET_TTL: Duration = Duration::from_secs(120);
const MAX_PAIRED_DEVICES: usize = 32;
const EVENT_STREAM_CAPACITY: usize = 16;
const EVENT_STREAM_HEARTBEAT: Duration = Duration::from_secs(20);

#[derive(Clone, PartialEq)]
struct PairedDevice {
    id: String,
    label: String,
    credential_hash: String,
    created_at: String,
}

#[derive(Clone, PartialEq)]
struct GatewaySettings {
    enabled: bool,
    bind_address: String,
    // Kept solely so older v1 clients can continue to work during the v2 QR migration. New QR
    // flows never place this shared credential in a QR payload.
    auth_token: String,
    paired_devices: Vec<PairedDevice>,
}

impl Settings for GatewaySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let settings = content.mobile_gateway.clone().unwrap_or_default();
        Self {
            enabled: settings.enabled.unwrap_or(false),
            bind_address: settings
                .bind_address
                .unwrap_or_else(|| "127.0.0.1:7331".into()),
            auth_token: settings.auth_token.unwrap_or_default(),
            paired_devices: settings
                .paired_devices
                .unwrap_or_default()
                .into_iter()
                .map(|device| PairedDevice {
                    id: device.id,
                    label: device.label,
                    credential_hash: device.credential_hash,
                    created_at: device.created_at,
                })
                .collect(),
        }
    }
}

impl GatewaySettings {
    fn address(&self) -> Result<SocketAddr, ApiError> {
        let address: SocketAddr = self.bind_address.parse().map_err(|_| {
            ApiError(
                400,
                "Mobile Gateway bind_address must be a literal IP:port".into(),
            )
        })?;
        if address.port() == 0 || !permitted_bind(address.ip()) {
            return Err(ApiError(400, "Mobile Gateway only accepts loopback or Tailscale IP addresses with a nonzero port".into()));
        }

        if self.paired_devices.len() > MAX_PAIRED_DEVICES
            || self.paired_devices.iter().any(|device| {
                !valid_opaque(&device.id, 128)
                    || !valid_label(&device.label)
                    || device.credential_hash.len() != 64
                    || !device
                        .credential_hash
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit())
            })
        {
            return Err(ApiError(
                400,
                "Mobile Gateway has an invalid paired device record".into(),
            ));
        }
        Ok(address)
    }

    fn legacy_token_is_valid(&self) -> bool {
        !self.auth_token.is_empty()
            && self.auth_token.len() <= 1024
            && self.auth_token.bytes().all(|byte| byte.is_ascii_graphic())
    }
}

fn permitted_bind(address: IpAddr) -> bool {
    if address.is_loopback() {
        return true;
    }
    match address {
        IpAddr::V4(address) => {
            let octets = address.octets();
            octets[0] == 100 && (64..=127).contains(&octets[1])
        }
        IpAddr::V6(address) => {
            let segments = address.segments();
            segments[..3] == [0xfd7a, 0x115c, 0xa1e0]
        }
    }
}

struct PairingTicket {
    secret: String,
    expires_at: Instant,
}

struct Gateway {
    settings: GatewaySettings,
    pairing_tickets: HashMap<String, PairingTicket>,
    opening_remote_projects: HashSet<String>,
    _listener: Option<Task<()>>,
}
impl Global for Gateway {}

#[derive(Clone)]
struct GatewayEvent {
    kind: &'static str,
    project_id: String,
    thread_id: String,
    turn_id: String,
}

struct EventStream {
    id: u64,
    sender: Sender<GatewayEvent>,
}

#[derive(Default)]
struct MobileGatewayEvents {
    next_stream_id: u64,
    streams: HashMap<String, EventStream>,
    panel_subscriptions: HashMap<String, Subscription>,
    conversation_subscriptions: HashMap<String, Subscription>,
    thread_subscriptions: HashMap<String, Subscription>,
}
impl Global for MobileGatewayEvents {}

#[derive(Default)]
struct InitializingThreads(Vec<InitializingThread>);
impl Global for InitializingThreads {}

struct InitializingThread {
    id: String,
    project_id: String,
    roots: Vec<PathBuf>,
    view: WeakEntity<ConversationView>,
}

pub(crate) struct PairingSession {
    pub(crate) payload: String,
    pub(crate) expires_in: Duration,
    pub(crate) bind_address: String,
}

pub fn init(cx: &mut App) {
    cx.default_global::<InitializingThreads>();
    cx.default_global::<MobileGatewayEvents>();
    GatewaySettings::register(cx);
    refresh(cx);
    cx.observe_global::<SettingsStore>(refresh).detach();
}

fn refresh(cx: &mut App) {
    let settings = GatewaySettings::get_global(cx).clone();
    if cx
        .try_global::<Gateway>()
        .is_some_and(|gateway| gateway.settings == settings)
    {
        return;
    }
    // Listener replacement also drops in-flight HTTP requests. Pairing tickets are deliberately
    // process-memory only, so a settings change invalidates an outstanding QR code. Event streams
    // are likewise ephemeral: clearing them prevents a removed device from receiving another event.
    cx.global_mut::<MobileGatewayEvents>().streams.clear();
    cx.set_global(Gateway {
        settings: settings.clone(),
        pairing_tickets: HashMap::new(),
        opening_remote_projects: HashSet::new(),
        _listener: None,
    });
    if !settings.enabled {
        return;
    }
    let address = match settings.address() {
        Ok(address) => address,
        Err(error) => {
            log::error!("{}", error.1);
            return;
        }
    };
    let task = cx.spawn(async move |cx| {
        let listener = match TcpListener::bind(address).await {
            Ok(listener) => listener,
            Err(error) => {
                log::error!("Mobile Gateway could not bind {address}: {error}");
                return;
            }
        };
        log::info!("Mobile Gateway listening on {address}");
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(connection) => connection,
                Err(error) => {
                    log::error!("Mobile Gateway accept failed: {error}");
                    break;
                }
            };
            // A direct notification stream can remain open for hours. Handle each connection in
            // its own foreground task so it cannot block pairing, prompts, or ordinary snapshots.
            let connection_cx = cx.clone();
            connection_cx
                .spawn(async move |cx| serve_connection(stream, cx).await)
                .detach();
        }
    });
    cx.global_mut::<Gateway>()._listener = Some(task);
}

/// Whether closing the final Zed window should leave this process alive as a private mobile host.
/// The listener itself still requires Mobile Gateway to be enabled and correctly bound.
pub(crate) fn is_enabled(cx: &App) -> bool {
    GatewaySettings::get_global(cx).enabled
}

pub(crate) fn begin_pairing(cx: &mut App) -> Result<PairingSession, String> {
    let (address, bind_address) = {
        let gateway = cx
            .try_global::<Gateway>()
            .ok_or_else(|| "Mobile Gateway is not initialized yet.".to_string())?;
        if !gateway.settings.enabled {
            return Err("Enable Mobile Gateway before connecting a device.".into());
        }
        let address = gateway.settings.address().map_err(|error| error.1)?;
        if address.ip().is_loopback() {
            return Err(
                "Set Bind Address to this desktop's Tailscale IP before connecting a phone.".into(),
            );
        }
        (address, gateway.settings.bind_address.clone())
    };

    let ticket_id = uuid::Uuid::new_v4().to_string();
    let secret = random_base64url(32);
    let expires_at = Instant::now() + PAIRING_TICKET_TTL;
    let origin = format!("http://{address}");
    let payload_json = json!({
        "origin": origin,
        "ticket_id": ticket_id,
        "secret": secret,
    });
    let payload_bytes = serde_json::to_vec(&payload_json)
        .map_err(|error| format!("Could not encode the pairing ticket: {error}"))?;
    let payload = format!("ZEDM2:{}", URL_SAFE_NO_PAD.encode(payload_bytes));

    let gateway = cx.global_mut::<Gateway>();
    let now = Instant::now();
    gateway
        .pairing_tickets
        .retain(|_, ticket| ticket.expires_at > now);
    gateway
        .pairing_tickets
        .insert(ticket_id, PairingTicket { secret, expires_at });
    Ok(PairingSession {
        payload,
        expires_in: PAIRING_TICKET_TTL,
        bind_address,
    })
}

#[derive(Debug)]
struct ApiError(u16, String);
type ApiResult = Result<Value, ApiError>;

struct Request {
    method: String,
    segments: Vec<String>,
    authorization: Option<String>,
    body: Vec<u8>,
}

fn valid_opaque(value: &str, maximum_length: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_length
        && value.bytes().all(|byte| byte.is_ascii_graphic())
}

fn valid_label(value: &str) -> bool {
    !value.is_empty()
        && value.chars().count() <= 128
        && value.chars().all(|character| !character.is_control())
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let mut difference = left.len() ^ right.len();
    for index in 0..left.len().max(right.len()) {
        difference |= usize::from(
            left.get(index).copied().unwrap_or_default()
                ^ right.get(index).copied().unwrap_or_default(),
        );
    }
    difference == 0
}

fn requested_bearer(request: &Request) -> Option<&str> {
    request
        .authorization
        .as_deref()?
        .strip_prefix("Bearer ")
        .filter(|credential| valid_opaque(credential, 4096))
}

fn authorize_v1(request: &Request, settings: &GatewaySettings) -> Result<(), ApiError> {
    let Some(credential) = requested_bearer(request) else {
        return Err(ApiError(401, "Unauthorized".into()));
    };
    if !settings.legacy_token_is_valid()
        || !constant_time_eq(credential.as_bytes(), settings.auth_token.as_bytes())
    {
        return Err(ApiError(401, "Unauthorized".into()));
    }
    Ok(())
}

fn authorize_device(
    request: &Request,
    settings: &GatewaySettings,
) -> Result<PairedDevice, ApiError> {
    let Some(credential) = requested_bearer(request) else {
        return Err(ApiError(401, "Unauthorized".into()));
    };
    let credential_hash = credential_hash(credential);
    let mut match_index = None;
    for (index, device) in settings.paired_devices.iter().enumerate() {
        if constant_time_eq(
            credential_hash.as_bytes(),
            device.credential_hash.as_bytes(),
        ) {
            match_index = Some(index);
        }
    }
    match_index
        .and_then(|index| settings.paired_devices.get(index))
        .cloned()
        .ok_or_else(|| ApiError(401, "Unauthorized".into()))
}

fn register_event_stream(
    request: &Request,
    cx: &mut App,
) -> Result<(String, u64, Receiver<GatewayEvent>), ApiError> {
    let settings = cx
        .try_global::<Gateway>()
        .ok_or_else(|| ApiError(503, "Mobile Gateway is not initialized".into()))?
        .settings
        .clone();
    let device = authorize_device(request, &settings)?;
    let (sender, receiver) = mpsc::channel(EVENT_STREAM_CAPACITY);
    let events = cx.global_mut::<MobileGatewayEvents>();
    events.next_stream_id = events.next_stream_id.wrapping_add(1);
    if events.next_stream_id == 0 {
        events.next_stream_id = 1;
    }
    let stream_id = events.next_stream_id;
    // A phone maintains one connection. Replacing a previous stream prevents duplicate Android
    // notifications after a process restart or a network handover.
    events.streams.insert(
        device.id.clone(),
        EventStream {
            id: stream_id,
            sender,
        },
    );
    ensure_event_subscriptions_for_open_projects(cx);
    Ok((device.id, stream_id, receiver))
}

fn unregister_event_stream(device_id: &str, stream_id: u64, cx: &mut App) {
    let streams = &mut cx.global_mut::<MobileGatewayEvents>().streams;
    if streams
        .get(device_id)
        .is_some_and(|stream| stream.id == stream_id)
    {
        streams.remove(device_id);
    }
}

fn remove_device_event_stream(device_id: &str, cx: &mut App) {
    cx.global_mut::<MobileGatewayEvents>()
        .streams
        .remove(device_id);
}

fn publish_agent_event(kind: &'static str, project_id: &str, thread_id: &str, cx: &mut App) {
    let event = GatewayEvent {
        kind,
        project_id: project_id.to_string(),
        thread_id: thread_id.to_string(),
        turn_id: uuid::Uuid::new_v4().to_string(),
    };
    // This is a bounded, live-only delivery channel. A slow or disconnected phone loses the
    // event rather than causing desktop memory growth, a retry, or a later notification.
    cx.global_mut::<MobileGatewayEvents>()
        .streams
        .retain(|_, stream| stream.sender.try_send(event.clone()).is_ok());
}

fn route_segments(path: &str) -> Result<Vec<String>, ApiError> {
    if !path.starts_with('/') || path.contains(['?', '#']) {
        return Err(ApiError(400, "Invalid API path".into()));
    }
    path[1..]
        .split('/')
        .map(|segment| {
            let bytes = segment.as_bytes();
            for (index, byte) in bytes.iter().enumerate() {
                if *byte == b'%'
                    && !bytes
                        .get(index + 1..index + 3)
                        .is_some_and(|bytes| bytes.iter().all(u8::is_ascii_hexdigit))
                {
                    return Err(ApiError(400, "Invalid path encoding".into()));
                }
            }
            let decoded = urlencoding::decode(segment)
                .map_err(|_| ApiError(400, "Invalid path encoding".into()))?
                .into_owned();
            if decoded.is_empty()
                || decoded == "."
                || decoded == ".."
                || decoded.contains(['/', '\\', '\0'])
            {
                return Err(ApiError(400, "Invalid path segment".into()));
            }
            Ok(decoded)
        })
        .collect()
}

fn parse_headers(bytes: &[u8]) -> Result<Option<(Request, usize, usize)>, ApiError> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut parsed = httparse::Request::new(&mut headers);
    let parsed_length = match parsed
        .parse(bytes)
        .map_err(|_| ApiError(400, "Malformed HTTP request".into()))?
    {
        httparse::Status::Partial => return Ok(None),
        httparse::Status::Complete(length) => length,
    };
    if parsed_length > MAX_HEADERS {
        return Err(ApiError(431, "Headers too large".into()));
    }
    if parsed
        .headers
        .iter()
        .any(|header| header.name.eq_ignore_ascii_case("origin"))
    {
        return Err(ApiError(403, "Browser origins are not supported".into()));
    }
    if parsed
        .headers
        .iter()
        .any(|header| header.name.eq_ignore_ascii_case("transfer-encoding"))
    {
        return Err(ApiError(
            400,
            "Transfer-Encoding is not supported; send Content-Length".into(),
        ));
    }
    let authorizations = parsed
        .headers
        .iter()
        .filter(|header| header.name.eq_ignore_ascii_case("authorization"))
        .collect::<Vec<_>>();
    if authorizations.len() > 1 {
        return Err(ApiError(400, "Duplicate Authorization header".into()));
    }
    let authorization = authorizations
        .first()
        .map(|header| {
            std::str::from_utf8(header.value)
                .map(str::to_owned)
                .map_err(|_| ApiError(400, "Authorization must be ASCII".into()))
        })
        .transpose()?;
    let lengths = parsed
        .headers
        .iter()
        .filter(|header| header.name.eq_ignore_ascii_case("content-length"))
        .collect::<Vec<_>>();
    if lengths.len() > 1 {
        return Err(ApiError(400, "Duplicate Content-Length".into()));
    }
    let length = match lengths.first() {
        Some(header) => std::str::from_utf8(header.value)
            .ok()
            .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| ApiError(400, "Invalid Content-Length".into()))?,
        None => 0,
    };
    if length > MAX_BODY {
        return Err(ApiError(413, "Request body exceeds 64 KiB".into()));
    }
    let method = parsed.method.unwrap_or_default().to_string();
    let segments = route_segments(parsed.path.unwrap_or_default())?;
    Ok(Some((
        Request {
            method,
            segments,
            authorization,
            body: Vec::new(),
        },
        parsed_length,
        length,
    )))
}

async fn read_request(stream: &mut TcpStream) -> Result<Request, ApiError> {
    let mut bytes = Vec::new();
    let (mut request, header_length, body_length) = loop {
        if let Some(parsed) = parse_headers(&bytes)? {
            break parsed;
        }
        if bytes.len() >= MAX_HEADERS {
            return Err(ApiError(431, "Headers too large".into()));
        }
        let mut buffer = [0; 2048];
        let length = stream
            .read(&mut buffer)
            .await
            .map_err(|_| ApiError(400, "Failed reading request".into()))?;
        if length == 0 {
            return Err(ApiError(400, "Incomplete request".into()));
        }
        bytes.extend_from_slice(&buffer[..length]);
    };
    while bytes.len() < header_length + body_length {
        let mut buffer = [0; 4096];
        let remaining = (header_length + body_length - bytes.len()).min(buffer.len());
        let length = stream
            .read(&mut buffer[..remaining])
            .await
            .map_err(|_| ApiError(400, "Failed reading body".into()))?;
        if length == 0 {
            return Err(ApiError(400, "Incomplete request body".into()));
        }
        bytes.extend_from_slice(&buffer[..length]);
    }
    request.body = bytes[header_length..header_length + body_length].to_vec();
    Ok(request)
}

fn encode_response(mut status: u16, body: Value) -> Vec<u8> {
    let mut body = body.to_string();
    if body.len() > MAX_SNAPSHOT {
        status = 413;
        body =
            json!({"error": "Snapshot exceeds 2 MiB; inspect this thread on desktop"}).to_string();
    }
    let challenge = if status == 401 {
        "WWW-Authenticate: Bearer\r\n"
    } else {
        ""
    };
    format!("HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n{challenge}\r\n{body}", body.len()).into_bytes()
}

fn is_event_stream_request(request: &Request) -> bool {
    request.method == "GET"
        && request
            .segments
            .iter()
            .map(String::as_str)
            .eq(["v2", "events"])
}

async fn write_response(stream: &mut TcpStream, result: ApiResult) {
    let (status, body) = match result {
        Ok(body) => (200, body),
        Err(ApiError(status, error)) => (status, json!({"error": error})),
    };
    let response = encode_response(status, body);
    let result = smol::future::or(async { stream.write_all(&response).await }, async {
        smol::Timer::after(Duration::from_secs(5)).await;
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "response timed out",
        ))
    })
    .await;
    if let Err(error) = result {
        log::debug!("Mobile Gateway response failed: {error}");
    }
}

fn encode_sse_event(event: &GatewayEvent) -> Vec<u8> {
    let data = json!({
        "project_id": &event.project_id,
        "thread_id": &event.thread_id,
        // This is a notification event ID, intentionally not a requestable or persistent Agent ID.
        "turn_id": &event.turn_id,
    });
    format!("event: {}\ndata: {data}\n\n", event.kind).into_bytes()
}

async fn serve_event_stream(
    stream: &mut TcpStream,
    device_id: String,
    stream_id: u64,
    mut receiver: Receiver<GatewayEvent>,
    cx: &AsyncApp,
) {
    let headers = b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nX-Accel-Buffering: no\r\nConnection: keep-alive\r\n\r\n";
    if let Err(error) = stream.write_all(headers).await {
        log::debug!("Mobile Gateway event stream could not start: {error}");
        cx.update(|cx| unregister_event_stream(&device_id, stream_id, cx));
        return;
    }

    loop {
        enum NextEvent {
            Event(Option<GatewayEvent>),
            Heartbeat,
        }
        let next = smol::future::or(async { NextEvent::Event(receiver.next().await) }, async {
            smol::Timer::after(EVENT_STREAM_HEARTBEAT).await;
            NextEvent::Heartbeat
        })
        .await;
        let payload = match next {
            NextEvent::Event(Some(event)) => encode_sse_event(&event),
            NextEvent::Event(None) => break,
            NextEvent::Heartbeat => {
                cx.update(ensure_event_subscriptions_for_open_projects);
                b": keep-alive\n\n".to_vec()
            }
        };
        if let Err(error) = stream.write_all(&payload).await {
            log::debug!("Mobile Gateway event stream ended: {error}");
            break;
        }
    }
    cx.update(|cx| unregister_event_stream(&device_id, stream_id, cx));
}

async fn serve_connection(mut stream: TcpStream, cx: &mut AsyncApp) {
    let request = smol::future::or(read_request(&mut stream), async {
        smol::Timer::after(Duration::from_secs(5)).await;
        Err(ApiError(408, "Request timed out".into()))
    })
    .await;

    match request {
        Ok(request) if is_event_stream_request(&request) => {
            match cx.update(|cx| register_event_stream(&request, cx)) {
                Ok((device_id, stream_id, receiver)) => {
                    serve_event_stream(&mut stream, device_id, stream_id, receiver, cx).await;
                }
                Err(error) => write_response(&mut stream, Err(error)).await,
            }
        }
        Ok(request) => write_response(&mut stream, dispatch_request(request, cx).await).await,
        Err(error) => write_response(&mut stream, Err(error)).await,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PairClaim {
    ticket_id: String,
    secret: String,
    device_label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateThread {
    project_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Prompt {
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalResolution {
    decision: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelSelection {
    provider_id: String,
    model_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ThreadConfigurationUpdate {
    model: Option<ModelSelection>,
    thinking_enabled: Option<bool>,
    thinking_effort: Option<Option<String>>,
}

fn body<T: serde::de::DeserializeOwned>(request: &Request) -> Result<T, ApiError> {
    serde_json::from_slice(&request.body)
        .map_err(|_| ApiError(400, "Invalid JSON request body".into()))
}

fn random_base64url(byte_count: usize) -> String {
    let mut bytes = vec![0; byte_count];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn credential_hash(credential: &str) -> String {
    format!("{:x}", Sha256::digest(credential.as_bytes()))
}

fn save_paired_devices(devices: Vec<PairedDevice>, cx: &mut App) {
    let content = devices
        .into_iter()
        .map(|device| MobileGatewayPairedDeviceSettingsContent {
            id: device.id,
            label: device.label,
            credential_hash: device.credential_hash,
            created_at: device.created_at,
        })
        .collect::<Vec<_>>();
    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
        settings
            .mobile_gateway
            .get_or_insert_default()
            .paired_devices = Some(content);
    });
}

fn claim_pairing(request: &Request, cx: &mut App) -> ApiResult {
    let claim: PairClaim = body(request)?;
    if !valid_opaque(&claim.ticket_id, 128)
        || !valid_opaque(&claim.secret, 4096)
        || !valid_label(&claim.device_label)
    {
        return Err(ApiError(400, "Invalid pairing claim".into()));
    }
    let ticket = {
        let gateway = cx.global_mut::<Gateway>();
        let now = Instant::now();
        gateway
            .pairing_tickets
            .retain(|_, ticket| ticket.expires_at > now);
        gateway
            .pairing_tickets
            .remove(&claim.ticket_id)
            .ok_or_else(|| ApiError(401, "Pairing ticket is invalid or expired".into()))?
    };
    // Consume the ticket before comparing its secret so a malformed or guessed claim cannot be
    // retried indefinitely. The QR already contains a high-entropy, single-use secret.
    if !constant_time_eq(ticket.secret.as_bytes(), claim.secret.as_bytes()) {
        return Err(ApiError(401, "Pairing ticket is invalid or expired".into()));
    }

    let credential = random_base64url(32);
    let device = PairedDevice {
        id: uuid::Uuid::new_v4().to_string(),
        label: claim.device_label,
        credential_hash: credential_hash(&credential),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let devices = {
        let gateway = cx.global_mut::<Gateway>();
        if gateway.settings.paired_devices.len() >= MAX_PAIRED_DEVICES {
            return Err(ApiError(
                429,
                "Remove a paired device before adding another one".into(),
            ));
        }
        gateway.settings.paired_devices.push(device.clone());
        gateway.settings.paired_devices.clone()
    };
    save_paired_devices(devices, cx);
    Ok(json!({"device_id": device.id, "credential": credential}))
}

fn revoke_device(device: &PairedDevice, cx: &mut App) -> ApiResult {
    let devices = {
        let gateway = cx.global_mut::<Gateway>();
        let previous_length = gateway.settings.paired_devices.len();
        gateway
            .settings
            .paired_devices
            .retain(|candidate| candidate.id != device.id);
        if gateway.settings.paired_devices.len() == previous_length {
            return Err(ApiError(409, "Device has already been revoked".into()));
        }
        gateway.settings.paired_devices.clone()
    };
    remove_device_event_stream(&device.id, cx);
    save_paired_devices(devices, cx);
    Ok(json!({"revoked": true}))
}

fn is_catalog_request(request: &Request) -> bool {
    let segments = request
        .segments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    matches!(
        (request.method.as_str(), segments.as_slice()),
        ("GET", ["v2", "projects", "catalog"]) | ("POST", ["v2", "projects", "catalog", _, "open"])
    )
}

fn authorize_catalog_request(request: &Request, cx: &mut App) -> Result<(), ApiError> {
    if request.segments.first().map(String::as_str) != Some("v2") {
        return Err(ApiError(404, "Unknown route or unsupported method".into()));
    }
    let settings = cx
        .try_global::<Gateway>()
        .ok_or_else(|| ApiError(503, "Mobile Gateway is not initialized".into()))?
        .settings
        .clone();
    authorize_device(request, &settings)?;
    Ok(())
}

async fn load_catalog_projects(cx: &AsyncApp) -> Result<Vec<CatalogProject>, ApiError> {
    let (fs, db) = cx.update(|cx| {
        let app_state = AppState::global(cx);
        (app_state.fs.clone(), WorkspaceDb::global(cx))
    });
    let recent_projects =
        recent_projects::get_recent_projects(None, Some(100), fs.clone(), &db).await;
    // `get_recent_projects` deliberately exposes only local folders to Zed's existing picker.
    // The gateway also needs saved native remote workspaces, but still receives those solely from
    // the desktop database, never from a phone-supplied connection or path.
    let recent_workspaces = db
        .recent_project_workspaces(fs.as_ref())
        .await
        .unwrap_or_default();
    Ok(cx.update(|cx| {
        let opening_remote_projects = cx
            .try_global::<Gateway>()
            .map(|gateway| gateway.opening_remote_projects.clone())
            .unwrap_or_default();
        catalog_projects(
            open_projects(cx),
            recent_projects,
            recent_workspaces,
            &opening_remote_projects,
        )
    }))
}

async fn dispatch_catalog_request(request: Request, cx: &AsyncApp) -> ApiResult {
    cx.update(|cx| authorize_catalog_request(&request, cx))?;
    let segments = request
        .segments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    match (request.method.as_str(), segments.as_slice()) {
        ("GET", ["v2", "projects", "catalog"]) => {
            let projects = load_catalog_projects(cx).await?;
            Ok(json!({"projects": projects.iter().map(project_json).collect::<Vec<_>>() }))
        }
        ("POST", ["v2", "projects", "catalog", project_id, "open"]) => {
            let projects = load_catalog_projects(cx).await?;
            let project = projects
                .into_iter()
                .find(|project| project.id == *project_id)
                .ok_or_else(not_found)?;
            if project.is_open || project.is_opening {
                return Ok(project_json(&project));
            }

            let roots = project.roots.clone();
            match project.location.clone() {
                CatalogProjectLocation::Local => {
                    let task = cx.update(|cx| {
                        workspace::open_paths(
                            &roots,
                            AppState::global(cx),
                            OpenOptions::default(),
                            cx,
                        )
                    });
                    if let Err(error) = task.await {
                        log::warn!("Mobile Gateway could not open selected project: {error:#}");
                        return Err(ApiError(
                            409,
                            "Desktop could not open the selected project".into(),
                        ));
                    }

                    let project = load_catalog_projects(cx)
                        .await?
                        .into_iter()
                        .find(|candidate| candidate.id == *project_id && candidate.is_open)
                        .ok_or_else(|| {
                            ApiError(409, "Desktop is still opening the selected project".into())
                        })?;
                    Ok(project_json(&project))
                }
                CatalogProjectLocation::Remote(mut connection_options) => {
                    let project_id = project.id.clone();
                    let (app_state, already_opening) = cx.update(|cx| {
                        if let RemoteConnectionOptions::Ssh(options) = &mut connection_options {
                            recent_projects::RemoteSettings::get_global(cx)
                                .fill_connection_options_from_settings(options);
                        }
                        let already_opening = !cx
                            .global_mut::<Gateway>()
                            .opening_remote_projects
                            .insert(project_id.clone());
                        (AppState::global(cx), already_opening)
                    });
                    if !already_opening {
                        // The native remote connection UI may wait for a host-key, MFA, or password
                        // interaction. The phone gets an immediate opening response and polls the
                        // catalog; it must not be held open while a desktop-only prompt is displayed.
                        cx.update(|cx| {
                            cx.spawn(async move |cx| {
                                if let Err(error) = recent_projects::open_remote_project(
                                    connection_options,
                                    roots,
                                    app_state,
                                    OpenOptions::default(),
                                    cx,
                                )
                                .await
                                {
                                    log::warn!(
                                        "Mobile Gateway could not open selected remote project: {error:#}"
                                    );
                                }
                                cx.update(|cx| {
                                    cx.global_mut::<Gateway>()
                                        .opening_remote_projects
                                        .remove(&project_id);
                                });
                            })
                            .detach();
                        });
                    }
                    let mut response = project_json(&project);
                    response["opening"] = Value::Bool(true);
                    Ok(response)
                }
            }
        }
        _ => Err(ApiError(404, "Unknown route or unsupported method".into())),
    }
}

async fn dispatch_request(request: Request, cx: &AsyncApp) -> ApiResult {
    if is_catalog_request(&request) {
        dispatch_catalog_request(request, cx).await
    } else {
        cx.update(|cx| dispatch(request, cx))
    }
}

fn dispatch(request: Request, cx: &mut App) -> ApiResult {
    let version = request
        .segments
        .first()
        .map(String::as_str)
        .ok_or_else(|| ApiError(404, "Unknown route or unsupported method".into()))?;
    let settings = cx
        .try_global::<Gateway>()
        .ok_or_else(|| ApiError(503, "Mobile Gateway is not initialized".into()))?
        .settings
        .clone();

    match version {
        "v1" => authorize_v1(&request, &settings)?,
        "v2" => {
            if request.method == "POST"
                && request
                    .segments
                    .iter()
                    .map(String::as_str)
                    .eq(["v2", "pair", "claim"])
            {
                return claim_pairing(&request, cx);
            }
            let device = authorize_device(&request, &settings)?;
            if request.method == "POST"
                && request
                    .segments
                    .iter()
                    .map(String::as_str)
                    .eq(["v2", "devices", "current", "revoke"])
            {
                return revoke_device(&device, cx);
            }
        }
        _ => return Err(ApiError(404, "Unknown route or unsupported method".into())),
    }

    dispatch_workspace(&request, cx)
}

struct OpenProject {
    id: String,
    roots: Vec<PathBuf>,
    is_remote: bool,
    location: CatalogProjectLocation,
    workspace: Entity<Workspace>,
    window: WindowHandle<MultiWorkspace>,
}

#[derive(Clone)]
enum CatalogProjectLocation {
    Local,
    Remote(RemoteConnectionOptions),
}

/// A project which the paired phone may select. The phone never supplies paths or connection
/// options: local roots and remote connection data come only from Zed's active workspaces and
/// persisted recent-workspace database.
#[derive(Clone)]
struct CatalogProject {
    id: String,
    name: String,
    path: String,
    roots: Vec<PathBuf>,
    location: CatalogProjectLocation,
    is_open: bool,
    is_opening: bool,
}

fn project_id(roots: &[PathBuf]) -> String {
    let mut roots = roots
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    roots.sort_unstable();
    format!("project-{:x}", Sha256::digest(roots.join("\n").as_bytes()))
}

fn remote_project_id(connection: &RemoteConnectionOptions, roots: &[PathBuf]) -> String {
    let mut roots = roots
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    roots.sort_unstable();
    // This stays entirely on desktop. The digest prevents a remote path on one server from
    // colliding with the same path on another server and never exposes connection options.
    let source = serde_json::to_vec(connection).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(source);
    hasher.update(b"\n");
    hasher.update(roots.join("\n").as_bytes());
    format!("remote-project-{:x}", hasher.finalize())
}

fn project_name(roots: &[PathBuf]) -> String {
    roots
        .first()
        .and_then(|path| path.file_name())
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn project_json(project: &CatalogProject) -> Value {
    let (kind, connection_name) = match &project.location {
        CatalogProjectLocation::Local => ("local", None),
        CatalogProjectLocation::Remote(connection) => (
            connection.connection_type(),
            Some(connection.display_name()),
        ),
    };
    json!({
        "id": project.id,
        "name": project.name,
        "path": project.path,
        "kind": kind,
        "connection_name": connection_name,
        "is_open": project.is_open,
        "opening": project.is_opening,
    })
}

fn open_projects(cx: &App) -> Vec<OpenProject> {
    let mut projects = Vec::new();
    for window in cx
        .windows()
        .into_iter()
        .filter_map(|window| window.downcast::<MultiWorkspace>())
    {
        let Ok(multi_workspace) = window.read(cx) else {
            continue;
        };
        for workspace in multi_workspace.workspaces() {
            let project_entity = workspace.read(cx).project().clone();
            let project = project_entity.read(cx);
            if project.is_via_collab() {
                continue;
            }
            let is_remote = !project.is_local();
            let location = if is_remote {
                let Some(connection) = project.remote_connection_options(cx) else {
                    continue;
                };
                CatalogProjectLocation::Remote(connection)
            } else {
                CatalogProjectLocation::Local
            };
            let roots = if is_remote {
                project
                    .visible_worktrees(cx)
                    .map(|tree| tree.read(cx).abs_path().to_path_buf())
                    .collect::<Vec<_>>()
            } else {
                let roots = project
                    .visible_worktrees(cx)
                    .map(|tree| std::fs::canonicalize(tree.read(cx).abs_path()))
                    .collect::<std::io::Result<Vec<_>>>();
                let Ok(roots) = roots else {
                    continue;
                };
                roots
            };
            if roots.is_empty() {
                continue;
            }
            let id = match &location {
                CatalogProjectLocation::Local => project_id(&roots),
                CatalogProjectLocation::Remote(connection) => remote_project_id(connection, &roots),
            };
            projects.push(OpenProject {
                id,
                roots,
                is_remote,
                location,
                workspace: workspace.clone(),
                window,
            });
        }
    }
    projects
}

fn catalog_projects(
    open_projects: Vec<OpenProject>,
    recent_projects: Vec<recent_projects::RecentProjectEntry>,
    recent_workspaces: Vec<workspace::RecentWorkspace>,
    opening_remote_projects: &HashSet<String>,
) -> Vec<CatalogProject> {
    // The desktop maintains the authoritative recent-project timestamps. An open workspace that
    // has not reached that database yet is treated as most recent for this response.
    let now = chrono::Utc::now();
    let mut opened_at = HashMap::new();
    let mut projects = open_projects
        .into_iter()
        .map(|project| {
            opened_at.insert(project.id.clone(), now);
            let mut name = project_name(&project.roots);
            if name.is_empty()
                && let CatalogProjectLocation::Remote(connection) = &project.location
            {
                name = connection.display_name();
            }
            CatalogProject {
                id: project.id,
                name,
                path: project
                    .roots
                    .iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
                roots: project.roots,
                location: project.location,
                is_open: true,
                is_opening: false,
            }
        })
        .collect::<Vec<_>>();

    for recent in recent_projects {
        // Recent-project lookup confirms that these paths exist. Canonicalizing again provides a
        // stable key that matches the same project once it is opened in a workspace.
        let roots = recent
            .paths
            .iter()
            .map(std::fs::canonicalize)
            .collect::<std::io::Result<Vec<_>>>();
        let Ok(roots) = roots else {
            continue;
        };
        if roots.is_empty() {
            continue;
        }
        let id = project_id(&roots);
        opened_at.insert(id.clone(), recent.timestamp);
        if projects.iter().any(|project| project.id == id) {
            continue;
        }
        projects.push(CatalogProject {
            id,
            name: recent.name.to_string(),
            path: recent.full_path.to_string(),
            roots,
            location: CatalogProjectLocation::Local,
            is_open: false,
            is_opening: false,
        });
    }

    for recent in recent_workspaces {
        let SerializedWorkspaceLocation::Remote(connection) = recent.location else {
            continue;
        };
        let roots = recent.paths.paths().to_vec();
        if roots.is_empty() {
            continue;
        }
        let id = remote_project_id(&connection, &roots);
        opened_at.insert(id.clone(), recent.timestamp);
        if projects.iter().any(|project| project.id == id) {
            continue;
        }
        let mut name = project_name(&roots);
        if name.is_empty() {
            name = connection.display_name();
        }
        let is_opening = opening_remote_projects.contains(&id);
        projects.push(CatalogProject {
            id,
            name,
            path: roots
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            roots,
            location: CatalogProjectLocation::Remote(connection),
            is_open: false,
            is_opening,
        });
    }

    projects.sort_by(|left, right| {
        opened_at
            .get(&right.id)
            .cmp(&opened_at.get(&left.id))
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
    });
    projects
}

fn ensure_event_subscriptions_for_open_projects(cx: &mut App) {
    for project in open_projects(cx) {
        let Some(panel) = project.workspace.read(cx).panel::<AgentPanel>(cx) else {
            continue;
        };
        ensure_agent_panel_event_subscription(&project.id, &panel, cx);
        ensure_conversation_event_subscriptions(&project.id, &panel, cx);
    }
}

fn ensure_agent_panel_event_subscription(
    project_id: &str,
    panel: &Entity<AgentPanel>,
    cx: &mut App,
) {
    if cx
        .global::<MobileGatewayEvents>()
        .panel_subscriptions
        .contains_key(project_id)
    {
        return;
    }

    let project_id = project_id.to_string();
    let callback_project_id = project_id.clone();
    let subscription = cx.subscribe(panel, move |panel, event: &AgentPanelEvent, cx| {
        if matches!(event, AgentPanelEvent::ActiveViewChanged) {
            ensure_conversation_event_subscriptions(&callback_project_id, &panel, cx);
        }
    });
    cx.global_mut::<MobileGatewayEvents>()
        .panel_subscriptions
        .insert(project_id, subscription);
}

fn ensure_conversation_event_subscriptions(
    project_id: &str,
    panel: &Entity<AgentPanel>,
    cx: &mut App,
) {
    for view in panel.read(cx).conversation_views() {
        let (conversation_id, is_native) = {
            let view = view.read(cx);
            (
                view.parent_id().to_key_string(),
                matches!(view.agent_key(), Agent::NativeAgent),
            )
        };
        if !is_native {
            continue;
        }

        ensure_conversation_thread_subscription(project_id, &view, cx);
        if cx
            .global::<MobileGatewayEvents>()
            .conversation_subscriptions
            .contains_key(&conversation_id)
        {
            continue;
        }

        let project_id = project_id.to_string();
        let subscription = cx.subscribe(&view, move |view, _: &StateChange, cx| {
            ensure_conversation_thread_subscription(&project_id, &view, cx);
        });
        cx.global_mut::<MobileGatewayEvents>()
            .conversation_subscriptions
            .insert(conversation_id, subscription);
    }
}

fn ensure_conversation_thread_subscription(
    project_id: &str,
    view: &Entity<ConversationView>,
    cx: &mut App,
) {
    let Some(thread_view) = view.read(cx).root_thread_view() else {
        return;
    };
    let thread_id = view.read(cx).parent_id().to_key_string();
    let thread = thread_view.read(cx).thread.clone();
    ensure_thread_event_subscription(&thread_id, project_id, &thread, cx);
}

fn ensure_thread_event_subscription(
    thread_id: &str,
    project_id: &str,
    thread: &Entity<acp_thread::AcpThread>,
    cx: &mut App,
) {
    if cx
        .global::<MobileGatewayEvents>()
        .thread_subscriptions
        .contains_key(thread_id)
    {
        return;
    }

    let thread_id = thread_id.to_string();
    let event_thread_id = thread_id.clone();
    let project_id = project_id.to_string();
    let subscription = cx.subscribe(thread, move |_, event: &AcpThreadEvent, cx| {
        let kind = match event {
            AcpThreadEvent::Stopped(acp::StopReason::EndTurn) => Some("thread_completed"),
            // Cancellation is initiated by the user and is not a completed task or an actionable
            // failure notification. All other terminal stop reasons are surfaced as failures.
            AcpThreadEvent::Stopped(acp::StopReason::Cancelled) => None,
            AcpThreadEvent::Stopped(_) | AcpThreadEvent::Error | AcpThreadEvent::LoadError(_) => {
                Some("thread_failed")
            }
            _ => None,
        };
        if let Some(kind) = kind {
            publish_agent_event(kind, &project_id, &event_thread_id, cx);
        }
    });
    cx.global_mut::<MobileGatewayEvents>()
        .thread_subscriptions
        .insert(thread_id, subscription);
}

fn conversation_allowed(view: &ConversationView, project: &OpenProject, cx: &App) -> bool {
    if !matches!(view.agent_key(), Agent::NativeAgent) {
        return false;
    }
    if view.root_thread_view().is_none() {
        return cx.global::<InitializingThreads>().0.iter().any(|pending| {
            pending.id == view.parent_id().to_key_string()
                && pending.project_id == project.id
                && pending.roots == project.roots
        });
    }
    let Some(native) = view.as_native_thread(cx) else {
        return false;
    };
    if native.read(cx).project() != project.workspace.read(cx).project() {
        return false;
    }
    let Some(thread_view) = view.root_thread_view() else {
        return false;
    };
    let thread = thread_view.read(cx).thread.read(cx);
    thread.work_dirs().is_none_or(|paths| {
        !paths.is_empty()
            && paths
                .paths()
                .iter()
                .all(|path| verified_project_path(path, project).is_some())
    })
}

fn summary(view: &ConversationView, cx: &App) -> Value {
    let status = view
        .root_thread_view()
        .map(|thread_view| {
            let thread = thread_view.read(cx).thread.read(cx);
            if thread.is_waiting_for_confirmation() {
                "waiting_for_approval"
            } else if thread.had_error() {
                "error"
            } else if thread.status() == ThreadStatus::Generating {
                "running"
            } else {
                "idle"
            }
        })
        .unwrap_or(if view.is_loading() {
            "initializing"
        } else {
            "error"
        });
    json!({"id": view.parent_id().to_key_string(), "title": view.title(cx).to_string(), "status": status})
}

fn legacy_messages(entries: &[AgentThreadEntry], cx: &App) -> Result<Vec<Value>, ApiError> {
    let mut size = 0;
    let mut messages = Vec::new();
    for entry in entries {
        let (role, text) = match entry {
            AgentThreadEntry::UserMessage(message) => {
                ("user", message.content.to_markdown(cx).to_string())
            }
            AgentThreadEntry::AssistantMessage(message) => (
                "assistant",
                message
                    .chunks
                    .iter()
                    .filter_map(|chunk| {
                        if let AssistantMessageChunk::Message { block, .. } = chunk {
                            Some(block.to_markdown(cx))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            ),
            _ => continue,
        };
        size += text.len();
        if size > MAX_SNAPSHOT {
            return Err(ApiError(
                413,
                "Thread is too large; inspect it on desktop".into(),
            ));
        }
        messages.push(json!({"role": role, "text": text}));
    }
    Ok(messages)
}

fn tool_status(status: &ToolCallStatus) -> &'static str {
    match status {
        ToolCallStatus::Pending => "pending",
        ToolCallStatus::WaitingForConfirmation { .. } => "waiting_for_approval",
        ToolCallStatus::InProgress => "running",
        ToolCallStatus::Completed => "completed",
        ToolCallStatus::Failed => "failed",
        ToolCallStatus::Rejected => "rejected",
        ToolCallStatus::Canceled => "canceled",
    }
}

fn transcript(entries: &[AgentThreadEntry], cx: &App) -> Result<Vec<Value>, ApiError> {
    let mut result = Vec::with_capacity(entries.len());
    let mut size = 0;
    for (index, entry) in entries.iter().enumerate() {
        let value = match entry {
            AgentThreadEntry::UserMessage(message) => json!({
                "index": index,
                "kind": "user_message",
                "text": message.content.to_markdown(cx),
            }),
            AgentThreadEntry::AssistantMessage(message) => json!({
                "index": index,
                "kind": "assistant_message",
                "chunks": message.chunks.iter().map(|chunk| match chunk {
                    AssistantMessageChunk::Message { block, .. } => json!({"kind":"message","text":block.to_markdown(cx)}),
                    AssistantMessageChunk::Thought { block, .. } => json!({"kind":"thought","text":block.to_markdown(cx)}),
                }).collect::<Vec<_>>(),
            }),
            AgentThreadEntry::ToolCall(call) => json!({
                "index": index,
                "kind": "tool_call",
                "tool_call_id": call.id.0.to_string(),
                "title": call.label.read(cx).source(),
                "tool_name": call.tool_name.as_ref().map(ToString::to_string),
                "status": tool_status(&call.status),
                "content": call.content.iter().map(|content| content.to_markdown(cx)).collect::<Vec<_>>(),
                "raw_input": call.raw_input,
                "raw_output": call.raw_output,
            }),
            AgentThreadEntry::Elicitation(_) => json!({
                "index": index,
                "kind": "elicitation",
                "text": entry.to_markdown(cx),
            }),
            AgentThreadEntry::CompletedPlan(plan) => json!({
                "index": index,
                "kind": "completed_plan",
                "entries": plan.iter().map(|item| json!({
                    "text": item.content.read(cx).source(),
                    "priority": format!("{:?}", item.priority),
                    "status": format!("{:?}", item.status),
                })).collect::<Vec<_>>(),
            }),
            AgentThreadEntry::ContextCompaction(compaction) => json!({
                "index": index,
                "kind": "context_compaction",
                "id": compaction.id.0.to_string(),
                "status": format!("{:?}", compaction.status),
                "summary": compaction.summary.iter().map(|block| block.to_markdown(cx)).collect::<Vec<_>>(),
                "error": compaction.error.as_ref().map(|error| error.read(cx).source()),
            }),
        };
        size += value.to_string().len();
        if size > MAX_SNAPSHOT {
            return Err(ApiError(
                413,
                "Thread transcript is too large; inspect it on desktop".into(),
            ));
        }
        result.push(value);
    }
    Ok(result)
}

fn approvals(entries: &[AgentThreadEntry], cx: &App) -> Value {
    let values = entries
        .iter()
        .filter_map(|entry| {
            let AgentThreadEntry::ToolCall(call) = entry else {
                return None;
            };
            let ToolCallStatus::WaitingForConfirmation { options, .. } = &call.status else {
                return None;
            };
            Some(json!({
                "tool_call_id": call.id.0.to_string(),
                "title": call.label.read(cx).source(),
                "tool_name": call.tool_name.as_ref().map(ToString::to_string),
                "content": call.content.iter().map(|content| content.to_markdown(cx)).collect::<Vec<_>>(),
                "raw_input": call.raw_input,
                "allow_once_available": options.allow_once_option_id().is_some(),
                "deny_once_available": options.deny_once_option_id().is_some(),
            }))
        })
        .collect::<Vec<_>>();
    json!({"approvals": values})
}

fn resolve_approval(
    thread: &Entity<acp_thread::AcpThread>,
    tool_call_id: &str,
    request: &Request,
    cx: &mut App,
) -> ApiResult {
    let resolution: ApprovalResolution = body(request)?;
    let id = acp::ToolCallId::new(tool_call_id.to_string());
    thread.update(cx, |thread, cx| {
        let Some((_, call)) = thread.tool_call(&id) else {
            return Err(ApiError(409, "Approval is no longer pending".into()));
        };
        let ToolCallStatus::WaitingForConfirmation { options, .. } = &call.status else {
            return Err(ApiError(409, "Approval is no longer pending".into()));
        };
        let outcome = match resolution.decision.as_str() {
            "allow" => options.allow_once_option_id().map(|option_id| {
                acp_thread::SelectedPermissionOutcome::new(
                    option_id,
                    acp::PermissionOptionKind::AllowOnce,
                )
            }),
            "deny" => options.deny_once_option_id().map(|option_id| {
                acp_thread::SelectedPermissionOutcome::new(
                    option_id,
                    acp::PermissionOptionKind::RejectOnce,
                )
            }),
            _ => return Err(ApiError(400, "decision must be allow or deny".into())),
        }
        .ok_or_else(|| ApiError(409, "That one-time approval option is unavailable".into()))?;
        thread.authorize_tool_call(id, outcome, cx);
        Ok(json!({"accepted": true}))
    })
}

fn available_models(cx: &App) -> Value {
    let models = LanguageModelRegistry::read_global(cx)
        .available_models(cx)
        .map(|model| {
            json!({
                "provider_id": model.provider_id().0,
                "provider_name": model.provider_name().0,
                "model_id": model.id().0,
                "model_name": model.name().0,
                "supports_thinking": model.supports_thinking(),
                "supports_disabling_thinking": model.supports_disabling_thinking(),
                "effort_levels": model.supported_effort_levels().iter().map(|level| json!({
                    "name": level.name,
                    "value": level.value,
                    "is_default": level.is_default,
                })).collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    json!({"models": models})
}

fn thread_configuration(native: &Entity<agent::Thread>, cx: &App) -> Value {
    let thread = native.read(cx);
    let model = thread.model().map(|model| {
        json!({
            "provider_id": model.provider_id().0,
            "provider_name": model.provider_name().0,
            "model_id": model.id().0,
            "model_name": model.name().0,
            "supports_thinking": model.supports_thinking(),
            "supports_disabling_thinking": model.supports_disabling_thinking(),
            "effort_levels": model.supported_effort_levels().iter().map(|level| json!({
                "name": level.name,
                "value": level.value,
                "is_default": level.is_default,
            })).collect::<Vec<_>>(),
        })
    });
    json!({
        "model": model,
        "thinking_enabled": thread.thinking_enabled(),
        "thinking_effort": thread.thinking_effort(),
    })
}

fn update_thread_configuration(
    native: &Entity<agent::Thread>,
    thread: &Entity<acp_thread::AcpThread>,
    request: &Request,
    cx: &mut App,
) -> ApiResult {
    let update: ThreadConfigurationUpdate = body(request)?;
    if update.model.is_none()
        && update.thinking_enabled.is_none()
        && update.thinking_effort.is_none()
    {
        return Err(ApiError(
            400,
            "No thread configuration fields were supplied".into(),
        ));
    }
    let is_busy = {
        let thread = thread.read(cx);
        thread.status() == ThreadStatus::Generating || thread.is_waiting_for_confirmation()
    };
    if is_busy {
        return Err(ApiError(
            409,
            "Wait for the current Agent turn or approval before changing its configuration".into(),
        ));
    }

    let selected_model = if let Some(selection) = update.model.as_ref() {
        if !valid_opaque(&selection.provider_id, 256) || !valid_opaque(&selection.model_id, 256) {
            return Err(ApiError(400, "Invalid model selection".into()));
        }
        LanguageModelRegistry::read_global(cx)
            .available_models(cx)
            .find(|model| {
                model.provider_id().0.as_ref() == selection.provider_id
                    && model.id().0.as_ref() == selection.model_id
            })
            .ok_or_else(|| {
                ApiError(
                    409,
                    "That model is unavailable or its provider is not authenticated".into(),
                )
            })?
    } else {
        native.read(cx).model().cloned().ok_or_else(|| {
            ApiError(
                409,
                "Select and configure an Agent model on desktop first".into(),
            )
        })?
    };

    if let Some(enabled) = update.thinking_enabled {
        if enabled && !selected_model.supports_thinking() {
            return Err(ApiError(
                409,
                "The selected model does not support thinking".into(),
            ));
        }
        if !enabled
            && selected_model.supports_thinking()
            && !selected_model.supports_disabling_thinking()
        {
            return Err(ApiError(
                409,
                "The selected model requires thinking and cannot disable it".into(),
            ));
        }
    }
    if let Some(Some(effort)) = update.thinking_effort.as_ref() {
        if !selected_model.supports_thinking()
            || !selected_model
                .supported_effort_levels()
                .iter()
                .any(|level| level.value.as_ref() == effort)
        {
            return Err(ApiError(
                409,
                "The selected model does not support that thinking effort".into(),
            ));
        }
    }

    native.update(cx, |thread, cx| {
        if update.model.is_some() {
            thread.set_model(selected_model, cx);
        }
        if let Some(enabled) = update.thinking_enabled {
            thread.set_thinking_enabled(enabled, cx);
        }
        if let Some(effort) = update.thinking_effort {
            thread.set_thinking_effort(effort, cx);
        }
    });
    Ok(thread_configuration(native, cx))
}

fn dispatch_workspace(request: &Request, cx: &mut App) -> ApiResult {
    let segments = request
        .segments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let version = segments
        .first()
        .copied()
        .ok_or_else(|| ApiError(404, "Unknown route or unsupported method".into()))?;
    if request.method == "GET" && segments.as_slice() == [version, "status"] {
        return Ok(
            json!({"api_version": if version == "v2" { 2 } else { 1 }, "name": "Zed-Desktop"}),
        );
    }
    let projects = open_projects(cx);
    match (request.method.as_str(), segments.as_slice()) {
        ("GET", [_, "projects"]) => Ok(json!({"projects": projects.iter().map(|project| {
            let path = project.roots.first().map(PathBuf::as_path).unwrap_or_else(|| Path::new(""));
            json!({"id": project.id, "name": path.file_name().unwrap_or_default().to_string_lossy(), "path": path.to_string_lossy()})
        }).collect::<Vec<_>>()})),
        ("GET", ["v2", "projects", project_id, "models"]) => {
            projects
                .iter()
                .find(|project| project.id == *project_id)
                .ok_or_else(not_found)?;
            Ok(available_models(cx))
        }
        ("GET", [_, "projects", project_id, "threads"]) => {
            let project = projects
                .iter()
                .find(|project| project.id == *project_id)
                .ok_or_else(not_found)?;
            let panel = project
                .workspace
                .read(cx)
                .panel::<AgentPanel>(cx)
                .ok_or_else(agent_unavailable)?;
            let threads = panel
                .read(cx)
                .conversation_views()
                .into_iter()
                .filter_map(|view| {
                    let view = view.read(cx);
                    conversation_allowed(view, project, cx).then(|| summary(view, cx))
                })
                .collect::<Vec<_>>();
            Ok(json!({"threads": threads}))
        }
        ("POST", [_, "threads"]) => {
            let create: CreateThread = body(request)?;
            let pending = cx
                .global::<InitializingThreads>()
                .0
                .iter()
                .filter(|pending| {
                    pending
                        .view
                        .upgrade()
                        .is_some_and(|view| view.read(cx).root_thread_view().is_none())
                })
                .map(|pending| pending.id.clone())
                .collect::<Vec<_>>();
            cx.global_mut::<InitializingThreads>()
                .0
                .retain(|thread| pending.contains(&thread.id));
            if pending.len() >= 128 {
                return Err(ApiError(
                    429,
                    "Too many uninitialized threads; close failed threads on desktop".into(),
                ));
            }
            let project = projects
                .iter()
                .find(|project| project.id == create.project_id)
                .ok_or_else(not_found)?;
            ensure_can_prompt(project, cx)?;
            let panel = project
                .workspace
                .read(cx)
                .panel::<AgentPanel>(cx)
                .ok_or_else(agent_unavailable)?;
            project
                .window
                .update(cx, |_, window, cx| {
                    let id = panel.update(cx, |panel, cx| {
                        panel.create_thread_with_options(
                            CreateThreadOptions {
                                agent: Some(Agent::NativeAgent),
                                ..Default::default()
                            },
                            AgentThreadSource::AgentPanel,
                            window,
                            cx,
                        )
                    });
                    let view = panel
                        .read(cx)
                        .conversation_view_for_id(&id, cx)
                        .cloned()
                        .ok_or_else(agent_unavailable)?;
                    cx.global_mut::<InitializingThreads>()
                        .0
                        .push(InitializingThread {
                            id: id.to_key_string(),
                            project_id: project.id.clone(),
                            roots: project.roots.clone(),
                            view: view.downgrade(),
                        });
                    Ok(summary(view.read(cx), cx))
                })
                .map_err(|_| agent_unavailable())?
        }
        (method, [version, "threads", thread_id, rest @ ..]) => {
            for project in &projects {
                let Some(panel) = project.workspace.read(cx).panel::<AgentPanel>(cx) else {
                    continue;
                };
                let view = panel
                    .read(cx)
                    .conversation_views()
                    .into_iter()
                    .find(|view| {
                        let view = view.read(cx);
                        view.parent_id().to_key_string() == *thread_id
                            && conversation_allowed(view, project, cx)
                    });
                let Some(view) = view else {
                    continue;
                };
                let Some(thread_view) = view.read(cx).root_thread_view() else {
                    if method == "GET" && rest.is_empty() {
                        let mut result = summary(view.read(cx), cx);
                        result["messages"] = json!([]);
                        if *version == "v2" {
                            result["transcript"] = json!([]);
                        }
                        return Ok(result);
                    }
                    return Err(agent_unavailable());
                };
                let thread = thread_view.read(cx).thread.clone();
                ensure_thread_event_subscription(thread_id, &project.id, &thread, cx);
                return match (method, rest) {
                    ("GET", []) => {
                        let mut result = summary(view.read(cx), cx);
                        result["messages"] = json!(legacy_messages(thread.read(cx).entries(), cx)?);
                        if *version == "v2" {
                            result["transcript"] =
                                json!(transcript(thread.read(cx).entries(), cx)?);
                        }
                        Ok(result)
                    }
                    ("GET", ["transcript"]) if *version == "v2" => {
                        Ok(json!({"entries": transcript(thread.read(cx).entries(), cx)?}))
                    }
                    ("GET", ["approvals"]) if *version == "v2" => {
                        Ok(approvals(thread.read(cx).entries(), cx))
                    }
                    ("POST", ["approvals", tool_call_id, "resolve"]) if *version == "v2" => {
                        resolve_approval(&thread, tool_call_id, request, cx)
                    }
                    ("GET", ["configuration"]) if *version == "v2" => {
                        let native = view
                            .read(cx)
                            .as_native_thread(cx)
                            .ok_or_else(agent_unavailable)?;
                        Ok(thread_configuration(&native, cx))
                    }
                    ("PUT", ["configuration"]) if *version == "v2" => {
                        let native = view
                            .read(cx)
                            .as_native_thread(cx)
                            .ok_or_else(agent_unavailable)?;
                        update_thread_configuration(&native, &thread, request, cx)
                    }
                    ("POST", ["messages"]) => {
                        let prompt: Prompt = body(request)?;
                        if prompt.text.trim().is_empty() {
                            return Err(ApiError(400, "Prompt text must not be empty".into()));
                        }
                        ensure_can_prompt(project, cx)?;
                        if thread.read(cx).status() == ThreadStatus::Generating {
                            return Err(ApiError(409, "Thread is already running".into()));
                        }
                        let native = view
                            .read(cx)
                            .as_native_thread(cx)
                            .ok_or_else(agent_unavailable)?;
                        let model = native.read(cx).model().ok_or_else(|| {
                            ApiError(
                                409,
                                "Select and configure an Agent model on desktop first".into(),
                            )
                        })?;
                        let provider =
                            LanguageModelRegistry::read_global(cx).provider(&model.provider_id());
                        if !provider.is_some_and(|provider| provider.is_authenticated(cx)) {
                            return Err(ApiError(
                                409,
                                "Authenticate the selected model provider on desktop first".into(),
                            ));
                        }
                        let send = thread
                            .update(cx, |thread, cx| thread.send(vec![prompt.text.into()], cx));
                        cx.spawn(async move |_cx| {
                            if let Err(error) = send.await {
                                log::error!("Mobile Gateway agent prompt failed: {error}");
                            }
                        })
                        .detach();
                        Ok(json!({"accepted": true}))
                    }
                    ("POST", ["cancel"]) => {
                        thread_view.update(cx, |view, cx| view.cancel_generation(cx));
                        Ok(json!({"accepted": true}))
                    }
                    ("GET", ["changes"]) => changes(thread.read(cx).entries(), project, cx),
                    _ => Err(ApiError(404, "Unknown route or unsupported method".into())),
                };
            }
            Err(not_found())
        }
        _ => Err(ApiError(404, "Unknown route or unsupported method".into())),
    }
}

fn not_found() -> ApiError {
    ApiError(404, "Project or thread is not open and allowed".into())
}

fn agent_unavailable() -> ApiError {
    ApiError(
        503,
        "Built-in Agent is not ready; open and configure the Agent panel on desktop".into(),
    )
}

fn ensure_can_prompt(project: &OpenProject, cx: &App) -> Result<(), ApiError> {
    if !agent_settings::AgentSettings::get_global(cx).enabled(cx) {
        return Err(ApiError(
            403,
            "The Agent is disabled in desktop settings".into(),
        ));
    }
    let project = project.workspace.read(cx).project().read(cx);
    if project.is_read_only(cx)
        || project::trusted_worktrees::TrustedWorktrees::has_restricted_worktrees(
            &project.worktree_store(),
            cx,
        )
    {
        return Err(ApiError(
            403,
            "Project is read-only or restricted; trust it on desktop first".into(),
        ));
    }
    Ok(())
}

fn verified_path(path: &Path, roots: &[PathBuf]) -> Option<PathBuf> {
    if !path.is_absolute() {
        return None;
    }
    std::fs::canonicalize(path)
        .ok()
        .filter(|path| roots.iter().any(|root| path.starts_with(root)))
}

fn verified_remote_path(path: &Path, roots: &[PathBuf]) -> Option<PathBuf> {
    path.is_absolute()
        .then(|| path.to_path_buf())
        .filter(|path| roots.iter().any(|root| path.starts_with(root)))
}

fn verified_project_path(path: &Path, project: &OpenProject) -> Option<PathBuf> {
    if project.is_remote {
        verified_remote_path(path, &project.roots)
    } else {
        verified_path(path, &project.roots)
    }
}

fn changes(entries: &[AgentThreadEntry], project: &OpenProject, cx: &App) -> ApiResult {
    let mut files = std::collections::BTreeMap::<PathBuf, (String, String)>::new();
    let mut size = 0;
    for diff in entries.iter().flat_map(AgentThreadEntry::diffs) {
        let diff = diff.read(cx);
        let buffer = diff.buffer().read(cx);
        let path = buffer
            .file()
            .and_then(|file| file.as_local())
            .map(|file| file.abs_path(cx))
            .or_else(|| diff.file_path(cx).map(PathBuf::from))
            .ok_or_else(|| ApiError(501, "A diff has no resolvable local file path".into()))?;
        let path = verified_project_path(&path, project).ok_or_else(|| {
            ApiError(
                403,
                "A diff path is outside allowed roots or cannot be verified".into(),
            )
        })?;
        size += diff.base_text().len() + buffer.len();
        if size > MAX_SNAPSHOT {
            return Err(ApiError(
                413,
                "Diff snapshot is too large; review on desktop".into(),
            ));
        }
        let new_text = buffer.text();
        files
            .entry(path)
            .and_modify(|texts| texts.1 = new_text.clone())
            .or_insert_with(|| (diff.base_text().to_string(), new_text));
    }
    Ok(json!({"files": files.into_iter().map(|(path, (old, new))| {
        let diff = language::unified_diff(&old, &new);
        let additions = diff.lines().filter(|line| line.starts_with('+')).count();
        let deletions = diff.lines().filter(|line| line.starts_with('-')).count();
        json!({"path": path.to_string_lossy(), "diff": diff, "additions": additions, "deletions": deletions})
    }).collect::<Vec<_>>()}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> GatewaySettings {
        GatewaySettings {
            enabled: true,
            bind_address: "127.0.0.1:7331".into(),
            auth_token: "test-secret".into(),
            paired_devices: Vec::new(),
        }
    }

    #[test]
    fn validates_opt_in_configuration_without_a_legacy_token() {
        let defaults = GatewaySettings::from_settings(&SettingsContent::default());
        assert!(!defaults.enabled);
        assert!(defaults.address().is_ok());
        for address in [
            "127.0.0.1:7331",
            "[::1]:7331",
            "100.64.0.1:7331",
            "100.127.255.254:7331",
            "[fd7a:115c:a1e0::1]:7331",
        ] {
            assert!(
                GatewaySettings {
                    bind_address: address.into(),
                    ..settings()
                }
                .address()
                .is_ok(),
                "{address}"
            );
        }
        for address in [
            "0.0.0.0:7331",
            "[::]:7331",
            "192.168.1.2:7331",
            "8.8.8.8:7331",
            "100.63.255.255:7331",
            "100.128.0.1:7331",
            "localhost:7331",
            "127.0.0.1:0",
            "[::ffff:100.64.0.1]:7331",
        ] {
            assert!(
                GatewaySettings {
                    bind_address: address.into(),
                    ..settings()
                }
                .address()
                .is_err(),
                "{address}"
            );
        }
    }

    #[test]
    fn v1_requires_its_legacy_token_and_v2_claim_does_not() {
        let v1 = b"GET /v1/status HTTP/1.1\r\nAuthorization: Bearer test-secret\r\n\r\n";
        let v2_claim = b"POST /v2/pair/claim HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
        let v2 = b"GET /v2/status HTTP/1.1\r\nAuthorization: Bearer device-secret\r\n\r\n";
        let v1_request = parse_headers(v1).expect("parse").expect("complete").0;
        assert!(authorize_v1(&v1_request, &settings()).is_ok());
        let claim_request = parse_headers(v2_claim).expect("parse").expect("complete").0;
        assert!(requested_bearer(&claim_request).is_none());
        let v2_request = parse_headers(v2).expect("parse").expect("complete").0;
        assert!(authorize_device(&v2_request, &settings()).is_err());
    }

    #[test]
    fn rejects_ambiguous_and_oversized_requests() {
        for (headers, status) in [
            ("Content-Length: 65537\r\n", 413),
            ("Content-Length: 1\r\nContent-Length: 1\r\n", 400),
            ("Transfer-Encoding: chunked\r\n", 400),
            ("Origin: https://example.com\r\n", 403),
            (
                "Authorization: Bearer one\r\nAuthorization: Bearer two\r\n",
                400,
            ),
            ("Content-Length: -1\r\n", 400),
        ] {
            let request = format!("POST /v2/pair/claim HTTP/1.1\r\n{headers}\r\n");
            assert!(
                matches!(parse_headers(request.as_bytes()), Err(ApiError(actual, _)) if actual == status)
            );
        }
    }

    #[test]
    fn decodes_ids_without_accepting_traversal() {
        assert_eq!(
            route_segments("/v2/threads/a%2Db").expect("valid route"),
            ["v2", "threads", "a-b"]
        );
        for path in [
            "/v2/threads/%2e%2e",
            "/v2/threads/a%2fb",
            "/v2/threads/%00",
            "/v2/threads/%",
            "/v2/threads/%zz",
            "/v2//threads",
            "http://localhost/v2/status",
            "/v2/status?token=secret",
        ] {
            assert!(route_segments(path).is_err(), "{path}");
        }
    }

    #[test]
    fn credential_hashes_are_stable_and_credentials_are_not_records() {
        let hash = credential_hash("device-secret");
        assert_eq!(hash.len(), 64);
        assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
        let device = PairedDevice {
            id: "device-id".into(),
            label: "My phone".into(),
            credential_hash: hash,
            created_at: "2026-01-01T00:00:00Z".into(),
        };
        assert_ne!(device.credential_hash, "device-secret");
    }

    #[test]
    fn responses_are_json_and_never_cached() {
        let response = String::from_utf8(encode_response(401, json!({"error": "Unauthorized"})))
            .expect("UTF-8 response");
        assert!(response.contains("Cache-Control: no-store\r\n"));
        assert!(response.contains("WWW-Authenticate: Bearer\r\n"));
        let (_, body) = response.split_once("\r\n\r\n").expect("body");
        assert_eq!(
            serde_json::from_str::<Value>(body).expect("JSON"),
            json!({"error": "Unauthorized"})
        );
    }
}
