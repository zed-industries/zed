use crate::{
    ConfirmCodeAction, Copy, CopyAndTrim, CopyPermalinkToLine, Cut, DebuggerEvaluateSelectedText,
    DisplayPoint, DisplaySnapshot, Editor, FindAllReferences, GoToDeclaration, GoToDefinition,
    GoToImplementation, GoToTypeDefinition, Paste, Rename, RevealInFileManager, SelectMode,
    SelectionExt, ToDisplayPoint, ToggleCodeActions,
    actions::{Format, FormatSelections},
    code_context_menus::CodeActionContents,
    selections_collection::SelectionsCollection,
};
use feature_flags::{Debugger, FeatureFlagAppExt as _};
use gpui::prelude::FluentBuilder;
use gpui::{
    Context, DismissEvent, Entity, FocusHandle, Focusable as _, Pixels, Point, Subscription, Task,
    Window,
};
use std::ops::Range;
use text::PointUtf16;
use ui::ContextMenu;
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

pub struct MouseCodeAction {
    pub actions: CodeActionContents,
    pub buffer: Entity<language::Buffer>,
}

pub struct MouseContextMenu {
    pub(crate) position: MenuPosition,
    pub(crate) context_menu: Entity<ui::ContextMenu>,
    pub(crate) code_action: Option<MouseCodeAction>,
    _dismiss_subscription: Subscription,
    _cursor_move_subscription: Subscription,
}

