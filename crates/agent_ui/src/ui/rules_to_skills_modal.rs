//! Mini modal shown when the user clicks the title-bar "Skills have
//! replaced Rules" announcement banner. Explains the one-time migration
//! of non-Default Rules to global Agent Skills.

use agent_skills::GLOBAL_SKILLS_DIR_DISPLAY;
use gpui::{
    DismissEvent, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
};
use ui::{
    AlertModal, Button, ButtonCommon, ButtonStyle, Clickable, KeyBinding, h_flex, prelude::*,
};
use workspace::{ModalView, Workspace};

pub struct RulesToSkillsModal {
    focus_handle: FocusHandle,
}

impl RulesToSkillsModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        workspace.toggle_modal(window, cx, |_window, cx| Self {
            focus_handle: cx.focus_handle(),
        });
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Focusable for RulesToSkillsModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for RulesToSkillsModal {}

impl ModalView for RulesToSkillsModal {}

impl Render for RulesToSkillsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        AlertModal::new("rules-to-skills-migration")
            .width(rems(28.))
            .key_context("RulesToSkillsModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| this.dismiss(cx)))
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| this.dismiss(cx)))
            .title("Skills have replaced Rules")
            .child(Label::new(format!(
                "Any Rules (not a .rules file, but rather Zed's old Rules \
                 feature) you had previously have been migrated to Skills \
                 in your {GLOBAL_SKILLS_DIR_DISPLAY} directory."
            )))
            .child(Label::new(
                "To include a Skill in a prompt, type /skill-name (in addition \
                 to @-mentioning it).",
            ))
            .footer(
                h_flex().p_3().items_center().justify_end().child(
                    Button::new("got-it", "Got it")
                        .style(ButtonStyle::Filled)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.dismiss(cx);
                            cx.stop_propagation();
                        })),
                ),
            )
    }
}
