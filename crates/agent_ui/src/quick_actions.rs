use agent_settings::{AgentQuickAction, AgentSettings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Task, WeakEntity, Window, prelude::*,
};
use log::warn;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{Label, LabelSize, ListItem, ListItemSpacing, prelude::*};
use settings::Settings;
use workspace::{ModalView, Workspace};

use crate::agent_panel::AgentPanel;

pub struct AgentQuickActionsModal {
    picker: Entity<Picker<QuickActionsDelegate>>,
}

impl ModalView for AgentQuickActionsModal {}

impl AgentQuickActionsModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let Some(previous_focus_handle) = window.focused(cx) else {
            return;
        };

        let workspace_handle = workspace.weak_handle();
        workspace.toggle_modal(window, cx, move |window, cx| {
            AgentQuickActionsModal::new(previous_focus_handle, workspace_handle.clone(), window, cx)
        });
    }

    fn new(
        previous_focus_handle: FocusHandle,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let quick_actions = AgentSettings::get_global(cx).quick_actions.clone();
        let delegate = QuickActionsDelegate::new(quick_actions, previous_focus_handle, workspace);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .width(rems(34.))
                .modal(true)
        });
        Self { picker }
    }
}

impl EventEmitter<DismissEvent> for AgentQuickActionsModal {}

impl Focusable for AgentQuickActionsModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for AgentQuickActionsModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AgentQuickActions")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

#[derive(Clone)]
struct QuickActionMatch {
    candidate_id: usize,
}

struct QuickActionsDelegate {
    workspace: WeakEntity<Workspace>,
    previous_focus_handle: FocusHandle,
    quick_actions: Vec<AgentQuickAction>,
    matches: Vec<QuickActionMatch>,
    selected_ix: usize,
    confirmed: bool,
}

impl QuickActionsDelegate {
    fn new(
        quick_actions: Vec<AgentQuickAction>,
        previous_focus_handle: FocusHandle,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        let matches = quick_actions
            .iter()
            .enumerate()
            .map(|(candidate_id, _)| QuickActionMatch { candidate_id })
            .collect();

        Self {
            workspace,
            previous_focus_handle,
            quick_actions,
            matches,
            selected_ix: 0,
            confirmed: false,
        }
    }

    fn selected_action(&self) -> Option<&AgentQuickAction> {
        self.matches
            .get(self.selected_ix)
            .and_then(|m| self.quick_actions.get(m.candidate_id))
    }

    fn filter_matches(&mut self, query: &str) {
        let normalized = query.trim().to_lowercase();

        if normalized.is_empty() {
            self.matches = self
                .quick_actions
                .iter()
                .enumerate()
                .map(|(candidate_id, _)| QuickActionMatch { candidate_id })
                .collect();
        } else {
            self.matches = self
                .quick_actions
                .iter()
                .enumerate()
                .filter(|(_, action)| {
                    let title = action.title.as_ref().to_ascii_lowercase();
                    let description = action
                        .description
                        .as_ref()
                        .map(|value| value.as_ref().to_ascii_lowercase());

                    title.contains(&normalized)
                        || description
                            .as_ref()
                            .map(|desc| desc.contains(&normalized))
                            .unwrap_or(false)
                })
                .map(|(candidate_id, _)| QuickActionMatch { candidate_id })
                .collect();
        }

        if self.matches.is_empty() {
            self.selected_ix = 0;
        } else if self.selected_ix >= self.matches.len() {
            self.selected_ix = self.matches.len() - 1;
        }
    }
}

impl PickerDelegate for QuickActionsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Choose an AI quick action…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        if self.match_count() == 0 {
            self.selected_ix = 0;
        } else {
            self.selected_ix = ix.min(self.matches.len().saturating_sub(1));
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.filter_matches(&query);
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(action) = self.selected_action().cloned() else {
            window.focus(&self.previous_focus_handle);
            return;
        };

        self.confirmed = true;
        let workspace = self.workspace.clone();
        let previous_focus = self.previous_focus_handle.clone();

        window.defer(cx, move |window, cx| {
            let Some(workspace) = workspace.upgrade() else {
                window.focus(&previous_focus);
                return;
            };

            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.focus_panel::<AgentPanel>(window, cx) {
                    if let Err(error) =
                        panel.update(cx, |panel, cx| panel.run_quick_action(&action, window, cx))
                    {
                        warn!("Failed to run quick action: {error:?}");
                        window.focus(&previous_focus);
                    }
                } else {
                    window.focus(&previous_focus);
                }
            });
        });
    }

    fn dismissed(&mut self, window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        if !self.confirmed {
            window.focus(&self.previous_focus_handle);
        }
        self.confirmed = false;
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No quick actions match your search".into())
    }

    fn confirm_input(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        // No custom input handling.
    }

    fn confirm_completion(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }

    fn confirm_update_query(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let quick_action = self
            .matches
            .get(ix)
            .and_then(|m| self.quick_actions.get(m.candidate_id))
            .cloned()?;

        let title = quick_action.title.clone();
        let description = quick_action.description.clone();

        let mut content = v_flex()
            .gap(rems(0.25))
            .child(Label::new(title));

        if let Some(description) = description {
            content = content.child(
                Label::new(description)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        }

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(content),
        )
    }
}
