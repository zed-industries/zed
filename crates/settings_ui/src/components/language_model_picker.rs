use std::rc::Rc;
use std::sync::Arc;

use fuzzy::StringMatch;
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, SharedString, Subscription, Task, Window, px,
};
use itertools::Itertools as _;
use language_model::{LanguageModel, LanguageModelProviderId, LanguageModelRegistry};
use picker::{Picker, PickerDelegate, ScrollBehavior};
use project::{
    AgentId, Project,
    external_agent_models::{
        ExternalAgentModel, ExternalAgentModelCatalog, ExternalAgentModelsState,
        refresh_external_agent_models,
    },
};
use settings::LanguageModelSelection;
use ui::{ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};

use crate::{render_picker_trigger_button, wire_picker_trigger_a11y};

type LanguageModelPicker = Picker<LanguageModelPickerDelegate>;

/// A selectable picker row: either a Zed-native language model, or a model
/// exposed by an installed external agent.
#[derive(Clone)]
pub enum PickerModel {
    Zed(LanguageModel),
    External {
        agent_id: AgentId,
        agent_name: SharedString,
        model: ExternalAgentModel,
    },
}

/// A non-`Ready` catalog state rendered as one muted, non-selectable row
/// per agent.
enum AgentStatus {
    Loading,
    AuthRequired,
    Failed(SharedString),
}

enum PickerItem {
    Model(PickerModel),
    AgentStatus {
        agent_id: AgentId,
        agent_name: SharedString,
        status: AgentStatus,
    },
}

impl PickerItem {
    /// The stable handle used to keep the highlighted row across rebuilds:
    /// provider-or-agent id plus model id. Status rows carry no model id.
    fn identity(&self) -> (SharedString, SharedString) {
        match self {
            Self::Model(PickerModel::Zed(model)) => (model.provider_id().0, model.id().0),
            Self::Model(PickerModel::External {
                agent_id, model, ..
            }) => (agent_id.0.clone(), model.id.clone()),
            Self::AgentStatus { agent_id, .. } => (agent_id.0.clone(), "".into()),
        }
    }

    /// Whether this row is the profile's stored `default_model` selection:
    /// provider + model id match for Zed models, agent id + model id for
    /// external models. Status rows never match.
    fn matches_selection(&self, selection: &LanguageModelSelection) -> bool {
        match self {
            Self::Model(PickerModel::Zed(model)) => {
                model.provider_id().0 == selection.provider.0 && model.id().0 == selection.model
            }
            Self::Model(PickerModel::External {
                agent_id, model, ..
            }) => agent_id.0 == selection.provider.0 && model.id == selection.model,
            Self::AgentStatus { .. } => false,
        }
    }

    /// The string the query filter checks: `<muted label>/<model name>` for
    /// models — the same `provider/model` shape as the Zed rows — and just
    /// the agent name for status rows.
    fn filter_text(&self) -> String {
        match self {
            Self::Model(PickerModel::Zed(model)) => {
                format!("{}/{}", model.provider_name().0, model.name().0)
            }
            Self::Model(PickerModel::External {
                agent_name, model, ..
            }) => format!("{}/{}", muted_label(agent_name, model), model.name),
            Self::AgentStatus { agent_name, .. } => agent_name.to_string(),
        }
    }
}

/// The muted label on an external model row: the agent's display name,
/// with ` · <group>` appended when the model carries a group.
fn muted_label(agent_name: &SharedString, model: &ExternalAgentModel) -> String {
    match &model.group {
        Some(group) => format!("{agent_name} · {group}"),
        None => agent_name.to_string(),
    }
}

