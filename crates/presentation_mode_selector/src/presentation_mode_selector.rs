use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, UpdateGlobal,
    WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use settings::{Settings, SettingsStore};
use theme::{ThemeRegistry, ThemeSettings};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};
pub mod presentation_mode_selector_button;
use presentation_mode_settings::{
    ActivePresentationMode, PresentationMode, PresentationModeSettings,
};
mod presentation_mode_settings;

pub fn init(cx: &mut App) {
    PresentationModeSettings::register(cx);

    cx.on_action(|_: &zed_actions::presentation_mode_selector::Toggle, cx| {
        workspace::with_active_or_new_workspace(cx, |workspace, window, cx| {
            toggle_presentation_mode_selector(workspace, window, cx);
        });
    });
}

fn toggle_presentation_mode_selector(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    workspace.toggle_modal(window, cx, |window, cx| {
        let settings = PresentationModeSettings::get_global(cx);
        let delegate = PresentationModeSelectorDelegate::new(
            cx.entity().downgrade(),
            settings.presentation_modes.clone(),
            window,
            cx,
        );
        PresentationModeSelector::new(delegate, window, cx)
    });
}

pub struct PresentationModeSelector {
    picker: Entity<Picker<PresentationModeSelectorDelegate>>,
}

impl ModalView for PresentationModeSelector {}

impl EventEmitter<DismissEvent> for PresentationModeSelector {}

impl Focusable for PresentationModeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for PresentationModeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl PresentationModeSelector {
    pub fn new(
        delegate: PresentationModeSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

pub struct PresentationModeSelectorDelegate {
    presentation_modes: Vec<Option<PresentationMode>>,
    matches: Vec<StringMatch>,
    selected_mode: Option<PresentationMode>,
    selected_index: usize,
    selection_completed: bool,
    selector: WeakEntity<PresentationModeSelector>,
}

impl PresentationModeSelectorDelegate {
    fn new(
        selector: WeakEntity<PresentationModeSelector>,
        mut configurations: Vec<PresentationMode>,
        _: &mut Window,
        _: &mut Context<PresentationModeSelector>,
    ) -> Self {
        configurations.sort_by_key(|c| c.name.clone());
        let mut configurations: Vec<_> = configurations.into_iter().map(Some).collect();
        configurations.insert(0, None);

        let matches = configurations
            .iter()
            .enumerate()
            .map(|(ix, mode)| StringMatch {
                candidate_id: ix,
                score: 0.0,
                positions: Default::default(),
                string: PresentationMode::display_name(mode),
            })
            .collect();

        Self {
            presentation_modes: configurations,
            matches,
            selected_mode: None,
            selected_index: 0,
            selection_completed: false,
            selector,
        }
    }

    /// Previews a presentation mode by temporarily applying only its non-window
    /// state changes. Window state changes are excluded to avoid slow,
    /// disruptive fullscreen transitions during preview.
    fn preview_presentation_mode(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) -> Option<PresentationMode> {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return None;
        };

        let Some(presentation_mode) = self.presentation_modes.get(mat.candidate_id) else {
            return None;
        };

        if let Some(presentation_mode) = presentation_mode {
            apply_theme_settings(presentation_mode, cx);
        }

        presentation_mode.clone()
    }
}

impl PickerDelegate for PresentationModeSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Select a presentation mode...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.selected_mode = self.preview_presentation_mode(window, cx);
        cx.refresh_windows();
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .presentation_modes
            .iter()
            .enumerate()
            .map(|(id, mode)| StringMatchCandidate::new(id, &PresentationMode::display_name(mode)))
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

            this.update_in(cx, |this, window, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                this.delegate.selected_mode = this.delegate.preview_presentation_mode(window, cx);
                cx.refresh_windows();
            })
            .ok();
        })
    }

    fn confirm(
        &mut self,
        _: bool,
        window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        self.selection_completed = true;

        match self.selected_mode {
            Some(_) => enable_presentation_mode(&self.selected_mode, window, cx),
            None => disable_presentation_mode(window, cx),
        }
        cx.refresh_windows();

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        restore_previous_presentation_mode(window, cx);
        cx.refresh_windows();
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let mode = &self.presentation_modes[mat.candidate_id];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    PresentationMode::display_name(mode),
                    mat.positions.clone(),
                )),
        )
    }
}

