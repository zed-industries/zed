use gpui::{
    elements::*, platform::CursorStyle, serde_json, Entity, ModelHandle, RenderContext, View,
    ViewContext,
};
use project::Project;
use settings::Settings;
use workspace::StatusItemView;

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
        MouseEventHandler::new::<Tag, _, _>(0, cx, |state, cx| {
            let style = &cx
                .global::<Settings>()
                .theme
                .workspace
                .status_bar
                .diagnostics;
            let summary_style = if self.summary.error_count > 0 {
                if state.hovered {
                    &style.summary_error_hover
                } else {
                    &style.summary_error
                }
            } else if self.summary.warning_count > 0 {
                if state.hovered {
                    &style.summary_warning_hover
                } else {
                    &style.summary_warning
                }
            } else if state.hovered {
                &style.summary_ok_hover
            } else {
                &style.summary_ok
            };

            let mut row = Flex::row();
            if self.summary.error_count > 0 {
                row.add_children([
                    Svg::new("icons/error-solid-14.svg")
                        .with_color(style.icon_color_error)
                        .constrained()
                        .with_width(style.icon_width)
                        .aligned()
                        .contained()
                        .with_margin_right(style.icon_spacing)
                        .named("error-icon"),
                    Label::new(
                        self.summary.error_count.to_string(),
                        summary_style.text.clone(),
                    )
                    .aligned()
                    .boxed(),
                ]);
            }

            if self.summary.warning_count > 0 {
                row.add_children([
                    Svg::new("icons/warning-solid-14.svg")
                        .with_color(style.icon_color_warning)
                        .constrained()
                        .with_width(style.icon_width)
                        .aligned()
                        .contained()
                        .with_margin_right(style.icon_spacing)
                        .with_margin_left(if self.summary.error_count > 0 {
                            style.summary_spacing
                        } else {
                            0.
                        })
                        .named("warning-icon"),
                    Label::new(
                        self.summary.warning_count.to_string(),
                        summary_style.text.clone(),
                    )
                    .aligned()
                    .boxed(),
                ]);
            }

            if self.summary.error_count == 0 && self.summary.warning_count == 0 {
                row.add_child(
                    Svg::new("icons/no-error-solid-14.svg")
                        .with_color(style.icon_color_ok)
                        .constrained()
                        .with_width(style.icon_width)
                        .aligned()
                        .named("ok-icon"),
                );
            }

            row.constrained()
                .with_height(style.height)
                .contained()
                .with_style(summary_style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(|cx| cx.dispatch_action(crate::Deploy))
        .boxed()
    }

    fn debug_json(&self, _: &gpui::AppContext) -> serde_json::Value {
        serde_json::json!({ "summary": self.summary })
    }
}

impl StatusItemView for DiagnosticSummary {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn workspace::ItemHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}
