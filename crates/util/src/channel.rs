use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref RELEASE_CHANNEL_NAME: String = if cfg!(debug_assertions) {
        env::var("ZED_RELEASE_CHANNEL")
            .unwrap_or_else(|_| include_str!("../../zed/RELEASE_CHANNEL").to_string())
    } else {
        include_str!("../../zed/RELEASE_CHANNEL").to_string()
    };
    pub static ref RELEASE_CHANNEL: ReleaseChannel = match RELEASE_CHANNEL_NAME.as_str() {
        "dev" => ReleaseChannel::Dev,
        "preview" => ReleaseChannel::Preview,
        "stable" => ReleaseChannel::Stable,
        _ => panic!("invalid release channel {}", *RELEASE_CHANNEL_NAME),
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum ReleaseChannel {
    #[default]
    Dev,
    Preview,
    Stable,
}

impl ReleaseChannel {
    pub fn display_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "Zed Dev",
            ReleaseChannel::Preview => "Zed Preview",
            ReleaseChannel::Stable => "Zed",
        }
    }

    pub fn dev_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "dev",
            ReleaseChannel::Preview => "preview",
            ReleaseChannel::Stable => "stable",
        }
    }
}
