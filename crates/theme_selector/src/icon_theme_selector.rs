use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, ModifiersChangedEvent, Render,
    UpdateGlobal, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::sync::Arc;
use theme::{Appearance, SystemAppearance, ThemeMeta, ThemeRegistry};
use theme_settings::{IconThemeName, IconThemeSelection, ThemeSettings};
use ui::{ListItem, ListItemSpacing, prelude::*, v_flex};
use util::ResultExt;
use workspace::{ModalView, ui::HighlightedLabel};
use zed_actions::{ExtensionCategoryFilter, Extensions};

use crate::render_compare_button;

pub(crate) struct IconThemeSelector {
    picker: Entity<Picker<IconThemeSelectorDelegate>>,
}

impl EventEmitter<DismissEvent> for IconThemeSelector {}

impl Focusable for IconThemeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ModalView for IconThemeSelector {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.revert_theme(cx);
        });
        workspace::DismissDecision::Dismiss(true)
    }
}

impl IconThemeSelector {
    pub fn new(
        delegate: IconThemeSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .set_showing_original_theme(event.modifiers.alt, cx);
        });
    }
}

impl Render for IconThemeSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("IconThemeSelector")
            .w(rems(34.))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
    }
}

pub(crate) struct IconThemeSelectorDelegate {
    fs: Arc<dyn Fs>,
    themes: Vec<ThemeMeta>,
    matches: Vec<StringMatch>,
    original_theme: IconThemeName,
    /// Whether the original icon theme is temporarily being shown instead of
    /// the currently selected one.
    showing_original_theme: bool,
    selection_completed: bool,
    selected_theme: Option<IconThemeName>,
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
        let original_theme = theme_settings
            .icon_theme
            .name(SystemAppearance::global(cx).0);

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
            showing_original_theme: false,
            selected_index: 0,
            selected_theme: None,
            selection_completed: false,
            selector,
        };

        this.select_if_matching(&original_theme.0);
        this
    }

    fn show_selected_theme(
        &mut self,
        cx: &mut Context<Picker<IconThemeSelectorDelegate>>,
    ) -> Option<IconThemeName> {
        let mat = self.matches.get(self.selected_index)?;
        let name = IconThemeName(mat.string.clone().into());
        if !self.showing_original_theme {
            Self::set_icon_theme(name.clone(), cx);
        }
        Some(name)
    }

    fn is_original_theme(&self, index: usize) -> bool {
        self.matches
            .get(index)
            .is_some_and(|mat| mat.string.as_str() == self.original_theme.0.as_ref())
    }

    fn set_showing_original_theme(
        &mut self,
        showing_original_theme: bool,
        cx: &mut Context<Picker<IconThemeSelectorDelegate>>,
    ) {
        if self.selection_completed || self.showing_original_theme == showing_original_theme {
            return;
        }

        self.showing_original_theme = showing_original_theme;

        let theme = if showing_original_theme {
            Some(self.original_theme.clone())
        } else {
            self.selected_theme.clone()
        };

        if let Some(theme) = theme {
            Self::set_icon_theme(theme, cx);
        }

        cx.notify();
    }

    fn select_if_matching(&mut self, theme_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == theme_name)
            .unwrap_or(self.selected_index);
    }

    fn revert_theme(&mut self, cx: &mut App) {
        if !self.selection_completed {
            Self::set_icon_theme(self.original_theme.clone(), cx);
            self.selection_completed = true;
        }
    }

    fn set_icon_theme(name: IconThemeName, cx: &mut App) {
        SettingsStore::update_global(cx, |store, _| {
            let mut theme_settings = store.get::<ThemeSettings>(None).clone();
            theme_settings.icon_theme = IconThemeSelection::Static(name);
            store.override_global(theme_settings);
        });
    }
}

impl PickerDelegate for IconThemeSelectorDelegate {
    type ListItem = ui::ListItem;

