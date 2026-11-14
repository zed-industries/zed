use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, WeakEntity, Window,
};
use picker::{HighlightedText, Picker, PickerDelegate, PreviewUpdate};
use settings::{ActiveSettingsProfileName, SettingsStore};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.on_action(|_: &zed_actions::settings_profile_selector::Toggle, cx| {
        workspace::with_active_or_new_workspace(cx, |workspace, window, cx| {
            toggle_settings_profile_selector(workspace, window, cx);
        });
    });
}

fn toggle_settings_profile_selector(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = SettingsProfileSelectorDelegate::new(cx.entity().downgrade(), window, cx);
        SettingsProfileSelector::new(delegate, project, window, cx)
    });
}

pub struct SettingsProfileSelector {
    picker: Entity<Picker<SettingsProfileSelectorDelegate>>,
}

impl ModalView for SettingsProfileSelector {}

impl EventEmitter<DismissEvent> for SettingsProfileSelector {}

impl Focusable for SettingsProfileSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SettingsProfileSelector {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl SettingsProfileSelector {
    pub fn new(
        delegate: SettingsProfileSelectorDelegate,
        project: Entity<project::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let preview = picker_preview::editor_preview(project, window, cx);
        let picker = cx.new(|cx| {
            Picker::uniform_list_with_preview(delegate, preview, window, cx)
                .initial_width(rems(22.))
        });
        Self { picker }
    }
}

pub struct SettingsProfileSelectorDelegate {
    matches: Vec<StringMatch>,
    profile_names: Vec<Option<String>>,
    original_profile_name: Option<String>,
    selected_profile_name: Option<String>,
    selected_index: usize,
    preview_enabled: bool,
    selection_completed: bool,
    /// The profile this selector last applied without it being confirmed yet,
    /// or `None` if it hasn't applied anything.
    applied_profile_name: Option<Option<String>>,
    selector: WeakEntity<SettingsProfileSelector>,
}

impl SettingsProfileSelectorDelegate {
    fn new(
        selector: WeakEntity<SettingsProfileSelector>,
        _: &mut Window,
        cx: &mut Context<SettingsProfileSelector>,
    ) -> Self {
        let settings_store = cx.global::<SettingsStore>();
        let mut profile_names: Vec<Option<String>> = settings_store
            .configured_settings_profiles()
            .map(|s| Some(s.to_string()))
            .collect();
        profile_names.insert(0, None);

        let matches = profile_names
            .iter()
            .enumerate()
            .map(|(ix, profile_name)| StringMatch {
                candidate_id: ix,
                score: 0.0,
                positions: Default::default(),
                string: display_name(profile_name),
            })
            .collect();

        let profile_name = cx
            .try_global::<ActiveSettingsProfileName>()
            .map(|p| p.0.clone());

        let mut this = Self {
            matches,
            profile_names,
            original_profile_name: profile_name.clone(),
            selected_profile_name: None,
            selected_index: 0,
            preview_enabled: false,
            selection_completed: false,
            applied_profile_name: None,
            selector,
        };

        if let Some(profile_name) = profile_name {
            this.select_if_matching(&profile_name);
        }

        this
    }

    fn select_if_matching(&mut self, profile_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == profile_name)
            .unwrap_or(self.selected_index);
    }

