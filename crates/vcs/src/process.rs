use crate::{
    Content, InitializeParams, InitializeResult, MAX_MESSAGE_BYTES, PROTOCOL_VERSION,
    ProviderError, ProviderInfo, RepositoryDescriptor, Snapshot, validate_path,
};
use anyhow::{Context as _, Result, bail, ensure};
use collections::HashMap;
use futures::{
    AsyncBufRead, AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _,
    channel::oneshot, io::BufReader,
};
use gpui::{BackgroundExecutor, Task};
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
use util::process::Child;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_HEADER_BYTES: usize = 8192;
const STDERR_LIMIT: usize = 64 * 1024;
type Pending = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value>>>>>;

#[derive(Clone, Debug)]
pub struct ProviderCommand {
    pub path: PathBuf,
    pub arguments: Vec<OsString>,
    pub env: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum ProviderNotification {
    DidChange { repository_id: String },
    Exited { message: String },
}

pub struct ProviderConnection {
    outbound: async_channel::Sender<Value>,
    notifications: async_channel::Receiver<ProviderNotification>,
    pending: Pending,
    writer_done: Mutex<Option<oneshot::Receiver<()>>>,
    next_id: AtomicU64,
    alive: Arc<AtomicBool>,
    initialized: AtomicBool,
    child: Arc<Mutex<Option<Child>>>,
    diagnostics: Arc<Mutex<String>>,
    executor: BackgroundExecutor,
    _tasks: Vec<Task<()>>,
}

impl ProviderConnection {
    /// The caller must establish workspace trust before starting the executable.
    pub fn start(
        command: ProviderCommand,
        cwd: &Path,
        executor: BackgroundExecutor,
    ) -> Result<Arc<Self>> {
        let mut process = std::process::Command::new(&command.path);
        process
            .args(&command.arguments)
            .envs(&command.env)
            .current_dir(cwd);
        let mut child = Child::spawn(process, Stdio::piped(), Stdio::piped(), Stdio::piped())?;
        let stdin = child
            .stdin
            .take()
            .context("VCS provider stdin is unavailable")?;
        let stdout = child
            .stdout
            .take()
            .context("VCS provider stdout is unavailable")?;
        let stderr = child
            .stderr
            .take()
            .context("VCS provider stderr is unavailable")?;
        let child = Arc::new(Mutex::new(Some(child)));
        let pending: Pending = Arc::default();
        let alive = Arc::new(AtomicBool::new(true));
        let diagnostics = Arc::new(Mutex::new(String::new()));
        let (outbound, outgoing) = async_channel::bounded::<Value>(128);
        let (notification_sender, notifications) = async_channel::bounded(128);
        let (writer_done_sender, writer_done) = oneshot::channel();

        let writer = executor.spawn({
            let pending = pending.clone();
            let alive = alive.clone();
            let notification_sender = notification_sender.clone();
            let child = child.clone();
            async move {
                let result: Result<()> = async {
                    let mut stdin = stdin;
                    while let Ok(message) = outgoing.recv().await {
                        let bytes = serde_json::to_vec(&message)?;
                        ensure!(
                            bytes.len() <= MAX_MESSAGE_BYTES,
                            "VCS request exceeds the message limit"
                        );
                        stdin
                            .write_all(
                                format!("Content-Length: {}\r\n\r\n", bytes.len()).as_bytes(),
                            )
                            .await?;
                        stdin.write_all(&bytes).await?;
                        stdin.flush().await?;
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = result {
                    fail_connection(
                        &pending,
                        &alive,
                        &notification_sender,
                        format!("VCS provider input failed: {error:#}"),
                    );
                    kill_child(&child);
                }
                if writer_done_sender.send(()).is_err() {
                    log::debug!("VCS writer completed after the connection closed");
                }
            }
        });

        let reader = executor.spawn({
            let pending = pending.clone();
            let alive = alive.clone();
            let child = child.clone();
            async move {
                let result: Result<()> = async {
                    let mut stdout = BufReader::new(stdout);
                    loop {
                        let message = read_message(&mut stdout).await?;
                        ensure!(
                            message.get("jsonrpc").and_then(Value::as_str) == Some("2.0"),
                            "invalid VCS JSON-RPC version"
                        );
                        ensure!(
                            message.get("method").is_none() || message.get("id").is_none(),
                            "VCS providers cannot issue requests to this client"
                        );
                        if let Some(id) = message.get("id").and_then(Value::as_u64) {
                            ensure!(
                                message.get("result").is_some() != message.get("error").is_some(),
                                "VCS response must contain exactly one result or error"
                            );
                            let response = if let Some(error) = message.get("error") {
                                Err(anyhow::Error::new(serde_json::from_value::<ProviderError>(
                                    error.clone(),
                                )?))
                            } else {
                                message
                                    .get("result")
                                    .cloned()
                                    .context("VCS response has no result")
                            };
                            if let Some(sender) = pending.lock().remove(&id) {
                                // A cancelled requester deliberately drops its receiver.
                                if sender.send(response).is_err() {
                                    log::debug!("VCS response arrived after request cancellation");
                                }
                            }
                        } else if message.get("method").and_then(Value::as_str)
                            == Some("vcs/didChange")
                        {
                            let repository_id = message
                                .get("params")
                                .and_then(|params| params.get("repositoryId"))
                                .and_then(Value::as_str)
                                .context("VCS change notification has no repository id")?;
                            match notification_sender.try_send(ProviderNotification::DidChange {
                                repository_id: repository_id.to_owned(),
                            }) {
                                Ok(()) | Err(async_channel::TrySendError::Full(_)) => {}
                                Err(async_channel::TrySendError::Closed(_)) => break,
                            }
                        } else if message.get("id").is_some() {
                            bail!("VCS providers cannot issue requests to this client");
                        }
                    }
                    Ok(())
                }
                .await;
                let message = match result {
                    Ok(()) => "VCS provider connection closed".to_owned(),
                    Err(error) => format!("VCS provider connection closed: {error:#}"),
                };
                if fail_connection(&pending, &alive, &notification_sender, message) {
                    kill_child(&child);
                }
            }
        });

        let stderr_task = executor.spawn({
            let diagnostics = diagnostics.clone();
            async move {
                let mut stderr = stderr;
                let mut bytes = [0; 4096];
                loop {
                    match stderr.read(&mut bytes).await {
                        Ok(0) => break,
                        Ok(count) => {
                            let mut diagnostics = diagnostics.lock();
                            diagnostics.push_str(&String::from_utf8_lossy(&bytes[..count]));
                            if diagnostics.len() > STDERR_LIMIT {
                                let mut start = diagnostics.len() - STDERR_LIMIT;
                                while !diagnostics.is_char_boundary(start) {
                                    start += 1;
                                }
                                diagnostics.drain(..start);
                            }
                        }
                        Err(error) => {
                            log::warn!("failed to read VCS provider diagnostics: {error}");
                            break;
                        }
                    }
                }
            }
        });

        Ok(Arc::new(Self {
            outbound,
            notifications,
            pending,
            writer_done: Mutex::new(Some(writer_done)),
            next_id: AtomicU64::new(1),
            alive,
            initialized: AtomicBool::new(false),
            child,
            diagnostics,
            executor,
            _tasks: vec![writer, reader, stderr_task],
        }))
    }

    pub fn notifications(&self) -> async_channel::Receiver<ProviderNotification> {
        self.notifications.clone()
    }

    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Acquire)
    }

