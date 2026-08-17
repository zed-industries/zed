//! A mock Jupyter kernel for testing frontend implementations.
//!
//! TestKernel implements the Jupyter messaging protocol with predictable behavior,
//! making it ideal for testing Jupyter clients without requiring a real kernel process.
//!
//! # Overview
//!
//! TestKernel provides a lightweight mock kernel that:
//! - Responds to all standard Jupyter protocol messages
//! - Echoes executed code back as stdout by default (echo mode)
//! - Supports canned responses for testing rich media types
//! - Requires no external dependencies (no Python, no ipykernel)
//!
//! # Quick Start
//!
//! Add the `test-kernel` feature to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! runtimelib = { version = "1.2", features = ["test-kernel"] }
//! ```
//!
//! # Common Usage Patterns
//!
//! ## Basic Test Setup (Echo Mode)
//!
//! The simplest way to test your frontend is using echo mode, where executed code
//! is echoed back as stdout:
//!
//! ```ignore
//! use runtimelib::{
//!     TestKernel, TestKernelConfig,
//!     create_client_shell_connection_with_identity,
//!     create_client_iopub_connection,
//!     peer_identity_for_session,
//! };
//! use jupyter_protocol::{ExecuteRequest, JupyterMessage, JupyterMessageContent};
//! use uuid::Uuid;
//!
//! #[tokio::test]
//! async fn test_my_frontend() {
//!     // 1. Start the test kernel
//!     let (kernel_handle, connection_info) =
//!         TestKernel::start_ephemeral(TestKernelConfig::default()).await.unwrap();
//!
//!     // 2. Give the kernel time to bind its sockets
//!     tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
//!
//!     // 3. Connect your client (shell for requests, iopub for outputs)
//!     let session_id = Uuid::new_v4().to_string();
//!     let identity = peer_identity_for_session(&session_id).unwrap();
//!
//!     let mut shell = create_client_shell_connection_with_identity(
//!         &connection_info, &session_id, identity
//!     ).await.unwrap();
//!
//!     let mut iopub = create_client_iopub_connection(
//!         &connection_info, "", &session_id
//!     ).await.unwrap();
//!
//!     // 4. Wait for iopub subscription to establish
//!     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//!
//!     // 5. Send an execute request
//!     let request: JupyterMessage = ExecuteRequest::new("print('hello')".to_string()).into();
//!     shell.send(request).await.unwrap();
//!
//!     // 6. Read outputs from iopub (will receive "print('hello')" as stdout)
//!     // ... your test assertions here ...
//!
//!     // 7. Clean up
//!     kernel_handle.abort();
//! }
//! ```
//!
//! ## Testing Rich Media (Canned Responses)
//!
//! For testing how your frontend handles different output types (images, HTML, JSON),
//! configure canned responses:
//!
//! ```ignore
//! use runtimelib::{TestKernel, TestKernelConfig, CannedResponse};
//! use jupyter_protocol::{DisplayData, MediaType, ExecuteResult, ExecutionCount};
//!
//! #[tokio::test]
//! async fn test_image_display() {
//!     let config = TestKernelConfig::default()
//!         // When user executes "show_plot()", return an image
//!         .with_response("show_plot()", CannedResponse {
//!             outputs: vec![
//!                 DisplayData::from(MediaType::Png("iVBORw0KGgo...".to_string())).into(),
//!             ],
//!         })
//!         // When user executes "get_data()", return JSON
//!         .with_response("get_data()", CannedResponse {
//!             outputs: vec![
//!                 ExecuteResult {
//!                     execution_count: ExecutionCount::new(1),
//!                     data: jupyter_protocol::Media::from(
//!                         MediaType::Json(serde_json::json!({"key": "value"}))
//!                     ),
//!                     metadata: Default::default(),
//!                     transient: None,
//!                 }.into(),
//!             ],
//!         });
//!
//!     let (kernel_handle, connection_info) =
//!         TestKernel::start_ephemeral(config).await.unwrap();
//!
//!     // Now when you execute "show_plot()", you'll get a PNG display_data
//!     // When you execute "get_data()", you'll get a JSON execute_result
//!     // Any other code falls back to echo mode
//!
//!     kernel_handle.abort();
//! }
//! ```
//!
//! ## Testing Error Handling
//!
//! Test how your frontend handles kernel errors:
//!
//! ```ignore
//! use runtimelib::{TestKernel, TestKernelConfig, CannedResponse};
//! use jupyter_protocol::ErrorOutput;
//!
//! let config = TestKernelConfig::default()
//!     .with_response("raise_error()", CannedResponse {
//!         outputs: vec![
//!             ErrorOutput {
//!                 ename: "ValueError".to_string(),
//!                 evalue: "Something went wrong".to_string(),
//!                 traceback: vec![
//!                     "Traceback (most recent call last):".to_string(),
//!                     "  File \"<test>\", line 1, in <module>".to_string(),
//!                     "ValueError: Something went wrong".to_string(),
//!                 ],
//!             }.into(),
//!         ],
//!     });
//! ```
//!
//! ## Integration Testing with Connection Files
//!
//! For testing code that reads connection files (like real kernel launchers):
//!
//! ```ignore
//! use runtimelib::{TestKernel, TestKernelConfig};
//! use jupyter_protocol::ConnectionInfo;
//! use std::io::Write;
//!
//! #[tokio::test]
//! async fn test_connection_file_workflow() {
//!     // Create a connection file like Jupyter would
//!     let connection_info = ConnectionInfo {
//!         ip: "127.0.0.1".to_string(),
//!         transport: jupyter_protocol::Transport::TCP,
//!         shell_port: 55555,
//!         iopub_port: 55556,
//!         stdin_port: 55557,
//!         control_port: 55558,
//!         hb_port: 55559,
//!         key: "test-key".to_string(),
//!         signature_scheme: "hmac-sha256".to_string(),
//!         kernel_name: Some("test".to_string()),
//!     };
//!
//!     let temp_dir = tempfile::tempdir().unwrap();
//!     let conn_file = temp_dir.path().join("kernel.json");
//!     std::fs::write(&conn_file, serde_json::to_string(&connection_info).unwrap()).unwrap();
//!
//!     // Start kernel from file (like a real kernel would be started)
//!     let kernel_handle = TestKernel::start_from_file(
//!         &conn_file,
//!         TestKernelConfig::default()
//!     ).await.unwrap();
//!
//!     // Your code under test can now connect to the ports in the connection file
//!
//!     kernel_handle.abort();
//! }
//! ```
//!
//! # Message Types Supported
//!
//! TestKernel responds to these Jupyter protocol messages:
//!
//! | Message | Response |
//! |---------|----------|
//! | `kernel_info_request` | Returns kernel info (name: "test", protocol: 5.3) |
//! | `execute_request` | Echoes code as stdout, or sends canned response |
//! | `complete_request` | Returns empty completions |
//! | `inspect_request` | Returns `found: false` |
//! | `is_complete_request` | Returns `complete` |
//! | `history_request` | Returns empty history |
//! | `comm_info_request` | Returns empty comms |
//! | `shutdown_request` | Shuts down the kernel |
//!
//! # Tips
//!
//! - **ZMQ timing**: Always add a small delay (~50-100ms) after connecting iopub
//!   before sending requests, to allow the pub/sub subscription to establish.
//!
//! - **Message correlation**: Filter iopub messages by `parent_header.msg_id` to
//!   only process outputs related to your request.
//!
//! - **Cleanup**: Always call `handle.abort()` at the end of tests to clean up
//!   the kernel's background tasks.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;

