use gpui::AnyElement;
use repl::{
    ExecutionState, JupyterSettings, Kernel, KernelSpecification, RuntimePanel, Session,
    SessionSupport,
};
use ui::{
    prelude::*, ButtonLike, ContextMenu, IconWithIndicator, IntoElement, PopoverMenu, Tooltip,
};

use gpui::ElementId;

// No session && no support known

// No session && no kernel installed for languages of known support
// - Intro to REPL
// - Link to docs

// No session but can start one
// - Start REPL
// - More info -> Docs?

// Yes Session
// - [Default kernel changed - restart (this kernel) to apply] // todo!(kyle): need some kind of state thing that says if this has happened
// - Info: Kernel name, language
//   example: chatlab-3.7-adsf87fsa (Python)
//   example: condapy-3.7 (Python)
// - Change Kernel -> https://zed.dev/docs/repl#change-kernel
// - ---
// - Run
// - Interrupt
// - Clear Outputs
// - ---
// - Restart
// - Shutdown
// - ---
// - Shutdown all kernels

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

        let session = repl_panel.update(cx, |repl_panel, cx| {
            repl_panel.session(editor.downgrade(), cx)
        });

        let session = match session {
            SessionSupport::ActiveSession(session) => session.read(cx),
            SessionSupport::Inactive(spec) => {
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

        let tooltip = |session: &Session| match &session.kernel {
            Kernel::RunningKernel(kernel) => match &kernel.execution_state {
                ExecutionState::Idle => {
                    format!("Run code on {} ({})", kernel_name, kernel_language)
                }
                ExecutionState::Busy => format!("Interrupt {} ({})", kernel_name, kernel_language),
            },
            Kernel::StartingKernel(_) => format!("{} is starting", kernel_name),
            Kernel::ErroredLaunch(e) => format!("Error with kernel {}: {}", kernel_name, e),
            Kernel::ShuttingDown => format!("{} is shutting down", kernel_name),
            Kernel::Shutdown => "Nothing running".to_string(),
        };

        let tooltip_text: SharedString = SharedString::from(tooltip(&session).clone());

        let id = "repl-menu".to_string();

        let element_id = |suffix| ElementId::Name(format!("{}-{}", id, suffix).into());

        let kernel = &session.kernel;
        let status_borrow = &kernel.status();
        let status = status_borrow.clone();

        let dropdown_menu = PopoverMenu::new(element_id("menu"))
            .menu(move |cx| {
                let kernel_name = kernel_name.clone();
                let kernel_language = kernel_language.clone();
                let status = status.clone();
                ContextMenu::build(cx, move |menu, _cx| {
                    let kernel_name = kernel_name.clone();
                    let kernel_language = kernel_language.clone();
                    let status = status.clone();
                    menu.when_else(
                        status.is_running(),
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
                    .separator()
                    // TODO:
                    // Check if there is a selection in the editor
                    // If there is, label is "Run Selection"
                    // otherwise, label is "Run Line" (Cell?)
                    .action("Run", Box::new(repl::Run))
                    // TODO: Add action
                    .action("Interrupt", Box::new(gpui::NoAction))
                    // TODO: Add action
                    .action("Clear Outputs", Box::new(repl::ClearOutputs))
                    .separator()
                    .link(
                        "Change Kernel",
                        Box::new(zed_actions::OpenBrowser {
                            url: ZED_REPL_DOCUMENTATION.into(),
                        }),
                    )
                    // TODO: Add action
                    .action("Restart", Box::new(gpui::NoAction))
                    // TODO: Add action
                    .action("Shut Down", Box::new(gpui::NoAction))
                    .separator()
                    // TODO: Add action
                    .action("Shut Down all Kernels", Box::new(gpui::NoAction))
                })
                .into()
            })
            .trigger(
                ButtonLike::new_rounded_right(element_id("dropdown"))
                    .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall))
                    .tooltip(move |cx| Tooltip::text("REPL Menu", cx))
                    .width(rems(1.).into()),
            );

        let button = ButtonLike::new_rounded_left("toggle_repl_icon")
            .child(
                IconWithIndicator::new(Icon::new(IconName::Play), Some(session.kernel.dot()))
                    .indicator_border_color(Some(cx.theme().colors().border)),
            )
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Subtle)
            .tooltip(move |cx| Tooltip::text(tooltip_text.clone(), cx))
            .on_click(|_, cx| cx.dispatch_action(Box::new(repl::Run {})))
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
            IconButton::new("toggle_repl_icon", IconName::Play)
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
            IconButton::new("toggle_repl_icon", IconName::Play)
                .size(ButtonSize::Compact)
                .icon_color(Color::Muted)
                .style(ButtonStyle::Subtle)
                .tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                .on_click(|_, cx| cx.open_url(ZED_REPL_DOCUMENTATION))
                .into_any_element(),
        )
    }
}
