use action_log::DiffStats;
use agent_ui::{TerminalId, thread_metadata_store::ThreadMetadata};
use gpui::{
    Action as _, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Modifiers,
    ModifiersChangedEvent, Render, ScrollHandle, SharedString, prelude::*,
};
use ui::{AgentThreadStatus, ThreadItem, ThreadItemWorktreeInfo, WithScrollbar, prelude::*};
use workspace::{ModalView, Workspace};
use zed_actions::agents_sidebar::ToggleThreadSwitcher;

#[derive(Clone)]
pub(crate) struct ThreadSwitcherThreadEntry {
    pub title: SharedString,
    pub icon: IconName,
    pub icon_from_external_svg: Option<SharedString>,
    pub status: AgentThreadStatus,
    pub metadata: ThreadMetadata,
    pub workspace: Entity<Workspace>,
    pub project_name: Option<SharedString>,
    pub worktrees: Vec<ThreadItemWorktreeInfo>,
    pub diff_stats: DiffStats,
    pub is_draft: bool,
    pub is_title_generating: bool,
    pub notified: bool,
    pub timestamp: SharedString,
}

#[derive(Clone)]
pub(crate) struct ThreadSwitcherTerminalEntry {
    pub terminal_id: TerminalId,
    pub title: SharedString,
    pub workspace: Entity<Workspace>,
    pub project_name: Option<SharedString>,
    pub worktrees: Vec<ThreadItemWorktreeInfo>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub notified: bool,
    pub timestamp: SharedString,
}

#[derive(Clone)]
pub(crate) enum ThreadSwitcherEntry {
    Thread(ThreadSwitcherThreadEntry),
    Terminal(ThreadSwitcherTerminalEntry),
}

#[derive(Clone)]
pub(crate) enum ThreadSwitcherSelection {
    Thread {
        metadata: ThreadMetadata,
        workspace: Entity<Workspace>,
    },
    Terminal {
        terminal_id: TerminalId,
        workspace: Entity<Workspace>,
    },
}

impl ThreadSwitcherEntry {
    pub(crate) fn selection(&self) -> ThreadSwitcherSelection {
        match self {
            Self::Thread(entry) => ThreadSwitcherSelection::Thread {
                metadata: entry.metadata.clone(),
                workspace: entry.workspace.clone(),
            },
            Self::Terminal(entry) => ThreadSwitcherSelection::Terminal {
                terminal_id: entry.terminal_id,
                workspace: entry.workspace.clone(),
            },
        }
    }

    fn element_id(&self) -> SharedString {
        match self {
            Self::Thread(entry) => SharedString::from(format!(
                "thread-switcher-thread-{:?}",
                entry.metadata.thread_id
            )),
            Self::Terminal(entry) => {
                SharedString::from(format!("thread-switcher-terminal-{}", entry.terminal_id))
            }
        }
    }

    fn title(&self) -> SharedString {
        match self {
            Self::Thread(entry) => entry.title.clone(),
            Self::Terminal(entry) => entry.title.clone(),
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Thread(entry) if entry.is_draft => IconName::Circle,
            Self::Thread(entry) => entry.icon,
            Self::Terminal(_) => IconName::Terminal,
        }
    }

    fn icon_from_external_svg(&self) -> Option<SharedString> {
        match self {
            Self::Thread(entry) if entry.is_draft => None,
            Self::Thread(entry) => entry.icon_from_external_svg.clone(),
            Self::Terminal(_) => None,
        }
    }

    fn status(&self) -> AgentThreadStatus {
        match self {
            Self::Thread(entry) => entry.status,
            Self::Terminal(_) => AgentThreadStatus::default(),
        }
    }

    fn project_name(&self) -> Option<SharedString> {
        match self {
            Self::Thread(entry) => entry.project_name.clone(),
            Self::Terminal(entry) => entry.project_name.clone(),
        }
    }

    fn worktrees(&self) -> Vec<ThreadItemWorktreeInfo> {
        match self {
            Self::Thread(entry) => entry.worktrees.clone(),
            Self::Terminal(entry) => entry.worktrees.clone(),
        }
    }

    fn timestamp(&self) -> SharedString {
        match self {
            Self::Thread(entry) => entry.timestamp.clone(),
            Self::Terminal(entry) => entry.timestamp.clone(),
        }
    }

    fn is_draft(&self) -> bool {
        match self {
            Self::Thread(entry) => entry.is_draft,
            Self::Terminal(_) => false,
        }
    }

    fn is_title_generating(&self) -> bool {
        match self {
            Self::Thread(entry) => entry.is_title_generating,
            Self::Terminal(_) => false,
        }
    }

