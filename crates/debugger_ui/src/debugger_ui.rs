use std::any::TypeId;

use debugger_panel::DebugPanel;
use editor::{Editor, MultiBufferOffsetUtf16};
use gpui::{Action, App, DispatchPhase, EntityInputHandler, actions};
use new_process_modal::{NewProcessModal, NewProcessMode};
use onboarding_modal::DebuggerOnboardingModal;
use project::debugger::{self, breakpoint_store::SourceBreakpoint, session::ThreadStatus};
use schemars::JsonSchema;
use serde::Deserialize;
use session::DebugSession;
use stack_trace_view::StackTraceView;
use tasks_ui::{Spawn, TaskOverrides};
use ui::{FluentBuilder, InteractiveElement};
use util::maybe;
use workspace::{ItemHandle, ShutdownDebugAdapters, Workspace};
use zed_actions::ToggleFocus;
use zed_actions::debugger::OpenOnboardingModal;

pub mod attach_modal;
pub mod debugger_panel;
mod dropdown_menus;
mod new_process_modal;
mod onboarding_modal;
mod persistence;
pub(crate) mod session;
mod stack_trace_view;

#[cfg(any(test, feature = "test-support"))]
pub mod tests;

actions!(
    debugger,
    [
        /// Starts a new debugging session.
        Start,
        /// Continues execution until the next breakpoint.
        Continue,
        /// Detaches the debugger from the running process.
        Detach,
        /// Pauses the currently running program.
        Pause,
        /// Restarts the current debugging session.
        Restart,
        /// Reruns the current debugging session with the same configuration.
        RerunSession,
        /// Steps into the next function call.
        StepInto,
        /// Steps over the current line.
        StepOver,
        /// Steps out of the current function.
        StepOut,
        /// Steps back to the previous statement.
        StepBack,
        /// Stops the debugging session.
        Stop,
        /// Toggles whether to ignore all breakpoints.
        ToggleIgnoreBreakpoints,
        /// Clears all breakpoints in the project.
        ClearAllBreakpoints,
        /// Focuses on the debugger console panel.
        FocusConsole,
        /// Focuses on the variables panel.
        FocusVariables,
        /// Focuses on the breakpoint list panel.
        FocusBreakpointList,
        /// Focuses on the call stack frames panel.
        FocusFrames,
        /// Focuses on the loaded modules panel.
        FocusModules,
        /// Focuses on the loaded sources panel.
        FocusLoadedSources,
        /// Focuses on the terminal panel.
        FocusTerminal,
        /// Shows the stack trace for the current thread.
        ShowStackTrace,
        /// Toggles the thread picker dropdown.
        ToggleThreadPicker,
        /// Toggles the session picker dropdown.
        ToggleSessionPicker,
        /// Reruns the last debugging session.
        #[action(deprecated_aliases = ["debugger::RerunLastSession"])]
        Rerun,
        /// Toggles expansion of the selected item in the debugger UI.
        ToggleExpandItem,
        /// Toggle the user frame filter in the stack frame list
        /// When toggled on, only frames from the user's code are shown
        /// When toggled off, all frames are shown
        ToggleUserFrames,
    ]
);

/// Extends selection down by a specified number of lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = debugger)]
#[serde(deny_unknown_fields)]
/// Set a data breakpoint on the selected variable or memory region.
pub struct ToggleDataBreakpoint {
    /// The type of data breakpoint
    /// Read & Write
    /// Read
    /// Write
    #[serde(default)]
    pub access_type: Option<dap::DataBreakpointAccessType>,
}

actions!(
    dev,
    [
        /// Copies debug adapter launch arguments to clipboard.
        CopyDebugAdapterArguments
    ]
);

