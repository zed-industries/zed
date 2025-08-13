use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::{AgentModelInfo, AgentModelList, AgentModelSelector};
use agent_client_protocol as acp;
use anyhow::Result;
use collections::IndexMap;
use futures::FutureExt;
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{Action, AsyncWindowContext, BackgroundExecutor, DismissEvent, Task, WeakEntity};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use ui::{
    AnyElement, App, Context, IntoElement, ListItem, ListItemSpacing, SharedString, Window,
    prelude::*, rems,
};
use util::ResultExt;

pub type AcpModelSelector = Picker<AcpModelPickerDelegate>;

pub fn acp_model_selector(
    session_id: acp::SessionId,
    selector: Rc<dyn AgentModelSelector>,
    window: &mut Window,
    cx: &mut Context<AcpModelSelector>,
) -> AcpModelSelector {
    let delegate = AcpModelPickerDelegate::new(session_id, selector, window, cx);
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
    session_id: acp::SessionId,
    selector: Rc<dyn AgentModelSelector>,
    filtered_entries: Vec<AcpModelPickerEntry>,
    models: Option<AgentModelList>,
    selected_index: usize,
    selected_model: Option<AgentModelInfo>,
    _refresh_models_task: Task<()>,
}

impl AcpModelPickerDelegate {
    fn new(
        session_id: acp::SessionId,
        selector: Rc<dyn AgentModelSelector>,
        window: &mut Window,
        cx: &mut Context<AcpModelSelector>,
    ) -> Self {
        let mut rx = selector.watch(cx);
        let refresh_models_task = cx.spawn_in(window, {
            let session_id = session_id.clone();
            async move |this, cx| {
                async fn refresh(
                    this: &WeakEntity<Picker<AcpModelPickerDelegate>>,
                    session_id: &acp::SessionId,
                    cx: &mut AsyncWindowContext,
                ) -> Result<()> {
                    let (models_task, selected_model_task) = this.update(cx, |this, cx| {
                        (
                            this.delegate.selector.list_models(cx),
                            this.delegate.selector.selected_model(session_id, cx),
                        )
                    })?;

                    let (models, selected_model) = futures::join!(models_task, selected_model_task);

                    this.update_in(cx, |this, window, cx| {
                        this.delegate.models = models.ok();
                        this.delegate.selected_model = selected_model.ok();
                        this.delegate.update_matches(this.query(cx), window, cx)
                    })?
                    .await;

                    Ok(())
                }

                refresh(&this, &session_id, cx).await.log_err();
                while let Ok(()) = rx.recv().await {
                    refresh(&this, &session_id, cx).await.log_err();
                }
            }
        });

        Self {
            session_id,
            selector,
            filtered_entries: Vec::new(),
            models: None,
            selected_model: None,
            selected_index: 0,
            _refresh_models_task: refresh_models_task,
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
            self.selector
                .select_model(self.session_id.clone(), model_info.id.clone(), cx)
                .detach_and_log_err(cx);
            self.selected_model = Some(model_info.clone());
            let current_index = self.selected_index;
            self.set_selected_index(current_index, window, cx);

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
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

                let model_icon_color = if is_selected {
                    Color::Accent
                } else {
                    Color::Muted
                };

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .start_slot::<Icon>(model_info.icon.map(|icon| {
                            Icon::new(icon)
                                .color(model_icon_color)
                                .size(IconSize::Small)
                        }))
                        .child(
                            h_flex()
                                .w_full()
                                .pl_0p5()
                                .gap_1p5()
                                .w(px(240.))
                                .child(Label::new(model_info.name.clone()).truncate()),
                        )
                        .end_slot(div().pr_3().when(is_selected, |this| {
                            this.child(
                                Icon::new(IconName::Check)
                                    .color(Color::Accent)
                                    .size(IconSize::Small),
                            )
                        }))
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .gap_4()
                .justify_between()
                .child(
                    Button::new("configure", "Configure")
                        .icon(IconName::Settings)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(
                                zed_actions::agent::OpenSettings.boxed_clone(),
                                cx,
                            );
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
            &query,
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
                            id: acp_thread::AgentModelId(model.to_string().into()),
                            name: model.to_string().into(),
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
