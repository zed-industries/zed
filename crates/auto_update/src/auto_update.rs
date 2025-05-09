use anyhow::{Context as _, Result, anyhow};
use client::{Client, TelemetrySettings};
use db::RELEASE_CHANNEL;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, Global, SemanticVersion, Task, Window, actions,
};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl};
use paths::remote_servers_dir;
use release_channel::{AppCommitSha, ReleaseChannel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use smol::{fs, io::AsyncReadExt};
use smol::{fs::File, process::Command};
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
use workspace::Workspace;

const SHOULD_SHOW_UPDATE_NOTIFICATION_KEY: &str = "auto-updater-should-show-updated-notification";
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

actions!(auto_update, [Check, DismissErrorMessage, ViewReleaseNotes,]);

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

#[derive(Deserialize, Debug)]
pub struct JsonRelease {
    pub version: String,
    pub url: String,
}

struct MacOsUnmounter {
    mount_path: PathBuf,
}

impl Drop for MacOsUnmounter {
    fn drop(&mut self) {
        let unmount_output = std::process::Command::new("hdiutil")
            .args(["detach", "-force"])
            .arg(&self.mount_path)
            .output();

        match unmount_output {
            Ok(output) if output.status.success() => {
                log::info!("Successfully unmounted the disk image");
            }
            Ok(output) => {
                log::error!(
                    "Failed to unmount disk image: {:?}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(error) => {
                log::error!("Error while trying to unmount disk image: {:?}", error);
            }
        }
    }
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

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let auto_update = [sources.server, sources.release_channel, sources.user]
            .into_iter()
            .find_map(|value| value.copied().flatten())
            .unwrap_or(sources.default.ok_or_else(Self::missing_default)?);

        Ok(Self(auto_update.0))
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        vscode.enum_setting("update.mode", current, |s| match s {
            "none" | "manual" => Some(AutoUpdateSettingContent(false)),
            _ => Some(AutoUpdateSettingContent(true)),
        });
    }
}

#[derive(Default)]
struct GlobalAutoUpdate(Option<Entity<AutoUpdater>>);

impl Global for GlobalAutoUpdate {}

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut App) {
    AutoUpdateSetting::register(cx);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, action: &Check, window, cx| check(action, window, cx));

        workspace.register_action(|_, action, _, cx| {
            view_release_notes(action, cx);
        });
    })
    .detach();

    let version = release_channel::AppVersion::global(cx);
    let auto_updater = cx.new(|cx| {
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

            cx.observe_global::<SettingsStore>(move |updater: &mut AutoUpdater, cx| {
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

pub fn check(_: &Check, window: &mut Window, cx: &mut App) {
    if let Some(message) = option_env!("ZED_UPDATE_EXPLANATION") {
        drop(window.prompt(
            gpui::PromptLevel::Info,
            "Zed was installed via a package manager.",
            Some(message),
            &["Ok"],
            cx,
        ));
        return;
    }

    if let Ok(message) = env::var("ZED_UPDATE_EXPLANATION") {
        drop(window.prompt(
            gpui::PromptLevel::Info,
            "Zed was installed via a package manager.",
            Some(&message),
            &["Ok"],
            cx,
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
        drop(window.prompt(
            gpui::PromptLevel::Info,
            "Could not check for updates",
            Some("Auto-updates disabled for non-bundled app."),
            &["Ok"],
            cx,
        ));
    }
}

pub fn view_release_notes(_: &ViewReleaseNotes, cx: &mut App) -> Option<()> {
    let auto_updater = AutoUpdater::get(cx)?;
    let release_channel = ReleaseChannel::try_global(cx)?;

    match release_channel {
        ReleaseChannel::Stable | ReleaseChannel::Preview => {
            let auto_updater = auto_updater.read(cx);
            let current_version = auto_updater.current_version;
            let release_channel = release_channel.dev_name();
            let path = format!("/releases/{release_channel}/{current_version}");
            let url = &auto_updater.http_client.build_url(&path);
            cx.open_url(url);
        }
        ReleaseChannel::Nightly => {
            cx.open_url("https://github.com/zed-industries/zed/commits/nightly/");
        }
        ReleaseChannel::Dev => {
            cx.open_url("https://github.com/zed-industries/zed/commits/main/");
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
struct InstallerDir(tempfile::TempDir);

#[cfg(not(target_os = "windows"))]
impl InstallerDir {
    async fn new() -> Result<Self> {
        Ok(Self(
            tempfile::Builder::new()
                .prefix("zed-auto-update")
                .tempdir()?,
        ))
    }

    fn path(&self) -> &Path {
        self.0.path()
    }
}

#[cfg(target_os = "windows")]
struct InstallerDir(PathBuf);

#[cfg(target_os = "windows")]
impl InstallerDir {
    async fn new() -> Result<Self> {
        let installer_dir = std::env::current_exe()?
            .parent()
            .context("No parent dir for Zed.exe")?
            .join("updates");
        if smol::fs::metadata(&installer_dir).await.is_ok() {
            smol::fs::remove_dir_all(&installer_dir).await?;
        }
        smol::fs::create_dir(&installer_dir).await?;
        Ok(Self(installer_dir))
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }
}

impl AutoUpdater {
    pub fn get(cx: &mut App) -> Option<Entity<Self>> {
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

    pub fn start_polling(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| {
            loop {
                this.update(cx, |this, cx| this.poll(cx))?;
                cx.background_executor().timer(POLL_INTERVAL).await;
            }
        })
    }

    pub fn poll(&mut self, cx: &mut Context<Self>) {
        if self.pending_poll.is_some() || self.status.is_updated() {
            return;
        }

        cx.notify();

        self.pending_poll = Some(cx.spawn(async move |this, cx| {
            let result = Self::update(this.upgrade()?, cx.clone()).await;
            this.update(cx, |this, cx| {
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

    pub fn current_version(&self) -> SemanticVersion {
        self.current_version
    }

    pub fn status(&self) -> AutoUpdateStatus {
        self.status.clone()
    }

    pub fn dismiss_error(&mut self, cx: &mut Context<Self>) -> bool {
        if self.status == AutoUpdateStatus::Idle {
            return false;
        }
        self.status = AutoUpdateStatus::Idle;
        cx.notify();
        true
    }

    // If you are packaging Zed and need to override the place it downloads SSH remotes from,
    // you can override this function. You should also update get_remote_server_release_url to return
    // Ok(None).
    pub async fn download_remote_server_release(
        os: &str,
        arch: &str,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        let this = cx.update(|cx| {
            cx.default_global::<GlobalAutoUpdate>()
                .0
                .clone()
                .ok_or_else(|| anyhow!("auto-update not initialized"))
        })??;

        let release = Self::get_release(
            &this,
            "zed-remote-server",
            os,
            arch,
            version,
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
            log::info!(
                "downloading zed-remote-server {os} {arch} version {}",
                release.version
            );
            download_remote_server_binary(&version_path, release, client, cx).await?;
        }

        Ok(version_path)
    }

    pub async fn get_remote_server_release_url(
        os: &str,
        arch: &str,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Result<Option<(String, String)>> {
        let this = cx.update(|cx| {
            cx.default_global::<GlobalAutoUpdate>()
                .0
                .clone()
                .ok_or_else(|| anyhow!("auto-update not initialized"))
        })??;

        let release = Self::get_release(
            &this,
            "zed-remote-server",
            os,
            arch,
            version,
            Some(release_channel),
            cx,
        )
        .await?;

        let update_request_body = build_remote_server_update_request_body(cx)?;
        let body = serde_json::to_string(&update_request_body)?;

        Ok(Some((release.url, body)))
    }

    async fn get_release(
        this: &Entity<Self>,
        asset: &str,
        os: &str,
        arch: &str,
        version: Option<SemanticVersion>,
        release_channel: Option<ReleaseChannel>,
        cx: &mut AsyncApp,
    ) -> Result<JsonRelease> {
        let client = this.read_with(cx, |this, _| this.http_client.clone())?;

        if let Some(version) = version {
            let channel = release_channel.map(|c| c.dev_name()).unwrap_or("stable");

            let url = format!("/api/releases/{channel}/{version}/{asset}-{os}-{arch}.gz?update=1",);

            Ok(JsonRelease {
                version: version.to_string(),
                url: client.build_url(&url),
            })
        } else {
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
            response.body_mut().read_to_end(&mut body).await?;

            if !response.status().is_success() {
                return Err(anyhow!(
                    "failed to fetch release: {:?}",
                    String::from_utf8_lossy(&body),
                ));
            }

            serde_json::from_slice(body.as_slice()).with_context(|| {
                format!(
                    "error deserializing release {:?}",
                    String::from_utf8_lossy(&body),
                )
            })
        }
    }

    async fn get_latest_release(
        this: &Entity<Self>,
        asset: &str,
        os: &str,
        arch: &str,
        release_channel: Option<ReleaseChannel>,
        cx: &mut AsyncApp,
    ) -> Result<JsonRelease> {
        Self::get_release(this, asset, os, arch, None, release_channel, cx).await
    }

    async fn update(this: Entity<Self>, mut cx: AsyncApp) -> Result<()> {
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

        let installer_dir = InstallerDir::new().await?;
        let filename = match OS {
            "macos" => Ok("Zed.dmg"),
            "linux" => Ok("zed.tar.gz"),
            "windows" => Ok("ZedUpdateInstaller.exe"),
            _ => Err(anyhow!("not supported: {:?}", OS)),
        }?;

        #[cfg(not(target_os = "windows"))]
        anyhow::ensure!(
            which::which("rsync").is_ok(),
            "Aborting. Could not find rsync which is required for auto-updates."
        );

        let downloaded_asset = installer_dir.path().join(filename);
        download_release(&downloaded_asset, release, client, &cx).await?;

        this.update(&mut cx, |this, cx| {
            this.status = AutoUpdateStatus::Installing;
            cx.notify();
        })?;

        let binary_path = match OS {
            "macos" => install_release_macos(&installer_dir, downloaded_asset, &cx).await,
            "linux" => install_release_linux(&installer_dir, downloaded_asset, &cx).await,
            "windows" => install_release_windows(downloaded_asset).await,
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

    pub fn set_should_show_update_notification(
        &self,
        should_show: bool,
        cx: &App,
    ) -> Task<Result<()>> {
        cx.background_spawn(async move {
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

    pub fn should_show_update_notification(&self, cx: &App) -> Task<Result<bool>> {
        cx.background_spawn(async move {
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
    cx: &AsyncApp,
) -> Result<()> {
    let temp = tempfile::Builder::new().tempfile_in(remote_servers_dir())?;
    let mut temp_file = File::create(&temp).await?;
    let update_request_body = build_remote_server_update_request_body(cx)?;
    let request_body = AsyncBody::from(serde_json::to_string(&update_request_body)?);

    let mut response = client.get(&release.url, request_body, true).await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "failed to download remote server release: {:?}",
            response.status()
        ));
    }
    smol::io::copy(response.body_mut(), &mut temp_file).await?;
    smol::fs::rename(&temp, &target_path).await?;

    Ok(())
}

fn build_remote_server_update_request_body(cx: &AsyncApp) -> Result<UpdateRequestBody> {
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

    Ok(UpdateRequestBody {
        installation_id,
        release_channel,
        telemetry: telemetry_enabled,
        is_staff,
        destination: "remote",
    })
}

async fn download_release(
    target_path: &Path,
    release: JsonRelease,
    client: Arc<HttpClientWithUrl>,
    cx: &AsyncApp,
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
    temp_dir: &InstallerDir,
    downloaded_tar_gz: PathBuf,
    cx: &AsyncApp,
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
        .args(["-av", "--delete"])
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
    temp_dir: &InstallerDir,
    downloaded_dmg: PathBuf,
    cx: &AsyncApp,
) -> Result<PathBuf> {
    let running_app_path = cx.update(|cx| cx.app_path())??;
    let running_app_filename = running_app_path
        .file_name()
        .ok_or_else(|| anyhow!("invalid running app path"))?;

    let mount_path = temp_dir.path().join("Zed");
    let mut mounted_app_path: OsString = mount_path.join(running_app_filename).into();

    mounted_app_path.push("/");
    let output = Command::new("hdiutil")
        .args(["attach", "-nobrowse"])
        .arg(&downloaded_dmg)
        .arg("-mountroot")
        .arg(temp_dir.path())
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to mount: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Create an MacOsUnmounter that will be dropped (and thus unmount the disk) when this function exits
    let _unmounter = MacOsUnmounter {
        mount_path: mount_path.clone(),
    };

    let output = Command::new("rsync")
        .args(["-av", "--delete"])
        .arg(&mounted_app_path)
        .arg(&running_app_path)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy app: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(running_app_path)
}

async fn install_release_windows(downloaded_installer: PathBuf) -> Result<PathBuf> {
    let output = Command::new(downloaded_installer)
        .arg("/verysilent")
        .arg("/update=true")
        .arg("!desktopicon")
        .arg("!quicklaunchicon")
        .output()
        .await?;
    anyhow::ensure!(
        output.status.success(),
        "failed to start installer: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(std::env::current_exe()?)
}

pub fn check_pending_installation() -> bool {
    let Some(installer_path) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("updates")))
    else {
        return false;
    };

    // The installer will create a flag file after it finishes updating
    let flag_file = installer_path.join("versions.txt");
    if flag_file.exists() {
        if let Some(helper) = installer_path
            .parent()
            .map(|p| p.join("tools\\auto_update_helper.exe"))
        {
            let _ = std::process::Command::new(helper).spawn();
            return true;
        }
    }
    false
}
