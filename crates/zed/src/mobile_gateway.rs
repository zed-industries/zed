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
    App, AsyncApp, Entity, Global, ReadGlobal as _, Subscription, Task, TaskExt as _, WeakEntity,
    WindowHandle,
};
use language_model::LanguageModelRegistry;
use project::ProjectItem as _;
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
    collections::{BTreeMap, HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use util::{ResultExt as _, truncate_lines_to_byte_limit, truncate_to_byte_limit};
use workspace::{
    AppState, MultiWorkspace, OpenOptions, SerializedWorkspaceLocation, Workspace, WorkspaceDb,
};

const MAX_HEADERS: usize = 8 * 1024;
const MAX_BODY: usize = 256 * 1024;
const MAX_SNAPSHOT: usize = 4 * 1024 * 1024;
// Connections are reused so a phone on a slow tailnet link pays the TCP handshake once. Each
// request still has its own read deadline, and an idle connection is closed promptly.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const IDLE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_REQUESTS_PER_CONNECTION: usize = 100;
// Compressing tiny bodies costs more than it saves.
const MIN_GZIP_BYTES: usize = 1024;
const MAX_QUERY_PARAMETERS: usize = 8;
const QUERY_KEYS: &[&str] = &["before", "limit", "offset", "path"];
// Large threads degrade instead of failing: the newest content that fits these budgets is sent,
// older content is reported as omitted, and any single oversized string is cut with a notice. The
// budgets sum to well below `MAX_SNAPSHOT` so the combined thread response always fits.
const MAX_TRANSCRIPT_BYTES: usize = 1024 * 1024;
const MAX_MESSAGES_BYTES: usize = 256 * 1024;
const MAX_ENTRY_STRING_BYTES: usize = 64 * 1024;
const DEFAULT_TRANSCRIPT_PAGE: usize = 200;
const MAX_TRANSCRIPT_PAGE: usize = 500;
const MAX_CHANGES_BYTES: usize = 1536 * 1024;
const MAX_DIFF_INPUT_BYTES: usize = 16 * 1024 * 1024;
const DEFAULT_DIFF_PAGE_LINES: usize = 400;
const MAX_DIFF_PAGE_LINES: usize = 2000;
const MAX_DIFF_PAGE_BYTES: usize = 1024 * 1024;
const TRUNCATION_NOTICE: &str =
    "\n\n[Truncated for mobile. Open this on desktop for the full content.]";
// Tool output is loaded only when a phone expands a tool card. Reserve enough space for JSON
// escaping, which can expand control bytes sixfold, while keeping one response well below the
// ordinary 2 MiB gateway response cap.
const MAX_TOOL_DETAILS_RESPONSE: usize = 512 * 1024;
const MAX_TOOL_DETAILS_TEXT: usize = MAX_TOOL_DETAILS_RESPONSE / 6 - 1024;
const TOOL_DETAILS_TRUNCATION_NOTICE: &str =
    "\n\n[Output truncated for mobile. Open this tool on desktop for the complete output.]";
const PAIRING_TICKET_TTL: Duration = Duration::from_secs(120);
const MAX_PAIRED_DEVICES: usize = 32;
const EVENT_STREAM_CAPACITY: usize = 16;
const EVENT_STREAM_HEARTBEAT: Duration = Duration::from_secs(20);
const MAX_EVENT_LABEL_BYTES: usize = 256;

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
    project_name: String,
    thread_id: String,
    thread_title: String,
    turn_id: String,
}

#[derive(Clone)]
struct EventThreadMetadata {
    project_id: String,
    project_name: String,
    thread_title: String,
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
    thread_metadata: HashMap<String, EventThreadMetadata>,
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
    query: Vec<(String, String)>,
    authorization: Option<String>,
    accepts_gzip: bool,
    keep_alive: bool,
    body: Vec<u8>,
}

impl Request {
    fn query(&self, key: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }

    fn query_number(&self, key: &str) -> Result<Option<usize>, ApiError> {
        self.query(key)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_| ApiError(400, format!("Query parameter {key} must be a number")))
            })
            .transpose()
    }
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

