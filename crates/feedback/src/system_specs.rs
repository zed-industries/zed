use gpui::AppContext;
use human_bytes::human_bytes;
use release_channel::{AppVersion, ReleaseChannel};
use serde::Serialize;
use std::{env, fmt::Display};
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Clone, Debug, Serialize)]
pub struct SystemSpecs {
    app_version: String,
    release_channel: &'static str,
    os_name: &'static str,
    os_version: Option<String>,
    memory: u64,
    architecture: &'static str,
}

impl SystemSpecs {
    pub fn new(cx: &AppContext) -> Self {
        let app_version = AppVersion::global(cx).to_string();
        let release_channel = ReleaseChannel::global(cx).display_name();
        let os_name = cx.app_metadata().os_name;
        let system = System::new_with_specifics(RefreshKind::new().with_memory());
        let memory = system.total_memory();
        let architecture = env::consts::ARCH;
        let os_version = cx
            .app_metadata()
            .os_version
            .map(|os_version| os_version.to_string());

        SystemSpecs {
            app_version,
            release_channel,
            os_name,
            os_version,
            memory,
            architecture,
        }
    }
}

impl Display for SystemSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let os_information = match &self.os_version {
            Some(os_version) => format!("OS: {} {}", self.os_name, os_version),
            None => format!("OS: {}", self.os_name),
        };
        let app_version_information =
            format!("Zed: v{} ({})", self.app_version, self.release_channel);
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
