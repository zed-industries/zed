use std::sync::Arc;

use fuzzy::StringMatchCandidate;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem};

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{self, ContextStore};
use crate::thread::ThreadId;
use crate::thread_store::ThreadStore;

pub struct ThreadContextPicker {
    picker: View<Picker<ThreadContextPickerDelegate>>,
}

impl ThreadContextPicker {
    pub fn new(
        thread_store: WeakModel<ThreadStore>,
        context_picker: WeakView<ContextPicker>,
        context_store: WeakModel<context_store::ContextStore>,
        confirm_behavior: ConfirmBehavior,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = ThreadContextPickerDelegate::new(
            thread_store,
            context_picker,
            context_store,
            confirm_behavior,
        );
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        ThreadContextPicker { picker }
    }
}

impl FocusableView for ThreadContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ThreadContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ThreadContextEntry {
    pub id: ThreadId,
    pub summary: SharedString,
}

pub struct ThreadContextPickerDelegate {
    thread_store: WeakModel<ThreadStore>,
    context_picker: WeakView<ContextPicker>,
    context_store: WeakModel<context_store::ContextStore>,
    confirm_behavior: ConfirmBehavior,
    matches: Vec<ThreadContextEntry>,
    selected_index: usize,
}

impl ThreadContextPickerDelegate {
    pub fn new(
        thread_store: WeakModel<ThreadStore>,
        context_picker: WeakView<ContextPicker>,
        context_store: WeakModel<context_store::ContextStore>,
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

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search threads…".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let Ok(threads) = self.thread_store.update(cx, |this, cx| {
            this.threads(cx)
                .into_iter()
                .map(|thread| {
                    let id = thread.read(cx).id().clone();
                    let summary = thread.read(cx).summary_or_default();
                    ThreadContextEntry { id, summary }
                })
                .collect::<Vec<_>>()
        }) else {
            return Task::ready(());
        };

        let executor = cx.background_executor().clone();
        let search_task = cx.background_executor().spawn(async move {
            if query.is_empty() {
                threads
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
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|mat| threads[mat.candidate_id].clone())
                    .collect()
            }
        });

        cx.spawn(|this, mut cx| async move {
            let matches = search_task.await;
            this.update(&mut cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(thread_store) = self.thread_store.upgrade() else {
            return;
        };

        let Some(thread) = thread_store.update(cx, |this, cx| this.open_thread(&entry.id, cx))
        else {
            return;
        };

        self.context_store
            .update(cx, |context_store, cx| context_store.add_thread(thread, cx))
            .ok();

        match self.confirm_behavior {
            ConfirmBehavior::KeepOpen => {}
            ConfirmBehavior::Close => self.dismissed(cx),
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
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
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let thread = &self.matches[ix];

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_thread_context_entry(thread, self.context_store.clone(), cx),
        ))
    }
}

pub fn render_thread_context_entry(
    thread: &ThreadContextEntry,
    context_store: WeakModel<ContextStore>,
    cx: &mut WindowContext,
) -> Div {
    let added = context_store.upgrade().map_or(false, |ctx_store| {
        ctx_store.read(cx).includes_thread(&thread.id).is_some()
    });

    h_flex()
        .gap_1()
        .w_full()
        .child(Icon::new(IconName::MessageCircle).size(IconSize::Small))
        .child(Label::new(thread.summary.clone()))
        .child(div().w_full())
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
