use std::str::FromStr;
use std::sync::Arc;

use client::ExtensionMetadata;
use extension::{ExtensionSettings, ExtensionStore};
use fs::Fs;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    prelude::*, AppContext, DismissEvent, EventEmitter, FocusableView, Task, View, WeakView,
};
use picker::{Picker, PickerDelegate};
use release_channel::ReleaseChannel;
use semantic_version::SemanticVersion;
use settings::update_settings_file;
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
    fs: Arc<dyn Fs>,
    view: WeakView<ExtensionVersionSelector>,
    extension_versions: Vec<ExtensionMetadata>,
    selected_index: usize,
    matches: Vec<StringMatch>,
}

impl ExtensionVersionSelectorDelegate {
    pub fn new(
        fs: Arc<dyn Fs>,
        weak_view: WeakView<ExtensionVersionSelector>,
        mut extension_versions: Vec<ExtensionMetadata>,
    ) -> Self {
        extension_versions.sort_unstable_by(|a, b| {
            let a_version = SemanticVersion::from_str(&a.manifest.version);
            let b_version = SemanticVersion::from_str(&b.manifest.version);

            match (a_version, b_version) {
                (Ok(a_version), Ok(b_version)) => b_version.cmp(&a_version),
                _ => b.published_at.cmp(&a.published_at),
            }
        });

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
            fs,
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

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(cx);
            return;
        }

        let candidate_id = self.matches[self.selected_index].candidate_id;
        let extension_version = &self.extension_versions[candidate_id];

        if !extension::is_version_compatible(ReleaseChannel::global(cx), extension_version) {
            return;
        }

        let extension_store = ExtensionStore::global(cx);
        extension_store.update(cx, |store, cx| {
            let extension_id = extension_version.id.clone();
            let version = extension_version.manifest.version.clone();

            update_settings_file::<ExtensionSettings>(self.fs.clone(), cx, {
                let extension_id = extension_id.clone();
                move |settings| {
                    settings.auto_update_extensions.insert(extension_id, false);
                }
            });

            store.install_extension(extension_id, version, cx);
        });
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
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let version_match = &self.matches[ix];
        let extension_version = &self.extension_versions[version_match.candidate_id];

        let is_version_compatible =
            extension::is_version_compatible(ReleaseChannel::global(cx), extension_version);
        let disabled = !is_version_compatible;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .disabled(disabled)
                .child(
                    HighlightedLabel::new(
                        version_match.string.clone(),
                        version_match.positions.clone(),
                    )
                    .when(disabled, |label| label.color(Color::Muted)),
                )
                .end_slot(
                    h_flex()
                        .gap_2()
                        .when(!is_version_compatible, |this| {
                            this.child(Label::new("Incompatible").color(Color::Muted))
                        })
                        .child(
                            Label::new(
                                extension_version
                                    .published_at
                                    .format("%Y-%m-%d")
                                    .to_string(),
                            )
                            .when(disabled, |label| label.color(Color::Muted)),
                        ),
                ),
        )
    }
}