    fn set_selected_profile(
        &mut self,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Option<String> {
        let mat = self.matches.get(self.selected_index)?;
        let profile_name = self.profile_names.get(mat.candidate_id)?.clone();
        let applied_profile_name = Self::update_active_profile_name_global(profile_name, cx);
        self.applied_profile_name = Some(applied_profile_name.clone());
        applied_profile_name
    }

    fn restore_original_profile(&mut self, cx: &mut Context<Picker<Self>>) {
        let Some(applied_profile_name) = self.applied_profile_name.take() else {
            return;
        };
        let active_profile_name = cx
            .try_global::<ActiveSettingsProfileName>()
            .map(|profile_name| profile_name.0.clone());
        // Only undo our own change: if the profile was changed elsewhere since
        // this selector applied one, keep that newer choice.
        if active_profile_name == applied_profile_name
            && active_profile_name != self.original_profile_name
        {
            Self::update_active_profile_name_global(self.original_profile_name.clone(), cx);
        }
        self.selected_profile_name = None;
    }

    fn update_active_profile_name_global(
        profile_name: Option<String>,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Option<String> {
        if let Some(profile_name) = profile_name {
            cx.set_global(ActiveSettingsProfileName(profile_name.clone()));
            return Some(profile_name);
        }

        if cx.has_global::<ActiveSettingsProfileName>() {
            cx.remove_global::<ActiveSettingsProfileName>();
        }

        None
    }
}

impl PickerDelegate for SettingsProfileSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "settings profile selector"
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> std::sync::Arc<str> {
        "Select a settings profile...".into()
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
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        if self.preview_enabled {
            self.selected_profile_name = self.set_selected_profile(cx);
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .profile_names
            .iter()
            .enumerate()
            .map(|(id, profile_name)| StringMatchCandidate::new(id, &display_name(profile_name)))
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

            this.update_in(cx, |this, _, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                if this.delegate.preview_enabled {
                    this.delegate.selected_profile_name = this.delegate.set_selected_profile(cx);
                }
            })
            .ok();
        })
    }

    fn confirm(
        &mut self,
        _: bool,
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        if self.matches.get(self.selected_index).is_none() {
            return;
        }
        self.selected_profile_name = self.set_selected_profile(cx);
        self.selection_completed = true;
        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        if !self.selection_completed {
            self.restore_original_profile(cx);
        }
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn preview_visibility_changed(
        &mut self,
        visible: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.preview_enabled = visible;
        if visible {
            self.selected_profile_name = self.set_selected_profile(cx);
        } else {
            self.restore_original_profile(cx);
        }
    }

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<PreviewUpdate> {
        let mat = self.matches.get(self.selected_index)?;
        let profile_name = self.profile_names.get(mat.candidate_id)?;
        Some(PreviewUpdate::message(HighlightedText {
            text: format!("Previewing \"{}\"", display_name(profile_name)).into(),
            highlights: Vec::new(),
        }))
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;
        let profile_name = &self.profile_names.get(mat.candidate_id)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    display_name(profile_name),
                    mat.positions.clone(),
                )),
        )
    }
}

