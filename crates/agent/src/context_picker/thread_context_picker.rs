use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use chrono::{DateTime, Utc};
use fuzzy::StringMatchCandidate;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, prelude::*};

use crate::context_picker::ContextPicker;
use crate::context_store::{self, ContextStore};
use crate::thread::ThreadId;
use crate::thread_store::{TextThreadStore, ThreadStore};

pub struct ThreadContextPicker {
    picker: Entity<Picker<ThreadContextPickerDelegate>>,
}

impl ThreadContextPicker {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        text_thread_context_store: WeakEntity<TextThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ThreadContextPickerDelegate::new(
            thread_store,
            text_thread_context_store,
            context_picker,
            context_store,
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

#[derive(Debug, Clone)]
pub enum ThreadContextEntry {
    Thread {
        id: ThreadId,
        title: SharedString,
    },
    Context {
        path: Arc<Path>,
        title: SharedString,
    },
}

impl ThreadContextEntry {
    pub fn title(&self) -> &SharedString {
        match self {
            Self::Thread { title, .. } => title,
            Self::Context { title, .. } => title,
        }
    }
}

pub struct ThreadContextPickerDelegate {
    thread_store: WeakEntity<ThreadStore>,
    text_thread_store: WeakEntity<TextThreadStore>,
    context_picker: WeakEntity<ContextPicker>,
    context_store: WeakEntity<context_store::ContextStore>,
    matches: Vec<ThreadContextEntry>,
    selected_index: usize,
}

impl ThreadContextPickerDelegate {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        text_thread_store: WeakEntity<TextThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
    ) -> Self {
        ThreadContextPickerDelegate {
            thread_store,
            context_picker,
            context_store,
            text_thread_store,
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
        let Some((thread_store, text_thread_context_store)) = self
            .thread_store
            .upgrade()
            .zip(self.text_thread_store.upgrade())
        else {
            return Task::ready(());
        };

        let search_task = search_threads(
            query,
            Arc::new(AtomicBool::default()),
            thread_store,
            text_thread_context_store,
            cx,
        );
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

        match entry {
            ThreadContextEntry::Thread { id, .. } => {
                let Some(thread_store) = self.thread_store.upgrade() else {
                    return;
                };
                let open_thread_task =
                    thread_store.update(cx, |this, cx| this.open_thread(&id, cx));

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
            ThreadContextEntry::Context { path, .. } => {
                let Some(text_thread_store) = self.text_thread_store.upgrade() else {
                    return;
                };
                let task = text_thread_store
                    .update(cx, |this, cx| this.open_local_context(path.clone(), cx));

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
        let thread = &self.matches[ix];

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_thread_context_entry(thread, self.context_store.clone(), cx),
        ))
    }
}

pub fn render_thread_context_entry(
    entry: &ThreadContextEntry,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Div {
    let is_added = match entry {
        ThreadContextEntry::Thread { id, .. } => context_store
            .upgrade()
            .map_or(false, |ctx_store| ctx_store.read(cx).includes_thread(&id)),
        ThreadContextEntry::Context { path, .. } => {
            context_store.upgrade().map_or(false, |ctx_store| {
                ctx_store.read(cx).includes_text_thread(path)
            })
        }
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
                    Icon::new(IconName::MessageBubbles)
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

#[derive(Clone)]
pub struct ThreadMatch {
    pub thread: ThreadContextEntry,
    pub is_recent: bool,
}

pub fn unordered_thread_entries(
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
    cx: &App,
) -> impl Iterator<Item = (DateTime<Utc>, ThreadContextEntry)> {
    let threads = thread_store.read(cx).unordered_threads().map(|thread| {
        (
            thread.updated_at,
            ThreadContextEntry::Thread {
                id: thread.id.clone(),
                title: thread.summary.clone(),
            },
        )
    });

    let text_threads = text_thread_store
        .read(cx)
        .unordered_contexts()
        .map(|context| {
            (
                context.mtime.to_utc(),
                ThreadContextEntry::Context {
                    path: context.path.clone(),
                    title: context.title.clone().into(),
                },
            )
        });

    threads.chain(text_threads)
}

pub(crate) fn search_threads(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
    cx: &mut App,
) -> Task<Vec<ThreadMatch>> {
    let mut threads =
        unordered_thread_entries(thread_store, text_thread_store, cx).collect::<Vec<_>>();
    threads.sort_unstable_by_key(|(updated_at, _)| std::cmp::Reverse(*updated_at));

    let executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        if query.is_empty() {
            threads
                .into_iter()
                .map(|(_, thread)| ThreadMatch {
                    thread,
                    is_recent: false,
                })
                .collect()
        } else {
            let candidates = threads
                .iter()
                .enumerate()
                .map(|(id, (_, thread))| StringMatchCandidate::new(id, &thread.title()))
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
                    thread: threads[mat.candidate_id].1.clone(),
                    is_recent: false,
                })
                .collect()
        }
    })
}
