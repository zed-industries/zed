mod icon_theme_selector;

use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, UpdateGlobal, WeakEntity,
    Window, actions,
};
use picker::{Picker, PickerDelegate};
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::Arc;
use theme::{
    Theme, ThemeAppearanceMode, ThemeMeta, ThemeName, ThemeRegistry, ThemeSelection, ThemeSettings,
};
use ui::{ListItem, ListItemSpacing, prelude::*, v_flex};
use util::ResultExt;
use workspace::{ModalView, Workspace, ui::HighlightedLabel, with_active_or_new_workspace};
use zed_actions::{ExtensionCategoryFilter, Extensions};

use crate::icon_theme_selector::{IconThemeSelector, IconThemeSelectorDelegate};

actions!(
    theme_selector,
    [
        /// Reloads all themes from disk.
        Reload
    ]
);

pub fn init(cx: &mut App) {
    cx.on_action(|action: &zed_actions::theme_selector::Toggle, cx| {
        let action = action.clone();
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            toggle_theme_selector(workspace, &action, window, cx);
        });
    });
    cx.on_action(|action: &zed_actions::icon_theme_selector::Toggle, cx| {
        let action = action.clone();
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            toggle_icon_theme_selector(workspace, &action, window, cx);
        });
    });
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
        v_flex()
            .key_context("ThemeSelector")
            .w(rems(34.))
            .child(self.picker.clone())
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
    /// The theme that was selected before the `ThemeSelector` menu was opened.
    ///
    /// We use this to return back to theme that was set if the user dismisses the menu.
    original_theme_settings: ThemeSettings,
    /// The currently selected new theme.
    new_theme: Arc<Theme>,
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
        let original_theme_settings = ThemeSettings::get_global(cx).clone();

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

        // Sort by dark vs light, then by name.
        themes.sort_unstable_by(|a, b| {
            a.appearance
                .is_light()
                .cmp(&b.appearance.is_light())
                .then(a.name.cmp(&b.name))
        });

        let matches: Vec<StringMatch> = themes
            .iter()
            .map(|meta| StringMatch {
                candidate_id: 0,
                score: 0.0,
                positions: Default::default(),
                string: meta.name.to_string(),
            })
            .collect();

        // The current theme is likely in this list, so default to first showing that.
        let selected_index = matches
            .iter()
            .position(|mat| mat.string == original_theme.name)
            .unwrap_or(0);

        Self {
            fs,
            themes,
            matches,
            original_theme_settings,
            new_theme: original_theme, // Start with the original theme.
            selected_index,
            selection_completed: false,
            selected_theme: None,
            selector,
        }
    }

    fn show_selected_theme(
        &mut self,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) -> Option<Arc<Theme>> {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let registry = ThemeRegistry::global(cx);

            match registry.get(&mat.string) {
                Ok(theme) => {
                    self.set_theme(theme.clone(), cx);
                    Some(theme)
                }
                Err(error) => {
                    log::error!("error loading theme {}: {}", mat.string, error);
                    None
                }
            }
        } else {
            None
        }
    }

    fn set_theme(&mut self, new_theme: Arc<Theme>, cx: &mut App) {
        let theme_name = ThemeName(new_theme.as_ref().name.clone().into());
        let new_theme_is_light = new_theme.appearance().is_light();

        SettingsStore::update_global(cx, |store, _| {
            let mut curr_theme_settings = store.get::<ThemeSettings>(None).clone();

            match (
                &curr_theme_settings.theme,
                &self.original_theme_settings.theme,
            ) {
                // Override the currently selected static theme.
                (ThemeSelection::Static(_), ThemeSelection::Static(_)) => {
                    curr_theme_settings.theme = ThemeSelection::Static(theme_name);
                }
                // If the current theme selection is dynamic, then only override the global setting
                // for the specific mode (light or dark).
                (
                    ThemeSelection::Dynamic { .. },
                    ThemeSelection::Dynamic {
                        light: original_light,
                        dark: original_dark,
                        ..
                    },
                ) => {
                    // Note that we want to retain the alternate theme selection of the original
                    // settings (before the menu was opened), not the currently selected theme
                    // (which likely has changed multiple times while the menu has been open).

                    curr_theme_settings.theme = if new_theme_is_light {
                        ThemeSelection::Dynamic {
                            // Force the appearance mode to change so the new theme is shown
                            // immediately, regardless of the system appearance setting.
                            mode: ThemeAppearanceMode::Light,
                            light: theme_name,
                            dark: original_dark.clone(),
                        }
                    } else {
                        ThemeSelection::Dynamic {
                            mode: ThemeAppearanceMode::Dark,
                            light: original_light.clone(),
                            dark: theme_name,
                        }
                    }
                }
                _ => unreachable!(
                    "The theme selection mode somehow changed while selecting new themes"
                ),
            };

            store.override_global(curr_theme_settings);
        });

        self.new_theme = new_theme;
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
        _secondary: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<ThemeSelectorDelegate>>,
    ) {
        self.selection_completed = true;

        let theme_appearance = self.new_theme.appearance;
        let theme_name: Arc<str> = self.new_theme.name.as_str().into();

        telemetry::event!("Settings Changed", setting = "theme", value = theme_name);

        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            theme::set_theme(settings, theme_name, theme_appearance);
        });

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<ThemeSelectorDelegate>>) {
        if !self.selection_completed {
            SettingsStore::update_global(cx, |store, _| {
                store.override_global(self.original_theme_settings.clone());
            });
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
                    true,
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
        let theme_match = &self.matches.get(ix)?;

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
                        .icon_size(IconSize::Small)
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
                                    id: None,
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
