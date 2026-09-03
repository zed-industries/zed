use editor::Editor;
use gpui::TaskExt;
use gpui::{App, Entity, Window};
use language::LanguageName;
use repl::{
    ExecutionState, JupyterSettings, Kernel, KernelSpecification, KernelStatus, Session,
    SessionSupport, worktree_id_for_editor,
};
use ui::{ContextMenu, Indicator, prelude::*};
use util::ResultExt;

use super::{
    QuickActionBarItem, QuickActionButton, QuickActionElement, QuickActionKernelSelector,
    QuickActionMenu, QuickActionTarget, VisibilityTrigger,
};

const ZED_REPL_DOCUMENTATION: &str = "https://zed.dev/docs/repl";

struct ReplMenuState {
    tooltip: SharedString,
    icon: IconName,
    icon_color: Color,
    icon_is_animating: bool,
    popover_disabled: bool,
    indicator: Option<Indicator>,

    status: KernelStatus,
    kernel_name: SharedString,
    kernel_language: SharedString,
}

pub(super) struct ReplMenu;

#[derive(PartialEq)]
pub(super) struct ReplContext {
    editor: Entity<Editor>,
    state: ReplState,
}

/// The [`SessionSupport`] states the menu has a representation for.
#[derive(PartialEq)]
enum ReplState {
    Inactive(KernelSpecification),
    RequiresSetup(LanguageName),
    Active(Entity<Session>),
}

impl QuickActionBarItem for ReplMenu {
    type Context = ReplContext;

    const ID: &'static str = "repl-menu";

    const TRIGGERS: &'static [VisibilityTrigger] = &[
        VisibilityTrigger::Settings,
        VisibilityTrigger::Editor,
        // Whether a language is supported depends on kernelspecs the store loads asynchronously,
        // and sessions starting or shutting down are only announced by the store.
        VisibilityTrigger::ReplStore,
    ];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<Self::Context> {
        if !JupyterSettings::enabled(cx) {
            return None;
        }

        let editor = target.editor()?;

        let is_valid_project = editor
            .read(cx)
            .workspace()
            .map(|workspace| {
                let project = workspace.read(cx).project().read(cx);
                !project.is_via_collab()
            })
            .unwrap_or(false);

        if !is_valid_project {
            return None;
        }

        let state = match repl::session(editor.downgrade(), cx) {
            SessionSupport::ActiveSession(session) => ReplState::Active(session),
            SessionSupport::Inactive(kernel_specification) => {
                ReplState::Inactive(kernel_specification)
            }
            SessionSupport::RequiresSetup(language) => ReplState::RequiresSetup(language),
            SessionSupport::Unsupported => return None,
        };

        Some(ReplContext {
            editor: editor.clone(),
            state,
        })
    }

    fn render(
        &self,
        context: &Self::Context,
        _window: &mut Window,
        cx: &mut App,
    ) -> QuickActionElement {
        let editor = &context.editor;
        let mut elements = Vec::new();
        elements.extend(kernel_selector(editor, &context.state, cx));

        match &context.state {
            ReplState::Inactive(kernel_specification) => {
                elements.push(QuickActionElement::Button(
                    QuickActionButton::new(
                        IconName::ReplNeutral,
                        format!("Start REPL for {}", kernel_specification.name()),
                        |window, cx| window.dispatch_action(Box::new(repl::Run {}), cx),
                    )
                    .action(Box::new(repl::Run {}))
                    .icon_color(Color::Muted),
                ));
            }
            ReplState::RequiresSetup(language) => {
                elements.push(QuickActionElement::Button(
                    QuickActionButton::new(
                        IconName::ReplNeutral,
                        format!("Setup Zed REPL for {}", language),
                        |_window, cx| {
                            cx.open_url(&format!("{}#installation", ZED_REPL_DOCUMENTATION))
                        },
                    )
                    .icon_color(Color::Muted),
                ));
            }
            ReplState::Active(session) => {
                elements.push(active_session_button(editor, session, cx));
            }
        }

        QuickActionElement::Group(elements)
    }
}