use jupyter_protocol::{
    CodeMirrorMode, CommInfoReply, CompleteReply, ConnectionInfo, ExecuteInput, ExecuteReply,
    ExecutionCount, HistoryReply, InspectReply, IsCompleteReply, IsCompleteReplyStatus,
    JupyterMessage, JupyterMessageContent, KernelInfoReply, LanguageInfo, Media, ReplyStatus,
    ShutdownReply, Status, StreamContent, Transport,
};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::{
    create_kernel_control_connection, create_kernel_heartbeat_connection,
    create_kernel_iopub_connection, create_kernel_shell_connection, create_kernel_stdin_connection,
    peek_ports, KernelIoPubConnection, Result, RouterRecvConnection, RouterSendConnection,
};

/// A canned response for a specific code input.
///
/// When the TestKernel receives an `ExecuteRequest` with code matching a key
/// in `TestKernelConfig::responses`, it sends these outputs instead of the default echo.
#[derive(Clone, Debug, Default)]
pub struct CannedResponse {
    /// The outputs to send on the iopub channel.
    /// Can include `StreamContent`, `DisplayData`, `ExecuteResult`, `ErrorOutput`, etc.
    pub outputs: Vec<JupyterMessageContent>,
}

/// Configuration for TestKernel behavior.
#[derive(Clone, Debug, Default)]
pub struct TestKernelConfig {
    /// Map of code → canned responses.
    /// If code is not found in this map, the kernel falls back to echo mode
    /// (echoing the code as stdout).
    pub responses: HashMap<String, CannedResponse>,
}

