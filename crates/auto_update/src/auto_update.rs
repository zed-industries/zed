mod update_notification;

use anyhow::{anyhow, Context, Result};
use client::{Client, TelemetrySettings, ZED_APP_PATH};
use db::kvp::KEY_VALUE_STORE;
use db::RELEASE_CHANNEL;
use editor::{Editor, MultiBuffer};
use gpui::{
    actions, AppContext, AsyncAppContext, Context as _, Global, Model, ModelContext,
    SemanticVersion, SharedString, Task, View, ViewContext, VisualContext, WindowContext,
};
use isahc::AsyncBody;

use markdown_preview::markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use smol::{fs, io::AsyncReadExt};

use settings::{Settings, SettingsSources, SettingsStore};
use smol::{fs::File, process::Command};

use http::{HttpClient, HttpClientWithUrl};
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use std::{
    env::consts::{ARCH, OS},
    ffi::OsString,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use update_notification::UpdateNotification;
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::Workspace;

const SHOULD_SHOW_UPDATE_NOTIFICATION_KEY: &str = "auto-updater-should-show-updated-notification";
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

actions!(
    auto_update,
    [
        Check,
        DismissErrorMessage,
        ViewReleaseNotes,
        ViewReleaseNotesLocally
    ]
);

#[derive(Serialize)]
struct UpdateRequestBody {
    installation_id: Option<Arc<str>>,
    release_channel: Option<&'static str>,
    telemetry: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub enum AutoUpdateStatus {
    Idle,
    Checking,
    Downloading,
    Installing,
    Updated { binary_path: PathBuf },
    Errored,
}

impl AutoUpdateStatus {
    pub fn is_updated(&self) -> bool {
        matches!(self, Self::Updated { .. })
    }
}

pub struct AutoUpdater {
    status: AutoUpdateStatus,
    current_version: SemanticVersion,
    http_client: Arc<HttpClientWithUrl>,
    pending_poll: Option<Task<Option<()>>>,
}

#[derive(Deserialize)]
struct JsonRelease {
    version: String,
    url: String,
}

struct AutoUpdateSetting(bool);

/// Whether or not to automatically check for updates.
///
/// Default: true
#[derive(Clone, Copy, Default, JsonSchema, Deserialize, Serialize)]
#[serde(transparent)]
struct AutoUpdateSettingContent(bool);

impl Settings for AutoUpdateSetting {
    const KEY: Option<&'static str> = Some("auto_update");

    type FileContent = Option<AutoUpdateSettingContent>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        let auto_update = [sources.release_channel, sources.user]
            .into_iter()
            .find_map(|value| value.copied().flatten())
            .unwrap_or(sources.default.ok_or_else(Self::missing_default)?);

        Ok(Self(auto_update.0))
    }
}

#[derive(Default)]
struct GlobalAutoUpdate(Option<Model<AutoUpdater>>);

impl Global for GlobalAutoUpdate {}

#[derive(Deserialize)]
struct ReleaseNotesBody {
    title: String,
    release_notes: String,
}

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut AppContext) {
    AutoUpdateSetting::register(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|_, action: &Check, cx| check(action, cx));

        workspace.register_action(|_, action, cx| {
            view_release_notes(action, cx);
        });

        workspace.register_action(|workspace, _: &ViewReleaseNotesLocally, cx| {
            view_release_notes_locally(workspace, cx);
        });
    })
    .detach();

    let version = release_channel::AppVersion::global(cx);
    let auto_updater = cx.new_model(|cx| {
        let updater = AutoUpdater::new(version, http_client);

        let mut update_subscription = AutoUpdateSetting::get_global(cx)
            .0
            .then(|| updater.start_polling(cx));

        cx.observe_global::<SettingsStore>(move |updater, cx| {
            if AutoUpdateSetting::get_global(cx).0 {
                if update_subscription.is_none() {
                    update_subscription = Some(updater.start_polling(cx))
                }
            } else {
                update_subscription.take();
            }
        })
        .detach();

        updater
    });
    cx.set_global(GlobalAutoUpdate(Some(auto_updater)));
}

