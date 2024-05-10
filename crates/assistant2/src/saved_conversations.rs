use std::sync::Arc;

use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, View, WeakView};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;

use crate::saved_conversation::SavedConversationMetadata;

pub struct SavedConversations {
    focus_handle: FocusHandle,
    picker: Option<View<Picker<SavedConversationPickerDelegate>>>,
}

impl EventEmitter<DismissEvent> for SavedConversations {}

impl FocusableView for SavedConversations {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        if let Some(picker) = self.picker.as_ref() {
            picker.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl SavedConversations {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            picker: None,
        }
    }

    pub fn init(
        &mut self,
        saved_conversations: Vec<SavedConversationMetadata>,
        cx: &mut ViewContext<Self>,
    ) {
        let delegate =
            SavedConversationPickerDelegate::new(cx.view().downgrade(), saved_conversations);
        self.picker = Some(cx.new_view(|cx| Picker::uniform_list(delegate, cx).modal(false)));
    }
}

impl Render for SavedConversations {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .bg(cx.theme().colors().panel_background)
            .children(self.picker.clone())
    }
}

pub struct SavedConversationPickerDelegate {
    view: WeakView<SavedConversations>,
    saved_conversations: Vec<SavedConversationMetadata>,
    selected_index: usize,
    matches: Vec<StringMatch>,
}

impl SavedConversationPickerDelegate {
    pub fn new(
        weak_view: WeakView<SavedConversations>,
        saved_conversations: Vec<SavedConversationMetadata>,
    ) -> Self {
        let matches = saved_conversations
            .iter()
            .map(|conversation| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: conversation.title.clone(),
            })
            .collect();

        Self {
            view: weak_view,
            saved_conversations,
            selected_index: 0,
            matches,
        }
    }
}

impl PickerDelegate for SavedConversationPickerDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select saved conversation...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background_executor = cx.background_executor().clone();
        let candidates = self
            .saved_conversations
            .iter()
            .enumerate()
            .map(|(id, conversation)| {
                let text = conversation.title.clone();

                StringMatchCandidate {
                    id,
                    char_bag: text.as_str().into(),
                    string: text,
                }
            })
            .collect::<Vec<_>>();

        cx.spawn(move |this, mut cx| async move {
            let matches = if query.is_empty() {
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
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background_executor,
                )
                .await
            };

            this.update(&mut cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(cx);
            return;
        }

        // TODO: Implement selecting a saved conversation.
    }

    fn dismissed(&mut self, cx: &mut ui::prelude::ViewContext<Picker<Self>>) {
        self.view
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let conversation_match = &self.matches[ix];
        let _conversation = &self.saved_conversations[conversation_match.candidate_id];

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(
                    conversation_match.string.clone(),
                    conversation_match.positions.clone(),
                )),
        )
    }
}
