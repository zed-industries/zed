use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::anyhow;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use prompt_store::{PromptId, UserPromptId};
use ui::{ListItem, prelude::*};

use crate::context::RULES_ICON;
use crate::context_picker::ContextPicker;
use crate::context_store::{self, ContextStore};
use crate::thread_store::ThreadStore;

pub struct RulesContextPicker {
    picker: Entity<Picker<RulesContextPickerDelegate>>,
}

impl RulesContextPicker {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = RulesContextPickerDelegate::new(thread_store, context_picker, context_store);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        RulesContextPicker { picker }
    }
}

impl Focusable for RulesContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RulesContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RulesContextEntry {
    pub prompt_id: UserPromptId,
    pub title: SharedString,
}

pub struct RulesContextPickerDelegate {
    thread_store: WeakEntity<ThreadStore>,
    context_picker: WeakEntity<ContextPicker>,
    context_store: WeakEntity<context_store::ContextStore>,
    matches: Vec<RulesContextEntry>,
    selected_index: usize,
}

impl RulesContextPickerDelegate {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
    ) -> Self {
        RulesContextPickerDelegate {
            thread_store,
            context_picker,
            context_store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for RulesContextPickerDelegate {
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
        "Search available rules…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(thread_store) = self.thread_store.upgrade() else {
            return Task::ready(());
        };

        let search_task = search_rules(query, Arc::new(AtomicBool::default()), thread_store, cx);
        cx.spawn_in(window, async move |this, cx| {
            let matches = search_task.await;
            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(thread_store) = self.thread_store.upgrade() else {
            return;
        };

        let prompt_id = entry.prompt_id;

        let load_rules_task = thread_store.update(cx, |thread_store, cx| {
            thread_store.load_rules(prompt_id, cx)
        });

        cx.spawn(async move |this, cx| {
            let (metadata, text) = load_rules_task.await?;
            let Some(title) = metadata.title else {
                return Err(anyhow!("Encountered user rule with no title when attempting to add it to agent context."));
            };
            this.update(cx, |this, cx| {
                this.delegate
                    .context_store
                    .update(cx, |context_store, cx| {
                        context_store.add_rules(prompt_id, title, text, true, cx)
                    })
                    .ok();
            })
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.context_picker
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let thread = &self.matches[ix];

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_thread_context_entry(thread, self.context_store.clone(), cx),
        ))
    }
}

pub fn render_thread_context_entry(
    user_rules: &RulesContextEntry,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Div {
    let added = context_store.upgrade().map_or(false, |ctx_store| {
        ctx_store
            .read(cx)
            .includes_user_rules(&user_rules.prompt_id)
            .is_some()
    });

    h_flex()
        .gap_1p5()
        .w_full()
        .justify_between()
        .child(
            h_flex()
                .gap_1p5()
                .max_w_72()
                .child(
                    Icon::new(RULES_ICON)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .child(Label::new(user_rules.title.clone()).truncate()),
        )
        .when(added, |el| {
            el.child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(Label::new("Added").size(LabelSize::Small)),
            )
        })
}

pub(crate) fn search_rules(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    thread_store: Entity<ThreadStore>,
    cx: &mut App,
) -> Task<Vec<RulesContextEntry>> {
    let Some(prompt_store) = thread_store.read(cx).prompt_store() else {
        return Task::ready(vec![]);
    };
    let search_task = prompt_store.read(cx).search(query, cancellation_flag, cx);
    cx.background_spawn(async move {
        search_task
            .await
            .into_iter()
            .flat_map(|metadata| {
                // Default prompts are filtered out as they are automatically included.
                if metadata.default {
                    None
                } else {
                    match metadata.id {
                        PromptId::EditWorkflow => None,
                        PromptId::User { uuid } => Some(RulesContextEntry {
                            prompt_id: uuid,
                            title: metadata.title?,
                        }),
                    }
                }
            })
            .collect::<Vec<_>>()
    })
}
