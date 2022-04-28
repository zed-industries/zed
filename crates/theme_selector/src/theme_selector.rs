use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, AppContext, Element, ElementBox, Entity, MutableAppContext,
    RenderContext, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate};
use settings::Settings;
use std::sync::Arc;
use theme::{Theme, ThemeRegistry};
use workspace::Workspace;

pub struct ThemeSelector {
    registry: Arc<ThemeRegistry>,
    theme_names: Vec<String>,
    matches: Vec<StringMatch>,
    original_theme: Arc<Theme>,
    picker: ViewHandle<Picker<Self>>,
    selection_completed: bool,
    selected_index: usize,
}

actions!(theme_selector, [Toggle, Reload]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ThemeSelector::toggle);
    cx.add_action(ThemeSelector::reload);
    Picker::<ThemeSelector>::init(cx);
}

pub enum Event {
    Dismissed,
}

impl ThemeSelector {
    fn new(registry: Arc<ThemeRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        let picker = cx.add_view(|cx| Picker::new(handle, cx));
        let original_theme = cx.global::<Settings>().theme.clone();
        let mut theme_names = registry.list().collect::<Vec<_>>();
        theme_names.sort_unstable_by(|a, b| {
            a.ends_with("dark")
                .cmp(&b.ends_with("dark"))
                .then_with(|| a.cmp(&b))
        });
        let matches = theme_names
            .iter()
            .map(|name| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: name.clone(),
            })
            .collect();
        let mut this = Self {
            registry,
            theme_names,
            matches,
            picker,
            original_theme: original_theme.clone(),
            selected_index: 0,
            selection_completed: false,
        };
        this.select_if_matching(&original_theme.name);
        this
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let themes = workspace.themes();
        workspace.toggle_modal(cx, |cx, _| {
            let this = cx.add_view(|cx| Self::new(themes, cx));
            cx.subscribe(&this, Self::on_event).detach();
            this
        });
    }

    fn reload(workspace: &mut Workspace, _: &Reload, cx: &mut ViewContext<Workspace>) {
        let current_theme_name = cx.global::<Settings>().theme.name.clone();
        let themes = workspace.themes();
        themes.clear();
        match themes.get(&current_theme_name) {
            Ok(theme) => {
                Self::set_theme(theme, cx);
                log::info!("reloaded theme {}", current_theme_name);
            }
            Err(error) => {
                log::error!("failed to load theme {}: {:?}", current_theme_name, error)
            }
        }
    }

    fn show_selected_theme(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            match self.registry.get(&mat.string) {
                Ok(theme) => Self::set_theme(theme, cx),
                Err(error) => {
                    log::error!("error loading theme {}: {}", mat.string, error)
                }
            }
        }
    }

    fn select_if_matching(&mut self, theme_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == theme_name)
            .unwrap_or(self.selected_index);
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<ThemeSelector>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }

    fn set_theme(theme: Arc<Theme>, cx: &mut MutableAppContext) {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.theme = theme;
            cx.refresh_windows();
        });
    }
}

impl PickerDelegate for ThemeSelector {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        self.selection_completed = true;
        cx.emit(Event::Dismissed);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        if !self.selection_completed {
            Self::set_theme(self.original_theme.clone(), cx);
            self.selection_completed = true;
        }
        cx.emit(Event::Dismissed);
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>) {
        self.selected_index = ix;
        self.show_selected_theme(cx);
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> gpui::Task<()> {
        let background = cx.background().clone();
        let candidates = self
            .theme_names
            .iter()
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
                this.show_selected_theme(cx);
                cx.notify();
            });
        })
    }

    fn render_match(&self, ix: usize, selected: bool, cx: &AppContext) -> ElementBox {
        let settings = cx.global::<Settings>();
        let theme = &settings.theme;
        let theme_match = &self.matches[ix];
        let style = if selected {
            &theme.picker.active_item
        } else {
            &theme.picker.item
        };

        Label::new(theme_match.string.clone(), style.label.clone())
            .with_highlights(theme_match.positions.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}

impl Entity for ThemeSelector {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        if !self.selection_completed {
            Self::set_theme(self.original_theme.clone(), cx);
        }
    }
}

impl View for ThemeSelector {
    fn ui_name() -> &'static str {
        "ThemeSelector"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.picker);
    }
}
