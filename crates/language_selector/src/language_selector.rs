use std::sync::Arc;

use editor::Editor;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, ModelHandle, MouseState,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
use language::{Buffer, LanguageRegistry};
use picker::{Picker, PickerDelegate};
use project::Project;
use settings::Settings;
use workspace::{AppState, Workspace};

actions!(language_selector, [Toggle]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    Picker::<LanguageSelector>::init(cx);
    cx.add_action({
        let language_registry = app_state.languages.clone();
        move |workspace, _: &Toggle, cx| {
            LanguageSelector::toggle(workspace, language_registry.clone(), cx)
        }
    });
}

pub enum Event {
    Dismissed,
}

pub struct LanguageSelector {
    buffer: ModelHandle<Buffer>,
    project: ModelHandle<Project>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    picker: ViewHandle<Picker<Self>>,
    selected_index: usize,
}

impl LanguageSelector {
    fn new(
        buffer: ModelHandle<Buffer>,
        project: ModelHandle<Project>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let handle = cx.weak_handle();
        let picker = cx.add_view(|cx| Picker::new("Select Language...", handle, cx));

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
            picker,
            selected_index: 0,
        }
    }

    fn toggle(
        workspace: &mut Workspace,
        registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some((_, buffer, _)) = workspace
            .active_item(cx)
            .and_then(|active_item| active_item.act_as::<Editor>(cx))
            .and_then(|editor| editor.read(cx).active_excerpt(cx))
        {
            workspace.toggle_modal(cx, |workspace, cx| {
                let project = workspace.project().clone();
                let this = cx.add_view(|cx| Self::new(buffer, project, registry, cx));
                cx.subscribe(&this, Self::on_event).detach();
                this
            });
        }
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<LanguageSelector>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }
}

impl Entity for LanguageSelector {
    type Event = Event;
}

impl View for LanguageSelector {
    fn ui_name() -> &'static str {
        "LanguageSelector"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone(), cx).boxed()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.picker);
        }
    }
}

impl PickerDelegate for LanguageSelector {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let language_name = &self.candidates[mat.candidate_id].string;
            let language = self.language_registry.language_for_name(language_name);
            cx.spawn(|this, mut cx| async move {
                let language = language.await?;
                this.update(&mut cx, |this, cx| {
                    this.project.update(cx, |project, cx| {
                        project.set_language_for_buffer(&this.buffer, language, cx);
                    });
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        cx.emit(Event::Dismissed);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Self>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> gpui::Task<()> {
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
                this.matches = matches;
                this.selected_index = this
                    .selected_index
                    .min(this.matches.len().saturating_sub(1));
                cx.notify();
            });
        })
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> ElementBox {
        let settings = cx.global::<Settings>();
        let theme = &settings.theme;
        let mat = &self.matches[ix];
        let style = theme.picker.item.style_for(mouse_state, selected);
        let buffer_language_name = self.buffer.read(cx).language().map(|l| l.name());
        let mut label = mat.string.clone();
        if buffer_language_name.as_deref() == Some(mat.string.as_str()) {
            label.push_str(" (current)");
        }

        Label::new(label, style.label.clone())
            .with_highlights(mat.positions.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
