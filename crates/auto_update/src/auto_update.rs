mod update_notification;

use anyhow::{anyhow, Context, Result};
use client::{Client, TelemetrySettings};
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

use http_client::{HttpClient, HttpClientWithUrl};
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use std::{
    env::{
        self,
        consts::{ARCH, OS},
    },
    ffi::OsString,
    path::{Path, PathBuf},
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
    is_staff: Option<bool>,
    destination: &'static str,
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

        let poll_for_updates = ReleaseChannel::try_global(cx)
            .map(|channel| channel.poll_for_updates())
            .unwrap_or(false);

        if option_env!("ZED_UPDATE_EXPLANATION").is_none()
            && env::var("ZED_UPDATE_EXPLANATION").is_err()
            && poll_for_updates
        {
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
        }

        updater
    });
    cx.set_global(GlobalAutoUpdate(Some(auto_updater)));
}

pub fn check(_: &Check, cx: &mut WindowContext) {
    if let Some(message) = option_env!("ZED_UPDATE_EXPLANATION") {
        drop(cx.prompt(
            gpui::PromptLevel::Info,
            "Zed was installed via a package manager.",
            Some(message),
            &["Ok"],
        ));
        return;
    }

    if let Some(message) = env::var("ZED_UPDATE_EXPLANATION").ok() {
        drop(cx.prompt(
            gpui::PromptLevel::Info,
            "Zed was installed via a package manager.",
            Some(&message),
            &["Ok"],
        ));
        return;
    }

    if !ReleaseChannel::try_global(cx)
        .map(|channel| channel.poll_for_updates())
        .unwrap_or(false)
    {
        return;
    }

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
                            workspace.add_item_to_active_pane(
                                Box::new(view.clone()),
                                None,
                                true,
                                cx,
                            );
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

    pub async fn get_latest_remote_server_release(
        os: &str,
        arch: &str,
        mut release_channel: ReleaseChannel,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf> {
        let this = cx.update(|cx| {
            cx.default_global::<GlobalAutoUpdate>()
                .0
                .clone()
                .ok_or_else(|| anyhow!("auto-update not initialized"))
        })??;

        if release_channel == ReleaseChannel::Dev {
            release_channel = ReleaseChannel::Nightly;
        }

        let release = Self::get_latest_release(
            &this,
            "zed-remote-server",
            os,
            arch,
            Some(release_channel),
            cx,
        )
        .await?;

        let servers_dir = paths::remote_servers_dir();
        let channel_dir = servers_dir.join(release_channel.dev_name());
        let platform_dir = channel_dir.join(format!("{}-{}", os, arch));
        let version_path = platform_dir.join(format!("{}.gz", release.version));
        smol::fs::create_dir_all(&platform_dir).await.ok();

        let client = this.read_with(cx, |this, _| this.http_client.clone())?;
        if smol::fs::metadata(&version_path).await.is_err() {
            log::info!("downloading zed-remote-server {os} {arch}");
            download_remote_server_binary(&version_path, release, client, cx).await?;
        }

        Ok(version_path)
    }

    async fn get_latest_release(
        this: &Model<Self>,
        asset: &str,
        os: &str,
        arch: &str,
        release_channel: Option<ReleaseChannel>,
        cx: &mut AsyncAppContext,
    ) -> Result<JsonRelease> {
        let client = this.read_with(cx, |this, _| this.http_client.clone())?;
        let mut url_string = client.build_url(&format!(
            "/api/releases/latest?asset={}&os={}&arch={}",
            asset, os, arch
        ));
        if let Some(param) = release_channel.and_then(|c| c.release_query_param()) {
            url_string += "&";
            url_string += param;
        }

        let mut response = client.get(&url_string, Default::default(), true).await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading release")?;

        if !response.status().is_success() {
            Err(anyhow!(
                "failed to fetch release: {:?}",
                String::from_utf8_lossy(&body),
            ))?;
        }

        serde_json::from_slice(body.as_slice()).with_context(|| {
            format!(
                "error deserializing release {:?}",
                String::from_utf8_lossy(&body),
            )
        })
    }

    async fn update(this: Model<Self>, mut cx: AsyncAppContext) -> Result<()> {
        let (client, current_version, release_channel) = this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Checking;
            cx.notify();
            (
                this.http_client.clone(),
                this.current_version,
                ReleaseChannel::try_global(cx),
            )
        })?;

        let release =
            Self::get_latest_release(&this, "zed", OS, ARCH, release_channel, &mut cx).await?;

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

        let filename = match OS {
            "macos" => Ok("Zed.dmg"),
            "linux" => Ok("zed.tar.gz"),
            _ => Err(anyhow!("not supported: {:?}", OS)),
        }?;
        let downloaded_asset = temp_dir.path().join(filename);
        download_release(&downloaded_asset, release, client, &cx).await?;

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Installing;
            cx.notify();
        })?;

        let binary_path = match OS {
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

async fn download_remote_server_binary(
    target_path: &PathBuf,
    release: JsonRelease,
    client: Arc<HttpClientWithUrl>,
    cx: &AsyncAppContext,
) -> Result<()> {
    let mut target_file = File::create(&target_path).await?;
    let (installation_id, release_channel, telemetry_enabled, is_staff) = cx.update(|cx| {
        let telemetry = Client::global(cx).telemetry().clone();
        let is_staff = telemetry.is_staff();
        let installation_id = telemetry.installation_id();
        let release_channel =
            ReleaseChannel::try_global(cx).map(|release_channel| release_channel.display_name());
        let telemetry_enabled = TelemetrySettings::get_global(cx).metrics;

        (
            installation_id,
            release_channel,
            telemetry_enabled,
            is_staff,
        )
    })?;
    let request_body = AsyncBody::from(serde_json::to_string(&UpdateRequestBody {
        installation_id,
        release_channel,
        telemetry: telemetry_enabled,
        is_staff,
        destination: "remote",
    })?);

    let mut response = client.get(&release.url, request_body, true).await?;
    smol::io::copy(response.body_mut(), &mut target_file).await?;
    Ok(())
}

async fn download_release(
    target_path: &Path,
    release: JsonRelease,
    client: Arc<HttpClientWithUrl>,
    cx: &AsyncAppContext,
) -> Result<()> {
    let mut target_file = File::create(&target_path).await?;

    let (installation_id, release_channel, telemetry_enabled, is_staff) = cx.update(|cx| {
        let telemetry = Client::global(cx).telemetry().clone();
        let is_staff = telemetry.is_staff();
        let installation_id = telemetry.installation_id();
        let release_channel =
            ReleaseChannel::try_global(cx).map(|release_channel| release_channel.display_name());
        let telemetry_enabled = TelemetrySettings::get_global(cx).metrics;

        (
            installation_id,
            release_channel,
            telemetry_enabled,
            is_staff,
        )
    })?;

    let request_body = AsyncBody::from(serde_json::to_string(&UpdateRequestBody {
        installation_id,
        release_channel,
        telemetry: telemetry_enabled,
        is_staff,
        destination: "local",
    })?);

    let mut response = client.get(&release.url, request_body, true).await?;
    smol::io::copy(response.body_mut(), &mut target_file).await?;
    log::info!("downloaded update. path:{:?}", target_path);

    Ok(())
}

async fn install_release_linux(
    temp_dir: &tempfile::TempDir,
    downloaded_tar_gz: PathBuf,
    cx: &AsyncAppContext,
) -> Result<PathBuf> {
    let channel = cx.update(|cx| ReleaseChannel::global(cx).dev_name())?;
    let home_dir = PathBuf::from(env::var("HOME").context("no HOME env var set")?);
    let running_app_path = cx.update(|cx| cx.app_path())??;

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
    let mut to = home_dir.join(".local");

    let expected_suffix = format!("{}/libexec/zed-editor", app_folder_name);

    if let Some(prefix) = running_app_path
        .to_str()
        .and_then(|str| str.strip_suffix(&expected_suffix))
    {
        to = PathBuf::from(prefix);
    }

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

    Ok(to.join(expected_suffix))
}

async fn install_release_macos(
    temp_dir: &tempfile::TempDir,
    downloaded_dmg: PathBuf,
    cx: &AsyncAppContext,
) -> Result<PathBuf> {
    let running_app_path = cx.update(|cx| cx.app_path())??;
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

    Ok(running_app_path)
}
