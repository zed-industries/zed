use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::{AgentModelIcon, AgentModelInfo, AgentModelList, AgentModelSelector};
use agent_client_protocol::ModelId;
use agent_servers::AgentServer;
use anyhow::Result;
use collections::{HashSet, IndexMap};
use fs::Fs;
use futures::FutureExt;
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{
    Action, AsyncWindowContext, BackgroundExecutor, DismissEvent, FocusHandle, Subscription, Task,
    WeakEntity,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::SettingsStore;
use ui::{DocumentationAside, DocumentationSide, IntoElement, prelude::*};
use util::ResultExt;
use zed_actions::agent::OpenSettings;

use crate::ui::{HoldForDefault, ModelSelectorFooter, ModelSelectorHeader, ModelSelectorListItem};

pub type AcpModelSelector = Picker<AcpModelPickerDelegate>;

pub fn acp_model_selector(
    selector: Rc<dyn AgentModelSelector>,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    window: &mut Window,
    cx: &mut Context<AcpModelSelector>,
) -> AcpModelSelector {
    let delegate =
        AcpModelPickerDelegate::new(selector, agent_server, fs, focus_handle, window, cx);
    Picker::list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems(20.))
        .max_height(Some(rems(20.).into()))
}

enum AcpModelPickerEntry {
    Separator(SharedString),
    Model(AgentModelInfo, bool),
}

pub struct AcpModelPickerDelegate {
    selector: Rc<dyn AgentModelSelector>,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    filtered_entries: Vec<AcpModelPickerEntry>,
    models: Option<AgentModelList>,
    selected_index: usize,
    selected_description: Option<(usize, SharedString, bool)>,
    selected_model: Option<AgentModelInfo>,
    favorites: HashSet<ModelId>,
    _refresh_models_task: Task<()>,
    _settings_subscription: Subscription,
    focus_handle: FocusHandle,
}

impl AcpModelPickerDelegate {
    fn new(
        selector: Rc<dyn AgentModelSelector>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<AcpModelSelector>,
    ) -> Self {
        let rx = selector.watch(cx);
        let refresh_models_task = {
            cx.spawn_in(window, {
                async move |this, cx| {
                    async fn refresh(
                        this: &WeakEntity<Picker<AcpModelPickerDelegate>>,
                        cx: &mut AsyncWindowContext,
                    ) -> Result<()> {
                        let (models_task, selected_model_task) = this.update(cx, |this, cx| {
                            (
                                this.delegate.selector.list_models(cx),
                                this.delegate.selector.selected_model(cx),
                            )
                        })?;

                        let (models, selected_model) =
                            futures::join!(models_task, selected_model_task);

                        this.update_in(cx, |this, window, cx| {
                            this.delegate.models = models.ok();
                            this.delegate.selected_model = selected_model.ok();
                            this.refresh(window, cx)
                        })
                    }

                    refresh(&this, cx).await.log_err();
                    if let Some(mut rx) = rx {
                        while let Ok(()) = rx.recv().await {
                            refresh(&this, cx).await.log_err();
                        }
                    }
                }
            })
        };

        let agent_server_for_subscription = agent_server.clone();
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |picker, window, cx| {
                // Only refresh if the favorites actually changed to avoid redundant work
                // when other settings are modified (e.g., user editing settings.json)
                let new_favorites = agent_server_for_subscription.favorite_model_ids(cx);
                if new_favorites != picker.delegate.favorites {
                    picker.delegate.favorites = new_favorites;
                    picker.refresh(window, cx);
                }
            });
        let favorites = agent_server.favorite_model_ids(cx);

