mod icon_theme_selector;

use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, UpdateGlobal, WeakEntity,
    Window, actions,
};
use picker::{Picker, PickerDelegate};
use settings::{SettingsStore, update_settings_file};
use std::sync::Arc;
use theme::{Appearance, Theme, ThemeMeta, ThemeRegistry, ThemeSettings};
use ui::{ListItem, ListItemSpacing, prelude::*, v_flex};
use util::ResultExt;
use workspace::{ModalView, Workspace, ui::HighlightedLabel};
use zed_actions::{ExtensionCategoryFilter, Extensions};

use crate::icon_theme_selector::{IconThemeSelector, IconThemeSelectorDelegate};

actions!(theme_selector, [Reload]);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(toggle_theme_selector)
                .register_action(toggle_icon_theme_selector);
        },
    )
    .detach();
}

fn toggle_theme_selector(
    workspace: &mut Workspace,
    toggle: &zed_actions::theme_selector::Toggle,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let fs = workspace.app_state().fs.clone();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = ThemeSelectorDelegate::new(
            cx.entity().downgrade(),
            fs,
            toggle.themes_filter.as_ref(),
            cx,
        );
        ThemeSelector::new(delegate, window, cx)
    });
}

fn toggle_icon_theme_selector(
    workspace: &mut Workspace,
    toggle: &zed_actions::icon_theme_selector::Toggle,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let fs = workspace.app_state().fs.clone();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = IconThemeSelectorDelegate::new(
            cx.entity().downgrade(),
            fs,
            toggle.themes_filter.as_ref(),
            cx,
        );
        IconThemeSelector::new(delegate, window, cx)
    });
}

impl ModalView for ThemeSelector {}

struct ThemeSelector {
    picker: Entity<Picker<ThemeSelectorDelegate>>,
}

impl EventEmitter<DismissEvent> for ThemeSelector {}

impl Focusable for ThemeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ThemeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl ThemeSelector {
    pub fn new(
        delegate: ThemeSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

struct ThemeSelectorDelegate {
    fs: Arc<dyn Fs>,
    themes: Vec<ThemeMeta>,
    matches: Vec<StringMatch>,
    original_theme: Arc<Theme>,
    selection_completed: bool,
    selected_theme: Option<Arc<Theme>>,
    selected_index: usize,
    selector: WeakEntity<ThemeSelector>,
}

impl ThemeSelectorDelegate {
    fn new(
        selector: WeakEntity<ThemeSelector>,
        fs: Arc<dyn Fs>,
        themes_filter: Option<&Vec<String>>,
        cx: &mut Context<ThemeSelector>,
    ) -> Self {
        let original_theme = cx.theme().clone();

        let registry = ThemeRegistry::global(cx);
        let mut themes = registry
            .list()
            .into_iter()
            .filter(|meta| {
                if let Some(theme_filter) = themes_filter {
                    theme_filter.contains(&meta.name.to_string())
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

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
            selected_theme: None,
            selection_completed: false,
            selector,
        };

        this.select_if_matching(&original_theme.name);
        this
    }

    fn show_selected_theme(
        &mut self,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) -> Option<Arc<Theme>> {
        self.matches.get(self.selected_index).and_then(|mat| {
            ThemeRegistry::global(cx)
                .get(&mat.string)
                .inspect(|theme| Self::set_theme(theme.clone(), cx))
                .inspect_err(|error| log::error!("error loading theme {}: {}", mat.string, error))
                .ok()
        })
    }

    fn select_if_matching(&mut self, theme_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == theme_name)
            .unwrap_or(self.selected_index);
    }

    fn set_theme(theme: Arc<Theme>, cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            let mut theme_settings = store.get::<ThemeSettings>(None).clone();
            theme_settings.active_theme = theme;
            theme_settings.apply_theme_overrides();
            store.override_global(theme_settings);
            cx.refresh_windows();
        });
    }
}

impl PickerDelegate for ThemeSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select Theme...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(
        &mut self,
        _: bool,
        window: &mut Window,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) {
        self.selection_completed = true;

        let theme_name = cx.theme().name.clone();

        telemetry::event!("Settings Changed", setting = "theme", value = theme_name);

        let appearance = Appearance::from(window.appearance());

        update_settings_file::<ThemeSettings>(self.fs.clone(), cx, move |settings, _| {
            settings.set_theme(theme_name.to_string(), appearance);
        });

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<ThemeSelectorDelegate>>) {
        if !self.selection_completed {
            Self::set_theme(self.original_theme.clone(), cx);
            self.selection_completed = true;
        }

        self.selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.selected_theme = self.show_selected_theme(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .themes
            .iter()
            .enumerate()
            .map(|(id, meta)| StringMatchCandidate::new(id, &meta.name))
            .collect::<Vec<_>>();

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
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                if query.is_empty() && this.delegate.selected_theme.is_none() {
                    this.delegate.selected_index = this
                        .delegate
                        .selected_index
                        .min(this.delegate.matches.len().saturating_sub(1));
                } else if let Some(selected) = this.delegate.selected_theme.as_ref() {
                    this.delegate.selected_index = this
                        .delegate
                        .matches
                        .iter()
                        .enumerate()
                        .find(|(_, mtch)| mtch.string == selected.name)
                        .map(|(ix, _)| ix)
                        .unwrap_or_default();
                } else {
                    this.delegate.selected_index = 0;
                }
                this.delegate.selected_theme = this.delegate.show_selected_theme(cx);
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let theme_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    theme_match.string.clone(),
                    theme_match.positions.clone(),
                )),
        )
    }

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        Some(
            h_flex()
                .p_2()
                .w_full()
                .justify_between()
                .gap_2()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("docs", "View Theme Docs")
                        .icon(IconName::ArrowUpRight)
                        .icon_position(IconPosition::End)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.open_url("https://zed.dev/docs/themes");
                        })),
                )
                .child(
                    Button::new("more-themes", "Install Themes").on_click(cx.listener({
                        move |_, _, window, cx| {
                            window.dispatch_action(
                                Box::new(Extensions {
                                    category_filter: Some(ExtensionCategoryFilter::Themes),
                                }),
                                cx,
                            );
                        }
                    })),
                )
                .into_any_element(),
        )
    }
}