    pub fn diagnostics(&self) -> String {
        self.diagnostics.lock().clone()
    }

    #[cfg(test)]
    pub(crate) fn has_taken_shutdown_child(&self) -> bool {
        self.child.lock().is_none()
    }

    pub async fn initialize(&self) -> Result<InitializeResult> {
        ensure!(
            !self.initialized.load(Ordering::Acquire),
            "VCS provider is already initialized"
        );
        let params = InitializeParams {
            protocol_version: PROTOCOL_VERSION,
            client_info: ProviderInfo {
                name: "Zed".into(),
                version: "1".into(),
            },
        };
        let result: InitializeResult = self
            .request("initialize", serde_json::to_value(params)?, REQUEST_TIMEOUT)
            .await?;
        result.validate()?;
        self.initialized.store(true, Ordering::Release);
        Ok(result)
    }

    pub async fn discover(&self, scope_uri: String) -> Result<Vec<RepositoryDescriptor>> {
        self.ensure_initialized()?;
        let repositories: Vec<RepositoryDescriptor> = self
            .request(
                "vcs/discover",
                json!({"scopeUri":scope_uri}),
                REQUEST_TIMEOUT,
            )
            .await?;
        let mut ids = std::collections::HashSet::new();
        for repository in &repositories {
            repository.validate(&scope_uri)?;
            ensure!(ids.insert(&repository.id), "duplicate VCS repository id");
        }
        Ok(repositories)
    }

