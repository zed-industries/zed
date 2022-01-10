use gpui::{
    elements::*, platform::CursorStyle, Entity, ModelHandle, RenderContext, View, ViewContext,
};
use postage::watch;
use project::Project;
use std::fmt::Write;
use workspace::{Settings, StatusItemView};

pub struct DiagnosticSummary {
    settings: watch::Receiver<Settings>,
    summary: project::DiagnosticSummary,
    in_progress: bool,
}

impl DiagnosticSummary {
    pub fn new(
        project: &ModelHandle<Project>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsUpdated { .. } => {
                this.summary = project.read(cx).diagnostic_summary(cx);
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsStarted => {
                this.in_progress = true;
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsFinished => {
                this.in_progress = false;
                cx.notify();
            }
            _ => {}
        })
        .detach();
        Self {
            settings,
            summary: project.read(cx).diagnostic_summary(cx),
            in_progress: project.read(cx).is_running_disk_based_diagnostics(),
        }
    }
}

impl Entity for DiagnosticSummary {
    type Event = ();
}

impl View for DiagnosticSummary {
    fn ui_name() -> &'static str {
        "DiagnosticSummary"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Tag {}

        let theme = &self.settings.borrow().theme.project_diagnostics;
        let mut message = String::new();
        if self.in_progress {
            message.push_str("Checking... ");
        }
        write!(
            message,
            "Errors: {}, Warnings: {}",
            self.summary.error_count, self.summary.warning_count
        )
        .unwrap();
        MouseEventHandler::new::<Tag, _, _, _>(0, cx, |_, _| {
            Label::new(message, theme.status_bar_item.text.clone())
                .contained()
                .with_style(theme.status_bar_item.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(|cx| cx.dispatch_action(crate::Deploy))
        .boxed()
    }
}

impl StatusItemView for DiagnosticSummary {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn workspace::ItemViewHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}
