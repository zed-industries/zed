use crate::{
    Copy, CopyAndTrim, CopyPermalinkToLine, Cut, DisplayPoint, DisplaySnapshot, Editor,
    EvaluateSelectedText, FindAllReferences, GoToDeclaration, GoToDefinition, GoToImplementation,
    GoToTypeDefinition, Paste, Rename, RevealInFileManager, RunToCursor, SelectMode,
    SelectionEffects, SelectionExt, ToDisplayPoint, ToggleCodeActions,
    actions::{Format, FormatSelections},
    selections_collection::SelectionsCollection,
};
use gpui::prelude::FluentBuilder;
use gpui::{Context, DismissEvent, Entity, Focusable as _, Pixels, Point, Subscription, Window};
use std::ops::Range;
use text::PointUtf16;
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
        offset: Point<Pixels>,
    },
}

pub struct MouseContextMenu {
    pub(crate) position: MenuPosition,
    pub(crate) context_menu: Entity<ui::ContextMenu>,
    _dismiss_subscription: Subscription,
    _cursor_move_subscription: Subscription,
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
        context_menu: Entity<ui::ContextMenu>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<Self> {
        let editor_snapshot = editor.snapshot(window, cx);
        let content_origin = editor.last_bounds?.origin
            + Point {
                x: editor.gutter_dimensions.width,
                y: Pixels(0.0),
            };
        let source_position = editor.to_pixel_point(source, &editor_snapshot, window)?;
        let menu_position = MenuPosition::PinnedToEditor {
            source,
            offset: position - (source_position + content_origin),
        };
        Some(MouseContextMenu::new(
            editor,
            menu_position,
            context_menu,
            window,
            cx,
        ))
    }

    pub(crate) fn new(
        editor: &Editor,
        position: MenuPosition,
        context_menu: Entity<ui::ContextMenu>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Self {
        let context_menu_focus = context_menu.focus_handle(cx);
        window.focus(&context_menu_focus);

        let _dismiss_subscription = cx.subscribe_in(&context_menu, window, {
            let context_menu_focus = context_menu_focus.clone();
            move |editor, _, _event: &DismissEvent, window, cx| {
                editor.mouse_context_menu.take();
                if context_menu_focus.contains_focused(window, cx) {
                    window.focus(&editor.focus_handle(cx));
                }
            }
        });

        let selection_init = editor.selections.newest_anchor().clone();

        let _cursor_move_subscription = cx.subscribe_in(
            &cx.entity(),
            window,
            move |editor, _, event: &crate::EditorEvent, window, cx| {
                let crate::EditorEvent::SelectionsChanged { local: true } = event else {
                    return;
                };
                let display_snapshot = &editor
                    .display_map
                    .update(cx, |display_map, cx| display_map.snapshot(cx));
                let selection_init_range = selection_init.display_range(display_snapshot);
                let selection_now_range = editor
                    .selections
                    .newest_anchor()
                    .display_range(display_snapshot);
                if selection_now_range == selection_init_range {
                    return;
                }
                editor.mouse_context_menu.take();
                if context_menu_focus.contains_focused(window, cx) {
                    window.focus(&editor.focus_handle(cx));
                }
            },
        );

        Self {
            position,
            context_menu,
            _dismiss_subscription,
            _cursor_move_subscription,
        }
    }
}

fn display_ranges<'a>(
    display_map: &'a DisplaySnapshot,
    selections: &'a SelectionsCollection,
) -> impl Iterator<Item = Range<DisplayPoint>> + 'a {
    let pending = selections.pending_anchor();
    selections
        .disjoint_anchors()
        .iter()
        .chain(pending)
        .map(move |s| s.start.to_display_point(display_map)..s.end.to_display_point(display_map))
}