impl TestKernelConfig {
    /// Create a new empty configuration (echo mode for all inputs).
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a canned response for a specific code input.
    pub fn with_response(mut self, code: impl Into<String>, response: CannedResponse) -> Self {
        self.responses.insert(code.into(), response);
        self
    }
}

/// A mock Jupyter kernel for testing frontend implementations.
///
/// TestKernel implements the Jupyter messaging protocol with simple, predictable behavior:
/// - By default, echoes executed code back as stdout (echo mode)
/// - Can be configured with canned responses for specific code inputs
/// - Responds to all standard Jupyter protocol messages
pub struct TestKernel {
    execution_count: ExecutionCount,
    iopub: KernelIoPubConnection,
    shell: RouterSendConnection,
    config: TestKernelConfig,
}

impl TestKernel {
    /// Start a TestKernel with the given ConnectionInfo.
    ///
    /// This is the primary constructor for tests where you create ConnectionInfo programmatically.
    ///
    /// # Arguments
    ///
    /// * `connection_info` - The connection info specifying ports and authentication
    /// * `config` - Configuration for kernel behavior (use `Default::default()` for echo mode)
    ///
    /// # Returns
    ///
    /// A `JoinHandle` that resolves when the kernel shuts down.
    pub async fn start(
        connection_info: ConnectionInfo,
        config: TestKernelConfig,
    ) -> Result<JoinHandle<Result<()>>> {
        let handle = tokio::spawn(async move { Self::run(&connection_info, config).await });
        Ok(handle)
    }

    /// Start a TestKernel from a connection file path.
    ///
    /// This mirrors how real kernels are started by Jupyter - useful for integration testing.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON connection file
    /// * `config` - Configuration for kernel behavior
    pub async fn start_from_file(
        path: impl AsRef<Path>,
        config: TestKernelConfig,
    ) -> Result<JoinHandle<Result<()>>> {
        let content = tokio::fs::read_to_string(path.as_ref()).await?;
        let connection_info: ConnectionInfo = serde_json::from_str(&content)?;
        Self::start(connection_info, config).await
    }

    /// Start a TestKernel with auto-assigned ports.
    ///
    /// This is the most convenient option for tests - it finds open ports automatically
    /// and returns the ConnectionInfo for clients to connect.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for kernel behavior
    ///
    /// # Returns
    ///
    /// A tuple of `(JoinHandle, ConnectionInfo)` - use the ConnectionInfo to connect clients.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let (handle, connection_info) = TestKernel::start_ephemeral(Default::default()).await?;
    ///
    /// // Connect a client
    /// let client = create_client_shell_connection(&connection_info, &session_id).await?;
    ///
    /// // ... run tests ...
    ///
    /// handle.abort();
    /// ```
    pub async fn start_ephemeral(
        config: TestKernelConfig,
    ) -> Result<(JoinHandle<Result<()>>, ConnectionInfo)> {
        let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ports = peek_ports(ip, 5).await?;

        let connection_info = ConnectionInfo {
            transport: Transport::TCP,
            ip: ip.to_string(),
            shell_port: ports[0],
            iopub_port: ports[1],
            stdin_port: ports[2],
            control_port: ports[3],
            hb_port: ports[4],
            signature_scheme: "hmac-sha256".to_string(),
            key: Uuid::new_v4().to_string(),
            kernel_name: Some("test".to_string()),
        };

        let handle = Self::start(connection_info.clone(), config).await?;
        Ok((handle, connection_info))
    }