/// Enables a presentation mode by setting the global state and applying all its
/// settings. This captures the current window state before applying changes so
/// it can be restored later.
fn enable_presentation_mode(
    presentation_mode: &Option<PresentationMode>,
    window: &mut Window,
    cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
) {
    let Some(mode) = presentation_mode else {
        return;
    };

    let disabled_mode_is_in_full_screen = cx
        .try_global::<ActivePresentationMode>()
        .map(|active| active.disabled_mode_is_in_full_screen)
        .unwrap_or_else(|| window.is_fullscreen());

    cx.set_global(ActivePresentationMode {
        presentation_mode: mode.clone(),
        disabled_mode_is_in_full_screen,
    });

    apply_window_state_settings(mode.settings.full_screen, window);
    apply_theme_settings(mode, cx);
}

/// Disables the active presentation mode by restoring the original window state
/// and resetting all theme settings to their defaults.
fn disable_presentation_mode(
    window: &mut Window,
    cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
) {
    if let Some(active_mode) = cx.try_global::<ActivePresentationMode>() {
        apply_window_state_settings(Some(active_mode.disabled_mode_is_in_full_screen), window);
        cx.remove_global::<ActivePresentationMode>();
    }

    restore_theme_settings_to_defaults(cx);
}

/// Restores the presentation state when the picker is dismissed.
/// If there's an active presentation mode, it reapplies its settings.
/// Otherwise, it resets all settings to defaults.
fn restore_previous_presentation_mode(
    window: &mut Window,
    cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
) {
    if let Some(active) = cx.try_global::<ActivePresentationMode>() {
        let mode = &active.presentation_mode.clone();
        apply_window_state_settings(mode.settings.full_screen, window);
        apply_theme_settings(mode, cx);
    } else {
        restore_theme_settings_to_defaults(cx);
    }
}

fn apply_window_state_settings(full_screen: Option<bool>, window: &mut Window) {
    if let Some(full_screen) = full_screen {
        if full_screen {
            window.enter_fullscreen();
        } else {
            window.exit_fullscreen();
        }
    }
}

/// Applies only the theme settings (font size, font family, theme) from a
/// presentation mode. This excludes window state changes and is used during
/// preview to avoid slow, disruptive changes.
fn apply_theme_settings(
    mode: &PresentationMode,
    cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
) {
    // TODO - sprinkle throughout various getters that either:
    // 1. Pull from ActivePresentationMode
    // 2. Inject into current globals BuffontFontSize
    // 3. Or the following pattern

    if let Some(_buffer_font_family) = &mode.settings.buffer_font_family {
        // TODO: adjust buffer_font_family
    }

    if let Some(buffer_font_size) = &mode.settings.buffer_font_size {
        theme::adjust_buffer_font_size(cx, |size| {
            *size = px(buffer_font_size.0);
        });
    }

    // Apply theme
    if let Some(theme) = &mode.settings.theme {
        let registry = ThemeRegistry::global(cx);
        match registry.get(theme) {
            Ok(theme) => {
                SettingsStore::update_global(cx, |store, _| {
                    let mut theme_settings = store.get::<ThemeSettings>(None).clone();
                    theme_settings.active_theme = theme;
                    theme_settings.apply_theme_overrides();
                    store.override_global(theme_settings);
                });
            }
            Err(_) => log::warn!("Theme not found: {}", theme),
        }
    }
}

