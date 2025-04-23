use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use fuzzy::StringMatchCandidate;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, prelude::*};

use crate::context_picker::ContextPicker;
use crate::context_store::{self, ContextStore};
use crate::thread::ThreadId;
use crate::thread_store::ThreadStore;

pub struct ThreadContextPicker {
    picker: Entity<Picker<ThreadContextPickerDelegate>>,
}

impl ThreadContextPicker {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate =
            ThreadContextPickerDelegate::new(thread_store, context_picker, context_store);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        ThreadContextPicker { picker }
    }
}

impl Focusable for ThreadContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ThreadContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ThreadContextEntry {
    pub id: ThreadId,
    pub summary: SharedString,
}

pub struct ThreadContextPickerDelegate {
    thread_store: WeakEntity<ThreadStore>,
    context_picker: WeakEntity<ContextPicker>,
    context_store: WeakEntity<context_store::ContextStore>,
    matches: Vec<ThreadContextEntry>,
    selected_index: usize,
}

impl ThreadContextPickerDelegate {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
    ) -> Self {
        ThreadContextPickerDelegate {
            thread_store,
            context_picker,
            context_store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for ThreadContextPickerDelegate {
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
        "Search threads…".into()
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

        let search_task = search_threads(query, Arc::new(AtomicBool::default()), thread_store, cx);
        cx.spawn_in(window, async move |this, cx| {
            let matches = search_task.await;
            this.update(cx, |this, cx| {
                this.delegate.matches = matches.into_iter().map(|mat| mat.thread).collect();
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

        let open_thread_task = thread_store.update(cx, |this, cx| this.open_thread(&entry.id, cx));

        cx.spawn(async move |this, cx| {
            let thread = open_thread_task.await?;
            this.update(cx, |this, cx| {
                this.delegate
                    .context_store
                    .update(cx, |context_store, cx| {
                        context_store.add_thread(thread, true, cx)
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
    thread: &ThreadContextEntry,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Div {
    let added = context_store.upgrade().map_or(false, |ctx_store| {
        ctx_store.read(cx).includes_thread(&thread.id).is_some()
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
                    Icon::new(IconName::MessageBubbles)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .child(Label::new(thread.summary.clone()).truncate()),
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

#[derive(Clone)]
pub struct ThreadMatch {
    pub thread: ThreadContextEntry,
    pub is_recent: bool,
}

pub(crate) fn search_threads(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    thread_store: Entity<ThreadStore>,
    cx: &mut App,
) -> Task<Vec<ThreadMatch>> {
    let threads = thread_store
        .read(cx)
        .threads()
        .into_iter()
        .map(|thread| ThreadContextEntry {
            id: thread.id,
            summary: thread.summary,
        })
        .collect::<Vec<_>>();

    let executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        if query.is_empty() {
            threads
                .into_iter()
                .map(|thread| ThreadMatch {
                    thread,
                    is_recent: false,
                })
                .collect()
        } else {
            let candidates = threads
                .iter()
                .enumerate()
                .map(|(id, thread)| StringMatchCandidate::new(id, &thread.summary))
                .collect::<Vec<_>>();
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await;

            matches
                .into_iter()
                .map(|mat| ThreadMatch {
                    thread: threads[mat.candidate_id].clone(),
                    is_recent: false,
                })
                .collect()
        }
    })
}