fn publish_agent_event(kind: &'static str, thread_id: &str, cx: &mut App) {
    let Some(metadata) = cx
        .global::<MobileGatewayEvents>()
        .thread_metadata
        .get(thread_id)
        .cloned()
    else {
        return;
    };
    let event = GatewayEvent {
        kind,
        project_id: metadata.project_id,
        project_name: metadata.project_name,
        thread_id: thread_id.to_string(),
        thread_title: metadata.thread_title,
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

fn parse_query(query: &str) -> Result<Vec<(String, String)>, ApiError> {
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let parameters = query
        .split('&')
        .map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            // Only paging and file-selection parameters exist. Anything else, such as a
            // credential placed in a URL, is refused rather than silently ignored.
            if !QUERY_KEYS.contains(&key) {
                return Err(ApiError(400, "Unsupported query parameter".into()));
            }
            let value = urlencoding::decode(&value.replace('+', " "))
                .map_err(|_| ApiError(400, "Invalid query encoding".into()))?
                .into_owned();
            if value.contains('\0') {
                return Err(ApiError(400, "Invalid query encoding".into()));
            }
            Ok((key.to_string(), value))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if parameters.len() > MAX_QUERY_PARAMETERS {
        return Err(ApiError(400, "Too many query parameters".into()));
    }
    let mut keys = HashSet::new();
    if !parameters.iter().all(|(key, _)| keys.insert(key.as_str())) {
        return Err(ApiError(400, "Duplicate query parameter".into()));
    }
    Ok(parameters)
}

fn header_value<'a>(headers: &'a [httparse::Header<'a>], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(name))
        .and_then(|header| std::str::from_utf8(header.value).ok())
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
        return Err(ApiError(413, "Request body exceeds 256 KiB".into()));
    }
    let method = parsed.method.unwrap_or_default().to_string();
    let target = parsed.path.unwrap_or_default();
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let segments = route_segments(path)?;
    let query = parse_query(query)?;
    let accepts_gzip = header_value(parsed.headers, "accept-encoding").is_some_and(|value| {
        value
            .split(',')
            .any(|encoding| encoding.trim().eq_ignore_ascii_case("gzip"))
    });
    let keep_alive = parsed.version == Some(1)
        && !header_value(parsed.headers, "connection")
            .is_some_and(|value| value.eq_ignore_ascii_case("close"));
    Ok(Some((
        Request {
            method,
            segments,
            query,
            authorization,
            accepts_gzip,
            keep_alive,
            body: Vec::new(),
        },
        parsed_length,
        length,
    )))
}

/// Reads one request. Bytes that arrive after it stay in `pending` for the next request on the
/// same connection.
async fn read_request(stream: &mut TcpStream, pending: &mut Vec<u8>) -> Result<Request, ApiError> {
    let (mut request, header_length, body_length) = loop {
        if let Some(parsed) = parse_headers(pending)? {
            break parsed;
        }
        if pending.len() >= MAX_HEADERS {
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
        pending.extend_from_slice(&buffer[..length]);
    };
    while pending.len() < header_length + body_length {
        let mut buffer = [0; 16 * 1024];
        let remaining = (header_length + body_length - pending.len()).min(buffer.len());
        let length = stream
            .read(&mut buffer[..remaining])
            .await
            .map_err(|_| ApiError(400, "Failed reading body".into()))?;
        if length == 0 {
            return Err(ApiError(400, "Incomplete request body".into()));
        }
        pending.extend_from_slice(&buffer[..length]);
    }
    request.body = pending[header_length..header_length + body_length].to_vec();
    pending.drain(..header_length + body_length);
    Ok(request)
}

/// Waits for the first byte of another request on a reused connection. Returns false when the
/// client closed the connection or stayed idle too long.
async fn await_next_request(stream: &mut TcpStream, pending: &mut Vec<u8>) -> bool {
    if !pending.is_empty() {
        return true;
    }
    let mut buffer = [0; 2048];
    let read = smol::future::or(async { Some(stream.read(&mut buffer).await) }, async {
        smol::Timer::after(IDLE_CONNECTION_TIMEOUT).await;
        None
    })
    .await;
    match read {
        Some(Ok(length)) if length > 0 => {
            pending.extend_from_slice(&buffer[..length]);
            true
        }
        _ => false,
    }
}

fn gzip(body: &[u8]) -> Option<Vec<u8>> {
    use std::io::Write as _;
    let mut encoder = flate2::write::GzEncoder::new(
        Vec::with_capacity(body.len() / 4),
        flate2::Compression::fast(),
    );
    encoder.write_all(body).log_err()?;
    encoder.finish().log_err()
}

fn encode_response(mut status: u16, body: Value, accepts_gzip: bool, keep_alive: bool) -> Vec<u8> {
    let mut body = body.to_string();
    if body.len() > MAX_SNAPSHOT {
        status = 413;
        body = json!({"error": "Response exceeds 4 MiB; load it in smaller pages or open it on desktop"})
            .to_string();
    }
    let challenge = if status == 401 {
        "WWW-Authenticate: Bearer\r\n"
    } else {
        ""
    };
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let (body, encoding) = match (accepts_gzip && body.len() >= MIN_GZIP_BYTES)
        .then(|| gzip(body.as_bytes()))
        .flatten()
    {
        Some(compressed) => (
            compressed,
            "Content-Encoding: gzip\r\nVary: Accept-Encoding\r\n",
        ),
        None => (body.into_bytes(), ""),
    };
    let mut response = format!("HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{encoding}Cache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: {connection}\r\n{challenge}\r\n", body.len()).into_bytes();
    response.extend_from_slice(&body);
    response
}

fn is_event_stream_request(request: &Request) -> bool {
    request.method == "GET"
        && request
            .segments
            .iter()
            .map(String::as_str)
            .eq(["v2", "events"])
}

/// Returns whether the response was fully written.
async fn write_response(
    stream: &mut TcpStream,
    result: ApiResult,
    accepts_gzip: bool,
    keep_alive: bool,
) -> bool {
    let (status, body) = match result {
        Ok(body) => (200, body),
        Err(ApiError(status, error)) => (status, json!({"error": error})),
    };
    let response = encode_response(status, body, accepts_gzip, keep_alive);
    let result = smol::future::or(async { stream.write_all(&response).await }, async {
        smol::Timer::after(REQUEST_TIMEOUT).await;
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "response timed out",
        ))
    })
    .await;
    match result {
        Ok(()) => true,
        Err(error) => {
            log::debug!("Mobile Gateway response failed: {error}");
            false
        }
    }
}