    pub async fn snapshot(&self, repository_id: String, baseline_id: String) -> Result<Snapshot> {
        self.ensure_initialized()?;
        let snapshot: Snapshot = self
            .request(
                "vcs/snapshot",
                json!({"repositoryId":repository_id,"baselineId":baseline_id}),
                REQUEST_TIMEOUT,
            )
            .await?;
        snapshot.validate(&repository_id, &baseline_id)?;
        Ok(snapshot)
    }

    pub async fn read_content(
        &self,
        repository_id: String,
        snapshot_id: String,
        baseline_id: String,
        path: String,
    ) -> Result<Content> {
        self.ensure_initialized()?;
        validate_path(&path)?;
        self.request("vcs/readContent", json!({"repositoryId":repository_id,"snapshotId":snapshot_id,"baselineId":baseline_id,"path":path}), REQUEST_TIMEOUT).await
    }

    pub async fn shutdown(&self) -> Result<()> {
        let result = if self.is_alive() {
            self.request::<Value>("shutdown", json!({}), SHUTDOWN_TIMEOUT)
                .await
                .map(|_| ())
        } else {
            Ok(())
        };
        if let Err(error) = self
            .outbound
            .try_send(json!({"jsonrpc":"2.0","method":"exit"}))
        {
            log::debug!("VCS exit notification was not queued: {error}");
        }
        self.outbound.close();
        self.alive.store(false, Ordering::Release);
        for (_, sender) in self.pending.lock().drain() {
            if sender
                .send(Err(anyhow::anyhow!("VCS provider is shutting down")))
                .is_err()
            {
                log::debug!("VCS requester closed during shutdown");
            }
        }
        self.notifications.close();
        let writer_done = self.writer_done.lock().take();
        if let Some(writer_done) = writer_done {
            let drained = writer_done.fuse();
            let timeout = self.executor.timer(SHUTDOWN_TIMEOUT).fuse();
            futures::pin_mut!(drained, timeout);
            futures::select_biased! {
                result = drained => if result.is_err() { log::debug!("VCS writer stopped before draining exit"); },
                _ = timeout => log::warn!("VCS provider exit notification did not drain before deadline"),
            }
        }
        self.reap_child().await?;
        result
    }

    fn ensure_initialized(&self) -> Result<()> {
        ensure!(
            self.initialized.load(Ordering::Acquire),
            "VCS provider has not initialized"
        );
        ensure!(self.is_alive(), "VCS provider is no longer running");
        Ok(())
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: &'static str,
        params: Value,
        timeout: Duration,
    ) -> Result<T> {
        ensure!(self.is_alive(), "VCS provider is no longer running");
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = oneshot::channel();
        {
            let mut pending = self.pending.lock();
            ensure!(self.is_alive(), "VCS provider is no longer running");
            ensure!(pending.len() < 1024, "too many pending VCS requests");
            pending.insert(id, sender);
        }
        let guard = RequestGuard {
            id,
            pending: self.pending.clone(),
            outbound: self.outbound.clone(),
        };
        self.outbound
            .try_send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .context("VCS provider request queue is unavailable")?;
        let response = receiver.fuse();
        let timer = self.executor.timer(timeout).fuse();
        futures::pin_mut!(response, timer);
        let result = futures::select_biased! {
            result = response => result.context("VCS provider response was interrupted")?,
            _ = timer => bail!("VCS provider request {method} timed out"),
        }?;
        drop(guard);
        serde_json::from_value(result)
            .with_context(|| format!("invalid VCS provider response to {method}"))
    }

    async fn reap_child(&self) -> Result<()> {
        let child = self.child.lock().take();
        if let Some(child) = child {
            ChildCleanup {
                child: Some(child),
                executor: self.executor.clone(),
            }
            .finish()
            .await?;
        }
        Ok(())
    }
}

impl Drop for ProviderConnection {
    fn drop(&mut self) {
        self.outbound.close();
        self.alive.store(false, Ordering::Release);
        self.pending.lock().clear();
        if let Some(child) = self.child.lock().take() {
            drop(ChildCleanup {
                child: Some(child),
                executor: self.executor.clone(),
            });
        }
    }
}

// The cleanup owner must survive cancellation of a shutdown waiter. On Unix,
// util::process::Child alone does not terminate the child when it is dropped.
struct ChildCleanup {
    child: Option<Child>,
    executor: BackgroundExecutor,
}

impl ChildCleanup {
    async fn finish(mut self) -> Result<()> {
        let child = self.child.as_mut().context("VCS cleanup lost its child")?;
        let natural_exit = {
            let status = child.status().fuse();
            let timeout = self.executor.timer(SHUTDOWN_TIMEOUT).fuse();
            futures::pin_mut!(status, timeout);
            futures::select_biased! {
                result = status => Some(result),
                _ = timeout => None,
            }
        };
        if let Some(status) = natural_exit {
            status.context("failed to wait for VCS provider shutdown")?;
        }
        // A provider can exit before its workers; the owned process group still
        // needs cleanup after the leader finishes its graceful shutdown.
        child
            .kill()
            .context("failed to stop VCS provider process group")?;
        child
            .status()
            .await
            .context("failed to reap VCS provider process")?;
        drop(self.child.take());
        Ok(())
    }
}

impl Drop for ChildCleanup {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            if let Err(error) = child.kill() {
                log::warn!("failed to stop VCS provider on close: {error:#}");
            }
            // Reaping must outlive a GPUI context. Its retiring test dispatcher
            // can drop a runnable synchronously from the process reaper's waker.
            smol::spawn(async move {
                if let Err(error) = child.status().await {
                    log::warn!("failed to reap VCS provider on close: {error}");
                }
            })
            .detach();
        }
    }
}

