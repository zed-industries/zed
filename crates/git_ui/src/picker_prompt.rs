use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate};

use core::cmp;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Task, WeakEntity,
    Window, rems,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

pub struct PickerPrompt {
    pub picker: Entity<Picker<PickerPromptDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

pub fn prompt(
    prompt: &str,
    options: Vec<SharedString>,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Task<Option<usize>> {
    if options.is_empty() {
        return Task::ready(None);
    } else if options.len() == 1 {
        return Task::ready(Some(0));
    }
    let prompt = prompt.to_string().into();

    window.spawn(cx, async move |cx| {
        // Modal branch picker has a longer trailoff than a popover one.
        let (tx, rx) = oneshot::channel();
        let delegate = PickerPromptDelegate::new(prompt, options, tx, 70);

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    PickerPrompt::new(delegate, 34., window, cx)
                })
            })
            .ok();

        (rx.await).ok()
    })
}

impl PickerPrompt {
    fn new(
        delegate: PickerPromptDelegate,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        Self {
            picker,
            rem_width,
            _subscription,
        }
    }
}
impl ModalView for PickerPrompt {}
impl EventEmitter<DismissEvent> for PickerPrompt {}

impl Focusable for PickerPrompt {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for PickerPrompt {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(self.rem_width))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
    }
}

pub struct PickerPromptDelegate {
    prompt: Arc<str>,
    matches: Vec<StringMatch>,
    all_options: Vec<SharedString>,
    selected_index: usize,
    max_match_length: usize,
    tx: Option<oneshot::Sender<usize>>,
}

impl PickerPromptDelegate {
    pub fn new(
        prompt: Arc<str>,
        options: Vec<SharedString>,
        tx: oneshot::Sender<usize>,
        max_chars: usize,
    ) -> Self {
        Self {
            prompt,
            all_options: options,
            matches: vec![],
            selected_index: 0,
            max_match_length: max_chars,
            tx: Some(tx),
        }
    }
}

impl PickerDelegate for PickerPromptDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.prompt.clone()
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
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |picker, cx| {
            let candidates = picker.read_with(cx, |picker, _| {
                picker
                    .delegate
                    .all_options
                    .iter()
                    .enumerate()
                    .map(|(ix, option)| StringMatchCandidate::new(ix, option))
                    .collect::<Vec<StringMatchCandidate>>()
            });
            let Some(candidates) = candidates.log_err() else {
                return;
            };
            let matches: Vec<StringMatch> = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
            };
            picker
                .update(cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(option) = self.matches.get(self.selected_index()) else {
            return;
        };

        self.tx.take().map(|tx| tx.send(option.candidate_id));
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches[ix];
        let shortened_option = util::truncate_and_trailoff(&hit.string, self.max_match_length);

        Some(
            ListItem::new(SharedString::from(format!("picker-prompt-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .map(|el| {
                    let highlights: Vec<_> = hit
                        .positions
                        .iter()
                        .filter(|index| index < &&self.max_match_length)
                        .copied()
                        .collect();

                    el.child(HighlightedLabel::new(shortened_option, highlights))
                }),
        )
    }
}
