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
    PresentationMode, PresentationModeConfiguration, PresentationModeSettings,
    PresentationModeState,
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
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
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
    selected_index: usize,
    selection_completed: bool,
    selector: WeakEntity<PresentationModeSelector>,
    state: PresentationModeState,
}

impl PresentationModeSelectorDelegate {
    fn new(
        selector: WeakEntity<PresentationModeSelector>,
        mut configurations: Vec<PresentationMode>,
        window: &mut Window,
        cx: &mut Context<PresentationModeSelector>,
    ) -> Self {
        configurations.sort_by_key(|c| c.name.clone());
        let mut configurations: Vec<_> = configurations.into_iter().map(Some).collect();
        // TODO: Should these be non-optional and the disabled state stored here?
        configurations.insert(0, None);

        let state = match cx.try_global::<PresentationModeState>() {
            Some(state) => state.clone(),
            None => PresentationModeState {
                disabled: PresentationMode {
                    name: PresentationMode::display_name(&None),
                    settings: PresentationModeConfiguration {
                        agent_font_size: Some(ThemeSettings::get_global(cx).agent_font_size(cx)),
                        buffer_font_size: Some(ThemeSettings::get_global(cx).buffer_font_size(cx)),
                        full_screen: Some(window.is_fullscreen()),
                        theme: Some(
                            ThemeSettings::get_global(cx)
                                .active_theme
                                .name
                                .clone()
                                .into(),
                        ),
                        ui_font_size: Some(ThemeSettings::get_global(cx).ui_font_size(cx)),
                    },
                },
                selected: None,
            },
        };

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

        let mut this = Self {
            presentation_modes: configurations,
            matches,
            selected_index: 0,
            selection_completed: false,
            selector,
            state: state.clone(),
        };

        if let Some(selected) = &state.selected {
            this.select_if_matching(&selected.name);
        }

        this
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

        // TODO: Clean up mode, selected, active
        let mode = match &presentation_mode {
            Some(selected) => selected,
            None => &self.state.disabled,
        };
        apply_theme_settings(mode, cx);

        presentation_mode.clone()
    }

    fn select_if_matching(&mut self, presentation_mode_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == presentation_mode_name)
            .unwrap_or(self.selected_index);
    }
}

impl PickerDelegate for PresentationModeSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> std::sync::Arc<str> {
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
        self.state.selected = self.preview_presentation_mode(window, cx);
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
                this.delegate.state.selected = this.delegate.preview_presentation_mode(window, cx);
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

        let mode = match &self.state.selected {
            Some(selected) => {
                cx.set_global(self.state.clone());
                selected
            }
            None => {
                if cx.has_global::<PresentationModeState>() {
                    cx.remove_global::<PresentationModeState>();
                }
                &self.state.disabled
            }
        };
        apply_window_state_settings(mode.settings.full_screen, window);
        apply_theme_settings(&mode, cx);
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
        match cx.try_global::<PresentationModeState>().cloned() {
            Some(state) => {
                if let Some(mode) = &state.selected {
                    apply_window_state_settings(mode.settings.full_screen, window);
                    apply_theme_settings(mode, cx);
                }
            }
            None => {
                let mode = &self.state.disabled;
                apply_window_state_settings(mode.settings.full_screen, window);
                apply_theme_settings(mode, cx);
            }
        }

        cx.refresh_windows();

        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
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
    if let Some(agent_font_size) = &mode.settings.agent_font_size {
        theme::adjust_agent_font_size(cx, |size| {
            *size = px(agent_font_size.0);
        });
    }

    if let Some(buffer_font_size) = &mode.settings.buffer_font_size {
        theme::adjust_buffer_font_size(cx, |size| {
            *size = px(buffer_font_size.0);
        });
    }

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

