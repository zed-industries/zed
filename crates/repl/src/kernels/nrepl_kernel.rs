use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use futures::channel::mpsc;
use futures::{AsyncReadExt as _, AsyncWriteExt as _, SinkExt as _, StreamExt as _};
use gpui::{App, Entity, Task, Window};
use jupyter_protocol::{ExecutionState, JupyterMessage, JupyterMessageContent, KernelInfoReply};
use smol::net::TcpStream;

use crate::kernels::bencode;
use crate::kernels::{KernelSession, RunningKernel};

pub(crate) fn jupyter_to_nrepl(
    message: &JupyterMessage,
    session: &str,
    namespace: Option<&str>,
) -> Option<bencode::Value> {
    match &message.content {
        JupyterMessageContent::ExecuteRequest(execute_request) => {
            let mut entries = vec![
                (b"op".to_vec(), bencode::Value::Bytes(b"eval".to_vec())),
                (
                    b"code".to_vec(),
                    bencode::Value::Bytes(execute_request.code.as_bytes().to_vec()),
                ),
                (
                    b"session".to_vec(),
                    bencode::Value::Bytes(session.as_bytes().to_vec()),
                ),
                (
                    b"id".to_vec(),
                    bencode::Value::Bytes(message.header.msg_id.as_bytes().to_vec()),
                ),
            ];

            if let Some(namespace) = namespace {
                entries.push((
                    b"ns".to_vec(),
                    bencode::Value::Bytes(namespace.as_bytes().to_vec()),
                ));
            }

            Some(bencode::Value::Dict(entries))
        }
        JupyterMessageContent::InterruptRequest(_) => Some(bencode::Value::Dict(vec![
            (b"op".to_vec(), bencode::Value::Bytes(b"interrupt".to_vec())),
            (
                b"session".to_vec(),
                bencode::Value::Bytes(session.as_bytes().to_vec()),
            ),
            (
                b"id".to_vec(),
                bencode::Value::Bytes(message.header.msg_id.as_bytes().to_vec()),
            ),
        ])),
        JupyterMessageContent::InputReply(input_reply) => {
            let mut stdin = input_reply.value.clone();
            stdin.push('\n');

            Some(bencode::Value::Dict(vec![
                (b"op".to_vec(), bencode::Value::Bytes(b"stdin".to_vec())),
                (b"stdin".to_vec(), bencode::Value::Bytes(stdin.into_bytes())),
                (
                    b"session".to_vec(),
                    bencode::Value::Bytes(session.as_bytes().to_vec()),
                ),
                (
                    b"id".to_vec(),
                    bencode::Value::Bytes(message.header.msg_id.as_bytes().to_vec()),
                ),
            ]))
        }
        _ => None,
    }
}

