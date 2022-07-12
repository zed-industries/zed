use context_menu::ContextMenuItem;
use gpui::{geometry::vector::Vector2F, impl_internal_actions, MutableAppContext, ViewContext};

use crate::{
    DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition, Rename, SelectMode,
    ToggleCodeActions,
};

#[derive(Clone, PartialEq)]
pub struct DeployMouseContextMenu {
    pub position: Vector2F,
    pub point: DisplayPoint,
}

impl_internal_actions!(editor, [DeployMouseContextMenu]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(deploy_context_menu);
}

pub fn deploy_context_menu(
    editor: &mut Editor,
    &DeployMouseContextMenu { position, point }: &DeployMouseContextMenu,
    cx: &mut ViewContext<Editor>,
) {
    // Don't show context menu for inline editors
    if editor.mode() != EditorMode::Full {
        return;
    }

    // Don't show the context menu if there isn't a project associated with this editor
    if editor.project.is_none() {
        return;
    }

    // Move the cursor to the clicked location so that dispatched actions make sense
    editor.change_selections(None, cx, |s| {
        s.clear_disjoint();
        s.set_pending_display_range(point..point, SelectMode::Character);
    });

    editor.mouse_context_menu.update(cx, |menu, cx| {
        menu.show(
            position,
            vec![
                ContextMenuItem::item("Rename Symbol", Rename),
                ContextMenuItem::item("Go To Definition", GoToDefinition),
                ContextMenuItem::item("Find All References", FindAllReferences),
                ContextMenuItem::item(
                    "Code Actions",
                    ToggleCodeActions {
                        deployed_from_indicator: false,
                    },
                ),
            ],
            cx,
        );
    });
    cx.notify();
}
