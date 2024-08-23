use std::ops::Range;

use crate::{
    actions::Format, selections_collection::SelectionsCollection, Copy, CopyPermalinkToLine, Cut,
    DisplayPoint, DisplaySnapshot, Editor, EditorMode, FindAllReferences, GoToDeclaration,
    GoToDefinition, GoToImplementation, GoToTypeDefinition, Paste, Rename, RevealInFileManager,
    SelectMode, ToDisplayPoint, ToggleCodeActions,
};
use gpui::prelude::FluentBuilder;
use gpui::{DismissEvent, Pixels, Point, Subscription, View, ViewContext};
use workspace::OpenInTerminal;

#[derive(Debug)]
pub enum MenuPosition {
    /// When the editor is scrolled, the context menu stays on the exact
    /// same position on the screen, never disappearing.
    PinnedToScreen(Point<Pixels>),
    /// When the editor is scrolled, the context menu follows the position it is associated with.
    /// Disappears when the position is no longer visible.
    PinnedToEditor {
        source: multi_buffer::Anchor,
        offset_x: Pixels,
        offset_y: Pixels,
    },
}

pub struct MouseContextMenu {
    pub(crate) position: MenuPosition,
    pub(crate) context_menu: View<ui::ContextMenu>,
    _subscription: Subscription,
}

impl std::fmt::Debug for MouseContextMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseContextMenu")
            .field("position", &self.position)
            .field("context_menu", &self.context_menu)
            .finish()
    }
}

impl MouseContextMenu {
    pub(crate) fn pinned_to_editor(
        editor: &mut Editor,
        source: multi_buffer::Anchor,
        position: Point<Pixels>,
        context_menu: View<ui::ContextMenu>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<Self> {
        let context_menu_focus = context_menu.focus_handle(cx);
        cx.focus(&context_menu_focus);

        let _subscription = cx.subscribe(
            &context_menu,
            move |editor, _, _event: &DismissEvent, cx| {
                editor.mouse_context_menu.take();
                if context_menu_focus.contains_focused(cx) {
                    editor.focus(cx);
                }
            },
        );

        let editor_snapshot = editor.snapshot(cx);
        let source_point = editor.to_pixel_point(source, &editor_snapshot, cx)?;
        let offset = position - source_point;

        Some(Self {
            position: MenuPosition::PinnedToEditor {
                source,
                offset_x: offset.x,
                offset_y: offset.y,
            },
            context_menu,
            _subscription,
        })
    }

    pub(crate) fn pinned_to_screen(
        position: Point<Pixels>,
        context_menu: View<ui::ContextMenu>,
        cx: &mut ViewContext<Editor>,
    ) -> Self {
        let context_menu_focus = context_menu.focus_handle(cx);
        cx.focus(&context_menu_focus);

        let _subscription = cx.subscribe(
            &context_menu,
            move |editor, _, _event: &DismissEvent, cx| {
                editor.mouse_context_menu.take();
                if context_menu_focus.contains_focused(cx) {
                    editor.focus(cx);
                }
            },
        );

        Self {
            position: MenuPosition::PinnedToScreen(position),
            context_menu,
            _subscription,
        }
    }
}

fn display_ranges<'a>(
    display_map: &'a DisplaySnapshot,
    selections: &'a SelectionsCollection,
) -> impl Iterator<Item = Range<DisplayPoint>> + 'a {
    let pending = selections
        .pending
        .as_ref()
        .map(|pending| &pending.selection);
    selections
        .disjoint
        .iter()
        .chain(pending)
        .map(move |s| s.start.to_display_point(&display_map)..s.end.to_display_point(&display_map))
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

    let display_map = editor.selections.display_map(cx);
    let source_anchor = display_map.display_point_to_anchor(point, text::Bias::Right);
    let context_menu = if let Some(custom) = editor.custom_context_menu.take() {
        let menu = custom(editor, point, cx);
        editor.custom_context_menu = Some(custom);
        let Some(menu) = menu else {
            return;
        };
        menu
    } else {
        // Don't show the context menu if there isn't a project associated with this editor
        if editor.project.is_none() {
            return;
        }

        let display_map = editor.selections.display_map(cx);
        let buffer = &editor.snapshot(cx).buffer_snapshot;
        let anchor = buffer.anchor_before(point.to_point(&display_map));
        if !display_ranges(&display_map, &editor.selections).any(|r| r.contains(&point)) {
            // Move the cursor to the clicked location so that dispatched actions make sense
            editor.change_selections(None, cx, |s| {
                s.clear_disjoint();
                s.set_pending_anchor_range(anchor..anchor, SelectMode::Character);
            });
        }

        let focus = cx.focused();
        ui::ContextMenu::build(cx, |menu, _cx| {
            let builder = menu
                .on_blur_subscription(Subscription::new(|| {}))
                .action("Rename Symbol", Box::new(Rename))
                .action("Go to Definition", Box::new(GoToDefinition))
                .action("Go to Declaration", Box::new(GoToDeclaration))
                .action("Go to Type Definition", Box::new(GoToTypeDefinition))
                .action("Go to Implementation", Box::new(GoToImplementation))
                .action("Find All References", Box::new(FindAllReferences))
                .action(
                    "Code Actions",
                    Box::new(ToggleCodeActions {
                        deployed_from_indicator: None,
                    }),
                )
                .action("Format Buffer", Box::new(Format))
                .separator()
                .action("Cut", Box::new(Cut))
                .action("Copy", Box::new(Copy))
                .action("Paste", Box::new(Paste))
                .separator()
                .when(cfg!(target_os = "macos"), |builder| {
                    builder.action("Reveal in Finder", Box::new(RevealInFileManager))
                })
                .when(cfg!(not(target_os = "macos")), |builder| {
                    builder.action("Reveal in File Manager", Box::new(RevealInFileManager))
                })
                .action("Open in Terminal", Box::new(OpenInTerminal))
                .action("Copy Permalink", Box::new(CopyPermalinkToLine));
            match focus {
                Some(focus) => builder.context(focus),
                None => builder,
            }
        })
    };

    editor.mouse_context_menu =
        MouseContextMenu::pinned_to_editor(editor, source_anchor, position, context_menu, cx);
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
