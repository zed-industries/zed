use gpui::AnyElement;
use repl::{ExecutionState, Kernel, RuntimePanel, Session};
use ui::{prelude::*, Tooltip};

use crate::QuickActionBar;

impl QuickActionBar {
    pub fn render_repl_menu(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let repl_supported_langs = ["python", "javascript", "typescript", "julia", "r"];

        let docs_link = "https://google.com";
        let active_session: Option<Session> = None;
        let supports_repl = false;

        let workspace = self.workspace.upgrade()?.read(cx);

        let (editor, repl_panel) = if let (Some(editor), Some(repl_panel)) =
            (self.active_editor(), workspace.panel::<RuntimePanel>(cx))
        {
            (editor, repl_panel)
        } else {
            return None;
        };

        let (session, supports_language) = repl_panel.update(cx, |repl_panel, _cx| {
            (repl_panel.session(editor.downgrade()), true)
        });

        let session = if let Some(session) = session {
            session.read(cx)
        } else if supports_language {
            // todo: If there is not a session and it supports the language, the user
            // should be able to start a session
            return self.render_repl_launch_menu(cx);
        } else {
            return None;
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
            .on_click(|_, cx| {
                cx.dispatch_action(
                    Box::new(repl::Run {})
                )
                })
            .into_any_element();

        Some(button)
    }

    pub fn render_repl_launch_menu(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        Some(IconButton::new("toggle_repl_icon", IconName::ReplOff)
            .size(ButtonSize::Compact)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |cx| Tooltip::text("Get started with Zed REPL", cx))
            .on_click(|_, cx| {
                cx.dispatch_action(
                    Box::new(repl::Run {})
                )
                })
            .into_any_element())
    }
}
