use std::sync::Arc;

use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, MouseState, MutableAppContext,
    RenderContext, View, ViewContext, ViewHandle,
};
use language::LanguageRegistry;
use picker::{Picker, PickerDelegate};
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
    language_registry: Arc<LanguageRegistry>,
    matches: Vec<StringMatch>,
    picker: ViewHandle<Picker<Self>>,
    selected_index: usize,
}

impl LanguageSelector {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        let picker = cx.add_view(|cx| Picker::new("Select Language...", handle, cx));

        let mut matches = language_registry
            .language_names()
            .into_iter()
            .enumerate()
            .map(|(candidate_id, name)| StringMatch {
                candidate_id,
                score: 0.0,
                positions: Default::default(),
                string: name,
            })
            .collect::<Vec<_>>();
        matches.sort_unstable_by(|mat1, mat2| mat1.string.cmp(&mat2.string));

        Self {
            language_registry,
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
        workspace.toggle_modal(cx, |_, cx| {
            let this = cx.add_view(|cx| Self::new(registry, cx));
            cx.subscribe(&this, Self::on_event).detach();
            this
        });
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
        todo!();
        cx.emit(Event::Dismissed);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> gpui::Task<()> {
        let background = cx.background().clone();
        let candidates = self
            .language_registry
            .language_names()
            .into_iter()
            .enumerate()
            .map(|(id, name)| StringMatchCandidate {
                id,
                char_bag: name.as_str().into(),
                string: name.clone(),
            })
            .collect::<Vec<_>>();

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
        let theme_match = &self.matches[ix];
        let style = theme.picker.item.style_for(mouse_state, selected);

        Label::new(theme_match.string.clone(), style.label.clone())
            .with_highlights(theme_match.positions.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
