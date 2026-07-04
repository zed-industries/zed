use gpui::{App, ClickEvent, Context, Empty, Entity, ParentElement as _, Subscription, Window};
use project::{
    Project,
    git_store::{GitStore, GitStoreEvent},
};
use ui::{ButtonLike, Tooltip, prelude::*};
use workspace::{HideStatusItem, StatusItemView, Workspace, item::ItemHandle};

use crate::project_diff::ToggleBranchDiffIndicators;

/// Status bar item shown only while branch-diff-indicators mode is active;
/// clicking it turns the mode back off.
pub struct BranchDiffIndicator {
    project: Entity<Project>,
    _subscription: Subscription,
}

impl BranchDiffIndicator {
    /// Builds the indicator and subscribes to the project's git store.
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let subscription = cx.subscribe(&git_store, Self::on_git_store_event);
        Self {
            project,
            _subscription: subscription,
        }
    }

    /// Re-renders when the mode is toggled so the item appears or disappears.
    fn on_git_store_event(
        &mut self,
        _git_store: Entity<GitStore>,
        event: &GitStoreEvent,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, GitStoreEvent::BranchDiffIndicatorsChanged) {
            cx.notify();
        }
    }

    /// Turns branch-diff-indicators mode off (the item is only shown when on).
    fn toggle_off(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let git_store = self.project.read(cx).git_store().clone();
        git_store.update(cx, |git_store, cx| {
            git_store.set_branch_diff_indicators_enabled(false, cx);
        });
    }
}

impl Render for BranchDiffIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enabled = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .branch_diff_indicators_enabled();
        if !enabled {
            return Empty.into_any_element();
        }

        let border_color = cx.theme().colors().text_accent.opacity(0.2);

        h_flex()
            .h(rems_from_px(22.))
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new("branch-diff-indicator")
                    .child(
                        h_flex()
                            .h_full()
                            .gap_1()
                            .child(
                                Icon::new(IconName::GitBranch)
                                    .size(IconSize::Small)
                                    .color(Color::Accent),
                            )
                            .child(Label::new("Branch Diff").size(LabelSize::Small)),
                    )
                    .tooltip(|_, cx| {
                        Tooltip::with_meta(
                            "Showing Branch Diff",
                            Some(&ToggleBranchDiffIndicators),
                            "Gutter and project panel show changes vs. the default branch. Click to turn off.",
                            cx,
                        )
                    })
                    .on_click(cx.listener(Self::toggle_off)),
            )
            .into_any_element()
    }
}

impl StatusItemView for BranchDiffIndicator {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}
