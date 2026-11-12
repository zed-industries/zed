mod preview;
mod repl_menu;

use agent_settings::AgentSettings;
use editor::actions::{
    AddSelectionAbove, AddSelectionBelow, CodeActionSource, DuplicateLineDown, GoToDiagnostic,
    GoToHunk, GoToPreviousDiagnostic, GoToPreviousHunk, MoveLineDown, MoveLineUp, SelectAll,
    SelectLargerSyntaxNode, SelectNext, SelectSmallerSyntaxNode, ToggleCodeActions,
    ToggleDiagnostics, ToggleGoToLine, ToggleInlineDiagnostics,
};
use editor::code_context_menus::{CodeContextMenu, ContextMenuOrigin};
use editor::{Editor, EditorSettings};
use gpui::{
    Action, Anchor, AnchoredPositionMode, AnyElement, ClickEvent, Context, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, ParentElement, Render, Styled,
    Subscription, WeakEntity, Window, anchored, deferred, point,
};
use project::{
    DisableAiSettings,
    project_settings::{DiagnosticSeverity, ProjectSettings},
};
use search::{BufferSearchBar, buffer_search};
use settings::{GitDiffBaseSetting, Settings, SettingsStore, update_settings_file};
use ui::{
    ButtonStyle, ContextMenu, ContextMenuEntry, DocumentationSide, IconButton, IconName, IconSize,
    PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::item::ItemBufferKind;
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, item::ItemHandle,
};
use zed_actions::{agent::AddSelectionToThread, assistant::InlineAssist, outline::ToggleOutline};

const MAX_CODE_ACTION_MENU_LINES: u32 = 16;

pub struct QuickActionBar {
    _inlay_hints_enabled_subscription: Option<Subscription>,
    _ai_settings_subscription: Subscription,
    active_item: Option<Box<dyn ItemHandle>>,
    buffer_search_bar: Entity<BufferSearchBar>,
    show: bool,
    run_task: String,
    toggle_selections_handle: PopoverMenuHandle<ContextMenu>,
    toggle_settings_handle: PopoverMenuHandle<ContextMenu>,
    workspace: WeakEntity<Workspace>,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: Entity<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut was_agent_enabled = AgentSettings::get_global(cx).enabled(cx);
        let mut was_agent_button = AgentSettings::get_global(cx).button;

        let ai_settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
            let agent_settings = AgentSettings::get_global(cx);
            let is_agent_enabled = agent_settings.enabled(cx);

            if was_agent_enabled != is_agent_enabled || was_agent_button != agent_settings.button {
                was_agent_enabled = is_agent_enabled;
                was_agent_button = agent_settings.button;
                cx.notify();
            }
        });

        let mut this = Self {
            _inlay_hints_enabled_subscription: None,
            _ai_settings_subscription: ai_settings_subscription,
            active_item: None,
            buffer_search_bar,
            show: true,
            run_task: String::new(),
            toggle_selections_handle: Default::default(),
            toggle_settings_handle: Default::default(),
            workspace: workspace.weak_handle(),
        };
        this.apply_settings(cx);
        cx.observe_global::<SettingsStore>(|this, cx| this.apply_settings(cx))
            .detach();
        this
    }

    fn active_editor(&self) -> Option<Entity<Editor>> {
        self.active_item
            .as_ref()
            .and_then(|item| item.downcast::<Editor>())
    }

    fn render_run_task_button(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.run_task.is_empty() {
            return None;
        }
        let active_item = self.active_item.as_ref()?;
        let editor = self.active_editor()?;
        let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
        buffer.read(cx).file()?;
        let action = tasks_ui::Spawn::ByName {
            task_name: self.run_task.clone(),
            reveal_target: None,
        };
        let button = IconButton::new("run-task", IconName::PlayOutlined)
            .aria_label(format!("Run Task: {}", self.run_task))
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(Tooltip::for_action_title(
                format!("Run Task: {}", self.run_task),
                &action,
            ))
            .on_click({
                let workspace = self.workspace.clone();
                let active_item = active_item.boxed_clone();
                move |_, window, cx| {
                    run_task_for_item(&workspace, active_item.as_ref(), &action, window, cx);
                }
            });
        Some(button.into_any_element())
    }

    fn apply_settings(&mut self, cx: &mut Context<Self>) {
        let (new_show, run_task) = {
            let toolbar = &EditorSettings::get_global(cx).toolbar;
            (toolbar.quick_actions, toolbar.run_task.clone())
        };
        if self.run_task != run_task {
            self.run_task = run_task;
            cx.notify();
        }
        if new_show != self.show {
            self.show = new_show;
            cx.emit(ToolbarItemEvent::ChangeLocation(
                self.get_toolbar_item_location(),
            ));
        }
    }

    fn get_toolbar_item_location(&self) -> ToolbarItemLocation {
        if self.show && self.active_editor().is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

fn run_task_for_item(
    workspace: &WeakEntity<Workspace>,
    active_item: &dyn ItemHandle,
    action: &tasks_ui::Spawn,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };
    // A toolbar can belong to an unfocused split pane. Activate it before
    // resolving task variables or saving the current buffer.
    let activated = workspace.update(cx, |workspace, cx| {
        let Some(pane) = workspace.pane_for(active_item) else {
            return false;
        };
        pane.update(cx, |pane, cx| {
            let Some(index) = pane.index_for_item(active_item) else {
                return false;
            };
            pane.activate_item(index, true, true, window, cx);
            true
        })
    });
    if activated {
        window.dispatch_action(Box::new(action.clone()), cx);
    }
}

