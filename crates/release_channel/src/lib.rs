use gpui::{AppContext, Global, SemanticVersion};
use once_cell::sync::Lazy;
use std::env;

#[doc(hidden)]
static RELEASE_CHANNEL_NAME: Lazy<String> = if cfg!(debug_assertions) {
    Lazy::new(|| {
        env::var("ZED_RELEASE_CHANNEL")
            .unwrap_or_else(|_| include_str!("../../zed/RELEASE_CHANNEL").trim().to_string())
    })
} else {
    Lazy::new(|| include_str!("../../zed/RELEASE_CHANNEL").trim().to_string())
};

#[doc(hidden)]
pub static RELEASE_CHANNEL: Lazy<ReleaseChannel> =
    Lazy::new(|| match RELEASE_CHANNEL_NAME.as_str() {
        "dev" => ReleaseChannel::Dev,
        "nightly" => ReleaseChannel::Nightly,
        "preview" => ReleaseChannel::Preview,
        "stable" => ReleaseChannel::Stable,
        _ => panic!("invalid release channel {}", *RELEASE_CHANNEL_NAME),
    });

#[derive(Clone)]
pub struct AppCommitSha(pub String);

struct GlobalAppCommitSha(AppCommitSha);

impl Global for GlobalAppCommitSha {}

impl AppCommitSha {
    pub fn try_global(cx: &AppContext) -> Option<AppCommitSha> {
        cx.try_global::<GlobalAppCommitSha>()
            .map(|sha| sha.0.clone())
    }

    pub fn set_global(sha: AppCommitSha, cx: &mut AppContext) {
        cx.set_global(GlobalAppCommitSha(sha))
    }
}

struct GlobalAppVersion(SemanticVersion);

impl Global for GlobalAppVersion {}

pub struct AppVersion;

impl AppVersion {
    pub fn init(pkg_version: &str, cx: &mut AppContext) {
        let version = if let Ok(from_env) = env::var("ZED_APP_VERSION") {
            from_env.parse().expect("invalid ZED_APP_VERSION")
        } else {
            cx.app_metadata()
                .app_version
                .unwrap_or_else(|| pkg_version.parse().expect("invalid version in Cargo.toml"))
        };
        cx.set_global(GlobalAppVersion(version))
    }

    pub fn global(cx: &AppContext) -> SemanticVersion {
        cx.global::<GlobalAppVersion>().0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReleaseChannel {
    #[default]
    Dev,
    Nightly,
    Preview,
    Stable,
}

struct GlobalReleaseChannel(ReleaseChannel);

impl Global for GlobalReleaseChannel {}

pub fn init(pkg_version: &str, cx: &mut AppContext) {
    AppVersion::init(pkg_version, cx);
    cx.set_global(GlobalReleaseChannel(*RELEASE_CHANNEL))
}

impl ReleaseChannel {
    pub fn global(cx: &AppContext) -> Self {
        cx.global::<GlobalReleaseChannel>().0
    }

    pub fn try_global(cx: &AppContext) -> Option<Self> {
        cx.try_global::<GlobalReleaseChannel>()
            .map(|channel| channel.0)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "Zed Dev",
            ReleaseChannel::Nightly => "Zed Nightly",
            ReleaseChannel::Preview => "Zed Preview",
            ReleaseChannel::Stable => "Zed",
        }
    }

    pub fn dev_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "dev",
            ReleaseChannel::Nightly => "nightly",
            ReleaseChannel::Preview => "preview",
            ReleaseChannel::Stable => "stable",
        }
    }

    pub fn url_scheme(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "zed-dev://",
            ReleaseChannel::Nightly => "zed-nightly://",
            ReleaseChannel::Preview => "zed-preview://",
            ReleaseChannel::Stable => "zed://",
        }
    }

    pub fn link_prefix(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "https://zed.dev/dev/",
            ReleaseChannel::Nightly => "https://zed.dev/nightly/",
            ReleaseChannel::Preview => "https://zed.dev/preview/",
            ReleaseChannel::Stable => "https://zed.dev/",
        }
    }

    pub fn release_query_param(&self) -> Option<&'static str> {
        match self {
            Self::Dev => None,
            Self::Nightly => Some("nightly=1"),
            Self::Preview => Some("preview=1"),
            Self::Stable => None,
        }
    }
}

pub fn parse_zed_link(link: &str) -> Option<&str> {
    for release in [
        ReleaseChannel::Dev,
        ReleaseChannel::Nightly,
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
