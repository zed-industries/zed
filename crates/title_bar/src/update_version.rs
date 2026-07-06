use std::sync::Arc;

use anyhow::anyhow;
use auto_update::{AutoUpdateStatus, AutoUpdater, UpdateCheckType};
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
                cx.notify();
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
                version: Version::new(1, 99, 0),
                progress: Some(0.5),
            },
            AutoUpdateStatus::Downloading { .. } => AutoUpdateStatus::Installing {
                version: Version::new(1, 99, 0),
            },
            AutoUpdateStatus::Installing { .. } => AutoUpdateStatus::Updated {
                version: Version::new(1, 99, 0),
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

    fn version_tooltip_message(version: &Version) -> String {
        format!("Update to Version: {version}")
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
            AutoUpdateStatus::Downloading { version, progress } => {
                let version = Self::version_tooltip_message(version);
                UpdateButton::downloading(version, *progress).into_any_element()
            }
            AutoUpdateStatus::Installing { version } => {
                let version = Self::version_tooltip_message(version);
                UpdateButton::installing(version).into_any_element()
            }
            AutoUpdateStatus::Updated { version } => {
                let version = Self::version_tooltip_message(version);
                UpdateButton::updated(version)
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
            AutoUpdateStatus::Idle | AutoUpdateStatus::Checking { .. } => Empty.into_any_element(),
        }
    }
}
#[cfg(test)]
mod tests {
    use semver::Version;

    use super::*;

    #[test]
    fn test_version_tooltip_message() {
        let message = UpdateVersion::version_tooltip_message(&Version::new(1, 0, 0));

        assert_eq!(message, "Update to Version: 1.0.0");

        let message = UpdateVersion::version_tooltip_message(
            &"1.0.0+nightly.14d9a4189f058d8736339b06ff2340101eaea5af"
                .parse()
                .unwrap(),
        );

        assert_eq!(
            message,
            "Update to Version: 1.0.0+nightly.14d9a4189f058d8736339b06ff2340101eaea5af"
        );
    }
}
