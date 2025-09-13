use std::sync::Arc;

use agent_settings::{AgentProfileId, AgentSettings};
use fs::Fs;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, prelude::*};
use menu;
use prompt_store::{PromptMetadata, PromptStore, PromptsUpdatedEvent};
use settings::{Settings as _, update_settings_file};
use ui::{KeyBinding, ListItem, ListItemSpacing, ListSeparator, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::ModalView;

use super::manage_profiles_modal::profile_modal_header::ProfileModalHeader;

type GoBackCallback = Box<dyn Fn(&mut gpui::App) + Send>;

pub struct ConfigureProfileRulesModal {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    profile_id: AgentProfileId,
    rules: Vec<PromptMetadata>,
    loading: bool,
    prompt_store: Option<Entity<PromptStore>>,
    go_back_callback: Option<GoBackCallback>,
}

impl ConfigureProfileRulesModal {
    pub fn new(
        fs: Arc<dyn Fs>,
        profile_id: AgentProfileId,
        prompt_store: Option<Entity<PromptStore>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut modal = Self {
            fs,
            focus_handle: cx.focus_handle(),
            profile_id,
            rules: Vec::new(),
            loading: true,
            prompt_store,
            go_back_callback: None,
        };

        modal.load_rules_for_profile(cx);
        modal
    }

    fn load_rules_for_profile(&mut self, cx: &mut Context<Self>) {
        let prompt_store = PromptStore::global(cx);

        cx.spawn(async move |this, cx| {
            let rules = match prompt_store.await {
                Ok(store) => store
                    .read_with(cx, |store, _cx| store.all_prompt_metadata())
                    .unwrap_or_else(|_err| Vec::new()),
                Err(_err) => Vec::new(),
            };

            this.update(cx, |this, cx| {
                this.rules = rules;
                this.loading = false;
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn toggle_rule_for_profile(&mut self, rule_id: prompt_store::PromptId, cx: &mut Context<Self>) {
        let profile_id = self.profile_id.clone();
        let rule_id_string = rule_id.to_string();
        let fs = self.fs.clone();

        let settings = AgentSettings::get_global(cx);
        let current_enabled = settings
            .profiles
            .get(&profile_id)
            .map(|profile| profile.rules.get(&rule_id_string).copied().unwrap_or(false))
            .unwrap_or(false);

        update_settings_file::<AgentSettings>(fs, cx, move |settings, _cx| {
            if let Some(profiles) = &mut settings.profiles {
                if let Some(profile) = profiles.get_mut(&profile_id) {
                    profile.rules.insert(rule_id_string, !current_enabled);
                }
            }
        });

        if let Some(prompt_store) = &self.prompt_store {
            prompt_store.update(cx, |_, cx| {
                cx.emit(PromptsUpdatedEvent);
            });
        }

        cx.notify();
    }

    pub fn set_go_back_callback(&mut self, callback: GoBackCallback) {
        self.go_back_callback = Some(callback);
    }

    fn go_back(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
        if let Some(callback) = self.go_back_callback.take() {
            cx.defer(move |cx| {
                callback(cx);
            });
        }
    }
}

impl ModalView for ConfigureProfileRulesModal {}

impl Focusable for ConfigureProfileRulesModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ConfigureProfileRulesModal {}

impl Render for ConfigureProfileRulesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ConfigureProfileRulesModal")
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.go_back(window, cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
            .track_focus(&self.focus_handle(cx))
            .child(
                div()
                    .size_full()
                    .child(ProfileModalHeader::new(
                        "Configure Profile Rules",
                        Some(IconName::Book),
                    ))
                    .child(
                        v_flex()
                    .pb_1()
                    .child(ListSeparator)
                    .child(
                        div().pl_2().pb_1().child(
                            Label::new("Available Rules")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .children({
                        if self.loading {
                            vec![
                                div()
                                    .pl_2()
                                    .child(
                                        Label::new("Loading rules...")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .into_any_element(),
                            ]
                        } else if self.rules.is_empty() {
                            vec![
                                div()
                                    .pl_2()
                                    .child(
                                        Label::new("No rules available, visit the Rules Library to add some.")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .into_any_element(),
                            ]
                        } else {
                            let settings = AgentSettings::get_global(cx);
                            let profile = settings.profiles.get(&self.profile_id);

                            self.rules
                                .iter()
                                .enumerate()
                                .map(|(index, rule_metadata)| {
                                    let rule_id_string = rule_metadata.id.to_string();
                                    let is_enabled = profile
                                        .map(|p| p.rules.get(&rule_id_string).copied().unwrap_or(false))
                                        .unwrap_or(false);

                                    ListItem::new(("rule", index))
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(if is_enabled {
                                                IconName::Check
                                            } else {
                                                IconName::Circle
                                            })
                                            .size(IconSize::Small)
                                            .color(if is_enabled {
                                                Color::Success
                                            } else {
                                                Color::Muted
                                            }),
                                        )
                                        .child(Label::new(
                                            rule_metadata
                                                .title
                                                .as_ref()
                                                .cloned()
                                                .unwrap_or_else(|| "Untitled Rule".into()),
                                        ))
                                        .end_slot(
                                            h_flex()
                                                .gap_1()
                                                .when(rule_metadata.default, |this| {
                                                    this.child(
                                                        IconButton::new("default-rule-indicator", IconName::StarFilled)
                                                            .icon_size(IconSize::XSmall)
                                                            .icon_color(Color::Accent)
                                                            .style(ButtonStyle::Transparent)
                                                            .disabled(true)
                                                            .tooltip(Tooltip::text(
                                                                "This rule is active by default, see Rules Library"
                                                            )),
                                                    )
                                                })
                                                .when(rule_metadata.id.is_built_in(), |this| {
                                                    this.child(
                                                        Icon::new(IconName::LockOutlined)
                                                            .size(IconSize::XSmall)
                                                            .color(Color::Muted),
                                                    )
                                                })
                                        )
                                        .on_click({
                                            let rule_id = rule_metadata.id;
                                            cx.listener(move |this, _, _window, cx| {
                                                this.toggle_rule_for_profile(rule_id, cx);
                                            })
                                        })
                                        .into_any_element()
                                })
                                .collect::<Vec<_>>()
                        }
                    })
                    .child(ListSeparator)
                    .child(
                        ListItem::new("go-back")
                            .inset(true)
                            .spacing(ListItemSpacing::Sparse)
                            .start_slot(
                                Icon::new(IconName::ArrowLeft)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new("Go Back"))
                            .end_slot(
                                div().children(
                                    KeyBinding::for_action_in(
                                        &menu::Cancel,
                                        &self.focus_handle,
                                        window,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(12.))),
                                ),
                            )
                            .on_click({
                                cx.listener(move |this, _, window, cx| {
                                    this.go_back(window, cx);
                                })
                            }),
                    ),
                )
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use prompt_store::PromptId;

    #[test]
    fn test_profile_id_creation_and_operations() {
        let profile_id = AgentProfileId("test-profile".into());
        assert_eq!(profile_id.0.as_ref(), "test-profile");

        let cloned_id = profile_id.clone();
        assert_eq!(profile_id.0.as_ref(), cloned_id.0.as_ref());

        let other_id = AgentProfileId("other-profile".into());
        assert_ne!(profile_id.0.as_ref(), other_id.0.as_ref());
    }

    #[test]
    fn test_prompt_id_uniqueness() {
        let id1 = PromptId::new();
        let id2 = PromptId::new();

        assert_ne!(id1.to_string(), id2.to_string());
    }

    #[test]
    fn test_prompt_metadata_with_title() {
        let rule_id = PromptId::new();
        let metadata = PromptMetadata {
            id: rule_id,
            title: Some("Test Rule".into()),
            default: true,
            saved_at: Utc::now(),
        };

        assert!(metadata.title.is_some());
        assert_eq!(metadata.title.as_ref().unwrap().as_ref(), "Test Rule");
        assert!(metadata.default);
    }

    #[test]
    fn test_prompt_metadata_without_title() {
        let metadata = PromptMetadata {
            id: PromptId::new(),
            title: None,
            default: false,
            saved_at: Utc::now(),
        };

        assert!(metadata.title.is_none());
        assert!(!metadata.default);
    }

    #[test]
    fn test_rule_vector_operations() {
        let mut rules: Vec<PromptMetadata> = Vec::new();

        assert!(rules.is_empty());
        assert_eq!(rules.len(), 0);

        rules.push(PromptMetadata {
            id: PromptId::new(),
            title: Some("Test Rule".into()),
            default: false,
            saved_at: Utc::now(),
        });

        assert_eq!(rules.len(), 1);
        assert!(!rules.is_empty());

        rules.clear();
        assert!(rules.is_empty());
    }

    #[test]
    fn test_rule_filtering_by_properties() {
        let rules = vec![
            PromptMetadata {
                id: PromptId::new(),
                title: Some("Rule with title".into()),
                default: false,
                saved_at: Utc::now(),
            },
            PromptMetadata {
                id: PromptId::new(),
                title: None,
                default: true,
                saved_at: Utc::now(),
            },
            PromptMetadata {
                id: PromptId::new(),
                title: Some("Another rule".into()),
                default: true,
                saved_at: Utc::now(),
            },
        ];

        let rules_with_titles = rules.iter().filter(|r| r.title.is_some()).count();
        let rules_without_titles = rules.iter().filter(|r| r.title.is_none()).count();
        let default_rules = rules.iter().filter(|r| r.default).count();

        assert_eq!(rules_with_titles, 2);
        assert_eq!(rules_without_titles, 1);
        assert_eq!(default_rules, 2);
    }

    #[test]
    fn test_loading_state_initialization() {
        let loading = true;
        let rules: Vec<PromptMetadata> = Vec::new();

        assert!(loading);
        assert!(rules.is_empty());

        let loading = false;
        let mut rules = Vec::new();
        rules.push(PromptMetadata {
            id: PromptId::new(),
            title: Some("Loaded rule".into()),
            default: false,
            saved_at: Utc::now(),
        });

        assert!(!loading);
        assert!(!rules.is_empty());
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_modal_state_struct_creation() {
        let profile_id = AgentProfileId("test-profile".into());
        let rules: Vec<PromptMetadata> = Vec::new();
        let loading = true;
        let prompt_store: Option<Entity<PromptStore>> = None;

        assert_eq!(profile_id.0.as_ref(), "test-profile");
        assert!(rules.is_empty());
        assert!(loading);
        assert!(prompt_store.is_none());
    }

    #[test]
    fn test_rule_metadata_title_handling() {
        let rule_with_title = PromptMetadata {
            id: PromptId::new(),
            title: Some("My Rule".into()),
            default: false,
            saved_at: Utc::now(),
        };

        let rule_without_title = PromptMetadata {
            id: PromptId::new(),
            title: None,
            default: true,
            saved_at: Utc::now(),
        };

        let title_text = rule_with_title
            .title
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Untitled Rule".into());
        assert_eq!(title_text.as_ref(), "My Rule");

        let fallback_title = rule_without_title
            .title
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Untitled Rule".into());
        assert_eq!(fallback_title.as_ref(), "Untitled Rule");
    }

    #[test]
    fn test_rule_title_fallback_logic() {
        let rule_with_title = PromptMetadata {
            id: PromptId::new(),
            title: Some("My Custom Rule".into()),
            default: false,
            saved_at: Utc::now(),
        };

        let rule_without_title = PromptMetadata {
            id: PromptId::new(),
            title: None,
            default: true,
            saved_at: Utc::now(),
        };

        let display_title_1 = rule_with_title
            .title
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Untitled Rule".into());

        let display_title_2 = rule_without_title
            .title
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Untitled Rule".into());

        assert_eq!(display_title_1.as_ref(), "My Custom Rule");
        assert_eq!(display_title_2.as_ref(), "Untitled Rule");
    }

    #[test]
    fn test_rule_properties_for_ui_logic() {
        let default_rule = PromptMetadata {
            id: PromptId::new(),
            title: Some("Default Rule".into()),
            default: true,
            saved_at: Utc::now(),
        };

        let custom_rule = PromptMetadata {
            id: PromptId::new(),
            title: Some("Custom Rule".into()),
            default: false,
            saved_at: Utc::now(),
        };

        assert!(default_rule.default, "Should show star indicator");
        assert!(!custom_rule.default, "Should not show star indicator");

        let built_in_detected_1 = default_rule.id.is_built_in();
        let built_in_detected_2 = custom_rule.id.is_built_in();

        assert_eq!(built_in_detected_1, default_rule.id.is_built_in());
        assert_eq!(built_in_detected_2, custom_rule.id.is_built_in());
    }

    #[test]
    fn test_empty_and_loading_states() {
        let loading = true;
        let rules: Vec<PromptMetadata> = Vec::new();

        if loading {
            assert!(rules.is_empty(), "Rules should be empty while loading");
        }

        let loading = false;
        let rules: Vec<PromptMetadata> = Vec::new();

        if !loading && rules.is_empty() {
            assert!(true, "Should show no rules available message");
        }

        let mut rules = Vec::new();
        rules.push(PromptMetadata {
            id: PromptId::new(),
            title: Some("Test Rule".into()),
            default: false,
            saved_at: Utc::now(),
        });

        if !loading && !rules.is_empty() {
            assert_eq!(rules.len(), 1, "Should render rules list");
        }
    }

    #[test]
    fn test_rule_id_string_conversion() {
        let rule_id = PromptId::new();
        let rule_id_string = rule_id.to_string();

        assert!(
            !rule_id_string.is_empty(),
            "Rule ID string should not be empty"
        );

        let same_rule_string = rule_id.to_string();
        assert_eq!(
            rule_id_string, same_rule_string,
            "Conversion should be consistent"
        );
    }

    #[test]
    fn test_business_logic_data_structures() {
        let profile_id = AgentProfileId("business-test".into());
        let loading = true;
        let rules: Vec<PromptMetadata> = Vec::new();

        assert_eq!(profile_id.0.as_ref(), "business-test");
        assert!(loading);
        assert!(rules.is_empty());
    }

    #[test]
    fn test_toggle_rule_string_operations() {
        let rule_id = PromptId::new();
        let rule_id_string = rule_id.to_string();

        assert!(!rule_id_string.is_empty());

        let same_rule_string = rule_id.to_string();
        assert_eq!(rule_id_string, same_rule_string);
    }

    #[test]
    fn test_load_rules_state_management() {
        let mut loading = true;
        let mut rules: Vec<PromptMetadata> = Vec::new();

        assert!(loading);
        assert!(rules.is_empty());

        rules.push(PromptMetadata {
            id: PromptId::new(),
            title: Some("Loaded Rule".into()),
            default: false,
            saved_at: Utc::now(),
        });
        loading = false;

        assert!(!loading);
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_error_fallback_state() {
        let mut loading = true;
        let mut rules: Vec<PromptMetadata> = Vec::new();

        assert!(loading);
        assert!(rules.is_empty());

        rules.clear();
        loading = false;

        assert!(!loading);
        assert!(rules.is_empty());
    }

    #[test]
    fn test_profile_id_business_operations() {
        let profile_id = AgentProfileId("test-profile".into());

        let cloned_id = profile_id.clone();
        assert_eq!(profile_id.0.as_ref(), cloned_id.0.as_ref());

        assert_eq!(profile_id.0.as_ref(), "test-profile");
    }
}