/// The same model source as the agent panel's `LanguageModelSelector`: every
/// visible provider that is currently authenticated contributes its recommended
/// models first, then its remaining models.
fn all_models(cx: &App) -> Vec<LanguageModel> {
    let providers: Vec<_> = LanguageModelRegistry::global(cx)
        .read(cx)
        .visible_providers()
        .into_iter()
        .filter(|provider| provider.is_authenticated(cx))
        .collect();

    let mut models = Vec::new();
    for provider in &providers {
        models.extend(provider.recommended_models(cx));
    }
    for provider in &providers {
        for model in provider.provided_models(cx) {
            let already_recommended = models.iter().any(|existing| {
                existing.provider_id() == model.provider_id() && existing.id() == model.id()
            });
            if !already_recommended {
                models.push(model);
            }
        }
    }
    models
}

fn build_items(
    project: Option<&Entity<Project>>,
    catalog: Option<&Entity<ExternalAgentModelCatalog>>,
    cx: &App,
) -> Vec<PickerItem> {
    let mut items: Vec<PickerItem> = all_models(cx)
        .into_iter()
        .map(|model| PickerItem::Model(PickerModel::Zed(model)))
        .collect();
    if let Some(project) = project {
        items.extend(external_items(project, catalog, cx));
    }
    items
}

/// The external section: every installed agent contributes its catalog
/// models (or one status row), listed after the Zed models. Agents are
/// ordered and named exactly as the External Agents settings page lists
/// them (`collect_agents`, F5): sorted by lowercase display name, falling
/// back to the agent id.
fn external_items(
    project: &Entity<Project>,
    catalog: Option<&Entity<ExternalAgentModelCatalog>>,
    cx: &App,
) -> Vec<PickerItem> {
    let store = project.read(cx).agent_server_store().clone();
    let agents = store
        .read(cx)
        .external_agents()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|agent_id| {
            let display_name = store
                .read(cx)
                .agent_display_name(&agent_id)
                .unwrap_or_else(|| agent_id.0.clone());
            (agent_id, display_name)
        })
        .sorted_unstable_by_key(|(_, display_name)| display_name.to_lowercase())
        .collect::<Vec<_>>();

    let registry = LanguageModelRegistry::global(cx);
    let catalog = catalog.map(|catalog| catalog.read(cx));

    let mut items = Vec::new();
    for (agent_id, agent_name) in agents {
        // AD-11: an agent whose id collides with a registered language-model
        // provider must not be listed — the native agent would resolve the
        // written selection as one of its own models.
        if registry
            .read(cx)
            .provider(&LanguageModelProviderId(agent_id.0.clone()))
            .is_some()
        {
            continue;
        }
        let Some(state) = catalog.and_then(|catalog| catalog.state(&agent_id)) else {
            continue;
        };
        match state {
            ExternalAgentModelsState::Ready(models) => {
                for model in models {
                    items.push(PickerItem::Model(PickerModel::External {
                        agent_id: agent_id.clone(),
                        agent_name: agent_name.clone(),
                        model: model.clone(),
                    }));
                }
            }
            ExternalAgentModelsState::Loading => items.push(PickerItem::AgentStatus {
                agent_id,
                agent_name,
                status: AgentStatus::Loading,
            }),
            ExternalAgentModelsState::AuthRequired => items.push(PickerItem::AgentStatus {
                agent_id,
                agent_name,
                status: AgentStatus::AuthRequired,
            }),
            ExternalAgentModelsState::Failed(message) => items.push(PickerItem::AgentStatus {
                agent_id,
                agent_name,
                status: AgentStatus::Failed(message.clone()),
            }),
        }
    }
    items
}

fn matches_for_query(items: &[PickerItem], query: &str) -> Vec<StringMatch> {
    let query_lower = query.to_lowercase();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            query.is_empty() || item.filter_text().to_lowercase().contains(&query_lower)
        })
        .map(|(index, item)| StringMatch {
            candidate_id: index,
            string: item.filter_text(),
            positions: Vec::new(),
            score: 0.0,
        })
        .collect()
}

