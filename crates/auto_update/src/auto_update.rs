use anyhow::{Context as _, Result};
use askpass::{AskPassDelegate, AskPassSession, EncryptedPassword};
use client::Client;
use db::kvp::KeyValueStore;
use futures::channel::oneshot;
use futures_lite::StreamExt;
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, Global,
    Task, TaskExt, Window, actions,
};
use http_client::{HttpClient, HttpClientWithUrl};
use paths::remote_servers_dir;
use release_channel::{AppCommitSha, ReleaseChannel};
use semver::Version;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, Settings, SettingsStore};
use smol::fs::File;
use smol::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};
use std::mem;
use std::{
    env::{
        self,
        consts::{ARCH, OS},
    },
    ffi::OsStr,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use util::command::new_command;
use workspace::Workspace;

const SHOULD_SHOW_UPDATE_NOTIFICATION_KEY: &str = "auto-updater-should-show-updated-notification";

#[derive(Debug)]
struct MissingDependencyError(String);

impl std::fmt::Display for MissingDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MissingDependencyError {}
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);
const NIGHTLY_POLL_INTERVAL: Duration = Duration::from_secs(15 * 60);
const REMOTE_SERVER_CACHE_LIMIT: usize = 5;

/// GitHub repository whose releases this fork updates from. Its stable builds
/// must not be replaced by the official zed.dev releases.
const FORK_RELEASE_REPOSITORY: &str = match option_env!("ZED_FORK_RELEASE_REPOSITORY") {
    Some(repository) => repository,
    None => "belizariogr/zed",
};

#[cfg(target_os = "linux")]
fn linux_rsync_install_hint() -> &'static str {
    let os_release = match std::fs::read_to_string("/etc/os-release") {
        Ok(os_release) => os_release,
        Err(_) => return "Please install rsync using your package manager",
    };

    let mut distribution_ids = Vec::new();
    for line in os_release.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("ID=") {
            distribution_ids.push(value.trim_matches('"').to_ascii_lowercase());
        } else if let Some(value) = trimmed.strip_prefix("ID_LIKE=") {
            for id in value.trim_matches('"').split_whitespace() {
                distribution_ids.push(id.to_ascii_lowercase());
            }
        }
    }

    let package_manager_hint = if distribution_ids
        .iter()
        .any(|distribution_id| distribution_id == "arch")
    {
        Some("Install it with: sudo pacman -S rsync")
    } else if distribution_ids
        .iter()
        .any(|distribution_id| distribution_id == "debian" || distribution_id == "ubuntu")
    {
        Some("Install it with: sudo apt install rsync")
    } else if distribution_ids.iter().any(|distribution_id| {
        distribution_id == "fedora"
            || distribution_id == "rhel"
            || distribution_id == "centos"
            || distribution_id == "rocky"
            || distribution_id == "almalinux"
    }) {
        Some("Install it with: sudo dnf install rsync")
    } else if distribution_ids
        .iter()
        .any(|distribution_id| distribution_id == "nixos")
    {
        Some("Install pkgs.rsync from nixpkgs")
    } else {
        None
    };

    package_manager_hint.unwrap_or("Please install rsync using your package manager")
}

actions!(
    auto_update,
    [
        /// Checks for available updates.
        Check,
        /// Dismisses the update error message.
        DismissMessage,
        /// Opens the release notes for the current version in a browser.
        ViewReleaseNotes,
    ]
);

#[derive(Clone, Debug)]
pub enum AutoUpdateStatus {
    Idle,
    Checking,
    Downloading {
        version: Version,
        /// Download progress as a fraction in the range `0.0..=1.0`, or `None`
        /// when the total download size is not yet known.
        progress: Option<f32>,
    },
    Installing {
        version: Version,
    },
    Updated {
        version: Version,
    },
    Errored {
        error: Arc<anyhow::Error>,
    },
}

impl PartialEq for AutoUpdateStatus {
    // `progress` is deliberately not compared: two `Downloading` statuses for
    // the same version are equal regardless of how far the download is.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AutoUpdateStatus::Idle, AutoUpdateStatus::Idle) => true,
            (AutoUpdateStatus::Checking, AutoUpdateStatus::Checking) => true,
            (
                AutoUpdateStatus::Downloading { version: v1, .. },
                AutoUpdateStatus::Downloading { version: v2, .. },
            ) => v1 == v2,
            (
                AutoUpdateStatus::Installing { version: v1 },
                AutoUpdateStatus::Installing { version: v2 },
            ) => v1 == v2,
            (
                AutoUpdateStatus::Updated { version: v1 },
                AutoUpdateStatus::Updated { version: v2 },
            ) => v1 == v2,
            (AutoUpdateStatus::Errored { error: e1 }, AutoUpdateStatus::Errored { error: e2 }) => {
                e1.to_string() == e2.to_string()
            }
            _ => false,
        }
    }
}

impl AutoUpdateStatus {
    pub fn is_updated(&self) -> bool {
        matches!(self, Self::Updated { .. })
    }
}

pub enum AutoUpdateEvent {
    /// A manual check received a release response and found no newer version.
    UpToDate,
}

pub struct AutoUpdater {
    status: AutoUpdateStatus,
    current_version: Version,
    client: Arc<Client>,
    pending_poll: Option<Task<Option<()>>>,
    quit_subscription: Option<gpui::Subscription>,
    update_check_type: UpdateCheckType,
    _wake_subscription: gpui::Subscription,
    dismissed_status: Option<AutoUpdateStatus>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReleaseAsset {
    pub version: String,
    pub url: String,
}

/// `release.json`, published with every release by `.github/workflows/fork_release.yml`.
#[derive(Deserialize, Debug)]
struct ForkReleaseManifest {
    version: String,
    tag: String,
    assets: Vec<String>,
}

fn fork_release_asset_name(os: &str, arch: &str) -> Result<String> {
    match os {
        "macos" => Ok(format!("Zed-{arch}.dmg")),
        "linux" => Ok(format!("zed-linux-{arch}.tar.gz")),
        "windows" => Ok(format!("Zed-{arch}.exe")),
        unsupported_os => anyhow::bail!("not supported: {unsupported_os}"),
    }
}

fn fork_remote_server_extension(os: &str) -> Result<&'static str> {
    match os {
        "macos" | "linux" => Ok("gz"),
        "windows" => Ok("zip"),
        unsupported_os => anyhow::bail!("not supported: {unsupported_os}"),
    }
}

fn fork_remote_server_asset_name(os: &str, arch: &str) -> Result<String> {
    let extension = fork_remote_server_extension(os)?;
    Ok(format!("zed-remote-server-{os}-{arch}.{extension}"))
}

fn fork_release_tag_for_version(version: &Version) -> String {
    format!(
        "fork-v{}.{}.{}",
        version.major, version.minor, version.patch
    )
}

/// The run number the release workflow builds with, from build metadata such
/// as `stable.7.<sha>`. Builds without one (local builds) only update to a
/// higher version.
fn fork_build_number(version: &Version) -> Option<u64> {
    version.build.as_str().split('.').nth(1)?.parse().ok()
}

struct MacOsUnmounter<'a> {
    mount_path: PathBuf,
    background_executor: &'a BackgroundExecutor,
}

impl MacOsUnmounter<'_> {
    /// Unmounts the disk image and waits for completion. This must happen
    /// before the `InstallerDir` is dropped: deleting the temp dir while the
    /// image is still mounted inside it fails silently and leaks the
    /// directory (and the downloaded DMG) in the system temp dir.
    async fn unmount(mut self) {
        let mount_path = mem::take(&mut self.mount_path);
        unmount_disk_image(&mount_path).await;
    }
}

impl Drop for MacOsUnmounter<'_> {
    fn drop(&mut self) {
        let mount_path = mem::take(&mut self.mount_path);
        // Safety net for early exits and cancellation; the happy path calls
        // `unmount`, which leaves the path empty.
        if mount_path.as_os_str().is_empty() {
            return;
        }
        self.background_executor
            .spawn(async move { unmount_disk_image(&mount_path).await })
            .detach();
    }
}