        Self {
            selector,
            agent_server,
            fs,
            filtered_entries: Vec::new(),
            models: None,
            selected_model: None,
            selected_index: 0,
            selected_description: None,
            favorites,
            _refresh_models_task: refresh_models_task,
            _settings_subscription: settings_subscription,
            focus_handle,
        }
    }

    pub fn active_model(&self) -> Option<&AgentModelInfo> {
        self.selected_model.as_ref()
    }

    pub fn favorites_count(&self) -> usize {
        self.favorites.len()
    }

    pub fn cycle_favorite_models(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.favorites.is_empty() {
            return;
        }

        let Some(models) = &self.models else {
            return;
        };

        let all_models: Vec<&AgentModelInfo> = match models {
            AgentModelList::Flat(list) => list.iter().collect(),
            AgentModelList::Grouped(index_map) => index_map.values().flatten().collect(),
        };

        let favorite_models: Vec<_> = all_models
            .into_iter()
            .filter(|model| self.favorites.contains(&model.id))
            .unique_by(|model| &model.id)
            .collect();

        if favorite_models.is_empty() {
            return;
        }

        let current_id = self.selected_model.as_ref().map(|m| &m.id);

        let current_index_in_favorites = current_id
            .and_then(|id| favorite_models.iter().position(|m| &m.id == id))
            .unwrap_or(usize::MAX);

        let next_index = if current_index_in_favorites == usize::MAX {
            0
        } else {
            (current_index_in_favorites + 1) % favorite_models.len()
        };

        let next_model = favorite_models[next_index].clone();

        self.selector
            .select_model(next_model.id.clone(), cx)
            .detach_and_log_err(cx);

        self.selected_model = Some(next_model);

        // Keep the picker selection aligned with the newly-selected model
        if let Some(new_index) = self.filtered_entries.iter().position(|entry| {
            matches!(entry, AcpModelPickerEntry::Model(model_info, _) if self.selected_model.as_ref().is_some_and(|selected| model_info.id == selected.id))
        }) {
            self.set_selected_index(new_index, window, cx);
        } else {
            cx.notify();
        }
    }
}

impl PickerDelegate for AcpModelPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_entries.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        match self.filtered_entries.get(ix) {
            Some(AcpModelPickerEntry::Model(_, _)) => true,
            Some(AcpModelPickerEntry::Separator(_)) | None => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a model…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let favorites = self.favorites.clone();

