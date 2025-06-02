use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use editor::Editor;
use feature_flags::{DebuggerFeatureFlag, FeatureFlagViewExt};
use gpui::{App, EntityInputHandler, actions};
use new_process_modal::{NewProcessModal, NewProcessMode};
use project::debugger::{self, breakpoint_store::SourceBreakpoint};
use session::DebugSession;
use settings::Settings;
use stack_trace_view::StackTraceView;
use tasks_ui::{Spawn, TaskOverrides};
use util::maybe;
use workspace::{ItemHandle, ShutdownDebugAdapters, Workspace};

pub mod attach_modal;
pub mod debugger_panel;
mod dropdown_menus;
mod new_process_modal;
mod persistence;
pub(crate) mod session;
mod stack_trace_view;

#[cfg(any(test, feature = "test-support"))]
pub mod tests;

actions!(
    debugger,
    [
        Start,
        Continue,
        Detach,
        Pause,
        Restart,
        StepInto,
        StepOver,
        StepOut,
        StepBack,
        Stop,
        ToggleIgnoreBreakpoints,
        ClearAllBreakpoints,
        FocusConsole,
        FocusVariables,
        FocusBreakpointList,
        FocusFrames,
        FocusModules,
        FocusLoadedSources,
        FocusTerminal,
        ShowStackTrace,
        ToggleThreadPicker,
        ToggleSessionPicker,
        RerunLastSession,
    ]
);