fn kernel_selector(
    editor: &Entity<Editor>,
    state: &ReplState,
    cx: &mut App,
) -> Option<QuickActionElement> {
    let worktree_id = worktree_id_for_editor(editor.downgrade(), cx)?;

    let store = repl::ReplStore::global(cx);
    if !store.read(cx).has_python_kernelspecs(worktree_id) {
        if let Some(project) = editor
            .read(cx)
            .workspace()
            .map(|workspace| workspace.read(cx).project().clone())
        {
            store
                .update(cx, |store, cx| {
                    store.refresh_python_kernelspecs(worktree_id, &project, cx)
                })
                .detach_and_log_err(cx);
        }
    }

    let current_kernel = match state {
        ReplState::Active(session) => Some(session.read(cx).kernel_specification.name()),
        ReplState::Inactive(kernel_specification) => Some(kernel_specification.name()),
        ReplState::RequiresSetup(_) => None,
    };

    let editor = editor.downgrade();
    Some(QuickActionElement::KernelSelector(
        QuickActionKernelSelector {
            current_kernel,
            worktree_id,
            on_select: Box::new(move |kernelspec, window, cx| {
                if kernelspec.has_ipykernel() {
                    repl::assign_kernelspec(kernelspec, editor.clone(), window, cx).ok();
                } else {
                    repl::install_ipykernel_and_assign(kernelspec, editor.clone(), window, cx).ok();
                }
            }),
        },
    ))
}