pub fn init(cx: &mut App) {
    workspace::FollowableViewRegistry::register::<DebugSession>(cx);

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace
            .register_action(spawn_task_or_modal)
            .register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<DebugPanel>(window, cx);
            })
            .register_action(|workspace: &mut Workspace, _: &Start, window, cx| {
                NewProcessModal::show(workspace, window, NewProcessMode::Debug, None, cx);
            })
            .register_action(|workspace: &mut Workspace, _: &Rerun, window, cx| {
                let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
                    return;
                };

                debug_panel.update(cx, |debug_panel, cx| {
                    debug_panel.rerun_last_session(workspace, window, cx);
                })
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
            .register_action(|workspace, _: &OpenOnboardingModal, window, cx| {
                DebuggerOnboardingModal::toggle(workspace, window, cx)
            })
            .register_action_renderer(|div, workspace, _, cx| {
                let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
                    return div;
                };
                let Some(active_item) = debug_panel
                    .read(cx)
                    .active_session()
                    .map(|session| session.read(cx).running_state().clone())
                else {
                    return div;
                };
                let running_state = active_item.read(cx);
                if running_state.session().read(cx).is_terminated() {
                    return div;
                }

                let caps = running_state.capabilities(cx);
                let supports_step_back = caps.supports_step_back.unwrap_or_default();
                let supports_detach = running_state.session().read(cx).is_attached();
                let status = running_state.thread_status(cx);

                let active_item = active_item.downgrade();
                div.when(status == Some(ThreadStatus::Running), |div| {
                    let active_item = active_item.clone();
                    div.on_action(move |_: &Pause, _, cx| {
                        active_item
                            .update(cx, |item, cx| item.pause_thread(cx))
                            .ok();
                    })
                })
                .when(status == Some(ThreadStatus::Stopped), |div| {
                    div.on_action({
                        let active_item = active_item.clone();
                        move |_: &StepInto, _, cx| {
                            active_item.update(cx, |item, cx| item.step_in(cx)).ok();
                        }
                    })
                    .on_action({
                        let active_item = active_item.clone();
                        move |_: &StepOver, _, cx| {
                            active_item.update(cx, |item, cx| item.step_over(cx)).ok();
                        }
                    })
                    .on_action({
                        let active_item = active_item.clone();
                        move |_: &StepOut, _, cx| {
                            active_item.update(cx, |item, cx| item.step_out(cx)).ok();
                        }
                    })
                    .when(supports_step_back, |div| {
                        let active_item = active_item.clone();
                        div.on_action(move |_: &StepBack, _, cx| {
                            active_item.update(cx, |item, cx| item.step_back(cx)).ok();
                        })
                    })
                    .on_action({
                        let active_item = active_item.clone();
                        move |_: &Continue, _, cx| {
                            active_item
                                .update(cx, |item, cx| item.continue_thread(cx))
                                .ok();
                        }
                    })
                    .on_action(cx.listener(
                        |workspace, _: &ShowStackTrace, window, cx| {
                            let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
                                return;
                            };

                            if let Some(existing) = workspace.item_of_type::<StackTraceView>(cx) {
                                let is_active = workspace
                                    .active_item(cx)
                                    .is_some_and(|item| item.item_id() == existing.item_id());
                                workspace.activate_item(&existing, true, !is_active, window, cx);
                            } else {
                                let Some(active_session) = debug_panel.read(cx).active_session()
                                else {
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
                    ))
                })
                .when(supports_detach, |div| {
                    let active_item = active_item.clone();
                    div.on_action(move |_: &Detach, _, cx| {
                        active_item
                            .update(cx, |item, cx| item.detach_client(cx))
                            .ok();
                    })
                })
                .on_action({
                    let active_item = active_item.clone();
                    move |_: &Restart, _, cx| {
                        active_item
                            .update(cx, |item, cx| item.restart_session(cx))
                            .ok();
                    }
                })
                .on_action({
                    let active_item = active_item.clone();
                    move |_: &RerunSession, window, cx| {
                        active_item
                            .update(cx, |item, cx| item.rerun_session(window, cx))
                            .ok();
                    }
                })
                .on_action({
                    let active_item = active_item.clone();
                    move |_: &Stop, _, cx| {
                        active_item.update(cx, |item, cx| item.stop_thread(cx)).ok();
                    }
                })
                .on_action({
                    let active_item = active_item.clone();
                    move |_: &ToggleIgnoreBreakpoints, _, cx| {
                        active_item
                            .update(cx, |item, cx| item.toggle_ignore_breakpoints(cx))
                            .ok();
                    }
                })
                .on_action(move |_: &ToggleUserFrames, _, cx| {
                    if let Some((thread_status, stack_frame_list)) = active_item
                        .read_with(cx, |item, cx| {
                            (item.thread_status(cx), item.stack_frame_list().clone())
                        })
                        .ok()
                    {
                        stack_frame_list.update(cx, |stack_frame_list, cx| {
                            stack_frame_list.toggle_frame_filter(thread_status, cx);
                        })
                    }
                })
            });
    })
    .detach();

    cx.observe_new({
        move |editor: &mut Editor, _, _| {
            editor
                .register_action_renderer(move |editor, window, cx| {
                    let Some(workspace) = editor.workspace() else {
                        return;
                    };
                    let Some(debug_panel) = workspace.read(cx).panel::<DebugPanel>(cx) else {
                        return;
                    };
                    let Some(active_session) =
                        debug_panel.update(cx, |panel, _| panel.active_session())
                    else {
                        return;
                    };

                    let session = active_session
                        .read(cx)
                        .running_state
                        .read(cx)
                        .session()
                        .read(cx);

                    if session.is_terminated() {
                        return;
                    }

                    let editor = cx.entity().downgrade();

                    window.on_action_when(
                        session.any_stopped_thread(),
                        TypeId::of::<editor::actions::RunToCursor>(),
                        {
                            let editor = editor.clone();
                            let active_session = active_session.clone();
                            move |_, phase, _, cx| {
                                if phase != DispatchPhase::Bubble {
                                    return;
                                }
                                maybe!({
                                    let (buffer, position, _) = editor
                                        .update(cx, |editor, cx| {
                                            let cursor_point: language::Point = editor
                                                .selections
                                                .newest(&editor.display_snapshot(cx))
                                                .head();

                                            editor
                                                .buffer()
                                                .read(cx)
                                                .point_to_buffer_point(cursor_point, cx)
                                        })
                                        .ok()??;

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
                            }
                        },
                    );

                    window.on_action(
                        TypeId::of::<editor::actions::EvaluateSelectedText>(),
                        move |_, _, window, cx| {
                            let status = maybe!({
                                let text = editor
                                    .update(cx, |editor, cx| {
                                        let range = editor
                                            .selections
                                            .newest::<MultiBufferOffsetUtf16>(
                                                &editor.display_snapshot(cx),
                                            )
                                            .range();
                                        editor.text_for_range(
                                            range.start.0.0..range.end.0.0,
                                            &mut None,
                                            window,
                                            cx,
                                        )
                                    })
                                    .ok()??;

                                active_session.update(cx, |session, cx| {
                                    session.running_state().update(cx, |state, cx| {
                                        let stack_id = state.selected_stack_frame_id(cx);

                                        state.session().update(cx, |session, cx| {
                                            session
                                                .evaluate(
                                                    text,
                                                    Some(dap::EvaluateArgumentsContext::Repl),
                                                    stack_id,
                                                    None,
                                                    cx,
                                                )
                                                .detach();
                                        });
                                    });
                                });

                                Some(())
                            });
                            if status.is_some() {
                                cx.stop_propagation();
                            }
                        },
                    );
                })
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