/// One-shot state `on_catalog_changed` leaves for the `update_matches`
/// call the observer makes right after: the highlight decision, and the
/// highlighted row's identity captured before `items` was rebuilt — the
/// stale `filtered_items` indices only resolve against the old `items`.
struct PendingHighlight {
    prefer_current_selection: bool,
    previous_identity: Option<(SharedString, SharedString)>,
}

struct LanguageModelPickerDelegate {
    items: Vec<PickerItem>,
    filtered_items: Vec<StringMatch>,
    selected_index: usize,
    /// The last query `update_matches` saw, kept so a catalog change can
    /// re-run the same filter instead of resetting it.
    query: String,
    /// Whether the user moved the highlight since the picker opened — gates
    /// the "re-highlight the selection match" behavior on catalog changes.
    user_moved_highlight: bool,
    pending_highlight: Option<PendingHighlight>,
    current_selection: Option<LanguageModelSelection>,
    project: Option<Entity<Project>>,
    catalog: Option<Entity<ExternalAgentModelCatalog>>,
    _catalog_subscription: Option<Subscription>,
    on_select: Rc<dyn Fn(PickerModel, &mut App) + 'static>,
}

impl LanguageModelPickerDelegate {
    fn new(
        current_selection: Option<LanguageModelSelection>,
        project: Option<Entity<Project>>,
        on_select: Rc<dyn Fn(PickerModel, &mut App) + 'static>,
        window: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) -> Self {
        // Opening the picker is the only trigger that refreshes the catalog
        // — renders elsewhere read it through `external_agent_model_catalog`.
        let catalog = project
            .as_ref()
            .and_then(|project| refresh_external_agent_models(project, cx));
        let subscription = catalog.as_ref().map(|catalog| {
            cx.observe_in(catalog, window, |picker, _catalog, window, cx| {
                let prefer_current_selection = picker.delegate.on_catalog_changed(cx);
                let query = picker.delegate.query.clone();
                // Route through `update_matches_with_options` so the picker's
                // `matches_updated` scrolls the re-highlighted row into view.
                picker.update_matches_with_options(
                    query,
                    if prefer_current_selection {
                        ScrollBehavior::RevealSelected
                    } else {
                        ScrollBehavior::PreserveOffset
                    },
                    window,
                    cx,
                );
            })
        });
        let items = build_items(project.as_ref(), catalog.as_ref(), cx);
        let filtered_items = matches_for_query(&items, "");
        let selected_index = current_selection
            .as_ref()
            .and_then(|selection| {
                filtered_items.iter().position(|item_match| {
                    items
                        .get(item_match.candidate_id)
                        .is_some_and(|item| item.matches_selection(selection))
                })
            })
            .unwrap_or(0);

        Self {
            items,
            filtered_items,
            selected_index,
            query: String::new(),
            user_moved_highlight: false,
            pending_highlight: None,
            current_selection,
            project,
            catalog,
            _catalog_subscription: subscription,
            on_select,
        }
    }

    /// A catalog change rebuilds the external section and reports whether
    /// the follow-up refilter should prefer the `current_selection` match —
    /// the case when the query is empty and the user has not moved the
    /// highlight since opening (the matching row may have just arrived).
    /// The refilter itself runs in `update_matches`, which the observer
    /// invokes next so the picker scrolls to the re-picked row.
    fn on_catalog_changed(&mut self, cx: &App) -> bool {
        let previous_identity = self.highlight_identity();
        self.items = build_items(self.project.as_ref(), self.catalog.as_ref(), cx);
        let prefer_current_selection = self.query.is_empty() && !self.user_moved_highlight;
        self.pending_highlight = Some(PendingHighlight {
            prefer_current_selection,
            previous_identity,
        });
        prefer_current_selection
    }

    /// The identity of the currently highlighted row, captured while
    /// `filtered_items` and `items` still belong to the same build.
    fn highlight_identity(&self) -> Option<(SharedString, SharedString)> {
        self.filtered_items
            .get(self.selected_index)
            .and_then(|item_match| self.items.get(item_match.candidate_id))
            .map(PickerItem::identity)
    }