pub fn check(_: &Check, cx: &mut WindowContext) {
    if let Some(updater) = AutoUpdater::get(cx) {
        updater.update(cx, |updater, cx| updater.poll(cx));
    } else {
        drop(cx.prompt(
            gpui::PromptLevel::Info,
            "Could not check for updates",
            Some("Auto-updates disabled for non-bundled app."),
            &["Ok"],
        ));
    }
}

pub fn view_release_notes(_: &ViewReleaseNotes, cx: &mut AppContext) -> Option<()> {
    let auto_updater = AutoUpdater::get(cx)?;
    let release_channel = ReleaseChannel::try_global(cx)?;

    if matches!(
        release_channel,
        ReleaseChannel::Stable | ReleaseChannel::Preview
    ) {
        let auto_updater = auto_updater.read(cx);
        let release_channel = release_channel.dev_name();
        let current_version = auto_updater.current_version;
        let url = &auto_updater
            .http_client
            .build_url(&format!("/releases/{release_channel}/{current_version}"));
        cx.open_url(&url);
    }

    None
}

fn view_release_notes_locally(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    let release_channel = ReleaseChannel::global(cx);
    let version = AppVersion::global(cx).to_string();

    let client = client::Client::global(cx).http_client();
    let url = client.build_url(&format!(
        "/api/release_notes/{}/{}",
        release_channel.dev_name(),
        version
    ));

    let markdown = workspace
        .app_state()
        .languages
        .language_for_name("Markdown");

    workspace
        .with_local_workspace(cx, move |_, cx| {
            cx.spawn(|workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                let response = client.get(&url, Default::default(), true).await;
                let Some(mut response) = response.log_err() else {
                    return;
                };

                let mut body = Vec::new();
                response.body_mut().read_to_end(&mut body).await.ok();

                let body: serde_json::Result<ReleaseNotesBody> =
                    serde_json::from_slice(body.as_slice());

                if let Ok(body) = body {
                    workspace
                        .update(&mut cx, |workspace, cx| {
                            let project = workspace.project().clone();
                            let buffer = project.update(cx, |project, cx| {
                                project.create_local_buffer("", markdown, cx)
                            });
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(0..0, body.release_notes)], None, cx)
                            });
                            let language_registry = project.read(cx).languages().clone();

                            let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));

                            let tab_description = SharedString::from(body.title.to_string());
                            let editor = cx.new_view(|cx| {
                                Editor::for_multibuffer(buffer, Some(project), true, cx)
                            });
                            let workspace_handle = workspace.weak_handle();
                            let view: View<MarkdownPreviewView> = MarkdownPreviewView::new(
                                MarkdownPreviewMode::Default,
                                editor,
                                workspace_handle,
                                language_registry,
                                Some(tab_description),
                                cx,
                            );
                            workspace.add_item_to_active_pane(Box::new(view.clone()), None, cx);
                            cx.notify();
                        })
                        .log_err();
                }
            })
            .detach();
        })
        .detach();
}

pub fn notify_of_any_new_update(cx: &mut ViewContext<Workspace>) -> Option<()> {
    let updater = AutoUpdater::get(cx)?;
    let version = updater.read(cx).current_version;
    let should_show_notification = updater.read(cx).should_show_update_notification(cx);

    cx.spawn(|workspace, mut cx| async move {
        let should_show_notification = should_show_notification.await?;
        if should_show_notification {
            workspace.update(&mut cx, |workspace, cx| {
                workspace.show_notification(
                    NotificationId::unique::<UpdateNotification>(),
                    cx,
                    |cx| cx.new_view(|_| UpdateNotification::new(version)),
                );
                updater
                    .read(cx)
                    .set_should_show_update_notification(false, cx)
                    .detach_and_log_err(cx);
            })?;
        }
        anyhow::Ok(())
    })
    .detach();

    None
}

impl AutoUpdater {
    pub fn get(cx: &mut AppContext) -> Option<Model<Self>> {
        cx.default_global::<GlobalAutoUpdate>().0.clone()
    }

