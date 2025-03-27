use std::sync::Arc;

use fuzzy::StringMatchCandidate;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, HighlightedLabel, ListItem};

use crate::context_picker::{ConfirmBehavior, ContextPicker};
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
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ThreadContextPickerDelegate::new(
            thread_store,
            context_picker,
            context_store,
            confirm_behavior,
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
pub struct ThreadContextEntry {
    pub id: ThreadId,
    pub summary: SharedString,
    pub highlight_positions: Option<Vec<usize>>,
}

pub struct ThreadContextPickerDelegate {
    thread_store: WeakEntity<ThreadStore>,
    context_picker: WeakEntity<ContextPicker>,
    context_store: WeakEntity<context_store::ContextStore>,
    confirm_behavior: ConfirmBehavior,
    matches: Vec<ThreadContextEntry>,
    selected_index: usize,
}

impl ThreadContextPickerDelegate {
    pub fn new(
        thread_store: WeakEntity<ThreadStore>,
        context_picker: WeakEntity<ContextPicker>,
        context_store: WeakEntity<context_store::ContextStore>,
        confirm_behavior: ConfirmBehavior,
    ) -> Self {
        ThreadContextPickerDelegate {
            thread_store,
            context_picker,
            context_store,
            confirm_behavior,
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
        let Some(threads) = self.thread_store.upgrade() else {
            return Task::ready(());
        };

        let search_task = search_threads(query, threads, cx);
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

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(thread_store) = self.thread_store.upgrade() else {
            return;
        };

        let open_thread_task = thread_store.update(cx, |this, cx| this.open_thread(&entry.id, cx));

        cx.spawn_in(window, async move |this, cx| {
            let thread = open_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.delegate
                    .context_store
                    .update(cx, |context_store, cx| {
                        context_store.add_thread(thread, true, cx)
                    })
                    .ok();

                match this.delegate.confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(window, cx),
                }
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

        let highlights = thread
            .highlight_positions
            .as_ref()
            .map(|vec| vec.as_slice());

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_thread_context_entry_with_highlights(
                thread,
                self.context_store.clone(),
                highlights.as_deref(),
                cx,
            ),
        ))
    }
}

pub fn render_thread_context_entry(
    thread: &ThreadContextEntry,
    context_store: WeakEntity<ContextStore>,
    cx: &App,
) -> Div {
    render_thread_context_entry_with_highlights(thread, context_store, None, cx)
}

pub fn render_thread_context_entry_with_highlights(
    thread: &ThreadContextEntry,
    context_store: WeakEntity<ContextStore>,
    highlight_positions: Option<&[usize]>,
    cx: &App,
) -> Div {
    let added = context_store.upgrade().map_or(false, |ctx_store| {
        ctx_store.read(cx).includes_thread(&thread.id).is_some()
    });

    // Choose between regular label or highlighted label based on position data
    let summary_element = match highlight_positions {
        Some(positions) => HighlightedLabel::new(thread.summary.clone(), positions.to_vec())
            .truncate()
            .into_any_element(),
        None => Label::new(thread.summary.clone())
            .truncate()
            .into_any_element(),
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
                    Icon::new(IconName::MessageCircle)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .child(summary_element),
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

pub(crate) fn search_threads(
    query: String,
    thread_store: Entity<ThreadStore>,
    cx: &mut App,
) -> Task<Vec<ThreadContextEntry>> {
    // Get threads from the thread store
    let threads = thread_store
        .read(cx)
        .threads()
        .into_iter()
        .map(|thread| ThreadContextEntry {
            id: thread.id,
            summary: thread.summary,
            highlight_positions: None, // Initialize with no highlights
        })
        .collect::<Vec<_>>();

    // Return early for empty queries or if there are no threads
    if threads.is_empty() || query.is_empty() {
        return Task::ready(threads);
    }

    // Create candidates list for fuzzy matching
    let candidates: Vec<_> = threads
        .iter()
        .enumerate()
        .map(|(id, thread)| StringMatchCandidate::new(id, &thread.summary))
        .collect();

    let executor = cx.background_executor().clone();
    let threads_clone = threads.clone();

    // Use background executor for the matching
    cx.background_executor().spawn(async move {
        // Perform fuzzy matching in background
        let matches = fuzzy::match_strings(
            &candidates,
            &query,
            false,
            100,
            &Default::default(),
            executor,
        )
        .await;

        // Create result entries with highlight positions included
        let result = matches
            .into_iter()
            .filter_map(|mat| {
                let thread = threads_clone.get(mat.candidate_id)?;
                // Create a new entry with the highlight positions
                Some(ThreadContextEntry {
                    id: thread.id.clone(),
                    summary: thread.summary.clone(),
                    highlight_positions: Some(mat.positions),
                })
            })
            .collect::<Vec<ThreadContextEntry>>();

        result
    })
}