fn encode_sse_event(event: &GatewayEvent) -> Vec<u8> {
    let data = json!({
        "project_id": &event.project_id,
        "project_name": &event.project_name,
        "thread_id": &event.thread_id,
        "thread_title": &event.thread_title,
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
    let mut pending = Vec::new();
    for request_number in 0..MAX_REQUESTS_PER_CONNECTION {
        if request_number > 0 && !await_next_request(&mut stream, &mut pending).await {
            return;
        }
        let request = smol::future::or(read_request(&mut stream, &mut pending), async {
            smol::Timer::after(REQUEST_TIMEOUT).await;
            Err(ApiError(408, "Request timed out".into()))
        })
        .await;

        let request = match request {
            Ok(request) => request,
            Err(error) => {
                write_response(&mut stream, Err(error), false, false).await;
                return;
            }
        };
        if is_event_stream_request(&request) {
            match cx.update(|cx| register_event_stream(&request, cx)) {
                Ok((device_id, stream_id, receiver)) => {
                    serve_event_stream(&mut stream, device_id, stream_id, receiver, cx).await;
                }
                Err(error) => {
                    write_response(&mut stream, Err(error), false, false).await;
                }
            }
            return;
        }
        let accepts_gzip = request.accepts_gzip;
        let keep_alive = request.keep_alive && request_number + 1 < MAX_REQUESTS_PER_CONNECTION;
        let result = dispatch_request(request, cx).await;
        // Authentication failures end the connection so a rejected client cannot keep probing it.
        let keep_alive = keep_alive && !matches!(result, Err(ApiError(401 | 403, _)));
        if !write_response(&mut stream, result, accepts_gzip, keep_alive).await || !keep_alive {
            return;
        }
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
    /// Legacy one-time shorthand: "allow" or "deny".
    decision: Option<String>,
    option_id: Option<String>,
    /// Indices into the prompt's pattern list, for pattern-scoped always rules.
    patterns: Option<Vec<usize>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewRequest {
    action: String,
    /// Paths exactly as reported by `changes`; omitted means every unreviewed file.
    paths: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ElicitationAnswer {
    action: String,
    content: Option<serde_json::Map<String, Value>>,
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

fn event_display_label(value: &str, fallback: &str) -> String {
    let normalized = value
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect::<String>();
    let normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized = normalized.trim();
    if normalized.is_empty() {
        fallback.to_string()
    } else {
        truncate_to_byte_limit(normalized, MAX_EVENT_LABEL_BYTES).to_string()
    }
}

fn event_project_name(project: &OpenProject) -> String {
    let name = project_name(&project.roots);
    let name = if name.is_empty() {
        match &project.location {
            CatalogProjectLocation::Local => "Project".to_string(),
            CatalogProjectLocation::Remote(connection) => connection.display_name().to_string(),
        }
    } else {
        name
    };
    event_display_label(&name, "Project")
}

fn update_event_thread_metadata(
    thread_id: &str,
    project_id: &str,
    project_name: &str,
    thread_title: &str,
    cx: &mut App,
) {
    cx.global_mut::<MobileGatewayEvents>()
        .thread_metadata
        .insert(
            thread_id.to_string(),
            EventThreadMetadata {
                project_id: project_id.to_string(),
                project_name: event_display_label(project_name, "Project"),
                thread_title: event_display_label(thread_title, "Untitled thread"),
            },
        );
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
        let project_name = event_project_name(&project);
        ensure_agent_panel_event_subscription(&project.id, &project_name, &panel, cx);
        ensure_conversation_event_subscriptions(&project.id, &project_name, &panel, cx);
    }
}

fn ensure_agent_panel_event_subscription(
    project_id: &str,
    project_name: &str,
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
    let project_name = project_name.to_string();
    let callback_project_id = project_id.clone();
    let callback_project_name = project_name.clone();
    let subscription = cx.subscribe(panel, move |panel, event: &AgentPanelEvent, cx| {
        if matches!(
            event,
            AgentPanelEvent::ActiveViewChanged | AgentPanelEvent::EntryChanged
        ) {
            // AgentPanel emits these for newly created desktop conversations and title changes.
            // Refresh so phone notifications cover all eligible turns, not only conversations
            // first opened through Mobile Gateway.
            ensure_conversation_event_subscriptions(
                &callback_project_id,
                &callback_project_name,
                &panel,
                cx,
            );
        }
    });
    cx.global_mut::<MobileGatewayEvents>()
        .panel_subscriptions
        .insert(project_id, subscription);
}

fn ensure_conversation_event_subscriptions(
    project_id: &str,
    project_name: &str,
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

        ensure_conversation_thread_subscription(project_id, project_name, &view, cx);
        if cx
            .global::<MobileGatewayEvents>()
            .conversation_subscriptions
            .contains_key(&conversation_id)
        {
            continue;
        }

        let project_id = project_id.to_string();
        let project_name = project_name.to_string();
        let subscription = cx.subscribe(&view, move |view, _: &StateChange, cx| {
            ensure_conversation_thread_subscription(&project_id, &project_name, &view, cx);
        });
        cx.global_mut::<MobileGatewayEvents>()
            .conversation_subscriptions
            .insert(conversation_id, subscription);
    }
}

fn ensure_conversation_thread_subscription(
    project_id: &str,
    project_name: &str,
    view: &Entity<ConversationView>,
    cx: &mut App,
) {
    let (thread_id, thread, thread_title) = {
        let view = view.read(cx);
        let Some(thread_view) = view.root_thread_view() else {
            return;
        };
        (
            view.parent_id().to_key_string(),
            thread_view.read(cx).thread.clone(),
            view.title(cx).to_string(),
        )
    };
    ensure_thread_event_subscription(
        &thread_id,
        project_id,
        project_name,
        &thread_title,
        &thread,
        cx,
    );
}

fn ensure_thread_event_subscription(
    thread_id: &str,
    project_id: &str,
    project_name: &str,
    thread_title: &str,
    thread: &Entity<acp_thread::AcpThread>,
    cx: &mut App,
) {
    // Conversation state can change after its ACP thread was first observed (notably while Zed
    // derives a title). Refresh the metadata even when the thread subscription already exists.
    update_event_thread_metadata(thread_id, project_id, project_name, thread_title, cx);
    if cx
        .global::<MobileGatewayEvents>()
        .thread_subscriptions
        .contains_key(thread_id)
    {
        return;
    }

    let thread_id = thread_id.to_string();
    let event_thread_id = thread_id.clone();
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
            publish_agent_event(kind, &event_thread_id, cx);
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

/// Returns the newest messages that fit `MAX_MESSAGES_BYTES`. v2 clients render the transcript, so
/// they only need user messages here (to recognize that a sent prompt has been recorded).
fn legacy_messages(entries: &[AgentThreadEntry], include_assistant: bool, cx: &App) -> Vec<Value> {
    let mut size = 0;
    let mut messages = Vec::new();
    for entry in entries.iter().rev() {
        let (role, text) = match entry {
            AgentThreadEntry::UserMessage(message) => {
                ("user", message.content.to_markdown(cx).to_string())
            }
            AgentThreadEntry::AssistantMessage(message) if include_assistant => (
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
        let text = truncate_with_notice(text, MAX_ENTRY_STRING_BYTES);
        size += text.len();
        if size > MAX_MESSAGES_BYTES && !messages.is_empty() {
            break;
        }
        messages.push(json!({"role": role, "text": text}));
    }
    messages.reverse();
    messages
}

fn truncate_with_notice(text: String, limit: usize) -> String {
    if text.len() <= limit {
        return text;
    }
    let prefix = truncate_to_byte_limit(&text, limit.saturating_sub(TRUNCATION_NOTICE.len()));
    format!("{prefix}{TRUNCATION_NOTICE}")
}

fn truncate_json_strings(value: &mut Value, limit: usize) {
    match value {
        Value::String(text) if text.len() > limit => {
            *text = truncate_with_notice(std::mem::take(text), limit);
        }
        Value::Array(items) => items
            .iter_mut()
            .for_each(|item| truncate_json_strings(item, limit)),
        Value::Object(map) => map
            .values_mut()
            .for_each(|item| truncate_json_strings(item, limit)),
        _ => {}
    }
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

/// Returns the newest entries that fit `MAX_TRANSCRIPT_BYTES` and the number of older entries
/// omitted. Entries keep their original `index`.
fn transcript_entry(
    index: usize,
    entry: &AgentThreadEntry,
    thread: &acp_thread::AcpThread,
    cx: &App,
) -> Value {
    match entry {
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
            // Detailed terminal, diff, resource, and raw output content is intentionally not
            // included in repeated thread polls. The paired device may request it on demand
            // for this exact allowed thread/tool call when the user expands its card.
            "details_available": !call.content.is_empty() || call.raw_output.is_some(),
        }),
        AgentThreadEntry::Elicitation(id) => {
            let mut value = json!({
                "index": index,
                "kind": "elicitation",
                "text": entry.to_markdown(cx),
            });
            if let Some((_, elicitation)) = thread.elicitation(id) {
                let details = elicitation_json(elicitation);
                value["elicitation_id"] = details["elicitation_id"].clone();
                value["status"] = details["status"].clone();
            }
            value
        }
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
    }
}

/// One page of the transcript, oldest first, ending just before entry index `before` (or at the
/// newest entry). The page stops at `limit` entries or the byte budget, whichever comes first;
/// the returned count is the number of older entries the phone can still request.
fn transcript(
    thread: &acp_thread::AcpThread,
    before: Option<usize>,
    limit: usize,
    cx: &App,
) -> (Vec<Value>, usize) {
    let entries = thread.entries();
    let end = before.unwrap_or(entries.len()).min(entries.len());
    let mut page = Vec::new();
    let mut size = 0;
    let mut first = end;
    for index in (0..end).rev() {
        if page.len() >= limit {
            break;
        }
        let mut value = transcript_entry(index, &entries[index], thread, cx);
        truncate_json_strings(&mut value, MAX_ENTRY_STRING_BYTES);
        size += value.to_string().len();
        // Always return at least one entry so paging makes progress.
        if size > MAX_TRANSCRIPT_BYTES && !page.is_empty() {
            break;
        }
        page.push(value);
        first = index;
    }
    page.reverse();
    (page, first)
}

fn with_transcript(result: &mut Value, thread: &acp_thread::AcpThread, cx: &App) {
    let (entries, omitted) = transcript(thread, None, DEFAULT_TRANSCRIPT_PAGE, cx);
    result["transcript"] = json!(entries);
    result["transcript_omitted_entries"] = json!(omitted);
}

fn bounded_tool_details_text(text: String) -> (String, bool) {
    if text.len() <= MAX_TOOL_DETAILS_TEXT {
        return (text, false);
    }
    let prefix_limit = MAX_TOOL_DETAILS_TEXT.saturating_sub(TOOL_DETAILS_TRUNCATION_NOTICE.len());
    let prefix = truncate_lines_to_byte_limit(&text, prefix_limit);
    (format!("{prefix}{TOOL_DETAILS_TRUNCATION_NOTICE}"), true)
}

fn tool_details_text(call: &acp_thread::ToolCall, cx: &App) -> Option<String> {
    let mut text = String::new();
    for content in &call.content {
        let content = content.to_markdown(cx);
        if content.trim().is_empty() {
            continue;
        }
        if !text.is_empty() {
            text.push_str("\n\n");
        }
        text.push_str(&content);
    }
    if let Some(raw_output) = &call.raw_output {
        if !text.is_empty() {
            text.push_str("\n\n");
        }
        text.push_str("Raw tool output (JSON):\n```json\n");
        text.push_str(&serde_json::to_string(raw_output).unwrap_or_else(|_| "null".into()));
        text.push_str("\n```");
    }
    (!text.trim().is_empty()).then_some(text)
}

fn tool_details(thread: &Entity<acp_thread::AcpThread>, tool_call_id: &str, cx: &App) -> ApiResult {
    let id = acp::ToolCallId::new(tool_call_id.to_string());
    let (resolved_tool_call_id, status, text) = {
        let thread = thread.read(cx);
        let (_, call) = thread.tool_call(&id).ok_or_else(not_found)?;
        (
            call.id.0.to_string(),
            tool_status(&call.status),
            tool_details_text(call, cx),
        )
    };
    let Some(text) = text else {
        return Ok(json!({
            "tool_call_id": resolved_tool_call_id,
            "status": status,
            "available": false,
        }));
    };
    let (output, truncated) = bounded_tool_details_text(text);
    Ok(json!({
        "tool_call_id": resolved_tool_call_id,
        "status": status,
        "available": true,
        "format": "markdown",
        "output": output,
        "truncated": truncated,
    }))
}

fn permission_kind_name(kind: acp::PermissionOptionKind) -> &'static str {
    match kind {
        acp::PermissionOptionKind::AllowOnce => "allow_once",
        acp::PermissionOptionKind::AllowAlways => "allow_always",
        acp::PermissionOptionKind::RejectOnce => "reject_once",
        acp::PermissionOptionKind::RejectAlways => "reject_always",
        _ => "other",
    }
}

fn permission_kind_allows(kind: acp::PermissionOptionKind) -> bool {
    !matches!(
        kind,
        acp::PermissionOptionKind::RejectOnce | acp::PermissionOptionKind::RejectAlways
    )
}

/// Every option the desktop offers for this prompt, deduplicated by ID and in desktop order.
fn permission_options(options: &acp_thread::PermissionOptions) -> Vec<&acp::PermissionOption> {
    let candidates: Vec<&acp::PermissionOption> = match options {
        acp_thread::PermissionOptions::Flat(options) => options.iter().collect(),
        acp_thread::PermissionOptions::Dropdown(choices)
        | acp_thread::PermissionOptions::DropdownWithPatterns { choices, .. } => choices
            .iter()
            .flat_map(|choice| [&choice.allow, &choice.deny])
            .collect(),
    };
    let mut seen = HashSet::new();
    candidates
        .into_iter()
        .filter(|option| seen.insert(option.option_id.0.clone()))
        .collect()
}

fn approvals(
    entries: &[AgentThreadEntry],
    thread_view: &Entity<agent_ui::ThreadView>,
    cx: &App,
) -> Value {
    let values = entries
        .iter()
        .filter_map(|entry| {
            let AgentThreadEntry::ToolCall(call) = entry else {
                return None;
            };
            let ToolCallStatus::WaitingForConfirmation { options, kind, .. } = &call.status else {
                return None;
            };
            let allow_blocked = thread_view
                .read(cx)
                .tool_call_allow_blocked_by_confusables(&call.id, cx);
            let patterns = match options {
                acp_thread::PermissionOptions::DropdownWithPatterns { patterns, .. } => patterns
                    .iter()
                    .enumerate()
                    .map(|(index, pattern)| {
                        json!({
                            "index": index,
                            "pattern": pattern.pattern,
                            "display_name": pattern.display_name,
                        })
                    })
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            };
            Some(json!({
                "tool_call_id": call.id.0.to_string(),
                "title": call.label.read(cx).source(),
                "tool_name": call.tool_name.as_ref().map(ToString::to_string),
                "content": call.content.iter().map(|content| content.to_markdown(cx)).collect::<Vec<_>>(),
                "raw_input": call.raw_input,
                "kind": match kind {
                    acp_thread::AuthorizationKind::PermissionGrant => "permission",
                    acp_thread::AuthorizationKind::ActionChoice => "action_choice",
                },
                "options": permission_options(options).into_iter().map(|option| {
                    let allows = permission_kind_allows(option.kind);
                    json!({
                        "option_id": option.option_id.0.to_string(),
                        "label": option.name,
                        "kind": permission_kind_name(option.kind),
                        "enabled": !(allows && allow_blocked && *kind == acp_thread::AuthorizationKind::PermissionGrant),
                    })
                }).collect::<Vec<_>>(),
                "patterns": patterns,
                "allow_blocked_reason": allow_blocked.then_some(
                    "This request contains suspicious Unicode. Review and acknowledge it on desktop before allowing it.",
                ),
                "allow_once_available": !allow_blocked && options.allow_once_option_id().is_some(),
                "deny_once_available": options.deny_once_option_id().is_some(),
            }))
        })
        .collect::<Vec<_>>();
    json!({"approvals": values})
}

fn resolve_approval(
    thread: &Entity<acp_thread::AcpThread>,
    thread_view: &Entity<agent_ui::ThreadView>,
    tool_call_id: &str,
    request: &Request,
    cx: &mut App,
) -> ApiResult {
    let resolution: ApprovalResolution = body(request)?;
    let id = acp::ToolCallId::new(tool_call_id.to_string());
    let allow_blocked = thread_view
        .read(cx)
        .tool_call_allow_blocked_by_confusables(&id, cx);
    thread.update(cx, |thread, cx| {
        let Some((_, call)) = thread.tool_call(&id) else {
            return Err(ApiError(409, "Approval is no longer pending".into()));
        };
        let ToolCallStatus::WaitingForConfirmation { options, kind, .. } = &call.status else {
            return Err(ApiError(409, "Approval is no longer pending".into()));
        };
        // The phone names a desktop-issued option; it never supplies a kind, rule or tool input.
        let option_id = match (resolution.option_id, resolution.decision.as_deref()) {
            (Some(option_id), None) => acp::PermissionOptionId::new(option_id),
            (None, Some("allow")) => options
                .allow_once_option_id()
                .ok_or_else(|| ApiError(409, "That one-time option is unavailable".into()))?,
            (None, Some("deny")) => options
                .deny_once_option_id()
                .ok_or_else(|| ApiError(409, "That one-time option is unavailable".into()))?,
            _ => {
                return Err(ApiError(
                    400,
                    "Send exactly one of option_id or decision (allow or deny)".into(),
                ));
            }
        };
        let option = permission_options(options)
            .into_iter()
            .find(|option| option.option_id == option_id)
            .ok_or_else(|| ApiError(409, "That option is no longer offered".into()))?;
        let allows = permission_kind_allows(option.kind);
        if allows && allow_blocked && *kind == acp_thread::AuthorizationKind::PermissionGrant {
            return Err(ApiError(
                409,
                "Acknowledge the suspicious Unicode warning on desktop before allowing this".into(),
            ));
        }
        let outcome = match resolution.patterns {
            Some(indices) if !indices.is_empty() => {
                let is_always = matches!(
                    option.kind,
                    acp::PermissionOptionKind::AllowAlways
                        | acp::PermissionOptionKind::RejectAlways
                );
                if !is_always {
                    return Err(ApiError(
                        400,
                        "Patterns may only accompany an always option".into(),
                    ));
                }
                let pattern_count = match options {
                    acp_thread::PermissionOptions::DropdownWithPatterns { patterns, .. } => {
                        patterns.len()
                    }
                    _ => 0,
                };
                if indices.iter().any(|index| *index >= pattern_count) {
                    return Err(ApiError(400, "Unknown pattern".into()));
                }
                options
                    .build_outcome_for_checked_patterns(&indices, allows)
                    .ok_or_else(|| ApiError(409, "Pattern rules are unavailable".into()))?
            }
            _ => acp_thread::SelectedPermissionOutcome::new(option.option_id.clone(), option.kind),
        };
        thread.authorize_tool_call(id, outcome, cx);
        Ok(json!({"accepted": true}))
    })
}

fn elicitation_json(elicitation: &acp_thread::Elicitation) -> Value {
    let status = match elicitation.status {
        acp_thread::ElicitationStatus::Pending { .. } => "pending",
        acp_thread::ElicitationStatus::Accepted => "accepted",
        acp_thread::ElicitationStatus::Declined => "declined",
        acp_thread::ElicitationStatus::Canceled => "canceled",
        acp_thread::ElicitationStatus::Completed => "completed",
    };
    let mut value = json!({
        "elicitation_id": elicitation.id.0.to_string(),
        "message": elicitation.request.message,
        "status": status,
    });
    match &elicitation.request.mode {
        acp::ElicitationMode::Form(mode) => {
            value["mode"] = json!("form");
            value["schema"] = serde_json::to_value(&mode.requested_schema).unwrap_or(Value::Null);
        }
        acp::ElicitationMode::Url(mode) => {
            value["mode"] = json!("url");
            value["url"] = json!(mode.url);
        }
        _ => value["mode"] = json!("unsupported"),
    }
    value
}

fn pending_elicitations(thread: &acp_thread::AcpThread) -> Value {
    let values = thread
        .entries()
        .iter()
        .filter_map(|entry| match entry {
            AgentThreadEntry::Elicitation(id) => thread.elicitation(id),
            _ => None,
        })
        .filter(|(_, elicitation)| {
            matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Pending { .. }
            )
        })
        .map(|(_, elicitation)| elicitation_json(elicitation))
        .collect::<Vec<_>>();
    json!({"elicitations": values})
}

fn respond_to_elicitation(
    thread: &Entity<acp_thread::AcpThread>,
    thread_view: &Entity<agent_ui::ThreadView>,
    elicitation_id: &str,
    request: &Request,
    cx: &mut App,
) -> ApiResult {
    let answer: ElicitationAnswer = body(request)?;
    let id = acp_thread::ElicitationEntryId(elicitation_id.into());
    let response = {
        let thread = thread.read(cx);
        let Some((_, elicitation)) = thread.elicitation(&id) else {
            return Err(ApiError(404, "Question not found".into()));
        };
        if !matches!(
            elicitation.status,
            acp_thread::ElicitationStatus::Pending { .. }
        ) {
            return Err(ApiError(409, "Question was already answered".into()));
        }
        match answer.action.as_str() {
            "decline" => acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
            "cancel" => acp::CreateElicitationResponse::new(acp::ElicitationAction::Cancel),
            "accept" => {
                let acp::ElicitationMode::Form(mode) = &elicitation.request.mode else {
                    return Err(ApiError(
                        409,
                        "This question needs a browser on desktop; decline or answer it there"
                            .into(),
                    ));
                };
                let values = answer.content.unwrap_or_default();
                let content =
                    agent_ui::validate_elicitation_values(&mode.requested_schema, &values)
                        .map_err(|errors| {
                            let mut errors = errors
                                .into_iter()
                                .map(|(field, error)| format!("{field}: {error}"))
                                .collect::<Vec<_>>();
                            errors.sort();
                            ApiError(422, errors.join("; "))
                        })?;
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new().content(content),
                ))
            }
            _ => {
                return Err(ApiError(
                    400,
                    "action must be accept, decline or cancel".into(),
                ));
            }
        }
    };
    thread_view.update(cx, |thread_view, cx| {
        thread_view.respond_to_elicitation(id, response, cx)
    });
    Ok(json!({"accepted": true}))
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
                let project_name = event_project_name(project);
                let thread_title = view.read(cx).title(cx).to_string();
                ensure_thread_event_subscription(
                    thread_id,
                    &project.id,
                    &project_name,
                    &thread_title,
                    &thread,
                    cx,
                );
                return match (method, rest) {
                    ("GET", []) => {
                        let mut result = summary(view.read(cx), cx);
                        let entries = thread.read(cx).entries();
                        result["messages"] = json!(legacy_messages(entries, *version != "v2", cx));
                        if *version == "v2" {
                            with_transcript(&mut result, thread.read(cx), cx);
                        }
                        Ok(result)
                    }
                    ("GET", ["transcript"]) if *version == "v2" => {
                        let before = request.query_number("before")?;
                        let limit = request
                            .query_number("limit")?
                            .unwrap_or(DEFAULT_TRANSCRIPT_PAGE)
                            .clamp(1, MAX_TRANSCRIPT_PAGE);
                        let (entries, omitted) = transcript(thread.read(cx), before, limit, cx);
                        Ok(json!({
                            "entries": entries,
                            "omitted_entries": omitted,
                            "has_more": omitted > 0,
                            "total_entries": thread.read(cx).entries().len(),
                        }))
                    }
                    ("GET", ["tool-calls", tool_call_id, "output"]) if *version == "v2" => {
                        tool_details(&thread, tool_call_id, cx)
                    }
                    ("GET", ["approvals"]) if *version == "v2" => {
                        Ok(approvals(thread.read(cx).entries(), &thread_view, cx))
                    }
                    ("POST", ["approvals", tool_call_id, "resolve"]) if *version == "v2" => {
                        resolve_approval(&thread, &thread_view, tool_call_id, request, cx)
                    }
                    ("GET", ["elicitations"]) if *version == "v2" => {
                        Ok(pending_elicitations(thread.read(cx)))
                    }
                    ("POST", ["elicitations", elicitation_id, "respond"]) if *version == "v2" => {
                        respond_to_elicitation(&thread, &thread_view, elicitation_id, request, cx)
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
                    ("GET", ["changes"]) => changes(thread.read(cx), project, cx),
                    ("GET", ["changes", "file"]) if *version == "v2" => {
                        changed_file_page(thread.read(cx), project, request, cx)
                    }
                    ("POST", ["changes", "review"]) if *version == "v2" => {
                        review_changes(&thread, &thread_view, project, request, cx)
                    }
                    ("POST", ["changes", "undo-reject"]) if *version == "v2" => {
                        let action_log = thread.read(cx).action_log().clone();
                        if !action_log.read(cx).has_pending_undo() {
                            return Err(ApiError(409, "There is no rejection to undo".into()));
                        }
                        action_log
                            .update(cx, |action_log, cx| action_log.undo_last_reject(cx))
                            .detach();
                        Ok(json!({"accepted": true}))
                    }
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

/// The before/after text of every file the thread's tool calls changed, keyed by verified path.
/// The bool reports whether the combined input exceeded the budget and was cut short.
fn changed_file_texts(
    entries: &[AgentThreadEntry],
    project: &OpenProject,
    cx: &App,
) -> Result<(BTreeMap<PathBuf, (String, String)>, bool), ApiError> {
    let mut files = BTreeMap::<PathBuf, (String, String)>::new();
    let mut input_size = 0;
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
        input_size += diff.base_text().len() + buffer.len();
        if input_size > MAX_DIFF_INPUT_BYTES {
            return Ok((files, true));
        }
        let new_text = buffer.text();
        files
            .entry(path)
            .and_modify(|texts| texts.1 = new_text.clone())
            .or_insert_with(|| (diff.base_text().to_string(), new_text));
    }
    Ok((files, false))
}

/// Buffers holding agent edits the user has not yet kept or rejected, with their verified paths.
/// Review actions only ever target this set, so a phone can never name an arbitrary file.
fn unreviewed_buffers(
    thread: &acp_thread::AcpThread,
    project: &OpenProject,
    cx: &App,
) -> Vec<(PathBuf, Entity<language::Buffer>)> {
    let workspace_project = project.workspace.read(cx).project().read(cx);
    thread
        .action_log()
        .read(cx)
        .changed_buffers(cx)
        .filter_map(|(buffer, _)| {
            let project_path = buffer.read(cx).project_path(cx)?;
            let absolute_path = workspace_project.absolute_path(&project_path, cx)?;
            let path = verified_project_path(&absolute_path, project)?;
            Some((path, buffer))
        })
        .collect()
}

fn diff_counts(diff: &str) -> (usize, usize) {
    let additions = diff
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .count();
    let deletions = diff
        .lines()
        .filter(|line| line.starts_with('-') && !line.starts_with("---"))
        .count();
    (additions, deletions)
}

fn changes(thread: &acp_thread::AcpThread, project: &OpenProject, cx: &App) -> ApiResult {
    let (files, too_large) = changed_file_texts(thread.entries(), project, cx)?;
    let unreviewed = unreviewed_buffers(thread, project, cx)
        .into_iter()
        .map(|(path, _)| path)
        .collect::<HashSet<_>>();

    // Files are listed in full while they fit; once the budget is spent, remaining files are still
    // listed with their change counts but without diff text, flagged `truncated`. Their full diff
    // can be paged through `changes/file`.
    let mut size = 0;
    let files = files
        .into_iter()
        .map(|(path, (old, new))| {
            let diff = language::unified_diff(&old, &new);
            let (additions, deletions) = diff_counts(&diff);
            let diff_bytes = diff.len();
            let fits = size + diff.len() <= MAX_CHANGES_BYTES;
            let diff = if fits {
                size += diff.len();
                diff
            } else {
                String::new()
            };
            json!({
                "path": path.to_string_lossy(),
                "diff": diff,
                "diff_bytes": diff_bytes,
                "additions": additions,
                "deletions": deletions,
                "truncated": !fits,
                "unreviewed": unreviewed.contains(&path),
            })
        })
        .collect::<Vec<_>>();
    let mut unreviewed_files = unreviewed
        .iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    unreviewed_files.sort();
    Ok(json!({
        "files": files,
        "truncated": too_large || files.iter().any(|file| file["truncated"] == true),
        "review": {
            "unreviewed_files": unreviewed_files,
            "can_undo_reject": thread.action_log().read(cx).has_pending_undo(),
        },
    }))
}

fn changed_file_page(
    thread: &acp_thread::AcpThread,
    project: &OpenProject,
    request: &Request,
    cx: &App,
) -> ApiResult {
    let requested = request
        .query("path")
        .ok_or_else(|| ApiError(400, "Query parameter path is required".into()))?;
    let offset = request.query_number("offset")?.unwrap_or(0);
    let limit = request
        .query_number("limit")?
        .unwrap_or(DEFAULT_DIFF_PAGE_LINES)
        .clamp(1, MAX_DIFF_PAGE_LINES);
    let (files, _) = changed_file_texts(thread.entries(), project, cx)?;
    // The phone may only name a path this endpoint previously reported for this thread.
    let (path, (old, new)) = files
        .into_iter()
        .find(|(path, _)| path.to_string_lossy() == requested)
        .ok_or_else(|| ApiError(404, "That file has no changes in this thread".into()))?;
    let diff = language::unified_diff(&old, &new);
    let (additions, deletions) = diff_counts(&diff);
    let lines = diff.lines().collect::<Vec<_>>();
    let start = offset.min(lines.len());
    let mut text = String::new();
    let mut end = start;
    for line in &lines[start..] {
        if end - start >= limit || (end > start && text.len() + line.len() > MAX_DIFF_PAGE_BYTES) {
            break;
        }
        text.push_str(&truncate_with_notice(
            line.to_string(),
            MAX_ENTRY_STRING_BYTES,
        ));
        text.push('\n');
        end += 1;
    }
    Ok(json!({
        "path": path.to_string_lossy(),
        "diff": text,
        "offset": start,
        "next_offset": (end < lines.len()).then_some(end),
        "total_lines": lines.len(),
        "additions": additions,
        "deletions": deletions,
    }))
}

fn review_changes(
    thread: &Entity<acp_thread::AcpThread>,
    thread_view: &Entity<agent_ui::ThreadView>,
    project: &OpenProject,
    request: &Request,
    cx: &mut App,
) -> ApiResult {
    let review: ReviewRequest = body(request)?;
    let keep = match review.action.as_str() {
        "keep" => true,
        "reject" => false,
        _ => return Err(ApiError(400, "action must be keep or reject".into())),
    };
    if thread.read(cx).status() == ThreadStatus::Generating {
        return Err(ApiError(
            409,
            "Wait for the Agent to finish before reviewing its edits".into(),
        ));
    }
    let buffers = unreviewed_buffers(thread.read(cx), project, cx);
    let selected = match review.paths {
        None => buffers,
        Some(paths) => {
            if paths.is_empty() {
                return Err(ApiError(
                    400,
                    "paths must not be empty; omit it for all files".into(),
                ));
            }
            let mut selected = Vec::with_capacity(paths.len());
            for requested in &paths {
                let buffer = buffers
                    .iter()
                    .find(|(path, _)| path.to_string_lossy() == requested.as_str())
                    .ok_or_else(|| {
                        ApiError(409, format!("{requested} has no unreviewed Agent edits"))
                    })?;
                selected.push(buffer.clone());
            }
            selected
        }
    };
    if selected.is_empty() {
        return Err(ApiError(409, "There are no unreviewed Agent edits".into()));
    }

    let telemetry = action_log::ActionLogTelemetry::from(thread.read(cx));
    let action_log = thread.read(cx).action_log().clone();
    let file_count = selected.len();
    action_log.update(cx, |action_log, cx| {
        if keep {
            for (_, buffer) in selected {
                let end = buffer.read(cx).max_point();
                action_log.keep_edits_in_range(
                    buffer,
                    language::Point::zero()..end,
                    Some(telemetry.clone()),
                    cx,
                );
            }
            return;
        }
        let mut undo_buffers = Vec::new();
        for (_, buffer) in selected {
            let range = language::Anchor::min_max_range_for_buffer(buffer.read(cx).remote_id());
            let (task, undo) =
                action_log.reject_edits_in_ranges(buffer, vec![range], Some(telemetry.clone()), cx);
            undo_buffers.extend(undo);
            task.detach_and_log_err(cx);
        }
        if !undo_buffers.is_empty() {
            action_log.set_last_reject_undo(action_log::LastRejectUndo {
                buffers: undo_buffers,
            });
        }
    });
    thread_view.update(cx, |_, cx| cx.notify());
    Ok(json!({"accepted": true, "files": file_count}))
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
            ("Content-Length: 262145\r\n", 413),
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
    fn event_labels_are_sanitized_and_sse_events_are_descriptive() {
        assert_eq!(
            event_display_label("  Project\n\tname  ", "Project"),
            "Project name"
        );
        let label = event_display_label(&"é".repeat(MAX_EVENT_LABEL_BYTES), "Project");
        assert!(label.len() <= MAX_EVENT_LABEL_BYTES);
        assert!(std::str::from_utf8(label.as_bytes()).is_ok());

        let event = GatewayEvent {
            kind: "thread_completed",
            project_id: "project-1".into(),
            project_name: "Zed Desktop".into(),
            thread_id: "thread-2".into(),
            thread_title: "Review notifications".into(),
            turn_id: "turn-3".into(),
        };
        let encoded = String::from_utf8(encode_sse_event(&event)).expect("UTF-8 SSE");
        assert!(encoded.starts_with("event: thread_completed\n"));
        let payload = encoded
            .strip_prefix("event: thread_completed\ndata: ")
            .and_then(|payload| payload.strip_suffix("\n\n"))
            .expect("SSE payload");
        assert_eq!(
            serde_json::from_str::<Value>(payload).expect("JSON event"),
            json!({
                "project_id": "project-1",
                "project_name": "Zed Desktop",
                "thread_id": "thread-2",
                "thread_title": "Review notifications",
                "turn_id": "turn-3",
            })
        );
    }

    #[test]
    fn tool_details_are_bounded_at_a_utf8_boundary() {
        let text = format!("{}\nmore output", "é".repeat(MAX_TOOL_DETAILS_TEXT));
        let (output, truncated) = bounded_tool_details_text(text);

        assert!(truncated);
        assert!(output.len() <= MAX_TOOL_DETAILS_TEXT);
        assert!(std::str::from_utf8(output.as_bytes()).is_ok());
        assert!(output.ends_with(TOOL_DETAILS_TRUNCATION_NOTICE));
    }

    #[test]
    fn oversized_strings_are_truncated_at_a_utf8_boundary() {
        let mut value = json!({"chunks": [{"text": "é".repeat(MAX_ENTRY_STRING_BYTES)}]});
        truncate_json_strings(&mut value, MAX_ENTRY_STRING_BYTES);

        let text = value["chunks"][0]["text"].as_str().expect("text");
        assert!(text.len() <= MAX_ENTRY_STRING_BYTES);
        assert!(text.ends_with(TRUNCATION_NOTICE));

        let mut small = json!({"text": "short"});
        truncate_json_strings(&mut small, MAX_ENTRY_STRING_BYTES);
        assert_eq!(small, json!({"text": "short"}));
    }

    #[test]
    fn responses_are_json_and_never_cached() {
        let response = String::from_utf8(encode_response(
            401,
            json!({"error": "Unauthorized"}),
            true,
            false,
        ))
        .expect("UTF-8 response");
        assert!(response.contains("Cache-Control: no-store\r\n"));
        assert!(response.contains("Connection: close\r\n"));
        assert!(
            !response.contains("Content-Encoding"),
            "small bodies stay plain"
        );
        assert!(response.contains("WWW-Authenticate: Bearer\r\n"));
        let (_, body) = response.split_once("\r\n\r\n").expect("body");
        assert_eq!(
            serde_json::from_str::<Value>(body).expect("JSON"),
            json!({"error": "Unauthorized"})
        );
    }

    #[test]
    fn large_responses_are_gzipped_only_when_accepted() {
        use std::io::Read as _;
        let body = json!({"text": "diff line\n".repeat(4096)});
        let plain = encode_response(200, body.clone(), false, true);
        let compressed = encode_response(200, body.clone(), true, true);
        assert!(compressed.len() < plain.len() / 4);

        let split = compressed
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("header end");
        let headers = std::str::from_utf8(&compressed[..split]).expect("ASCII headers");
        assert!(headers.contains("Content-Encoding: gzip"));
        assert!(headers.contains("Connection: keep-alive"));
        let mut decoded = String::new();
        flate2::read::GzDecoder::new(&compressed[split + 4..])
            .read_to_string(&mut decoded)
            .expect("gzip body");
        assert_eq!(serde_json::from_str::<Value>(&decoded).expect("JSON"), body);
    }

    #[test]
    fn parses_only_known_query_parameters() {
        let request = parse_headers(
            b"GET /v2/threads/t/changes/file?path=%2Fsrc%2Fmain.rs&offset=40 HTTP/1.1\r\nAccept-Encoding: gzip, br\r\n\r\n",
        )
        .expect("parse")
        .expect("complete")
        .0;
        assert_eq!(request.segments, ["v2", "threads", "t", "changes", "file"]);
        assert_eq!(request.query("path"), Some("/src/main.rs"));
        assert_eq!(request.query_number("offset").expect("number"), Some(40));
        assert!(request.accepts_gzip);
        assert!(request.keep_alive);

        for target in [
            "/v2/status?token=secret",
            "/v2/status?limit=1&limit=2",
            "/v2/status?path=%00",
        ] {
            let raw = format!("GET {target} HTTP/1.1\r\n\r\n");
            assert!(
                matches!(parse_headers(raw.as_bytes()), Err(ApiError(400, _))),
                "{target}"
            );
        }
        let closing = parse_headers(b"GET /v2/status HTTP/1.1\r\nConnection: close\r\n\r\n")
            .expect("parse")
            .expect("complete")
            .0;
        assert!(!closing.keep_alive);
    }
}
