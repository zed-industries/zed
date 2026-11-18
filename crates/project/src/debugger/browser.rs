use anyhow::{Context as _, Result, bail};
use async_tungstenite::{client_async, tungstenite};
use dap::adapters::DebugAdapterBinary;
use futures::{AsyncReadExt as _, FutureExt as _, StreamExt as _};
use gpui::BackgroundExecutor;
use http_client::{HttpClient, HttpRequestExt as _};
use serde::Deserialize;
use smol::net::TcpStream;
use std::{path::PathBuf, time::Duration};
use url::Url;

#[derive(PartialEq, Eq)]
pub(super) struct LaunchedBrowser {
    endpoint: Url,
    profile: BrowserProfile,
}

/// The `--user-data-dir` a launched browser runs with, or `None` for js-debug's default profile.
pub(super) type BrowserProfile = Option<PathBuf>;

impl LaunchedBrowser {
    /// The profile of a browser that Zed tracks for this launch, if it tracks one.
    pub fn launch_profile(binary: &DebugAdapterBinary) -> Option<BrowserProfile> {
        let configuration = &binary.request_args.configuration;
        if binary.request_args.request != dap::StartDebuggingRequestArgumentsRequest::Launch
            || !matches!(
                configuration
                    .get("type")
                    .and_then(serde_json::Value::as_str),
                Some("pwa-chrome" | "pwa-msedge")
            )
            || configuration
                .get("browserLaunchLocation")
                .and_then(serde_json::Value::as_str)
                == Some("ui")
            || configuration
                .get("port")
                .and_then(serde_json::Value::as_u64)
                .is_none()
        {
            return None;
        }
        let mut arguments = configuration
            .get("runtimeArgs")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str);
        while let Some(argument) = arguments.next() {
            if let Some(directory) = argument.strip_prefix("--user-data-dir=") {
                return Some(Some(PathBuf::from(directory)));
            }
            if argument == "--user-data-dir" {
                return Some(arguments.next().map(PathBuf::from));
            }
        }
        Some(
            configuration
                .get("userDataDir")
                .and_then(serde_json::Value::as_str)
                .map(PathBuf::from),
        )
    }

    pub fn profile(&self) -> &BrowserProfile {
        &self.profile
    }

    pub async fn capture(
        binary: &DebugAdapterBinary,
        http_client: &dyn HttpClient,
    ) -> Result<Option<Self>> {
        let Some(profile) = Self::launch_profile(binary) else {
            return Ok(None);
        };
        let Some(port) = binary
            .request_args
            .configuration
            .get("port")
            .and_then(serde_json::Value::as_u64)
        else {
            return Ok(None);
        };

        let request = http_client::Request::get(format!("http://127.0.0.1:{port}/json/version"))
            .timeout(std::time::Duration::from_secs(3))
            .body(Default::default())?;
        let mut response = http_client.send(request).await?;
        if !response.status().is_success() {
            bail!(
                "Could not identify the launched browser: {}",
                response.status()
            );
        }
        let mut body = Vec::new();
        response
            .body_mut()
            .take(64 * 1024)
            .read_to_end(&mut body)
            .await?;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BrowserVersion {
            web_socket_debugger_url: Url,
        }
        let version: BrowserVersion = serde_json::from_slice(&body)?;
        // The browser ID in the endpoint prevents closing a different process if the port is reused.
        Ok(Some(Self {
            endpoint: version.web_socket_debugger_url,
            profile,
        }))
    }

    /// Closes the browser and waits for it to release its profile, so a browser launched next
    /// with the same profile starts its own process instead of handing its window to this one.
    pub async fn close_and_wait_for_exit(self, executor: &BackgroundExecutor) -> Result<()> {
        let host = self
            .endpoint
            .host_str()
            .context("Browser debugging endpoint has no host")?
            .to_owned();
        let port = self
            .endpoint
            .port_or_known_default()
            .context("Browser debugging endpoint has no port")?;
        let singleton_lock = self
            .profile
            .as_ref()
            .map(|profile| profile.join("SingletonLock"));
        futures::select_biased! {
            result = self.close().fuse() => result?,
            _ = executor.timer(Duration::from_secs(3)).fuse() => {
                bail!("Timed out closing a browser launched by the debugger")
            }
        }
        for _ in 0..50 {
            let endpoint_closed = TcpStream::connect((host.as_str(), port)).await.is_err();
            let profile_released = singleton_lock
                .as_ref()
                .is_none_or(|lock| std::fs::symlink_metadata(lock).is_err());
            if endpoint_closed && profile_released {
                return Ok(());
            }
            executor.timer(Duration::from_millis(100)).await;
        }
        bail!("The browser launched by the debugger did not exit")
    }

    pub async fn close(self) -> Result<()> {
        let host = self
            .endpoint
            .host_str()
            .context("Browser debugging endpoint has no host")?;
        let port = self
            .endpoint
            .port_or_known_default()
            .context("Browser debugging endpoint has no port")?;
        let stream = match TcpStream::connect((host, port)).await {
            Ok(stream) => stream,
            Err(error) if error.kind() == std::io::ErrorKind::ConnectionRefused => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let (mut connection, _) = match client_async(self.endpoint.as_str(), stream).await {
            Ok(connection) => connection,
            Err(tungstenite::Error::Http(response))
                if response.status() == http_client::StatusCode::NOT_FOUND =>
            {
                return Ok(());
            }
            Err(error) => return Err(error.into()),
        };
        connection
            .send(tungstenite::Message::Text(
                r#"{"id":1,"method":"Browser.close"}"#.into(),
            ))
            .await?;
        while let Some(message) = connection.next().await {
            match message? {
                tungstenite::Message::Text(message) => {
                    let response: serde_json::Value = serde_json::from_str(&message)?;
                    if response.get("id").and_then(serde_json::Value::as_u64) == Some(1) {
                        if let Some(error) = response.get("error") {
                            bail!("Could not close the launched browser: {error}");
                        }
                        return Ok(());
                    }
                }
                tungstenite::Message::Close(_) => return Ok(()),
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{AsyncBufReadExt as _, AsyncWriteExt as _};
    use http_client::{AsyncBody, FakeHttpClient, Response};
    use serde_json::json;
    use smol::net::TcpListener;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    fn binary(
        configuration: serde_json::Value,
        request: dap::StartDebuggingRequestArgumentsRequest,
    ) -> DebugAdapterBinary {
        DebugAdapterBinary {
            command: None,
            arguments: Vec::new(),
            envs: Default::default(),
            cwd: None,
            connection: None,
            request_args: dap::StartDebuggingRequestArguments {
                configuration,
                request,
            },
        }
    }

    #[test]
    fn identifies_the_profile_of_tracked_browser_launches() {
        let launch = dap::StartDebuggingRequestArgumentsRequest::Launch;
        let profile =
            |configuration| LaunchedBrowser::launch_profile(&binary(configuration, launch));
        assert_eq!(
            profile(json!({
                "type": "pwa-chrome", "port": 9222,
                "runtimeArgs": ["--disable-gpu", "--user-data-dir=/chrome-dev"],
            })),
            Some(Some(PathBuf::from("/chrome-dev")))
        );
        assert_eq!(
            profile(json!({
                "type": "pwa-msedge", "port": 9222,
                "runtimeArgs": ["--user-data-dir", "/edge-dev"],
            })),
            Some(Some(PathBuf::from("/edge-dev")))
        );
        assert_eq!(
            profile(json!({"type": "pwa-chrome", "port": 9222, "userDataDir": "/configured"})),
            Some(Some(PathBuf::from("/configured")))
        );
        assert_eq!(
            profile(json!({"type": "pwa-chrome", "port": 9222})),
            Some(None)
        );
        for configuration in [
            json!({"type": "pwa-chrome"}),
            json!({"type": "pwa-node", "port": 9222}),
            json!({"type": "pwa-chrome", "port": 9222, "browserLaunchLocation": "ui"}),
        ] {
            assert_eq!(profile(configuration), None);
        }
        assert_eq!(
            LaunchedBrowser::launch_profile(&binary(
                json!({"type": "pwa-chrome", "port": 9222}),
                dap::StartDebuggingRequestArgumentsRequest::Attach,
            )),
            None
        );
    }

    #[test]
    #[expect(
        clippy::result_large_err,
        reason = "The WebSocket handshake callback requires tungstenite's HTTP response type"
    )]
    fn closes_the_captured_browser_without_rediscovering_the_port() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).await?;
            let port = listener.local_addr()?.port();
            let endpoint = format!("ws://127.0.0.1:{port}/devtools/browser/original-browser");
            let requests = Arc::new(AtomicUsize::new(0));
            let requests_received = requests.clone();
            let http_client = FakeHttpClient::create(move |request| {
                let endpoint = endpoint.clone();
                let requests_received = requests_received.clone();
                async move {
                    assert_eq!(requests_received.fetch_add(1, Ordering::SeqCst), 0);
                    assert_eq!(request.uri().path(), "/json/version");
                    Ok(Response::builder().body(AsyncBody::from(serde_json::to_vec(
                        &json!({"webSocketDebuggerUrl": endpoint}),
                    )?))?)
                }
            });
            let browser = LaunchedBrowser::capture(
                &binary(
                    json!({"type": "pwa-chrome", "port": port}),
                    dap::StartDebuggingRequestArgumentsRequest::Launch,
                ),
                http_client.as_ref(),
            )
            .await?
            .context("A browser launch should be tracked")?;
            let server = smol::spawn(async move {
                let (stream, _) = listener.accept().await?;
                let mut connection = async_tungstenite::accept_hdr_async(
                    stream,
                    |request: &tungstenite::handshake::server::Request, response| {
                        assert_eq!(request.uri().path(), "/devtools/browser/original-browser");
                        Ok(response)
                    },
                )
                .await?;
                let request = connection.next().await.context("Browser close request")??;
                let request: serde_json::Value = serde_json::from_str(request.to_text()?)?;
                assert_eq!(request, json!({"id": 1, "method": "Browser.close"}));
                connection
                    .send(tungstenite::Message::Text(r#"{"id":1,"result":{}}"#.into()))
                    .await?;
                anyhow::Ok(())
            });
            browser.close().await?;
            server.await?;
            assert_eq!(requests.load(Ordering::SeqCst), 1);
            Ok(())
        })
    }

    #[test]
    fn does_not_track_attached_browsers_or_other_targets() -> Result<()> {
        smol::block_on(async {
            let http_client = FakeHttpClient::create(|_| async {
                bail!("These sessions must not take ownership of a browser")
            });
            for binary in [
                binary(
                    json!({"type": "pwa-chrome", "port": 9222}),
                    dap::StartDebuggingRequestArgumentsRequest::Attach,
                ),
                binary(
                    json!({"type": "pwa-node", "port": 9222}),
                    dap::StartDebuggingRequestArgumentsRequest::Launch,
                ),
                binary(
                    json!({"type": "pwa-chrome", "port": 9222, "browserLaunchLocation": "ui"}),
                    dap::StartDebuggingRequestArgumentsRequest::Launch,
                ),
            ] {
                assert!(
                    LaunchedBrowser::capture(&binary, http_client.as_ref())
                        .await?
                        .is_none()
                );
            }
            Ok(())
        })
    }

    #[test]
    fn does_not_close_a_replacement_browser_on_the_same_port() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).await?;
            let port = listener.local_addr()?.port();
            let browser = LaunchedBrowser {
                endpoint: Url::parse(&format!(
                    "ws://127.0.0.1:{port}/devtools/browser/exited-browser"
                ))?,
                profile: None,
            };
            let server = smol::spawn(async move {
                let (stream, _) = listener.accept().await?;
                let mut stream = futures::io::BufReader::new(stream);
                loop {
                    let mut header = String::new();
                    stream.read_line(&mut header).await?;
                    if header == "\r\n" {
                        break;
                    }
                }
                stream
                    .get_mut()
                    .write_all(
                        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    )
                    .await?;
                stream.get_mut().flush().await?;
                Ok::<_, anyhow::Error>(())
            });
            browser.close().await?;
            server.await?;
            Ok(())
        })
    }

    #[test]
    #[ignore = "requires a Chromium executable in ZED_CHROMIUM_TEST_EXECUTABLE"]
    #[expect(
        clippy::disallowed_methods,
        reason = "This integration test drives a real browser outside GPUI's deterministic scheduler"
    )]
    fn closes_a_real_browser_after_its_debugged_tab_has_closed() -> Result<()> {
        smol::block_on(async {
            let executable = std::env::var("ZED_CHROMIUM_TEST_EXECUTABLE")?;
            let profile = tempfile::tempdir()?;
            let mut process = smol::process::Command::new(executable)
                .args([
                    "--headless=new",
                    "--no-sandbox",
                    "--remote-debugging-port=0",
                ])
                .arg(format!("--user-data-dir={}", profile.path().display()))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .kill_on_drop(true)
                .spawn()?;
            let endpoint = async {
                loop {
                    if let Ok(contents) =
                        smol::fs::read_to_string(profile.path().join("DevToolsActivePort")).await
                    {
                        let mut lines = contents.lines();
                        if let (Some(port), Some(path)) = (lines.next(), lines.next()) {
                            break Url::parse(&format!("ws://127.0.0.1:{port}{path}"));
                        }
                    }
                    smol::Timer::after(std::time::Duration::from_millis(25)).await;
                }
            };
            let endpoint = futures::select_biased! {
                endpoint = endpoint.fuse() => endpoint?,
                _ = futures::FutureExt::fuse(smol::Timer::after(std::time::Duration::from_secs(10))) => bail!("Chromium did not start"),
            };
            let stream = TcpStream::connect((
                endpoint.host_str().context("browser host")?,
                endpoint.port().context("browser port")?,
            ))
            .await?;
            let (mut connection, _) = client_async(endpoint.as_str(), stream).await?;
            async fn request(
                connection: &mut async_tungstenite::WebSocketStream<TcpStream>,
                sequence: u64,
                method: &str,
                params: serde_json::Value,
            ) -> Result<serde_json::Value> {
                connection
                    .send(tungstenite::Message::Text(
                        json!({"id": sequence, "method": method, "params": params})
                            .to_string()
                            .into(),
                    ))
                    .await?;
                while let Some(message) = connection.next().await {
                    if let tungstenite::Message::Text(message) = message? {
                        let message: serde_json::Value = serde_json::from_str(&message)?;
                        if message["id"] == sequence {
                            if let Some(error) = message.get("error") {
                                bail!("CDP request failed: {error}");
                            }
                            return Ok(message["result"].clone());
                        }
                    }
                }
                bail!("Browser closed before responding")
            }
            let debugged = request(
                &mut connection,
                1,
                "Target.createTarget",
                json!({"url": "data:text/html,Debugged"}),
            )
            .await?;
            let other = request(
                &mut connection,
                2,
                "Target.createTarget",
                json!({"url": "data:text/html,Other"}),
            )
            .await?;
            request(
                &mut connection,
                3,
                "Target.closeTarget",
                json!({"targetId": debugged["targetId"]}),
            )
            .await?;
            let targets = request(&mut connection, 4, "Target.getTargets", json!({})).await?;
            assert!(
                targets["targetInfos"]
                    .as_array()
                    .context("browser targets")?
                    .iter()
                    .any(|target| target["targetId"] == other["targetId"])
            );
            LaunchedBrowser {
                endpoint,
                profile: None,
            }
            .close()
            .await?;
            futures::select_biased! {
                status = process.status().fuse() => { status?; },
                _ = futures::FutureExt::fuse(smol::Timer::after(std::time::Duration::from_secs(3))) => bail!("Browser process survived workspace cleanup"),
            }
            Ok(())
        })
    }
}
