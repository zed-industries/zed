use gpui::ElementId;
use gpui::{AnyElement, Entity};
use picker::Picker;
use repl::{
    ExecutionState, JupyterSettings, Kernel, KernelSpecification, KernelStatus, Session,
    SessionSupport,
    components::{KernelPickerDelegate, KernelSelector},
    worktree_id_for_editor,
};
use ui::{
    ButtonLike, CommonAnimationExt, ContextMenu, IconWithIndicator, Indicator, IntoElement,
    PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use util::ResultExt;

use super::QuickActionBar;

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

impl QuickActionBar {
    pub fn render_repl_menu(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !JupyterSettings::enabled(cx) {
            return None;
        }

        let editor = self.active_editor()?;

        let is_local_project = editor
            .read(cx)
            .workspace()
            .map(|workspace| workspace.read(cx).project().read(cx).is_local())
            .unwrap_or(false);

        if !is_local_project {
            return None;
        }

        let has_nonempty_selection = {
            editor.update(cx, |this, cx| {
                this.selections
                    .count()
                    .ne(&0)
                    .then(|| {
                        let snapshot = this.display_snapshot(cx);
                        let latest = this.selections.newest_display(&snapshot);
                        !latest.is_empty()
                    })
                    .unwrap_or_default()
            })
        };

        let session = repl::session(editor.downgrade(), cx);
        let session = match session {
            SessionSupport::ActiveSession(session) => session,
            SessionSupport::Inactive(spec) => {
                return self.render_repl_launch_menu(spec, cx);
            }
            SessionSupport::RequiresSetup(language) => {
                return self.render_repl_setup(language.as_ref(), cx);
            }
            SessionSupport::Unsupported => return None,
        };

        let menu_state = session_state(session.clone(), cx);

        let id = "repl-menu";

        let element_id = |suffix| ElementId::Name(format!("{}-{}", id, suffix).into());

        let editor = editor.downgrade();
        let dropdown_menu = PopoverMenu::new(element_id("menu"))
            .menu(move |window, cx| {
                let editor = editor.clone();
                let session = session.clone();
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
                .into()
            })
            .trigger_with_tooltip(
                ButtonLike::new_rounded_right(element_id("dropdown"))
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .width(rems(1.))
                    .disabled(menu_state.popover_disabled),
                Tooltip::text("REPL Menu"),
            );

        let button = ButtonLike::new_rounded_left("toggle_repl_icon")
            .child(if menu_state.icon_is_animating {
                Icon::new(menu_state.icon)
                    .color(menu_state.icon_color)
                    .with_rotate_animation(5)
                    .into_any_element()
            } else {
                IconWithIndicator::new(
                    Icon::new(IconName::ReplNeutral).color(menu_state.icon_color),
                    menu_state.indicator,
                )
                .indicator_border_color(Some(cx.theme().colors().toolbar_background))
                .into_any_element()
            })
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Subtle)
            .tooltip(Tooltip::text(menu_state.tooltip))
            .on_click(|_, window, cx| window.dispatch_action(Box::new(repl::Run {}), cx))
            .into_any_element();

        Some(
            h_flex()
                .child(self.render_kernel_selector(cx))
                .child(button)
                .child(dropdown_menu)
                .into_any_element(),
        )
    }
    pub fn render_repl_launch_menu(
        &self,
        kernel_specification: KernelSpecification,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let tooltip: SharedString =
            SharedString::from(format!("Start REPL for {}", kernel_specification.name()));

        Some(
            h_flex()
                .child(self.render_kernel_selector(cx))
                .child(
                    IconButton::new("toggle_repl_icon", IconName::ReplNeutral)
                        .size(ButtonSize::Compact)
                        .icon_color(Color::Muted)
                        .style(ButtonStyle::Subtle)
                        .tooltip(Tooltip::text(tooltip))
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(repl::Run {}), cx)
                        }),
                )
                .into_any_element(),
        )
    }

    pub fn render_kernel_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let editor = if let Some(editor) = self.active_editor() {
            editor
        } else {
            return div().into_any_element();
        };

        let Some(worktree_id) = worktree_id_for_editor(editor.downgrade(), cx) else {
            return div().into_any_element();
        };

        let session = repl::session(editor.downgrade(), cx);

        let current_kernelspec = match session {
            SessionSupport::ActiveSession(session) => {
                Some(session.read(cx).kernel_specification.clone())
            }
            SessionSupport::Inactive(kernel_specification) => Some(kernel_specification),
            SessionSupport::RequiresSetup(_language_name) => None,
            SessionSupport::Unsupported => None,
        };

        let current_kernel_name = current_kernelspec.as_ref().map(|spec| spec.name());

        let menu_handle: PopoverMenuHandle<Picker<KernelPickerDelegate>> =
            PopoverMenuHandle::default();
        KernelSelector::new(
            {
                Box::new(move |kernelspec, window, cx| {
                    repl::assign_kernelspec(kernelspec, editor.downgrade(), window, cx).ok();
                })
            },
            worktree_id,
            ButtonLike::new("kernel-selector")
                .style(ButtonStyle::Subtle)
                .size(ButtonSize::Compact)
                .child(
                    h_flex()
                        .w_full()
                        .gap_0p5()
                        .child(
                            div()
                                .overflow_x_hidden()
                                .flex_grow()
                                .whitespace_nowrap()
                                .child(
                                    Label::new(if let Some(name) = current_kernel_name {
                                        name
                                    } else {
                                        SharedString::from("Select Kernel")
                                    })
                                    .size(LabelSize::Small)
                                    .color(if current_kernelspec.is_some() {
                                        Color::Default
                                    } else {
                                        Color::Placeholder
                                    })
                                    .into_any_element(),
                                ),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                ),
            Tooltip::text("Select Kernel"),
        )
        .with_handle(menu_handle)
        .into_any_element()
    }

    pub fn render_repl_setup(&self, language: &str, cx: &mut Context<Self>) -> Option<AnyElement> {
        let tooltip: SharedString = SharedString::from(format!("Setup Zed REPL for {}", language));
        Some(
            h_flex()
                .gap(DynamicSpacing::Base06.rems(cx))
                .child(self.render_kernel_selector(cx))
                .child(
                    IconButton::new("toggle_repl_icon", IconName::ReplNeutral)
                        .style(ButtonStyle::Subtle)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(ui::IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text(tooltip))
                        .on_click(|_, _window, cx| {
                            cx.open_url(&format!("{}#installation", ZED_REPL_DOCUMENTATION))
                        }),
                )
                .into_any_element(),
        )
    }
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

    match &session.kernel {
        Kernel::Restarting => ReplMenuState {
            tooltip: format!("Restarting {}", kernel_name).into(),
            icon_is_animating: true,
            popover_disabled: true,
            icon_color: Color::Muted,
            indicator: Some(Indicator::dot().color(Color::Muted)),
            status: session.kernel.status(),
            ..fill_fields()
        },
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
        },
        Kernel::StartingKernel(_) => ReplMenuState {
            tooltip: format!("{} is starting", kernel_name).into(),
            icon_is_animating: true,
            popover_disabled: true,
            icon_color: Color::Muted,
            indicator: Some(Indicator::dot().color(Color::Muted)),
            status: session.kernel.status(),
            ..fill_fields()
        },
        Kernel::ErroredLaunch(e) => ReplMenuState {
            tooltip: format!("Error with kernel {}: {}", kernel_name, e).into(),
            popover_disabled: false,
            indicator: Some(Indicator::dot().color(Color::Error)),
            status: session.kernel.status(),
            ..fill_fields()
        },
        Kernel::ShuttingDown => ReplMenuState {
            tooltip: format!("{} is shutting down", kernel_name).into(),
            popover_disabled: true,
            icon_color: Color::Muted,
            indicator: Some(Indicator::dot().color(Color::Muted)),
            status: session.kernel.status(),
            ..fill_fields()
        },
        Kernel::Shutdown => ReplMenuState {
            tooltip: "Nothing running".into(),
            icon: IconName::ReplNeutral,
            icon_color: Color::Default,
            icon_is_animating: false,
            popover_disabled: false,
            indicator: None,
            status: KernelStatus::Shutdown,
            ..fill_fields()
        },
    }
}
