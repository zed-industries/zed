use crate::render_summary;
use gpui::{
    elements::*, platform::CursorStyle, Entity, ModelHandle, RenderContext, View, ViewContext,
};
use project::Project;
use workspace::{Settings, StatusItemView};

pub struct DiagnosticSummary {
    summary: project::DiagnosticSummary,
    in_progress: bool,
}

impl DiagnosticSummary {
    pub fn new(project: &ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsUpdated => {
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsStarted => {
                this.in_progress = true;
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsFinished => {
                this.summary = project.read(cx).diagnostic_summary(cx);
                this.in_progress = false;
                cx.notify();
            }
            _ => {}
        })
        .detach();
        Self {
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

        let in_progress = self.in_progress;
        MouseEventHandler::new::<Tag, _, _>(0, cx, |_, cx| {
            let theme = &cx.app_state::<Settings>().theme.project_diagnostics;
            if in_progress {
                Label::new(
                    "Checking... ".to_string(),
                    theme.status_bar_item.text.clone(),
                )
                .contained()
                .with_style(theme.status_bar_item.container)
                .boxed()
            } else {
                render_summary(&self.summary, &theme.status_bar_item.text, &theme)
            }
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