impl Render for QuickActionBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(editor) = self.active_editor() else {
            return div().id("empty quick action bar");
        };

        let supports_inlay_hints = editor.update(cx, |editor, cx| editor.supports_inlay_hints(cx));
        let supports_semantic_tokens =
            editor.update(cx, |editor, cx| editor.supports_semantic_tokens(cx));
        let supports_code_lens = editor.update(cx, |editor, cx| editor.supports_code_lens(cx));
        let editor_value = editor.read(cx);
        let selection_menu_enabled = editor_value.selection_menu_enabled(cx);
        let inlay_hints_enabled = editor_value.inlay_hints_enabled();
        let inline_values_enabled = editor_value.inline_values_enabled();
        let semantic_highlights_enabled = editor_value.semantic_highlights_enabled();
        let code_lens_enabled = editor_value.code_lens_enabled();
        let is_full = editor_value.mode().is_full();
        let diagnostics_enabled = editor_value.diagnostics_enabled()
            && editor_value.diagnostics_max_severity != DiagnosticSeverity::Off;
        let supports_inline_diagnostics = editor_value.inline_diagnostics_enabled();
        let inline_diagnostics_enabled = editor_value.show_inline_diagnostics();
        let git_blame_inline_enabled = editor_value.git_blame_inline_enabled();
        let show_git_blame_gutter = editor_value.show_git_blame_gutter();
        let auto_signature_help_enabled = editor_value.auto_signature_help_enabled(cx);
        let show_line_numbers = editor_value.line_numbers_enabled(cx);
        let has_edit_prediction_provider = editor_value.edit_prediction_provider().is_some();
        let show_edit_predictions = editor_value.edit_predictions_enabled();
        let edit_predictions_enabled_at_cursor =
            editor_value.edit_predictions_enabled_at_cursor(cx);
        let supports_minimap = editor_value.supports_minimap(cx);
        let minimap_enabled = supports_minimap && editor_value.minimap().is_some();
        let has_available_code_actions = editor_value.has_available_code_actions_for_selection();
        let code_action_enabled = editor_value.code_actions_enabled_for_toolbar(cx);
        let focus_handle = editor_value.focus_handle(cx);

        let search_button = (editor.buffer_kind(cx) == ItemBufferKind::Singleton).then(|| {
            QuickActionBarButton::new(
                "toggle buffer search",
                search::SEARCH_ICON,
                !self.buffer_search_bar.read(cx).is_dismissed(),
                Box::new(buffer_search::Deploy::find()),
                focus_handle.clone(),
                "Buffer Search",
                {
                    let buffer_search_bar = self.buffer_search_bar.clone();
                    move |_, window, cx| {
                        buffer_search_bar.update(cx, |search_bar, cx| {
                            search_bar.toggle(&buffer_search::Deploy::find(), window, cx)
                        });
                    }
                },
            )
        });

        let assistant_button = QuickActionBarButton::new(
            "toggle inline assistant",
            IconName::ZedAssistant,
            false,
            Box::new(InlineAssist::default()),
            focus_handle,
            "Inline Assist",
            move |_, window, cx| {
                window.dispatch_action(Box::new(InlineAssist::default()), cx);
            },
        );

        let code_actions_dropdown = code_action_enabled.then(|| {
            let is_deployed = {
                let menu_ref = editor.read(cx).context_menu().borrow();
                let code_action_menu = menu_ref
                    .as_ref()
                    .filter(|menu| matches!(menu, CodeContextMenu::CodeActions(..)));
                code_action_menu
                    .as_ref()
                    .is_some_and(|menu| matches!(menu.origin(), ContextMenuOrigin::QuickActionBar))
            };
            let code_action_element = is_deployed
                .then(|| {
                    editor.update(cx, |editor, cx| {
                        editor.render_context_menu(MAX_CODE_ACTION_MENU_LINES, window, cx)
                    })
                })
                .flatten();
            v_flex()
                .child(
                    IconButton::new("toggle_code_actions_icon", IconName::BoltOutlined)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Subtle)
                        .disabled(!has_available_code_actions)
                        .toggle_state(is_deployed)
                        .when(!is_deployed, |this| {
                            this.when(has_available_code_actions, |this| {
                                this.tooltip(Tooltip::for_action_title(
                                    "Code Actions",
                                    &ToggleCodeActions::default(),
                                ))
                            })
                            .when(
                                !has_available_code_actions,
                                |this| {
                                    this.tooltip(Tooltip::for_action_title(
                                        "No Code Actions Available",
                                        &ToggleCodeActions::default(),
                                    ))
                                },
                            )
                        })
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
                                editor.update(cx, |editor, cx| {
                                    editor.toggle_code_actions(
                                        &ToggleCodeActions {
                                            deployed_from: Some(CodeActionSource::QuickActionBar),
                                            quick_launch: false,
                                        },
                                        window,
                                        cx,
                                    );
                                })
                            }
                        }),
                )
                .children(code_action_element.map(|menu| {
                    deferred(
                        anchored()
                            .position_mode(AnchoredPositionMode::Local)
                            .position(point(px(20.), px(20.)))
                            .anchor(Anchor::TopRight)
                            .child(menu),
                    )
                }))
        });

        let editor_selections_dropdown = selection_menu_enabled.then(|| {
            let has_diff_hunks = editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx)
                .has_diff_hunks();
            let has_selection = editor.update(cx, |editor, cx| {
                editor.has_non_empty_selection(&editor.display_snapshot(cx))
            });

            let focus = editor.focus_handle(cx);

            let disable_ai = DisableAiSettings::get_global(cx).disable_ai;

            PopoverMenu::new("editor-selections-dropdown")
                .trigger_with_tooltip(
                    IconButton::new("toggle_editor_selections_icon", IconName::CursorIBeam)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Subtle)
                        .toggle_state(self.toggle_selections_handle.is_deployed()),
                    Tooltip::text("Selection Controls"),
                )
                .with_handle(self.toggle_selections_handle.clone())
                .anchor(Anchor::TopRight)
                .menu(move |window, cx| {
                    let focus = focus.clone();
                    let menu = ContextMenu::build(window, cx, move |menu, _, _| {
                        menu.context(focus.clone())
                            .action("Select All", Box::new(SelectAll))
                            .action(
                                "Select Next Occurrence",
                                Box::new(SelectNext {
                                    replace_newest: false,
                                }),
                            )
                            .action("Expand Selection", Box::new(SelectLargerSyntaxNode))
                            .action("Shrink Selection", Box::new(SelectSmallerSyntaxNode))
                            .action(
                                "Add Cursor Above",
                                Box::new(AddSelectionAbove {
                                    skip_soft_wrap: true,
                                }),
                            )
                            .action(
                                "Add Cursor Below",
                                Box::new(AddSelectionBelow {
                                    skip_soft_wrap: true,
                                }),
                            )
                            .when(!disable_ai, |this| {
                                this.separator().action_disabled_when(
                                    !has_selection,
                                    "Add to Agent Thread",
                                    Box::new(AddSelectionToThread),
                                )
                            })
                            .separator()
                            .action("Go to Symbol", Box::new(ToggleOutline))
                            .action("Go to Line/Column", Box::new(ToggleGoToLine))
                            .separator()
                            .action("Next Problem", Box::new(GoToDiagnostic::default()))
                            .action(
                                "Previous Problem",
                                Box::new(GoToPreviousDiagnostic::default()),
                            )
                            .separator()
                            .action_disabled_when(!has_diff_hunks, "Next Hunk", Box::new(GoToHunk))
                            .action_disabled_when(
                                !has_diff_hunks,
                                "Previous Hunk",
                                Box::new(GoToPreviousHunk),
                            )
                            .separator()
                            .action("Move Line Up", Box::new(MoveLineUp))
                            .action("Move Line Down", Box::new(MoveLineDown))
                            .action("Duplicate Selection", Box::new(DuplicateLineDown))
                    });
                    Some(menu)
                })
        });

        let editor_focus_handle = editor.focus_handle(cx);
        let editor = editor.downgrade();
        let editor_settings_dropdown = {
            let vim_mode_enabled = VimModeSetting::get_global(cx).0;
            let helix_mode_enabled = HelixModeSetting::get_global(cx).0;
            let diff_against_default_branch =
                ProjectSettings::get_global(cx).git.diff_base == GitDiffBaseSetting::DefaultBranch;
            let fs = self
                .workspace
                .upgrade()
                .map(|workspace| workspace.read(cx).app_state().fs.clone());

            PopoverMenu::new("editor-settings")
                .trigger_with_tooltip(
                    IconButton::new("toggle_editor_settings_icon", IconName::Filter)
                        .icon_size(IconSize::Small)
                        .toggle_state(self.toggle_settings_handle.is_deployed()),
                    Tooltip::text("Editor Controls"),
                )
                .anchor(Anchor::TopRight)
                .with_handle(self.toggle_settings_handle.clone())
                .menu(move |window, cx| {
                    let menu = ContextMenu::build(window, cx, {
                        let focus_handle = editor_focus_handle.clone();
                        |mut menu, _, _| {
                            menu = menu.context(focus_handle);

                            if supports_inlay_hints {
                                menu = menu.toggleable_entry(
                                    "Inlay Hints",
                                    inlay_hints_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleInlayHints.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_inlay_hints(
                                                        &editor::actions::ToggleInlayHints,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );

                                menu = menu.toggleable_entry(
                                    "Inline Values",
                                    inline_values_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleInlineValues.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_inline_values(
                                                        &editor::actions::ToggleInlineValues,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    }
                                );
                            }

                            if supports_semantic_tokens {
                                menu = menu.toggleable_entry(
                                    "Semantic Highlights",
                                    semantic_highlights_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleSemanticHighlights.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_semantic_highlights(
                                                        &editor::actions::ToggleSemanticHighlights,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );
                            }

                            if supports_code_lens {
                                menu = menu.toggleable_entry(
                                    "Code Lens",
                                    code_lens_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleCodeLens.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_code_lens_action(
                                                        &editor::actions::ToggleCodeLens,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );
                            }

                            if supports_minimap {
                                menu = menu.toggleable_entry("Minimap", minimap_enabled, IconPosition::Start, Some(editor::actions::ToggleMinimap.boxed_clone()), {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_minimap(
                                                    &editor::actions::ToggleMinimap,
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                    }
                                },)
                            }

                            if has_edit_prediction_provider {
                                let mut edit_prediction_entry = ContextMenuEntry::new("Edit Predictions")
                                    .toggleable(IconPosition::Start, edit_predictions_enabled_at_cursor && show_edit_predictions)
                                    .disabled(!edit_predictions_enabled_at_cursor)
                                    .action(
                                        editor::actions::ToggleEditPrediction.boxed_clone(),
                                    ).handler({
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_edit_predictions(
                                                        &editor::actions::ToggleEditPrediction,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    });
                                if !edit_predictions_enabled_at_cursor {
                                    edit_prediction_entry = edit_prediction_entry.documentation_aside(DocumentationSide::Left, |_| {
                                        Label::new("You can't toggle edit predictions for this file as it is within the excluded files list.").into_any_element()
                                    });
                                }

                                menu = menu.item(edit_prediction_entry);
                            }

                            menu = menu.separator();

                            if is_full {
                                menu = menu.toggleable_entry(
                                    "Diagnostics",
                                    diagnostics_enabled,
                                    IconPosition::Start,
                                    Some(ToggleDiagnostics.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_diagnostics(
                                                        &ToggleDiagnostics,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );

                                if supports_inline_diagnostics {
                                    let mut inline_diagnostics_item = ContextMenuEntry::new("Inline Diagnostics")
                                        .toggleable(IconPosition::Start, diagnostics_enabled && inline_diagnostics_enabled)
                                        .action(ToggleInlineDiagnostics.boxed_clone())
                                        .handler({
                                            let editor = editor.clone();
                                            move |window, cx| {
                                                editor
                                                    .update(cx, |editor, cx| {
                                                        editor.toggle_inline_diagnostics(
                                                            &ToggleInlineDiagnostics,
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                    .ok();
                                            }
                                        });
                                    if !diagnostics_enabled {
                                        inline_diagnostics_item = inline_diagnostics_item.disabled(true).documentation_aside(DocumentationSide::Left, |_|  Label::new("Inline diagnostics are not available until regular diagnostics are enabled.").into_any_element());
                                    }
                                    menu = menu.item(inline_diagnostics_item)
                                }

                                menu = menu.separator();
                            }

                            menu = menu.toggleable_entry(
                                "Line Numbers",
                                show_line_numbers,
                                IconPosition::Start,
                                Some(editor::actions::ToggleLineNumbers.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_line_numbers(
                                                    &editor::actions::ToggleLineNumbers,
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Selection Menu",
                                selection_menu_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleSelectionMenu.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_selection_menu(
                                                    &editor::actions::ToggleSelectionMenu,
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Auto Signature Help",
                                auto_signature_help_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleAutoSignatureHelp.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_auto_signature_help_menu(
                                                    &editor::actions::ToggleAutoSignatureHelp,
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.separator();

                            menu = menu.toggleable_entry(
                                "Inline Git Blame",
                                git_blame_inline_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleGitBlameInline.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_git_blame_inline(
                                                    &editor::actions::ToggleGitBlameInline,
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Column Git Blame",
                                show_git_blame_gutter,
                                IconPosition::Start,
                                Some(git::Blame.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_git_blame(
                                                    &git::Blame,
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .ok();
                                    }
                                },
                            );

                            if let Some(fs) = fs.clone() {
                                menu = menu.toggleable_entry(
                                    "Diff Against Default Branch",
                                    diff_against_default_branch,
                                    IconPosition::Start,
                                    None,
                                    {
                                        move |_window, cx| {
                                            let diff_base = if diff_against_default_branch {
                                                GitDiffBaseSetting::Head
                                            } else {
                                                GitDiffBaseSetting::DefaultBranch
                                            };
                                            update_settings_file(fs.clone(), cx, move |settings, _| {
                                                settings.git.get_or_insert_default().diff_base =
                                                    Some(diff_base);
                                            });
                                        }
                                    },
                                );
                            }

                            menu = menu.separator();

                            menu = menu.toggleable_entry(
                                "Vim Mode",
                                vim_mode_enabled,
                                IconPosition::Start,
                                None,
                                {
                                    move |window, cx| {
                                        let new_value = !vim_mode_enabled;
                                        VimModeSetting::override_global(VimModeSetting(new_value), cx);
                                        HelixModeSetting::override_global(HelixModeSetting(false), cx);
                                        window.refresh();
                                    }
                                },
                            );
                            menu = menu.toggleable_entry(
                                "Helix Mode",
                                helix_mode_enabled,
                                IconPosition::Start,
                                None,
                                {
                                    move |window, cx| {
                                        let new_value = !helix_mode_enabled;
                                        HelixModeSetting::override_global(HelixModeSetting(new_value), cx);
                                        VimModeSetting::override_global(VimModeSetting(false), cx);
                                        window.refresh();
                                    }
                                }
                            );

                            menu
                        }
                    });
                    Some(menu)
                })
        };

        h_flex()
            .id("quick action bar")
            .gap(DynamicSpacing::Base01.rems(cx))
            .children(self.render_repl_menu(cx))
            .children(self.render_preview_button(cx))
            .children(self.render_run_task_button(cx))
            .children(search_button)
            .when(
                AgentSettings::get_global(cx).enabled(cx) && AgentSettings::get_global(cx).button,
                |bar| bar.child(assistant_button),
            )
            .children(code_actions_dropdown)
            .children(editor_selections_dropdown)
            .child(editor_settings_dropdown)
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

#[derive(IntoElement)]
struct QuickActionBarButton {
    id: ElementId,
    icon: IconName,
    toggled: bool,
    action: Box<dyn Action>,
    focus_handle: FocusHandle,
    tooltip: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
}

impl QuickActionBarButton {
    fn new(
        id: impl Into<ElementId>,
        icon: IconName,
        toggled: bool,
        action: Box<dyn Action>,
        focus_handle: FocusHandle,
        tooltip: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            toggled,
            action,
            focus_handle,
            tooltip: tooltip.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for QuickActionBarButton {
    fn render(self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        let tooltip = self.tooltip.clone();
        let action = self.action.boxed_clone();

        IconButton::new(self.id.clone(), self.icon)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .toggle_state(self.toggled)
            .tooltip(move |_window, cx| {
                Tooltip::for_action_in(tooltip.clone(), &*action, &self.focus_handle, cx)
            })
            .on_click(move |event, window, cx| (self.on_click)(event, window, cx))
    }
}

impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_item = active_pane_item.map(ItemHandle::boxed_clone);
        if let Some(active_item) = active_pane_item {
            self._inlay_hints_enabled_subscription.take();

            if let Some(editor) = active_item.downcast::<Editor>() {
                let (
                    mut inlay_hints_enabled,
                    mut supports_inlay_hints,
                    mut supports_semantic_tokens,
                ) = editor.update(cx, |editor, cx| {
                    (
                        editor.inlay_hints_enabled(),
                        editor.supports_inlay_hints(cx),
                        editor.supports_semantic_tokens(cx),
                    )
                });
                self._inlay_hints_enabled_subscription =
                    Some(cx.observe(&editor, move |_, editor, cx| {
                        let (
                            new_inlay_hints_enabled,
                            new_supports_inlay_hints,
                            new_supports_semantic_tokens,
                        ) = editor.update(cx, |editor, cx| {
                            (
                                editor.inlay_hints_enabled(),
                                editor.supports_inlay_hints(cx),
                                editor.supports_semantic_tokens(cx),
                            )
                        });
                        let should_notify = inlay_hints_enabled != new_inlay_hints_enabled
                            || supports_inlay_hints != new_supports_inlay_hints
                            || supports_semantic_tokens != new_supports_semantic_tokens;
                        inlay_hints_enabled = new_inlay_hints_enabled;
                        supports_inlay_hints = new_supports_inlay_hints;
                        supports_semantic_tokens = new_supports_semantic_tokens;
                        if should_notify {
                            cx.notify()
                        }
                    }));
            }
        }
        self.get_toolbar_item_location()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext, UpdateGlobal as _, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;
    use workspace::{MultiWorkspace, OpenOptions, SplitDirection};

    struct FakeTerminalProvider;

    impl workspace::TerminalProvider for FakeTerminalProvider {
        fn spawn(
            &self,
            _task: task::SpawnInTerminal,
            _window: &mut Window,
            _cx: &mut App,
        ) -> gpui::Task<Option<anyhow::Result<std::process::ExitStatus>>> {
            gpui::Task::ready(Some(Ok(std::process::ExitStatus::default())))
        }
    }

    async fn setup(cx: &mut TestAppContext) -> (Entity<Workspace>, &mut VisualTestContext) {
        crate::zed::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {"tasks.json": r#"[
                    {"label":"custom task","command":"echo","args":["$ZED_FILE"]}
                ]"#},
                "main.custom": "first file",
                "other.custom": "second file"
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |view, _| view.workspace().clone());
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    path!("/project/main.custom").into(),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .expect("test file should open");
        cx.run_until_parked();
        workspace.update(cx, |workspace, _| {
            workspace.set_terminal_provider(FakeTerminalProvider);
        });
        (workspace, cx)
    }

    fn set_task(name: &str, cx: &mut VisualTestContext) {
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.toolbar.get_or_insert_default().run_task = Some(name.into());
                });
            });
        });
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_run_task_button_settings_and_click(cx: &mut TestAppContext) {
        let (workspace, cx) = setup(cx).await;
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());

        set_task("custom task", cx);
        let bounds = cx
            .debug_bounds("ICON-PlayOutlined")
            .expect("setting a task should show the run button for any saved file type");
        cx.simulate_click(bounds.center(), Modifiers::none());
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            let inventory = workspace
                .project()
                .read(cx)
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
                .expect("task inventory should exist");
            let (_, task) = inventory
                .read(cx)
                .last_scheduled_task(None)
                .expect("click should schedule the configured task");
            assert_eq!(
                task.resolved.args,
                vec![path!("/project/main.custom").to_string()]
            );
        });

        set_task("missing task", cx);
        let bounds = cx
            .debug_bounds("ICON-PlayOutlined")
            .expect("button should remain visible");
        cx.simulate_click(bounds.center(), Modifiers::none());
        cx.run_until_parked();
        assert!(workspace.update_in(cx, |workspace, window, cx| {
            workspace.has_active_modal(window, cx)
        }));

        set_task("", cx);
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());
    }

    #[gpui::test]
    async fn test_run_task_targets_its_own_pane(cx: &mut TestAppContext) {
        let (workspace, cx) = setup(cx).await;
        let left_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        let left_item = workspace.read_with(cx, |workspace, cx| {
            workspace.active_item(cx).expect("left item should exist")
        });
        workspace.update_in(cx, |workspace, window, cx| {
            let right = workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Right,
                window,
                cx,
            );
            right.read(cx).focus_handle(cx).focus(window, cx);
        });
        cx.run_until_parked();
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    path!("/project/other.custom").into(),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .expect("right file should open");
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_ne!(workspace.active_pane().entity_id(), left_pane.entity_id());
            assert_ne!(
                workspace.active_item(cx).expect("right item").item_id(),
                left_item.item_id()
            );
        });
        let handle = workspace.downgrade();
        cx.update(|window, cx| {
            run_task_for_item(
                &handle,
                left_item.as_ref(),
                &tasks_ui::Spawn::ByName {
                    task_name: "custom task".into(),
                    reveal_target: None,
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).expect("active item").item_id(),
                left_item.item_id()
            );
            let inventory = workspace
                .project()
                .read(cx)
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
                .expect("task inventory should exist");
            let (_, task) = inventory
                .read(cx)
                .last_scheduled_task(None)
                .expect("task should be scheduled");
            assert_eq!(
                task.resolved.args,
                vec![path!("/project/main.custom").to_string()]
            );
        });
    }

    #[gpui::test]
    async fn test_run_task_button_hidden_for_untitled_and_disabled_toolbar(
        cx: &mut TestAppContext,
    ) {
        let (workspace, cx) = setup(cx).await;
        set_task("custom task", cx);
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_some());
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .editor
                        .toolbar
                        .get_or_insert_default()
                        .quick_actions = Some(false);
                });
            });
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .editor
                        .toolbar
                        .get_or_insert_default()
                        .quick_actions = Some(true);
                });
            });
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_some());
        let untitled = cx.new_window_entity(|window, cx| Editor::multi_line(window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_center(Box::new(untitled), window, cx);
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());
    }

    #[gpui::test]
    async fn test_run_task_button_appears_after_first_save(cx: &mut TestAppContext) {
        let (workspace, cx) = setup(cx).await;
        set_task("custom task", cx);
        let untitled = cx.new_window_entity(|window, cx| Editor::multi_line(window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_center(Box::new(untitled.clone()), window, cx);
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());

        let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
        let worktree_id = project.read_with(cx, |project, cx| {
            project
                .visible_worktrees(cx)
                .next()
                .expect("test worktree")
                .read(cx)
                .id()
        });
        let buffer = untitled.read_with(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("untitled buffer")
        });
        project
            .update(cx, |project, cx| {
                project.save_buffer_as(buffer, (worktree_id, rel_path("saved.custom")).into(), cx)
            })
            .await
            .expect("first save should succeed");
        cx.run_until_parked();
        assert!(
            cx.debug_bounds("ICON-PlayOutlined").is_some(),
            "saving an untitled file should show the button without switching tabs"
        );
    }

    #[gpui::test]
    async fn test_run_task_button_hidden_for_excerpts(cx: &mut TestAppContext) {
        let (workspace, cx) = setup(cx).await;
        set_task("custom task", cx);
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_some());
        let (project, buffer) = workspace.read_with(cx, |workspace, cx| {
            let editor = workspace
                .active_item_as::<Editor>(cx)
                .expect("saved editor");
            (
                workspace.project().clone(),
                editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .expect("saved buffer"),
            )
        });
        let excerpts = cx.new(|cx| {
            let mut excerpts = editor::MultiBuffer::new(project.read(cx).capability());
            excerpts.set_excerpts_for_path(
                editor::PathKey::sorted(0),
                buffer,
                [language::Point::new(0, 0)..language::Point::new(0, 5)],
                0,
                cx,
            );
            excerpts
        });
        let editor = cx.new_window_entity(|window, cx| {
            Editor::for_multibuffer(excerpts, Some(project), window, cx)
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_center(Box::new(editor), window, cx);
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-PlayOutlined").is_none());
    }
}
