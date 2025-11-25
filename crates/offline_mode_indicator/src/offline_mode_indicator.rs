use gpui::{actions, Context, Render, Subscription, Window};
use offline_mode::OfflineModeSetting;
use settings::{Settings, SettingsStore};
use ui::{prelude::*, Tooltip};
use workspace::{item::ItemHandle, StatusItemView};

actions!(
    offline_mode_indicator,
    [
        /// Toggle offline mode on/off
        ToggleOfflineMode
    ]
);

pub struct OfflineModeIndicator {
    _settings_subscription: Subscription,
}

impl OfflineModeIndicator {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe_global::<SettingsStore>(|_, cx| {
            cx.notify();
        });

        Self {
            _settings_subscription: subscription,
        }
    }
}

impl Render for OfflineModeIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_offline = OfflineModeSetting::get_global(cx).0;

        let (icon_name, icon_color, tooltip_text) = if is_offline {
            (
                IconName::XCircle,
                Color::Warning,
                "Offline Mode - Network features disabled",
            )
        } else {
            (IconName::Circle, Color::Success, "Online")
        };

        IconButton::new("offline-indicator", icon_name)
            .icon_size(IconSize::Small)
            .icon_color(icon_color)
            .on_click(move |_, _window, cx| {
                let current_value = OfflineModeSetting::get_global(cx).0;
                let new_value = !current_value;
                SettingsStore::update(cx, |store, _| {
                    store.override_global(OfflineModeSetting(new_value));
                });
            })
            .tooltip(move |window, cx| Tooltip::text(tooltip_text)(window, cx))
    }
}

impl StatusItemView for OfflineModeIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offline_mode_setting() {
        let online_setting = OfflineModeSetting(false);
        assert!(!online_setting.0, "Should be in online mode");

        let offline_setting = OfflineModeSetting(true);
        assert!(offline_setting.0, "Should be in offline mode");
    }

    #[test]
    fn test_icon_selection_logic() {
        let is_offline = false;
        let (icon_name, icon_color) = if is_offline {
            (IconName::XCircle, Color::Warning)
        } else {
            (IconName::Circle, Color::Success)
        };
        assert_eq!(icon_name, IconName::Circle);
        assert_eq!(icon_color, Color::Success);

        let is_offline = true;
        let (icon_name, icon_color) = if is_offline {
            (IconName::XCircle, Color::Warning)
        } else {
            (IconName::Circle, Color::Success)
        };
        assert_eq!(icon_name, IconName::XCircle);
        assert_eq!(icon_color, Color::Warning);
    }
}
