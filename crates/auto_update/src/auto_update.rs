mod update_notification;

use anyhow::{anyhow, Context, Result};
use client::{http::HttpClient, ZED_SECRET_CLIENT_TOKEN};
use gpui::{
    actions,
    elements::{Empty, MouseEventHandler, Text},
    platform::AppVersion,
    AppContext, AsyncAppContext, Element, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task, View, ViewContext, WeakViewHandle,
};
use lazy_static::lazy_static;
use serde::Deserialize;
use settings::Settings;
use smol::{fs::File, io::AsyncReadExt, process::Command};
use std::{env, ffi::OsString, path::PathBuf, sync::Arc, time::Duration};
use update_notification::UpdateNotification;
use workspace::{ItemHandle, StatusItemView, Workspace};

const SHOULD_SHOW_UPDATE_NOTIFICATION_KEY: &'static str =
    "auto-updater-should-show-updated-notification";
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

lazy_static! {
    pub static ref ZED_APP_VERSION: Option<AppVersion> = env::var("ZED_APP_VERSION")
        .ok()
        .and_then(|v| v.parse().ok());
    pub static ref ZED_APP_PATH: Option<PathBuf> = env::var("ZED_APP_PATH").ok().map(PathBuf::from);
}

actions!(auto_update, [Check, DismissErrorMessage, ViewReleaseNotes]);

#[derive(Clone, PartialEq, Eq)]
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
    db: Arc<project::Db>,
    server_url: String,
}

pub struct AutoUpdateIndicator {
    updater: Option<ModelHandle<AutoUpdater>>,
}

#[derive(Deserialize)]
struct JsonRelease {
    version: String,
    url: String,
}

impl Entity for AutoUpdater {
    type Event = ();
}

pub fn init(
    db: Arc<project::Db>,
    http_client: Arc<dyn HttpClient>,
    server_url: String,
    cx: &mut MutableAppContext,
) {
    if let Some(version) = ZED_APP_VERSION.clone().or(cx.platform().app_version().ok()) {
        let auto_updater = cx.add_model(|cx| {
            let updater = AutoUpdater::new(version, db.clone(), http_client, server_url.clone());
            updater.start_polling(cx).detach();
            updater
        });
        cx.set_global(Some(auto_updater));
        cx.add_global_action(|_: &Check, cx| {
            if let Some(updater) = AutoUpdater::get(cx) {
                updater.update(cx, |updater, cx| updater.poll(cx));
            }
        });
        cx.add_global_action(move |_: &ViewReleaseNotes, cx| {
            cx.platform().open_url(&format!("{server_url}/releases"));
        });
        cx.add_action(AutoUpdateIndicator::dismiss_error_message);
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
    fn get(cx: &mut MutableAppContext) -> Option<ModelHandle<Self>> {
        cx.default_global::<Option<ModelHandle<Self>>>().clone()
    }

    fn new(
        current_version: AppVersion,
        db: Arc<project::Db>,
        http_client: Arc<dyn HttpClient>,
        server_url: String,
    ) -> Self {
        Self {
            status: AutoUpdateStatus::Idle,
            current_version,
            db,
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

    async fn update(this: ModelHandle<Self>, mut cx: AsyncAppContext) -> Result<()> {
        let (client, server_url, current_version) = this.read_with(&cx, |this, _| {
            (
                this.http_client.clone(),
                this.server_url.clone(),
                this.current_version,
            )
        });
        let mut response = client
            .get(
                &format!("{server_url}/api/releases/latest?token={ZED_SECRET_CLIENT_TOKEN}&asset=Zed.dmg"),
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
        let mut mounted_app_path: OsString = mount_path.join("Zed.app").into();
        mounted_app_path.push("/");
        let running_app_path = ZED_APP_PATH
            .clone()
            .map_or_else(|| cx.platform().app_path(), Ok)?;

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
        let db = self.db.clone();
        cx.background().spawn(async move {
            if should_show {
                db.write([(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY, "")])?;
            } else {
                db.delete([(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY)])?;
            }
            Ok(())
        })
    }

    fn should_show_update_notification(&self, cx: &AppContext) -> Task<Result<bool>> {
        let db = self.db.clone();
        cx.background().spawn(async move {
            Ok(db.read([(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY)])?[0].is_some())
        })
    }
}

impl Entity for AutoUpdateIndicator {
    type Event = ();
}

impl View for AutoUpdateIndicator {
    fn ui_name() -> &'static str {
        "AutoUpdateIndicator"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        if let Some(updater) = &self.updater {
            let theme = &cx.global::<Settings>().theme.workspace.status_bar;
            match &updater.read(cx).status {
                AutoUpdateStatus::Checking => Text::new(
                    "Checking for updates…".to_string(),
                    theme.auto_update_progress_message.clone(),
                )
                .boxed(),
                AutoUpdateStatus::Downloading => Text::new(
                    "Downloading update…".to_string(),
                    theme.auto_update_progress_message.clone(),
                )
                .boxed(),
                AutoUpdateStatus::Installing => Text::new(
                    "Installing update…".to_string(),
                    theme.auto_update_progress_message.clone(),
                )
                .boxed(),
                AutoUpdateStatus::Updated => Text::new(
                    "Restart to update Zed".to_string(),
                    theme.auto_update_done_message.clone(),
                )
                .boxed(),
                AutoUpdateStatus::Errored => {
                    MouseEventHandler::new::<Self, _, _>(0, cx, |_, cx| {
                        let theme = &cx.global::<Settings>().theme.workspace.status_bar;
                        Text::new(
                            "Auto update failed".to_string(),
                            theme.auto_update_done_message.clone(),
                        )
                        .boxed()
                    })
                    .on_click(|_, _, cx| cx.dispatch_action(DismissErrorMessage))
                    .boxed()
                }
                AutoUpdateStatus::Idle => Empty::new().boxed(),
            }
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for AutoUpdateIndicator {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}

impl AutoUpdateIndicator {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let updater = AutoUpdater::get(cx);
        if let Some(updater) = &updater {
            cx.observe(updater, |_, _, cx| cx.notify()).detach();
        }
        Self { updater }
    }

    fn dismiss_error_message(&mut self, _: &DismissErrorMessage, cx: &mut ViewContext<Self>) {
        if let Some(updater) = &self.updater {
            updater.update(cx, |updater, cx| {
                updater.status = AutoUpdateStatus::Idle;
                cx.notify();
            });
        }
    }
}
