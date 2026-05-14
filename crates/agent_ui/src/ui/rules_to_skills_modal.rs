//! Mini modal shown when the user clicks the title-bar Skills
//! announcement banner. Renders one of two flavours depending on the
//! persisted [`MigrationResult`]:
//!
//! * **No rules migrated** (new user, or an existing user who never
//!   touched Rules): a generic "Introducing Skills" intro that explains
//!   what Skills are and how to invoke them.
//! * **Rules migrated**: a per-destination summary of exactly which
//!   Rules ended up where (Skills directory, global AGENTS.md, top of
//!   AGENTS.md for customized built-ins), capped at three names per
//!   section with an "…and N more" overflow line.

use agent_skills::GLOBAL_SKILLS_DIR_DISPLAY;
use gpui::{
    DismissEvent, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
};
use paths::GLOBAL_AGENTS_FILE_DISPLAY;
use prompt_store::rules_to_skills_migration::{self, MigrationResult};
use ui::{
    AlertModal, Button, ButtonCommon, ButtonStyle, Clickable, KeyBinding, ListBulletItem, h_flex,
    prelude::*,
};
use workspace::{ModalView, Workspace};

/// Maximum number of rule names to list inline in the modal before
/// collapsing the rest into an "…and N more" line.
const MAX_LISTED_NAMES: usize = 3;

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
        let result = rules_to_skills_migration::migration_result().unwrap_or_default();

        let mut modal = AlertModal::new("rules-to-skills-migration")
            .width(rems(28.))
            .key_context("RulesToSkillsModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| this.dismiss(cx)))
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| this.dismiss(cx)));

        if result.is_empty() {
            modal = render_introducing_skills(modal);
        } else {
            modal = render_migration_summary(modal, &result);
        }

        // Both flavours close with the same invocation instructions.
        modal = modal.child(Label::new(
            "To include a Skill in a prompt, type /skill-name (or @-mention it).",
        ));

        modal.footer(
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

/// Render the modal body for users who had no Rules to migrate — a
/// generic introduction to the Skills feature.
fn render_introducing_skills(modal: AlertModal) -> AlertModal {
    modal.title("Introducing Skills").child(Label::new(format!(
        "Skills are reusable instructions for the agent, stored as Markdown files \
         under {GLOBAL_SKILLS_DIR_DISPLAY}/<name>/SKILL.md."
    )))
}

/// Render the modal body for users whose Rules were migrated, listing
/// each destination's contents (capped at [`MAX_LISTED_NAMES`] per
/// section).
fn render_migration_summary(mut modal: AlertModal, result: &MigrationResult) -> AlertModal {
    modal = modal.title("Skills have replaced Rules");

    if !result.skill_names.is_empty() {
        modal = modal.child(Label::new(format!(
            "These Rules have been migrated to Skills in {GLOBAL_SKILLS_DIR_DISPLAY}:"
        )));
        modal = add_bulleted_names(modal, &result.skill_names);
    }

    if !result.agents_md_names.is_empty() {
        modal = modal.child(Label::new(format!(
            "These Default Rules were added to {GLOBAL_AGENTS_FILE_DISPLAY}:"
        )));
        modal = add_bulleted_names(modal, &result.agents_md_names);
    }

    if !result.customized_builtins.is_empty() {
        modal = modal.child(Label::new(customized_builtins_sentence(
            &result.customized_builtins,
        )));
    }

    modal
}

/// Append up to [`MAX_LISTED_NAMES`] bullet items naming individual
/// rules, plus a final "…and N more" bullet if the list is longer.
fn add_bulleted_names(mut modal: AlertModal, names: &[String]) -> AlertModal {
    for name in names.iter().take(MAX_LISTED_NAMES) {
        modal = modal.child(ListBulletItem::new(name.clone()));
    }
    if names.len() > MAX_LISTED_NAMES {
        let extras = names.len() - MAX_LISTED_NAMES;
        let label = if extras == 1 {
            "…and 1 more".to_string()
        } else {
            format!("…and {extras} more")
        };
        modal = modal.child(ListBulletItem::new(label));
    }
    modal
}

/// Build the sentence describing any customized built-in prompts that
/// were prepended to AGENTS.md. Singular wording for the common
/// one-built-in case; comma-joined for the (currently hypothetical)
/// multi-built-in case.
fn customized_builtins_sentence(names: &[String]) -> String {
    debug_assert!(
        !names.is_empty(),
        "caller should only invoke this for a non-empty list"
    );
    if names.len() == 1 {
        format!(
            "Your customization of the {name} built-in prompt has been added to the top of \
             {GLOBAL_AGENTS_FILE_DISPLAY}.",
            name = names[0],
        )
    } else {
        format!(
            "Your customizations of these built-in prompts have been added to the top of \
             {GLOBAL_AGENTS_FILE_DISPLAY}: {joined}.",
            joined = names.join(", "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn customized_builtins_sentence_uses_singular_wording_for_one_item() {
        let sentence = customized_builtins_sentence(&["Commit message".to_string()]);
        assert!(sentence.contains("Your customization of the Commit message"));
        assert!(sentence.contains("built-in prompt has been added"));
        assert!(sentence.contains(GLOBAL_AGENTS_FILE_DISPLAY));
    }

    #[test]
    fn customized_builtins_sentence_uses_plural_wording_for_multiple_items() {
        let sentence = customized_builtins_sentence(&[
            "Commit message".to_string(),
            "Future Built-in".to_string(),
        ]);
        assert!(sentence.contains("customizations of these built-in prompts"));
        assert!(sentence.contains("Commit message, Future Built-in"));
    }
}
