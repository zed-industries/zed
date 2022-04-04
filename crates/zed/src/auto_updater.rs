use anyhow::{anyhow, Result};
use client::http::{self, HttpClient};
use gpui::{platform::AppVersion, AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use serde::Deserialize;
use smol::io::AsyncReadExt;
use std::{sync::Arc, time::Duration};
use surf::Request;

const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone, PartialEq, Eq)]
pub enum AutoUpdateStatus {
    Idle,
    Checking,
    Downloading,
    Updated,
    Errored { error: String },
}

pub struct AutoUpdater {
    status: AutoUpdateStatus,
    current_version: AppVersion,
    http_client: Arc<dyn HttpClient>,
    pending_poll: Option<Task<()>>,
    server_url: String,
}

#[derive(Deserialize)]
struct JsonRelease {
    version: String,
    url: http::Url,
}

impl Entity for AutoUpdater {
    type Event = ();
}

impl AutoUpdater {
    pub fn new(
        current_version: AppVersion,
        http_client: Arc<dyn HttpClient>,
        server_url: String,
    ) -> Self {
        Self {
            status: AutoUpdateStatus::Idle,
            current_version,
            http_client,
            server_url,
            pending_poll: None,
        }
    }

    pub fn start_polling(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        cx.spawn(|this, mut cx| async move {
            loop {
                this.update(&mut cx, |this, cx| this.poll(cx));
                cx.background().timer(POLL_INTERVAL).await;
            }
        })
    }

    pub fn poll(&mut self, cx: &mut ModelContext<Self>) {
        if self.pending_poll.is_some() {
            return;
        }

        self.status = AutoUpdateStatus::Checking;
        self.pending_poll = Some(cx.spawn(|this, mut cx| async move {
            if let Err(error) = Self::update(this.clone(), cx.clone()).await {
                this.update(&mut cx, |this, cx| {
                    this.status = AutoUpdateStatus::Errored {
                        error: error.to_string(),
                    };
                    cx.notify();
                });
            }

            this.update(&mut cx, |this, _| this.pending_poll = None);
        }));
        cx.notify();
    }

    async fn update(this: ModelHandle<Self>, mut cx: AsyncAppContext) -> Result<()> {
        let (client, server_url) = this.read_with(&cx, |this, _| {
            (this.http_client.clone(), this.server_url.clone())
        });
        let mut response = client
            .send(Request::new(
                http::Method::Get,
                http::Url::parse(&format!("{server_url}/api/releases/latest"))?,
            ))
            .await?;
        let release = response
            .body_json::<JsonRelease>()
            .await
            .map_err(|err| anyhow!("error deserializing release {:?}", err))?;
        let latest_version = release.version.parse::<AppVersion>()?;
        let current_version = cx.platform().app_version()?;
        if latest_version <= current_version {
            this.update(&mut cx, |this, cx| {
                this.status = AutoUpdateStatus::Idle;
                cx.notify();
            });
            return Ok(());
        }

        let temp_dir = tempdir::TempDir::new("zed")?;
        let dmg_path = temp_dir.path().join("Zed.dmg");
        let mut dmg_file = smol::fs::File::create(dmg_path).await?;
        let response = client
            .send(Request::new(http::Method::Get, release.url))
            .await?;
        smol::io::copy(response.bytes(), &mut dmg_file).await?;

        Ok(())
    }
}
