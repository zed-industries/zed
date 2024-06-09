use gpui::AppContext;
use human_bytes::human_bytes;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use serde::Serialize;
use std::{env, fmt::Display};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Clone, Debug, Serialize)]
pub struct SystemSpecs {
    app_version: String,
    release_channel: &'static str,
    os_name: &'static str,
    os_version: Option<String>,
    memory: u64,
    architecture: &'static str,
    commit_sha: Option<String>,
}

impl SystemSpecs {
    pub fn new(cx: &AppContext) -> Self {
        let app_version = AppVersion::global(cx).to_string();
        let release_channel = ReleaseChannel::global(cx);
        let os_name = cx.app_metadata().os_name;
        let system = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        let memory = system.total_memory();
        let architecture = env::consts::ARCH;
        let os_version = cx
            .app_metadata()
            .os_version
            .map(|os_version| os_version.to_string());
        let commit_sha = match release_channel {
            ReleaseChannel::Dev | ReleaseChannel::Nightly => {
                AppCommitSha::try_global(cx).map(|sha| sha.0.clone())
            }
            _ => None,
        };

        SystemSpecs {
            app_version,
            release_channel: release_channel.display_name(),
            os_name,
            os_version,
            memory,
            architecture,
            commit_sha,
        }
    }
}

impl Display for SystemSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let os_information = match &self.os_version {
            Some(os_version) => format!("OS: {} {}", self.os_name, os_version),
            None => format!("OS: {}", self.os_name),
        };
        let app_version_information = format!(
            "Zed: v{} ({})",
            self.app_version,
            match &self.commit_sha {
                Some(commit_sha) => format!("{} {}", self.release_channel, commit_sha),
                None => self.release_channel.to_string(),
            }
        );
        let system_specs = [
            app_version_information,
            os_information,
            format!("Memory: {}", human_bytes(self.memory as f64)),
            format!("Architecture: {}", self.architecture),
        ]
        .into_iter()
        .collect::<Vec<String>>()
        .join("\n");

        write!(f, "{system_specs}")
    }
}
