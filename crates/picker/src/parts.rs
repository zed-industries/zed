//! Components used in multiple pickers

use gpui::Entity;
use project::Project;
use ui::{CommonAnimationExt, Tooltip, prelude::*};

pub fn project_scan_indicator(
    has_query: bool,
    project: &Entity<Project>,
    cx: &App,
) -> Option<impl IntoElement> {
    let is_project_scan_running = {
        let worktree_store = project.read(cx).worktree_store();
        !worktree_store.read(cx).initial_scan_completed()
    };
    (has_query && is_project_scan_running).then(|| {
        h_flex()
            .id("project-scan-indicator")
            .tooltip(Tooltip::text("Project Scan in Progress…"))
            .child(
                Icon::new(IconName::LoadCircle)
                    .color(Color::Accent)
                    .size(IconSize::Small)
                    .with_rotate_animation(2),
            )
    })
}
