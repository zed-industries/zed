use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task,
    WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use settings::{ActiveSettingsProfileName, SettingsStore};
use ui::{Button, Color, HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{DismissDecision, ModalView, Workspace};

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
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = SettingsProfileSelectorDelegate::new(cx.entity().downgrade(), window, cx);
        SettingsProfileSelector::new(delegate, window, cx)
    });
}

pub struct SettingsProfileSelector {
    picker: Entity<Picker<SettingsProfileSelectorDelegate>>,
}

impl ModalView for SettingsProfileSelector {
    fn on_before_dismiss(&mut self, _: &mut Window, cx: &mut Context<Self>) -> DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.live_preview = false;
            picker.delegate.restore_preview(cx);
        });
        DismissDecision::Dismiss(true)
    }
}

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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker =
            cx.new(|cx| Picker::uniform_list(delegate, window, cx).initial_width(rems(22.)));
        Self { picker }
    }
}

pub struct SettingsProfileSelectorDelegate {
    matches: Vec<StringMatch>,
    profile_names: Vec<Option<String>>,
    selected_index: usize,
    live_preview: bool,
    preview: Option<ProfilePreview>,
    selector: WeakEntity<SettingsProfileSelector>,
}

struct ProfilePreview {
    previous: Option<String>,
    applied: Option<String>,
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
            selected_index: 0,
            live_preview: false,
            preview: None,
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

    fn selected_profile_name(&self) -> Option<&Option<String>> {
        let selected_match = self.matches.get(self.selected_index)?;
        self.profile_names.get(selected_match.candidate_id)
    }

    fn active_profile(cx: &App) -> Option<String> {
        cx.try_global::<ActiveSettingsProfileName>()
            .map(|name| name.0.clone())
    }

    fn apply_profile(profile: Option<String>, cx: &mut App) {
        if let Some(profile) = profile {
            cx.set_global(ActiveSettingsProfileName(profile));
        } else if cx.has_global::<ActiveSettingsProfileName>() {
            cx.remove_global::<ActiveSettingsProfileName>();
        }
    }

    fn restore_preview(&mut self, cx: &mut App) {
        if let Some(preview) = self.preview.take()
            && Self::active_profile(cx) == preview.applied
        {
            Self::apply_profile(preview.previous, cx);
        }
    }

    fn update_preview(&mut self, cx: &mut App) {
        if !self.live_preview {
            return;
        }
        let Some(profile) = self.selected_profile_name().cloned() else {
            self.restore_preview(cx);
            return;
        };
        let active = Self::active_profile(cx);
        if profile == active {
            if self
                .preview
                .as_ref()
                .is_some_and(|preview| preview.applied != active)
            {
                self.preview = None;
            }
            return;
        }
        let previous = match self.preview.take() {
            Some(preview) if preview.applied == active => preview.previous,
            _ => active,
        };
        Self::apply_profile(profile.clone(), cx);
        self.preview = Some(ProfilePreview {
            previous,
            applied: profile,
        });
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
        self.update_preview(cx);
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
                this.delegate.update_preview(cx);
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
        let Some(profile_name) = self.selected_profile_name().cloned() else {
            return;
        };
        self.preview = None;
        Self::apply_profile(profile_name, cx);
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
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    div()
                        .debug_selector(|| "settings-profile-live-preview".into())
                        .child(
                            Button::new("settings-profile-live-preview", "Live Preview")
                                .when(self.live_preview, |button| button.color(Color::Accent))
                                .on_click(cx.listener(|picker, _, _, cx| {
                                    let delegate = &mut picker.delegate;
                                    delegate.live_preview = !delegate.live_preview;
                                    if delegate.live_preview {
                                        delegate.update_preview(cx);
                                    } else {
                                        delegate.restore_preview(cx);
                                    }
                                    cx.notify();
                                })),
                        ),
                )
                .into_any_element(),
        )
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
                .child(
                    div()
                        .debug_selector(|| format!("settings-profile-row-{ix}"))
                        .child(HighlightedLabel::new(
                            display_name(profile_name),
                            mat.positions.clone(),
                        )),
                ),
        )
    }
}

