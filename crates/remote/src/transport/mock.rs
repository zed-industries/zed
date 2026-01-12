//! Mock transport for testing remote connections.
//!
//! This module provides a mock implementation of the `RemoteConnection` trait
//! that allows testing remote editing functionality without actual SSH/WSL/Docker
//! connections.
//!
//! # Usage
//!
//! ```rust,ignore
//! use remote::{MockConnection, RemoteClient};
//!
//! #[gpui::test]
//! async fn test_remote_editing(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
//!     let (opts, server_session) = MockConnection::new(cx, server_cx);
//!
//!     // Create the headless project (server side)
//!     server_cx.update(HeadlessProject::init);
//!     let _headless = server_cx.new(|cx| {
//!         HeadlessProject::new(
//!             HeadlessAppState { session: server_session, /* ... */ },
//!             false,
//!             cx,
//!         )
//!     });
//!
//!     // Create the client using the helper
//!     let (client, server_client) = RemoteClient::new_mock(cx, server_cx).await;
//!     // ... test logic ...
//! }
//! ```

use crate::remote_client::{
    ChannelClient, CommandTemplate, RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions,
};
use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    FutureExt, SinkExt, StreamExt,
    channel::{
        mpsc::{self, Sender},
        oneshot,
    },
    select_biased,
};
use gpui::{App, AppContext as _, AsyncApp, Global, Task, TestAppContext};
use rpc::{AnyProtoClient, proto::Envelope};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
use util::paths::{PathStyle, RemotePathBuf};

/// Unique identifier for a mock connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MockConnectionOptions {
    pub id: u64,
}

/// A mock implementation of `RemoteConnection` for testing.
pub struct MockRemoteConnection {
    options: MockConnectionOptions,
    server_channel: Arc<ChannelClient>,
    server_cx: SendableCx,
}

/// Wrapper to pass `AsyncApp` across thread boundaries in tests.
///
/// # Safety
///
/// This is safe because in test mode, GPUI is always single-threaded and so
/// having access to one async app means being on the same main thread.
pub(crate) struct SendableCx(AsyncApp);

impl SendableCx {
    pub(crate) fn new(cx: &TestAppContext) -> Self {
        Self(cx.to_async())
    }

    pub(crate) fn get(&self, _: &AsyncApp) -> AsyncApp {
        self.0.clone()
    }
}

// SAFETY: In test mode, GPUI is always single-threaded, and SendableCx
// is only accessed from the main thread via the get() method which
// requires a valid AsyncApp reference.
unsafe impl Send for SendableCx {}
unsafe impl Sync for SendableCx {}

/// Global registry that holds pre-created mock connections.
///
/// When `ConnectionPool::connect` is called with `MockConnectionOptions`,
/// it retrieves the connection from this registry.
#[derive(Default)]
pub struct MockConnectionRegistry {
    pending: HashMap<MockConnectionOptions, (oneshot::Receiver<()>, Arc<MockRemoteConnection>)>,
}

impl Global for MockConnectionRegistry {}

impl MockConnectionRegistry {
    /// Called by `ConnectionPool::connect` to retrieve a pre-registered mock connection.
    pub fn take(
        &mut self,
        opts: &MockConnectionOptions,
    ) -> Option<impl Future<Output = Arc<MockRemoteConnection>> + use<>> {
        let (guard, con) = self.pending.remove(opts)?;
        Some(async move {
            _ = guard.await;
            con
        })
    }
}

/// Helper for creating mock connection pairs in tests.
pub struct MockConnection;

pub type ConnectGuard = oneshot::Sender<()>;

