use gpui::Pixels;
use settings::Settings;
use ui::px;
use workspace::dock::DockPosition;

#[derive(Debug)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Debug)]
pub struct NotificationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

impl Settings for CollaborationPanelSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let panel = content.collaboration_panel.as_ref().unwrap();

        Self {
            button: panel.button.unwrap(),
            dock: panel.dock.unwrap().into(),
            default_width: panel.default_width.map(px).unwrap(),
        }
    }
}

impl Settings for NotificationPanelSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let panel = content.notification_panel.as_ref().unwrap();
        return Self {
            button: panel.button.unwrap(),
            dock: panel.dock.unwrap().into(),
            default_width: panel.default_width.map(px).unwrap(),
        };
    }
}
