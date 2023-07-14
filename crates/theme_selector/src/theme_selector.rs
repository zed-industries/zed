use fs::Fs;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{actions, elements::*, AnyElement, AppContext, Element, MouseState, ViewContext};
use picker::{Picker, PickerDelegate, PickerEvent};
use settings::{update_settings_file, SettingsStore};
use staff_mode::StaffMode;
use std::sync::Arc;
use theme::{Theme, ThemeMeta, ThemeRegistry, ThemeSettings};
use util::ResultExt;
use workspace::Workspace;

actions!(theme_selector, [Toggle, Reload]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle);
    ThemeSelector::init(cx);
}

pub fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    workspace.toggle_modal(cx, |workspace, cx| {
        let fs = workspace.app_state().fs.clone();
        cx.add_view(|cx| ThemeSelector::new(ThemeSelectorDelegate::new(fs, cx), cx))
    });
}

#[cfg(debug_assertions)]
pub fn reload(cx: &mut AppContext) {
    let current_theme_name = theme::current(cx).meta.name.clone();
    let registry = cx.global::<Arc<ThemeRegistry>>();
    registry.clear();
    match registry.get(&current_theme_name) {
        Ok(theme) => {
            ThemeSelectorDelegate::set_theme(theme, cx);
            log::info!("reloaded theme {}", current_theme_name);
        }
        Err(error) => {
            log::error!("failed to load theme {}: {:?}", current_theme_name, error)
        }
    }
}

pub type ThemeSelector = Picker<ThemeSelectorDelegate>;

pub struct ThemeSelectorDelegate {
    fs: Arc<dyn Fs>,
    theme_data: Vec<ThemeMeta>,
    matches: Vec<StringMatch>,
    original_theme: Arc<Theme>,
    selection_completed: bool,
    selected_index: usize,
}

impl ThemeSelectorDelegate {
    fn new(fs: Arc<dyn Fs>, cx: &mut ViewContext<ThemeSelector>) -> Self {
        let original_theme = theme::current(cx).clone();

        let staff_mode = **cx.default_global::<StaffMode>();
        let registry = cx.global::<Arc<ThemeRegistry>>();
        let mut theme_names = registry.list(staff_mode).collect::<Vec<_>>();
        theme_names.sort_unstable_by(|a, b| a.is_light.cmp(&b.is_light).then(a.name.cmp(&b.name)));
        let matches = theme_names
            .iter()
            .map(|meta| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: meta.name.clone(),
            })
            .collect();
        let mut this = Self {
            fs,
            theme_data: theme_names,
            matches,
            original_theme: original_theme.clone(),
            selected_index: 0,
            selection_completed: false,
        };
        this.select_if_matching(&original_theme.meta.name);
        this
    }

    fn show_selected_theme(&mut self, cx: &mut ViewContext<ThemeSelector>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let registry = cx.global::<Arc<ThemeRegistry>>();
            match registry.get(&mat.string) {
                Ok(theme) => {
                    Self::set_theme(theme, cx);
                }
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

    fn set_theme(theme: Arc<Theme>, cx: &mut AppContext) {
        cx.update_global::<SettingsStore, _, _>(|store, cx| {
            let mut theme_settings = store.get::<ThemeSettings>(None).clone();
            theme_settings.theme = theme;
            store.override_global(theme_settings);
            cx.refresh_windows();
        });
    }
}

impl PickerDelegate for ThemeSelectorDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Select Theme...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<ThemeSelector>) {
        self.selection_completed = true;

        let theme_name = theme::current(cx).meta.name.clone();
        update_settings_file::<ThemeSettings>(self.fs.clone(), cx, |settings| {
            settings.theme = Some(theme_name);
        });

        cx.emit(PickerEvent::Dismiss);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<ThemeSelector>) {
        if !self.selection_completed {
            Self::set_theme(self.original_theme.clone(), cx);
            self.selection_completed = true;
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<ThemeSelector>) {
        self.selected_index = ix;
        self.show_selected_theme(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<ThemeSelector>,
    ) -> gpui::Task<()> {
        let background = cx.background().clone();
        let candidates = self
            .theme_data
            .iter()
            .enumerate()
            .map(|(id, meta)| StringMatchCandidate {
                id,
                char_bag: meta.name.as_str().into(),
                string: meta.name.clone(),
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
                let delegate = this.delegate_mut();
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                delegate.show_selected_theme(cx);
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
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);

        let theme_match = &self.matches[ix];
        Label::new(theme_match.string.clone(), style.label.clone())
            .with_highlights(theme_match.positions.clone())
            .contained()
            .with_style(style.container)
            .into_any()
    }
}