pub fn init(cx: &mut App) {
    DebuggerSettings::register(cx);
    workspace::FollowableViewRegistry::register::<DebugSession>(cx);

    cx.observe_new(|_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        cx.when_flag_enabled::<DebuggerFeatureFlag>(window, |workspace, _, _| {
            workspace
                .register_action(spawn_task_or_modal)
                .register_action(|workspace, _: &ToggleFocus, window, cx| {
                    workspace.toggle_panel_focus::<DebugPanel>(window, cx);
                })
                .register_action(|workspace, _: &Pause, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.pause_thread(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Restart, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.restart_session(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Continue, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.continue_thread(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepInto, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.step_in(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepOver, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.step_over(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepOut, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.step_out(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepBack, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.step_back(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Stop, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            cx.defer(move |cx| {
                                active_item.update(cx, |item, cx| item.stop_thread(cx))
                            })
                        }
                    }
                })
                .register_action(|workspace, _: &ToggleIgnoreBreakpoints, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel
                            .read(cx)
                            .active_session()
                            .map(|session| session.read(cx).running_state().clone())
                        {
                            active_item.update(cx, |item, cx| item.toggle_ignore_breakpoints(cx))
                        }
                    }
                })
                .register_action(
                    |workspace: &mut Workspace, _: &ShutdownDebugAdapters, _window, cx| {
                        workspace.project().update(cx, |project, cx| {
                            project.dap_store().update(cx, |store, cx| {
                                store.shutdown_sessions(cx).detach();
                            })
                        })
                    },
                )
                .register_action(
                    |workspace: &mut Workspace, _: &ShowStackTrace, window, cx| {
                        let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
                            return;
                        };

                        if let Some(existing) = workspace.item_of_type::<StackTraceView>(cx) {
                            let is_active = workspace
                                .active_item(cx)
                                .is_some_and(|item| item.item_id() == existing.item_id());
                            workspace.activate_item(&existing, true, !is_active, window, cx);
                        } else {
                            let Some(active_session) = debug_panel.read(cx).active_session() else {
                                return;
                            };

                            let project = workspace.project();

                            let stack_trace_view = active_session.update(cx, |session, cx| {
                                session.stack_trace_view(project, window, cx).clone()
                            });

                            workspace.add_item_to_active_pane(
                                Box::new(stack_trace_view),
                                None,
                                true,
                                window,
                                cx,
                            );
                        }
                    },
                )
                .register_action(|workspace: &mut Workspace, _: &Start, window, cx| {
                    NewProcessModal::show(workspace, window, NewProcessMode::Debug, None, cx);
                })
                .register_action(
                    |workspace: &mut Workspace, _: &RerunLastSession, window, cx| {
                        let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
                            return;
                        };

                        debug_panel.update(cx, |debug_panel, cx| {
                            debug_panel.rerun_last_session(workspace, window, cx);
                        })
                    },
                );
        })
    })
    .detach();

    cx.observe_new({
        move |editor: &mut Editor, _, cx| {
            editor
                .register_action(cx.listener(
                    move |editor, _: &editor::actions::DebuggerRunToCursor, _, cx| {
                        maybe!({
                            let debug_panel =
                                editor.workspace()?.read(cx).panel::<DebugPanel>(cx)?;
                            let cursor_point: language::Point = editor.selections.newest(cx).head();
                            let active_session = debug_panel.read(cx).active_session()?;

                            let (buffer, position, _) = editor
                                .buffer()
                                .read(cx)
                                .point_to_buffer_point(cursor_point, cx)?;

                            let path =
                                debugger::breakpoint_store::BreakpointStore::abs_path_from_buffer(
                                    &buffer, cx,
                                )?;

                            let source_breakpoint = SourceBreakpoint {
                                row: position.row,
                                path,
                                message: None,
                                condition: None,
                                hit_condition: None,
                                state: debugger::breakpoint_store::BreakpointState::Enabled,
                            };

                            active_session.update(cx, |session, cx| {
                                session.running_state().update(cx, |state, cx| {
                                    if let Some(thread_id) = state.selected_thread_id() {
                                        state.session().update(cx, |session, cx| {
                                            session.run_to_position(
                                                source_breakpoint,
                                                thread_id,
                                                cx,
                                            );
                                        })
                                    }
                                });
                            });

                            Some(())
                        });
                    },
                ))
                .detach();

            editor
                .register_action(cx.listener(
                    move |editor, _: &editor::actions::DebuggerEvaluateSelectedText, window, cx| {
                        maybe!({
                            let debug_panel =
                                editor.workspace()?.read(cx).panel::<DebugPanel>(cx)?;
                            let active_session = debug_panel.read(cx).active_session()?;

                            let text = editor.text_for_range(
                                editor.selections.newest(cx).range(),
                                &mut None,
                                window,
                                cx,
                            )?;

                            active_session.update(cx, |session, cx| {
                                session.running_state().update(cx, |state, cx| {
                                    let stack_id = state.selected_stack_frame_id(cx);

                                    state.session().update(cx, |session, cx| {
                                        session.evaluate(text, None, stack_id, None, cx).detach();
                                    });
                                });
                            });

                            Some(())
                        });
                    },
                ))
                .detach();
        }
    })
    .detach();
}

fn spawn_task_or_modal(
    workspace: &mut Workspace,
    action: &Spawn,
    window: &mut ui::Window,
    cx: &mut ui::Context<Workspace>,
) {
    match action {
        Spawn::ByName {
            task_name,
            reveal_target,
        } => {
            let overrides = reveal_target.map(|reveal_target| TaskOverrides {
                reveal_target: Some(reveal_target),
            });
            let name = task_name.clone();
            tasks_ui::spawn_tasks_filtered(
                move |(_, task)| task.label.eq(&name),
                overrides,
                window,
                cx,
            )
            .detach_and_log_err(cx)
        }
        Spawn::ByTag {
            task_tag,
            reveal_target,
        } => {
            let overrides = reveal_target.map(|reveal_target| TaskOverrides {
                reveal_target: Some(reveal_target),
            });
            let tag = task_tag.clone();
            tasks_ui::spawn_tasks_filtered(
                move |(_, task)| task.tags.contains(&tag),
                overrides,
                window,
                cx,
            )
            .detach_and_log_err(cx)
        }
        Spawn::ViaModal { reveal_target } => {
            NewProcessModal::show(workspace, window, NewProcessMode::Task, *reveal_target, cx);
        }
    }
}