    fn name() -> &'static str {
        "icon theme selector"
    }

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

        // The global settings can't be relied upon here, as they hold the original icon theme
        // while it is temporarily being shown.
        let theme_name = self.selected_theme.clone().unwrap_or_else(|| {
            ThemeSettings::get_global(cx)
                .icon_theme
                .name(SystemAppearance::global(cx).0)
        });

        telemetry::event!(
            "Settings Changed",
            setting = "icon_theme",
            value = theme_name
        );

        let appearance = Appearance::from(window.appearance());

        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            theme_settings::set_icon_theme(settings, theme_name, appearance);
        });

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<IconThemeSelectorDelegate>>) {
        self.revert_theme(cx);

        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
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
        self.selected_theme = self.show_selected_theme(cx);
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
                        .find(|(_, mtch)| mtch.string.as_str() == selected.0.as_ref())
                        .map(|(ix, _)| ix)
                        .unwrap_or_default();
                } else {
                    this.delegate.selected_index = 0;
                }
                // Preserve the previously selected theme when the filter yields no results.
                if let Some(theme) = this.delegate.show_selected_theme(cx) {
                    this.delegate.selected_theme = Some(theme);
                }
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
        let is_original_theme = self.is_original_theme(ix);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    theme_match.string.clone(),
                    theme_match.positions.clone(),
                ))
                .when(is_original_theme, |this| {
                    this.end_slot(Icon::new(IconName::Check).color(Color::Muted))
                }),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        // The button is kept while the original icon theme is being shown, even if the selection
        // moves to the original icon theme, so that releasing the button is still handled.
        let show_compare_button =
            self.showing_original_theme || !self.is_original_theme(self.selected_index);
        let compare_button = show_compare_button.then(|| {
            let picker = cx.entity().downgrade();
            render_compare_button(
                move |showing_original_theme, cx| {
                    picker
                        .update(cx, |picker, cx| {
                            picker
                                .delegate
                                .set_showing_original_theme(showing_original_theme, cx);
                        })
                        .log_err();
                },
                cx,
            )
        });

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
                        .end_icon(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .on_click(|_event, _window, cx| {
                            cx.open_url("https://zed.dev/docs/icon-themes");
                        }),
                )
                .child(h_flex().gap_2().children(compare_button).child(
                    Button::new("more-icon-themes", "Install Icon Themes").on_click(
                        move |_event, window, cx| {
                            window.dispatch_action(
                                Box::new(Extensions {
                                    category_filter: Some(ExtensionCategoryFilter::IconThemes),
                                    id: None,
                                }),
                                cx,
                            );
                        },
                    ),
                ))
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use gpui::{Modifiers, TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use theme::{ChevronIcons, DirectoryIcons, IconTheme, ThemeRegistry};
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) -> Arc<workspace::AppState> {
        cx.update(|cx| {
            let app_state = workspace::AppState::test(cx);
            settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
            app_state
        })
    }

    fn register_test_icon_themes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let registry = ThemeRegistry::global(cx);
            let make_icon_theme = |name: &str, appearance: Appearance| IconTheme {
                id: name.to_lowercase().replace(' ', "-"),
                name: SharedString::from(name.to_string()),
                appearance,
                directory_icons: DirectoryIcons {
                    collapsed: None,
                    expanded: None,
                },
                named_directory_icons: HashMap::default(),
                chevron_icons: ChevronIcons {
                    collapsed: None,
                    expanded: None,
                },
                file_icons: HashMap::default(),
                file_stems: HashMap::default(),
                file_suffixes: HashMap::default(),
            };
            registry.register_test_icon_themes([
                make_icon_theme("Test Icons A", Appearance::Dark),
                make_icon_theme("Test Icons B", Appearance::Dark),
            ]);
        });
    }

    async fn setup_test(cx: &mut TestAppContext) -> Arc<workspace::AppState> {
        let app_state = init_test(cx);
        register_test_icon_themes(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/test"), json!({}))
            .await;
        app_state
    }

    fn open_icon_theme_selector(
        workspace: &Entity<workspace::Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<IconThemeSelectorDelegate>> {
        cx.dispatch_action(zed_actions::icon_theme_selector::Toggle {
            themes_filter: None,
        });
        cx.run_until_parked();
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<IconThemeSelector>(cx)
                .expect("icon theme selector should be open")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn selected_theme_name(
        picker: &Entity<Picker<IconThemeSelectorDelegate>>,
        cx: &mut VisualTestContext,
    ) -> String {
        picker.read_with(cx, |picker, _| {
            picker
                .delegate
                .matches
                .get(picker.delegate.selected_index)
                .expect("selected index should point to a match")
                .string
                .clone()
        })
    }

    fn previewed_theme_name(
        _picker: &Entity<Picker<IconThemeSelectorDelegate>>,
        cx: &mut VisualTestContext,
    ) -> String {
        cx.read(|cx| {
            ThemeSettings::get_global(cx)
                .icon_theme
                .name(SystemAppearance::global(cx).0)
                .0
                .to_string()
        })
    }

    #[gpui::test]
    async fn test_icon_theme_selector_preserves_selection_on_empty_filter(cx: &mut TestAppContext) {
        let app_state = setup_test(cx).await;
        let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let picker = open_icon_theme_selector(&workspace, cx);

        let target_index = picker.read_with(cx, |picker, _| {
            picker
                .delegate
                .matches
                .iter()
                .position(|m| m.string == "Test Icons A")
                .unwrap()
        });
        picker.update_in(cx, |picker, window, cx| {
            picker.set_selected_index(target_index, None, true, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(previewed_theme_name(&picker, cx), "Test Icons A");

        picker.update_in(cx, |picker, window, cx| {
            picker.update_matches("zzz".to_string(), window, cx);
        });
        cx.run_until_parked();

        picker.update_in(cx, |picker, window, cx| {
            picker.update_matches("".to_string(), window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            selected_theme_name(&picker, cx),
            "Test Icons A",
            "selected icon theme should be preserved after clearing an empty filter"
        );
        assert_eq!(
            previewed_theme_name(&picker, cx),
            "Test Icons A",
            "previewed icon theme should be preserved after clearing an empty filter"
        );
    }

    fn select_theme(
        picker: &Entity<Picker<IconThemeSelectorDelegate>>,
        theme_name: &str,
        cx: &mut VisualTestContext,
    ) {
        let target_index = picker.read_with(cx, |picker, _| {
            picker
                .delegate
                .matches
                .iter()
                .position(|theme_match| theme_match.string == theme_name)
                .expect("icon theme should be in the list of matches")
        });
        picker.update_in(cx, |picker, window, cx| {
            picker.set_selected_index(target_index, None, true, window, cx);
        });
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_icon_theme_selector_shows_original_theme_while_alt_is_held(
        cx: &mut TestAppContext,
    ) {
        let app_state = setup_test(cx).await;
        let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let picker = open_icon_theme_selector(&workspace, cx);
        let original_theme_name =
            picker.read_with(cx, |picker, _| picker.delegate.original_theme.0.to_string());

        select_theme(&picker, "Test Icons A", cx);
        assert_eq!(previewed_theme_name(&picker, cx), "Test Icons A");

        cx.simulate_modifiers_change(Modifiers::alt());
        assert_eq!(
            previewed_theme_name(&picker, cx),
            original_theme_name,
            "original icon theme should be shown while alt is held"
        );

        select_theme(&picker, "Test Icons B", cx);
        assert_eq!(
            previewed_theme_name(&picker, cx),
            original_theme_name,
            "changing the selection while alt is held should keep showing the original icon theme"
        );
        assert_eq!(selected_theme_name(&picker, cx), "Test Icons B");

        cx.simulate_modifiers_change(Modifiers::none());
        assert_eq!(
            previewed_theme_name(&picker, cx),
            "Test Icons B",
            "selected icon theme should be shown again once alt is released"
        );
    }
}
