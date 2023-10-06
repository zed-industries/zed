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

    pub fn url_scheme(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "zed-dev://",
            ReleaseChannel::Preview => "zed-preview://",
            ReleaseChannel::Stable => "zed://",
        }
    }

    pub fn link_prefix(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "https://zed.dev/dev/",
            ReleaseChannel::Preview => "https://zed.dev/preview/",
            ReleaseChannel::Stable => "https://zed.dev/",
        }
    }
}

pub fn parse_zed_link(link: &str) -> Option<&str> {
    for release in [
        ReleaseChannel::Dev,
        ReleaseChannel::Preview,
        ReleaseChannel::Stable,
    ] {
        if let Some(stripped) = link.strip_prefix(release.link_prefix()) {
            return Some(stripped);
        }
        if let Some(stripped) = link.strip_prefix(release.url_scheme()) {
            return Some(stripped);
        }
    }
    None
}
