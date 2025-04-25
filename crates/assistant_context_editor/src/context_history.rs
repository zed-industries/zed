use std::sync::Arc;

use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use project::Project;
use ui::utils::{DateTimeType, format_distance_from_now};
use ui::{Avatar, ListItem, ListItemSpacing, prelude::*};
use workspace::{Item, Workspace};

use crate::{
    AssistantPanelDelegate, ContextStore, DEFAULT_TAB_TITLE, RemoteContextMetadata,
    SavedContextMetadata,
};

#[derive(Clone)]
pub enum ContextMetadata {
    Remote(RemoteContextMetadata),
    Saved(SavedContextMetadata),
}

enum SavedContextPickerEvent {
    Confirmed(ContextMetadata),
}

pub struct ContextHistory {
    picker: Entity<Picker<SavedContextPickerDelegate>>,
    _subscriptions: Vec<Subscription>,
    workspace: WeakEntity<Workspace>,
}

impl ContextHistory {
    pub fn new(
        project: Entity<Project>,
        context_store: Entity<ContextStore>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                SavedContextPickerDelegate::new(project, context_store.clone()),
                window,
                cx,
            )
            .modal(false)
            .max_height(None)
        });

        let subscriptions = vec![
            cx.observe_in(&context_store, window, |this, _, window, cx| {
                this.picker
                    .update(cx, |picker, cx| picker.refresh(window, cx));
            }),
            cx.subscribe_in(&picker, window, Self::handle_picker_event),
        ];

        Self {
            picker,
            _subscriptions: subscriptions,
            workspace,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: &Entity<Picker<SavedContextPickerDelegate>>,
        event: &SavedContextPickerEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let SavedContextPickerEvent::Confirmed(context) = event;

        let Some(assistant_panel_delegate) = <dyn AssistantPanelDelegate>::try_global(cx) else {
            return;
        };

        self.workspace
            .update(cx, |workspace, cx| match context {
                ContextMetadata::Remote(metadata) => {
                    assistant_panel_delegate
                        .open_remote_context(workspace, metadata.id.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
                ContextMetadata::Saved(metadata) => {
                    assistant_panel_delegate
                        .open_saved_context(workspace, metadata.path.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
            })
            .ok();
    }
}

impl Render for ContextHistory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.picker.clone())
    }
}

impl Focusable for ContextHistory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<()> for ContextHistory {}

impl Item for ContextHistory {
    type Event = ();

    fn tab_content_text(&self, _detail: usize,  _window: &Window, _cx: &App) -> SharedString {
        "History".into()
    }
}

struct SavedContextPickerDelegate {
    store: Entity<ContextStore>,
    project: Entity<Project>,
    matches: Vec<ContextMetadata>,
    selected_index: usize,
}

impl EventEmitter<SavedContextPickerEvent> for Picker<SavedContextPickerDelegate> {}

impl SavedContextPickerDelegate {
    fn new(project: Entity<Project>, store: Entity<ContextStore>) -> Self {
        Self {
            project,
            store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SavedContextPickerDelegate {
    type ListItem = ListItem;

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

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let search = self.store.read(cx).search(query, cx);
        cx.spawn(async move |this, cx| {
            let matches = search.await;
            this.update(cx, |this, cx| {
                let host_contexts = this.delegate.store.read(cx).host_contexts();
                this.delegate.matches = host_contexts
                    .iter()
                    .cloned()
                    .map(ContextMetadata::Remote)
                    .chain(matches.into_iter().map(ContextMetadata::Saved))
                    .collect();
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(metadata) = self.matches.get(self.selected_index) {
            cx.emit(SavedContextPickerEvent::Confirmed(metadata.clone()));
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let context = self.matches.get(ix)?;
        let item = match context {
            ContextMetadata::Remote(context) => {
                let host_user = self.project.read(cx).host().and_then(|collaborator| {
                    self.project
                        .read(cx)
                        .user_store()
                        .read(cx)
                        .get_cached_user(collaborator.user_id)
                });
                div()
                    .flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().flex_1().overflow_x_hidden().child(
                            Label::new(context.summary.clone().unwrap_or(DEFAULT_TAB_TITLE.into()))
                                .size(LabelSize::Small),
                        ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .children(if let Some(host_user) = host_user {
                                vec![
                                    Avatar::new(host_user.avatar_uri.clone()).into_any_element(),
                                    Label::new(format!("Shared by @{}", host_user.github_login))
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .into_any_element(),
                                ]
                            } else {
                                vec![
                                    Label::new("Shared by host")
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .into_any_element(),
                                ]
                            }),
                    )
            }
            ContextMetadata::Saved(context) => div()
                .flex()
                .w_full()
                .justify_between()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .child(Label::new(context.title.clone()).size(LabelSize::Small))
                        .overflow_x_hidden(),
                )
                .child(
                    Label::new(format_distance_from_now(
                        DateTimeType::Local(context.mtime),
                        false,
                        true,
                        true,
                    ))
                    .color(Color::Muted)
                    .size(LabelSize::Small),
                ),
        };
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(item),
        )
    }
}