    fn notified(&self) -> bool {
        match self {
            Self::Thread(entry) => entry.notified,
            Self::Terminal(entry) => entry.notified,
        }
    }

    fn diff_stats(&self) -> DiffStats {
        match self {
            Self::Thread(entry) => entry.diff_stats,
            Self::Terminal(_) => DiffStats::default(),
        }
    }

    #[cfg(test)]
    pub fn thread_id(&self) -> Option<agent_ui::ThreadId> {
        match self {
            Self::Thread(entry) => Some(entry.metadata.thread_id),
            Self::Terminal(_) => None,
        }
    }

    #[cfg(test)]
    pub fn terminal_id(&self) -> Option<TerminalId> {
        match self {
            Self::Thread(_) => None,
            Self::Terminal(entry) => Some(entry.terminal_id),
        }
    }
}

pub(crate) enum ThreadSwitcherEvent {
    Preview(ThreadSwitcherSelection),
    Confirmed(ThreadSwitcherSelection),
    Dismissed,
}

pub(crate) struct ThreadSwitcher {
    focus_handle: FocusHandle,
    entries: Vec<ThreadSwitcherEntry>,
    selected_index: usize,
    init_modifiers: Option<Modifiers>,
    scroll_handle: ScrollHandle,
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
            cx.emit(ThreadSwitcherEvent::Preview(entry.selection()));
        }

        let focus_handle = cx.focus_handle();
        cx.on_focus_out(&focus_handle, window, |_this, _event, _window, cx| {
            cx.emit(ThreadSwitcherEvent::Dismissed);
            cx.emit(DismissEvent);
        })
        .detach();

        let scroll_handle = ScrollHandle::new();
        scroll_handle.scroll_to_item(selected_index);

        Self {
            focus_handle,
            entries,
            selected_index,
            init_modifiers,
            scroll_handle,
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
        self.scroll_handle.scroll_to_item(self.selected_index);
        if let Some(entry) = self.entries.get(self.selected_index) {
            cx.emit(ThreadSwitcherEvent::Preview(entry.selection()));
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.confirm_selected(cx);
    }

    fn confirm_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            cx.emit(ThreadSwitcherEvent::Confirmed(entry.selection()));
        }
        cx.emit(DismissEvent);
    }

    fn select_and_confirm(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.entries.len() {
            self.selected_index = index;
            self.confirm_selected(cx);
        }
    }

    fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.entries.len() || index == self.selected_index {
            return;
        }
        self.selected_index = index;
        self.scroll_handle.scroll_to_item(index);
        self.emit_preview(cx);
        cx.notify();
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
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_index = self.selected_index;

        v_flex()
            .key_context("ThreadSwitcher")
            .track_focus(&self.focus_handle)
            .p_1p5()
            .w(rems_from_px(440.))
            .elevation_3(cx)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::toggle))
            .child(
                v_flex()
                    .id("thread-switcher-list")
                    .gap_0p5()
                    .max_h_128()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(self.entries.iter().enumerate().map(|(ix, entry)| {
                        let diff_stats = entry.diff_stats();

                        ThreadItem::new(entry.element_id(), entry.title())
                            .rounded(true)
                            .icon(entry.icon())
                            .when(entry.is_draft(), |this| {
                                this.icon_color(Color::Custom(
                                    cx.theme().colors().icon_muted.opacity(0.2),
                                ))
                            })
                            .status(entry.status())
                            .when_some(entry.icon_from_external_svg(), |this, svg| {
                                this.custom_icon_from_external_svg(svg)
                            })
                            .when_some(entry.project_name(), |this, name| this.project_name(name))
                            .worktrees(entry.worktrees())
                            .timestamp(entry.timestamp())
                            .title_generating(entry.is_title_generating())
                            .notified(entry.notified())
                            .when(diff_stats.lines_added > 0, |this| {
                                this.added(diff_stats.lines_added as usize)
                            })
                            .when(diff_stats.lines_removed > 0, |this| {
                                this.removed(diff_stats.lines_removed as usize)
                            })
                            .selected(ix == selected_index)
                            .base_bg(cx.theme().colors().elevated_surface_background)
                            .on_hover(cx.listener(move |this, hovered: &bool, _window, cx| {
                                if *hovered {
                                    this.select_index(ix, cx);
                                }
                            }))
                            // TODO: This is not properly propagating to the tread item.
                            .on_click(cx.listener(
                                move |this, _event: &gpui::ClickEvent, _window, cx| {
                                    this.select_and_confirm(ix, cx);
                                },
                            ))
                            .into_any_element()
                    })),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
    }
}