    /// Re-run the filter and re-pick the highlighted row: the
    /// `current_selection` match when `prefer_current_selection` is set,
    /// otherwise the previously highlighted row kept by identity, falling
    /// back to row 0 when it left the filtered set.
    fn refilter(
        &mut self,
        prefer_current_selection: bool,
        previous_identity: Option<(SharedString, SharedString)>,
    ) {
        self.filtered_items = matches_for_query(&self.items, &self.query);

        let current_match = self
            .current_selection
            .as_ref()
            .filter(|_| prefer_current_selection)
            .and_then(|selection| {
                self.filtered_items.iter().position(|item_match| {
                    self.items
                        .get(item_match.candidate_id)
                        .is_some_and(|item| item.matches_selection(selection))
                })
            });
        self.selected_index = current_match
            .or_else(|| {
                previous_identity.and_then(|identity| {
                    self.filtered_items.iter().position(|item_match| {
                        self.items
                            .get(item_match.candidate_id)
                            .is_some_and(|item| item.identity() == identity)
                    })
                })
            })
            .unwrap_or(0);
    }
}

impl PickerDelegate for LanguageModelPickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "language model picker"
    }

    fn match_count(&self) -> usize {
        self.filtered_items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) {
        self.user_moved_highlight = true;
        self.selected_index = ix.min(self.filtered_items.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(
        &self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<LanguageModelPicker>,
    ) -> bool {
        self.filtered_items
            .get(ix)
            .and_then(|item_match| self.items.get(item_match.candidate_id))
            .is_some_and(|item| matches!(item, PickerItem::Model(_)))
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a model…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No models available. Configure a provider in LLM Providers.".into())
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) -> Task<()> {
        // A catalog change leaves `pending_highlight` for this call; a plain
        // typed query finds none and captures the identity from the still-
        // consistent `filtered_items`/`items` instead.
        let (prefer_current_selection, previous_identity) = match self.pending_highlight.take() {
            Some(pending) => (pending.prefer_current_selection, pending.previous_identity),
            None => (false, self.highlight_identity()),
        };
        self.query = query;
        self.refilter(prefer_current_selection, previous_identity);
        cx.notify();

        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) {
        let Some(item) = self
            .filtered_items
            .get(self.selected_index)
            .and_then(|item_match| self.items.get(item_match.candidate_id))
        else {
            return;
        };
        // Status rows are informational: confirming one does nothing and
        // keeps the picker open.
        let PickerItem::Model(model) = item else {
            return;
        };
        (self.on_select)(model.clone(), cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<LanguageModelPicker>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<LanguageModelPicker>,
    ) -> Option<Self::ListItem> {
        let item_match = self.filtered_items.get(ix)?;
        let item = self.items.get(item_match.candidate_id)?;

        match item {
            PickerItem::Model(PickerModel::Zed(model)) => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(
                        h_flex().gap_2().child(Label::new(model.name().0)).child(
                            Label::new(model.provider_name().0)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .into_any_element(),
            ),
            PickerItem::Model(PickerModel::External {
                agent_name, model, ..
            }) => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Label::new(model.name.clone()))
                            .child(
                                Label::new(muted_label(agent_name, model))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .into_any_element(),
            ),
            PickerItem::AgentStatus {
                agent_name, status, ..
            } => {
                let text = match status {
                    AgentStatus::Loading => format!("{agent_name} · Loading models…"),
                    AgentStatus::AuthRequired => format!("{agent_name} · Not signed in"),
                    AgentStatus::Failed(_) => format!("{agent_name} · Couldn't load models"),
                };
                let mut row = ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .selectable(false)
                    .disabled(true)
                    .child(Label::new(text).color(Color::Muted));
                if let AgentStatus::Failed(message) = status {
                    row = row.tooltip(Tooltip::text(message.clone()));
                }
                Some(row.into_any_element())
            }
        }
    }
}

pub fn render_language_model_picker(
    trigger_label: SharedString,
    current_selection: Option<LanguageModelSelection>,
    project: Option<Entity<Project>>,
    disabled: bool,
    on_select: impl Fn(PickerModel, &mut App) + 'static,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let on_select = Rc::new(on_select);
    let handle = PopoverMenuHandle::default();

    PopoverMenu::new("language-model-picker")
        .trigger(wire_picker_trigger_a11y(
            render_picker_trigger_button("language_model_picker_trigger".into(), trigger_label)
                .disabled(disabled)
                .aria_label("Default Model"),
            handle.clone(),
        ))
        .menu(move |window, cx| {
            let on_select = on_select.clone();
            let current_selection = current_selection.clone();
            let project = project.clone();
            Some(cx.new(|cx| {
                Picker::uniform_list(
                    LanguageModelPickerDelegate::new(
                        current_selection,
                        project,
                        on_select,
                        window,
                        cx,
                    ),
                    window,
                    cx,
                )
                .show_scrollbar(true)
                .initial_width(rems_from_px(280_f32))
                .max_height(rems(18.))
                .popover()
            }))
        })
        .anchor(gpui::Anchor::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(handle)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use fs::{FakeFs, Fs};
    use gpui::{AsyncApp, TestAppContext, UpdateGlobal as _, VisualTestContext};
    use language_model::fake_provider::FakeLanguageModelProvider;
    use project::ExternalAgentSource;
    use project::agent_server_store::{
        AgentServerCommand, ExternalAgentEntry, ExternalAgentServer,
    };
    use project::external_agent_models::{
        external_agent_model_catalog, set_external_agent_model_source,
    };
    use settings::{CustomAgentServerSettings, SettingsStore};

    use super::*;

    async fn init_picker_test(
        cx: &mut TestAppContext,
        agent_ids: &[&str],
    ) -> (Arc<FakeFs>, Entity<Project>, Arc<FakeLanguageModelProvider>) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.save(
            paths::settings_file().as_path(),
            &"{}".into(),
            Default::default(),
        )
        .await
        .expect("failed to save settings");
        let fake_provider = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            menu::init();
            let fake_provider = LanguageModelRegistry::test(cx);
            if !agent_ids.is_empty() {
                SettingsStore::update_global(cx, |store, cx| {
                    store.update_user_settings(cx, |content| {
                        let agent_servers = content.agent_servers.get_or_insert_default();
                        for id in agent_ids {
                            agent_servers.insert(
                                id.to_string(),
                                CustomAgentServerSettings::Custom {
                                    path: PathBuf::from("/usr/bin/fake-agent"),
                                    args: Vec::new(),
                                    env: Default::default(),
                                    default_mode: None,
                                    default_config_options: Default::default(),
                                    favorite_config_option_values: Default::default(),
                                },
                            );
                        }
                    });
                });
            }
            fake_provider
        });
        let project = Project::test(fs.clone(), [], cx).await;
        (fs, project, fake_provider)
    }

    /// A `get_command` stub for directly inserted store entries — the picker
    /// only ever reads ids and display names, so the server is never run.
    struct StubAgentServer;

    impl ExternalAgentServer for StubAgentServer {
        fn get_command(
            &mut self,
            _extra_args: Vec<String>,
            _extra_env: collections::HashMap<String, String>,
            _cx: &mut AsyncApp,
        ) -> Task<anyhow::Result<AgentServerCommand>> {
            Task::ready(Err(anyhow::anyhow!("stub server is never launched")))
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    fn install_named_agent(
        project: &Entity<Project>,
        agent_id: &str,
        display_name: &str,
        cx: &mut TestAppContext,
    ) {
        let store = project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        store.update(cx, |store, _cx| {
            store.external_agents.insert(
                AgentId::new(agent_id),
                ExternalAgentEntry::new(
                    Box::new(StubAgentServer),
                    ExternalAgentSource::Custom,
                    None,
                    Some(display_name.into()),
                ),
            );
        });
    }

    fn ext_model(id: &str, name: &str, group: Option<&str>) -> ExternalAgentModel {
        ExternalAgentModel {
            id: id.into(),
            name: name.into(),
            group: group.map(Into::into),
        }
    }

    fn selection(provider: &str, model: &str) -> LanguageModelSelection {
        LanguageModelSelection {
            provider: settings::LanguageModelProviderSetting(provider.to_string()),
            model: model.to_string(),
            enable_thinking: false,
            effort: None,
            speed: None,
        }
    }

    fn build_picker(
        window_cx: &mut VisualTestContext,
        current_selection: Option<LanguageModelSelection>,
        project: Option<Entity<Project>>,
        received: Rc<RefCell<Vec<PickerModel>>>,
    ) -> Entity<LanguageModelPicker> {
        window_cx.update(|window, cx| {
            cx.new(|cx| {
                Picker::uniform_list(
                    LanguageModelPickerDelegate::new(
                        current_selection,
                        project,
                        Rc::new(move |model, _cx| received.borrow_mut().push(model)),
                        window,
                        cx,
                    ),
                    window,
                    cx,
                )
            })
        })
    }

    fn filtered_labels(
        picker: &Entity<LanguageModelPicker>,
        cx: &VisualTestContext,
    ) -> Vec<String> {
        picker.read_with(cx, |picker, _cx| {
            picker
                .delegate
                .filtered_items
                .iter()
                .filter_map(|item_match| {
                    picker
                        .delegate
                        .items
                        .get(item_match.candidate_id)
                        .map(PickerItem::filter_text)
                })
                .collect()
        })
    }

    fn selected_label(
        picker: &Entity<LanguageModelPicker>,
        cx: &VisualTestContext,
    ) -> Option<String> {
        picker.read_with(cx, |picker, _cx| {
            picker
                .delegate
                .filtered_items
                .get(picker.delegate.selected_index)
                .and_then(|item_match| picker.delegate.items.get(item_match.candidate_id))
                .map(PickerItem::filter_text)
        })
    }

    #[gpui::test]
    async fn external_models_follow_zed_in_display_name_order(cx: &mut TestAppContext) {
        let (_fs, project, _provider) = init_picker_test(cx, &["zeta-agent", "alpha-agent"]).await;
        install_named_agent(&project, "zzz-custom", "Aardvark", cx);

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("zeta-agent"),
                            ExternalAgentModelsState::Ready(vec![ext_model(
                                "zeta-1", "Zeta One", None,
                            )]),
                            cx,
                        );
                        catalog.set_state(
                            AgentId::new("alpha-agent"),
                            ExternalAgentModelsState::Ready(vec![ext_model(
                                "alpha-1",
                                "Alpha One",
                                None,
                            )]),
                            cx,
                        );
                        catalog.set_state(
                            AgentId::new("zzz-custom"),
                            ExternalAgentModelsState::Ready(vec![ext_model(
                                "aardvark-1",
                                "Aardvark One",
                                Some("Frontier"),
                            )]),
                            cx,
                        );
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let received = Rc::new(RefCell::new(Vec::new()));
        let picker = build_picker(cx, None, Some(project), received);

        let labels = filtered_labels(&picker, cx);
        // Zed model first, then agents sorted by display name — "Aardvark"
        // sorts before "alpha-agent" despite its `zzz-custom` id.
        assert_eq!(
            labels,
            vec![
                "Fake/Fake",
                "Aardvark · Frontier/Aardvark One",
                "alpha-agent/Alpha One",
                "zeta-agent/Zeta One",
            ]
        );
    }

    #[gpui::test]
    async fn ready_empty_and_agent_with_no_state_yield_no_rows(cx: &mut TestAppContext) {
        let (_fs, project, _provider) = init_picker_test(cx, &["empty-agent", "ghost-agent"]).await;
        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("empty-agent"),
                            ExternalAgentModelsState::Ready(vec![]),
                            cx,
                        );
                        // ghost-agent is never assigned a state.
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let picker = build_picker(cx, None, Some(project), Rc::new(RefCell::new(Vec::new())));

        assert_eq!(filtered_labels(&picker, cx), vec!["Fake/Fake"]);
    }

    #[gpui::test]
    async fn provider_id_collision_agent_is_excluded(cx: &mut TestAppContext) {
        // The fake provider registers under id "fake" — an agent with the
        // same id must not be listed (AD-11).
        let (_fs, project, _provider) = init_picker_test(cx, &["fake", "clean-agent"]).await;
        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        for agent_id in ["fake", "clean-agent"] {
                            catalog.set_state(
                                AgentId::new(agent_id),
                                ExternalAgentModelsState::Ready(vec![ext_model(
                                    "m1",
                                    "Model One",
                                    None,
                                )]),
                                cx,
                            );
                        }
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let picker = build_picker(cx, None, Some(project), Rc::new(RefCell::new(Vec::new())));

        assert_eq!(
            filtered_labels(&picker, cx),
            vec!["Fake/Fake", "clean-agent/Model One"]
        );
    }

    #[gpui::test]
    async fn status_rows_are_not_selectable_and_confirm_does_not_dismiss(cx: &mut TestAppContext) {
        let (_fs, project, _provider) =
            init_picker_test(cx, &["loading-agent", "auth-agent", "broken-agent"]).await;
        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("auth-agent"),
                            ExternalAgentModelsState::AuthRequired,
                            cx,
                        );
                        catalog.set_state(
                            AgentId::new("broken-agent"),
                            ExternalAgentModelsState::Failed("probe timed out".into()),
                            cx,
                        );
                        catalog.set_state(
                            AgentId::new("loading-agent"),
                            ExternalAgentModelsState::Loading,
                            cx,
                        );
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let received = Rc::new(RefCell::new(Vec::new()));
        let picker = build_picker(cx, None, Some(project), received.clone());

        assert_eq!(
            filtered_labels(&picker, cx),
            vec!["Fake/Fake", "auth-agent", "broken-agent", "loading-agent",]
        );

        let dismissals = Arc::new(AtomicUsize::new(0));
        cx.update(|_, cx| {
            cx.subscribe(&picker, {
                let dismissals = dismissals.clone();
                move |_picker, _event: &DismissEvent, _cx| {
                    dismissals.fetch_add(1, Ordering::SeqCst);
                }
            })
            .detach();
        });

        // Every status row reports non-selectable, and confirming while the
        // highlight sits on one changes nothing and emits no DismissEvent.
        picker.update_in(cx, |picker, window, cx| {
            for ix in 1..picker.delegate.match_count() {
                assert!(
                    !picker.delegate.can_select(ix, window, cx),
                    "row {ix} must not be selectable"
                );
                picker.delegate.selected_index = ix;
                picker.delegate.confirm(false, window, cx);
            }
        });
        assert!(received.borrow().is_empty());
        assert_eq!(dismissals.load(Ordering::SeqCst), 0);

        // The Zed row still confirms normally.
        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.selected_index = 0;
            picker.delegate.confirm(false, window, cx);
        });
        assert_eq!(received.borrow().len(), 1);
        assert!(matches!(received.borrow()[0], PickerModel::Zed(_)));
        assert_eq!(dismissals.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn catalog_change_preserves_typed_query(cx: &mut TestAppContext) {
        let (_fs, project, _provider) = init_picker_test(cx, &["model-agent"]).await;
        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("model-agent"),
                            ExternalAgentModelsState::Loading,
                            cx,
                        );
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let picker = build_picker(
            cx,
            None,
            Some(project.clone()),
            Rc::new(RefCell::new(Vec::new())),
        );
        let catalog = cx
            .update(|_, cx| external_agent_model_catalog(&project, cx))
            .expect("catalog created by the picker's refresh");

        picker.update_in(cx, |picker, window, cx| {
            picker.update_matches("one".to_string(), window, cx);
        });
        assert_eq!(filtered_labels(&picker, cx), Vec::<String>::new());

        // The catalog moves to Ready while the query is typed: the query and
        // its filter are preserved, and the new rows are filtered by it.
        catalog.update(cx, |catalog, cx| {
            catalog.set_state(
                AgentId::new("model-agent"),
                ExternalAgentModelsState::Ready(vec![
                    ext_model("m1", "Model One", None),
                    ext_model("x2", "Auxiliary", None),
                ]),
                cx,
            );
        });
        cx.run_until_parked();

        picker.read_with(cx, |picker, _cx| {
            assert_eq!(picker.delegate.query, "one");
        });
        assert_eq!(filtered_labels(&picker, cx), vec!["model-agent/Model One"]);
    }

    #[gpui::test]
    async fn selection_match_is_highlighted_when_model_arrives_late(cx: &mut TestAppContext) {
        let (_fs, project, provider) = init_picker_test(cx, &["late-agent"]).await;
        // Enough Zed models that the external row lands below the 18rem
        // viewport — the scroll assertion below only means something when
        // the row starts off-screen.
        for index in 0..30 {
            provider.model(&format!("zed-model-{index:02}"));
        }
        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("late-agent"),
                            ExternalAgentModelsState::Loading,
                            cx,
                        );
                    });
                },
                cx,
            );
        });

        let cx = cx.add_empty_window();
        let picker = build_picker(
            cx,
            Some(selection("late-agent", "m1")),
            Some(project.clone()),
            Rc::new(RefCell::new(Vec::new())),
        );
        let catalog = cx
            .update(|_, cx| external_agent_model_catalog(&project, cx))
            .expect("catalog");

        // Only the Loading status row exists for the agent — nothing matches
        // the selection yet, so the highlight sits on row 0.
        assert_eq!(selected_label(&picker, cx).as_deref(), Some("Fake/Fake"));

        catalog.update(cx, |catalog, cx| {
            catalog.set_state(
                AgentId::new("late-agent"),
                ExternalAgentModelsState::Ready(vec![ext_model("m1", "Model One", None)]),
                cx,
            );
        });
        cx.run_until_parked();

        // The arrived row matches `current_selection` and takes the
        // highlight without the user touching it.
        assert_eq!(
            selected_label(&picker, cx).as_deref(),
            Some("late-agent/Model One")
        );
        // …and the picker scrolled the off-screen row into view (the scroll
        // target is the selected row, which sits after the 31 Zed rows).
        picker.read_with(cx, |picker, _cx| {
            let selected_index = picker.delegate.selected_index;
            assert!(selected_index > 30, "external row should be off-screen");
            assert_eq!(picker.logical_scroll_top_index(), selected_index);
        });
    }

    #[gpui::test]
    async fn no_project_or_no_source_lists_zed_models_only(cx: &mut TestAppContext) {
        let (_fs, project, _provider) = init_picker_test(cx, &["some-agent"]).await;
        let cx = cx.add_empty_window();

        // No source registered at all — refresh returns None, so even with
        // a project only Zed models appear.
        let picker = build_picker(cx, None, Some(project), Rc::new(RefCell::new(Vec::new())));
        assert_eq!(filtered_labels(&picker, cx), vec!["Fake/Fake"]);

        // No project — the picker is Zed-only by construction.
        let picker = build_picker(cx, None, None, Rc::new(RefCell::new(Vec::new())));
        assert_eq!(filtered_labels(&picker, cx), vec!["Fake/Fake"]);
    }
}
