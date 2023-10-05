use std::env;

use lazy_static::lazy_static;
use url::Url;

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

    pub static ref URL_SCHEME_PREFIX: String = match RELEASE_CHANNEL_NAME.as_str() {
        "dev" => "zed-dev:/",
        "preview" => "zed-preview:/",
        "stable" => "zed:/",
        // NOTE: this must be kept in sync with osx_url_schemes in Cargo.toml and with https://zed.dev.
        _ => unreachable!(),
    }.to_string();
    pub static ref LINK_PREFIX: Url = Url::parse(match RELEASE_CHANNEL_NAME.as_str() {
        "dev" => "http://localhost:3000/dev/",
        "preview" => "https://zed.dev/preview/",
        "stable" => "https://zed.dev/",
        // NOTE: this must be kept in sync with https://zed.dev.
        _ => unreachable!(),
    })
    .unwrap();
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