    fn new(current_version: SemanticVersion, http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            status: AutoUpdateStatus::Idle,
            current_version,
            http_client,
            pending_poll: None,
        }
    }

    pub fn start_polling(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        cx.spawn(|this, mut cx| async move {
            loop {
                this.update(&mut cx, |this, cx| this.poll(cx))?;
                cx.background_executor().timer(POLL_INTERVAL).await;
            }
        })
    }

    pub fn poll(&mut self, cx: &mut ModelContext<Self>) {
        if self.pending_poll.is_some() || self.status.is_updated() {
            return;
        }

        self.status = AutoUpdateStatus::Checking;
        cx.notify();

        self.pending_poll = Some(cx.spawn(|this, mut cx| async move {
            let result = Self::update(this.upgrade()?, cx.clone()).await;
            this.update(&mut cx, |this, cx| {
                this.pending_poll = None;
                if let Err(error) = result {
                    log::error!("auto-update failed: error:{:?}", error);
                    this.status = AutoUpdateStatus::Errored;
                    cx.notify();
                }
            })
            .ok()
        }));
    }

    pub fn status(&self) -> AutoUpdateStatus {
        self.status.clone()
    }

    pub fn dismiss_error(&mut self, cx: &mut ModelContext<Self>) {
        self.status = AutoUpdateStatus::Idle;
        cx.notify();
    }

    async fn update(this: Model<Self>, mut cx: AsyncAppContext) -> Result<()> {
        // Skip auto-update for flatpaks
        #[cfg(target_os = "linux")]
        if matches!(std::env::var("ZED_IS_FLATPAK_INSTALL"), Ok(_)) {
            this.update(&mut cx, |this, cx| {
                this.status = AutoUpdateStatus::Idle;
                cx.notify();
            })?;
            return Ok(());
        }

        let (client, current_version) = this.read_with(&cx, |this, _| {
            (this.http_client.clone(), this.current_version)
        })?;

        let asset = match OS {
            "linux" => format!("zed-linux-{}.tar.gz", ARCH),
            "macos" => "Zed.dmg".into(),
            _ => return Err(anyhow!("auto-update not supported for OS {:?}", OS)),
        };

        let mut url_string = client.build_url(&format!(
            "/api/releases/latest?asset={}&os={}&arch={}",
            asset, OS, ARCH
        ));
        cx.update(|cx| {
            if let Some(param) = ReleaseChannel::try_global(cx)
                .and_then(|release_channel| release_channel.release_query_param())
            {
                url_string += "&";
                url_string += param;
            }
        })?;

        let mut response = client.get(&url_string, Default::default(), true).await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading release")?;

        let release: JsonRelease =
            serde_json::from_slice(body.as_slice()).context("error deserializing release")?;

        let should_download = match *RELEASE_CHANNEL {
            ReleaseChannel::Nightly => cx
                .update(|cx| AppCommitSha::try_global(cx).map(|sha| release.version != sha.0))
                .ok()
                .flatten()
                .unwrap_or(true),
            _ => release.version.parse::<SemanticVersion>()? > current_version,
        };

        if !should_download {
            this.update(&mut cx, |this, cx| {
                this.status = AutoUpdateStatus::Idle;
                cx.notify();
            })?;
            return Ok(());
        }

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Downloading;
            cx.notify();
        })?;

        let temp_dir = tempfile::Builder::new()
            .prefix("zed-auto-update")
            .tempdir()?;
        let downloaded_asset = download_release(&temp_dir, release, &asset, client, &cx).await?;

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Installing;
            cx.notify();
        })?;

        // We store the path of our current binary, before we install, since installation might
        // delete it. Once deleted, it's hard to get the path to our binary on Linux.
        // So we cache it here, which allows us to then restart later on.
        let binary_path = cx.update(|cx| cx.app_path())??;

        match OS {
            "macos" => install_release_macos(&temp_dir, downloaded_asset, &cx).await,
            "linux" => install_release_linux(&temp_dir, downloaded_asset, &cx).await,
            _ => Err(anyhow!("not supported: {:?}", OS)),
        }?;

        this.update(&mut cx, |this, cx| {
            this.set_should_show_update_notification(true, cx)
                .detach_and_log_err(cx);
            this.status = AutoUpdateStatus::Updated { binary_path };
            cx.notify();
        })?;

        Ok(())
    }

    fn set_should_show_update_notification(
        &self,
        should_show: bool,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        cx.background_executor().spawn(async move {
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
        cx.background_executor().spawn(async move {
            Ok(KEY_VALUE_STORE
                .read_kvp(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY)?
                .is_some())
        })
    }
}

async fn download_release(
    temp_dir: &tempfile::TempDir,
    release: JsonRelease,
    target_filename: &str,
    client: Arc<HttpClientWithUrl>,
    cx: &AsyncAppContext,
) -> Result<PathBuf> {
    let target_path = temp_dir.path().join(target_filename);
    let mut target_file = File::create(&target_path).await?;

    let (installation_id, release_channel, telemetry) = cx.update(|cx| {
        let installation_id = Client::global(cx).telemetry().installation_id();
        let release_channel =
            ReleaseChannel::try_global(cx).map(|release_channel| release_channel.display_name());
        let telemetry = TelemetrySettings::get_global(cx).metrics;

        (installation_id, release_channel, telemetry)
    })?;

    let request_body = AsyncBody::from(serde_json::to_string(&UpdateRequestBody {
        installation_id,
        release_channel,
        telemetry,
    })?);

    let mut response = client.get(&release.url, request_body, true).await?;
    smol::io::copy(response.body_mut(), &mut target_file).await?;
    log::info!("downloaded update. path:{:?}", target_path);

    Ok(target_path)
}

async fn install_release_linux(
    temp_dir: &tempfile::TempDir,
    downloaded_tar_gz: PathBuf,
    cx: &AsyncAppContext,
) -> Result<()> {
    let channel = cx.update(|cx| ReleaseChannel::global(cx).dev_name())?;
    let home_dir = PathBuf::from(std::env::var("HOME").context("no HOME env var set")?);

    let extracted = temp_dir.path().join("zed");
    fs::create_dir_all(&extracted)
        .await
        .context("failed to create directory into which to extract update")?;

    let output = Command::new("tar")
        .arg("-xzf")
        .arg(&downloaded_tar_gz)
        .arg("-C")
        .arg(&extracted)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to extract {:?} to {:?}: {:?}",
        downloaded_tar_gz,
        extracted,
        String::from_utf8_lossy(&output.stderr)
    );

    let suffix = if channel != "stable" {
        format!("-{}", channel)
    } else {
        String::default()
    };
    let app_folder_name = format!("zed{}.app", suffix);

    let from = extracted.join(&app_folder_name);
    let to = home_dir.join(".local");

    let output = Command::new("rsync")
        .args(&["-av", "--delete"])
        .arg(&from)
        .arg(&to)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy Zed update from {:?} to {:?}: {:?}",
        from,
        to,
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

async fn install_release_macos(
    temp_dir: &tempfile::TempDir,
    downloaded_dmg: PathBuf,
    cx: &AsyncAppContext,
) -> Result<()> {
    let running_app_path = ZED_APP_PATH
        .clone()
        .map_or_else(|| cx.update(|cx| cx.app_path())?, Ok)?;
    let running_app_filename = running_app_path
        .file_name()
        .ok_or_else(|| anyhow!("invalid running app path"))?;

    let mount_path = temp_dir.path().join("Zed");
    let mut mounted_app_path: OsString = mount_path.join(running_app_filename).into();

    mounted_app_path.push("/");
    let output = Command::new("hdiutil")
        .args(&["attach", "-nobrowse"])
        .arg(&downloaded_dmg)
        .arg("-mountroot")
        .arg(&temp_dir.path())
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to mount: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = Command::new("rsync")
        .args(&["-av", "--delete"])
        .arg(&mounted_app_path)
        .arg(&running_app_path)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy app: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = Command::new("hdiutil")
        .args(&["detach"])
        .arg(&mount_path)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to unount: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}
