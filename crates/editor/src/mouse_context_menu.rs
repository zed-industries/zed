use crate::{
    DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition, GoToTypeDefinition,
    Rename, RevealInFinder, SelectMode, ToggleCodeActions,
};
use context_menu::ContextMenuItem;
use gpui::{elements::AnchorCorner, geometry::vector::Vector2F, ViewContext};

pub fn deploy_context_menu(
    editor: &mut Editor,
    position: Vector2F,
    point: DisplayPoint,
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
                ContextMenuItem::action("Rename Symbol", Rename),
                ContextMenuItem::action("Go to Definition", GoToDefinition),
                ContextMenuItem::action("Go to Type Definition", GoToTypeDefinition),
                ContextMenuItem::action("Find All References", FindAllReferences),
                ContextMenuItem::action(
                    "Code Actions",
                    ToggleCodeActions {
                        deployed_from_indicator: false,
                    },
                ),
                ContextMenuItem::Separator,
                ContextMenuItem::action("Reveal in Finder", RevealInFinder),
            ],
            cx,
        );
    });
    cx.notify();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use indoc::indoc;

    #[gpui::test]
    async fn test_mouse_context_menu(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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
        cx.update_editor(|editor, cx| deploy_context_menu(editor, Default::default(), point, cx));

        cx.assert_editor_state(indoc! {"
            fn test() {
                do_wˇork();
            }
        "});
        cx.editor(|editor, app| assert!(editor.mouse_context_menu.read(app).visible()));
    }
}