        cx.spawn_in(window, async move |this, cx| {
            let filtered_models = match this
                .read_with(cx, |this, cx| {
                    this.delegate.models.clone().map(move |models| {
                        fuzzy_search(models, query, cx.background_executor().clone())
                    })
                })
                .ok()
                .flatten()
            {
                Some(task) => task.await,
                None => AgentModelList::Flat(vec![]),
            };

            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_entries =
                    info_list_to_picker_entries(filtered_models, &favorites);
                // Finds the currently selected model in the list
                let new_index = this
                    .delegate
                    .selected_model
                    .as_ref()
                    .and_then(|selected| {
                        this.delegate.filtered_entries.iter().position(|entry| {
                            if let AcpModelPickerEntry::Model(model_info, _) = entry {
                                model_info.id == selected.id
                            } else {
                                false
                            }
                        })
                    })
                    .unwrap_or(0);
                this.set_selected_index(new_index, Some(picker::Direction::Down), true, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(AcpModelPickerEntry::Model(model_info, _)) =
            self.filtered_entries.get(self.selected_index)
        {
            if window.modifiers().secondary() {
                let default_model = self.agent_server.default_model(cx);
                let is_default = default_model.as_ref() == Some(&model_info.id);

                self.agent_server.set_default_model(
                    if is_default {
                        None
                    } else {
                        Some(model_info.id.clone())
                    },
                    self.fs.clone(),
                    cx,
                );
            }

            self.selector
                .select_model(model_info.id.clone(), cx)
                .detach_and_log_err(cx);
            self.selected_model = Some(model_info.clone());
            let current_index = self.selected_index;
            self.set_selected_index(current_index, window, cx);

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.defer_in(window, |picker, window, cx| {
            picker.set_query("", window, cx);
        });
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            AcpModelPickerEntry::Separator(title) => {
                Some(ModelSelectorHeader::new(title, ix > 1).into_any_element())
            }
            AcpModelPickerEntry::Model(model_info, is_favorite) => {
                let is_selected = Some(model_info) == self.selected_model.as_ref();
                let default_model = self.agent_server.default_model(cx);
                let is_default = default_model.as_ref() == Some(&model_info.id);

                let is_favorite = *is_favorite;
                let handle_action_click = {
                    let model_id = model_info.id.clone();
                    let fs = self.fs.clone();
                    let agent_server = self.agent_server.clone();

                    cx.listener(move |_, _, _, cx| {
                        agent_server.toggle_favorite_model(
                            model_id.clone(),
                            !is_favorite,
                            fs.clone(),
                            cx,
                        );
                    })
                };

                Some(
                    div()
                        .id(("model-picker-menu-child", ix))
                        .when_some(model_info.description.clone(), |this, description| {
                            this.on_hover(cx.listener(move |menu, hovered, _, cx| {
                                if *hovered {
                                    menu.delegate.selected_description =
                                        Some((ix, description.clone(), is_default));
                                } else if matches!(menu.delegate.selected_description, Some((id, _, _)) if id == ix) {
                                    menu.delegate.selected_description = None;
                                }
                                cx.notify();
                            }))
                        })
                        .child(
                            ModelSelectorListItem::new(ix, model_info.name.clone())
                                .map(|this| match &model_info.icon {
                                    Some(AgentModelIcon::Path(path)) => this.icon_path(path.clone()),
                                    Some(AgentModelIcon::Named(icon)) => this.icon(*icon),
                                    None => this,
                                })
                                .is_selected(is_selected)
                                .is_focused(selected)
                                .is_favorite(is_favorite)
                                .on_toggle_favorite(handle_action_click),
                        )
                        .into_any_element(),
                )
            }
        }
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<ui::DocumentationAside> {
        self.selected_description
            .as_ref()
            .map(|(_, description, is_default)| {
                let description = description.clone();
                let is_default = *is_default;

                DocumentationAside::new(
                    DocumentationSide::Left,
                    Rc::new(move |_| {
                        v_flex()
                            .gap_1()
                            .child(Label::new(description.clone()))
                            .child(HoldForDefault::new(is_default))
                            .into_any_element()
                    }),
                )
            })
    }

    fn documentation_aside_index(&self) -> Option<usize> {
        self.selected_description.as_ref().map(|(ix, _, _)| *ix)
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        if !self.selector.should_render_footer() {
            return None;
        }

        Some(ModelSelectorFooter::new(OpenSettings.boxed_clone(), focus_handle).into_any_element())
    }
}

fn info_list_to_picker_entries(
    model_list: AgentModelList,
    favorites: &HashSet<ModelId>,
) -> Vec<AcpModelPickerEntry> {
    let mut entries = Vec::new();

    let all_models: Vec<_> = match &model_list {
        AgentModelList::Flat(list) => list.iter().collect(),
        AgentModelList::Grouped(index_map) => index_map.values().flatten().collect(),
    };

    let favorite_models: Vec<_> = all_models
        .iter()
        .filter(|m| favorites.contains(&m.id))
        .unique_by(|m| &m.id)
        .collect();

    let has_favorites = !favorite_models.is_empty();
    if has_favorites {
        entries.push(AcpModelPickerEntry::Separator("Favorite".into()));
        for model in favorite_models {
            entries.push(AcpModelPickerEntry::Model((*model).clone(), true));
        }
    }

    match model_list {
        AgentModelList::Flat(list) => {
            if has_favorites {
                entries.push(AcpModelPickerEntry::Separator("All".into()));
            }
            for model in list {
                let is_favorite = favorites.contains(&model.id);
                entries.push(AcpModelPickerEntry::Model(model, is_favorite));
            }
        }
        AgentModelList::Grouped(index_map) => {
            for (group_name, models) in index_map {
                entries.push(AcpModelPickerEntry::Separator(group_name.0));
                for model in models {
                    let is_favorite = favorites.contains(&model.id);
                    entries.push(AcpModelPickerEntry::Model(model, is_favorite));
                }
            }
        }
    }

    entries
}

async fn fuzzy_search(
    model_list: AgentModelList,
    query: String,
    executor: BackgroundExecutor,
) -> AgentModelList {
    async fn fuzzy_search_list(
        model_list: Vec<AgentModelInfo>,
        query: &str,
        executor: BackgroundExecutor,
    ) -> Vec<AgentModelInfo> {
        let candidates = model_list
            .iter()
            .enumerate()
            .map(|(ix, model)| StringMatchCandidate::new(ix, model.name.as_ref()))
            .collect::<Vec<_>>();
        let mut matches = match_strings(
            &candidates,
            query,
            false,
            true,
            100,
            &Default::default(),
            executor,
        )
        .await;

        matches.sort_unstable_by_key(|mat| {
            let candidate = &candidates[mat.candidate_id];
            (Reverse(OrderedFloat(mat.score)), candidate.id)
        });

        matches
            .into_iter()
            .map(|mat| model_list[mat.candidate_id].clone())
            .collect()
    }

    match model_list {
        AgentModelList::Flat(model_list) => {
            AgentModelList::Flat(fuzzy_search_list(model_list, &query, executor).await)
        }
        AgentModelList::Grouped(index_map) => {
            let groups =
                futures::future::join_all(index_map.into_iter().map(|(group_name, models)| {
                    fuzzy_search_list(models, &query, executor.clone())
                        .map(|results| (group_name, results))
                }))
                .await;
            AgentModelList::Grouped(IndexMap::from_iter(
                groups
                    .into_iter()
                    .filter(|(_, results)| !results.is_empty()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use agent_client_protocol as acp;
    use gpui::TestAppContext;

    use super::*;

    fn create_model_list(grouped_models: Vec<(&str, Vec<&str>)>) -> AgentModelList {
        AgentModelList::Grouped(IndexMap::from_iter(grouped_models.into_iter().map(
            |(group, models)| {
                (
                    acp_thread::AgentModelGroupName(group.to_string().into()),
                    models
                        .into_iter()
                        .map(|model| acp_thread::AgentModelInfo {
                            id: acp::ModelId::new(model.to_string()),
                            name: model.to_string().into(),
                            description: None,
                            icon: None,
                        })
                        .collect::<Vec<_>>(),
                )
            },
        )))
    }

    fn assert_models_eq(result: AgentModelList, expected: Vec<(&str, Vec<&str>)>) {
        let AgentModelList::Grouped(groups) = result else {
            panic!("Expected LanguageModelInfoList::Grouped, got {:?}", result);
        };

        assert_eq!(
            groups.len(),
            expected.len(),
            "Number of groups doesn't match"
        );

        for (i, (expected_group, expected_models)) in expected.iter().enumerate() {
            let (actual_group, actual_models) = groups.get_index(i).unwrap();
            assert_eq!(
                actual_group.0.as_ref(),
                *expected_group,
                "Group at position {} doesn't match expected group",
                i
            );
            assert_eq!(
                actual_models.len(),
                expected_models.len(),
                "Number of models in group {} doesn't match",
                expected_group
            );

            for (j, expected_model_name) in expected_models.iter().enumerate() {
                assert_eq!(
                    actual_models[j].name, *expected_model_name,
                    "Model at position {} in group {} doesn't match expected model",
                    j, expected_group
                );
            }
        }
    }

    fn create_favorites(models: Vec<&str>) -> HashSet<ModelId> {
        models
            .into_iter()
            .map(|m| ModelId::new(m.to_string()))
            .collect()
    }

    fn get_entry_model_ids(entries: &[AcpModelPickerEntry]) -> Vec<&str> {
        entries
            .iter()
            .filter_map(|entry| match entry {
                AcpModelPickerEntry::Model(info, _) => Some(info.id.0.as_ref()),
                _ => None,
            })
            .collect()
    }

    fn get_entry_labels(entries: &[AcpModelPickerEntry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| match entry {
                AcpModelPickerEntry::Model(info, _) => info.id.0.as_ref(),
                AcpModelPickerEntry::Separator(s) => &s,
            })
            .collect()
    }

    #[gpui::test]
    async fn test_fuzzy_match(cx: &mut TestAppContext) {
        let models = create_model_list(vec![
            (
                "zed",
                vec![
                    "Claude 3.7 Sonnet",
                    "Claude 3.7 Sonnet Thinking",
                    "gpt-4.1",
                    "gpt-4.1-nano",
                ],
            ),
            ("openai", vec!["gpt-3.5-turbo", "gpt-4.1", "gpt-4.1-nano"]),
            ("ollama", vec!["mistral", "deepseek"]),
        ]);

        // Results should preserve models order whenever possible.
        // In the case below, `zed/gpt-4.1` and `openai/gpt-4.1` have identical
        // similarity scores, but `zed/gpt-4.1` was higher in the models list,
        // so it should appear first in the results.
        let results = fuzzy_search(models.clone(), "41".into(), cx.executor()).await;
        assert_models_eq(
            results,
            vec![
                ("zed", vec!["gpt-4.1", "gpt-4.1-nano"]),
                ("openai", vec!["gpt-4.1", "gpt-4.1-nano"]),
            ],
        );

        // Fuzzy search
        let results = fuzzy_search(models.clone(), "4n".into(), cx.executor()).await;
        assert_models_eq(
            results,
            vec![
                ("zed", vec!["gpt-4.1-nano"]),
                ("openai", vec!["gpt-4.1-nano"]),
            ],
        );
    }

    #[gpui::test]
    fn test_favorites_section_appears_when_favorites_exist(_cx: &mut TestAppContext) {
        let models = create_model_list(vec![
            ("zed", vec!["zed/claude", "zed/gemini"]),
            ("openai", vec!["openai/gpt-5"]),
        ]);
        let favorites = create_favorites(vec!["zed/gemini"]);

        let entries = info_list_to_picker_entries(models, &favorites);

        assert!(matches!(
            entries.first(),
            Some(AcpModelPickerEntry::Separator(s)) if s == "Favorite"
        ));

        let model_ids = get_entry_model_ids(&entries);
        assert_eq!(model_ids[0], "zed/gemini");
    }

    #[gpui::test]
    fn test_no_favorites_section_when_no_favorites(_cx: &mut TestAppContext) {
        let models = create_model_list(vec![("zed", vec!["zed/claude", "zed/gemini"])]);
        let favorites = create_favorites(vec![]);

        let entries = info_list_to_picker_entries(models, &favorites);

        assert!(matches!(
            entries.first(),
            Some(AcpModelPickerEntry::Separator(s)) if s == "zed"
        ));
    }

    #[gpui::test]
    fn test_models_have_correct_actions(_cx: &mut TestAppContext) {
        let models = create_model_list(vec![
            ("zed", vec!["zed/claude", "zed/gemini"]),
            ("openai", vec!["openai/gpt-5"]),
        ]);
        let favorites = create_favorites(vec!["zed/claude"]);

        let entries = info_list_to_picker_entries(models, &favorites);

        for entry in &entries {
            if let AcpModelPickerEntry::Model(info, is_favorite) = entry {
                if info.id.0.as_ref() == "zed/claude" {
                    assert!(is_favorite, "zed/claude should be a favorite");
                } else {
                    assert!(!is_favorite, "{} should not be a favorite", info.id.0);
                }
            }
        }
    }

    #[gpui::test]
    fn test_favorites_appear_in_both_sections(_cx: &mut TestAppContext) {
        let models = create_model_list(vec![
            ("zed", vec!["zed/claude", "zed/gemini"]),
            ("openai", vec!["openai/gpt-5", "openai/gpt-4"]),
        ]);
        let favorites = create_favorites(vec!["zed/gemini", "openai/gpt-5"]);

        let entries = info_list_to_picker_entries(models, &favorites);
        let model_ids = get_entry_model_ids(&entries);

        assert_eq!(model_ids[0], "zed/gemini");
        assert_eq!(model_ids[1], "openai/gpt-5");

        assert!(model_ids[2..].contains(&"zed/gemini"));
        assert!(model_ids[2..].contains(&"openai/gpt-5"));
    }

    #[gpui::test]
    fn test_favorites_are_not_duplicated_when_repeated_in_other_sections(_cx: &mut TestAppContext) {
        let models = create_model_list(vec![
            ("Recommended", vec!["zed/claude", "anthropic/claude"]),
            ("Zed", vec!["zed/claude", "zed/gpt-5"]),
            ("Antropic", vec!["anthropic/claude"]),
            ("OpenAI", vec!["openai/gpt-5"]),
        ]);

        let favorites = create_favorites(vec!["zed/claude"]);

        let entries = info_list_to_picker_entries(models, &favorites);
        let labels = get_entry_labels(&entries);

        assert_eq!(
            labels,
            vec![
                "Favorite",
                "zed/claude",
                "Recommended",
                "zed/claude",
                "anthropic/claude",
                "Zed",
                "zed/claude",
                "zed/gpt-5",
                "Antropic",
                "anthropic/claude",
                "OpenAI",
                "openai/gpt-5"
            ]
        );
    }

    #[gpui::test]
    fn test_flat_model_list_with_favorites(_cx: &mut TestAppContext) {
        let models = AgentModelList::Flat(vec![
            acp_thread::AgentModelInfo {
                id: acp::ModelId::new("zed/claude".to_string()),
                name: "Claude".into(),
                description: None,
                icon: None,
            },
            acp_thread::AgentModelInfo {
                id: acp::ModelId::new("zed/gemini".to_string()),
                name: "Gemini".into(),
                description: None,
                icon: None,
            },
        ]);
        let favorites = create_favorites(vec!["zed/gemini"]);

        let entries = info_list_to_picker_entries(models, &favorites);

        assert!(matches!(
            entries.first(),
            Some(AcpModelPickerEntry::Separator(s)) if s == "Favorite"
        ));

        assert!(entries.iter().any(|e| matches!(
            e,
            AcpModelPickerEntry::Separator(s) if s == "All"
        )));
    }

    #[gpui::test]
    fn test_favorites_count_returns_correct_count(_cx: &mut TestAppContext) {
        let empty_favorites: HashSet<ModelId> = HashSet::default();
        assert_eq!(empty_favorites.len(), 0);

        let one_favorite = create_favorites(vec!["model-a"]);
        assert_eq!(one_favorite.len(), 1);

        let multiple_favorites = create_favorites(vec!["model-a", "model-b", "model-c"]);
        assert_eq!(multiple_favorites.len(), 3);

        let with_duplicates = create_favorites(vec!["model-a", "model-a", "model-b"]);
        assert_eq!(with_duplicates.len(), 2);
    }

    #[gpui::test]
    fn test_is_favorite_flag_set_correctly_in_entries(_cx: &mut TestAppContext) {
        let models = AgentModelList::Flat(vec![
            acp_thread::AgentModelInfo {
                id: acp::ModelId::new("favorite-model".to_string()),
                name: "Favorite".into(),
                description: None,
                icon: None,
            },
            acp_thread::AgentModelInfo {
                id: acp::ModelId::new("regular-model".to_string()),
                name: "Regular".into(),
                description: None,
                icon: None,
            },
        ]);
        let favorites = create_favorites(vec!["favorite-model"]);

        let entries = info_list_to_picker_entries(models, &favorites);

        for entry in &entries {
            if let AcpModelPickerEntry::Model(info, is_favorite) = entry {
                if info.id.0.as_ref() == "favorite-model" {
                    assert!(*is_favorite, "favorite-model should have is_favorite=true");
                } else if info.id.0.as_ref() == "regular-model" {
                    assert!(!*is_favorite, "regular-model should have is_favorite=false");
                }
            }
        }
    }
}
