// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

static REQUEST_ID: AtomicU32 = AtomicU32::new(1);

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResult {
    pub duration: u128,
    pub refresh_id: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PetInfoResponse {
    pub pet_version: String,
    pub build_id: Option<String>,
    pub commit_sha: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentNotification {
    pub executable: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ManagerNotification {
    pub tool: Option<String>,
    pub executable: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonRpcNotification {
    pub method: String,
    pub params: Value,
}

struct ClientState {
    pending: Mutex<HashMap<u32, mpsc::Sender<Result<Value, String>>>>,
    notifications: Mutex<Vec<JsonRpcNotification>>,
    stderr_lines: Mutex<Vec<String>>,
}

impl ClientState {
    fn new() -> Self {
        Self {
            pending: Mutex::new(HashMap::new()),
            notifications: Mutex::new(Vec::new()),
            stderr_lines: Mutex::new(Vec::new()),
        }
    }
}

struct ClientInner {
    child: Mutex<Child>,
    stdin: Mutex<Option<ChildStdin>>,
    state: Arc<ClientState>,
    reader_handle: Mutex<Option<JoinHandle<()>>>,
    stderr_handle: Mutex<Option<JoinHandle<()>>>,
}

impl Drop for ClientInner {
    fn drop(&mut self) {
        let _ = self.stdin.lock().unwrap().take();

        {
            let mut child = self.child.lock().unwrap();
            if child.try_wait().ok().flatten().is_none() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }

        if let Some(handle) = self.reader_handle.lock().unwrap().take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.stderr_handle.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}

#[derive(Clone)]
pub struct PetJsonRpcClient {
    inner: Arc<ClientInner>,
}

impl PetJsonRpcClient {
    pub fn spawn() -> Result<Self, String> {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_pet"));
        cmd.arg("server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // Clear all inherited env vars to prevent host-specific tool
            // configuration from leaking into the test environment, then
            // restore only the minimum required for the OS to function.
            .env_clear()
            .env("PATH", "");
        // On Windows, SYSTEMROOT is required for basic OS functionality
        // (crypto, networking, etc.). Only set it when present.
        #[cfg(windows)]
        if let Ok(val) = std::env::var("SYSTEMROOT") {
            cmd.env("SYSTEMROOT", val);
        }
        let mut process = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn pet server: {e}"))?;

        let stdin = process
            .stdin
            .take()
            .ok_or("Failed to capture pet stdin".to_string())?;
        let stdout = process
            .stdout
            .take()
            .ok_or("Failed to capture pet stdout".to_string())?;
        let stderr = process
            .stderr
            .take()
            .ok_or("Failed to capture pet stderr".to_string())?;

        let state = Arc::new(ClientState::new());
        let reader_handle = spawn_stdout_reader(stdout, state.clone());
        let stderr_handle = spawn_stderr_reader(stderr, state.clone());

        Ok(Self {
            inner: Arc::new(ClientInner {
                child: Mutex::new(process),
                stdin: Mutex::new(Some(stdin)),
                state,
                reader_handle: Mutex::new(Some(reader_handle)),
                stderr_handle: Mutex::new(Some(stderr_handle)),
            }),
        })
    }

    pub fn configure(&self, config: Value) -> Result<(), String> {
        self.send_request_value("configure", config, DEFAULT_REQUEST_TIMEOUT)
            .map(|_| ())
    }

    pub fn refresh(&self, params: Option<Value>) -> Result<RefreshResult, String> {
        self.send_request(
            "refresh",
            params.unwrap_or_else(|| json!({})),
            DEFAULT_REQUEST_TIMEOUT,
        )
    }

    pub fn info(&self) -> Result<PetInfoResponse, String> {
        self.send_request("info", json!({}), DEFAULT_REQUEST_TIMEOUT)
    }

    #[allow(dead_code)]
    pub fn resolve(&self, executable: &str) -> Result<Value, String> {
        self.send_request_value(
            "resolve",
            json!({ "executable": executable }),
            DEFAULT_REQUEST_TIMEOUT,
        )
    }

    pub fn clear_notifications(&self) {
        self.inner.state.notifications.lock().unwrap().clear();
    }

    pub fn notifications(&self) -> Vec<JsonRpcNotification> {
        self.inner.state.notifications.lock().unwrap().clone()
    }

    pub fn notification_count(&self, method: &str) -> usize {
        self.notifications()
            .into_iter()
            .filter(|notification| notification.method == method)
            .count()
    }

    pub fn telemetry_events(&self, event: &str) -> Vec<Value> {
        self.notifications()
            .into_iter()
            .filter(|notification| {
                notification.method == "telemetry"
                    && notification.params["event"].as_str() == Some(event)
            })
            .map(|notification| notification.params)
            .collect()
    }

    pub fn telemetry_event_count(&self, event: &str) -> usize {
        self.telemetry_events(event).len()
    }

    pub fn wait_for_telemetry_event_count(
        &self,
        event: &str,
        expected_count: usize,
        timeout: Duration,
    ) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        while Instant::now() <= deadline {
            if self.telemetry_event_count(event) >= expected_count {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }
        Err(format!(
            "Timed out waiting for {expected_count} '{event}' telemetry events; saw {}. stderr: {}",
            self.telemetry_event_count(event),
            self.stderr_output()
        ))
    }
    pub fn wait_for_notification_count(
        &self,
        method: &str,
        expected_count: usize,
        timeout: Duration,
    ) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        while Instant::now() <= deadline {
            if self.notification_count(method) >= expected_count {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }
        Err(format!(
            "Timed out waiting for {expected_count} '{method}' notifications; saw {}. stderr: {}",
            self.notification_count(method),
            self.stderr_output()
        ))
    }

    pub fn environment_notifications(&self) -> Vec<EnvironmentNotification> {
        self.notifications()
            .into_iter()
            .filter(|notification| notification.method == "environment")
            .map(|notification| {
                serde_json::from_value(notification.params)
                    .expect("environment notification payload should deserialize")
            })
            .collect()
    }

    pub fn manager_notifications(&self) -> Vec<ManagerNotification> {
        self.notifications()
            .into_iter()
            .filter(|notification| notification.method == "manager")
            .map(|notification| {
                serde_json::from_value(notification.params)
                    .expect("manager notification payload should deserialize")
            })
            .collect()
    }

    pub fn stderr_output(&self) -> String {
        self.inner.state.stderr_lines.lock().unwrap().join("")
    }

    fn send_request<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<T, String> {
        let result = self.send_request_value(method, params, timeout)?;
        serde_json::from_value(result)
            .map_err(|e| format!("Failed to deserialize response for {method}: {e}"))
    }

    fn send_request_value(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, String> {
        let id = REQUEST_ID.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });
        let request_text = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize {method} request: {e}"))?;
        let wire_message = format!(
            "Content-Length: {}\r\n\r\n{}",
            request_text.len(),
            request_text
        );

        let (tx, rx) = mpsc::channel();
        self.inner.state.pending.lock().unwrap().insert(id, tx);

        let write_result = {
            let mut stdin_guard = self.inner.stdin.lock().unwrap();
            let stdin = stdin_guard
                .as_mut()
                .ok_or_else(|| "PET stdin is no longer available".to_string())?;
            stdin
                .write_all(wire_message.as_bytes())
                .and_then(|_| stdin.flush())
                .map_err(|e| format!("Failed to send {method} request: {e}"))
        };

        if let Err(err) = write_result {
            self.inner.state.pending.lock().unwrap().remove(&id);
            return Err(err);
        }

        match rx.recv_timeout(timeout) {
            Ok(result) => result,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.inner.state.pending.lock().unwrap().remove(&id);
                Err(format!(
                    "Timed out waiting for {method} response after {timeout:?}"
                ))
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(format!(
                "Response channel disconnected while waiting for {method}; stderr: {}",
                self.stderr_output()
            )),
        }
    }
}

fn spawn_stdout_reader(stdout: ChildStdout, state: Arc<ClientState>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let read_result = loop {
            match read_message(&mut reader) {
                Ok(Some(message)) => {
                    if let Some(method) = message.get("method").and_then(|value| value.as_str()) {
                        state
                            .notifications
                            .lock()
                            .unwrap()
                            .push(JsonRpcNotification {
                                method: method.to_string(),
                                params: message.get("params").cloned().unwrap_or(Value::Null),
                            });
                        continue;
                    }

                    if let Some(id) = message.get("id").and_then(|value| value.as_u64()) {
                        if let Some(sender) = state.pending.lock().unwrap().remove(&(id as u32)) {
                            if let Some(error) = message.get("error") {
                                let _ = sender.send(Err(format!("JSONRPC error: {error:?}")));
                            } else {
                                let _ = sender.send(Ok(message
                                    .get("result")
                                    .cloned()
                                    .unwrap_or(Value::Null)));
                            }
                        }
                    }
                }
                Ok(None) => break Ok(()),
                Err(err) => break Err(err),
            }
        };

        let failure = match read_result {
            Ok(()) => "PET stdout closed".to_string(),
            Err(err) => format!("Failed to read PET stdout: {err}"),
        };
        let pending = std::mem::take(&mut *state.pending.lock().unwrap());
        for (_, sender) in pending {
            let _ = sender.send(Err(failure.clone()));
        }
    })
}

fn spawn_stderr_reader(
    stderr: impl Read + Send + 'static,
    state: Arc<ClientState>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => return,
                Ok(_) => state.stderr_lines.lock().unwrap().push(line),
                Err(err) => {
                    state
                        .stderr_lines
                        .lock()
                        .unwrap()
                        .push(format!("Failed to read PET stderr: {err}\n"));
                    return;
                }
            }
        }
    })
}

fn read_message(reader: &mut BufReader<ChildStdout>) -> io::Result<Option<Value>> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut header = String::new();
        let bytes_read = reader.read_line(&mut header)?;
        if bytes_read == 0 {
            if content_length.is_none() {
                return Ok(None);
            }
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "unexpected EOF while reading JSONRPC headers",
            ));
        }

        let trimmed = header.trim();
        if trimmed.is_empty() {
            break;
        }

        if let Some(length) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(length.parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid Content-Length header: {e}"),
                )
            })?);
        }
    }

    let content_length = content_length.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length header")
    })?;
    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body)?;
    let body_text = String::from_utf8(body).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid UTF-8 body: {e}"),
        )
    })?;
    let message = serde_json::from_str::<Value>(&body_text).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid JSONRPC payload: {e}"),
        )
    })?;
    Ok(Some(message))
}
