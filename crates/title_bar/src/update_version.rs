use std::sync::Arc;

use anyhow::anyhow;
use auto_update::{AutoUpdateStatus, AutoUpdater, UpdateCheckType};
use gpui::{Empty, Render};
use semver::Version;
use ui::{Tooltip, UpdateButton, prelude::*};

pub struct UpdateVersion {
    status: AutoUpdateStatus,
    update_check_type: UpdateCheckType,
    dismissed_status: Option<AutoUpdateStatus>,
}

impl UpdateVersion {
    pub fn new(cx: &mut Context<Self>) -> Self {
        if let Some(auto_updater) = AutoUpdater::get(cx) {
            cx.observe(&auto_updater, |this, auto_update, cx| {
                let auto_update = auto_update.read(cx);
                this.status = auto_update.status();
                this.update_check_type = auto_update.update_check_type();
                this.dismissed_status = auto_update.dismissed_status();
                cx.notify();
            })
            .detach();
            Self {
                status: auto_updater.read(cx).status(),
                update_check_type: UpdateCheckType::Automatic,
                dismissed_status: auto_updater.read(cx).dismissed_status(),
            }
        } else {
            Self {
                status: AutoUpdateStatus::Idle,
                update_check_type: UpdateCheckType::Automatic,
                dismissed_status: None,
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
        self.dismissed_status = None;
        cx.notify()
    }

    pub fn show_update_in_menu_bar(&self) -> bool {
        self.is_dismissed() && self.status.is_updated()
    }

    fn is_dismissed(&self) -> bool {
        self.dismissed_status.as_ref() == Some(&self.status)
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissed_status = Some(self.status.clone());
        if let Some(auto_updater) = AutoUpdater::get(cx) {
            let status = self.status.clone();
            auto_updater.update(cx, |auto_updater, cx| {
                auto_updater.dismiss_status(status, cx)
            });
        }
        cx.notify()
    }

    fn version_tooltip_message(version: &Version) -> String {
        UpdateButton::version_tooltip_message(version)
    }
}

impl Render for UpdateVersion {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_dismissed() {
            return Empty.into_any_element();
        }
        match &self.status {
            AutoUpdateStatus::Checking if self.update_check_type.is_manual() => {
                UpdateButton::checking().into_any_element()
            }
            AutoUpdateStatus::Downloading { version, progress } => {
                let rendered_version = version.clone();
                let tooltip = Tooltip::element(move |_, cx| {
                    let status = AutoUpdater::get(cx).map(|updater| updater.read(cx).status());
                    let message = match &status {
                        Some(AutoUpdateStatus::Downloading { version, progress }) => {
                            UpdateButton::downloading_tooltip_message(version, *progress)
                        }
                        _ => Self::version_tooltip_message(&rendered_version),
                    };
                    Label::new(message).into_any_element()
                });
                UpdateButton::downloading(*progress)
                    .tooltip_fn(tooltip)
                    .into_any_element()
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
                    .on_dismiss(cx.listener(|this, _, _window, cx| this.dismiss(cx)))
                    .into_any_element()
            }
            AutoUpdateStatus::Errored { error } => {
                let error_str = error.to_string();
                UpdateButton::errored(error_str)
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::OpenLog), cx);
                    })
                    .on_dismiss(cx.listener(|this, _, _window, cx| this.dismiss(cx)))
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

    #[test]
    fn test_downloading_tooltip_message() {
        let version = Version::new(1, 0, 0);

        let message = UpdateButton::downloading_tooltip_message(&version, None);
        assert_eq!(message, "Update to Version: 1.0.0");

        let message = UpdateButton::downloading_tooltip_message(&version, Some(0.454));
        assert_eq!(message, "Update to Version: 1.0.0 (45% downloaded)");

        let message = UpdateButton::downloading_tooltip_message(&version, Some(1.5));
        assert_eq!(message, "Update to Version: 1.0.0 (100% downloaded)");
    }
}
