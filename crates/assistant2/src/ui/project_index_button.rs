use assistant_tooling::ToolRegistry;
use gpui::{percentage, prelude::*, Animation, AnimationExt, Model, Transformation};
use semantic_index::{ProjectIndex, Status};
use std::{sync::Arc, time::Duration};
use ui::{prelude::*, ButtonLike, Color, Icon, IconName, Indicator, Tooltip};

use crate::tools::ProjectIndexTool;

pub struct ProjectIndexButton {
    project_index: Model<ProjectIndex>,
    tool_registry: Arc<ToolRegistry>,
}

impl ProjectIndexButton {
    pub fn new(
        project_index: Model<ProjectIndex>,
        tool_registry: Arc<ToolRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&project_index, |_this, _, _status: &Status, cx| {
            cx.notify();
        })
        .detach();
        Self {
            project_index,
            tool_registry,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.tool_registry
            .set_tool_enabled::<ProjectIndexTool>(enabled);
    }
}

impl Render for ProjectIndexButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let status = self.project_index.read(cx).status();
        let is_enabled = self.tool_registry.is_tool_enabled::<ProjectIndexTool>();

        let icon = if is_enabled {
            match status {
                Status::Idle => Icon::new(IconName::Code)
                    .size(IconSize::XSmall)
                    .color(Color::Default),
                Status::Loading => Icon::new(IconName::Code)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
                Status::Scanning { .. } => Icon::new(IconName::Code)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            }
        } else {
            Icon::new(IconName::Code)
                .size(IconSize::XSmall)
                .color(Color::Disabled)
        };

        let indicator = if is_enabled {
            match status {
                Status::Idle => Some(Indicator::dot().color(Color::Success)),
                Status::Scanning { .. } => Some(Indicator::dot().color(Color::Warning)),
                Status::Loading => Some(Indicator::icon(
                    Icon::new(IconName::Spinner)
                        .color(Color::Accent)
                        .with_animation(
                            "arrow-circle",
                            Animation::new(Duration::from_secs(2)).repeat(),
                            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                        ),
                )),
            }
        } else {
            None
        };

        ButtonLike::new("project-index")
            .child(
                ui::IconWithIndicator::new(icon, indicator)
                    .indicator_border_color(Some(gpui::transparent_black())),
            )
            .tooltip({
                move |cx| {
                    let (tooltip, meta) = match (is_enabled, status) {
                        (false, _) => (
                            "Project index disabled".to_string(),
                            Some("Click to enable".to_string()),
                        ),
                        (_, Status::Idle) => (
                            "Project index ready".to_string(),
                            Some("Click to disable".to_string()),
                        ),
                        (_, Status::Loading) => ("Project index loading...".to_string(), None),
                        (_, Status::Scanning { remaining_count }) => (
                            "Project index scanning...".to_string(),
                            Some(format!("{} remaining...", remaining_count)),
                        ),
                    };

                    if let Some(meta) = meta {
                        Tooltip::with_meta(tooltip, None, meta, cx)
                    } else {
                        Tooltip::text(tooltip, cx)
                    }
                }
            })
            .on_click(cx.listener(move |this, _, cx| {
                this.set_enabled(!is_enabled);
                cx.notify();
            }))
    }
}
