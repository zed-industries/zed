use std::{env, fmt::Display};

use gpui::AppContext;
use util::channel::ReleaseChannel;

pub struct SystemSpecs {
    os_name: &'static str,
    os_version: Option<String>,
    app_version: &'static str,
    release_channel: &'static str,
    architecture: &'static str,
}

impl SystemSpecs {
    pub fn new(cx: &AppContext) -> Self {
        let platform = cx.platform();

        SystemSpecs {
            os_name: platform.os_name(),
            os_version: platform
                .os_version()
                .ok()
                .map(|os_version| os_version.to_string()),
            app_version: env!("CARGO_PKG_VERSION"),
            release_channel: cx.global::<ReleaseChannel>().dev_name(),
            architecture: env::consts::ARCH,
        }
    }
}

impl Display for SystemSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let os_information = match &self.os_version {
            Some(os_version) => format!("OS: {} {}", self.os_name, os_version),
            None => format!("OS: {}", self.os_name),
        };
        let system_specs = [
            os_information,
            format!("Zed: {} ({})", self.app_version, self.release_channel),
            format!("Architecture: {}", self.architecture),
        ]
        .join("\n");

        write!(f, "{system_specs}")
    }
}
