use gpui::AnyElement;
use repl::{ExecutionState, Kernel, KernelSpecification, RuntimePanel, Session, SessionSupport};
use ui::{prelude::*, Tooltip};

use crate::QuickActionBar;

const ZED_REPL_DOCUMENTATION: &str = "https://zed.dev/docs/repl";

impl QuickActionBar {
    pub fn render_repl_menu(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
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

        let icon = |kernel: &Kernel| match kernel {
            Kernel::RunningKernel(_) => IconName::ReplPlay,
            Kernel::StartingKernel(_) => IconName::ReplNeutral,
            Kernel::ErroredLaunch(_) => IconName::ReplNeutral,
            Kernel::ShuttingDown => IconName::ReplNeutral,
            Kernel::Shutdown => IconName::ReplPause,
        };

        let kernel_name: SharedString = session.kernel_specification.name.clone().into();

        let tooltip = |session: &Session| {
            match &session.kernel {
                Kernel::RunningKernel(kernel) => {
                    match &kernel.execution_state {
                        ExecutionState::Idle => format!("{} is ready", kernel_name), // Play
                        ExecutionState::Busy => format!("{} is executing", kernel_name), // Interrupt button
                    }
                }
                Kernel::StartingKernel(_) => format!("{} is starting", kernel_name),
                Kernel::ErroredLaunch(e) => format!("Error: {}", e),
                Kernel::ShuttingDown => format!("{} is shutting down", kernel_name),
                Kernel::Shutdown => "Nothing running".to_string(),
            }
        };

        let tooltip_text: SharedString = SharedString::from(tooltip(&session).clone());

        let button = IconButton::new("toggle_repl_icon", icon(&session.kernel))
            .size(ButtonSize::Compact)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |cx| Tooltip::text(tooltip_text.clone(), cx))
            .on_click(|_, cx| cx.dispatch_action(Box::new(repl::Run {})))
            .into_any_element();

        Some(button)
    }

    pub fn render_repl_launch_menu(
        &self,
        kernel_specification: KernelSpecification,
        _cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let tooltip: SharedString =
            SharedString::from(format!("Start REPL for {}", kernel_specification.name));

        Some(
            IconButton::new("toggle_repl_icon", IconName::ReplOff)
                .size(ButtonSize::Compact)
                .icon_size(IconSize::Small)
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
            IconButton::new("toggle_repl_icon", IconName::ReplOff)
                .size(ButtonSize::Compact)
                .icon_size(IconSize::Small)
                .style(ButtonStyle::Subtle)
                .tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
                .on_click(|_, cx| cx.open_url(ZED_REPL_DOCUMENTATION))
                .into_any_element(),
        )
    }
}