/// Resets all theme settings (font size, font family, theme) to system
/// defaults.
fn restore_theme_settings_to_defaults(cx: &mut Context<Picker<PresentationModeSelectorDelegate>>) {
    // Reset font size
    theme::reset_buffer_font_size(cx);

    // TODO: Reset font family when implemented

    // TODO: Reset theme to user's default theme
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor;
    use gpui::{DismissEvent, TestAppContext, VisualTestContext};
    use language;
    use menu::{Confirm, SelectNext};
    use presentation_mode_settings::PresentationModeConfiguration;
    use project::{FakeFs, Project};
    use workspace::{self, AppState};
    use zed_actions::presentation_mode_selector;

    async fn init_test(
        presentation_modes: Vec<PresentationMode>,
        full_screen: bool,
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, &mut VisualTestContext) {
        cx.update(|cx| {
            let state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            super::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        });

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<PresentationModeSettings>(cx, |settings| {
                    *settings = presentation_modes;
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, ["/test".as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        cx.update(|window, cx| {
            apply_window_state_settings(Some(full_screen), window);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        (workspace, cx)
    }

    #[track_caller]
    fn active_presentation_mode_picker(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<PresentationModeSelectorDelegate>> {
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<PresentationModeSelector>(cx)
                .expect("presentation mode selector is not open")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn full_screen_mode() -> PresentationMode {
        PresentationMode {
            name: "Full Screen Mode".to_string(),
            settings: PresentationModeConfiguration {
                full_screen: Some(true),
                ..Default::default()
            },
        }
    }

    // TODO: Use in a test
    fn windowed_mode() -> PresentationMode {
        PresentationMode {
            name: "Windowed Mode".to_string(),
            settings: PresentationModeConfiguration {
                full_screen: Some(false),
                ..Default::default()
            },
        }
    }

    fn no_full_screen_setting_mode() -> PresentationMode {
        PresentationMode {
            name: "No Full Screen Setting".to_string(),
            settings: PresentationModeConfiguration::default(),
        }
    }

    #[gpui::test]
    async fn test_full_screen_is_not_entered_when_setting_not_configured_when_starting_in_windowed_mode(
        cx: &mut TestAppContext,
    ) {
        let no_full_screen_setting_mode = no_full_screen_setting_mode();
        let presentation_modes = vec![no_full_screen_setting_mode.clone()];
        let (workspace, cx) = init_test(presentation_modes, true, cx).await;
        cx.update(|window, _| {
            assert!(window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert_eq!(picker.delegate.matches.len(), 2);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in full screen mode
        cx.update(|window, _| {
            assert!(window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select no full screen mode option
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(
                picker.delegate.selected_mode,
                Some(no_full_screen_setting_mode.clone())
            );
            let active_presentation_mode = cx.try_global::<ActivePresentationMode>();
            assert_eq!(
                active_presentation_mode,
                Some(&ActivePresentationMode {
                    presentation_mode: no_full_screen_setting_mode.clone(),
                    disabled_mode_is_in_full_screen: true
                })
            );
        });

        // Remains in full screen mode
        cx.update(|window, _| {
            assert!(window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in full screen mode
        cx.update(|window, _| {
            assert!(window.is_fullscreen());
        });
    }

    #[gpui::test]
    async fn test_full_screen_remains_when_setting_not_configured_when_starting_in_full_screen_mode(
        cx: &mut TestAppContext,
    ) {
        let no_full_screen_setting_mode = no_full_screen_setting_mode();
        let presentation_modes = vec![no_full_screen_setting_mode.clone()];
        let (workspace, cx) = init_test(presentation_modes, false, cx).await;
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert_eq!(picker.delegate.matches.len(), 2);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select no full screen mode option
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(
                picker.delegate.selected_mode,
                Some(no_full_screen_setting_mode.clone())
            );
            let active_presentation_mode = cx.try_global::<ActivePresentationMode>();
            assert_eq!(
                active_presentation_mode,
                Some(&ActivePresentationMode {
                    presentation_mode: no_full_screen_setting_mode.clone(),
                    disabled_mode_is_in_full_screen: false
                })
            );
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });
    }

    #[gpui::test]
    async fn test_full_screen_is_entered_on_mode_switch_and_exited_when_disabling_when_starting_in_windowed_mode(
        cx: &mut TestAppContext,
    ) {
        let full_screen_mode = full_screen_mode();
        let presentation_modes = vec![full_screen_mode.clone()];
        let (workspace, cx) = init_test(presentation_modes, false, cx).await;
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert_eq!(picker.delegate.matches.len(), 2);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, _| {
            assert_eq!(picker.delegate.selected_mode, None);
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select full screen mode option
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(
                picker.delegate.selected_mode,
                Some(full_screen_mode.clone())
            );

            let active_presentation_mode = cx.try_global::<ActivePresentationMode>();
            assert_eq!(
                active_presentation_mode,
                Some(&ActivePresentationMode {
                    presentation_mode: full_screen_mode.clone(),
                    disabled_mode_is_in_full_screen: false
                })
            );
        });

        // Switches to full screen mode
        cx.update(|window, _| {
            assert!(window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        // Select disabled option
        cx.dispatch_action(Confirm);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Switches to windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });
    }

    #[gpui::test]
    async fn test_presentation_mode_dismissed_reverts_to_previous_state(cx: &mut TestAppContext) {
        let full_screen_mode = full_screen_mode();
        let presentation_modes = vec![full_screen_mode.clone()];
        let (workspace, cx) = init_test(presentation_modes, false, cx).await;
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // ---------------------------------------------------------------------

        cx.dispatch_action(presentation_mode_selector::Toggle);
        let picker = active_presentation_mode_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_mode, None);
            assert_eq!(picker.delegate.matches.len(), 2);
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Preview full screen mode, but do not select (confirm) it
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(
                picker.delegate.selected_mode,
                Some(full_screen_mode.clone())
            );
            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });

        // Dismiss the picker, should revert back to previous settings
        picker.update(cx, |_, cx| cx.emit(DismissEvent));

        picker.read_with(cx, |picker, cx| {
            assert_eq!(
                picker.delegate.selected_mode,
                Some(full_screen_mode.clone())
            );

            assert!(!cx.has_global::<ActivePresentationMode>());
        });

        // Remains in windowed mode
        cx.update(|window, _| {
            assert!(!window.is_fullscreen());
        });
    }
}
