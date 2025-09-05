use std::mem;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::ExtensionHostProxy;
use fs::RealFs;
use futures::SinkExt;
use futures::channel::mpsc;
use git::GitHostingProviderRegistry;
use gpui::AppContext;
use gpui_tokio::Tokio;
use iroh::Endpoint;
use iroh::endpoint::Connection;
use iroh::protocol::{AcceptError, ProtocolHandler, Router};
use iroh_persist::KeyRetriever;
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use proto::Envelope;
use release_channel::{AppCommitSha, AppVersion};
use remote::{MAX_MESSAGE_SIZE, Message, RemoteClient, ZED_ALPN, ZedIrohTicket};
use reqwest_client::ReqwestClient;
use smol::stream::StreamExt as _;
use tokio_util::bytes::Bytes;
use tokio_util::codec::LengthDelimitedCodec;

use crate::unix::{
    VERSION, handle_crash_files_requests, init_paths, initialize_settings, read_proxy_settings,
};
use crate::{HeadlessAppState, HeadlessProject};

enum Im {
    NewSession {
        id: String,
        sender: futures::channel::oneshot::Sender<(
            mpsc::UnboundedSender<Envelope>,
            mpsc::UnboundedReceiver<Envelope>,
            mpsc::UnboundedReceiver<()>,
        )>,
    },
}

fn init_logging_p2p() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn,tracing::span=off"),
    )
    .filter_level(log::LevelFilter::Info)
    .parse_default_env()
    .init()
}

pub(crate) fn execute(persist: bool, mut persist_at: Option<PathBuf>) -> Result<()> {
    init_logging_p2p();

    // Known bugs
    // - Server: logs are not captured
    // - UI: can't use arrows

    let app = gpui::Application::headless();
    let id = std::process::id().to_string();
    app.background_executor()
        .spawn(crashes::init(crashes::InitCrashHandler {
            session_id: id,
            zed_version: VERSION.to_owned(),
            binary: "zed-remote-server".to_string(),
            release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
            commit_sha: option_env!("ZED_COMMIT_SHA").unwrap_or("no_sha").to_owned(),
        }))
        .detach();

    log::info!("starting p2p process. PID: {}", std::process::id());

    init_paths()?;

    if persist && persist_at.is_none() {
        let mut secret_path = paths::config_dir().clone();
        secret_path.push("zedIrohKey.pem");
        persist_at = Some(secret_path);
    }

    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    app.run(move |cx| {
        settings::init(cx);
        let app_commit_sha = option_env!("ZED_COMMIT_SHA").map(|s| AppCommitSha::new(s.to_owned()));
        let app_version = AppVersion::load(
            env!("ZED_PKG_VERSION"),
            option_env!("ZED_BUILD_ID"),
            app_commit_sha,
        );
        release_channel::init(app_version, cx);
        gpui_tokio::init(cx);

        HeadlessProject::init(cx);

        log::info!("gpui app started, initializing server");
        log::debug!("Persist at: [{:?}]", persist_at);

        GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);
        git_hosting_providers::init(cx);
        dap_adapters::init(cx);

        extension::init(cx);
        let extension_host_proxy_main = ExtensionHostProxy::global(cx);

        let (s, mut r) = mpsc::unbounded::<Im>();

        gpui_tokio::Tokio::spawn(cx, async move {
            let iroh = match IrohZedListener::accept(s, persist_at.as_ref()).await {
                Ok(iroh) => iroh,
                Err(error) => {
                    log::error!("failed to start iroh {error:?}");
                    return;
                }
            };
            log::info!("ADDR: iroh started {}", iroh.endpoint().id());

            iroh.endpoint().online().await;
            let ticket = iroh.ticket();
            println!("TICKET: {}", ticket);

            // TODO: how to do a better shutdown
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        })
        .detach();

        cx.spawn(async move |cx| {
            while let Some(message) = futures::StreamExt::next(&mut r).await {
                match message {
                    Im::NewSession {
                        id,
                        sender: response_sender,
                    } => {
                        log::info!("new session started: {id}");
                        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
                        let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
                        let (app_quit_tx, app_quit_rx) = mpsc::unbounded::<()>();

                        let extension_host_proxy = extension_host_proxy_main.clone();
                        let project = cx.update(|cx| {
                            cx.on_app_quit(move |_| {
                                let mut app_quit_tx = app_quit_tx.clone();
                                async move {
                                    log::info!("project quitting");
                                    app_quit_tx.send(()).await.ok();
                                }
                            })
                            .detach();
                            let session = RemoteClient::proto_client_from_channels(
                                incoming_rx,
                                outgoing_tx,
                                cx,
                                "server",
                                false,
                            );
                            let project = cx.new(|cx| {
                                let fs =
                                    Arc::new(RealFs::new(None, cx.background_executor().clone()));
                                let node_settings_rx =
                                    initialize_settings(session.clone(), fs.clone(), cx);

                                let proxy_url = read_proxy_settings(cx);

                                let http_client = {
                                    let _guard = Tokio::handle(cx).enter();
                                    Arc::new(
                                        ReqwestClient::proxy_and_user_agent(
                                            proxy_url,
                                            &format!(
                                                "Zed-Server/{} ({}; {})",
                                                env!("CARGO_PKG_VERSION"),
                                                std::env::consts::OS,
                                                std::env::consts::ARCH
                                            ),
                                        )
                                        .expect("Could not start HTTP client"),
                                    )
                                };

                                let node_runtime =
                                    NodeRuntime::new(http_client.clone(), None, node_settings_rx);

                                let mut languages =
                                    LanguageRegistry::new(cx.background_executor().clone());
                                languages.set_language_server_download_dir(
                                    paths::languages_dir().clone(),
                                );
                                let languages = Arc::new(languages);

                                log::info!("creating project");
                                HeadlessProject::new(
                                    HeadlessAppState {
                                        session: session.clone(),
                                        fs,
                                        http_client,
                                        node_runtime,
                                        languages,
                                        extension_host_proxy,
                                    },
                                    cx,
                                )
                            });

                            handle_crash_files_requests(&project, &session);
                            project
                        })?;
                        response_sender
                            .send((incoming_tx, outgoing_rx, app_quit_rx))
                            .ok();

                        log::info!("project handled");
                        mem::forget(project);
                    }
                }
            }

            anyhow::Ok(())
        })
        .detach();
    });

    log::info!("gpui app is shut down. quitting.");

    Ok(())
}

