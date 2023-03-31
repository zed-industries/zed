mod update_notification;

use anyhow::{anyhow, Context, Result};
use client::{ZED_APP_PATH, ZED_APP_VERSION, ZED_SECRET_CLIENT_TOKEN};
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    actions, platform::AppVersion, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle,
    MutableAppContext, Task, WeakViewHandle,
};
use serde::Deserialize;
use settings::Settings;
use smol::{fs::File, io::AsyncReadExt, process::Command};
use std::{ffi::OsString, sync::Arc, time::Duration};
use update_notification::UpdateNotification;
use util::channel::ReleaseChannel;
use util::http::HttpClient;
use workspace::Workspace;

const SHOULD_SHOW_UPDATE_NOTIFICATION_KEY: &str = "auto-updater-should-show-updated-notification";
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

actions!(auto_update, [Check, DismissErrorMessage, ViewReleaseNotes]);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AutoUpdateStatus {
    Idle,
    Checking,
    Downloading,
    Installing,
    Updated,
    Errored,
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
    url: String,
}

impl Entity for AutoUpdater {
    type Event = ();
}

pub fn init(http_client: Arc<dyn HttpClient>, server_url: String, cx: &mut MutableAppContext) {
    if let Some(version) = (*ZED_APP_VERSION).or_else(|| cx.platform().app_version().ok()) {
        let server_url = server_url;
        let auto_updater = cx.add_model(|cx| {
            let updater = AutoUpdater::new(version, http_client, server_url.clone());

            let mut update_subscription = cx
                .global::<Settings>()
                .auto_update
                .then(|| updater.start_polling(cx));

            cx.observe_global::<Settings, _>(move |updater, cx| {
                if cx.global::<Settings>().auto_update {
                    if update_subscription.is_none() {
                        *(&mut update_subscription) = Some(updater.start_polling(cx))
                    }
                } else {
                    (&mut update_subscription).take();
                }
            })
            .detach();

            updater
        });
        cx.set_global(Some(auto_updater));
        cx.add_global_action(|_: &Check, cx| {
            if let Some(updater) = AutoUpdater::get(cx) {
                updater.update(cx, |updater, cx| updater.poll(cx));
            }
        });
        cx.add_global_action(move |_: &ViewReleaseNotes, cx| {
            let latest_release_url = if cx.has_global::<ReleaseChannel>()
                && *cx.global::<ReleaseChannel>() == ReleaseChannel::Preview
            {
                format!("{server_url}/releases/preview/latest")
            } else {
                format!("{server_url}/releases/latest")
            };
            cx.platform().open_url(&latest_release_url);
        });
        cx.add_action(UpdateNotification::dismiss);
    }
}

pub fn notify_of_any_new_update(
    workspace: WeakViewHandle<Workspace>,
    cx: &mut MutableAppContext,
) -> Option<()> {
    let updater = AutoUpdater::get(cx)?;
    let version = updater.read(cx).current_version;
    let should_show_notification = updater.read(cx).should_show_update_notification(cx);

    cx.spawn(|mut cx| async move {
        let should_show_notification = should_show_notification.await?;
        if should_show_notification {
            if let Some(workspace) = workspace.upgrade(&cx) {
                workspace.update(&mut cx, |workspace, cx| {
                    workspace.show_notification(0, cx, |cx| {
                        cx.add_view(|_| UpdateNotification::new(version))
                    });
                    updater
                        .read(cx)
                        .set_should_show_update_notification(false, cx)
                        .detach_and_log_err(cx);
                });
            }
        }
        anyhow::Ok(())
    })
    .detach();

    None
}

impl AutoUpdater {
    pub fn get(cx: &mut MutableAppContext) -> Option<ModelHandle<Self>> {
        cx.default_global::<Option<ModelHandle<Self>>>().clone()
    }