    if let Some(ui_font_size) = &mode.settings.ui_font_size {
        theme::adjust_ui_font_size(cx, |size| {
            *size = px(ui_font_size.0);
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use editor;
//     use gpui::{AppContext, DismissEvent, TestAppContext, VisualTestContext};
//     use language;
//     use menu::{Confirm, SelectNext};
//     use presentation_mode_settings::PresentationModeConfiguration;
//     use project::{FakeFs, Project};
//     use theme::ThemeSettingsContent;
//     use workspace::{self, AppState};
//     use zed_actions::presentation_mode_selector;

//     async fn init_test(
//         presentation_modes_functions: Vec<Box<dyn Fn(&mut TestAppContext) -> PresentationMode>>,
//         full_screen: bool,
//         cx: &mut TestAppContext,
//     ) -> (Entity<Workspace>, &mut VisualTestContext) {
//         cx.update(|cx| {
//             let state = AppState::test(cx);
//             theme::init(theme::LoadThemes::JustBase, cx);
//             language::init(cx);
//             super::init(cx);
//             editor::init(cx);
//             workspace::init_settings(cx);
//             Project::init_settings(cx);
//             state
//         });

//         let presentation_modes: Vec<_> =
//             presentation_modes_functions.iter().map(|f| f(cx)).collect();

//         cx.update(|cx| {
//             SettingsStore::update_global(cx, |store, cx| {
//                 store.update_user_settings::<PresentationModeSettings>(cx, |settings| {
//                     *settings = presentation_modes;
//                 });
//             });
//         });

//         let fs = FakeFs::new(cx.executor());
//         let project = Project::test(fs, ["/test".as_ref()], cx).await;
//         let (workspace, cx) =
//             cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

//         cx.update(|window, cx| {
//             apply_window_state_settings(Some(full_screen), window);
//             assert!(!cx.has_global::<PresentationModeState>());
//         });

//         (workspace, cx)
//     }

//     #[track_caller]
//     fn active_presentation_mode_picker(
//         workspace: &Entity<Workspace>,
//         cx: &mut VisualTestContext,
//     ) -> Entity<Picker<PresentationModeSelectorDelegate>> {
//         workspace.update(cx, |workspace, cx| {
//             workspace
//                 .active_modal::<PresentationModeSelector>(cx)
//                 .expect("presentation mode selector is not open")
//                 .read(cx)
//                 .picker
//                 .clone()
//         })
//     }

//     fn full_screen_mode() -> PresentationMode {
//         PresentationMode {
//             name: "Full Screen Mode".to_string(),
//             settings: PresentationModeConfiguration {
//                 agent_font_size: None,
//                 buffer_font_size: None,
//                 full_screen: Some(true),
//                 theme: None,
//                 ui_font_size: None,
//             },
//         }
//     }

//     // TODO: Use in a test
//     fn windowed_mode() -> PresentationMode {
//         PresentationMode {
//             name: "Windowed Mode".to_string(),
//             settings: PresentationModeConfiguration {
//                 agent_font_size: None,
//                 buffer_font_size: None,
//                 full_screen: Some(false),
//                 theme: None,
//                 ui_font_size: None,
//             },
//         }
//     }

//     fn no_full_screen_setting_mode(cx: &mut VisualTestContext) -> PresentationMode {
//         let agent_font_size = cx.update(|_, cx| ThemeSettings::get_global(cx).agent_font_size(cx));
//         let buffer_font_size =
//             cx.update(|_, cx| ThemeSettings::get_global(cx).buffer_font_size(cx));
//         let theme = cx.update(|_, cx| {
//             ThemeSettings::get_global(cx)
//                 .active_theme
//                 .name
//                 .clone()
//                 .into()
//         });
//         let ui_font_size = cx.update(|_, cx| ThemeSettings::get_global(cx).ui_font_size(cx));

//         PresentationMode {
//             name: "No Full Screen Setting".to_string(),
//             settings: PresentationModeConfiguration {
//                 agent_font_size: Some(agent_font_size),
//                 buffer_font_size: Some(buffer_font_size),
//                 full_screen: None,
//                 theme: Some(theme),
//                 ui_font_size: Some(ui_font_size),
//             },
//         }
//     }

//     #[gpui::test]
//     async fn test_full_screen_is_not_entered_when_setting_not_configured_when_starting_in_windowed_mode(
//         cx: &mut TestAppContext,
//     ) {
//         let presentation_modes = vec![no_full_screen_setting_mode];
//         let (workspace, cx) =
//             init_test(vec![Box::new(no_full_screen_setting_mode)], true, cx).await;
//         cx.update(|window, _| {
//             assert!(window.is_fullscreen());
//         });

//         // ---------------------------------------------------------------------

//         cx.dispatch_action(presentation_mode_selector::Toggle);
//         let picker = active_presentation_mode_picker(&workspace, cx);

//         picker.read_with(cx, |picker, cx| {
//             assert_eq!(picker.delegate.state.selected, None);
//             assert_eq!(picker.delegate.matches.len(), 2);
//             assert!(!cx.has_global::<PresentationModeState>());
//         });

//         // Select disabled option
//         cx.dispatch_action(Confirm);

//         picker.read_with(cx, |picker, cx| {
//             assert_eq!(picker.delegate.state.selected, None);
//             assert!(!cx.has_global::<PresentationModeState>());
//         });

//         // Remains in full screen mode
//         cx.update(|window, _| {
//             assert!(window.is_fullscreen());
//         });

//         // ---------------------------------------------------------------------

//         cx.dispatch_action(presentation_mode_selector::Toggle);
//         let picker = active_presentation_mode_picker(&workspace, cx);

//         // Select no full screen mode option
//         cx.dispatch_action(SelectNext);
//         cx.dispatch_action(Confirm);

//         picker.read_with(cx, |picker, cx| {
//             assert_eq!(
//                 picker.delegate.state.selected,
//                 Some(no_full_screen_setting_mode.clone())
//             );
//             let state = cx.try_global::<PresentationModeState>().unwrap();
//             assert_eq!(
//                 state.disabled,
//                 PresentationMode {
//                     name: PresentationMode::display_name(&None),
//                     settings: Default::default()
//                 }
//             );
//             assert_eq!(state.selected, Some(no_full_screen_setting_mode));
//         });

// // Remains in full screen mode
// cx.update(|window, _| {
//     assert!(window.is_fullscreen());
// });

// // ---------------------------------------------------------------------

// cx.dispatch_action(presentation_mode_selector::Toggle);
// let picker = active_presentation_mode_picker(&workspace, cx);

// // Select disabled option
// cx.dispatch_action(Confirm);

// picker.read_with(cx, |picker, cx| {
//     assert_eq!(picker.delegate.selected_mode, None);
//     assert!(!cx.has_global::<DisabledPresentationModeState>());
// });

// // Remains in full screen mode
// cx.update(|window, _| {
//     assert!(window.is_fullscreen());
// });
// }

// #[gpui::test]
// async fn test_full_screen_remains_when_setting_not_configured_when_starting_in_full_screen_mode(
//     cx: &mut TestAppContext,
// ) {
//     let no_full_screen_setting_mode = no_full_screen_setting_mode();
//     let presentation_modes = vec![no_full_screen_setting_mode.clone()];
//     let (workspace, cx) = init_test(presentation_modes, false, cx).await;
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert_eq!(picker.delegate.matches.len(), 2);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Select disabled option
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     // Select no full screen mode option
//     cx.dispatch_action(SelectNext);
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(
//             picker.delegate.selected_mode,
//             Some(no_full_screen_setting_mode.clone())
//         );
//         let active_presentation_mode = cx.try_global::<DisabledPresentationModeState>();
//         assert_eq!(
//             active_presentation_mode,
//             Some(&DisabledPresentationModeState {
//                 presentation_mode: no_full_screen_setting_mode.clone(),
//                 full_screen: false
//             })
//         );
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     // Select disabled option
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });
// }

// #[gpui::test]
// async fn test_full_screen_is_entered_on_mode_switch_and_exited_when_disabling_when_starting_in_windowed_mode(
//     cx: &mut TestAppContext,
// ) {
//     let full_screen_mode = full_screen_mode();
//     let presentation_modes = vec![full_screen_mode.clone()];
//     let (workspace, cx) = init_test(presentation_modes, false, cx).await;
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert_eq!(picker.delegate.matches.len(), 2);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Select disabled option
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, _| {
//         assert_eq!(picker.delegate.selected_mode, None);
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     // Select full screen mode option
//     cx.dispatch_action(SelectNext);
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(
//             picker.delegate.selected_mode,
//             Some(full_screen_mode.clone())
//         );

//         let active_presentation_mode = cx.try_global::<DisabledPresentationModeState>();
//         assert_eq!(
//             active_presentation_mode,
//             Some(&DisabledPresentationModeState {
//                 presentation_mode: full_screen_mode.clone(),
//                 full_screen: false
//             })
//         );
//     });

//     // Switches to full screen mode
//     cx.update(|window, _| {
//         assert!(window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     // Select disabled option
//     cx.dispatch_action(Confirm);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Switches to windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });
// }

// #[gpui::test]
// async fn test_presentation_mode_dismissed_reverts_to_previous_state(cx: &mut TestAppContext) {
//     let full_screen_mode = full_screen_mode();
//     let presentation_modes = vec![full_screen_mode.clone()];
//     let (workspace, cx) = init_test(presentation_modes, false, cx).await;
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // ---------------------------------------------------------------------

//     cx.dispatch_action(presentation_mode_selector::Toggle);
//     let picker = active_presentation_mode_picker(&workspace, cx);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(picker.delegate.selected_mode, None);
//         assert_eq!(picker.delegate.matches.len(), 2);
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Preview full screen mode, but do not select (confirm) it
//     cx.dispatch_action(SelectNext);

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(
//             picker.delegate.selected_mode,
//             Some(full_screen_mode.clone())
//         );
//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });

//     // Dismiss the picker, should revert back to previous settings
//     picker.update(cx, |_, cx| cx.emit(DismissEvent));

//     picker.read_with(cx, |picker, cx| {
//         assert_eq!(
//             picker.delegate.selected_mode,
//             Some(full_screen_mode.clone())
//         );

//         assert!(!cx.has_global::<DisabledPresentationModeState>());
//     });

//     // Remains in windowed mode
//     cx.update(|window, _| {
//         assert!(!window.is_fullscreen());
//     });
// }
// }
