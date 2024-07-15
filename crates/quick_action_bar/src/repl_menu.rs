use std::time::Duration;

use gpui::{percentage, Animation, AnimationExt, AnyElement, Transformation};
use repl::{
    ExecutionState, JupyterSettings, Kernel, KernelSpecification, RuntimePanel, SessionSupport,
};
use ui::{
    prelude::*, ButtonLike, ContextMenu, IconWithIndicator, Indicator, IntoElement, PopoverMenu,
    Tooltip,
};

use gpui::ElementId;
use util::ResultExt;

use crate::QuickActionBar;

const ZED_REPL_DOCUMENTATION: &str = "https://zed.dev/docs/repl";

impl QuickActionBar {
    pub fn render_repl_menu(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        if !JupyterSettings::enabled(cx) {
            return None;
        }

        let workspace = self.workspace.upgrade()?.read(cx);

        let (editor, repl_panel) = if let (Some(editor), Some(repl_panel)) =
            (self.active_editor(), workspace.panel::<RuntimePanel>(cx))
        {
            (editor, repl_panel)
        } else {
            return None;
        };

        let has_nonempty_selection = {
            editor.update(cx, |this, cx| {
                this.selections
                    .count()
                    .ne(&0)
                    .then(|| {
                        let latest = this.selections.newest_display(cx);
                        !latest.is_empty()
                    })
                    .unwrap_or_default()
            })
        };

        let session = repl_panel.update(cx, |repl_panel, cx| {
            repl_panel.session(editor.downgrade(), cx)
        });

        let session = match session {
            SessionSupport::ActiveSession(session) => session.read(cx),
            SessionSupport::Inactive(spec) => {
                let spec = *spec;
                return self.render_repl_launch_menu(spec, cx);
            }
            SessionSupport::RequiresSetup(language) => {
                return self.render_repl_setup(&language, cx);
            }
            SessionSupport::Unsupported => return None,
        };

        let kernel_name: SharedString = session.kernel_specification.name.clone().into();
        let kernel_language: SharedString = session
            .kernel_specification
            .kernelspec
            .language
            .clone()
            .into();

        struct ReplMenuState {
            tooltip: SharedString,
            icon: IconName,
            icon_color: Color,
            icon_is_animating: bool,
            popover_disabled: bool,
            indicator: Option<Indicator>,
            // TODO: Persist rotation state so the
            // icon doesn't reset on every state change
            // current_delta: Duration,
        }

        impl Default for ReplMenuState {
            fn default() -> Self {
                Self {
                    tooltip: "Nothing running".into(),
                    icon: IconName::ReplNeutral,
                    icon_color: Color::Default,
                    icon_is_animating: false,
                    popover_disabled: false,
                    indicator: None,
                    // current_delta: Duration::default(),
                }
            }
        }

        let menu_state = match &session.kernel {
            Kernel::RunningKernel(kernel) => match &kernel.execution_state {
                ExecutionState::Idle => ReplMenuState {
                    tooltip: format!("Run code on {} ({})", kernel_name, kernel_language).into(),
                    indicator: Some(Indicator::dot().color(Color::Success)),
                    ..Default::default()
                },
                ExecutionState::Busy => ReplMenuState {
                    tooltip: format!("Interrupt {} ({})", kernel_name, kernel_language).into(),
                    icon_is_animating: true,
                    popover_disabled: false,
                    indicator: None,
                    ..Default::default()
                },
            },
            Kernel::StartingKernel(_) => ReplMenuState {
                tooltip: format!("{} is starting", kernel_name).into(),
                icon_is_animating: true,
                popover_disabled: true,
                icon_color: Color::Muted,
                indicator: Some(Indicator::dot().color(Color::Muted)),
                ..Default::default()
            },
            Kernel::ErroredLaunch(e) => ReplMenuState {
                tooltip: format!("Error with kernel {}: {}", kernel_name, e).into(),
                popover_disabled: false,
                indicator: Some(Indicator::dot().color(Color::Error)),
                ..Default::default()
            },
            Kernel::ShuttingDown => ReplMenuState {
                tooltip: format!("{} is shutting down", kernel_name).into(),
                popover_disabled: true,
                icon_color: Color::Muted,
                indicator: Some(Indicator::dot().color(Color::Muted)),
                ..Default::default()
            },
            Kernel::Shutdown => ReplMenuState::default(),
        };

        let id = "repl-menu".to_string();

        let element_id = |suffix| ElementId::Name(format!("{}-{}", id, suffix).into());

        let kernel = &session.kernel;
        let status_borrow = &kernel.status();
        let status = status_borrow.clone();
        let panel_clone = repl_panel.clone();
        let editor_clone = editor.downgrade();
        let dropdown_menu = PopoverMenu::new(element_id("menu"))
            .menu(move |cx| {
                let kernel_name = kernel_name.clone();
                let kernel_language = kernel_language.clone();
                let status = status.clone();
                let panel_clone = panel_clone.clone();
                let editor_clone = editor_clone.clone();
                ContextMenu::build(cx, move |menu, _cx| {
                    let editor_clone = editor_clone.clone();
                    let panel_clone = panel_clone.clone();
                    let kernel_name = kernel_name.clone();
                    let kernel_language = kernel_language.clone();
                    let status = status.clone();
                    menu.when_else(
                        status.is_connected(),
                        |running| {
                            let status = status.clone();
                            running
                                .custom_row(move |_cx| {
                                    h_flex()
                                        .child(
                                            Label::new(format!(
                                                "kernel: {} ({})",
                                                kernel_name.clone(),
                                                kernel_language.clone()
                                            ))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                        )
                                        .into_any_element()
                                })
                                .custom_row(move |_cx| {
                                    h_flex()
                                        .child(
                                            Label::new(status.clone().to_string())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element()
                                })
                        },
                        |not_running| {
                            let status = status.clone();
                            not_running.custom_row(move |_cx| {
                                h_flex()
                                    .child(
                                        Label::new(format!("{}...", status.clone().to_string()))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .into_any_element()
                            })
                        },
                    )
                    .separator()
                    // Run
                    .custom_entry(
                        move |_cx| {
                            Label::new(if has_nonempty_selection {
                                "Run Selection"
                            } else {
                                "Run Line"
                            })
                            .into_any_element()
                        },
                        {
                            let panel_clone = panel_clone.clone();
                            let editor_clone = editor_clone.clone();
                            move |cx| {
                                let editor_clone = editor_clone.clone();
                                panel_clone.update(cx, |this, cx| {
                                    this.run(editor_clone.clone(), cx).log_err();
                                });
                            }
                        },
                    )
                    // Interrupt
                    .custom_entry(
                        move |_cx| {
                            Label::new("Interrupt")
                                .size(LabelSize::Small)
                                .color(Color::Error)
                                .into_any_element()
                        },
                        {
                            let panel_clone = panel_clone.clone();
                            let editor_clone = editor_clone.clone();
                            move |cx| {
                                let editor_clone = editor_clone.clone();
                                panel_clone.update(cx, |this, cx| {
                                    this.interrupt(editor_clone, cx);
                                });
                            }
                        },
                    )
                    // Clear Outputs
                    .custom_entry(
                        move |_cx| {
                            Label::new("Clear Outputs")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .into_any_element()
                        },
                        {
                            let panel_clone = panel_clone.clone();
                            let editor_clone = editor_clone.clone();
                            move |cx| {
                                let editor_clone = editor_clone.clone();
                                panel_clone.update(cx, |this, cx| {
                                    this.clear_outputs(editor_clone, cx);
                                });
                            }
                        },
                    )
                    .separator()
                    .link(
                        "Change Kernel",
                        Box::new(zed_actions::OpenBrowser {
                            url: format!("{}#change-kernel", ZED_REPL_DOCUMENTATION),
                        }),
                    )
                    // TODO: Add Restart action
                    // .action("Restart", Box::new(gpui::NoAction))
                    // Shut down kernel
                    .custom_entry(
                        move |_cx| {
                            Label::new("Shut Down Kernel")
                                .size(LabelSize::Small)
                                .color(Color::Error)
                                .into_any_element()
                        },
                        {
                            let panel_clone = panel_clone.clone();
                            let editor_clone = editor_clone.clone();
                            move |cx| {
                                let editor_clone = editor_clone.clone();
                                panel_clone.update(cx, |this, cx| {
                                    this.shutdown(editor_clone, cx);
                                });
                            }
                        },
                    )
                    // .separator()
                    // TODO: Add shut down all kernels action
                    // .action("Shut Down all Kernels", Box::new(gpui::NoAction))
                })
                .into()
            })
            .trigger(
                ButtonLike::new_rounded_right(element_id("dropdown"))
                    .child(
                        Icon::new(IconName::ChevronDownSmall)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .tooltip(move |cx| Tooltip::text("REPL Menu", cx))
                    .width(rems(1.).into())
                    .disabled(menu_state.popover_disabled),
            );

        let button = ButtonLike::new_rounded_left("toggle_repl_icon")
            .child(if menu_state.icon_is_animating {
                Icon::new(menu_state.icon)
                    .color(menu_state.icon_color)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(5)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    )
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
            .tooltip(move |cx| Tooltip::text(menu_state.tooltip.clone(), cx))
            .on_click(|_, cx| cx.dispatch_action(Box::new(repl::Run {})))
            .into_any_element();

        Some(
            h_flex()
                .child(button)
                .child(dropdown_menu)
                .into_any_element(),
        )
    }

    pub fn render_repl_launch_menu(
        &self,
        kernel_specification: KernelSpecification,
        _cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let tooltip: SharedString =
            SharedString::from(format!("Start REPL for {}", kernel_specification.name));

        Some(
            IconButton::new("toggle_repl_icon", IconName::ReplNeutral)
                .size(ButtonSize::Compact)
                .icon_color(Color::Muted)
                .style(ButtonStyle::Subtle)
                .tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                .on_click(|_, cx| cx.dispatch_action(Box::new(repl::Run {})))
                .into_any_element(),
        )
    }

    pub fn render_repl_setup(
        &self,
        language: &str,
        _cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let tooltip: SharedString = SharedString::from(format!("Setup Zed REPL for {}", language));
        Some(
            IconButton::new("toggle_repl_icon", IconName::ReplNeutral)
                .size(ButtonSize::Compact)
                .icon_color(Color::Muted)
                .style(ButtonStyle::Subtle)
                .tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                .on_click(|_, cx| cx.open_url(&format!("{}#installation", ZED_REPL_DOCUMENTATION)))
                .into_any_element(),
        )
    }
}
