use client::telemetry;
use gpui::{App, AppContext as _, SemanticVersion, Task, Window};
use human_bytes::human_bytes;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use serde::Serialize;
use std::{env, fmt::Display};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Clone, Debug, Serialize)]
pub struct SystemSpecs {
    app_version: String,
    release_channel: &'static str,
    os_name: String,
    os_version: String,
    memory: u64,
    architecture: &'static str,
    commit_sha: Option<String>,
    bundle_type: Option<String>,
    gpu_specs: Option<String>,
}

impl SystemSpecs {
    pub fn new(window: &mut Window, cx: &mut App) -> Task<Self> {
        let app_version = AppVersion::global(cx).to_string();
        let release_channel = ReleaseChannel::global(cx);
        let os_name = telemetry::os_name();
        let system = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        let memory = system.total_memory();
        let architecture = env::consts::ARCH;
        let commit_sha = match release_channel {
            ReleaseChannel::Dev | ReleaseChannel::Nightly => {
                AppCommitSha::try_global(cx).map(|sha| sha.full())
            }
            _ => None,
        };
        let bundle_type = bundle_type();

        let gpu_specs = window.gpu_specs().map(|specs| {
            format!(
                "{} || {} || {}",
                specs.device_name, specs.driver_name, specs.driver_info
            )
        });

        cx.background_spawn(async move {
            let os_version = telemetry::os_version();
            SystemSpecs {
                app_version,
                release_channel: release_channel.display_name(),
                bundle_type,
                os_name,
                os_version,
                memory,
                architecture,
                commit_sha,
                gpu_specs,
            }
        })
    }

    pub fn new_stateless(
        app_version: SemanticVersion,
        app_commit_sha: Option<AppCommitSha>,
        release_channel: ReleaseChannel,
    ) -> Self {
        let os_name = telemetry::os_name();
        let os_version = telemetry::os_version();
        let system = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        let memory = system.total_memory();
        let architecture = env::consts::ARCH;
        let commit_sha = match release_channel {
            ReleaseChannel::Dev | ReleaseChannel::Nightly => app_commit_sha.map(|sha| sha.full()),
            _ => None,
        };
        let bundle_type = bundle_type();

        Self {
            app_version: app_version.to_string(),
            release_channel: release_channel.display_name(),
            os_name,
            os_version,
            memory,
            architecture,
            commit_sha,
            bundle_type,
            gpu_specs: try_determine_available_gpus(),
        }
    }
}

impl Display for SystemSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let os_information = format!("OS: {} {}", self.os_name, self.os_version);
        let app_version_information = format!(
            "Zed: v{} ({}) {}{}",
            self.app_version,
            match &self.commit_sha {
                Some(commit_sha) => format!("{} {}", self.release_channel, commit_sha),
                None => self.release_channel.to_string(),
            },
            if let Some(bundle_type) = &self.bundle_type {
                format!("({bundle_type})")
            } else {
                "".to_string()
            },
            if cfg!(debug_assertions) {
                "(Taylor's Version)"
            } else {
                ""
            },
        );
        let system_specs = [
            app_version_information,
            os_information,
            format!("Memory: {}", human_bytes(self.memory as f64)),
            format!("Architecture: {}", self.architecture),
        ]
        .into_iter()
        .chain(
            self.gpu_specs
                .as_ref()
                .map(|specs| format!("GPU: {}", specs)),
        )
        .collect::<Vec<String>>()
        .join("\n");

        write!(f, "{system_specs}")
    }
}

fn try_determine_available_gpus() -> Option<String> {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        std::process::Command::new("vulkaninfo")
            .args(&["--summary"])
            .output()
            .ok()
            .map(|output| {
                [
                    "<details><summary>`vulkaninfo --summary` output</summary>",
                    "",
                    "```",
                    String::from_utf8_lossy(&output.stdout).as_ref(),
                    "```",
                    "</details>",
                ]
                .join("\n")
            })
            .or(Some("Failed to run `vulkaninfo --summary`".to_string()))
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        None
    }
}

/// Returns value of `ZED_BUNDLE_TYPE` set at compiletime or else at runtime.
///
/// The compiletime value is used by flatpak since it doesn't seem to have a way to provide a
/// runtime environment variable.
///
/// The runtime value is used by snap since the Zed snaps use release binaries directly, and so
/// cannot have this baked in.
fn bundle_type() -> Option<String> {
    option_env!("ZED_BUNDLE_TYPE")
        .map(|bundle_type| bundle_type.to_string())
        .or_else(|| env::var("ZED_BUNDLE_TYPE").ok())
}
