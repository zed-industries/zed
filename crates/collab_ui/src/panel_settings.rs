use gpui::Pixels;
use settings::Settings;
use ui::px;
use util::MergeFrom as _;
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

#[derive(Clone, Default, Debug)]
// todo! are these settings even relevant any more?
pub struct MessageEditorSettings {
    /// Whether to automatically replace emoji shortcodes with emoji characters.
    /// For example: typing `:wave:` gets replaced with `👋`.
    ///
    /// Default: false
    pub auto_replace_emoji_shortcode: bool,
}

impl Settings for CollaborationPanelSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let panel = content.collaboration_panel.as_ref().unwrap();

        Self {
            button: panel.button.unwrap(),
            dock: panel.dock.unwrap().into(),
            default_width: panel.default_width.map(px).unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut ui::App) {
        if let Some(panel) = content.collaboration_panel.as_ref() {
            self.button.merge_from(&panel.button);
            self.default_width
                .merge_from(&panel.default_width.map(Pixels::from));
            self.dock.merge_from(&panel.dock.map(Into::into));
        }
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _content: &mut settings::SettingsContent,
    ) {
    }
}

impl Settings for NotificationPanelSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let panel = content.notification_panel.as_ref().unwrap();
        return Self {
            button: panel.button.unwrap(),
            dock: panel.dock.unwrap().into(),
            default_width: panel.default_width.map(px).unwrap(),
        };
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut ui::App) {
        let Some(panel) = content.notification_panel.as_ref() else {
            return;
        };
        self.button.merge_from(&panel.button);
        self.dock.merge_from(&panel.dock.map(Into::into));
        self.default_width.merge_from(&panel.default_width.map(px));
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _current: &mut settings::SettingsContent,
    ) {
    }
}

impl Settings for MessageEditorSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let messages = content.message_editor.as_ref().unwrap();
        Self {
            auto_replace_emoji_shortcode: messages.auto_replace_emoji_shortcode.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut ui::App) {
        let Some(messages) = content.message_editor.as_ref() else {
            return;
        };
        self.auto_replace_emoji_shortcode
            .merge_from(&messages.auto_replace_emoji_shortcode);
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _current: &mut settings::SettingsContent,
    ) {
    }
}
