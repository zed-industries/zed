use agent_settings::AgentSettings;
use gpui::{App, Window};
use settings::Settings;
use ui::IconName;
use zed_actions::assistant::InlineAssist;

use super::{
    QuickActionBarItem, QuickActionButton, QuickActionElement, QuickActionTarget, VisibilityTrigger,
};

pub(super) struct InlineAssistButton;

impl QuickActionBarItem for InlineAssistButton {
    type Context = ();
    const ID: &'static str = "toggle-inline-assistant";
    const TRIGGERS: &'static [VisibilityTrigger] = &[VisibilityTrigger::Settings];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<()> {
        target.editor()?;
        let agent_settings = AgentSettings::get_global(cx);
        (agent_settings.enabled(cx) && agent_settings.button).then_some(())
    }

    fn render(&self, _: &(), _window: &mut Window, _cx: &mut App) -> QuickActionElement {
        QuickActionElement::Button(
            QuickActionButton::new(IconName::ZedAssistant, "Inline Assist", |window, cx| {
                window.dispatch_action(Box::new(InlineAssist::default()), cx);
            })
            .action(Box::new(InlineAssist::default())),
        )
    }
}