#[derive(Debug, Clone)]
struct IrohZedListener {
    endpoint: Endpoint,
    _router: Router,
}

impl IrohZedListener {
    async fn accept(tx: mpsc::UnboundedSender<Im>, persist_at: Option<&PathBuf>) -> Result<Self> {
        let key = KeyRetriever::new("zed")
            .persist_at(persist_at)
            .lenient()
            .get()
            .await;
        let endpoint = Endpoint::builder()
            .secret_key(key)
            .alpns(vec![ZED_ALPN.to_vec()])
            .bind()
            .await?;

        let router = Router::builder(endpoint.clone())
            .accept(ZED_ALPN, IrohZedProtocolHandler::new(tx))
            .spawn();

        Ok(Self {
            endpoint,
            _router: router,
        })
    }

    fn ticket(&self) -> ZedIrohTicket {
        let addr = self.endpoint.addr();
        ZedIrohTicket::new(addr)
    }

    fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }
}

#[derive(Debug)]
struct IrohZedProtocolHandler {
    tx: mpsc::UnboundedSender<Im>,
}

impl IrohZedProtocolHandler {
    fn new(tx: mpsc::UnboundedSender<Im>) -> Self {
        Self { tx }
    }
}

impl ProtocolHandler for IrohZedProtocolHandler {
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        let remote_id = connection.remote_id();
        log::info!("accepted connection: {remote_id:?}");

        let (send, recv) = connection.accept_bi().await?;
        let mut codec = LengthDelimitedCodec::builder();
        codec.max_frame_length(MAX_MESSAGE_SIZE);
        let mut write = codec.new_write(send);
        let mut read = codec.new_read(recv);

        let (s, r) = futures::channel::oneshot::channel();

        // TODO: do we need to wait for id?
        self.tx
            .unbounded_send(Im::NewSession {
                id: "new".to_string(),
                sender: s,
            })
            .map_err(|err| AcceptError::from_err(err))?;

        let (incoming_tx, mut outgoing_rx, mut app_quit_rx) =
            r.await.map_err(AcceptError::from_err)?;

        //let log_rx = self.log_rx.clone();

        tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    outgoing_message = outgoing_rx.next() => {
                        if let Some(outgoing_message) = outgoing_message {
                            let encoded = postcard::to_stdvec(&Message::Envelope(outgoing_message)).expect("invalid encoding");

                            if let Err(error) = write.send(Bytes::from(encoded)).await {
                                log::error!("failed to write outgoing message: {:?}", error);
                                break;
                            }
                        }
                    }
                    _ = app_quit_rx.next() => {
                        break;
                    }
                    // log_message = log_rx.recv() => {
                    //     if let Ok(log_message) = log_message {
                    //         if let Ok(record) = serde_json::from_slice::<LogRecord>(&log_message) {
                    //             let encoded = postcard::to_stdvec(&Message::Log(record)).expect("invalid encoding");
                    //             if let Err(error) = write.send(Bytes::from(encoded)).await {
                    //                 log::error!("failed to write outgoing message: {:?}", error);
                    //                 break;
                    //             }
                    //         } else {
                    //             eprintln!("(remote) {}", String::from_utf8_lossy(&log_message));
                    //         }
                    //     }
                    // }
                }
            }

            log::warn!("exiting write task");
        });

        tokio::task::spawn(async move {
            while let Some(raw) = read.next().await {
                let raw = match raw {
                    Ok(raw) => raw,
                    Err(error) => {
                        log::error!("received invalid message: {error:?}");
                        break;
                    }
                };
                match postcard::from_bytes::<Message>(&raw) {
                    Ok(message) => {
                        log::info!("received message {:?}", message);
                        match message {
                            Message::Envelope(envelope) => {
                                if let Err(error) = incoming_tx.unbounded_send(envelope) {
                                    log::error!(
                                        "failed to send message to application: {error:?}. exiting."
                                    );
                                    break;
                                }
                            }
                            Message::Log(record) => record.log(log::logger()),
                        }
                    }
                    Err(error) => {
                        log::error!("received in valid message: {error:?}.");
                    }
                }
            }
            log::warn!("exiting read task");
        });

        // Wait until the remote closes the connection, which it does once it
        // received the response.
        connection.closed().await;
        log::warn!("exiting conn {remote_id:?}");

        Ok(())
    }
}