async fn unmount_disk_image(mount_path: &Path) {
    let unmount_output = new_command("hdiutil")
        .args(["detach", "-force"])
        .arg(mount_path)
        .output()
        .await;
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

#[derive(Clone, Copy, Debug, RegisterSetting)]
struct AutoUpdateSetting(bool);

/// Whether or not to automatically check for updates.
///
/// Default: true
impl Settings for AutoUpdateSetting {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self(content.auto_update.unwrap())
    }
}

#[derive(Default)]
struct GlobalAutoUpdate(Option<Entity<AutoUpdater>>);

impl Global for GlobalAutoUpdate {}

type PasswordPrompt = dyn Fn(String, oneshot::Sender<EncryptedPassword>, oneshot::Receiver<()>, &mut AsyncApp)
    + Send
    + Sync;

/// Asks for the password `sudo` needs to install an update into a Linux
/// system prefix. Registered by the UI, which owns the password modal.
struct GlobalInstallPasswordPrompt(Arc<PasswordPrompt>);

impl Global for GlobalInstallPasswordPrompt {}

pub fn set_install_password_prompt(
    cx: &mut App,
    prompt: impl Fn(String, oneshot::Sender<EncryptedPassword>, oneshot::Receiver<()>, &mut AsyncApp)
    + Send
    + Sync
    + 'static,
) {
    cx.set_global(GlobalInstallPasswordPrompt(Arc::new(prompt)));
}

