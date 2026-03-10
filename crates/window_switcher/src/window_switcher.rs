use std::sync::Arc;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, AnyWindowHandle, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, ParentElement, Render, Styled, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, HighlightedLabel, Indicator, ListItem, ListItemSpacing, SpinnerLabel};
use workspace::{AgentActivityStatus, ModalView, Workspace};

actions!(window_switcher, [Toggle]);

pub fn init(cx: &mut App) {
    cx.observe_new(WindowSwitcher::register).detach();
}

impl ModalView for WindowSwitcher {}

pub struct WindowSwitcher {
    picker: Entity<Picker<WindowSwitcherDelegate>>,
}

fn workspace_title(workspace: &Workspace, cx: &App) -> String {
    if let Some(custom_name) = workspace.custom_name() {
        return custom_name.to_string();
    }
    let project = workspace.project().read(cx);
    let mut title = String::new();
    for (i, worktree) in project.visible_worktrees(cx).enumerate() {
        if i > 0 {
            title.push_str(", ");
        }
        title.push_str(worktree.read(cx).root_name_str());
    }
    if title.is_empty() {
        title = "empty project".to_string();
    }
    title
}

impl WindowSwitcher {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let current_window_handle = window.window_handle();
            let current_window_id = current_window_handle.window_id();

            let current_title = workspace_title(workspace, cx);
            let current_agent_status = workspace.agent_activity_status(cx);

            let workspace_store = workspace.app_state().workspace_store.read(cx);

            let mut entries = vec![WindowEntry {
                title: current_title,
                window_handle: current_window_handle,
                is_current: true,
                agent_status: current_agent_status,
            }];

            for (window_handle, weak_workspace) in workspace_store.workspaces_with_windows() {
                if window_handle.window_id() == current_window_id {
                    continue;
                }

                let Some(workspace_entity) = weak_workspace.upgrade() else {
                    continue;
                };

                let workspace_ref = workspace_entity.read(cx);
                let title = workspace_title(workspace_ref, cx);
                let agent_status = workspace_ref.agent_activity_status(cx);

                entries.push(WindowEntry {
                    title,
                    window_handle,
                    is_current: false,
                    agent_status,
                });
            }

            workspace.toggle_modal(window, cx, |window, cx| {
                WindowSwitcher::new(entries, window, cx)
            });
        });
    }

    fn new(windows: Vec<WindowEntry>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = WindowSwitcherDelegate {
            window_switcher: cx.entity().downgrade(),
            windows,
            matches: Vec::new(),
            selected_index: 0,
        };
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl EventEmitter<DismissEvent> for WindowSwitcher {}

impl Focusable for WindowSwitcher {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for WindowSwitcher {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("WindowSwitcher")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

struct WindowEntry {
    title: String,
    window_handle: AnyWindowHandle,
    is_current: bool,
    agent_status: AgentActivityStatus,
}

pub struct WindowSwitcherDelegate {
    window_switcher: WeakEntity<WindowSwitcher>,
    windows: Vec<WindowEntry>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl PickerDelegate for WindowSwitcherDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Switch to window…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No windows".into())
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = self
                .windows
                .iter()
                .enumerate()
                .map(|(ix, entry)| StringMatch {
                    candidate_id: ix,
                    score: 0.0,
                    positions: Vec::new(),
                    string: entry.title.clone(),
                })
                .collect();
            self.selected_index = self.selected_index.min(self.matches.len().saturating_sub(1));
            return Task::ready(());
        }

        let candidates: Vec<StringMatchCandidate> = self
            .windows
            .iter()
            .enumerate()
            .map(|(ix, entry)| StringMatchCandidate::new(ix, &entry.title))
            .collect();

        let executor = cx.background_executor().clone();
        cx.spawn(async move |this, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                false,
                100,
                &Default::default(),
                executor,
            )
            .await;

            this.update(cx, |picker, _cx| {
                let delegate = &mut picker.delegate;
                delegate.matches = matches;
                if delegate.matches.is_empty() {
                    delegate.selected_index = 0;
                } else {
                    delegate.selected_index =
                        delegate.selected_index.min(delegate.matches.len() - 1);
                }
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(selected_match) = self.matches.get(self.selected_index) {
            let window_entry = &self.windows[selected_match.candidate_id];
            if !window_entry.is_current {
                let handle = window_entry.window_handle;
                handle
                    .update(cx, |_, window, _| window.activate_window())
                    .ok();
            }
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.window_switcher
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matching = self.matches.get(ix)?;
        let entry = &self.windows[matching.candidate_id];

        let agent_status_element: Option<AnyElement> = match entry.agent_status {
            AgentActivityStatus::Working => Some(
                h_flex()
                    .gap_1p5()
                    .child(SpinnerLabel::new().size(LabelSize::Small).color(Color::Accent))
                    .child(
                        Label::new("Working")
                            .size(LabelSize::Small)
                            .color(Color::Accent),
                    )
                    .into_any_element(),
            ),
            AgentActivityStatus::WaitingForConfirmation => Some(
                h_flex()
                    .gap_1p5()
                    .child(Indicator::dot().color(Color::Warning))
                    .child(
                        Label::new("Needs input")
                            .size(LabelSize::Small)
                            .color(Color::Warning),
                    )
                    .into_any_element(),
            ),
            AgentActivityStatus::Idle => Some(
                h_flex()
                    .gap_1p5()
                    .child(Indicator::dot().color(Color::Success))
                    .child(
                        Label::new("Done")
                            .size(LabelSize::Small)
                            .color(Color::Success),
                    )
                    .into_any_element(),
            ),
            AgentActivityStatus::Inactive => None,
        };

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .py_px()
                        .gap_2()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(HighlightedLabel::new(
                                    entry.title.clone(),
                                    matching.positions.clone(),
                                ))
                                .when(entry.is_current, |this| {
                                    this.child(
                                        Label::new("(current)")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                        .children(agent_status_element),
                ),
        )
    }
}
