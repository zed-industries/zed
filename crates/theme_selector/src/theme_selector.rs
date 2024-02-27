use client::telemetry::Telemetry;
use feature_flags::FeatureFlagAppExt;
use fs::Fs;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusableView, Render, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use settings::{update_settings_file, SettingsStore};
use std::sync::Arc;
use theme::{
    Appearance, Theme, ThemeMeta, ThemeMode, ThemeRegistry, ThemeSelection, ThemeSettings,
};
use ui::{prelude::*, v_flex, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ui::HighlightedLabel, ModalView, Workspace};

actions!(theme_selector, [Toggle, Reload]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(toggle);
        },
    )
    .detach();
}

pub fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    let fs = workspace.app_state().fs.clone();
    let telemetry = workspace.client().telemetry().clone();
    workspace.toggle_modal(cx, |cx| {
        ThemeSelector::new(
            ThemeSelectorDelegate::new(cx.view().downgrade(), fs, telemetry, cx),
            cx,
        )
    });
}

impl ModalView for ThemeSelector {}

pub struct ThemeSelector {
    picker: View<Picker<ThemeSelectorDelegate>>,
}

impl EventEmitter<DismissEvent> for ThemeSelector {}

impl FocusableView for ThemeSelector {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ThemeSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl ThemeSelector {
    pub fn new(delegate: ThemeSelectorDelegate, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
        Self { picker }
    }
}

pub struct ThemeSelectorDelegate {
    fs: Arc<dyn Fs>,
    themes: Vec<ThemeMeta>,
    matches: Vec<StringMatch>,
    original_theme: Arc<Theme>,
    selection_completed: bool,
    selected_index: usize,
    telemetry: Arc<Telemetry>,
    view: WeakView<ThemeSelector>,
}

impl ThemeSelectorDelegate {
    fn new(
        weak_view: WeakView<ThemeSelector>,
        fs: Arc<dyn Fs>,
        telemetry: Arc<Telemetry>,
        cx: &mut ViewContext<ThemeSelector>,
    ) -> Self {
        let original_theme = cx.theme().clone();

        let staff_mode = cx.is_staff();
        let registry = ThemeRegistry::global(cx);
        let mut themes = registry.list(staff_mode);
        themes.sort_unstable_by(|a, b| {
            a.appearance
                .is_light()
                .cmp(&b.appearance.is_light())
                .then(a.name.cmp(&b.name))
        });
        let matches = themes
            .iter()
            .map(|meta| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: meta.name.to_string(),
            })
            .collect();
        let mut this = Self {
            fs,
            themes,
            matches,
            original_theme: original_theme.clone(),
            selected_index: 0,
            selection_completed: false,
            telemetry,
            view: weak_view,
        };
        this.select_if_matching(&original_theme.name);
        this
    }

    fn show_selected_theme(&mut self, cx: &mut ViewContext<Picker<ThemeSelectorDelegate>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let registry = ThemeRegistry::global(cx);
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
        cx.update_global(|store: &mut SettingsStore, cx| {
            let mut theme_settings = store.get::<ThemeSettings>(None).clone();
            theme_settings.active_theme = theme;
            theme_settings.apply_theme_overrides();
            store.override_global(theme_settings);
            cx.refresh();
        });
    }
}

impl PickerDelegate for ThemeSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select Theme...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<ThemeSelectorDelegate>>) {
        self.selection_completed = true;

        let theme_name = cx.theme().name.clone();

        self.telemetry
            .report_setting_event("theme", theme_name.to_string());

        let appearance = Appearance::from(cx.appearance());

        update_settings_file::<ThemeSettings>(self.fs.clone(), cx, move |settings| {
            if let Some(selection) = settings.theme.as_mut() {
                let theme_to_update = match selection {
                    ThemeSelection::Static(theme) => theme,
                    ThemeSelection::Dynamic { mode, light, dark } => match mode {
                        ThemeMode::Light => light,
                        ThemeMode::Dark => dark,
                        ThemeMode::System => match appearance {
                            Appearance::Light => light,
                            Appearance::Dark => dark,
                        },
                    },
                };

                *theme_to_update = theme_name.to_string();
            } else {
                settings.theme = Some(ThemeSelection::Static(theme_name.to_string()));
            }
        });

        self.view
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<ThemeSelectorDelegate>>) {
        if !self.selection_completed {
            Self::set_theme(self.original_theme.clone(), cx);
            self.selection_completed = true;
        }

        self.view
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        cx: &mut ViewContext<Picker<ThemeSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.show_selected_theme(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<ThemeSelectorDelegate>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .themes
            .iter()
            .enumerate()
            .map(|(id, meta)| StringMatchCandidate {
                id,
                char_bag: meta.name.as_ref().into(),
                string: meta.name.to_string(),
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
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                this.delegate.show_selected_theme(cx);
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let theme_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(
                    theme_match.string.clone(),
                    theme_match.positions.clone(),
                )),
        )
    }
}
