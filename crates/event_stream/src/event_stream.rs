use anyhow::Result;
use gpui::{AppContext, BorrowAppContext, Global, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use smol::{channel::Sender, io::AsyncWriteExt, stream::StreamExt};
use std::{path::PathBuf, time::Duration};
use util::{ResultExt, TryFutureExt};

#[derive(Serialize)]
pub enum OutputEvent {
    Hello,
    Save { path: PathBuf },
}

struct EventServer {
    handler: Option<IoHandler>,
}

struct IoHandler {
    tx: Sender<OutputEvent>,
    _task: Task<Option<()>>,
}

impl Global for EventServer {}

impl EventServer {
    pub fn send(&self, event: OutputEvent) {
        if let Some(handler) = &self.handler {
            if dbg!(handler.tx.receiver_count()) > 1 {
                handler.tx.try_send(event).log_err();
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema, Default)]
struct EventStreamSettings {
    event_stream: Option<PathBuf>,
}

impl Settings for EventStreamSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        sources: settings::SettingsSources<Self::FileContent>,
        _cx: &mut AppContext,
    ) -> gpui::Result<Self>
    where
        Self: Sized,
    {
        sources.json_merge()
    }
}

pub fn init(cx: &mut AppContext) {
    EventStreamSettings::register(cx);
    let mut socket_path = EventStreamSettings::get_global(cx).event_stream.clone();
    let mut server = EventServer::new();
    server.set_socket_path(socket_path.clone(), cx).log_err();
    cx.set_global(server);

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_socket_path = &EventStreamSettings::get_global(cx).event_stream;
        if *new_socket_path != socket_path {
            socket_path = new_socket_path.clone();
            cx.update_global(|server: &mut EventServer, cx| {
                server.set_socket_path(socket_path.clone(), cx).log_err();
            });
        }
    })
    .detach();

    // TODO: Remove this test code
    cx.spawn(|cx| async move {
        loop {
            cx.background_executor()
                .timer(Duration::from_millis(2000))
                .await;

            cx.update(|cx| {
                cx.global::<EventServer>().send(OutputEvent::Hello);
            })
            .ok();
        }
    })
    .detach();
}

impl EventServer {
    pub fn new() -> Self {
        Self { handler: None }
    }

    pub fn set_socket_path(&mut self, path: Option<PathBuf>, cx: &AppContext) -> Result<()> {
        if let Some(path) = path {
            let executor = cx.background_executor().clone();
            let (tx, rx) = smol::channel::unbounded();
            let _task = cx.background_executor().spawn(
                async move {
                    smol::fs::remove_file(&path).await.ok();
                    let listener = smol::net::unix::UnixListener::bind(&path)?;

                    let mut incoming = listener.incoming();
                    while let Some(stream) = incoming.next().await {
                        if let Some(mut stream) = stream.log_err() {
                            let rx = rx.clone();
                            executor
                                .spawn(
                                    async move {
                                        while let Some(message) = rx.recv().await.ok() {
                                            stream
                                                .write_all(
                                                    serde_json::to_string(&message)?.as_bytes(),
                                                )
                                                .await?;
                                            stream.write_all(b"\n").await?;
                                        }
                                        anyhow::Ok(())
                                    }
                                    .log_err(),
                                )
                                .detach();
                        }
                    }

                    anyhow::Ok(())
                }
                .log_err(),
            );

            self.handler = Some(IoHandler { tx, _task });
        } else {
            self.handler = None;
        }

        Ok(())
    }
}
