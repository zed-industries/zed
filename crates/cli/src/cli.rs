use anyhow::Result;
use collections::HashMap;
pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    Open {
        paths: Vec<String>,
        urls: Vec<String>,
        diff_paths: Vec<[String; 2]>,
        diff_all: bool,
        wsl: Option<String>,
        wait: bool,
        open_new_workspace: Option<bool>,
        #[serde(default)]
        force_existing_window: bool,
        reuse: bool,
        env: Option<HashMap<String, String>>,
        user_data_dir: Option<String>,
        dev_container: bool,
    },
    SetOpenBehavior {
        /// true = existing window, false = new window
        existing_window: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Ping,
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
    PromptOpenBehavior,
}

/// When Zed started not as an *.app but as a binary (e.g. local development),
/// there's a possibility to tell it to behave "regularly".
///
/// Note that in the main zed binary, this variable is unset after it's read for the first time,
/// therefore it should always be accessed through the `FORCE_CLI_MODE` static.
pub const FORCE_CLI_MODE_ENV_VAR_NAME: &str = "ZED_FORCE_CLI_MODE";

/// Abstracts the transport for sending CLI responses (Zed → CLI).
///
/// Production code uses `IpcSender<CliResponse>`. Tests can provide in-memory
/// implementations to avoid OS-level IPC.
pub trait CliResponseSink: Send + 'static {
    fn send(&self, response: CliResponse) -> Result<()>;
}

impl CliResponseSink for ipc::IpcSender<CliResponse> {
    fn send(&self, response: CliResponse) -> Result<()> {
        ipc::IpcSender::send(self, response).map_err(|error| anyhow::anyhow!("{error}"))
    }
}

/// Runs the CLI-side response loop: sends the initial request, then processes
/// responses until an `Exit` is received.
///
/// `prompt_open_behavior` is called when Zed asks the user to choose between
/// adding to an existing window or opening a new one. It should return
/// `Some(true)` for "existing window", `Some(false)` for "new window", or
/// `None` to default to existing window.
pub fn run_cli_response_loop(
    send_request: impl Fn(CliRequest) -> Result<()>,
    recv_response: impl Fn() -> Result<CliResponse>,
    initial_request: CliRequest,
    mut prompt_open_behavior: impl FnMut() -> Option<bool>,
    mut on_stdout: impl FnMut(&str),
    mut on_stderr: impl FnMut(&str),
) -> Result<i32> {
    send_request(initial_request)?;

    loop {
        let response = recv_response()?;
        match response {
            CliResponse::Ping => {}
            CliResponse::Stdout { message } => on_stdout(&message),
            CliResponse::Stderr { message } => on_stderr(&message),
            CliResponse::Exit { status } => return Ok(status),
            CliResponse::PromptOpenBehavior => {
                let existing_window = prompt_open_behavior().unwrap_or(true);
                send_request(CliRequest::SetOpenBehavior { existing_window })?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_loop_exit() {
        let responses = vec![CliResponse::Exit { status: 0 }];
        let responses = std::sync::Mutex::new(responses.into_iter());
        let requests = std::sync::Mutex::new(Vec::new());

        let status = run_cli_response_loop(
            |req| {
                requests.lock().unwrap().push(req);
                Ok(())
            },
            || {
                responses
                    .lock()
                    .unwrap()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no more responses"))
            },
            CliRequest::Open {
                paths: vec!["/tmp/test".into()],
                urls: vec![],
                diff_paths: vec![],
                diff_all: false,
                wsl: None,
                wait: false,
                open_new_workspace: None,
                force_existing_window: false,
                reuse: false,
                env: None,
                user_data_dir: None,
                dev_container: false,
            },
            || panic!("should not be called"),
            |_| {},
            |_| {},
        )
        .unwrap();

        assert_eq!(status, 0);
        let sent = requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert!(matches!(sent[0], CliRequest::Open { .. }));
    }

    #[test]
    fn test_response_loop_prompt_existing_window() {
        let responses = vec![
            CliResponse::PromptOpenBehavior,
            CliResponse::Exit { status: 0 },
        ];
        let responses = std::sync::Mutex::new(responses.into_iter());
        let requests = std::sync::Mutex::new(Vec::new());

        let status = run_cli_response_loop(
            |req| {
                requests.lock().unwrap().push(req);
                Ok(())
            },
            || {
                responses
                    .lock()
                    .unwrap()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no more responses"))
            },
            CliRequest::Open {
                paths: vec![],
                urls: vec![],
                diff_paths: vec![],
                diff_all: false,
                wsl: None,
                wait: false,
                open_new_workspace: None,
                force_existing_window: false,
                reuse: false,
                env: None,
                user_data_dir: None,
                dev_container: false,
            },
            || Some(true),
            |_| {},
            |_| {},
        )
        .unwrap();

        assert_eq!(status, 0);
        let sent = requests.lock().unwrap();
        assert_eq!(sent.len(), 2);
        assert!(matches!(sent[0], CliRequest::Open { .. }));
        assert!(matches!(
            sent[1],
            CliRequest::SetOpenBehavior {
                existing_window: true
            }
        ));
    }

    #[test]
    fn test_response_loop_prompt_new_window() {
        let responses = vec![
            CliResponse::PromptOpenBehavior,
            CliResponse::Exit { status: 0 },
        ];
        let responses = std::sync::Mutex::new(responses.into_iter());
        let requests = std::sync::Mutex::new(Vec::new());

        let status = run_cli_response_loop(
            |req| {
                requests.lock().unwrap().push(req);
                Ok(())
            },
            || {
                responses
                    .lock()
                    .unwrap()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no more responses"))
            },
            CliRequest::Open {
                paths: vec![],
                urls: vec![],
                diff_paths: vec![],
                diff_all: false,
                wsl: None,
                wait: false,
                open_new_workspace: None,
                force_existing_window: false,
                reuse: false,
                env: None,
                user_data_dir: None,
                dev_container: false,
            },
            || Some(false),
            |_| {},
            |_| {},
        )
        .unwrap();

        assert_eq!(status, 0);
        let sent = requests.lock().unwrap();
        assert_eq!(sent.len(), 2);
        assert!(matches!(
            sent[1],
            CliRequest::SetOpenBehavior {
                existing_window: false
            }
        ));
    }

    #[test]
    fn test_response_loop_stdout_stderr() {
        let responses = vec![
            CliResponse::Stdout {
                message: "hello".into(),
            },
            CliResponse::Stderr {
                message: "warning".into(),
            },
            CliResponse::Ping,
            CliResponse::Exit { status: 42 },
        ];
        let responses = std::sync::Mutex::new(responses.into_iter());
        let stdout = std::sync::Mutex::new(Vec::new());
        let stderr = std::sync::Mutex::new(Vec::new());

        let status = run_cli_response_loop(
            |_| Ok(()),
            || {
                responses
                    .lock()
                    .unwrap()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no more responses"))
            },
            CliRequest::Open {
                paths: vec![],
                urls: vec![],
                diff_paths: vec![],
                diff_all: false,
                wsl: None,
                wait: false,
                open_new_workspace: None,
                force_existing_window: false,
                reuse: false,
                env: None,
                user_data_dir: None,
                dev_container: false,
            },
            || panic!("should not prompt"),
            |msg| stdout.lock().unwrap().push(msg.to_string()),
            |msg| stderr.lock().unwrap().push(msg.to_string()),
        )
        .unwrap();

        assert_eq!(status, 42);
        assert_eq!(*stdout.lock().unwrap(), vec!["hello"]);
        assert_eq!(*stderr.lock().unwrap(), vec!["warning"]);
    }
}