pub fn init(client: Arc<Client>, cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, action, window, cx| check(action, window, cx));

        workspace.register_action(|_, action, _, cx| {
            view_release_notes(action, cx);
        });
    })
    .detach();

    let version = release_channel::AppVersion::global(cx);
    let auto_updater = cx.new(|cx| {
        let updater = AutoUpdater::new(version, client, cx);

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
    if let Some(message) = option_env!("ZED_UPDATE_EXPLANATION")
        .map(ToOwned::to_owned)
        .or_else(|| env::var("ZED_UPDATE_EXPLANATION").ok())
    {
        drop(window.prompt(
            gpui::PromptLevel::Info,
            "Zed was installed via a package manager.",
            Some(&message),
            &["OK"],
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
        updater.update(cx, |updater, cx| updater.poll(UpdateCheckType::Manual, cx));
    } else {
        drop(window.prompt(
            gpui::PromptLevel::Info,
            "Could not check for updates",
            Some("Auto-updates disabled for non-bundled app."),
            &["OK"],
            cx,
        ));
    }
}

pub fn release_notes_url(cx: &mut App) -> Option<String> {
    let release_channel = ReleaseChannel::try_global(cx)?;
    let url = match release_channel {
        ReleaseChannel::Stable | ReleaseChannel::Preview => {
            let auto_updater = AutoUpdater::get(cx)?;
            let auto_updater = auto_updater.read(cx);
            let mut current_version = auto_updater.current_version.clone();
            current_version.pre = semver::Prerelease::EMPTY;
            current_version.build = semver::BuildMetadata::EMPTY;
            let release_channel = release_channel.dev_name();
            let path = format!("/releases/{release_channel}/{current_version}");
            auto_updater.client.http_client().build_url(&path)
        }
        ReleaseChannel::Nightly => {
            "https://github.com/zed-industries/zed/commits/nightly/".to_string()
        }
        ReleaseChannel::Dev => "https://github.com/zed-industries/zed/commits/main/".to_string(),
    };
    Some(url)
}

pub fn view_release_notes(_: &ViewReleaseNotes, cx: &mut App) -> Option<()> {
    let url = release_notes_url(cx)?;
    cx.open_url(&url);
    None
}

#[cfg(not(target_os = "windows"))]
const INSTALLER_DIR_PREFIX: &str = "zed-auto-update";

#[cfg(not(target_os = "windows"))]
struct InstallerDir(tempfile::TempDir);

#[cfg(not(target_os = "windows"))]
impl InstallerDir {
    async fn new() -> Result<Self> {
        Ok(Self(
            tempfile::Builder::new()
                .prefix(INSTALLER_DIR_PREFIX)
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UpdateCheckType {
    Automatic,
    Manual,
}

impl UpdateCheckType {
    pub fn is_manual(self) -> bool {
        self == Self::Manual
    }
}

impl EventEmitter<AutoUpdateEvent> for AutoUpdater {}

impl AutoUpdater {
    pub fn get(cx: &mut App) -> Option<Entity<Self>> {
        cx.default_global::<GlobalAutoUpdate>().0.clone()
    }

    fn new(current_version: Version, client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        // On windows, executable files cannot be overwritten while they are
        // running, so we must wait to overwrite the application until quitting
        // or restarting. When quitting the app, we spawn the auto update helper
        // to finish the auto update process after Zed exits. When restarting
        // the app after an update, we use `set_restart_path` to run the auto
        // update helper instead of the app, so that it can overwrite the app
        // and then spawn the new binary.
        #[cfg(target_os = "windows")]
        let quit_subscription = Some(cx.on_app_quit(|_, _| finalize_auto_update_on_quit()));
        #[cfg(not(target_os = "windows"))]
        let quit_subscription = None;

        cx.on_app_restart(|this, _| {
            this.quit_subscription.take();
        })
        .detach();

        // A download or check that was in flight when the machine went to sleep
        // is almost certainly riding a TCP connection that silently died during
        // suspend, so it would otherwise appear to stall indefinitely.
        let wake_subscription = cx.on_system_wake({
            let this = cx.entity().downgrade();
            move |cx| {
                this.update(cx, |this, cx| this.restart_after_wake(cx)).ok();
            }
        });

        Self {
            status: AutoUpdateStatus::Idle,
            current_version,
            client,
            pending_poll: None,
            quit_subscription,
            update_check_type: UpdateCheckType::Automatic,
            _wake_subscription: wake_subscription,
            dismissed_status: None,
        }
    }

    fn restart_after_wake(&mut self, cx: &mut Context<Self>) {
        // Only network phases can be safely restarted. `Installing` is a local
        // operation (mounting a dmg, rsync, etc.) that must not be interrupted.
        if !matches!(
            self.status,
            AutoUpdateStatus::Checking | AutoUpdateStatus::Downloading { .. }
        ) {
            return;
        }

        let check_type = self.update_check_type;
        self.pending_poll.take();
        self.status = AutoUpdateStatus::Idle;
        self.poll(check_type, cx);
    }

    pub fn start_polling(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let poll_interval =
            ReleaseChannel::try_global(cx).map_or(POLL_INTERVAL, |channel| match channel {
                ReleaseChannel::Nightly => NIGHTLY_POLL_INTERVAL,
                _ => POLL_INTERVAL,
            });

        cx.spawn(async move |this, cx| {
            if cfg!(target_os = "windows") {
                use util::ResultExt;

                cleanup_windows()
                    .await
                    .context("failed to cleanup old directories")
                    .log_err();
            }

            #[cfg(all(not(target_os = "windows"), not(test)))]
            cx.background_spawn(cleanup_stale_installer_dirs()).detach();

            loop {
                this.update(cx, |this, cx| this.poll(UpdateCheckType::Automatic, cx))?;
                cx.background_executor().timer(poll_interval).await;
            }
        })
    }

    pub fn update_check_type(&self) -> UpdateCheckType {
        self.update_check_type
    }

    pub fn poll(&mut self, check_type: UpdateCheckType, cx: &mut Context<Self>) {
        if check_type.is_manual() {
            self.dismissed_status = None;
        }
        if self.pending_poll.is_some() {
            if self.update_check_type == UpdateCheckType::Automatic {
                self.update_check_type = check_type;
                cx.notify();
            }
            return;
        }
        self.update_check_type = check_type;

        cx.notify();

        self.pending_poll = Some(cx.spawn(async move |this, cx| {
            let result = Self::update(this.upgrade()?, cx).await;
            this.update(cx, |this, cx| {
                this.pending_poll = None;
                if let Err(error) = result {
                    let is_missing_dependency =
                        error.downcast_ref::<MissingDependencyError>().is_some();
                    this.status = match check_type {
                        UpdateCheckType::Automatic if is_missing_dependency => {
                            log::warn!("auto-update: {}", error);
                            AutoUpdateStatus::Errored {
                                error: Arc::new(error),
                            }
                        }
                        // Be quiet if the check was automated (e.g. when offline)
                        UpdateCheckType::Automatic => {
                            log::info!("auto-update check failed: error:{:?}", error);
                            AutoUpdateStatus::Idle
                        }
                        UpdateCheckType::Manual => {
                            log::error!("auto-update failed: error:{:?}", error);
                            AutoUpdateStatus::Errored {
                                error: Arc::new(error),
                            }
                        }
                    };

                    cx.notify();
                }
            })
            .ok()
        }));
    }

    pub fn current_version(&self) -> Version {
        self.current_version.clone()
    }

    pub fn status(&self) -> AutoUpdateStatus {
        self.status.clone()
    }

    pub fn dismissed_status(&self) -> Option<AutoUpdateStatus> {
        self.dismissed_status.clone()
    }

    pub fn dismiss_status(&mut self, status: AutoUpdateStatus, cx: &mut Context<Self>) {
        self.dismissed_status = Some(status);
        cx.notify();
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) -> bool {
        if let AutoUpdateStatus::Idle = self.status {
            return false;
        }
        self.status = AutoUpdateStatus::Idle;
        cx.notify();
        true
    }

    pub async fn download_remote_server_release(
        release_channel: ReleaseChannel,
        version: Option<Version>,
        os: &str,
        arch: &str,
        set_status: impl Fn(&str, &mut AsyncApp) + Send + 'static,
        cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        let this = cx.update(|cx| {
            cx.default_global::<GlobalAutoUpdate>()
                .0
                .clone()
                .context("auto-update not initialized")
        })?;

        set_status("Fetching remote server release", cx);
        let release =
            Self::get_fork_remote_server_release(&this, version.as_ref(), os, arch, cx).await?;

        let extension = fork_remote_server_extension(os)?;
        let servers_dir = paths::remote_servers_dir();
        let channel_dir = servers_dir.join(release_channel.dev_name());
        let platform_dir = channel_dir.join(format!("{}-{}", os, arch));
        let version_path = platform_dir.join(format!("{}.{extension}", release.version));
        smol::fs::create_dir_all(&platform_dir).await.ok();

        let client = this.read_with(cx, |this, _| this.client.http_client());

        if smol::fs::metadata(&version_path).await.is_err() {
            log::info!(
                "downloading zed-remote-server {os} {arch} version {}",
                release.version
            );
            set_status("Downloading remote server", cx);
            download_remote_server_binary(&version_path, release, client).await?;
        }

        if let Err(error) =
            cleanup_remote_server_cache(&platform_dir, &version_path, REMOTE_SERVER_CACHE_LIMIT)
                .await
        {
            log::warn!(
                "Failed to clean up remote server cache in {:?}: {error:#}",
                platform_dir
            );
        }

        Ok(version_path)
    }

    pub async fn get_remote_server_release_url(
        _channel: ReleaseChannel,
        version: Option<Version>,
        os: &str,
        arch: &str,
        cx: &mut AsyncApp,
    ) -> Result<Option<String>> {
        let this = cx.update(|cx| {
            cx.default_global::<GlobalAutoUpdate>()
                .0
                .clone()
                .context("auto-update not initialized")
        })?;

        let release =
            Self::get_fork_remote_server_release(&this, version.as_ref(), os, arch, cx).await?;

        Ok(Some(release.url))
    }

    async fn fetch_fork_release_manifest(
        http_client: &Arc<HttpClientWithUrl>,
        manifest_url: &str,
    ) -> Result<Option<ForkReleaseManifest>> {
        let mut response = http_client
            .get(manifest_url, Default::default(), true)
            .await?;
        if response.status() == http_client::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        anyhow::ensure!(
            response.status().is_success(),
            "failed to fetch {manifest_url}: {}",
            response.status(),
        );
        let manifest: ForkReleaseManifest =
            serde_json::from_slice(body.as_slice()).with_context(|| {
                format!(
                    "error deserializing release manifest {:?}",
                    String::from_utf8_lossy(&body),
                )
            })?;
        Ok(Some(manifest))
    }

    async fn get_fork_release_manifest(
        http_client: &Arc<HttpClientWithUrl>,
        version: Option<&Version>,
    ) -> Result<ForkReleaseManifest> {
        let latest_url = format!(
            "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/latest/download/release.json"
        );
        if let Some(requested_version) = version {
            let tag = fork_release_tag_for_version(requested_version);
            let tagged_url = format!(
                "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/{tag}/release.json"
            );
            if let Some(manifest) =
                Self::fetch_fork_release_manifest(http_client, &tagged_url).await?
            {
                return Ok(manifest);
            }
            // Fall back to the latest release when this build was published
            // under a custom `fork-v*` tag whose name differs from `fork-v{version}`.
            let manifest = Self::fetch_fork_release_manifest(http_client, &latest_url)
                .await?
                .with_context(|| format!("failed to fetch {latest_url}: 404 Not Found"))?;
            let manifest_version = manifest.version.parse::<Version>().with_context(|| {
                format!("invalid version in release manifest: {}", manifest.version)
            })?;
            anyhow::ensure!(
                (
                    manifest_version.major,
                    manifest_version.minor,
                    manifest_version.patch,
                ) == (
                    requested_version.major,
                    requested_version.minor,
                    requested_version.patch,
                ),
                "no release {tag} found in {FORK_RELEASE_REPOSITORY} (latest is {})",
                manifest.tag,
            );
            return Ok(manifest);
        }

        Self::fetch_fork_release_manifest(http_client, &latest_url)
            .await?
            .with_context(|| format!("failed to fetch {latest_url}: 404 Not Found"))
    }

    async fn get_fork_release(
        this: &Entity<Self>,
        os: &str,
        arch: &str,
        cx: &mut AsyncApp,
    ) -> Result<ReleaseAsset> {
        let http_client = this.read_with(cx, |this, _| this.client.http_client());
        let manifest = Self::get_fork_release_manifest(&http_client, None).await?;
        let asset = fork_release_asset_name(os, arch)?;
        anyhow::ensure!(
            manifest.assets.contains(&asset),
            "the latest release ({}) has no build for {os} {arch}",
            manifest.tag,
        );
        Ok(ReleaseAsset {
            version: manifest.version,
            url: format!(
                "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/{}/{asset}",
                manifest.tag
            ),
        })
    }

    async fn get_fork_remote_server_release(
        this: &Entity<Self>,
        version: Option<&Version>,
        os: &str,
        arch: &str,
        cx: &mut AsyncApp,
    ) -> Result<ReleaseAsset> {
        let http_client = this.read_with(cx, |this, _| this.client.http_client());
        let manifest = Self::get_fork_release_manifest(&http_client, version).await?;
        let asset = fork_remote_server_asset_name(os, arch)?;
        anyhow::ensure!(
            manifest.assets.contains(&asset),
            "release {} has no remote server for {os} {arch}",
            manifest.tag,
        );
        Ok(ReleaseAsset {
            version: manifest.version,
            url: format!(
                "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/{}/{asset}",
                manifest.tag
            ),
        })
    }

    async fn update(this: Entity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (client, installed_version, previous_status, release_channel) =
            this.read_with(cx, |this, cx| {
                (
                    this.client.http_client(),
                    this.current_version.clone(),
                    this.status.clone(),
                    ReleaseChannel::try_global(cx).unwrap_or(ReleaseChannel::Stable),
                )
            });

        Self::check_dependencies()?;

        this.update(cx, |this, cx| {
            this.status = AutoUpdateStatus::Checking;
            log::info!("Auto Update: checking for updates");
            cx.notify();
        });

        let fetched_release_data = Self::get_fork_release(&this, OS, ARCH, cx).await?;
        let fetched_version = fetched_release_data.clone().version;
        let app_commit_sha = Ok(cx.update(|cx| AppCommitSha::try_global(cx).map(|sha| sha.full())));
        let newer_version = Self::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version,
            previous_status.clone(),
        )?;

        let Some(newer_version) = newer_version else {
            this.update(cx, |this, cx| {
                this.status = match previous_status {
                    AutoUpdateStatus::Updated { .. } => previous_status,
                    _ => {
                        if this.update_check_type.is_manual() {
                            cx.emit(AutoUpdateEvent::UpToDate);
                        }
                        AutoUpdateStatus::Idle
                    }
                };
                cx.notify();
            });
            return Ok(());
        };

        this.update(cx, |this, cx| {
            this.status = AutoUpdateStatus::Downloading {
                version: newer_version.clone(),
                progress: None,
            };
            cx.notify();
        });

        let installer_dir = InstallerDir::new()
            .await
            .context("Failed to create installer dir")?;
        let target_path = Self::target_path(&installer_dir).await?;
        let progress_entity = this.clone();
        let mut progress_cx = cx.clone();
        download_release(
            &target_path,
            fetched_release_data,
            client,
            move |progress| {
                progress_entity.update(&mut progress_cx, |this, cx| {
                    if let AutoUpdateStatus::Downloading {
                        progress: current_progress,
                        ..
                    } = &mut this.status
                    {
                        *current_progress = progress;
                        cx.notify();
                    }
                });
            },
        )
        .await
        .with_context(|| format!("Failed to download update to {}", target_path.display()))?;

        this.update(cx, |this, cx| {
            this.status = AutoUpdateStatus::Installing {
                version: newer_version.clone(),
            };
            cx.notify();
        });

        #[cfg(test)]
        let install_result = match cx
            .try_read_global::<tests::InstallOverride, _>(|g, _| g.0.clone())
            .map(|test_install| test_install(&target_path, cx))
        {
            Some(result) => result,
            None => return Ok(()),
        };

        #[cfg(not(test))]
        let install_result = {
            let running_app_path = cx.update(|cx| cx.app_path())?;
            let askpass = Self::install_askpass_session(&running_app_path, cx).await?;
            let askpass_script = askpass
                .as_ref()
                .map(|askpass| PathBuf::from(askpass.script_path().as_ref()));
            let background_executor = cx.background_executor().clone();
            let channel = cx.update(|cx| ReleaseChannel::global(cx).dev_name());
            let install_result = cx
                .background_spawn(Self::install_release(
                    installer_dir,
                    target_path.clone(),
                    running_app_path,
                    channel,
                    askpass_script,
                    background_executor,
                ))
                .await;
            drop(askpass);
            install_result
        };
        let new_binary_path = install_result
            .with_context(|| format!("Failed to install update at: {}", target_path.display()))?;
        if let Some(new_binary_path) = new_binary_path {
            cx.update(|cx| cx.set_restart_path(new_binary_path));
        }

        this.update(cx, |this, cx| {
            this.set_should_show_update_notification(true, cx)
                .detach_and_log_err(cx);
            this.status = AutoUpdateStatus::Updated {
                version: newer_version,
            };
            cx.notify();
        });
        Ok(())
    }

    fn check_if_fetched_version_is_newer(
        release_channel: ReleaseChannel,
        app_commit_sha: Result<Option<String>>,
        installed_version: Version,
        fetched_version: String,
        status: AutoUpdateStatus,
    ) -> Result<Option<Version>> {
        let fetched_version = fetched_version.parse::<Version>()?;

        match release_channel {
            ReleaseChannel::Nightly => {
                let should_download = if let AutoUpdateStatus::Updated { version } = status {
                    fetched_version != version
                } else {
                    let fetched_sha = fetched_version.build.as_str().rsplit('.').next();
                    app_commit_sha
                        .ok()
                        .flatten()
                        .is_none_or(|sha| fetched_sha != Some(sha.as_str()))
                };
                Ok(should_download.then_some(fetched_version))
            }
            _ => {
                let current_version = if let AutoUpdateStatus::Updated { version } = status {
                    version
                } else {
                    installed_version
                };
                Ok(Self::check_if_fetched_version_is_newer_non_nightly(
                    current_version,
                    fetched_version,
                ))
            }
        }
    }

    fn check_dependencies() -> Result<()> {
        #[cfg(target_os = "linux")]
        if let Some(prefix) = env::current_exe()
            .ok()
            .and_then(|running_app_path| linux_system_install_prefix(&running_app_path))
        {
            if which::which("sudo").is_err() && !is_root() {
                return Err(MissingDependencyError(format!(
                    "sudo is required to install updates into {}. Install sudo, or update by running install-zed-linux.sh from https://github.com/{FORK_RELEASE_REPOSITORY}/releases/latest as root",
                    prefix.display()
                ))
                .into());
            }
        } else if which::which("rsync").is_err() {
            let install_hint = linux_rsync_install_hint();
            return Err(MissingDependencyError(format!(
                "rsync is required for auto-updates but is not installed. {install_hint}"
            ))
            .into());
        }

        #[cfg(target_os = "macos")]
        anyhow::ensure!(
            which::which("rsync").is_ok(),
            "Could not auto-update because the required rsync utility was not found."
        );

        Ok(())
    }

    async fn target_path(installer_dir: &InstallerDir) -> Result<PathBuf> {
        let filename = match OS {
            "macos" => anyhow::Ok("Zed.dmg"),
            "linux" => Ok("zed.tar.gz"),
            "windows" => Ok("Zed.exe"),
            unsupported_os => anyhow::bail!("not supported: {unsupported_os}"),
        }?;

        Ok(installer_dir.path().join(filename))
    }

    /// Linux system installs need root, which `sudo` gets from Zed's own
    /// password prompt through askpass, so no polkit agent is needed.
    #[cfg_attr(test, allow(dead_code))]
    async fn install_askpass_session(
        running_app_path: &Path,
        cx: &mut AsyncApp,
    ) -> Result<Option<AskPassSession>> {
        if OS != "linux" || is_root() || linux_system_install_prefix(running_app_path).is_none() {
            return Ok(None);
        }
        let prompt = cx
            .update(|cx| {
                cx.try_global::<GlobalInstallPasswordPrompt>()
                    .map(|prompt| prompt.0.clone())
            })
            .context("no password prompt is available to install the update")?;
        let delegate = AskPassDelegate::new_with_cancellation(
            cx,
            move |prompt_text, response_sender, cancellation, cx| {
                prompt(prompt_text, response_sender, cancellation, cx)
            },
        );
        let session = AskPassSession::new(cx.background_executor().clone(), delegate).await?;
        Ok(Some(session))
    }

    #[cfg_attr(test, allow(dead_code))]
    async fn install_release(
        installer_dir: InstallerDir,
        target_path: PathBuf,
        running_app_path: PathBuf,
        channel: &str,
        askpass_script: Option<PathBuf>,
        background_executor: BackgroundExecutor,
    ) -> Result<Option<PathBuf>> {
        match OS {
            "macos" => {
                install_release_macos(
                    &installer_dir,
                    &target_path,
                    running_app_path,
                    &background_executor,
                )
                .await
            }
            "linux" => {
                install_release_linux(
                    &installer_dir,
                    &target_path,
                    channel,
                    running_app_path,
                    askpass_script,
                )
                .await
            }
            "windows" => install_release_windows(&target_path).await,
            unsupported_os => anyhow::bail!("not supported: {unsupported_os}"),
        }
    }

    fn check_if_fetched_version_is_newer_non_nightly(
        mut installed_version: Version,
        fetched_version: Version,
    ) -> Option<Version> {
        let installed_build_number = fork_build_number(&installed_version);
        let fetched_build_number = fork_build_number(&fetched_version);
        // Semver orders build metadata too, so it is compared separately: fork
        // releases rebuild the same version with a higher run number.
        installed_version.pre = semver::Prerelease::EMPTY;
        installed_version.build = semver::BuildMetadata::EMPTY;
        let mut fetched_release = fetched_version.clone();
        fetched_release.build = semver::BuildMetadata::EMPTY;
        let is_newer = fetched_release > installed_version
            || (fetched_release == installed_version
                && matches!(
                    (fetched_build_number, installed_build_number),
                    (Some(fetched), Some(installed)) if fetched > installed
                ));
        is_newer.then_some(fetched_version)
    }

    pub fn set_should_show_update_notification(
        &self,
        should_show: bool,
        cx: &App,
    ) -> Task<Result<()>> {
        let kvp = KeyValueStore::global(cx);
        cx.background_spawn(async move {
            if should_show {
                kvp.write_kvp(
                    SHOULD_SHOW_UPDATE_NOTIFICATION_KEY.to_string(),
                    "".to_string(),
                )
                .await?;
            } else {
                kvp.delete_kvp(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY.to_string())
                    .await?;
            }
            Ok(())
        })
    }

    pub fn should_show_update_notification(&self, cx: &App) -> Task<Result<bool>> {
        let kvp = KeyValueStore::global(cx);
        cx.background_spawn(async move {
            Ok(kvp.read_kvp(SHOULD_SHOW_UPDATE_NOTIFICATION_KEY)?.is_some())
        })
    }
}

async fn download_remote_server_binary(
    target_path: &PathBuf,
    release: ReleaseAsset,
    client: Arc<HttpClientWithUrl>,
) -> Result<()> {
    let temp = tempfile::Builder::new().tempfile_in(remote_servers_dir())?;
    let mut temp_file = File::create(&temp).await?;

    let mut response = client.get(&release.url, Default::default(), true).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "failed to download remote server release: {:?}",
        response.status()
    );
    smol::io::copy(response.body_mut(), &mut temp_file).await?;
    smol::fs::rename(&temp, &target_path).await?;

    Ok(())
}

async fn cleanup_remote_server_cache(
    platform_dir: &Path,
    keep_path: &Path,
    limit: usize,
) -> Result<()> {
    if limit == 0 {
        return Ok(());
    }

    let mut entries = smol::fs::read_dir(platform_dir).await?;
    let now = SystemTime::now();
    let mut candidates = Vec::new();

    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if !matches!(path.extension().and_then(OsStr::to_str), Some("gz" | "zip")) {
            continue;
        }

        let mtime = if path == keep_path {
            now
        } else {
            smol::fs::metadata(&path)
                .await
                .and_then(|metadata| metadata.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        };

        candidates.push((path, mtime));
    }

    if candidates.len() <= limit {
        return Ok(());
    }

    candidates.sort_by(|(path_a, time_a), (path_b, time_b)| {
        time_b.cmp(time_a).then_with(|| path_a.cmp(path_b))
    });

    for (index, (path, _)) in candidates.into_iter().enumerate() {
        if index < limit || path == keep_path {
            continue;
        }

        if let Err(error) = smol::fs::remove_file(&path).await {
            log::warn!(
                "Failed to remove old remote server archive {:?}: {}",
                path,
                error
            );
        }
    }

    Ok(())
}

async fn download_release(
    target_path: &Path,
    release: ReleaseAsset,
    client: Arc<HttpClientWithUrl>,
    mut on_progress: impl FnMut(Option<f32>),
) -> Result<()> {
    let mut target_file = File::create(&target_path).await?;

    let mut response = client.get(&release.url, Default::default(), true).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "failed to download update: {:?}",
        response.status()
    );

    let total_bytes = response
        .headers()
        .get(http_client::http::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|total_bytes| *total_bytes > 0);

    let mut downloaded_bytes: u64 = 0;
    let mut last_reported_percent: Option<u8> = None;
    let mut buffer = [0u8; 8192];
    let body = response.body_mut();
    loop {
        let bytes_read = body.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        target_file.write_all(&buffer[..bytes_read]).await?;
        downloaded_bytes += bytes_read as u64;

        if let Some(total_bytes) = total_bytes {
            let fraction = (downloaded_bytes as f32 / total_bytes as f32).clamp(0.0, 1.0);
            // Only report when the whole-number percentage changes to avoid notifying the UI on every chunk.
            let percent = (fraction * 100.0) as u8;
            if last_reported_percent != Some(percent) {
                last_reported_percent = Some(percent);
                on_progress(Some(fraction));
            }
        }
    }
    target_file.flush().await?;
    if total_bytes.is_some() && last_reported_percent != Some(100) {
        on_progress(Some(1.0));
    }
    log::info!("downloaded update. path:{:?}", target_path);

    Ok(())
}

/// The prefix of an install made by `script/install-linux-release`, which
/// places the editor in `$PREFIX/lib/zed/libexec/zed-editor`.
fn linux_system_install_prefix(running_app_path: &Path) -> Option<PathBuf> {
    // Once an update replaces the running binary, /proc/self/exe reports it as deleted.
    let running_app_path = running_app_path.to_str()?.trim_end_matches(" (deleted)");
    let running_app_path = Path::new(running_app_path);
    if !running_app_path.is_absolute() || !running_app_path.ends_with("lib/zed/libexec/zed-editor")
    {
        return None;
    }
    running_app_path.ancestors().nth(4).map(Path::to_path_buf)
}

fn is_root() -> bool {
    std::fs::read_to_string("/proc/self/status").is_ok_and(|status| {
        status
            .lines()
            .find_map(|line| line.strip_prefix("Uid:"))
            .and_then(|uids| uids.split_whitespace().nth(1))
            == Some("0")
    })
}

/// Installs the update the same way as a manual install: with the archive's
/// own `install-linux-release`, which runs `sudo -A` once to replace the files
/// under the system prefix, getting the password from Zed through askpass.
async fn install_release_linux_system(
    extracted: &Path,
    downloaded_tar_gz: &Path,
    prefix: &Path,
    askpass_script: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    let install_script = extracted.join("zed.app").join("install-linux-release");
    anyhow::ensure!(
        fs::metadata(&install_script).await.is_ok(),
        "the update archive has no installer; run install-zed-linux.sh from https://github.com/{FORK_RELEASE_REPOSITORY}/releases/latest",
    );

    let mut cmd = new_command("bash");
    cmd.arg(&install_script)
        .arg(downloaded_tar_gz)
        .env("PREFIX", prefix);
    match askpass_script {
        Some(askpass_script) => {
            cmd.env("SUDO", "sudo").env("SUDO_ASKPASS", askpass_script);
        }
        None => {
            anyhow::ensure!(
                is_root(),
                "cannot ask for the password to install the update"
            );
            cmd.env("SUDO", "");
        }
    }
    let output = cmd
        .output()
        .await
        .with_context(|| format!("failed to run {}", install_script.display()))?;
    anyhow::ensure!(
        output.status.success(),
        "failed to install the update into {}: {}",
        prefix.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    );

    Ok(Some(prefix.join("lib/zed/libexec/zed-editor")))
}

async fn install_release_linux(
    temp_dir: &InstallerDir,
    downloaded_tar_gz: &Path,
    channel: &str,
    running_app_path: PathBuf,
    askpass_script: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    let home_dir = PathBuf::from(env::var("HOME").context("no HOME env var set")?);

    let extracted = temp_dir.path().join("zed");
    fs::create_dir_all(&extracted)
        .await
        .context("failed to create directory into which to extract update")?;

    let mut cmd = new_command("tar");
    cmd.arg("-xzf")
        .arg(&downloaded_tar_gz)
        .arg("-C")
        .arg(&extracted);
    let output = cmd
        .output()
        .await
        .with_context(|| "failed to extract: {cmd}")?;

    anyhow::ensure!(
        output.status.success(),
        "failed to extract {:?} to {:?}: {:?}",
        downloaded_tar_gz,
        extracted,
        String::from_utf8_lossy(&output.stderr)
    );

    if let Some(prefix) = linux_system_install_prefix(&running_app_path) {
        return install_release_linux_system(
            &extracted,
            downloaded_tar_gz,
            &prefix,
            askpass_script,
        )
        .await;
    }

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

    let mut cmd = new_command("rsync");
    cmd.args(["-av", "--delete"]).arg(&from).arg(&to);
    let output = cmd
        .output()
        .await
        .with_context(|| "failed to rsync: {cmd}")?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy Zed update from {:?} to {:?}: {:?}",
        from,
        to,
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(Some(to.join(expected_suffix)))
}

async fn install_release_macos(
    temp_dir: &InstallerDir,
    downloaded_dmg: &Path,
    running_app_path: PathBuf,
    background_executor: &BackgroundExecutor,
) -> Result<Option<PathBuf>> {
    let running_app_filename = running_app_path
        .file_name()
        .with_context(|| format!("invalid running app path {running_app_path:?}"))?;

    let mount_path = temp_dir.path().join("Zed");
    let mut mounted_app_path: OsString = mount_path.join(running_app_filename).into();

    mounted_app_path.push("/");
    let mut cmd = new_command("hdiutil");
    cmd.args(["attach", "-nobrowse"])
        .arg(&downloaded_dmg)
        .arg("-mountroot")
        .arg(temp_dir.path());
    let output = cmd
        .output()
        .await
        .with_context(|| "failed to mount: {cmd}")?;

    anyhow::ensure!(
        output.status.success(),
        "failed to mount: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let unmounter = MacOsUnmounter {
        mount_path: mount_path.clone(),
        background_executor,
    };

    let mut cmd = new_command("rsync");
    cmd.args(["-av", "--delete", "--exclude", "Icon?"])
        .arg(&mounted_app_path)
        .arg(&running_app_path);
    let rsync_output = cmd.output().await;

    // Await the unmount (even if rsync failed) so that the installer temp dir
    // can be deleted once this function returns.
    unmounter.unmount().await;

    let output = rsync_output.with_context(|| "failed to rsync: {cmd}")?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy app: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(None)
}

/// Removes stale installer dirs from the system temp dir. Older Zed versions
/// leaked one per update by deleting the dir while the downloaded disk image
/// was still mounted inside it, which made the deletion fail silently.
#[cfg(any(rust_analyzer, all(not(target_os = "windows"), not(test))))]
async fn cleanup_stale_installer_dirs() {
    const STALE_INSTALLER_DIR_AGE: Duration = Duration::from_secs(24 * 60 * 60);

    let temp_dir = std::env::temp_dir();
    let Ok(mut entries) = fs::read_dir(&temp_dir).await else {
        log::warn!("failed to read temp dir {temp_dir:?} while cleaning up installer dirs");
        return;
    };
    while let Some(entry) = entries.next().await {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with(INSTALLER_DIR_PREFIX)
        {
            continue;
        }
        // Leave recent dirs alone, as they may belong to an update currently
        // in progress in another Zed instance.
        let is_stale = entry.metadata().await.ok().is_some_and(|metadata| {
            metadata.is_dir()
                && metadata.modified().ok().is_some_and(|modified| {
                    SystemTime::now()
                        .duration_since(modified)
                        .is_ok_and(|age| age > STALE_INSTALLER_DIR_AGE)
                })
        });
        if is_stale {
            if let Err(error) = fs::remove_dir_all(entry.path()).await {
                log::warn!(
                    "failed to remove stale installer dir {:?}: {error}",
                    entry.path()
                );
            } else {
                log::info!("removed stale installer dir {:?}", entry.path());
            }
        }
    }
}

async fn cleanup_windows() -> Result<()> {
    let parent = std::env::current_exe()?
        .parent()
        .context("No parent dir for Zed.exe")?
        .to_owned();

    // keep in sync with crates/auto_update_helper/src/updater.rs
    _ = smol::fs::remove_dir(parent.join("updates")).await;
    _ = smol::fs::remove_dir(parent.join("install")).await;
    _ = smol::fs::remove_dir(parent.join("old")).await;

    Ok(())
}

async fn install_release_windows(downloaded_installer: &Path) -> Result<Option<PathBuf>> {
    let mut cmd = new_command(downloaded_installer);
    cmd.arg("/verysilent")
        .arg("/update=true")
        .arg("/MERGETASKS=!desktopicon");
    let output = cmd.output().await?;
    anyhow::ensure!(
        output.status.success(),
        "failed to start installer: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    // We return the path to the update helper program, because it will
    // perform the final steps of the update process, copying the new binary,
    // deleting the old one, and launching the new binary.
    let helper_path = std::env::current_exe()?
        .parent()
        .context("No parent dir for Zed.exe")?
        .join("tools")
        .join("auto_update_helper.exe");
    Ok(Some(helper_path))
}

pub async fn finalize_auto_update_on_quit() {
    let Some(installer_path) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("updates")))
    else {
        return;
    };

    // The installer will create a flag file after it finishes updating
    let flag_file = installer_path.join("versions.txt");
    if flag_file.exists()
        && let Some(helper) = installer_path
            .parent()
            .map(|p| p.join("tools").join("auto_update_helper.exe"))
    {
        let mut command = util::command::new_command(helper);
        command.arg("--launch");
        command.arg("false");
        if let Ok(mut cmd) = command.spawn() {
            _ = cmd.status().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use client::Client;
    use clock::FakeSystemClock;
    use futures::channel::oneshot;
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, Response};
    use settings::default_settings;
    use std::{
        rc::Rc,
        sync::{
            Arc,
            atomic::{self, AtomicBool},
        },
    };
    use tempfile::tempdir;

    #[ctor::ctor(unsafe)]
    fn init_logger() {
        zlog::init_test();
    }

    use super::*;

    pub(super) struct InstallOverride(pub Rc<dyn Fn(&Path, &AsyncApp) -> Result<Option<PathBuf>>>);
    impl Global for InstallOverride {}

    #[gpui::test]
    fn test_auto_update_defaults_to_true(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let mut store = SettingsStore::new(cx, &settings::default_settings());
            store
                .set_default_settings(&default_settings(), cx)
                .expect("Unable to set default settings");
            store
                .set_user_settings("{}", cx)
                .expect("Unable to set user settings");
            cx.set_global(store);
            assert!(AutoUpdateSetting::get_global(cx).0);
        });
    }

    #[gpui::test]
    async fn test_auto_update_downloads(cx: &mut TestAppContext) {
        cx.background_executor.allow_parking();
        zlog::init_test();
        let release_available = Arc::new(AtomicBool::new(false));

        let (dmg_tx, dmg_rx) = oneshot::channel::<String>();

        cx.update(|cx| {
            settings::init(cx);

            let current_version = semver::Version::new(0, 100, 0);
            release_channel::init_test(current_version, ReleaseChannel::Stable, cx);

            let clock = Arc::new(FakeSystemClock::new());
            let release_available = Arc::clone(&release_available);
            let dmg_rx = Arc::new(parking_lot::Mutex::new(Some(dmg_rx)));
            let fake_client_http = FakeHttpClient::create(move |req| {
                let release_available = release_available.load(atomic::Ordering::Relaxed);
                let dmg_rx = dmg_rx.clone();
                async move {
                let asset = fork_release_asset_name(OS, ARCH).unwrap();
                if req.uri().path() == format!("/{FORK_RELEASE_REPOSITORY}/releases/latest/download/release.json") {
                    let (version, tag) = if release_available {
                        ("0.100.1", "new")
                    } else {
                        ("0.100.0", "old")
                    };
                    return Ok(Response::builder().status(200).body(
                        format!(r#"{{"version":"{version}","tag":"{tag}","assets":["{asset}"]}}"#).into()
                    ).unwrap());
                } else if req.uri().path() == format!("/{FORK_RELEASE_REPOSITORY}/releases/download/new/{asset}") {
                    return Ok(Response::builder().status(200).body({
                        let dmg_rx = dmg_rx.lock().take().unwrap();
                        dmg_rx.await.unwrap().into()
                    }).unwrap());
                }
                Ok(Response::builder().status(404).body("".into()).unwrap())
                }
            });
            let client = Client::new(clock, fake_client_http, cx);
            crate::init(client, cx);
        });

        let auto_updater = cx.update(|cx| AutoUpdater::get(cx).expect("auto updater should exist"));

        cx.background_executor.run_until_parked();

        auto_updater.read_with(cx, |updater, _| {
            assert_eq!(updater.status(), AutoUpdateStatus::Idle);
            assert_eq!(updater.current_version(), semver::Version::new(0, 100, 0));
        });

        release_available.store(true, atomic::Ordering::SeqCst);
        cx.background_executor.advance_clock(POLL_INTERVAL);
        cx.background_executor.run_until_parked();

        loop {
            cx.background_executor.timer(Duration::from_millis(0)).await;
            cx.run_until_parked();
            let status = auto_updater.read_with(cx, |updater, _| updater.status());
            if !matches!(status, AutoUpdateStatus::Idle) {
                break;
            }
        }
        let status = auto_updater.read_with(cx, |updater, _| updater.status());
        assert_eq!(
            status,
            AutoUpdateStatus::Downloading {
                version: semver::Version::new(0, 100, 1),
                progress: None,
            }
        );

        dmg_tx.send("<fake-zed-update>".to_owned()).unwrap();

        let tmp_dir = Arc::new(tempdir().unwrap());

        cx.update(|cx| {
            let tmp_dir = tmp_dir.clone();
            cx.set_global(InstallOverride(Rc::new(move |target_path, _cx| {
                let tmp_dir = tmp_dir.clone();
                let dest_path = tmp_dir.path().join("zed");
                std::fs::copy(&target_path, &dest_path)?;
                Ok(Some(dest_path))
            })));
        });

        loop {
            cx.background_executor.timer(Duration::from_millis(0)).await;
            cx.run_until_parked();
            let status = auto_updater.read_with(cx, |updater, _| updater.status());
            if !matches!(status, AutoUpdateStatus::Downloading { .. }) {
                break;
            }
        }
        let status = auto_updater.read_with(cx, |updater, _| updater.status());
        assert_eq!(
            status,
            AutoUpdateStatus::Updated {
                version: semver::Version::new(0, 100, 1)
            }
        );
        let will_restart = cx.expect_restart();
        cx.update(|cx| cx.restart());
        let (path, arguments) = will_restart.await.unwrap();
        assert!(arguments.is_empty());
        let path = path.unwrap();
        assert_eq!(path, tmp_dir.path().join("zed"));
        assert_eq!(std::fs::read_to_string(path).unwrap(), "<fake-zed-update>");
    }

    #[gpui::test]
    async fn test_download_release_reports_progress(cx: &mut TestAppContext) {
        cx.background_executor.allow_parking();

        let body = vec![0u8; 20_000];
        let content_length = body.len();

        let client = FakeHttpClient::create(move |_req| {
            let body = body.clone();
            async move {
                Ok(Response::builder()
                    .status(200)
                    .header(
                        http_client::http::header::CONTENT_LENGTH,
                        body.len().to_string(),
                    )
                    .body(body.into())
                    .unwrap())
            }
        });

        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("zed-download");
        let release = ReleaseAsset {
            version: "1.0.0".to_string(),
            url: "https://test.example/download".to_string(),
        };

        let reported = Rc::new(std::cell::RefCell::new(Vec::<f32>::new()));
        download_release(&target_path, release, client, {
            let reported = reported.clone();
            move |fraction| {
                if let Some(fraction) = fraction {
                    reported.borrow_mut().push(fraction);
                }
            }
        })
        .await
        .unwrap();

        let reported = reported.borrow();
        assert!(
            reported.len() >= 2,
            "expected progress to be reported across multiple reads, got {reported:?}"
        );
        assert_eq!(
            reported.last().copied(),
            Some(1.0),
            "download should finish at 100%"
        );
        for fraction in reported.iter() {
            assert!(
                (0.0..=1.0).contains(fraction),
                "progress {fraction} out of range"
            );
        }
        for pair in reported.windows(2) {
            assert!(
                pair[0] <= pair[1],
                "progress must not decrease: {reported:?}"
            );
        }

        let downloaded_len = std::fs::metadata(&target_path).unwrap().len();
        assert_eq!(downloaded_len, content_length as u64);
    }

    #[gpui::test]
    async fn test_download_release_without_content_length_reports_no_progress(
        cx: &mut TestAppContext,
    ) {
        cx.background_executor.allow_parking();

        let body = vec![0u8; 20_000];
        let content_length = body.len();

        let client = FakeHttpClient::create(move |_req| {
            let body = body.clone();
            async move { Ok(Response::builder().status(200).body(body.into()).unwrap()) }
        });

        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("zed-download");
        let release = ReleaseAsset {
            version: "1.0.0".to_string(),
            url: "https://test.example/download".to_string(),
        };

        let reported = Rc::new(std::cell::RefCell::new(Vec::<Option<f32>>::new()));
        download_release(&target_path, release, client, {
            let reported = reported.clone();
            move |fraction| {
                reported.borrow_mut().push(fraction);
            }
        })
        .await
        .unwrap();

        assert!(
            reported.borrow().is_empty(),
            "progress should not be reported when the total size is unknown, got {:?}",
            reported.borrow()
        );

        let downloaded_len = std::fs::metadata(&target_path).unwrap().len();
        assert_eq!(downloaded_len, content_length as u64);
    }

    #[test]
    fn test_stable_does_not_update_when_fetched_version_is_not_higher() {
        let release_channel = ReleaseChannel::Stable;
        let app_commit_sha = Ok(Some("a".to_string()));
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Idle;
        let fetched_version = semver::Version::new(1, 0, 0);

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.to_string(),
            status,
        );

        assert_eq!(newer_version.unwrap(), None);
    }

    #[test]
    fn test_stable_does_update_when_fetched_version_is_higher() {
        let release_channel = ReleaseChannel::Stable;
        let app_commit_sha = Ok(Some("a".to_string()));
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Idle;
        let fetched_version = semver::Version::new(1, 0, 1);

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.to_string(),
            status,
        );

        assert_eq!(newer_version.unwrap(), Some(fetched_version));
    }

    #[test]
    fn test_stable_does_not_update_when_fetched_version_is_not_higher_than_cached() {
        let release_channel = ReleaseChannel::Stable;
        let app_commit_sha = Ok(Some("a".to_string()));
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Updated {
            version: semver::Version::new(1, 0, 1),
        };
        let fetched_version = semver::Version::new(1, 0, 1);

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.to_string(),
            status,
        );

        assert_eq!(newer_version.unwrap(), None);
    }

    #[test]
    fn test_stable_does_update_when_fetched_version_is_higher_than_cached() {
        let release_channel = ReleaseChannel::Stable;
        let app_commit_sha = Ok(Some("a".to_string()));
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Updated {
            version: semver::Version::new(1, 0, 1),
        };
        let fetched_version = semver::Version::new(1, 0, 2);

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.to_string(),
            status,
        );

        assert_eq!(newer_version.unwrap(), Some(fetched_version));
    }

    #[test]
    fn test_stable_updates_to_a_higher_fork_build_of_the_same_version() {
        let installed_version: Version = "1.28.0+stable.6.aaa".parse().unwrap();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer_non_nightly(
            installed_version.clone(),
            "1.28.0+stable.7.bbb".parse().unwrap(),
        );
        assert_eq!(newer_version, Some("1.28.0+stable.7.bbb".parse().unwrap()));

        let same_build = AutoUpdater::check_if_fetched_version_is_newer_non_nightly(
            installed_version,
            "1.28.0+stable.6.aaa".parse().unwrap(),
        );
        assert_eq!(same_build, None);

        let local_build = AutoUpdater::check_if_fetched_version_is_newer_non_nightly(
            "1.28.0+stable.ccc".parse().unwrap(),
            "1.28.0+stable.7.bbb".parse().unwrap(),
        );
        assert_eq!(local_build, None);
    }

    #[test]
    fn test_linux_system_install_prefix() {
        assert_eq!(
            linux_system_install_prefix(Path::new("/usr/lib/zed/libexec/zed-editor")),
            Some(PathBuf::from("/usr"))
        );
        assert_eq!(
            linux_system_install_prefix(Path::new("/opt/zed/lib/zed/libexec/zed-editor (deleted)")),
            Some(PathBuf::from("/opt/zed"))
        );
        assert_eq!(
            linux_system_install_prefix(Path::new("/home/user/.local/zed.app/libexec/zed-editor")),
            None
        );
        assert_eq!(
            linux_system_install_prefix(Path::new("/usr/lib/zed/zed-editor")),
            None
        );
    }

    #[gpui::test]
    async fn test_remote_server_release_url_resolves_from_fork_release(cx: &mut TestAppContext) {
        cx.background_executor.allow_parking();
        cx.update(|cx| {
            settings::init(cx);
            let current_version: Version = "1.30.0+stable.10.aaa".parse().unwrap();
            release_channel::init_test(current_version, ReleaseChannel::Stable, cx);

            let clock = Arc::new(FakeSystemClock::new());
            let fake_client_http = FakeHttpClient::create(|req| async move {
                if req.uri().path()
                    == format!("/{FORK_RELEASE_REPOSITORY}/releases/download/fork-v1.30.0/release.json")
                {
                    return Ok(Response::builder()
                        .status(200)
                        .body(
                            r#"{"version":"1.30.0+stable.10.aaa","tag":"fork-v1.30.0","assets":["zed-remote-server-linux-x86_64.gz","zed-remote-server-windows-x86_64.zip"]}"#.into(),
                        )
                        .unwrap());
                }
                Ok(Response::builder().status(404).body("".into()).unwrap())
            });
            let client = Client::new(clock, fake_client_http, cx);
            crate::init(client, cx);
        });

        let requested: Version = "1.30.0+stable.10.aaa".parse().unwrap();
        let linux_url = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some(requested.clone()),
            "linux",
            "x86_64",
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        assert_eq!(
            linux_url.as_deref(),
            Some(
                format!(
                    "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/fork-v1.30.0/zed-remote-server-linux-x86_64.gz"
                )
                .as_str()
            )
        );

        let windows_url = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some(requested.clone()),
            "windows",
            "x86_64",
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        assert_eq!(
            windows_url.as_deref(),
            Some(
                format!(
                    "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/fork-v1.30.0/zed-remote-server-windows-x86_64.zip"
                )
                .as_str()
            )
        );

        let missing_arch = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some(requested),
            "macos",
            "aarch64",
            &mut cx.to_async(),
        )
        .await;
        assert!(missing_arch.is_err());
    }

    #[gpui::test]
    async fn test_remote_server_release_url_falls_back_to_latest_for_custom_tag(
        cx: &mut TestAppContext,
    ) {
        cx.background_executor.allow_parking();
        cx.update(|cx| {
            settings::init(cx);
            let current_version: Version = "1.30.0+stable.10.aaa".parse().unwrap();
            release_channel::init_test(current_version, ReleaseChannel::Stable, cx);

            let clock = Arc::new(FakeSystemClock::new());
            let fake_client_http = FakeHttpClient::create(|req| async move {
                if req.uri().path()
                    == format!("/{FORK_RELEASE_REPOSITORY}/releases/latest/download/release.json")
                {
                    return Ok(Response::builder()
                        .status(200)
                        .body(
                            r#"{"version":"1.30.0+stable.10.aaa","tag":"fork-v1.30.0-custom","assets":["zed-remote-server-linux-aarch64.gz"]}"#.into(),
                        )
                        .unwrap());
                }
                Ok(Response::builder().status(404).body("".into()).unwrap())
            });
            let client = Client::new(clock, fake_client_http, cx);
            crate::init(client, cx);
        });

        let matching_url = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some("1.30.0+stable.10.aaa".parse().unwrap()),
            "linux",
            "aarch64",
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        assert_eq!(
            matching_url.as_deref(),
            Some(
                format!(
                    "https://github.com/{FORK_RELEASE_REPOSITORY}/releases/download/fork-v1.30.0-custom/zed-remote-server-linux-aarch64.gz"
                )
                .as_str()
            )
        );

        let mismatched = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some("1.28.0+stable.9.bbb".parse().unwrap()),
            "linux",
            "aarch64",
            &mut cx.to_async(),
        )
        .await;
        assert!(mismatched.is_err());
    }

    #[test]
    fn test_nightly_does_not_update_when_fetched_sha_is_same() {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(Some("a".to_string()));
        let mut installed_version = semver::Version::new(1, 0, 0);
        installed_version.build = semver::BuildMetadata::new("a").unwrap();
        let status = AutoUpdateStatus::Idle;
        let fetched_version = "1.0.0+a".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version,
            status,
        );

        assert_eq!(newer_version.unwrap(), None);
    }

    #[test]
    fn test_nightly_does_update_when_fetched_sha_is_not_same() {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(Some("a".to_string()));
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Idle;
        let fetched_version = "1.0.0+b".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.clone(),
            status,
        );

        assert_eq!(
            newer_version.unwrap(),
            Some(fetched_version.parse().unwrap())
        );
    }

    #[test]
    fn test_nightly_does_not_update_when_fetched_version_is_same_as_cached() {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(Some("a".to_string()));
        let mut installed_version = semver::Version::new(1, 0, 0);
        installed_version.build = semver::BuildMetadata::new("a").unwrap();
        let status = AutoUpdateStatus::Updated {
            version: "1.0.0+b".parse().unwrap(),
        };
        let fetched_version = "1.0.0+b".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version,
            status,
        );

        assert_eq!(newer_version.unwrap(), None);
    }

    #[test]
    fn test_nightly_does_update_when_fetched_sha_is_not_same_as_cached() {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(Some("a".to_string()));
        let mut installed_version = semver::Version::new(1, 0, 0);
        installed_version.build = semver::BuildMetadata::new("a").unwrap();
        let status = AutoUpdateStatus::Updated {
            version: "1.0.0+b".parse().unwrap(),
        };
        let fetched_version = "1.0.0+c".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.clone(),
            status,
        );

        assert_eq!(
            newer_version.unwrap(),
            Some(fetched_version.parse().unwrap())
        );
    }

    #[test]
    fn test_nightly_does_not_redownload_after_updating_to_fetched_version() {
        let release_channel = ReleaseChannel::Nightly;
        let installed_version = semver::Version::new(1, 0, 0);
        let fetched_version = "1.0.0+nightly.b".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            Ok(Some("a".to_string())),
            installed_version.clone(),
            fetched_version.clone(),
            AutoUpdateStatus::Idle,
        )
        .unwrap()
        .expect("a newer nightly version should be available");

        let next_check = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            Ok(Some("a".to_string())),
            installed_version,
            fetched_version,
            AutoUpdateStatus::Updated {
                version: newer_version,
            },
        );

        assert_eq!(next_check.unwrap(), None);
    }

    #[test]
    fn test_nightly_does_update_when_installed_versions_sha_cannot_be_retrieved() {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(None);
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Idle;
        let fetched_version = "1.0.0+a".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.clone(),
            status,
        );

        assert_eq!(
            newer_version.unwrap(),
            Some(fetched_version.parse().unwrap())
        );
    }

    #[test]
    fn test_nightly_does_not_update_when_cached_update_is_same_as_fetched_and_installed_versions_sha_cannot_be_retrieved()
     {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(None);
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Updated {
            version: "1.0.0+b".parse().unwrap(),
        };
        let fetched_version = "1.0.0+b".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version,
            status,
        );

        assert_eq!(newer_version.unwrap(), None);
    }

    #[test]
    fn test_nightly_does_update_when_cached_update_is_not_same_as_fetched_and_installed_versions_sha_cannot_be_retrieved()
     {
        let release_channel = ReleaseChannel::Nightly;
        let app_commit_sha = Ok(None);
        let installed_version = semver::Version::new(1, 0, 0);
        let status = AutoUpdateStatus::Updated {
            version: "1.0.0+b".parse().unwrap(),
        };
        let fetched_version = "1.0.0+c".to_string();

        let newer_version = AutoUpdater::check_if_fetched_version_is_newer(
            release_channel,
            app_commit_sha,
            installed_version,
            fetched_version.clone(),
            status,
        );

        assert_eq!(
            newer_version.unwrap(),
            Some(fetched_version.parse().unwrap())
        );
    }
}