    async fn run(connection_info: &ConnectionInfo, config: TestKernelConfig) -> Result<()> {
        let session_id = Uuid::new_v4().to_string();

        // Create all connections
        let mut heartbeat = create_kernel_heartbeat_connection(connection_info).await?;
        let shell_connection = create_kernel_shell_connection(connection_info, &session_id).await?;
        let (shell_writer, mut shell_reader) = shell_connection.split();
        let mut control_connection =
            create_kernel_control_connection(connection_info, &session_id).await?;
        let _stdin_connection =
            create_kernel_stdin_connection(connection_info, &session_id).await?;
        let iopub_connection = create_kernel_iopub_connection(connection_info, &session_id).await?;

        let mut kernel = Self {
            execution_count: Default::default(),
            iopub: iopub_connection,
            shell: shell_writer,
            config,
        };

        // Heartbeat task
        let heartbeat_handle =
            tokio::spawn(async move { while heartbeat.single_heartbeat().await.is_ok() {} });

        // Control channel task
        let control_handle = tokio::spawn(async move {
            while let Ok(message) = control_connection.read().await {
                match &message.content {
                    JupyterMessageContent::KernelInfoRequest(_) => {
                        let reply = Self::kernel_info().as_child_of(&message);
                        let _ = control_connection.send(reply).await;
                    }
                    JupyterMessageContent::ShutdownRequest(req) => {
                        let reply: JupyterMessage = ShutdownReply {
                            restart: req.restart,
                            status: ReplyStatus::Ok,
                            error: None,
                        }
                        .as_child_of(&message);
                        let _ = control_connection.send(reply).await;
                        return;
                    }
                    _ => {}
                }
            }
        });

        // Shell channel task
        let shell_handle = tokio::spawn(async move {
            if let Err(err) = kernel.handle_shell(&mut shell_reader).await {
                eprintln!("TestKernel shell error: {}", err);
            }
        });

        // Wait for any task to complete
        tokio::select! {
            _ = heartbeat_handle => {}
            _ = control_handle => {}
            _ = shell_handle => {}
        }

        Ok(())
    }

    async fn handle_shell(&mut self, reader: &mut RouterRecvConnection) -> Result<()> {
        loop {
            let msg = reader.read().await?;
            if let Err(err) = self.handle_shell_message(&msg).await {
                eprintln!("TestKernel error handling message: {}", err);
            }
        }
    }

    async fn handle_shell_message(&mut self, parent: &JupyterMessage) -> Result<()> {
        // Send busy status
        self.iopub.send(Status::busy().as_child_of(parent)).await?;

        match &parent.content {
            JupyterMessageContent::KernelInfoRequest(_) => {
                let reply = Self::kernel_info().as_child_of(parent);
                self.shell.send(reply).await?;
            }

            JupyterMessageContent::ExecuteRequest(req) => {
                self.handle_execute_request(&req.code, req.store_history, parent)
                    .await?;
            }

            JupyterMessageContent::CompleteRequest(req) => {
                let reply = CompleteReply {
                    matches: vec![],
                    cursor_start: req.cursor_pos,
                    cursor_end: req.cursor_pos,
                    metadata: Default::default(),
                    status: ReplyStatus::Ok,
                    error: None,
                }
                .as_child_of(parent);
                self.shell.send(reply).await?;
            }

            JupyterMessageContent::InspectRequest(_) => {
                let reply = InspectReply {
                    found: false,
                    data: Media::default(),
                    metadata: Default::default(),
                    status: ReplyStatus::Ok,
                    error: None,
                }
                .as_child_of(parent);
                self.shell.send(reply).await?;
            }

            JupyterMessageContent::IsCompleteRequest(_) => {
                let reply = IsCompleteReply {
                    status: IsCompleteReplyStatus::Complete,
                    indent: String::new(),
                }
                .as_child_of(parent);
                self.shell.send(reply).await?;
            }

            JupyterMessageContent::HistoryRequest(_) => {
                let reply = HistoryReply {
                    history: vec![],
                    status: ReplyStatus::Ok,
                    error: None,
                }
                .as_child_of(parent);
                self.shell.send(reply).await?;
            }

            JupyterMessageContent::CommInfoRequest(_) => {
                let reply = CommInfoReply {
                    status: ReplyStatus::Ok,
                    comms: Default::default(),
                    error: None,
                }
                .as_child_of(parent);
                self.shell.send(reply).await?;
            }

            _ => {}
        }

        // Send idle status
        self.iopub.send(Status::idle().as_child_of(parent)).await?;

        Ok(())
    }