pub(crate) fn nrepl_to_jupyter(
    response: &bencode::Value,
    parent: &JupyterMessage,
    execution_count: &mut usize,
    has_error: &mut bool,
) -> Vec<JupyterMessage> {
    let mut messages = Vec::new();

    if let Some(stdout_text) = response.get("out").and_then(|value| value.as_str()) {
        let stream = runtimelib::StreamContent::stdout(stdout_text);
        messages.push(stream.as_child_of(parent));
    }

    if let Some(stderr_text) = response.get("err").and_then(|value| value.as_str()) {
        let stream = runtimelib::StreamContent::stderr(stderr_text);
        messages.push(stream.as_child_of(parent));
    }

    if let Some(result_text) = response.get("value").and_then(|value| value.as_str()) {
        *execution_count += 1;
        let execute_result = runtimelib::ExecuteResult::from((
            runtimelib::ExecutionCount::new(*execution_count),
            runtimelib::MediaType::Plain(result_text.to_string()),
        ));
        messages.push(execute_result.as_child_of(parent));
    }

    if let Some(exception_text) = response.get("ex").and_then(|value| value.as_str()) {
        *has_error = true;
        let evalue_text = response
            .get("root-ex")
            .and_then(|value| value.as_str())
            .unwrap_or(exception_text);
        let error_output = runtimelib::ErrorOutput {
            ename: exception_text.to_string(),
            evalue: evalue_text.to_string(),
            traceback: Vec::new(),
        };
        messages.push(error_output.as_child_of(parent));
    }

    let mut is_done = false;
    let mut needs_input = false;

    if let Some(status_value) = response.get("status") {
        match status_value {
            bencode::Value::Bytes(status_bytes) => {
                if let Ok(status_str) = std::str::from_utf8(status_bytes) {
                    if status_str == "need-input" {
                        needs_input = true;
                    }
                    if status_str == "done" {
                        is_done = true;
                    }
                }
            }
            bencode::Value::List(status_list) => {
                for status_item in status_list {
                    if let Some(status_str) = status_item.as_str() {
                        if status_str == "need-input" {
                            needs_input = true;
                        }
                        if status_str == "done" {
                            is_done = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if needs_input {
        let input_request = runtimelib::InputRequest {
            prompt: String::new(),
            password: false,
        };
        messages.push(input_request.as_child_of(parent));
    }

    if is_done {
        let status = if *has_error {
            runtimelib::ReplyStatus::Error
        } else {
            runtimelib::ReplyStatus::Ok
        };
        let execute_reply = runtimelib::ExecuteReply {
            status,
            execution_count: runtimelib::ExecutionCount::new(*execution_count),
            payload: Vec::new(),
            user_expressions: None,
            error: None,
        };
        messages.push(execute_reply.as_child_of(parent));

        let idle_status = runtimelib::Status {
            execution_state: runtimelib::ExecutionState::Idle,
        };
        messages.push(idle_status.as_child_of(parent));
    }

    messages
}

pub struct NreplRunningKernel {
    _routing_task: Task<Result<()>>,
    _receiving_task: Task<Result<()>>,
    pub working_directory: PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub stdin_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
    pub session_id: String,
    stream: Arc<TcpStream>,
}

impl NreplRunningKernel {
    pub fn new<S: KernelSession + 'static>(
        host: String,
        port: u16,
        working_directory: PathBuf,
        session: Entity<S>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        window.spawn(cx, async move |cx| {
            let stream = TcpStream::connect((host.as_str(), port))
                .await
                .with_context(|| format!("connecting to nREPL at {host}:{port}"))?;
            stream
                .set_nodelay(true)
                .context("disabling Nagle on nREPL socket")?;

            let stream = Arc::new(stream);
            let mut reader = stream.as_ref().clone();
            let mut writer = stream.as_ref().clone();

            let clone_request_id = uuid::Uuid::new_v4().to_string();
            let clone_message = bencode::Value::Dict(vec![
                (b"op".to_vec(), bencode::Value::Bytes(b"clone".to_vec())),
                (
                    b"id".to_vec(),
                    bencode::Value::Bytes(clone_request_id.as_bytes().to_vec()),
                ),
            ]);
            writer
                .write_all(&bencode::encode(&clone_message))
                .await
                .context("sending clone op to nREPL")?;

            let mut read_buffer = Vec::new();
            let mut chunk_buffer = [0u8; 4096];
            let mut session_id = None;

            while session_id.is_none() {
                let bytes_read = reader
                    .read(&mut chunk_buffer)
                    .await
                    .context("reading clone response from nREPL")?;
                if bytes_read == 0 {
                    anyhow::bail!("nREPL connection closed while waiting for new-session");
                }
                read_buffer.extend_from_slice(&chunk_buffer[..bytes_read]);

                while let Some((value, consumed)) = bencode::decode_available(&read_buffer)? {
                    read_buffer.drain(..consumed);
                    if let Some(new_session) = value.get("new-session").and_then(|val| val.as_str())
                    {
                        session_id = Some(new_session.to_string());
                        break;
                    }
                }
            }

            let session_id = session_id.context("failed to obtain session_id from nREPL clone")?;

            let describe_request_id = uuid::Uuid::new_v4().to_string();
            let describe_message = bencode::Value::Dict(vec![
                (b"op".to_vec(), bencode::Value::Bytes(b"describe".to_vec())),
                (
                    b"id".to_vec(),
                    bencode::Value::Bytes(describe_request_id.as_bytes().to_vec()),
                ),
            ]);
            writer
                .write_all(&bencode::encode(&describe_message))
                .await
                .context("sending describe op to nREPL")?;

            let mut banner = String::new();
            let mut describe_done = false;

            while !describe_done {
                while let Some((value, consumed)) = bencode::decode_available(&read_buffer)? {
                    read_buffer.drain(..consumed);
                    if let Some(versions) = value.get("versions") {
                        banner = format!("{versions:?}");
                    }
                    if let Some(status_value) = value.get("status") {
                        let mut is_done = false;
                        match status_value {
                            bencode::Value::Bytes(status_bytes) => {
                                if status_bytes == b"done" {
                                    is_done = true;
                                }
                            }
                            bencode::Value::List(status_list) => {
                                for item in status_list {
                                    if item.as_str() == Some("done") {
                                        is_done = true;
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                        if is_done {
                            describe_done = true;
                            break;
                        }
                    }
                }

                if !describe_done {
                    let bytes_read = reader
                        .read(&mut chunk_buffer)
                        .await
                        .context("reading describe response from nREPL")?;
                    if bytes_read == 0 {
                        anyhow::bail!(
                            "nREPL connection closed while waiting for describe response"
                        );
                    }
                    read_buffer.extend_from_slice(&chunk_buffer[..bytes_read]);
                }
            }

            let kernel_info = KernelInfoReply {
                status: runtimelib::ReplyStatus::Ok,
                protocol_version: "5.3".to_string(),
                implementation: "nrepl".to_string(),
                implementation_version: "0.1.0".to_string(),
                language_info: runtimelib::LanguageInfo {
                    name: "clojure".to_string(),
                    version: "".to_string(),
                    mimetype: Some("text/x-clojure".to_string()),
                    file_extension: Some(".clj".to_string()),
                    pygments_lexer: Some("clojure".to_string()),
                    codemirror_mode: None,
                    nbconvert_exporter: None,
                },
                banner,
                help_links: Vec::new(),
                debugger: false,
                error: None,
            };

            let (request_tx, mut request_rx) = mpsc::channel::<JupyterMessage>(100);
            let (stdin_tx, mut stdin_rx) = mpsc::channel::<JupyterMessage>(100);
            let (parent_message_tx, mut parent_message_rx) = mpsc::channel::<JupyterMessage>(100);

            let session_id_for_write = session_id.clone();
            let session_for_write = session.clone();
            let routing_task = cx.spawn({
                let mut writer = stream.as_ref().clone();
                async move |cx| {
                    loop {
                        let maybe_message = futures::select! {
                            request_msg = request_rx.next() => request_msg,
                            stdin_msg = stdin_rx.next() => stdin_msg,
                        };

                        let Some(message) = maybe_message else {
                            break;
                        };

                        if matches!(message.content, JupyterMessageContent::ExecuteRequest(_)) {
                            let busy_status = runtimelib::Status {
                                execution_state: runtimelib::ExecutionState::Busy,
                            };
                            let busy_message = busy_status.as_child_of(&message);
                            session_for_write
                                .update_in(cx, |session, window, cx| {
                                    session.route(&busy_message, window, cx);
                                })
                                .ok();
                        }

                        if let Some(nrepl_value) =
                            jupyter_to_nrepl(&message, &session_id_for_write, None)
                        {
                            // Only evaluations produce output; interrupt and stdin replies
                            // are answered under the id of the evaluation they belong to.
                            if matches!(message.content, JupyterMessageContent::ExecuteRequest(_)) {
                                parent_message_tx.clone().send(message).await?;
                            }
                            let encoded = bencode::encode(&nrepl_value);
                            writer.write_all(&encoded).await?;
                        }
                    }
                    anyhow::Ok(())
                }
            });

            let receiving_task = cx.spawn({
                let session = session.clone();
                let mut reader = stream.as_ref().clone();

                async move |cx| {
                    let mut current_parent: Option<JupyterMessage> = None;
                    let mut execution_count = 0usize;
                    let mut has_error = false;
                    let mut read_buffer = read_buffer;
                    let mut chunk_buffer = [0u8; 4096];

                    loop {
                        while let Some((value, consumed)) = bencode::decode_available(&read_buffer)?
                        {
                            read_buffer.drain(..consumed);

                            if current_parent.is_none() {
                                current_parent = parent_message_rx.next().await;
                            }

                            if let Some(parent) = &current_parent {
                                let response_id = value.get("id").and_then(|id| id.as_str());
                                if response_id != Some(parent.header.msg_id.as_str()) {
                                    continue;
                                }
                                let jupyter_messages = nrepl_to_jupyter(
                                    &value,
                                    parent,
                                    &mut execution_count,
                                    &mut has_error,
                                );

                                let mut finished = false;
                                for jupyter_message in jupyter_messages {
                                    if matches!(
                                        jupyter_message.content,
                                        JupyterMessageContent::ExecuteReply(_)
                                    ) {
                                        finished = true;
                                    }
                                    session
                                        .update_in(cx, |session, window, cx| {
                                            session.route(&jupyter_message, window, cx);
                                        })
                                        .ok();
                                }

                                if finished {
                                    current_parent = None;
                                    has_error = false;
                                }
                            }
                        }

                        let bytes_read = reader.read(&mut chunk_buffer).await?;
                        if bytes_read == 0 {
                            break;
                        }
                        read_buffer.extend_from_slice(&chunk_buffer[..bytes_read]);
                    }
                    anyhow::Ok(())
                }
            });

            Ok(Box::new(Self {
                _routing_task: routing_task,
                _receiving_task: receiving_task,
                working_directory,
                request_tx,
                stdin_tx,
                execution_state: ExecutionState::Idle,
                kernel_info: Some(kernel_info),
                session_id,
                stream,
            }) as Box<dyn RunningKernel>)
        })
    }
}

impl Debug for NreplRunningKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NreplRunningKernel")
            .field("working_directory", &self.working_directory)
            .field("request_tx", &self.request_tx)
            .field("execution_state", &self.execution_state)
            .field("kernel_info", &self.kernel_info)
            .field("session_id", &self.session_id)
            .finish()
    }
}

impl RunningKernel for NreplRunningKernel {
    fn request_tx(&self) -> mpsc::Sender<JupyterMessage> {
        self.request_tx.clone()
    }

    fn stdin_tx(&self) -> mpsc::Sender<JupyterMessage> {
        self.stdin_tx.clone()
    }

    fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    fn execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }

    fn set_execution_state(&mut self, state: ExecutionState) {
        self.execution_state = state;
    }

    fn kernel_info(&self) -> Option<&KernelInfoReply> {
        self.kernel_info.as_ref()
    }

    fn set_kernel_info(&mut self, info: KernelInfoReply) {
        self.kernel_info = Some(info);
    }

    fn force_shutdown(&mut self, window: &mut Window, cx: &mut App) -> Task<Result<()>> {
        let session_id = self.session_id.clone();
        let stream = self.stream.clone();
        self.request_tx.close_channel();
        self.stdin_tx.close_channel();

        window.spawn(cx, async move |_| {
            let close_request = bencode::Value::Dict(vec![
                (b"op".to_vec(), bencode::Value::Bytes(b"close".to_vec())),
                (
                    b"session".to_vec(),
                    bencode::Value::Bytes(session_id.into_bytes()),
                ),
                (
                    b"id".to_vec(),
                    bencode::Value::Bytes(uuid::Uuid::new_v4().to_string().into_bytes()),
                ),
            ]);
            let encoded = bencode::encode(&close_request);
            let mut writer = stream.as_ref().clone();
            writer.write_all(&encoded).await.ok();
            stream.shutdown(std::net::Shutdown::Both).ok();
            Ok(())
        })
    }

    fn kill(&mut self) {
        self.request_tx.close_channel();
        self.stdin_tx.close_channel();
        let stream = self.stream.clone();
        smol::spawn(async move {
            stream.shutdown(std::net::Shutdown::Both).ok();
        })
        .detach();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runtimelib::{ExecuteRequest, InputReply, InterruptRequest, ReplyStatus};

    #[test]
    fn test_execute_request_without_namespace() {
        let execute_request = ExecuteRequest {
            code: "(+ 1 2)".to_string(),
            ..ExecuteRequest::default()
        };
        let message: JupyterMessage = execute_request.into();
        let session = "test-session-1";

        let bencode_value = jupyter_to_nrepl(&message, session, None);
        assert!(bencode_value.is_some());
        let dict = bencode_value.unwrap();

        assert_eq!(dict.get("op").and_then(|v| v.as_str()), Some("eval"));
        assert_eq!(dict.get("code").and_then(|v| v.as_str()), Some("(+ 1 2)"));
        assert_eq!(
            dict.get("session").and_then(|v| v.as_str()),
            Some("test-session-1")
        );
        assert_eq!(
            dict.get("id").and_then(|v| v.as_str()),
            Some(message.header.msg_id.as_str())
        );
        assert_eq!(dict.get("ns"), None);
    }

    #[test]
    fn test_execute_request_with_namespace() {
        let execute_request = ExecuteRequest {
            code: "(def x 10)".to_string(),
            ..ExecuteRequest::default()
        };
        let message: JupyterMessage = execute_request.into();
        let session = "test-session-2";
        let namespace = "my-namespace";

        let bencode_value = jupyter_to_nrepl(&message, session, Some(namespace));
        assert!(bencode_value.is_some());
        let dict = bencode_value.unwrap();

        assert_eq!(dict.get("op").and_then(|v| v.as_str()), Some("eval"));
        assert_eq!(
            dict.get("code").and_then(|v| v.as_str()),
            Some("(def x 10)")
        );
        assert_eq!(
            dict.get("session").and_then(|v| v.as_str()),
            Some("test-session-2")
        );
        assert_eq!(
            dict.get("id").and_then(|v| v.as_str()),
            Some(message.header.msg_id.as_str())
        );
        assert_eq!(
            dict.get("ns").and_then(|v| v.as_str()),
            Some("my-namespace")
        );
    }

    #[test]
    fn test_interrupt_request() {
        let interrupt_request = InterruptRequest {};
        let message: JupyterMessage = interrupt_request.into();
        let session = "test-session-3";

        let bencode_value = jupyter_to_nrepl(&message, session, None);
        assert!(bencode_value.is_some());
        let dict = bencode_value.unwrap();

        assert_eq!(dict.get("op").and_then(|v| v.as_str()), Some("interrupt"));
        assert_eq!(
            dict.get("session").and_then(|v| v.as_str()),
            Some("test-session-3")
        );
        assert_eq!(
            dict.get("id").and_then(|v| v.as_str()),
            Some(message.header.msg_id.as_str())
        );
    }

    #[test]
    fn test_input_reply() {
        let input_reply = InputReply {
            value: "user response".to_string(),
            status: ReplyStatus::Ok,
            error: None,
        };
        let message: JupyterMessage = input_reply.into();
        let session = "test-session-4";

        let bencode_value = jupyter_to_nrepl(&message, session, None);
        assert!(bencode_value.is_some());
        let dict = bencode_value.unwrap();

        assert_eq!(dict.get("op").and_then(|v| v.as_str()), Some("stdin"));
        assert_eq!(
            dict.get("stdin").and_then(|v| v.as_str()),
            Some("user response\n")
        );
        assert_eq!(
            dict.get("session").and_then(|v| v.as_str()),
            Some("test-session-4")
        );
        assert_eq!(
            dict.get("id").and_then(|v| v.as_str()),
            Some(message.header.msg_id.as_str())
        );
    }

    #[test]
    fn test_other_message_returns_none() {
        let message: JupyterMessage = runtimelib::ShutdownRequest { restart: false }.into();
        let session = "test-session-5";

        assert_eq!(jupyter_to_nrepl(&message, session, None), None);
    }

    #[test]
    fn test_nrepl_to_jupyter_stdout() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"out".to_vec(),
            bencode::Value::Bytes(b"hello world\n".to_vec()),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::StreamContent(stream) => {
                assert!(matches!(stream.name, runtimelib::Stdio::Stdout));
                assert_eq!(stream.text, "hello world\n");
            }
            other => panic!("expected StreamContent stdout, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_stderr() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"err".to_vec(),
            bencode::Value::Bytes(b"warning alert\n".to_vec()),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::StreamContent(stream) => {
                assert!(matches!(stream.name, runtimelib::Stdio::Stderr));
                assert_eq!(stream.text, "warning alert\n");
            }
            other => panic!("expected StreamContent stderr, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_value() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"value".to_vec(),
            bencode::Value::Bytes(b"42".to_vec()),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert_eq!(count, 1);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::ExecuteResult(result) => {
                assert_eq!(result.execution_count, runtimelib::ExecutionCount::new(1));
                assert_eq!(
                    result.data.content,
                    vec![runtimelib::MediaType::Plain("42".to_string())]
                );
            }
            other => panic!("expected ExecuteResult, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_ex_without_root_ex() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"ex".to_vec(),
            bencode::Value::Bytes(b"class java.lang.Exception".to_vec()),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert!(has_error);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::ErrorOutput(error) => {
                assert_eq!(error.ename, "class java.lang.Exception");
                assert_eq!(error.evalue, "class java.lang.Exception");
                assert!(error.traceback.is_empty());
            }
            other => panic!("expected ErrorOutput, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_ex_with_root_ex() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![
            (
                b"ex".to_vec(),
                bencode::Value::Bytes(b"class clojure.lang.Compiler$CompilerException".to_vec()),
            ),
            (
                b"root-ex".to_vec(),
                bencode::Value::Bytes(b"class java.lang.ArithmeticException: / by zero".to_vec()),
            ),
        ]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert!(has_error);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::ErrorOutput(error) => {
                assert_eq!(error.ename, "class clojure.lang.Compiler$CompilerException");
                assert_eq!(
                    error.evalue,
                    "class java.lang.ArithmeticException: / by zero"
                );
                assert!(error.traceback.is_empty());
            }
            other => panic!("expected ErrorOutput, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_need_input() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 0;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"status".to_vec(),
            bencode::Value::List(vec![bencode::Value::Bytes(b"need-input".to_vec())]),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        match &messages[0].content {
            JupyterMessageContent::InputRequest(input_req) => {
                assert_eq!(input_req.prompt, "");
                assert!(!input_req.password);
            }
            other => panic!("expected InputRequest, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_done_ok() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 2;
        let mut has_error = false;
        let response = bencode::Value::Dict(vec![(
            b"status".to_vec(),
            bencode::Value::List(vec![bencode::Value::Bytes(b"done".to_vec())]),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 2);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        assert_eq!(
            messages[1].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );

        match &messages[0].content {
            JupyterMessageContent::ExecuteReply(reply) => {
                assert_eq!(reply.status, ReplyStatus::Ok);
                assert_eq!(reply.execution_count, runtimelib::ExecutionCount::new(2));
            }
            other => panic!("expected ExecuteReply, got {other:?}"),
        }

        match &messages[1].content {
            JupyterMessageContent::Status(status) => {
                assert_eq!(status.execution_state, runtimelib::ExecutionState::Idle);
            }
            other => panic!("expected Status idle, got {other:?}"),
        }
    }

    #[test]
    fn test_nrepl_to_jupyter_done_error() {
        let parent: JupyterMessage = ExecuteRequest::default().into();
        let mut count = 1;
        let mut has_error = true;
        let response = bencode::Value::Dict(vec![(
            b"status".to_vec(),
            bencode::Value::List(vec![bencode::Value::Bytes(b"done".to_vec())]),
        )]);

        let messages = nrepl_to_jupyter(&response, &parent, &mut count, &mut has_error);
        assert_eq!(messages.len(), 2);
        assert_eq!(
            messages[0].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );
        assert_eq!(
            messages[1].parent_header.as_ref().map(|h| &h.msg_id),
            Some(&parent.header.msg_id)
        );

        match &messages[0].content {
            JupyterMessageContent::ExecuteReply(reply) => {
                assert_eq!(reply.status, ReplyStatus::Error);
                assert_eq!(reply.execution_count, runtimelib::ExecutionCount::new(1));
            }
            other => panic!("expected ExecuteReply, got {other:?}"),
        }

        match &messages[1].content {
            JupyterMessageContent::Status(status) => {
                assert_eq!(status.execution_state, runtimelib::ExecutionState::Idle);
            }
            other => panic!("expected Status idle, got {other:?}"),
        }
    }

    fn dict(entries: &[(&str, bencode::Value)]) -> bencode::Value {
        bencode::Value::Dict(
            entries
                .iter()
                .map(|(key, value)| (key.as_bytes().to_vec(), value.clone()))
                .collect(),
        )
    }

    fn text(value: &str) -> bencode::Value {
        bencode::Value::Bytes(value.as_bytes().to_vec())
    }

    async fn read_value(stream: &mut smol::net::TcpStream, buffer: &mut Vec<u8>) -> bencode::Value {
        let mut chunk = [0u8; 4096];
        loop {
            if let Some((value, consumed)) = bencode::decode_available(buffer).unwrap() {
                buffer.drain(..consumed);
                return value;
            }
            let bytes_read = stream.read(&mut chunk).await.unwrap();
            assert!(bytes_read > 0, "connection closed");
            buffer.extend_from_slice(&chunk[..bytes_read]);
        }
    }

    #[test]
    fn test_eval_round_trip_against_fake_server() {
        smol::block_on(async {
            let listener = smol::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();

            let server = smol::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = Vec::new();
                let request = read_value(&mut stream, &mut buffer).await;
                assert_eq!(request.get("op").and_then(|v| v.as_str()), Some("eval"));
                let id = request
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string();
                let responses = [
                    dict(&[("id", text(&id)), ("out", text("hola\n"))]),
                    dict(&[
                        ("id", text(&id)),
                        ("value", text("3")),
                        ("ns", text("user")),
                    ]),
                    dict(&[
                        ("id", text(&id)),
                        ("status", bencode::Value::List(vec![text("done")])),
                    ]),
                ];
                for response in responses {
                    stream.write_all(&bencode::encode(&response)).await.unwrap();
                }
            });

            let mut client = smol::net::TcpStream::connect(("127.0.0.1", port))
                .await
                .unwrap();
            let message: JupyterMessage = ExecuteRequest {
                code: "(+ 1 2)".to_string(),
                ..ExecuteRequest::default()
            }
            .into();
            let request = jupyter_to_nrepl(&message, "s1", Some("user")).unwrap();
            client.write_all(&bencode::encode(&request)).await.unwrap();

            let mut buffer = Vec::new();
            let mut execution_count = 0;
            let mut has_error = false;
            let mut jupyter_messages = Vec::new();
            for _ in 0..3 {
                let response = read_value(&mut client, &mut buffer).await;
                jupyter_messages.extend(nrepl_to_jupyter(
                    &response,
                    &message,
                    &mut execution_count,
                    &mut has_error,
                ));
            }
            server.await;

            let kinds = jupyter_messages
                .iter()
                .map(|message| message.content.message_type().to_string())
                .collect::<Vec<_>>();
            assert_eq!(
                kinds,
                ["stream", "execute_result", "execute_reply", "status"]
            );
            match &jupyter_messages[1].content {
                JupyterMessageContent::ExecuteResult(result) => {
                    assert!(format!("{:?}", result.data).contains("\"3\""));
                }
                other => panic!("expected ExecuteResult, got {other:?}"),
            }
        });
    }
}
