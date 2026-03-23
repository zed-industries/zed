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
    Appearance, SystemAppearance, Theme, ThemeAppearanceMode, ThemeMeta, ThemeName, ThemeRegistry,
    ThemeSelection, ThemeSettings,
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
    /// The current system appearance.
    original_system_appearance: Appearance,
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
        let original_system_appearance = SystemAppearance::global(cx).0;

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
            original_system_appearance,
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
        // Update the global (in-memory) theme settings.
        SettingsStore::update_global(cx, |store, _| {
            override_global_theme(
                store,
                &new_theme,
                &self.original_theme_settings.theme,
                self.original_system_appearance,
            )
        });

        self.new_theme = new_theme;
    }
}

/// Overrides the global (in-memory) theme settings.
///
/// Note that this does **not** update the user's `settings.json` file (see the
/// [`ThemeSelectorDelegate::confirm`] method and [`theme::set_theme`] function).
fn override_global_theme(
    store: &mut SettingsStore,
    new_theme: &Theme,
    original_theme: &ThemeSelection,
    system_appearance: Appearance,
) {
    let theme_name = ThemeName(new_theme.name.clone().into());
    let new_appearance = new_theme.appearance();
    let new_theme_is_light = new_appearance.is_light();

    let mut curr_theme_settings = store.get::<ThemeSettings>(None).clone();

    match (original_theme, &curr_theme_settings.theme) {
        // Override the currently selected static theme.
        (ThemeSelection::Static(_), ThemeSelection::Static(_)) => {
            curr_theme_settings.theme = ThemeSelection::Static(theme_name);
        }

        // If the current theme selection is dynamic, then only override the global setting for the
        // specific mode (light or dark).
        (
            ThemeSelection::Dynamic {
                mode: original_mode,
                light: original_light,
                dark: original_dark,
            },
            ThemeSelection::Dynamic { .. },
        ) => {
            let new_mode = update_mode_if_new_appearance_is_different_from_system(
                original_mode,
                system_appearance,
                new_appearance,
            );

            let updated_theme = retain_original_opposing_theme(
                new_theme_is_light,
                new_mode,
                theme_name,
                original_light,
                original_dark,
            );

            curr_theme_settings.theme = updated_theme;
        }

        // The theme selection mode changed while selecting new themes (someone edited the settings
        // file on disk while we had the dialogue open), so don't do anything.
        _ => return,
    };

    store.override_global(curr_theme_settings);
}

/// Helper function for determining the new [`ThemeAppearanceMode`] for the new theme.
///
/// If the the original theme mode was [`System`] and the new theme's appearance matches the system
/// appearance, we don't need to change the mode setting.
///
/// Otherwise, we need to change the mode in order to see the new theme.
///
/// [`System`]: ThemeAppearanceMode::System
fn update_mode_if_new_appearance_is_different_from_system(
    original_mode: &ThemeAppearanceMode,
    system_appearance: Appearance,
    new_appearance: Appearance,
) -> ThemeAppearanceMode {
    if original_mode == &ThemeAppearanceMode::System && system_appearance == new_appearance {
        ThemeAppearanceMode::System
    } else {
        ThemeAppearanceMode::from(new_appearance)
    }
}

/// Helper function for updating / displaying the [`ThemeSelection`] while using the theme selector.
///
/// We want to retain the alternate theme selection of the original settings (before the menu was
/// opened), not the currently selected theme (which likely has changed multiple times while the
/// menu has been open).
fn retain_original_opposing_theme(
    new_theme_is_light: bool,
    new_mode: ThemeAppearanceMode,
    theme_name: ThemeName,
    original_light: &ThemeName,
    original_dark: &ThemeName,
) -> ThemeSelection {
    if new_theme_is_light {
        ThemeSelection::Dynamic {
            mode: new_mode,
            light: theme_name,
            dark: original_dark.clone(),
        }
    } else {
        ThemeSelection::Dynamic {
            mode: new_mode,
            light: original_light.clone(),
            dark: theme_name,
        }
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

        let theme_name: Arc<str> = self.new_theme.name.as_str().into();
        let theme_appearance = self.new_theme.appearance;
        let system_appearance = SystemAppearance::global(cx).0;

        telemetry::event!("Settings Changed", setting = "theme", value = theme_name);

        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            theme::set_theme(settings, theme_name, theme_appearance, system_appearance);
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