fn display_name(profile_name: &Option<String>) -> String {
    profile_name.clone().unwrap_or("Disabled".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use menu::{Cancel, Confirm, SelectNext, SelectPrevious};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::Settings;
    use theme_settings::ThemeSettings;
    use workspace::{self, AppState, MultiWorkspace};
    use zed_actions::settings_profile_selector;

    async fn init_test(
        user_settings_json: serde_json::Value,
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, &mut VisualTestContext) {
        cx.update(|cx| {
            // Persistence (e.g. the picker's last preview layout) lives in the
            // key-value store, so give every test its own database instead of
            // sharing the process-wide fallback.
            cx.set_global(db::AppDatabase::test_new());
            let state = AppState::test(cx);
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            settings::init(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            super::init(cx);
            editor::init(cx);
            state
        });

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(&user_settings_json.to_string(), cx)
                    .unwrap();
            });
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, ["/test".as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let cx = VisualTestContext::from_window(*window, cx).into_mut();
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        cx.update(|_, cx| {
            assert!(!cx.has_global::<ActiveSettingsProfileName>());
        });

        (workspace, cx)
    }

    #[track_caller]
    fn active_settings_profile_picker(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<SettingsProfileSelectorDelegate>> {
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<SettingsProfileSelector>(cx)
                .expect("settings profile selector is not open")
                .read(cx)
                .picker
                .clone()
        })
    }

    #[gpui::test]
    async fn test_settings_profile_selector_state(cx: &mut TestAppContext) {
        let classroom_and_streaming_profile_name = "Classroom / Streaming".to_string();
        let demo_videos_profile_name = "Demo Videos".to_string();

        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                classroom_and_streaming_profile_name.clone(): {
                    "settings": {
                        "buffer_font_size": 20.0,
                    }
                },
                demo_videos_profile_name.clone(): {
                    "settings": {
                        "buffer_font_size": 15.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.matches.len(), 3);
            assert_eq!(picker.delegate.matches[0].string, display_name(&None));
            assert_eq!(
                picker.delegate.matches[1].string,
                classroom_and_streaming_profile_name
            );
            assert_eq!(picker.delegate.matches[2].string, demo_videos_profile_name);
            assert_eq!(picker.delegate.matches.get(3), None);

            assert_eq!(picker.delegate.selected_index, 0);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Cancel);

        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 2);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 2);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(SelectPrevious);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(Cancel);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );

            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 2);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );

            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(SelectPrevious);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(picker.delegate.selected_profile_name, None);

            // The preview is hidden, so moving the highlight leaves the
            // previously confirmed profile active.
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name.clone())
            );

            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(SelectPrevious);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 0);
            assert_eq!(picker.delegate.selected_profile_name, None);

            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(demo_videos_profile_name)
            );

            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });
    }

    #[gpui::test]
    async fn test_settings_profile_with_user_base(cx: &mut TestAppContext) {
        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                "Explicit User": {
                    "base": "user",
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                },
                "Implicit User": {
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        // Select "Explicit User" (index 1) — profile applies on top of user settings on confirm.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Explicit User".to_string())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        // Select "Implicit User" (index 2) — no base specified, same behavior.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Implicit User".to_string())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });
    }

    #[gpui::test]
    async fn test_settings_profile_with_default_base(cx: &mut TestAppContext) {
        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                "Clean Slate": {
                    "base": "default"
                },
                "Custom on Defaults": {
                    "base": "default",
                    "settings": {
                        "buffer_font_size": 30.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        // User has buffer_font_size: 10, factory default is 15.
        cx.update(|_, cx| {
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // "Clean Slate" has base: "default" with no settings overrides, so
        // confirming it gives the factory default (15), not the user's value (10).
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            // The preview is hidden, so nothing is applied until confirm.
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Clean Slate".to_string())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        // "Custom on Defaults" has base: "default" with buffer_font_size: 30, so
        // the profile's override (30) applies on top of the factory default (15),
        // not on top of the user's value (10).
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Custom on Defaults".to_string())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(30.0));
        });
    }

    #[gpui::test]
    async fn test_settings_profile_selector_is_in_user_configuration_order(
        cx: &mut TestAppContext,
    ) {
        // Must be unique names (HashMap)
        let user_settings_json = json!({
            "profiles": {
                "z": { "settings": {} },
                "e": { "settings": {} },
                "d": { "settings": {} },
                " ": { "settings": {} },
                "r": { "settings": {} },
                "u": { "settings": {} },
                "l": { "settings": {} },
                "3": { "settings": {} },
                "s": { "settings": {} },
                "!": { "settings": {} },
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        picker.read_with(cx, |picker, _| {
            assert_eq!(picker.delegate.matches.len(), 11);
            assert_eq!(picker.delegate.matches[0].string, display_name(&None));
            assert_eq!(picker.delegate.matches[1].string, "z");
            assert_eq!(picker.delegate.matches[2].string, "e");
            assert_eq!(picker.delegate.matches[3].string, "d");
            assert_eq!(picker.delegate.matches[4].string, " ");
            assert_eq!(picker.delegate.matches[5].string, "r");
            assert_eq!(picker.delegate.matches[6].string, "u");
            assert_eq!(picker.delegate.matches[7].string, "l");
            assert_eq!(picker.delegate.matches[8].string, "3");
            assert_eq!(picker.delegate.matches[9].string, "s");
            assert_eq!(picker.delegate.matches[10].string, "!");
        });
    }

    #[gpui::test]
    async fn test_settings_profile_selector_preview(cx: &mut TestAppContext) {
        let large_font_profile_name = "Large Font".to_string();

        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                large_font_profile_name.clone(): {
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.update(|_, cx| {
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // The picker opens with its preview hidden by default.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        picker.read_with(cx, |picker, _| {
            assert!(!picker.delegate.preview_enabled);
        });

        // Turning the preview on applies the highlighted profile, which is the
        // "Disabled" entry here, so nothing changes yet.
        cx.dispatch_action(picker::TogglePreview);
        cx.run_until_parked();

        picker.read_with(cx, |picker, cx| {
            assert!(picker.delegate.preview_enabled);
            assert_eq!(picker.delegate.selected_index, 0);
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // With the preview shown, moving the highlight applies the profile live.
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(
                picker.delegate.selected_profile_name,
                Some(large_font_profile_name.clone())
            );
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some(large_font_profile_name.clone())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        // Cancel restores the profile that was active when the picker opened.
        cx.dispatch_action(Cancel);
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // The preview layout is persisted, so it is still shown when reopening.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        picker.read_with(cx, |picker, cx| {
            assert!(picker.delegate.preview_enabled);
            assert_eq!(picker.delegate.selected_index, 0);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // ...and the highlight still applies profiles live.
        cx.dispatch_action(SelectNext);
        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        // Turning the preview off restores the original profile immediately.
        cx.dispatch_action(picker::TogglePreview);
        cx.run_until_parked();

        picker.read_with(cx, |picker, cx| {
            assert!(!picker.delegate.preview_enabled);
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        // ...and the highlight no longer applies profiles.
        cx.dispatch_action(SelectPrevious);
        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 0);
            assert_eq!(picker.delegate.selected_profile_name, None);
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(Cancel);
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_settings_profile_selector_restores_profile_when_closed_from_outside(
        cx: &mut TestAppContext,
    ) {
        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                "Large Font": {
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.dispatch_action(settings_profile_selector::Toggle);
        active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(picker::TogglePreview);
        cx.dispatch_action(SelectNext);

        cx.update(|_, cx| {
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        // Closing the modal from outside the picker (e.g. clicking outside of
        // it) must not keep the previewed profile.
        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.hide_modal(window, cx));
        });
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });
    }

    #[gpui::test]
    async fn test_settings_profile_selector_cancel_keeps_profile_changed_elsewhere(
        cx: &mut TestAppContext,
    ) {
        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                "Large Font": {
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.dispatch_action(settings_profile_selector::Toggle);
        active_settings_profile_picker(&workspace, cx);

        // The profile changes from somewhere else (e.g. another window) while
        // this selector is open with its preview hidden.
        cx.update(|_, cx| {
            cx.set_global(ActiveSettingsProfileName("Large Font".to_string()));
        });

        // This selector never applied anything, so cancelling it must not undo
        // that change.
        cx.dispatch_action(Cancel);
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Large Font".to_string())
            );
        });
    }

    #[gpui::test]
    async fn test_settings_profile_selector_hiding_preview_keeps_profile_changed_elsewhere(
        cx: &mut TestAppContext,
    ) {
        let user_settings_json = json!({
            "buffer_font_size": 10.0,
            "profiles": {
                "Large Font": {
                    "settings": {
                        "buffer_font_size": 20.0
                    }
                },
                "Huge Font": {
                    "settings": {
                        "buffer_font_size": 30.0
                    }
                }
            }
        });
        let (workspace, cx) = init_test(user_settings_json, cx).await;

        cx.dispatch_action(settings_profile_selector::Toggle);
        active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(picker::TogglePreview);
        cx.dispatch_action(SelectNext);
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Large Font".to_string())
            );
        });

        // While this selector previews "Large Font", the profile changes from
        // somewhere else (e.g. another window).
        cx.update(|_, cx| {
            cx.set_global(ActiveSettingsProfileName("Huge Font".to_string()));
        });

        // Hiding the preview and cancelling must only undo this selector's own
        // change, not the newer one made elsewhere.
        cx.dispatch_action(picker::TogglePreview);
        cx.run_until_parked();
        cx.dispatch_action(Cancel);
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|p| p.0.clone()),
                Some("Huge Font".to_string())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(30.0));
        });
    }
}
