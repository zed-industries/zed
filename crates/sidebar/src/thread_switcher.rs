use action_log::DiffStats;
use agent_client_protocol as acp;
use agent_ui::thread_metadata_store::ThreadMetadata;
use gpui::{
    Action as _, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Modifiers,
    ModifiersChangedEvent, Render, SharedString, prelude::*,
};
use ui::{AgentThreadStatus, ThreadItem, ThreadItemWorktreeInfo, prelude::*};
use workspace::{ModalView, Workspace};
use zed_actions::agents_sidebar::ToggleThreadSwitcher;

pub(crate) struct ThreadSwitcherEntry {
    pub session_id: acp::SessionId,
    pub title: SharedString,
    pub icon: IconName,
    pub icon_from_external_svg: Option<SharedString>,
    pub status: AgentThreadStatus,
    pub metadata: ThreadMetadata,
    pub workspace: Entity<Workspace>,
    pub project_name: Option<SharedString>,
    pub worktrees: Vec<ThreadItemWorktreeInfo>,
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
        self.confirm_selected(cx);
    }

    fn confirm_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            cx.emit(ThreadSwitcherEvent::Confirmed {
                metadata: entry.metadata.clone(),
                workspace: entry.workspace.clone(),
            });
        }
        cx.emit(DismissEvent);
    }

    fn select_and_confirm(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.entries.len() {
            self.selected_index = index;
            self.confirm_selected(cx);
        }
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

        v_flex()
            .key_context("ThreadSwitcher")
            .track_focus(&self.focus_handle)
            .w(rems_from_px(440.))
            .p_1p5()
            .gap_0p5()
            .elevation_3(cx)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::toggle))
            .children(self.entries.iter().enumerate().map(|(ix, entry)| {
                let id = SharedString::from(format!("thread-switcher-{}", entry.session_id));

                div()
                    .id(id.clone())
                    .on_click(
                        cx.listener(move |this, _event: &gpui::ClickEvent, _window, cx| {
                            this.select_and_confirm(ix, cx);
                        }),
                    )
                    .child(
                        ThreadItem::new(id, entry.title.clone())
                            .rounded(true)
                            .icon(entry.icon)
                            .status(entry.status)
                            .when_some(entry.icon_from_external_svg.clone(), |this, svg| {
                                this.custom_icon_from_external_svg(svg)
                            })
                            .when_some(entry.project_name.clone(), |this, name| {
                                this.project_name(name)
                            })
                            .worktrees(entry.worktrees.clone())
                            .timestamp(entry.timestamp.clone())
                            .title_generating(entry.is_title_generating)
                            .notified(entry.notified)
                            .when(entry.diff_stats.lines_added > 0, |this| {
                                this.added(entry.diff_stats.lines_added as usize)
                            })
                            .when(entry.diff_stats.lines_removed > 0, |this| {
                                this.removed(entry.diff_stats.lines_removed as usize)
                            })
                            .selected(ix == selected_index)
                            .base_bg(cx.theme().colors().elevated_surface_background),
                    )
                    .into_any_element()
            }))
    }
}
