use client::ZED_APP_VERSION;
use gpui::AppContext;
use human_bytes::human_bytes;
use serde::Serialize;
use std::{env, fmt::Display};
use sysinfo::{RefreshKind, System, SystemExt};
use util::channel::ReleaseChannel;

#[derive(Clone, Debug, Serialize)]
pub struct SystemSpecs {
    app_version: Option<String>,
    release_channel: &'static str,
    os_name: &'static str,
    os_version: Option<String>,
    memory: u64,
    architecture: &'static str,
}

impl SystemSpecs {
    pub fn new(cx: &AppContext) -> Self {
        let app_version = ZED_APP_VERSION
            .or_else(|| cx.app_metadata().app_version)
            .map(|v| v.to_string());
        let release_channel = cx.global::<ReleaseChannel>().display_name();
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
        let app_version_information = self
            .app_version
            .as_ref()
            .map(|app_version| format!("Zed: v{} ({})", app_version, self.release_channel));
        let system_specs = [
            app_version_information,
            Some(os_information),
            Some(format!("Memory: {}", human_bytes(self.memory as f64))),
            Some(format!("Architecture: {}", self.architecture)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");

        write!(f, "{system_specs}")
    }
}
