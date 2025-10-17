use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::{
    context_picker::ContextPicker,
    context_store::{self, ContextStore},
};
use agent::{HistoryEntry, HistoryStore};
use fuzzy::StringMatchCandidate;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, prelude::*};
use workspace::Workspace;

pub struct ThreadContextPicker {
    picker: Entity<Picker<ThreadContextPickerDelegate>>,
}

impl ThreadContextPicker {
    pub fn new(
        thread_store: WeakEntity<HistoryStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ThreadContextPickerDelegate::new(
            thread_store,
            context_picker,
            context_store,
            workspace,
        );
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

pub struct ThreadContextPickerDelegate {
    thread_store: WeakEntity<HistoryStore>,
    context_picker: WeakEntity<ContextPicker>,
    context_store: WeakEntity<context_store::ContextStore>,
    workspace: WeakEntity<Workspace>,
    matches: Vec<HistoryEntry>,
    selected_index: usize,
}

impl ThreadContextPickerDelegate {
    pub fn new(
        thread_store: WeakEntity<HistoryStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        ThreadContextPickerDelegate {
            thread_store,
            context_picker,
            context_store,
            workspace,
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

        let search_task = search_threads(query, Arc::new(AtomicBool::default()), &thread_store, cx);
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
        let Some(project) = self
            .workspace
            .upgrade()
            .map(|w| w.read(cx).project().clone())
        else {
            return;
        };
        let Some((entry, thread_store)) = self
            .matches
            .get(self.selected_index)
            .zip(self.thread_store.upgrade())
        else {
            return;
        };

        match entry {
            HistoryEntry::AcpThread(thread) => {
                let load_thread_task =
                    agent::load_agent_thread(thread.id.clone(), thread_store, project, cx);

                cx.spawn(async move |this, cx| {
                    let thread = load_thread_task.await?;
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
            HistoryEntry::TextThread(thread) => {
                let task = thread_store.update(cx, |this, cx| {
                    this.load_text_thread(thread.path.clone(), cx)
                });

                cx.spawn(async move |this, cx| {
                    let thread = task.await?;
                    this.update(cx, |this, cx| {
                        this.delegate
                            .context_store
                            .update(cx, |context_store, cx| {
                                context_store.add_text_thread(thread, true, cx)
                            })
                            .ok();
                    })
                })
                .detach_and_log_err(cx);
            }
        }
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
        let thread = &self.matches.get(ix)?;

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_thread_context_entry(thread, self.context_store.clone(), cx),
        ))
    }
}

pub fn render_thread_context_entry(
    entry: &HistoryEntry,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Div {
    let is_added = match entry {
        HistoryEntry::AcpThread(thread) => context_store
            .upgrade()
            .is_some_and(|ctx_store| ctx_store.read(cx).includes_thread(&thread.id)),
        HistoryEntry::TextThread(thread) => context_store
            .upgrade()
            .is_some_and(|ctx_store| ctx_store.read(cx).includes_text_thread(&thread.path)),
    };

    h_flex()
        .gap_1p5()
        .w_full()
        .justify_between()
        .child(
            h_flex()
                .gap_1p5()
                .max_w_72()
                .child(
                    Icon::new(IconName::Thread)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .child(Label::new(entry.title().clone()).truncate()),
        )
        .when(is_added, |el| {
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

pub(crate) fn search_threads(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    thread_store: &Entity<HistoryStore>,
    cx: &mut App,
) -> Task<Vec<HistoryEntry>> {
    let threads = thread_store.read(cx).entries().collect();
    if query.is_empty() {
        return Task::ready(threads);
    }

    let executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        let candidates = threads
            .iter()
            .enumerate()
            .map(|(id, thread)| StringMatchCandidate::new(id, thread.title()))
            .collect::<Vec<_>>();
        let matches = fuzzy::match_strings(
            &candidates,
            &query,
            false,
            true,
            100,
            &cancellation_flag,
            executor,
        )
        .await;

        matches
            .into_iter()
            .map(|mat| threads[mat.candidate_id].clone())
            .collect()
    })
}
