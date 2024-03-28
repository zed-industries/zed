use std::sync::Arc;

use client::ExtensionMetadata;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    prelude::*, AppContext, DismissEvent, EventEmitter, FocusableView, Task, View, WeakView,
};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::ModalView;

pub struct ExtensionVersionSelector {
    picker: View<Picker<ExtensionVersionSelectorDelegate>>,
}

impl ModalView for ExtensionVersionSelector {}

impl EventEmitter<DismissEvent> for ExtensionVersionSelector {}

impl FocusableView for ExtensionVersionSelector {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ExtensionVersionSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl ExtensionVersionSelector {
    pub fn new(delegate: ExtensionVersionSelectorDelegate, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
        Self { picker }
    }
}

pub struct ExtensionVersionSelectorDelegate {
    view: WeakView<ExtensionVersionSelector>,
    extension_versions: Vec<ExtensionMetadata>,
    selected_index: usize,
    matches: Vec<StringMatch>,
}

impl ExtensionVersionSelectorDelegate {
    pub fn new(
        weak_view: WeakView<ExtensionVersionSelector>,
        extension_versions: Vec<ExtensionMetadata>,
    ) -> Self {
        let matches = extension_versions
            .iter()
            .map(|extension| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: format!("v{}", extension.manifest.version),
            })
            .collect();

        Self {
            view: weak_view,
            extension_versions,
            selected_index: 0,
            matches,
        }
    }
}

impl PickerDelegate for ExtensionVersionSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select extension version...".into()
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

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let background_executor = cx.background_executor().clone();
        let candidates = self
            .extension_versions
            .iter()
            .enumerate()
            .map(|(id, extension)| {
                let text = format!("v{}", extension.manifest.version);

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

    fn confirm(&mut self, _secondary: bool, _cx: &mut ViewContext<Picker<Self>>) {
        // TODO
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
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
        let version_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(
                    version_match.string.clone(),
                    version_match.positions.clone(),
                )),
        )
    }
}
