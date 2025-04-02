use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, UpdateGlobal, WeakEntity,
    Window,
};
use picker::{Picker, PickerDelegate};
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::sync::Arc;
use theme::{Appearance, IconTheme, ThemeMeta, ThemeRegistry, ThemeSettings};
use ui::{ListItem, ListItemSpacing, prelude::*, v_flex};
use util::ResultExt;
use workspace::{ModalView, ui::HighlightedLabel};
use zed_actions::{ExtensionCategoryFilter, Extensions};

pub(crate) struct IconThemeSelector {
    picker: Entity<Picker<IconThemeSelectorDelegate>>,
}

impl EventEmitter<DismissEvent> for IconThemeSelector {}

impl Focusable for IconThemeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ModalView for IconThemeSelector {}

impl IconThemeSelector {
    pub fn new(
        delegate: IconThemeSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for IconThemeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub(crate) struct IconThemeSelectorDelegate {
    fs: Arc<dyn Fs>,
    themes: Vec<ThemeMeta>,
    matches: Vec<StringMatch>,
    original_theme: Arc<IconTheme>,
    selection_completed: bool,
    selected_index: usize,
    selector: WeakEntity<IconThemeSelector>,
}

impl IconThemeSelectorDelegate {
    pub fn new(
        selector: WeakEntity<IconThemeSelector>,
        fs: Arc<dyn Fs>,
        themes_filter: Option<&Vec<String>>,
        cx: &mut Context<IconThemeSelector>,
    ) -> Self {
        let theme_settings = ThemeSettings::get_global(cx);
        let original_theme = theme_settings.active_icon_theme.clone();

        let registry = ThemeRegistry::global(cx);
        let mut themes = registry
            .list_icon_themes()
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
            selection_completed: false,
            selector,
        };

        this.select_if_matching(&original_theme.name);
        this
    }

    fn show_selected_theme(&mut self, cx: &mut Context<Picker<IconThemeSelectorDelegate>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let registry = ThemeRegistry::global(cx);
            match registry.get_icon_theme(&mat.string) {
                Ok(theme) => {
                    Self::set_icon_theme(theme, cx);
                }
                Err(err) => {
                    log::error!("error loading icon theme {}: {err}", mat.string);
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

    fn set_icon_theme(theme: Arc<IconTheme>, cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            let mut theme_settings = store.get::<ThemeSettings>(None).clone();
            theme_settings.active_icon_theme = theme;
            store.override_global(theme_settings);
            cx.refresh_windows();
        });
    }
}

impl PickerDelegate for IconThemeSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select Icon Theme...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(
        &mut self,
        _: bool,
        window: &mut Window,
        cx: &mut Context<Picker<IconThemeSelectorDelegate>>,
    ) {
        self.selection_completed = true;

        let theme_settings = ThemeSettings::get_global(cx);
        let theme_name = theme_settings.active_icon_theme.name.clone();

        telemetry::event!(
            "Settings Changed",
            setting = "icon_theme",
            value = theme_name
        );

        let appearance = Appearance::from(window.appearance());

        update_settings_file::<ThemeSettings>(self.fs.clone(), cx, move |settings, _| {
            settings.set_icon_theme(theme_name.to_string(), appearance);
        });

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<IconThemeSelectorDelegate>>) {
        if !self.selection_completed {
            Self::set_icon_theme(self.original_theme.clone(), cx);
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
        cx: &mut Context<Picker<IconThemeSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.show_selected_theme(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<IconThemeSelectorDelegate>>,
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
        _window: &mut Window,
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
                    Button::new("docs", "View Icon Theme Docs")
                        .icon(IconName::ArrowUpRight)
                        .icon_position(IconPosition::End)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .on_click(|_event, _window, cx| {
                            cx.open_url("https://zed.dev/docs/icon-themes");
                        }),
                )
                .child(
                    Button::new("more-icon-themes", "Install Icon Themes").on_click(
                        move |_event, window, cx| {
                            window.dispatch_action(
                                Box::new(Extensions {
                                    category_filter: Some(ExtensionCategoryFilter::IconThemes),
                                }),
                                cx,
                            );
                        },
                    ),
                )
                .into_any_element(),
        )
    }
}