impl MockConnection {
    /// Creates a new mock connection pair for testing.
    ///
    /// This function:
    /// 1. Creates a unique `MockConnectionOptions` identifier
    /// 2. Sets up the server-side channel (returned as `AnyProtoClient`)
    /// 3. Creates a `MockRemoteConnection` and registers it in the global registry
    /// 4. The connection will be retrieved from the registry when `ConnectionPool::connect` is called
    ///
    /// Returns:
    /// - `MockConnectionOptions` to pass to `remote::connect()` or `RemoteClient` creation
    /// - `AnyProtoClient` to pass to `HeadlessProject::new()` as the session
    ///
    /// # Arguments
    /// - `client_cx`: The test context for the client side
    /// - `server_cx`: The test context for the server/headless side
    pub(crate) fn new(
        client_cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) -> (MockConnectionOptions, AnyProtoClient, ConnectGuard) {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let opts = MockConnectionOptions { id };

        let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
        let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
        let server_client = server_cx
            .update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx, "mock-server", false));

        let connection = Arc::new(MockRemoteConnection {
            options: opts.clone(),
            server_channel: server_client.clone(),
            server_cx: SendableCx::new(server_cx),
        });

        let (tx, rx) = oneshot::channel();

        client_cx.update(|cx| {
            cx.default_global::<MockConnectionRegistry>()
                .pending
                .insert(opts.clone(), (rx, connection));
        });

        (opts, server_client.into(), tx)
    }
}

#[async_trait(?Send)]
impl RemoteConnection for MockRemoteConnection {
    async fn kill(&self) -> Result<()> {
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        false
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        _working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let shell_program = program.unwrap_or_else(|| "sh".to_string());
        let mut shell_args = Vec::new();
        shell_args.push(shell_program);
        shell_args.extend(args.iter().cloned());
        Ok(CommandTemplate {
            program: "mock".into(),
            args: shell_args,
            env: env.clone(),
        })
    }

    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Ok(CommandTemplate {
            program: "mock".into(),
            args: std::iter::once("-N".to_owned())
                .chain(forwards.into_iter().map(|(local_port, host, remote_port)| {
                    format!("{local_port}:{host}:{remote_port}")
                }))
                .collect(),
            env: Default::default(),
        })
    }

    fn upload_directory(
        &self,
        _src_path: PathBuf,
        _dest_path: RemotePathBuf,
        _cx: &App,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Mock(self.options.clone())
    }

    fn simulate_disconnect(&self, cx: &AsyncApp) {
        let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
        let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
        self.server_channel
            .reconnect(incoming_rx, outgoing_tx, &self.server_cx.get(cx));
    }

    fn start_proxy(
        &self,
        _unique_identifier: String,
        _reconnect: bool,
        mut client_incoming_tx: mpsc::UnboundedSender<Envelope>,
        mut client_outgoing_rx: mpsc::UnboundedReceiver<Envelope>,
        mut connection_activity_tx: Sender<()>,
        _delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        let (mut server_incoming_tx, server_incoming_rx) = mpsc::unbounded::<Envelope>();
        let (server_outgoing_tx, mut server_outgoing_rx) = mpsc::unbounded::<Envelope>();

        self.server_channel.reconnect(
            server_incoming_rx,
            server_outgoing_tx,
            &self.server_cx.get(cx),
        );

        cx.background_spawn(async move {
            loop {
                select_biased! {
                    server_to_client = server_outgoing_rx.next().fuse() => {
                        let Some(server_to_client) = server_to_client else {
                            return Ok(1)
                        };
                        connection_activity_tx.try_send(()).ok();
                        client_incoming_tx.send(server_to_client).await.ok();
                    }
                    client_to_server = client_outgoing_rx.next().fuse() => {
                        let Some(client_to_server) = client_to_server else {
                            return Ok(1)
                        };
                        server_incoming_tx.send(client_to_server).await.ok();
                    }
                }
            }
        })
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::local()
    }

    fn shell(&self) -> String {
        "sh".to_owned()
    }

    fn default_system_shell(&self) -> String {
        "sh".to_owned()
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

/// Mock delegate for tests that don't need delegate functionality.
pub struct MockDelegate;

impl RemoteClientDelegate for MockDelegate {
    fn ask_password(
        &self,
        _prompt: String,
        _sender: futures::channel::oneshot::Sender<askpass::EncryptedPassword>,
        _cx: &mut AsyncApp,
    ) {
        unreachable!("MockDelegate::ask_password should not be called in tests")
    }

    fn download_server_binary_locally(
        &self,
        _platform: crate::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<PathBuf>> {
        unreachable!("MockDelegate::download_server_binary_locally should not be called in tests")
    }

    fn get_download_url(
        &self,
        _platform: crate::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<Option<String>>> {
        unreachable!("MockDelegate::get_download_url should not be called in tests")
    }

    fn set_status(&self, _status: Option<&str>, _cx: &mut AsyncApp) {}
}