struct RequestGuard {
    id: u64,
    pending: Pending,
    outbound: async_channel::Sender<Value>,
}

impl Drop for RequestGuard {
    fn drop(&mut self) {
        if self.pending.lock().remove(&self.id).is_some() {
            if let Err(error) = self.outbound.try_send(
                json!({"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":self.id}}),
            ) {
                log::debug!("VCS cancellation could not be queued: {error}");
            }
        }
    }
}

fn kill_child(child: &Arc<Mutex<Option<Child>>>) {
    if let Some(child) = child.lock().as_mut() {
        if let Err(error) = child.kill() {
            log::warn!("failed to stop disconnected VCS provider: {error:#}");
        }
    }
}

fn fail_connection(
    pending: &Pending,
    alive: &AtomicBool,
    notifications: &async_channel::Sender<ProviderNotification>,
    message: String,
) -> bool {
    let was_alive = alive.swap(false, Ordering::AcqRel);
    if was_alive {
        for (_, sender) in pending.lock().drain() {
            if sender.send(Err(anyhow::anyhow!(message.clone()))).is_err() {
                log::debug!("VCS requester closed before transport failure");
            }
        }
        match notifications.try_send(ProviderNotification::Exited { message }) {
            Ok(()) | Err(async_channel::TrySendError::Closed(_)) => {}
            Err(async_channel::TrySendError::Full(_)) => {
                log::debug!("VCS notification queue full while disconnecting")
            }
        }
    }
    notifications.close();
    was_alive
}

async fn read_message<R: AsyncBufRead + Unpin>(reader: &mut R) -> Result<Value> {
    let mut headers = Vec::new();
    let mut length = None;
    loop {
        let available = reader.fill_buf().await?;
        ensure!(
            !available.is_empty(),
            "unexpected EOF in VCS message headers"
        );
        let count = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |index| index + 1);
        ensure!(
            headers.len() + count <= MAX_HEADER_BYTES,
            "VCS message headers exceed the limit"
        );
        headers.extend_from_slice(&available[..count]);
        reader.consume_unpin(count);
        if headers.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    for line in std::str::from_utf8(&headers)?.split("\r\n") {
        if let Some(value) = line.strip_prefix("Content-Length:") {
            ensure!(length.is_none(), "duplicate VCS Content-Length header");
            length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .context("invalid VCS message length")?,
            );
        }
    }
    let length = length.context("missing VCS Content-Length header")?;
    ensure!(length <= MAX_MESSAGE_BYTES, "VCS message exceeds the limit");
    let mut bytes = vec![0; length];
    reader.read_exact(&mut bytes).await?;
    serde_json::from_slice(&bytes).context("invalid VCS JSON message")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framing_accepts_bytes_not_character_lengths_and_rejects_invalid_frames() {
        smol::block_on(async {
            let value = json!({"jsonrpc":"2.0","id":1,"result":"Пример"});
            let body = serde_json::to_vec(&value).unwrap();
            let mut frame = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
            frame.extend(body);
            assert_eq!(read_message(&mut frame.as_slice()).await.unwrap(), value);
            for mut invalid in [
                b"Content-Length: 1\r\nContent-Length: 1\r\n\r\n0".as_slice(),
                b"Content-Length: 999999999\r\n\r\n",
                b"Content-Length: 10\r\n\r\n{}",
                b"Content-Length: -1\r\n\r\n",
            ] {
                assert!(read_message(&mut invalid).await.is_err());
            }
        });
    }
}
