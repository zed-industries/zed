mod active_buffer_language;

pub use active_buffer_language::ActiveBufferLanguage;
use anyhow::Context as _;
use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, Styled, WeakEntity, Window, actions,
};
use language::{Buffer, LanguageMatcher, LanguageName, LanguageRegistry};
use open_path_prompt::file_finder_settings::FileFinderSettings;
use picker::{Picker, PickerDelegate};
use project::Project;
use settings::Settings;
use std::{ops::Not as _, path::Path, sync::Arc};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(
    language_selector,
    [
        /// Toggles the language selector modal.
        Toggle
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(LanguageSelector::register).detach();
}

pub struct LanguageSelector {
    picker: Entity<Picker<LanguageSelectorDelegate>>,
}

impl LanguageSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let registry = workspace.app_state().languages.clone();
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        workspace.toggle_modal(window, cx, move |window, cx| {
            LanguageSelector::new(buffer, project, registry, window, cx)
        });
        Some(())
    }

    fn new(
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = LanguageSelectorDelegate::new(
            cx.entity().downgrade(),
            buffer,
            project,
            language_registry,
        );

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for LanguageSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("LanguageSelector")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl Focusable for LanguageSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for LanguageSelector {}
impl ModalView for LanguageSelector {}

pub struct LanguageSelectorDelegate {
    language_selector: WeakEntity<LanguageSelector>,
    buffer: Entity<Buffer>,
    project: Entity<Project>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl LanguageSelectorDelegate {
    fn new(
        language_selector: WeakEntity<LanguageSelector>,
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let candidates = language_registry
            .language_names()
            .into_iter()
            .filter_map(|name| {
                language_registry
                    .available_language_for_name(name.as_ref())?
                    .hidden()
                    .not()
                    .then_some(name)
            })
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name.as_ref()))
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

    fn language_data_for_match(&self, mat: &StringMatch, cx: &App) -> (String, Option<Icon>) {
        let mut label = mat.string.clone();
        let buffer_language = self.buffer.read(cx).language();
        let need_icon = FileFinderSettings::get_global(cx).file_icons;

        if let Some(buffer_language) = buffer_language
            .filter(|buffer_language| buffer_language.name().as_ref() == mat.string.as_str())
        {
            label.push_str(" (current)");
            let icon = need_icon
                .then(|| self.language_icon(&buffer_language.config().matcher, cx))
                .flatten();
            (label, icon)
        } else {
            let icon = need_icon
                .then(|| {
                    let language_name = LanguageName::new(mat.string.as_str());
                    self.language_registry
                        .available_language_for_name(language_name.as_ref())
                        .and_then(|available_language| {
                            self.language_icon(available_language.matcher(), cx)
                        })
                })
                .flatten();
            (label, icon)
        }
    }

    fn language_icon(&self, matcher: &LanguageMatcher, cx: &App) -> Option<Icon> {
        matcher
            .path_suffixes
            .iter()
            .find_map(|extension| file_icons::FileIcons::get_icon(Path::new(extension), cx))
            .map(Icon::from_path)
            .map(|icon| icon.color(Color::Muted))
    }
}

impl PickerDelegate for LanguageSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a language…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let language_name = &self.candidates[mat.candidate_id].string;
            let language = self.language_registry.language_for_name(language_name);
            let project = self.project.downgrade();
            let buffer = self.buffer.downgrade();
            cx.spawn_in(window, async move |_, cx| {
                let language = language.await?;
                let project = project.upgrade().context("project was dropped")?;
                let buffer = buffer.upgrade().context("buffer was dropped")?;
                project.update(cx, |project, cx| {
                    project.set_language_for_buffer(&buffer, language, cx);
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.language_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
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
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn_in(window, async move |this, cx| {
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
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
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
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;
        let (label, language_icon) = self.language_data_for_match(mat, cx);
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot::<Icon>(language_icon)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}
