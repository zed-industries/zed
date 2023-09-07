mod active_buffer_language;

pub use active_buffer_language::ActiveBufferLanguage;
use anyhow::anyhow;
use editor::Editor;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{actions, elements::*, AppContext, ModelHandle, MouseState, ViewContext};
use language::{Buffer, LanguageRegistry};
use picker::{Picker, PickerDelegate, PickerEvent};
use project::Project;
use std::sync::Arc;
use util::ResultExt;
use workspace::Workspace;

actions!(language_selector, [Toggle]);

pub fn init(cx: &mut AppContext) {
    Picker::<LanguageSelectorDelegate>::init(cx);
    cx.add_action(toggle);
}

pub fn toggle(
    workspace: &mut Workspace,
    _: &Toggle,
    cx: &mut ViewContext<Workspace>,
) -> Option<()> {
    let (_, buffer, _) = workspace
        .active_item(cx)?
        .act_as::<Editor>(cx)?
        .read(cx)
        .active_excerpt(cx)?;
    workspace.toggle_modal(cx, |workspace, cx| {
        let registry = workspace.app_state().languages.clone();
        cx.add_view(|cx| {
            Picker::new(
                LanguageSelectorDelegate::new(buffer, workspace.project().clone(), registry),
                cx,
            )
        })
    });
    Some(())
}

pub struct LanguageSelectorDelegate {
    buffer: ModelHandle<Buffer>,
    project: ModelHandle<Project>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl LanguageSelectorDelegate {
    fn new(
        buffer: ModelHandle<Buffer>,
        project: ModelHandle<Project>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let candidates = language_registry
            .language_names()
            .into_iter()
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name))
            .collect::<Vec<_>>();
        let mut matches = candidates
            .iter()
            .map(|candidate| StringMatch {
                candidate_id: candidate.id,
                score: 0.,
                positions: Default::default(),
                string: candidate.string.clone(),
            })
            .collect::<Vec<_>>();
        matches.sort_unstable_by(|mat1, mat2| mat1.string.cmp(&mat2.string));

        Self {
            buffer,
            project,
            language_registry,
            candidates,
            matches,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for LanguageSelectorDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Select a language...".into()
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
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("project was dropped"))?;
                let buffer = buffer
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("buffer was dropped"))?;
                project.update(&mut cx, |project, cx| {
                    project.set_language_for_buffer(&buffer, language, cx);
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        cx.emit(PickerEvent::Dismiss);
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

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
        let background = cx.background().clone();
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
                let delegate = this.delegate_mut();
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
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = theme::current(cx);
        let mat = &self.matches[ix];
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);
        let buffer_language_name = self.buffer.read(cx).language().map(|l| l.name());
        let mut label = mat.string.clone();
        if buffer_language_name.as_deref() == Some(mat.string.as_str()) {
            label.push_str(" (current)");
        }

        Label::new(label, style.label.clone())
            .with_highlights(mat.positions.clone())
            .contained()
            .with_style(style.container)
            .into_any()
    }
}