fn active_session_button(
    editor: &Entity<Editor>,
    session: &Entity<Session>,
    cx: &mut App,
) -> QuickActionElement {
    let menu_state = session_state(session.clone(), cx);

    let button = QuickActionButton::new(menu_state.icon, menu_state.tooltip, |window, cx| {
        window.dispatch_action(Box::new(repl::Run), cx)
    })
    .action(Box::new(repl::Run))
    .icon_color(menu_state.icon_color)
    .indicator(menu_state.indicator)
    .animating(menu_state.icon_is_animating);

    let weak_editor = editor.downgrade();
    let session = session.clone();
    let menu = QuickActionMenu::new("REPL Menu", move |window, cx| {
        let editor = weak_editor.clone();
        let session = session.clone();
        let has_nonempty_selection = editor
            .update(cx, |editor, cx| {
                editor.selections.count() != 0 && {
                    let snapshot = editor.display_snapshot(cx);
                    !editor.selections.newest_display(&snapshot).is_empty()
                }
            })
            .unwrap_or(false);
        ContextMenu::build(window, cx, move |menu, _, cx| {
            let menu_state = session_state(session, cx);
            let status = menu_state.status;
            let editor = editor.clone();

            menu.map(|menu| {
                if status.is_connected() {
                    let status = status.clone();
                    menu.custom_row(move |_window, _cx| {
                        h_flex()
                            .child(
                                Label::new(format!(
                                    "kernel: {} ({})",
                                    menu_state.kernel_name, menu_state.kernel_language
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            )
                            .into_any_element()
                    })
                    .custom_row(move |_window, _cx| {
                        h_flex()
                            .child(
                                Label::new(status.clone().to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .into_any_element()
                    })
                } else {
                    let status = status.clone();
                    menu.custom_row(move |_window, _cx| {
                        h_flex()
                            .child(
                                Label::new(format!("{}...", status.to_string()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .into_any_element()
                    })
                }
            })
            .separator()
            .custom_entry(
                move |_window, _cx| {
                    Label::new(if has_nonempty_selection {
                        "Run Selection"
                    } else {
                        "Run Line"
                    })
                    .into_any_element()
                },
                {
                    let editor = editor.clone();
                    move |window, cx| {
                        repl::run(editor.clone(), true, window, cx).log_err();
                    }
                },
            )
            .custom_entry(
                move |_window, _cx| {
                    Label::new("Interrupt")
                        .size(LabelSize::Small)
                        .color(Color::Error)
                        .into_any_element()
                },
                {
                    let editor = editor.clone();
                    move |_, cx| {
                        repl::interrupt(editor.clone(), cx);
                    }
                },
            )
            .custom_entry(
                move |_window, _cx| {
                    Label::new("Clear Outputs")
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .into_any_element()
                },
                {
                    let editor = editor.clone();
                    move |_, cx| {
                        repl::clear_outputs(editor.clone(), cx);
                    }
                },
            )
            .separator()
            .custom_entry(
                move |_window, _cx| {
                    Label::new("Shut Down Kernel")
                        .size(LabelSize::Small)
                        .color(Color::Error)
                        .into_any_element()
                },
                {
                    let editor = editor.clone();
                    move |window, cx| {
                        repl::shutdown(editor.clone(), window, cx);
                    }
                },
            )
            .custom_entry(
                move |_window, _cx| {
                    Label::new("Restart Kernel")
                        .size(LabelSize::Small)
                        .color(Color::Error)
                        .into_any_element()
                },
                {
                    move |window, cx| {
                        repl::restart(editor.clone(), window, cx);
                    }
                },
            )
            .separator()
            .action("View Sessions", Box::new(repl::Sessions))
            // TODO: Add shut down all kernels action
            // .action("Shut Down all Kernels", Box::new(gpui::NoAction))
        })
    })
    .disabled(menu_state.popover_disabled);

    QuickActionElement::SplitButton { button, menu }
}

fn session_state(session: Entity<Session>, cx: &mut App) -> ReplMenuState {
    let session = session.read(cx);

    let kernel_name = session.kernel_specification.name();
    let kernel_language: SharedString = session.kernel_specification.language();

    let fill_fields = || {
        ReplMenuState {
            tooltip: "Nothing running".into(),
            icon: IconName::ReplNeutral,
            icon_color: Color::Default,
            icon_is_animating: false,
            popover_disabled: false,
            indicator: None,
            kernel_name: kernel_name.clone(),
            kernel_language: kernel_language.clone(),
            // TODO: Technically not shutdown, but indeterminate
            status: KernelStatus::Shutdown,
            // current_delta: Duration::default(),
        }
    };

    let transitional =
        |tooltip: SharedString, animating: bool, popover_disabled: bool| ReplMenuState {
            tooltip,
            icon_is_animating: animating,
            popover_disabled,
            icon_color: Color::Muted,
            indicator: Some(Indicator::dot().color(Color::Muted)),
            status: session.kernel.status(),
            ..fill_fields()
        };

    let starting = || transitional(format!("{} is starting", kernel_name).into(), true, true);
    let restarting = || transitional(format!("Restarting {}", kernel_name).into(), true, true);
    let shutting_down = || {
        transitional(
            format!("{} is shutting down", kernel_name).into(),
            false,
            true,
        )
    };
    let auto_restarting = || {
        transitional(
            format!("Auto-restarting {}", kernel_name).into(),
            true,
            true,
        )
    };
    let unknown = || transitional(format!("{} state unknown", kernel_name).into(), false, true);
    let other = |state: &str| {
        transitional(
            format!("{} state: {}", kernel_name, state).into(),
            false,
            true,
        )
    };

    let shutdown = || ReplMenuState {
        tooltip: "Nothing running".into(),
        icon: IconName::ReplNeutral,
        icon_color: Color::Default,
        icon_is_animating: false,
        popover_disabled: false,
        indicator: None,
        status: KernelStatus::Shutdown,
        ..fill_fields()
    };

    match &session.kernel {
        Kernel::Restarting => restarting(),
        Kernel::RunningKernel(kernel) => match &kernel.execution_state() {
            ExecutionState::Idle => ReplMenuState {
                tooltip: format!("Run code on {} ({})", kernel_name, kernel_language).into(),
                indicator: Some(Indicator::dot().color(Color::Success)),
                status: session.kernel.status(),
                ..fill_fields()
            },
            ExecutionState::Busy => ReplMenuState {
                tooltip: format!("Interrupt {} ({})", kernel_name, kernel_language).into(),
                icon_is_animating: true,
                popover_disabled: false,
                indicator: None,
                status: session.kernel.status(),
                ..fill_fields()
            },
            ExecutionState::Unknown => unknown(),
            ExecutionState::Starting => starting(),
            ExecutionState::Restarting => restarting(),
            ExecutionState::Terminating => shutting_down(),
            ExecutionState::AutoRestarting => auto_restarting(),
            ExecutionState::Dead => shutdown(),
            ExecutionState::Other(state) => other(state),
        },
        Kernel::StartingKernel(_) => starting(),
        Kernel::ErroredLaunch(e) => ReplMenuState {
            tooltip: format!("Error with kernel {}: {}", kernel_name, e).into(),
            popover_disabled: false,
            indicator: Some(Indicator::dot().color(Color::Error)),
            status: session.kernel.status(),
            ..fill_fields()
        },
        Kernel::ShuttingDown => shutting_down(),
        Kernel::Shutdown => shutdown(),
    }
}
