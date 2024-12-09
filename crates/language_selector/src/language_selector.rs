mod active_buffer_language;

pub use active_buffer_language::ActiveBufferLanguage;
use anyhow::anyhow;
use editor::Editor;
use file_finder::file_finder_settings::FileFinderSettings;
use file_icons::FileIcons;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WeakView,
};
use language::{Buffer, LanguageMatcher, LanguageName, LanguageRegistry};
use picker::{Picker, PickerDelegate};
use project::Project;
use settings::Settings;
use std::{ops::Not as _, path::Path, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(language_selector, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(LanguageSelector::register).detach();
}

pub struct LanguageSelector {
    picker: View<Picker<LanguageSelectorDelegate>>,
}

impl LanguageSelector {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &Toggle, cx| {
            Self::toggle(workspace, cx);
        });
    }

    fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Option<()> {
        let registry = workspace.app_state().languages.clone();
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        workspace.toggle_modal(cx, move |cx| {
            LanguageSelector::new(buffer, project, registry, cx)
        });
        Some(())
    }

    fn new(
        buffer: Model<Buffer>,
        project: Model<Project>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = LanguageSelectorDelegate::new(
            cx.view().downgrade(),
            buffer,
            project,
            language_registry,
        );

        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
        Self { picker }
    }
}

impl Render for LanguageSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl FocusableView for LanguageSelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for LanguageSelector {}
impl ModalView for LanguageSelector {}

pub struct LanguageSelectorDelegate {
    language_selector: WeakView<LanguageSelector>,
    buffer: Model<Buffer>,
    project: Model<Project>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl LanguageSelectorDelegate {
    fn new(
        language_selector: WeakView<LanguageSelector>,
        buffer: Model<Buffer>,
        project: Model<Project>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let candidates = language_registry
            .language_names()
            .into_iter()
            .filter_map(|name| {
                language_registry
                    .available_language_for_name(&name)?
                    .hidden()
                    .not()
                    .then_some(name)
            })
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name))
            .collect::<Vec<_>>();

        Self {
            language_selector,
            buffer,
            project,
            language_registry,
            candidates,
            matches: vec![],
            selected_index: 0,
        }
    }

    fn language_data_for_match(
        &self,
        mat: &StringMatch,
        cx: &AppContext,
    ) -> (String, Option<Icon>) {
        let mut label = mat.string.clone();
        let buffer_language = self.buffer.read(cx).language();
        let need_icon = FileFinderSettings::get_global(cx).file_icons;
        if let Some(buffer_language) = buffer_language {
            let buffer_language_name = buffer_language.name();
            if buffer_language_name.0.as_ref() == mat.string.as_str() {
                label.push_str(" (current)");
                let icon = need_icon
                    .then(|| self.language_icon(&buffer_language.config().matcher, cx))
                    .flatten();
                return (label, icon);
            }
        }

        if need_icon {
            let language_name = LanguageName::new(mat.string.as_str());
            match self
                .language_registry
                .available_language_for_name(&language_name.0)
            {
                Some(available_language) => {
                    let icon = self.language_icon(available_language.matcher(), cx);
                    (label, icon)
                }
                None => (label, None),
            }
        } else {
            (label, None)
        }
    }

    fn language_icon(&self, matcher: &LanguageMatcher, cx: &AppContext) -> Option<Icon> {
        matcher
            .path_suffixes
            .iter()
            .find_map(|extension| {
                if extension.contains('.') {
                    None
                } else {
                    FileIcons::get_icon(Path::new(&format!("file.{extension}")), cx)
                }
            })
            .map(Icon::from_path)
            .map(|icon| icon.color(Color::Muted))
    }
}

impl PickerDelegate for LanguageSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a language…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let language_name = &self.candidates[mat.candidate_id].string;
            let language = self.language_registry.language_for_name(language_name);
            let project = self.project.downgrade();
            let buffer = self.buffer.downgrade();
            cx.spawn(|_, mut cx| async move {
                let language = language.await?;
                let project = project
                    .upgrade()
                    .ok_or_else(|| anyhow!("project was dropped"))?;
                let buffer = buffer
                    .upgrade()
                    .ok_or_else(|| anyhow!("buffer was dropped"))?;
                project.update(&mut cx, |project, cx| {
                    project.set_language_for_buffer(&buffer, language, cx);
                })
            })
            .detach_and_log_err(cx);
        }
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.language_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn(|this, mut cx| async move {
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
                    background,
                )
                .await
            };

            this.update(&mut cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let (label, language_icon) = self.language_data_for_match(mat, cx);
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .start_slot::<Icon>(language_icon)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}