    async fn handle_execute_request(
        &mut self,
        code: &str,
        _store_history: bool,
        parent: &JupyterMessage,
    ) -> Result<()> {
        // Increment execution count
        self.execution_count.0 += 1;
        let exec_count = self.execution_count;

        // Send execute_input on iopub
        self.iopub
            .send(
                ExecuteInput {
                    code: code.to_string(),
                    execution_count: exec_count,
                }
                .as_child_of(parent),
            )
            .await?;

        // Check for canned response
        if let Some(response) = self.config.responses.get(code) {
            // Send canned outputs
            for output in &response.outputs {
                let msg = JupyterMessage::new(output.clone(), Some(parent));
                self.iopub.send(msg).await?;
            }
        } else {
            // Default: echo code as stdout
            self.iopub
                .send(StreamContent::stdout(code).as_child_of(parent))
                .await?;
        }

        // Send execute_reply
        let reply = ExecuteReply {
            status: ReplyStatus::Ok,
            execution_count: exec_count,
            payload: vec![],
            user_expressions: None,
            error: None,
        }
        .as_child_of(parent);
        self.shell.send(reply).await?;

        Ok(())
    }

    fn kernel_info() -> KernelInfoReply {
        KernelInfoReply {
            status: ReplyStatus::Ok,
            protocol_version: "5.3".to_string(),
            implementation: "TestKernel".to_string(),
            implementation_version: env!("CARGO_PKG_VERSION").to_string(),
            language_info: LanguageInfo {
                name: "test".to_string(),
                version: "1.0".to_string(),
                mimetype: Some("text/plain".to_string()),
                file_extension: Some(".txt".to_string()),
                pygments_lexer: None,
                codemirror_mode: Some(CodeMirrorMode::Simple("text".to_string())),
                nbconvert_exporter: None,
            },
            banner: "TestKernel - A mock kernel for testing Jupyter frontends".to_string(),
            help_links: vec![],
            debugger: false,
            error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        create_client_iopub_connection, create_client_shell_connection_with_identity,
        peer_identity_for_session,
    };
    use jupyter_protocol::{
        DisplayData, ExecuteRequest, ExecutionState, KernelInfoRequest, MediaType, ShutdownRequest,
    };

    #[tokio::test]
    async fn test_start_ephemeral_returns_valid_connection_info() {
        let (handle, connection_info) = TestKernel::start_ephemeral(TestKernelConfig::default())
            .await
            .unwrap();

        assert_eq!(connection_info.ip, "127.0.0.1");
        assert_eq!(connection_info.transport, Transport::TCP);
        assert!(connection_info.shell_port > 0);
        assert!(connection_info.iopub_port > 0);
        assert!(connection_info.stdin_port > 0);
        assert!(connection_info.control_port > 0);
        assert!(connection_info.hb_port > 0);
        assert!(!connection_info.key.is_empty());

        handle.abort();
    }

    #[tokio::test]
    async fn test_kernel_info_request() {
        let (handle, connection_info) = TestKernel::start_ephemeral(TestKernelConfig::default())
            .await
            .unwrap();

        // Give kernel time to bind
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let session_id = Uuid::new_v4().to_string();
        let identity = peer_identity_for_session(&session_id).unwrap();
        let mut shell =
            create_client_shell_connection_with_identity(&connection_info, &session_id, identity)
                .await
                .unwrap();

        // Send kernel_info_request
        let request: JupyterMessage = KernelInfoRequest {}.into();
        shell.send(request).await.unwrap();

        // Read reply
        let reply = shell.read().await.unwrap();
        match reply.content {
            JupyterMessageContent::KernelInfoReply(info) => {
                assert_eq!(info.status, ReplyStatus::Ok);
                assert_eq!(info.implementation, "TestKernel");
                assert_eq!(info.language_info.name, "test");
            }
            other => panic!("Expected KernelInfoReply, got {:?}", other),
        }

        handle.abort();
    }

    #[tokio::test]
    async fn test_execute_echo_mode() {
        let (handle, connection_info) = TestKernel::start_ephemeral(TestKernelConfig::default())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let session_id = Uuid::new_v4().to_string();
        let identity = peer_identity_for_session(&session_id).unwrap();

        let mut shell =
            create_client_shell_connection_with_identity(&connection_info, &session_id, identity)
                .await
                .unwrap();
        let mut iopub = create_client_iopub_connection(&connection_info, "", &session_id)
            .await
            .unwrap();

        // Give ZMQ pub/sub time to establish subscription
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Send execute request
        let request: JupyterMessage = ExecuteRequest::new("hello world".to_string()).into();
        let request_id = request.header.msg_id.clone();
        shell.send(request).await.unwrap();

        // Collect iopub messages until idle
        let mut received_stream = false;
        let mut stream_text = String::new();

        loop {
            let msg = tokio::time::timeout(tokio::time::Duration::from_secs(5), iopub.read())
                .await
                .expect("timeout waiting for iopub message")
                .unwrap();

            // Only process messages related to our request
            let is_ours = msg
                .parent_header
                .as_ref()
                .map(|h| h.msg_id == request_id)
                .unwrap_or(false);

            if !is_ours {
                continue;
            }

            match &msg.content {
                JupyterMessageContent::StreamContent(stream) => {
                    received_stream = true;
                    stream_text.push_str(&stream.text);
                }
                JupyterMessageContent::Status(status) => {
                    if status.execution_state == ExecutionState::Idle {
                        break;
                    }
                }
                _ => {}
            }
        }

        assert!(received_stream, "Should have received stream output");
        assert_eq!(stream_text, "hello world", "Stream should echo the code");

        // Verify shell reply
        let reply = shell.read().await.unwrap();
        match reply.content {
            JupyterMessageContent::ExecuteReply(rep) => {
                assert_eq!(rep.status, ReplyStatus::Ok);
                assert_eq!(rep.execution_count.0, 1);
            }
            other => panic!("Expected ExecuteReply, got {:?}", other),
        }

        handle.abort();
    }

    #[tokio::test]
    async fn test_execute_canned_response() {
        let config = TestKernelConfig::default().with_response(
            "show_image()",
            CannedResponse {
                outputs: vec![
                    DisplayData::from(MediaType::Plain("test image output".to_string())).into(),
                ],
            },
        );

        let (handle, connection_info) = TestKernel::start_ephemeral(config).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let session_id = Uuid::new_v4().to_string();
        let identity = peer_identity_for_session(&session_id).unwrap();

        let mut shell =
            create_client_shell_connection_with_identity(&connection_info, &session_id, identity)
                .await
                .unwrap();
        let mut iopub = create_client_iopub_connection(&connection_info, "", &session_id)
            .await
            .unwrap();

        // Give ZMQ pub/sub time to establish subscription
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Send execute request with code matching the canned response
        let request: JupyterMessage = ExecuteRequest::new("show_image()".to_string()).into();
        let request_id = request.header.msg_id.clone();
        shell.send(request).await.unwrap();

        // Collect iopub messages until idle
        let mut received_display_data = false;

        loop {
            let msg = tokio::time::timeout(tokio::time::Duration::from_secs(5), iopub.read())
                .await
                .expect("timeout waiting for iopub message")
                .unwrap();

            let is_ours = msg
                .parent_header
                .as_ref()
                .map(|h| h.msg_id == request_id)
                .unwrap_or(false);

            if !is_ours {
                continue;
            }

            match &msg.content {
                JupyterMessageContent::DisplayData(_data) => {
                    received_display_data = true;
                }
                JupyterMessageContent::Status(status) => {
                    if status.execution_state == ExecutionState::Idle {
                        break;
                    }
                }
                _ => {}
            }
        }

        assert!(
            received_display_data,
            "Should have received canned DisplayData response"
        );

        handle.abort();
    }

    #[tokio::test]
    async fn test_shutdown_request() {
        let (handle, connection_info) = TestKernel::start_ephemeral(TestKernelConfig::default())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let session_id = Uuid::new_v4().to_string();
        let mut control = crate::create_client_control_connection(&connection_info, &session_id)
            .await
            .unwrap();

        // Send shutdown request
        let request: JupyterMessage = ShutdownRequest { restart: false }.into();
        control.send(request).await.unwrap();

        // Read reply
        let reply = control.read().await.unwrap();
        match reply.content {
            JupyterMessageContent::ShutdownReply(rep) => {
                assert_eq!(rep.status, ReplyStatus::Ok);
                assert!(!rep.restart);
            }
            other => panic!("Expected ShutdownReply, got {:?}", other),
        }

        // The kernel should exit cleanly
        let result = tokio::time::timeout(tokio::time::Duration::from_secs(2), handle).await;
        assert!(
            result.is_ok(),
            "Kernel should shut down after shutdown request"
        );
    }
}
