use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use editor::Editor;
use feature_flags::{DebuggerFeatureFlag, FeatureFlagViewExt};
use gpui::{App, EntityInputHandler, actions};
use new_session_modal::NewSessionModal;
use project::debugger::{self, breakpoint_store::SourceBreakpoint};
use session::DebugSession;
use settings::Settings;
use util::maybe;
use workspace::{ShutdownDebugAdapters, Workspace};

pub mod attach_modal;
pub mod debugger_panel;
mod new_session_modal;
mod persistence;
pub(crate) mod session;

#[cfg(any(test, feature = "test-support"))]
pub mod tests;

actions!(
    debugger,
    [
        Start,
        Continue,
        Disconnect,
        Pause,
        Restart,
        StepInto,
        StepOver,
        StepOut,
        StepBack,
        Stop,
        ToggleIgnoreBreakpoints,
        ClearAllBreakpoints,
        CreateDebuggingSession,
        FocusConsole,
        FocusVariables,
        FocusBreakpointList,
        FocusFrames,
        FocusModules,
        FocusLoadedSources,
        FocusTerminal,
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
                .register_action(|workspace, _: &ToggleFocus, window, cx| {
                    workspace.toggle_panel_focus::<DebugPanel>(window, cx);
                })
                .register_action(|workspace, _: &Pause, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.pause_thread(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Restart, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.restart_session(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepInto, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.step_in(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepOver, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.step_over(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepBack, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            active_item.update(cx, |item, cx| item.step_back(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Stop, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
                            cx.defer(move |cx| {
                                active_item.update(cx, |item, cx| item.stop_thread(cx))
                            })
                        }
                    }
                })
                .register_action(|workspace, _: &ToggleIgnoreBreakpoints, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session()
                                .map(|session| session.read(cx).running_state().clone())
                        }) {
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
                    |workspace: &mut Workspace, _: &CreateDebuggingSession, window, cx| {
                        if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                            let weak_panel = debug_panel.downgrade();
                            let weak_workspace = cx.weak_entity();

                            workspace.toggle_modal(window, cx, |window, cx| {
                                NewSessionModal::new(
                                    debug_panel.read(cx).past_debug_definition.clone(),
                                    weak_panel,
                                    weak_workspace,
                                    None,
                                    window,
                                    cx,
                                )
                            });
                        }
                    },
                )
                .register_action(|workspace: &mut Workspace, _: &Start, window, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        let weak_panel = debug_panel.downgrade();
                        let weak_workspace = cx.weak_entity();
                        let task_store = workspace.project().read(cx).task_store().clone();

                        workspace.toggle_modal(window, cx, |window, cx| {
                            NewSessionModal::new(
                                debug_panel.read(cx).past_debug_definition.clone(),
                                weak_panel,
                                weak_workspace,
                                Some(task_store),
                                window,
                                cx,
                            )
                        });
                    }
                });
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