    fn new(
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

    pub fn start_polling(&self, cx: &mut ModelContext<Self>) -> Task<()> {
        cx.spawn(|this, mut cx| async move {
            loop {
                this.update(&mut cx, |this, cx| this.poll(cx));
                cx.background().timer(POLL_INTERVAL).await;
            }
        })
    }

    pub fn poll(&mut self, cx: &mut ModelContext<Self>) {
        if self.pending_poll.is_some() || self.status == AutoUpdateStatus::Updated {
            return;
        }

        self.status = AutoUpdateStatus::Checking;
        cx.notify();

        self.pending_poll = Some(cx.spawn(|this, mut cx| async move {
            let result = Self::update(this.clone(), cx.clone()).await;
            this.update(&mut cx, |this, cx| {
                this.pending_poll = None;
                if let Err(error) = result {
                    log::error!("auto-update failed: error:{:?}", error);
                    this.status = AutoUpdateStatus::Errored;
                    cx.notify();
                }
            });
        }));
    }

    pub fn status(&self) -> AutoUpdateStatus {
        self.status
    }

    pub fn dismiss_error(&mut self, cx: &mut ModelContext<Self>) {
        self.status = AutoUpdateStatus::Idle;
        cx.notify();
    }

    async fn update(this: ModelHandle<Self>, mut cx: AsyncAppContext) -> Result<()> {
        let (client, server_url, current_version) = this.read_with(&cx, |this, _| {
            (
                this.http_client.clone(),
                this.server_url.clone(),
                this.current_version,
            )
        });

        let preview_param = cx.read(|cx| {
            if cx.has_global::<ReleaseChannel>() {
                if *cx.global::<ReleaseChannel>() == ReleaseChannel::Preview {
                    return "&preview=1";
                }
            }
            ""
        });

        let mut response = client
            .get(
                &format!("{server_url}/api/releases/latest?token={ZED_SECRET_CLIENT_TOKEN}&asset=Zed.dmg{preview_param}"),
                Default::default(),
                true,
            )
            .await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading release")?;
        let release: JsonRelease =
            serde_json::from_slice(body.as_slice()).context("error deserializing release")?;

        let latest_version = release.version.parse::<AppVersion>()?;
        if latest_version <= current_version {
            this.update(&mut cx, |this, cx| {
                this.status = AutoUpdateStatus::Idle;
                cx.notify();
            });
            return Ok(());
        }

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Downloading;
            cx.notify();
        });

        let temp_dir = tempdir::TempDir::new("zed-auto-update")?;
        let dmg_path = temp_dir.path().join("Zed.dmg");
        let mount_path = temp_dir.path().join("Zed");
        let running_app_path = ZED_APP_PATH
            .clone()
            .map_or_else(|| cx.platform().app_path(), Ok)?;
        let running_app_filename = running_app_path
            .file_name()
            .ok_or_else(|| anyhow!("invalid running app path"))?;
        let mut mounted_app_path: OsString = mount_path.join(running_app_filename).into();
        mounted_app_path.push("/");

        let mut dmg_file = File::create(&dmg_path).await?;
        let mut response = client.get(&release.url, Default::default(), true).await?;
        smol::io::copy(response.body_mut(), &mut dmg_file).await?;
        log::info!("downloaded update. path:{:?}", dmg_path);

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Installing;
            cx.notify();
        });

        let output = Command::new("hdiutil")
            .args(&["attach", "-nobrowse"])
            .arg(&dmg_path)
            .arg("-mountroot")
            .arg(&temp_dir.path())
            .output()
            .await?;
        if !output.status.success() {
            Err(anyhow!(
                "failed to mount: {:?}",
                String::from_utf8_lossy(&output.stderr)
            ))?;
        }

        let output = Command::new("rsync")
            .args(&["-av", "--delete"])
            .arg(&mounted_app_path)
            .arg(&running_app_path)
            .output()
            .await?;
        if !output.status.success() {
            Err(anyhow!(
                "failed to copy app: {:?}",
                String::from_utf8_lossy(&output.stderr)
            ))?;
        }

        let output = Command::new("hdiutil")
            .args(&["detach"])
            .arg(&mount_path)
            .output()
            .await?;
        if !output.status.success() {
            Err(anyhow!(
                "failed to unmount: {:?}",
                String::from_utf8_lossy(&output.stderr)
            ))?;
        }

        this.update(&mut cx, |this, cx| {
            this.set_should_show_update_notification(true, cx)
                .detach_and_log_err(cx);
            this.status = AutoUpdateStatus::Updated;
            cx.notify();
        });
        Ok(())
    }

    fn set_should_show_update_notification(
        &self,
        should_show: bool,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        cx.background().spawn(async move {
            if should_show {
                KEY_VALUE_STORE
                    .write_kvp(
                        SHOULD_SHOW_UPDATE_NOTIFICATION_KEY.to_string(),
                        "".to_string(),
                    )
                    .await?;
            } else {
                KEY_VALUE_STORE
                    .delete_kvp(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY.to_string())
                    .await?;
            }
            Ok(())
        })
    }

    fn should_show_update_notification(&self, cx: &AppContext) -> Task<Result<bool>> {
        cx.background().spawn(async move {
            Ok(KEY_VALUE_STORE
                .read_kvp(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY)?
                .is_some())
        })
    }
}
