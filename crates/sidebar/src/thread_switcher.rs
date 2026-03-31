use action_log::DiffStats;
use agent_client_protocol as acp;
use agent_ui::thread_metadata_store::ThreadMetadata;
use gpui::{
    Action as _, Animation, AnimationExt, AnyElement, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, Hsla, Modifiers, ModifiersChangedEvent, Render, SharedString,
    prelude::*, pulsating_between,
};
use std::time::Duration;
use ui::{
    AgentThreadStatus, Color, CommonAnimationExt, DecoratedIcon, DiffStat, Icon, IconDecoration,
    IconDecorationKind, IconName, IconSize, Label, LabelSize, prelude::*,
};
use workspace::{ModalView, Workspace};
use zed_actions::agents_sidebar::ToggleThreadSwitcher;

const PANEL_WIDTH_REMS: f32 = 28.;

pub(crate) struct ThreadSwitcherEntry {
    pub session_id: acp::SessionId,
    pub title: SharedString,
    pub icon: IconName,
    pub icon_from_external_svg: Option<SharedString>,
    pub status: AgentThreadStatus,
    pub metadata: ThreadMetadata,
    pub workspace: Entity<Workspace>,
    pub worktree_name: Option<SharedString>,
    pub diff_stats: DiffStats,
    pub is_title_generating: bool,
    pub notified: bool,
    pub timestamp: SharedString,
}

pub(crate) enum ThreadSwitcherEvent {
    Preview {
        metadata: ThreadMetadata,
        workspace: Entity<Workspace>,
    },
    Confirmed {
        metadata: ThreadMetadata,
        workspace: Entity<Workspace>,
    },
    Dismissed,
}

pub(crate) struct ThreadSwitcher {
    focus_handle: FocusHandle,
    entries: Vec<ThreadSwitcherEntry>,
    selected_index: usize,
    init_modifiers: Option<Modifiers>,
}

impl ThreadSwitcher {
    pub fn new(
        entries: Vec<ThreadSwitcherEntry>,
        select_last: bool,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let init_modifiers = window.modifiers().modified().then_some(window.modifiers());
        let selected_index = if entries.is_empty() {
            0
        } else if select_last {
            entries.len() - 1
        } else {
            1.min(entries.len().saturating_sub(1))
        };

        if let Some(entry) = entries.get(selected_index) {
            cx.emit(ThreadSwitcherEvent::Preview {
                metadata: entry.metadata.clone(),
                workspace: entry.workspace.clone(),
            });
        }

        let focus_handle = cx.focus_handle();
        cx.on_focus_out(&focus_handle, window, |_this, _event, _window, cx| {
            cx.emit(ThreadSwitcherEvent::Dismissed);
            cx.emit(DismissEvent);
        })
        .detach();

        Self {
            focus_handle,
            entries,
            selected_index,
            init_modifiers,
        }
    }

    pub fn selected_entry(&self) -> Option<&ThreadSwitcherEntry> {
        self.entries.get(self.selected_index)
    }

    #[cfg(test)]
    pub fn entries(&self) -> &[ThreadSwitcherEntry] {
        &self.entries
    }

    #[cfg(test)]
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn cycle_selection(&mut self, cx: &mut Context<Self>) {
        if self.entries.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.entries.len();
        self.emit_preview(cx);
    }

    pub fn select_last(&mut self, cx: &mut Context<Self>) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.entries.len() - 1;
        } else {
            self.selected_index -= 1;
        }
        self.emit_preview(cx);
    }

    fn emit_preview(&mut self, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            cx.emit(ThreadSwitcherEvent::Preview {
                metadata: entry.metadata.clone(),
                workspace: entry.workspace.clone(),
            });
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            cx.emit(ThreadSwitcherEvent::Confirmed {
                metadata: entry.metadata.clone(),
                workspace: entry.workspace.clone(),
            });
        }
        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        cx.emit(ThreadSwitcherEvent::Dismissed);
        cx.emit(DismissEvent);
    }

    fn toggle(
        &mut self,
        action: &ToggleThreadSwitcher,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        if action.select_last {
            self.select_last(cx);
        } else {
            self.cycle_selection(cx);
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers else {
            return;
        };
        if !event.modified() || !init_modifiers.is_subset_of(event) {
            self.init_modifiers = None;
            if self.entries.is_empty() {
                cx.emit(DismissEvent);
            } else {
                window.dispatch_action(menu::Confirm.boxed_clone(), cx);
            }
        }
    }
}

impl ModalView for ThreadSwitcher {}

impl EventEmitter<DismissEvent> for ThreadSwitcher {}
impl EventEmitter<ThreadSwitcherEvent> for ThreadSwitcher {}

