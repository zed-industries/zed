use crate::{
    DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition, GoToImplementation,
    GoToTypeDefinition, Rename, RevealInFinder, SelectMode, ToggleCodeActions,
};
use gpui::{DismissEvent, Pixels, Point, Subscription, View, ViewContext};

pub struct MouseContextMenu {
    pub(crate) position: Point<Pixels>,
    pub(crate) context_menu: View<ui::ContextMenu>,
    _subscription: Subscription,
}

impl MouseContextMenu {
    pub(crate) fn new(
        position: Point<Pixels>,
        context_menu: View<ui::ContextMenu>,
        cx: &mut ViewContext<Editor>,
    ) -> Self {
        let context_menu_focus = context_menu.focus_handle(cx);
        cx.focus(&context_menu_focus);

        let _subscription =
            cx.subscribe(&context_menu, move |this, _, _event: &DismissEvent, cx| {
                this.mouse_context_menu.take();
                if context_menu_focus.contains_focused(cx) {
                    this.focus(cx);
                }
            });

        Self {
            position,
            context_menu,
            _subscription,
        }
    }
}

pub fn deploy_context_menu(
    editor: &mut Editor,
    position: Point<Pixels>,
    point: DisplayPoint,
    cx: &mut ViewContext<Editor>,
) {
    if !editor.is_focused(cx) {
        editor.focus(cx);
    }

    // Don't show context menu for inline editors
    if editor.mode() != EditorMode::Full {
        return;
    }

    let context_menu = if let Some(custom) = editor.custom_context_menu.take() {
        let menu = custom(editor, point, cx);
        editor.custom_context_menu = Some(custom);
        if menu.is_none() {
            return;
        }
        menu.unwrap()
    } else {
        // Don't show the context menu if there isn't a project associated with this editor
        if editor.project.is_none() {
            return;
        }

        // Move the cursor to the clicked location so that dispatched actions make sense
        editor.change_selections(None, cx, |s| {
            s.clear_disjoint();
            s.set_pending_display_range(point..point, SelectMode::Character);
        });

        ui::ContextMenu::build(cx, |menu, _cx| {
            menu.action("Rename Symbol", Box::new(Rename))
                .action("Go to Definition", Box::new(GoToDefinition))
                .action("Go to Type Definition", Box::new(GoToTypeDefinition))
                .action("Go to Implementation", Box::new(GoToImplementation))
                .action("Find All References", Box::new(FindAllReferences))
                .action(
                    "Code Actions",
                    Box::new(ToggleCodeActions {
                        deployed_from_indicator: false,
                    }),
                )
                .separator()
                .action("Reveal in Finder", Box::new(RevealInFinder))
        })
    };
    let mouse_context_menu = MouseContextMenu::new(position, context_menu, cx);
    editor.mouse_context_menu = Some(mouse_context_menu);
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
        cx.editor(|editor, _app| assert!(editor.mouse_context_menu.is_none()));
        cx.update_editor(|editor, cx| deploy_context_menu(editor, Default::default(), point, cx));

        cx.assert_editor_state(indoc! {"
            fn test() {
                do_wˇork();
            }
        "});
        cx.editor(|editor, _app| assert!(editor.mouse_context_menu.is_some()));
    }
}
