use gpui::{
    elements::*, platform::CursorStyle, Entity, ModelHandle, RenderContext, View, ViewContext,
};
use postage::watch;
use project::Project;
use workspace::{Settings, StatusItemView};

pub struct DiagnosticSummary {
    settings: watch::Receiver<Settings>,
    summary: project::DiagnosticSummary,
}

impl DiagnosticSummary {
    pub fn new(
        project: &ModelHandle<Project>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(project, |this, project, event, cx| {
            if let project::Event::DiskBasedDiagnosticsUpdated { .. } = event {
                this.summary = project.read(cx).diagnostic_summary(cx);
                cx.notify();
            }
        })
        .detach();
        Self {
            settings,
            summary: project.read(cx).diagnostic_summary(cx),
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
        MouseEventHandler::new::<Tag, _, _, _>(0, cx, |_, _| {
            Label::new(
                format!(
                    "Errors: {}, Warnings: {}",
                    self.summary.error_count, self.summary.warning_count
                ),
                theme.status_bar_item.text.clone(),
            )
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