enum CodeActionLoadState {
    Loading,
    Loaded(CodeActionContents),
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
        code_action: Option<MouseCodeAction>,
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
            editor,
            menu_position,
            context_menu,
            code_action,
            window,
            cx,
        ));
    }

    pub(crate) fn new(
        editor: &Editor,
        position: MenuPosition,
        context_menu: Entity<ui::ContextMenu>,
        code_action: Option<MouseCodeAction>,
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
                let selection_init_range = selection_init.display_range(&display_snapshot);
                let selection_now_range = editor
                    .selections
                    .newest_anchor()
                    .display_range(&display_snapshot);
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
            code_action,
            _dismiss_subscription,
            _cursor_move_subscription,
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
            // Move the cursor to the clicked location so that dispatched actions make sense
            editor.change_selections(None, window, cx, |s| {
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
        let has_git_repo = anchor.buffer_id.is_some_and(|buffer_id| {
            project
                .read(cx)
                .git_store()
                .read(cx)
                .repository_and_path_for_buffer_id(buffer_id, cx)
                .is_some()
        });

        let evaluate_selection = command_palette_hooks::CommandPaletteFilter::try_global(cx)
            .map_or(false, |filter| {
                !filter.is_hidden(&DebuggerEvaluateSelectedText)
            });

        let menu = build_context_menu(
            focus,
            has_selections,
            has_reveal_target,
            has_git_repo,
            evaluate_selection,
            Some(CodeActionLoadState::Loading),
            window,
            cx,
        );

        set_context_menu(editor, menu, source_anchor, position, None, window, cx);

        let mut actions_task = editor.code_actions_task.take();
        cx.spawn_in(window, async move |editor, cx| {
            while let Some(prev_task) = actions_task {
                prev_task.await.log_err();
                actions_task = editor.update(cx, |this, _| this.code_actions_task.take())?;
            }
            let action = ToggleCodeActions {
                deployed_from_indicator: Some(point.row()),
            };
            let context_menu_task = editor.update_in(cx, |editor, window, cx| {
                let code_actions_task = editor.prepare_code_actions_task(&action, window, cx);
                Some(cx.spawn_in(window, async move |editor, cx| {
                    let code_action_result = code_actions_task.await;
                    if let Ok(editor_task) = editor.update_in(cx, |editor, window, cx| {
                        let Some(mouse_context_menu) = editor.mouse_context_menu.take() else {
                            return Task::ready(Ok::<_, anyhow::Error>(()));
                        };
                        if mouse_context_menu
                            .context_menu
                            .focus_handle(cx)
                            .contains_focused(window, cx)
                        {
                            window.focus(&editor.focus_handle(cx));
                        }
                        drop(mouse_context_menu);
                        let (state, code_action) =
                            if let Some((buffer, actions)) = code_action_result {
                                (
                                    CodeActionLoadState::Loaded(actions.clone()),
                                    Some(MouseCodeAction { actions, buffer }),
                                )
                            } else {
                                (
                                    CodeActionLoadState::Loaded(CodeActionContents::default()),
                                    None,
                                )
                            };
                        let menu = build_context_menu(
                            window.focused(cx),
                            has_selections,
                            has_reveal_target,
                            has_git_repo,
                            evaluate_selection,
                            Some(state),
                            window,
                            cx,
                        );
                        set_context_menu(
                            editor,
                            menu,
                            source_anchor,
                            position,
                            code_action,
                            window,
                            cx,
                        );
                        Task::ready(Ok(()))
                    }) {
                        editor_task.await
                    } else {
                        Ok(())
                    }
                }))
            })?;
            if let Some(task) = context_menu_task {
                task.await?;
            }
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    };
}

fn build_context_menu(
    focus: Option<FocusHandle>,
    has_selections: bool,
    has_reveal_target: bool,
    has_git_repo: bool,
    evaluate_selection: bool,
    code_action_load_state: Option<CodeActionLoadState>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Entity<ContextMenu> {
    ui::ContextMenu::build(window, cx, |menu, _window, cx| {
        let menu = menu
            .on_blur_subscription(Subscription::new(|| {}))
            .when(evaluate_selection && has_selections, |builder| {
                builder
                    .action("Evaluate Selection", Box::new(DebuggerEvaluateSelectedText))
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
                        .disabled_action(reveal_in_finder_label, Box::new(RevealInFileManager))
                        .disabled_action(OPEN_IN_TERMINAL_LABEL, Box::new(OpenInTerminal))
                }
            })
            .map(|builder| {
                const COPY_PERMALINK_LABEL: &str = "Copy Permalink";
                if has_git_repo {
                    builder.action(COPY_PERMALINK_LABEL, Box::new(CopyPermalinkToLine))
                } else {
                    builder.disabled_action(COPY_PERMALINK_LABEL, Box::new(CopyPermalinkToLine))
                }
            })
            .when_some(code_action_load_state, |menu, state| {
                menu.separator().map(|menu| match state {
                    CodeActionLoadState::Loading => menu.disabled_action(
                        "Loading code actions...",
                        Box::new(ConfirmCodeAction {
                            item_ix: None,
                            from_mouse_context_menu: true,
                        }),
                    ),
                    CodeActionLoadState::Loaded(actions) => {
                        if actions.is_empty() {
                            menu.disabled_action(
                                "No code actions available",
                                Box::new(ConfirmCodeAction {
                                    item_ix: None,
                                    from_mouse_context_menu: true,
                                }),
                            )
                        } else {
                            actions
                                .iter()
                                .filter(|action| {
                                    if action
                                        .as_task()
                                        .map(|task| {
                                            matches!(task.task_type(), task::TaskType::Debug(_))
                                        })
                                        .unwrap_or(false)
                                    {
                                        cx.has_flag::<Debugger>()
                                    } else {
                                        true
                                    }
                                })
                                .enumerate()
                                .fold(menu, |menu, (ix, action)| {
                                    menu.action(
                                        action.label(),
                                        Box::new(ConfirmCodeAction {
                                            item_ix: Some(ix),
                                            from_mouse_context_menu: true,
                                        }),
                                    )
                                })
                        }
                    }
                })
            });
        match focus {
            Some(focus) => menu.context(focus),
            None => menu,
        }
    })
}

fn set_context_menu(
    editor: &mut Editor,
    context_menu: Entity<ui::ContextMenu>,
    source_anchor: multi_buffer::Anchor,
    position: Option<Point<Pixels>>,
    code_action: Option<MouseCodeAction>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    editor.mouse_context_menu = match position {
        Some(position) => MouseContextMenu::pinned_to_editor(
            editor,
            source_anchor,
            position,
            code_action,
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
                editor,
                menu_position,
                context_menu,
                code_action,
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