fn display_name(profile_name: &Option<String>) -> String {
    profile_name.clone().unwrap_or("Disabled".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use menu::{Cancel, Confirm, SelectNext};
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

            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });

        cx.dispatch_action(SelectNext);
        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.selected_index, 1);
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });
        cx.dispatch_action(Cancel);

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        picker.update_in(cx, |picker, window, cx| {
            picker.set_query("Demo Videos", window, cx)
        });
        cx.run_until_parked();
        picker.read_with(cx, |picker, cx| {
            assert_eq!(picker.delegate.matches.len(), 1);
            assert_eq!(picker.delegate.matches[0].string, demo_videos_profile_name);
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });
        cx.dispatch_action(Confirm);
        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|name| name.0.as_str()),
                Some(demo_videos_profile_name.as_str())
            );
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        picker.read_with(cx, |picker, _| {
            assert_eq!(picker.delegate.selected_index, 2)
        });
        picker.update_in(cx, |picker, window, cx| {
            picker.set_query("zzzzzz", window, cx)
        });
        cx.run_until_parked();
        picker.read_with(cx, |picker, _| assert!(picker.delegate.matches.is_empty()));
        cx.dispatch_action(Confirm);
        active_settings_profile_picker(&workspace, cx);
        cx.update(|_, cx| {
            assert_eq!(
                cx.try_global::<ActiveSettingsProfileName>()
                    .map(|name| name.0.as_str()),
                Some(demo_videos_profile_name.as_str())
            );
        });

        picker.update_in(cx, |picker, window, cx| picker.set_query("", window, cx));
        cx.run_until_parked();
        cx.dispatch_action(Confirm);
        cx.update(|_, cx| {
            assert_eq!(cx.try_global::<ActiveSettingsProfileName>(), None);
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(10.0));
        });
    }

    #[cfg(debug_assertions)]
    #[gpui::test]
    async fn test_settings_profile_selector_mouse_controls(cx: &mut TestAppContext) {
        let (workspace, cx) =
            init_test(json!({"profiles": {"Preview": {"settings": {}}}}), cx).await;
        let assert_active = |cx: &mut VisualTestContext, expected| {
            cx.update(|_, cx| {
                assert_eq!(
                    SettingsProfileSelectorDelegate::active_profile(cx).as_deref(),
                    expected
                );
            });
        };
        let click_preview = |cx: &mut VisualTestContext| {
            cx.run_until_parked();
            let button = cx
                .debug_bounds("settings-profile-live-preview")
                .expect("preview button bounds");
            cx.simulate_click(button.center(), gpui::Modifiers::none());
        };

        cx.dispatch_action(settings_profile_selector::Toggle);
        cx.dispatch_action(SelectNext);
        assert_active(cx, None);
        click_preview(cx);
        assert_active(cx, Some("Preview"));
        click_preview(cx);
        assert_active(cx, None);
        click_preview(cx);
        assert_active(cx, Some("Preview"));
        cx.run_until_parked();
        cx.simulate_click(gpui::point(px(5.), px(200.)), gpui::Modifiers::none());
        assert_active(cx, None);
        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace
                    .active_modal::<SettingsProfileSelector>(cx)
                    .is_none()
            );
        });

        cx.dispatch_action(settings_profile_selector::Toggle);
        cx.run_until_parked();
        let row = cx
            .debug_bounds("settings-profile-row-1")
            .expect("profile row bounds");
        cx.simulate_click(row.center(), gpui::Modifiers::none());
        assert_active(cx, Some("Preview"));
    }

    #[gpui::test]
    async fn test_preview_rollback_on_dismiss(cx: &mut TestAppContext) {
        let (workspace, cx) = init_test(
            json!({"profiles": {"Preview": {"settings": {}}, "External": {"settings": {}}}}),
            cx,
        )
        .await;

        for (external_change, confirm, expected) in [
            (false, false, None),
            (true, false, Some("External")),
            (false, true, Some("Preview")),
        ] {
            cx.update(|_, cx| SettingsProfileSelectorDelegate::apply_profile(None, cx));
            cx.dispatch_action(settings_profile_selector::Toggle);
            let picker = active_settings_profile_picker(&workspace, cx);
            picker.update(cx, |picker, _| picker.delegate.live_preview = true);
            cx.dispatch_action(SelectNext);
            cx.update(|_, cx| {
                assert_eq!(
                    SettingsProfileSelectorDelegate::active_profile(cx).as_deref(),
                    Some("Preview")
                );
                if external_change {
                    cx.set_global(ActiveSettingsProfileName("External".into()));
                }
            });

            if confirm {
                cx.dispatch_action(Confirm);
            } else if external_change {
                cx.dispatch_action(Cancel);
            } else {
                cx.dispatch_action(settings_profile_selector::Toggle);
            }

            workspace.read_with(cx, |workspace, cx| {
                assert!(
                    workspace
                        .active_modal::<SettingsProfileSelector>(cx)
                        .is_none()
                );
                assert_eq!(
                    SettingsProfileSelectorDelegate::active_profile(cx).as_deref(),
                    expected
                );
            });
        }
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

        // Select "Explicit User" (index 1) — profile applies on top of user settings.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .selected_profile_name()
                    .and_then(Option::as_deref),
                Some("Explicit User")
            );
        });
        cx.dispatch_action(Confirm);
        cx.update(|_, cx| {
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(20.0));
        });

        // Select "Implicit User" (index 2) — no base specified, same behavior.
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .selected_profile_name()
                    .and_then(Option::as_deref),
                Some("Implicit User")
            );
        });
        cx.dispatch_action(Confirm);
        cx.update(|_, cx| {
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

        // "Clean Slate" has base: "default" with no settings overrides,
        // so we get the factory default (15), not the user's value (10).
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .selected_profile_name()
                    .and_then(Option::as_deref),
                Some("Clean Slate")
            );
        });
        cx.dispatch_action(Confirm);
        cx.update(|_, cx| {
            assert_eq!(ThemeSettings::get_global(cx).buffer_font_size(cx), px(15.0));
        });
        cx.dispatch_action(settings_profile_selector::Toggle);
        let picker = active_settings_profile_picker(&workspace, cx);

        // "Custom on Defaults" has base: "default" with buffer_font_size: 30,
        // so the profile's override (30) applies on top of the factory default,
        // not on top of the user's value (10).
        cx.dispatch_action(SelectNext);

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .selected_profile_name()
                    .and_then(Option::as_deref),
                Some("Custom on Defaults")
            );
        });

        cx.dispatch_action(Confirm);

        cx.update(|_, cx| {
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
}
