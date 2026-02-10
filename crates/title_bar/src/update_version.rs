use std::sync::Arc;

use anyhow::anyhow;
use auto_update::{AutoUpdateStatus, AutoUpdater, UpdateCheckType, VersionCheckType};
use gpui::{Empty, Render};
use semver::Version;
use ui::{UpdateButton, prelude::*};

pub struct UpdateVersion {
    status: AutoUpdateStatus,
    update_check_type: UpdateCheckType,
    dismissed: bool,
}

impl UpdateVersion {
    pub fn new(cx: &mut Context<Self>) -> Self {
        if let Some(auto_updater) = AutoUpdater::get(cx) {
            cx.observe(&auto_updater, |this, auto_update, cx| {
                this.status = auto_update.read(cx).status();
                this.update_check_type = auto_update.read(cx).update_check_type();
                if this.status.is_updated() {
                    this.dismissed = false;
                }
            })
            .detach();
            Self {
                status: auto_updater.read(cx).status(),
                update_check_type: UpdateCheckType::Automatic,
                dismissed: false,
            }
        } else {
            Self {
                status: AutoUpdateStatus::Idle,
                update_check_type: UpdateCheckType::Automatic,
                dismissed: false,
            }
        }
    }

    pub fn update_simulation(&mut self, cx: &mut Context<Self>) {
        let next_state = match self.status {
            AutoUpdateStatus::Idle => AutoUpdateStatus::Checking,
            AutoUpdateStatus::Checking => AutoUpdateStatus::Downloading {
                version: VersionCheckType::Semantic(Version::new(1, 99, 0)),
            },
            AutoUpdateStatus::Downloading { .. } => AutoUpdateStatus::Installing {
                version: VersionCheckType::Semantic(Version::new(1, 99, 0)),
            },
            AutoUpdateStatus::Installing { .. } => AutoUpdateStatus::Updated {
                version: VersionCheckType::Semantic(Version::new(1, 99, 0)),
            },
            AutoUpdateStatus::Updated { .. } => AutoUpdateStatus::Errored {
                error: Arc::new(anyhow!("Network timeout")),
            },
            AutoUpdateStatus::Errored { .. } => AutoUpdateStatus::Idle,
        };

        self.status = next_state;
        self.update_check_type = UpdateCheckType::Manual;
        self.dismissed = false;
        cx.notify()
    }

    pub fn show_update_in_menu_bar(&self) -> bool {
        self.dismissed && self.status.is_updated()
    }

    fn version_tooltip_message(version: &VersionCheckType) -> String {
        format!("Version: {}", {
            match version {
                VersionCheckType::Sha(sha) => format!("{}…", sha.short()),
                VersionCheckType::Semantic(semantic_version) => semantic_version.to_string(),
            }
        })
    }
}

impl Render for UpdateVersion {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.dismissed {
            return Empty.into_any_element();
        }
        match &self.status {
            AutoUpdateStatus::Checking if self.update_check_type.is_manual() => {
                UpdateButton::checking().into_any_element()
            }
            AutoUpdateStatus::Downloading { version } if self.update_check_type.is_manual() => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::downloading(tooltip).into_any_element()
            }
            AutoUpdateStatus::Installing { version } if self.update_check_type.is_manual() => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::installing(tooltip).into_any_element()
            }
            AutoUpdateStatus::Updated { version } => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::updated(tooltip)
                    .on_click(|_, _, cx| {
                        workspace::reload(cx);
                    })
                    .on_dismiss(cx.listener(|this, _, _window, cx| {
                        this.dismissed = true;
                        cx.notify()
                    }))
                    .into_any_element()
            }
            AutoUpdateStatus::Errored { error } => {
                let error_str = error.to_string();
                UpdateButton::errored(error_str)
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::OpenLog), cx);
                    })
                    .on_dismiss(cx.listener(|this, _, _window, cx| {
                        this.dismissed = true;
                        cx.notify()
                    }))
                    .into_any_element()
            }
            AutoUpdateStatus::Idle
            | AutoUpdateStatus::Checking { .. }
            | AutoUpdateStatus::Downloading { .. }
            | AutoUpdateStatus::Installing { .. } => Empty.into_any_element(),
        }
    }
}
#[cfg(test)]
mod tests {
    use auto_update::VersionCheckType;
    use release_channel::AppCommitSha;
    use semver::Version;

    use super::*;

    #[test]
    fn test_version_tooltip_message() {
        let message = UpdateVersion::version_tooltip_message(&VersionCheckType::Semantic(
            Version::new(1, 0, 0),
        ));

        assert_eq!(message, "Version: 1.0.0");

        let message = UpdateVersion::version_tooltip_message(&VersionCheckType::Sha(
            AppCommitSha::new("14d9a4189f058d8736339b06ff2340101eaea5af".to_string()),
        ));

        assert_eq!(message, "Version: 14d9a41…");
    }
}
