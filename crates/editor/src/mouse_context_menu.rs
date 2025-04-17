use crate::{
    ConfirmCodeAction, Copy, CopyAndTrim, CopyPermalinkToLine, Cut, DebuggerEvaluateSelectedText,
    DisplayPoint, DisplaySnapshot, Editor, FindAllReferences, GoToDeclaration, GoToDefinition,
    GoToImplementation, GoToTypeDefinition, MultiBufferRow, OffsetRangeExt, Paste, Rename,
    RevealInFileManager, SelectMode, ToDisplayPoint,
    actions::{Format, FormatSelections},
    code_context_menus::CodeActionContents,
    selections_collection::SelectionsCollection,
};
use gpui::prelude::FluentBuilder;
use gpui::{
    Context, DismissEvent, Entity, Focusable as _, Pixels, Point, Subscription, Task, Window,
};
use std::ops::Range;
use text::PointUtf16;
use util::ResultExt;
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

pub struct MouseCodeActionState {
    pub contents: CodeActionContents,
    pub buffer: Entity<language::Buffer>,
}

pub struct MouseContextMenu {
    pub(crate) position: MenuPosition,
    pub(crate) context_menu: Entity<ui::ContextMenu>,
    pub(crate) code_action_state: Option<MouseCodeActionState>,
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
        code_action_state: Option<MouseCodeActionState>,
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
        return Some(MouseContextMenu::new(
            menu_position,
            context_menu,
            code_action_state,
            window,
            cx,
        ));
    }

    pub(crate) fn new(
        position: MenuPosition,
        context_menu: Entity<ui::ContextMenu>,
        code_action_state: Option<MouseCodeActionState>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Self {
        let context_menu_focus = context_menu.focus_handle(cx);
        window.focus(&context_menu_focus);

        let _subscription = cx.subscribe_in(
            &context_menu,
            window,
            move |editor, _, _event: &DismissEvent, window, cx| {
                editor.mouse_context_menu.take();
                if context_menu_focus.contains_focused(window, cx) {
                    window.focus(&editor.focus_handle(cx));
                }
            },
        );

        Self {
            position,
            context_menu,
            code_action_state,
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
    if let Some(custom) = editor.custom_context_menu.take() {
        let menu = custom(editor, point, window, cx);
        editor.custom_context_menu = Some(custom);
        let Some(menu) = menu else {
            return;
        };
        set_context_menu(editor, menu, source_anchor, position, None, window, cx);
    } else {
        // Don't show the context menu if there isn't a project associated with this editor
        let Some(project) = editor.project.clone() else {
            return;
        };

        let display_map = editor.selections.display_map(cx);
        let buffer = &editor.snapshot(window, cx).buffer_snapshot;
        let anchor = buffer.anchor_before(point.to_point(&display_map));
        if !display_ranges(&display_map, &editor.selections).any(|r| r.contains(&point)) {
            // We don't need to debounce in case of mouse click
            editor.force_code_action_task = true;
            // Move the cursor to the clicked location so that dispatched actions make sense
            editor.change_selections(None, window, cx, |s| {
                s.clear_disjoint();
                s.set_pending_anchor_range(anchor..anchor, SelectMode::Character);
            });
            editor.force_code_action_task = false;
        }

        let mut code_action_task = editor.code_actions_task.take();
        cx.spawn_in(window, async move |editor, cx| {
            while let Some(prev_task) = code_action_task {
                prev_task.await.log_err();
                code_action_task = editor.update(cx, |this, _| this.code_actions_task.take())?;
            }
            let context_menu_task = editor.update_in(cx, |editor, window, cx| {
                let focus = window.focused(cx);
                let has_reveal_target = editor.target_file(cx).is_some();
                let has_selections = editor
                    .selections
                    .all::<PointUtf16>(cx)
                    .into_iter()
                    .any(|s| !s.is_empty());
                let has_git_repo = anchor.buffer_id.is_some_and(|buffer_id| {
                    project
                        .read(cx)
                        .git_store()
                        .read(cx)
                        .repository_and_path_for_buffer_id(buffer_id, cx)
                        .is_some()
                });

                let evaluate_selection =
                    command_palette_hooks::CommandPaletteFilter::try_global(cx)
                        .map_or(false, |filter| {
                            !filter.is_hidden(&DebuggerEvaluateSelectedText)
                        });

                let editor_snapshot = &editor.snapshot(window, cx);
                let code_action_state = {
                    if let Some((buffer, buffer_row)) = editor_snapshot
                        .buffer_snapshot
                        .buffer_line_for_row(MultiBufferRow(point.to_point(editor_snapshot).row))
                        .and_then(|(buffer_snapshot, range)| {
                            editor
                                .buffer
                                .read(cx)
                                .buffer(buffer_snapshot.remote_id())
                                .map(|buffer| (buffer, range.start.row))
                        })
                    {
                        editor.available_code_actions.clone().and_then(
                            |(location, code_actions)| {
                                let snapshot = location.buffer.read(cx).snapshot();
                                let point_range = location.range.to_point(&snapshot);
                                let point_range = point_range.start.row..=point_range.end.row;
                                if point_range.contains(&buffer_row) {
                                    Some(MouseCodeActionState {
                                        contents: CodeActionContents {
                                            actions: Some(code_actions),
                                            tasks: None,
                                        },
                                        buffer,
                                    })
                                } else {
                                    None
                                }
                            },
                        )
                    } else {
                        None
                    }
                };

                let action_labels = code_action_state
                    .as_ref()
                    .map(|state| {
                        if state.contents.is_empty() {
                            None
                        } else {
                            Some(
                                state
                                    .contents
                                    .iter()
                                    .map(|item| item.label())
                                    .collect::<Vec<_>>(),
                            )
                        }
                    })
                    .flatten();

                let menu = ui::ContextMenu::build(window, cx, |menu, _window, _| {
                    let menu = menu
                        .on_blur_subscription(Subscription::new(|| {}))
                        .when_some(action_labels, |menu, labels| {
                            menu.map(|menu| {
                                labels
                                    .into_iter()
                                    .enumerate()
                                    .fold(menu, |menu, (ix, label)| {
                                        menu.action(
                                            label,
                                            Box::new(ConfirmCodeAction {
                                                item_ix: Some(ix),
                                                from_mouse_context_menu: true,
                                            }),
                                        )
                                    })
                            })
                            .separator()
                        })
                        .when(evaluate_selection && has_selections, |builder| {
                            builder
                                .action(
                                    "Evaluate Selection",
                                    Box::new(DebuggerEvaluateSelectedText),
                                )
                                .separator()
                        })
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
                        .separator()
                        .action("Cut", Box::new(Cut))
                        .action("Copy", Box::new(Copy))
                        .action("Copy and trim", Box::new(CopyAndTrim))
                        .action("Paste", Box::new(Paste))
                        .separator()
                        .map(|builder| {
                            let reveal_in_finder_label = if cfg!(target_os = "macos") {
                                "Reveal in Finder"
                            } else {
                                "Reveal in File Manager"
                            };
                            const OPEN_IN_TERMINAL_LABEL: &str = "Open in Terminal";
                            if has_reveal_target {
                                builder
                                    .action(reveal_in_finder_label, Box::new(RevealInFileManager))
                                    .action(OPEN_IN_TERMINAL_LABEL, Box::new(OpenInTerminal))
                            } else {
                                builder
                                    .disabled_action(
                                        reveal_in_finder_label,
                                        Box::new(RevealInFileManager),
                                    )
                                    .disabled_action(
                                        OPEN_IN_TERMINAL_LABEL,
                                        Box::new(OpenInTerminal),
                                    )
                            }
                        })
                        .map(|builder| {
                            const COPY_PERMALINK_LABEL: &str = "Copy Permalink";
                            if has_git_repo {
                                builder.action(COPY_PERMALINK_LABEL, Box::new(CopyPermalinkToLine))
                            } else {
                                builder.disabled_action(
                                    COPY_PERMALINK_LABEL,
                                    Box::new(CopyPermalinkToLine),
                                )
                            }
                        });
                    match focus {
                        Some(focus) => menu.context(focus),
                        None => menu,
                    }
                });

                set_context_menu(
                    editor,
                    menu,
                    source_anchor,
                    position,
                    code_action_state,
                    window,
                    cx,
                );

                Some(Task::ready(Ok::<_, anyhow::Error>(())))
            })?;

            if let Some(task) = context_menu_task {
                task.await?;
            }

            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    };
}

fn set_context_menu(
    editor: &mut Editor,
    context_menu: Entity<ui::ContextMenu>,
    source_anchor: multi_buffer::Anchor,
    position: Option<Point<Pixels>>,
    code_action_state: Option<MouseCodeActionState>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    editor.mouse_context_menu = match position {
        Some(position) => MouseContextMenu::pinned_to_editor(
            editor,
            source_anchor,
            position,
            code_action_state,
            context_menu,
            window,
            cx,
        ),
        None => {
            let character_size = editor.character_size(window);
            let menu_position = MenuPosition::PinnedToEditor {
                source: source_anchor,
                offset: gpui::point(character_size.width, character_size.height),
            };
            Some(MouseContextMenu::new(
                menu_position,
                context_menu,
                code_action_state,
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
