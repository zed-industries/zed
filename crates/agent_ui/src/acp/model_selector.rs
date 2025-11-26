use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::{AgentModelInfo, AgentModelList, AgentModelSelector};
use agent_servers::AgentServer;
use anyhow::Result;
use collections::IndexMap;
use fs::Fs;
use futures::FutureExt;
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{
    Action, AsyncWindowContext, BackgroundExecutor, DismissEvent, FocusHandle, Task, WeakEntity,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use ui::{
    DocumentationAside, DocumentationEdge, DocumentationSide, IntoElement, KeyBinding, ListItem,
    ListItemSpacing, prelude::*,
};
use util::ResultExt;
use zed_actions::agent::OpenSettings;

use crate::ui::HoldForDefault;

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
    Model(AgentModelInfo),
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
    _refresh_models_task: Task<()>,
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

        Self {
            selector,
            agent_server,
            fs,
            filtered_entries: Vec::new(),
            models: None,
            selected_model: None,
            selected_index: 0,
            selected_description: None,
            _refresh_models_task: refresh_models_task,
            focus_handle,
        }
    }

    pub fn active_model(&self) -> Option<&AgentModelInfo> {
        self.selected_model.as_ref()
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
            Some(AcpModelPickerEntry::Model(_)) => true,
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
                    info_list_to_picker_entries(filtered_models).collect();
                // Finds the currently selected model in the list
                let new_index = this
                    .delegate
                    .selected_model
                    .as_ref()
                    .and_then(|selected| {
                        this.delegate.filtered_entries.iter().position(|entry| {
                            if let AcpModelPickerEntry::Model(model_info) = entry {
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
        if let Some(AcpModelPickerEntry::Model(model_info)) =
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
            AcpModelPickerEntry::Separator(title) => Some(
                div()
                    .px_2()
                    .pb_1()
                    .when(ix > 1, |this| {
                        this.mt_1()
                            .pt_2()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        Label::new(title)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            ),
            AcpModelPickerEntry::Model(model_info) => {
                let is_selected = Some(model_info) == self.selected_model.as_ref();
                let default_model = self.agent_server.default_model(cx);
                let is_default = default_model.as_ref() == Some(&model_info.id);

                let model_icon_color = if is_selected {
                    Color::Accent
                } else {
                    Color::Muted
                };

                Some(
                    div()
                        .id(("model-picker-menu-child", ix))
                        .when_some(model_info.description.clone(), |this, description| {
                            this
                                .on_hover(cx.listener(move |menu, hovered, _, cx| {
                                    if *hovered {
                                        menu.delegate.selected_description = Some((ix, description.clone(), is_default));
                                    } else if matches!(menu.delegate.selected_description, Some((id, _, _)) if id == ix) {
                                        menu.delegate.selected_description = None;
                                    }
                                    cx.notify();
                                }))
                        })
                        .child(
                            ListItem::new(ix)
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .toggle_state(selected)
                                .child(
                                    h_flex()
                                        .w_full()
                                        .gap_1p5()
                                        .when_some(model_info.icon, |this, icon| {
                                            this.child(
                                                Icon::new(icon)
                                                    .color(model_icon_color)
                                                    .size(IconSize::Small)
                                            )
                                        })
                                        .child(Label::new(model_info.name.clone()).truncate()),
                                )
                                .end_slot(div().pr_3().when(is_selected, |this| {
                                    this.child(
                                        Icon::new(IconName::Check)
                                            .color(Color::Accent)
                                            .size(IconSize::Small),
                                    )
                                })),
                        )
                        .into_any_element()
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
                    DocumentationEdge::Top,
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

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        if !self.selector.should_render_footer() {
            return None;
        }

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("configure", "Configure")
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .key_binding(
                            KeyBinding::for_action_in(&OpenSettings, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(OpenSettings.boxed_clone(), cx);
                        }),
                )
                .into_any(),
        )
    }
}

fn info_list_to_picker_entries(
    model_list: AgentModelList,
) -> impl Iterator<Item = AcpModelPickerEntry> {
    match model_list {
        AgentModelList::Flat(list) => {
            itertools::Either::Left(list.into_iter().map(AcpModelPickerEntry::Model))
        }
        AgentModelList::Grouped(index_map) => {
            itertools::Either::Right(index_map.into_iter().flat_map(|(group_name, models)| {
                std::iter::once(AcpModelPickerEntry::Separator(group_name.0))
                    .chain(models.into_iter().map(AcpModelPickerEntry::Model))
            }))
        }
    }
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
            .map(|(ix, model)| {
                StringMatchCandidate::new(ix, &format!("{}/{}", model.id, model.name))
            })
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
                            id: acp::ModelId(model.to_string().into()),
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
}