pub fn deploy_context_menu(
    editor: &mut Editor,
    position: Option<Point<Pixels>>,
    point: DisplayPoint,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if !editor.is_focused(window) {
        window.focus(&editor.focus_handle(cx));
    }

    // Don't show context menu for inline editors
    if !editor.mode().is_full() {
        return;
    }

    let display_map = editor.selections.display_map(cx);
    let source_anchor = display_map.display_point_to_anchor(point, text::Bias::Right);
    let context_menu = if let Some(custom) = editor.custom_context_menu.take() {
        let menu = custom(editor, point, window, cx);
        editor.custom_context_menu = Some(custom);
        let Some(menu) = menu else {
            return;
        };
        menu
    } else {
        // Don't show the context menu if there isn't a project associated with this editor
        let Some(project) = editor.project.clone() else {
            return;
        };

        let display_map = editor.selections.display_map(cx);
        let buffer = &editor.snapshot(window, cx).buffer_snapshot;
        let anchor = buffer.anchor_before(point.to_point(&display_map));
        if !display_ranges(&display_map, &editor.selections).any(|r| r.contains(&point)) {
            // Move the cursor to the clicked location so that dispatched actions make sense
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.clear_disjoint();
                s.set_pending_anchor_range(anchor..anchor, SelectMode::Character);
            });
        }

        let focus = window.focused(cx);
        let has_reveal_target = editor.target_file(cx).is_some();
        let has_selections = editor
            .selections
            .all::<PointUtf16>(cx)
            .into_iter()
            .any(|s| !s.is_empty());
        let has_git_repo = buffer
            .buffer_id_for_anchor(anchor)
            .is_some_and(|buffer_id| {
                project
                    .read(cx)
                    .git_store()
                    .read(cx)
                    .repository_and_path_for_buffer_id(buffer_id, cx)
                    .is_some()
            });

        let evaluate_selection = window.is_action_available(&EvaluateSelectedText, cx);
        let run_to_cursor = window.is_action_available(&RunToCursor, cx);

        ui::ContextMenu::build(window, cx, |menu, _window, _cx| {
            let builder = menu
                .on_blur_subscription(Subscription::new(|| {}))
                .when(run_to_cursor, |builder| {
                    builder.action("Run to Cursor", Box::new(RunToCursor))
                })
                .when(evaluate_selection && has_selections, |builder| {
                    builder.action("Evaluate Selection", Box::new(EvaluateSelectedText))
                })
                .when(
                    run_to_cursor || (evaluate_selection && has_selections),
                    |builder| builder.separator(),
                )
                .action("Go to Definition", Box::new(GoToDefinition))
                .action("Go to Declaration", Box::new(GoToDeclaration))
                .action("Go to Type Definition", Box::new(GoToTypeDefinition))
                .action("Go to Implementation", Box::new(GoToImplementation))
                .action("Find All References", Box::new(FindAllReferences))
                .separator()
                .action("Rename Symbol", Box::new(Rename))
                .action("Format Buffer", Box::new(Format))
                .when(has_selections, |cx| {
                    cx.action("Format Selections", Box::new(FormatSelections))
                })
                .action(
                    "Show Code Actions",
                    Box::new(ToggleCodeActions {
                        deployed_from: None,
                        quick_launch: false,
                    }),
                )
                .separator()
                .action("Cut", Box::new(Cut))
                .action("Copy", Box::new(Copy))
                .action("Copy and Trim", Box::new(CopyAndTrim))
                .action("Paste", Box::new(Paste))
                .separator()
                .action_disabled_when(
                    !has_reveal_target,
                    if cfg!(target_os = "macos") {
                        "Reveal in Finder"
                    } else {
                        "Reveal in File Manager"
                    },
                    Box::new(RevealInFileManager),
                )
                .action_disabled_when(
                    !has_reveal_target,
                    "Open in Terminal",
                    Box::new(OpenInTerminal),
                )
                .action_disabled_when(
                    !has_git_repo,
                    "Copy Permalink",
                    Box::new(CopyPermalinkToLine),
                );
            match focus {
                Some(focus) => builder.context(focus),
                None => builder,
            }
        })
    };

    editor.mouse_context_menu = match position {
        Some(position) => MouseContextMenu::pinned_to_editor(
            editor,
            source_anchor,
            position,
            context_menu,
            window,
            cx,
        ),
        None => {
            let character_size = editor.character_dimensions(window);
            let menu_position = MenuPosition::PinnedToEditor {
                source: source_anchor,
                offset: gpui::point(character_size.em_width, character_size.line_height),
            };
            Some(MouseContextMenu::new(
                editor,
                menu_position,
                context_menu,
                window,
                cx,
            ))
        }
    };
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
        cx.editor(|editor, _window, _app| assert!(editor.mouse_context_menu.is_none()));
        cx.update_editor(|editor, window, cx| {
            deploy_context_menu(editor, Some(Default::default()), point, window, cx)
        });

        cx.assert_editor_state(indoc! {"
            fn test() {
                do_wˇork();
            }
        "});
        cx.editor(|editor, _window, _app| assert!(editor.mouse_context_menu.is_some()));
    }
}