impl Focusable for ThreadSwitcher {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadSwitcher {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_index = self.selected_index;
        let color = cx.theme().colors();
        let panel_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.2));

        v_flex()
            .key_context("ThreadSwitcher")
            .track_focus(&self.focus_handle)
            .w(gpui::rems(PANEL_WIDTH_REMS))
            .elevation_3(cx)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::toggle))
            .children(self.entries.iter().enumerate().map(|(ix, entry)| {
                let is_first = ix == 0;
                let is_last = ix == self.entries.len() - 1;
                let selected = ix == selected_index;
                let base_bg = if selected {
                    color.element_active
                } else {
                    panel_bg
                };

                let dot_separator = || {
                    Label::new("\u{2022}")
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .alpha(0.5)
                };

                let icon_container = || h_flex().size_4().flex_none().justify_center();

                let agent_icon = || {
                    if let Some(ref svg) = entry.icon_from_external_svg {
                        Icon::from_external_svg(svg.clone())
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    } else {
                        Icon::new(entry.icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    }
                };

                let decoration = |kind: IconDecorationKind, deco_color: Hsla| {
                    IconDecoration::new(kind, base_bg, cx)
                        .color(deco_color)
                        .position(gpui::Point {
                            x: px(-2.),
                            y: px(-2.),
                        })
                };

                let icon_element: AnyElement = if entry.status == AgentThreadStatus::Running {
                    icon_container()
                        .child(
                            Icon::new(IconName::LoadCircle)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                                .with_rotate_animation(2),
                        )
                        .into_any_element()
                } else if entry.status == AgentThreadStatus::Error {
                    icon_container()
                        .child(DecoratedIcon::new(
                            agent_icon(),
                            Some(decoration(IconDecorationKind::X, cx.theme().status().error)),
                        ))
                        .into_any_element()
                } else if entry.status == AgentThreadStatus::WaitingForConfirmation {
                    icon_container()
                        .child(DecoratedIcon::new(
                            agent_icon(),
                            Some(decoration(
                                IconDecorationKind::Triangle,
                                cx.theme().status().warning,
                            )),
                        ))
                        .into_any_element()
                } else if entry.notified {
                    icon_container()
                        .child(DecoratedIcon::new(
                            agent_icon(),
                            Some(decoration(IconDecorationKind::Dot, color.text_accent)),
                        ))
                        .into_any_element()
                } else {
                    icon_container().child(agent_icon()).into_any_element()
                };

                let title_label: AnyElement = if entry.is_title_generating {
                    Label::new(entry.title.clone())
                        .color(Color::Muted)
                        .with_animation(
                            "generating-title",
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.4, 0.8)),
                            |label, delta| label.alpha(delta),
                        )
                        .into_any_element()
                } else {
                    Label::new(entry.title.clone()).into_any_element()
                };

                let has_diff_stats =
                    entry.diff_stats.lines_added > 0 || entry.diff_stats.lines_removed > 0;
                let has_worktree = entry.worktree_name.is_some();
                let has_timestamp = !entry.timestamp.is_empty();

                v_flex()
                    .id(ix)
                    .w_full()
                    .py_1()
                    .px_1p5()
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .when(selected, |s| s.bg(color.element_active))
                    .when(is_first, |s| s.rounded_t_lg())
                    .when(is_last, |s| s.rounded_b_lg())
                    .child(
                        h_flex()
                            .min_w_0()
                            .w_full()
                            .gap_1p5()
                            .child(icon_element)
                            .child(title_label),
                    )
                    .when(has_worktree || has_diff_stats || has_timestamp, |this| {
                        this.child(
                            h_flex()
                                .min_w_0()
                                .gap_1p5()
                                .child(icon_container())
                                .when_some(entry.worktree_name.clone(), |this, worktree| {
                                    this.child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                Icon::new(IconName::GitWorktree)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Muted),
                                            )
                                            .child(
                                                Label::new(worktree)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            ),
                                    )
                                })
                                .when(has_worktree && (has_diff_stats || has_timestamp), |this| {
                                    this.child(dot_separator())
                                })
                                .when(has_diff_stats, |this| {
                                    this.child(DiffStat::new(
                                        ix,
                                        entry.diff_stats.lines_added as usize,
                                        entry.diff_stats.lines_removed as usize,
                                    ))
                                })
                                .when(has_diff_stats && has_timestamp, |this| {
                                    this.child(dot_separator())
                                })
                                .when(has_timestamp, |this| {
                                    this.child(
                                        Label::new(entry.timestamp.clone())
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                    })
            }))
    }
}
