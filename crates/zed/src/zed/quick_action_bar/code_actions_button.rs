use editor::Editor;
use editor::actions::{CodeActionSource, ToggleCodeActions};
use editor::code_context_menus::{CodeContextMenu, ContextMenuOrigin};
use gpui::{App, Entity, Window};
use ui::IconName;

use super::{
    QuickActionBarItem, QuickActionButton, QuickActionElement, QuickActionTarget, VisibilityTrigger,
};

const MAX_CODE_ACTION_MENU_LINES: u32 = 16;

pub(super) struct CodeActionsButton;

impl QuickActionBarItem for CodeActionsButton {
    type Context = Entity<Editor>;
    const ID: &'static str = "toggle-code-actions";
    const TRIGGERS: &'static [VisibilityTrigger] =
        &[VisibilityTrigger::Settings, VisibilityTrigger::Editor];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<Entity<Editor>> {
        let editor = target.editor()?;
        editor
            .read(cx)
            .code_actions_enabled_for_toolbar(cx)
            .then(|| editor.clone())
    }

    fn render(
        &self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) -> QuickActionElement {
        let has_available_code_actions = editor.read(cx).has_available_code_actions_for_selection();
        let is_deployed = {
            let menu_ref = editor.read(cx).context_menu().borrow();
            let code_action_menu = menu_ref
                .as_ref()
                .filter(|menu| matches!(menu, CodeContextMenu::CodeActions(..)));
            code_action_menu
                .as_ref()
                .is_some_and(|menu| matches!(menu.origin(), ContextMenuOrigin::QuickActionBar))
        };
        let popup = is_deployed
            .then(|| {
                editor.update(cx, |editor, cx| {
                    editor.render_context_menu(MAX_CODE_ACTION_MENU_LINES, window, cx)
                })
            })
            .flatten();

        let tooltip = if has_available_code_actions {
            "Code Actions"
        } else {
            "No Code Actions Available"
        };

        let editor = editor.clone();
        QuickActionElement::Button(
            QuickActionButton::new(IconName::BoltOutlined, tooltip, move |window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.toggle_code_actions(
                        &ToggleCodeActions {
                            deployed_from: Some(CodeActionSource::QuickActionBar),
                            quick_launch: false,
                        },
                        window,
                        cx,
                    );
                });
            })
            .action(Box::new(ToggleCodeActions::default()))
            .toggled(is_deployed)
            .disabled(!has_available_code_actions)
            .popup(popup),
        )
    }
}
