use context_menu::ContextMenuItem;
use gpui::{
    elements::AnchorCorner, geometry::vector::Vector2F, impl_internal_actions, MutableAppContext,
    ViewContext,
};

use crate::{
    DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition, GoToTypeDefinition,
    Rename, SelectMode, ToggleCodeActions,
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
    if !editor.focused {
        cx.focus_self();
    }

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
            AnchorCorner::TopLeft,
            vec![
                ContextMenuItem::item("Rename Symbol", Rename),
                ContextMenuItem::item("Go To Definition", GoToDefinition),
                ContextMenuItem::item("Go To Type Definition", GoToTypeDefinition),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::EditorLspTestContext;
    use indoc::indoc;

    #[gpui::test]
    async fn test_mouse_context_menu(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            fn teˇst() {
                do_work();
            }
        "});
        let point = cx.display_point(indoc! {"
            fn test() {
                do_wˇork();
            }
        "});
        cx.update_editor(|editor, cx| {
            deploy_context_menu(
                editor,
                &DeployMouseContextMenu {
                    position: Default::default(),
                    point,
                },
                cx,
            )
        });

        cx.assert_editor_state(indoc! {"
            fn test() {
                do_wˇork();
            }
        "});
        cx.editor(|editor, app| assert!(editor.mouse_context_menu.read(app).visible()));
    }
}
