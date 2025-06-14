use agent_settings::AgentSettings;
use collections::{HashSet, IndexMap};
use feature_flags::ZedProFeatureFlag;
use fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, BackgroundExecutor, Context, DismissEvent, Subscription, Task, Window,
    action_with_deprecated_aliases, actions, impl_actions,
};
use language_model::{
    AuthenticateError, ConfiguredModel, LanguageModel, LanguageModelProviderId,
    LanguageModelRegistry,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use proto::Plan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, update_settings_file};
use std::{cmp::Reverse, sync::Arc};
use ui::{IconButton, ListItem, ListItemSpacing, Tooltip, prelude::*};

action_with_deprecated_aliases!(
    agent,
    ToggleModelSelector,
    [
        "assistant::ToggleModelSelector",
        "assistant2::ToggleModelSelector"
    ]
);

const TRY_ZED_PRO_URL: &str = "https://zed.dev/pro";

type OnModelChanged = Arc<dyn Fn(Arc<dyn LanguageModel>, &mut App) + 'static>;
type GetActiveModel = Arc<dyn Fn(&App) -> Option<ConfiguredModel> + 'static>;

pub type LanguageModelSelector = Picker<LanguageModelPickerDelegate>;

pub fn language_model_selector(
    get_active_model: impl Fn(&App) -> Option<ConfiguredModel> + 'static,
    on_model_changed: impl Fn(Arc<dyn LanguageModel>, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<LanguageModelSelector>,
) -> LanguageModelSelector {
    let delegate = LanguageModelPickerDelegate::new(get_active_model, on_model_changed, window, cx);
    Picker::list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems(20.))
        .max_height(Some(rems(20.).into()))
}

fn all_models(cx: &App) -> GroupedModels {
    let providers = LanguageModelRegistry::global(cx).read(cx).providers();

    let recommended = providers
        .iter()
        .flat_map(|provider| {
            provider
                .recommended_models(cx)
                .into_iter()
                .map(|model| ModelInfo {
                    model,
                    icon: provider.icon(),
                })
        })
        .collect();

    let other = providers
        .iter()
        .flat_map(|provider| {
            provider
                .provided_models(cx)
                .into_iter()
                .map(|model| ModelInfo {
                    model,
                    icon: provider.icon(),
                })
        })
        .collect();

    let favorites: Vec<_> = AgentSettings::get_global(cx)
        .favorite_models
        .iter()
        .filter_map(|selection| {
            providers.iter().find_map(|provider| {
                if provider.id().0 == selection.provider.0 {
                    provider
                        .provided_models(cx)
                        .into_iter()
                        .find(|model| model.id().0 == selection.model)
                        .map(|model| ModelInfo {
                            model,
                            icon: provider.icon(),
                        })
                } else {
                    None
                }
            })
        })
        .collect();

    GroupedModels::with_favorites(other, recommended, favorites)
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct ToggleModelFavorite {
    #[serde(default)]
    pub provider_id: String,
    #[serde(default)]
    pub model_id: String,
}
actions!(agent, [ToggleFavoriteHotkey]);
impl_actions!(agent, [ToggleModelFavorite]);

#[derive(Clone)]
struct ModelInfo {
    model: Arc<dyn LanguageModel>,
    icon: IconName,
}

pub struct LanguageModelPickerDelegate {
    on_model_changed: OnModelChanged,
    get_active_model: GetActiveModel,
    all_models: Arc<GroupedModels>,
    filtered_entries: Vec<LanguageModelPickerEntry>,
    selected_index: usize,
    _authenticate_all_providers_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl LanguageModelPickerDelegate {
    pub fn handle_toggle_favorite(
        &mut self,
        action: &ToggleModelFavorite,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let provider_id = &action.provider_id;
        let model_id = &action.model_id;

        let fs = <dyn fs::Fs>::global(cx);
        update_settings_file::<AgentSettings>(fs, cx, {
            let provider_id = provider_id.clone();
            let model_id = model_id.clone();
            move |settings, _| {
                if settings.is_favorite_model(&provider_id, &model_id) {
                    settings.remove_favorite_model(&provider_id, &model_id);
                } else {
                    settings.add_favorite_model(&provider_id, &model_id);
                }
            }
        });
    }

    fn toggle_selected_favorite(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(LanguageModelPickerEntry::Model(model_info)) =
            self.filtered_entries.get(self.selected_index)
        {
            let provider_id = model_info.model.provider_id().0.to_string();
            let model_id = model_info.model.id().0.to_string();
            self.handle_toggle_favorite(
                &ToggleModelFavorite {
                    provider_id,
                    model_id,
                },
                window,
                cx,
            );
        }
    }
}

impl LanguageModelPickerDelegate {
    fn new(
        get_active_model: impl Fn(&App) -> Option<ConfiguredModel> + 'static,
        on_model_changed: impl Fn(Arc<dyn LanguageModel>, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let on_model_changed = Arc::new(on_model_changed);
        let all_models = Arc::new(all_models(cx));
        let entries = all_models.entries();

        Self {
            on_model_changed: on_model_changed.clone(),
            all_models,
            selected_index: Self::get_active_model_index(&entries, get_active_model(cx)),
            filtered_entries: entries,
            get_active_model: Arc::new(get_active_model),
            _authenticate_all_providers_task: Self::authenticate_all_providers(cx),
            _subscriptions: vec![
                cx.subscribe_in(
                    &LanguageModelRegistry::global(cx),
                    window,
                    |picker, _, event, window, cx| match event {
                        language_model::Event::ProviderStateChanged
                        | language_model::Event::AddedProvider(_)
                        | language_model::Event::RemovedProvider(_) => {
                            let query = picker.query(cx);
                            picker.delegate.all_models =
                                Arc::new(crate::language_model_selector::all_models(cx));
                            // Update matches will automatically drop the previous task
                            // if we get a provider event again
                            picker.update_matches(query, window, cx)
                        }
                        _ => {}
                    },
                ),
                cx.observe_global_in::<SettingsStore>(window, |picker, window, cx| {
                    let query = picker.query(cx);
                    picker.delegate.all_models =
                        Arc::new(crate::language_model_selector::all_models(cx));
                    picker.update_matches(query, window, cx)
                }),
            ],
        }
    }

    fn get_active_model_index(
        entries: &[LanguageModelPickerEntry],
        active_model: Option<ConfiguredModel>,
    ) -> usize {
        entries
            .iter()
            .position(|entry| {
                if let LanguageModelPickerEntry::Model(model) = entry {
                    active_model
                        .as_ref()
                        .map(|active_model| {
                            active_model.model.id() == model.model.id()
                                && active_model.provider.id() == model.model.provider_id()
                        })
                        .unwrap_or_default()
                } else {
                    false
                }
            })
            .unwrap_or(0)
    }

    /// Authenticates all providers in the [`LanguageModelRegistry`].
    ///
    /// We do this so that we can populate the language selector with all of the
    /// models from the configured providers.
    fn authenticate_all_providers(cx: &mut App) -> Task<()> {
        let authenticate_all_providers = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .map(|provider| (provider.id(), provider.name(), provider.authenticate(cx)))
            .collect::<Vec<_>>();

        cx.spawn(async move |_cx| {
            for (provider_id, provider_name, authenticate_task) in authenticate_all_providers {
                if let Err(err) = authenticate_task.await {
                    if matches!(err, AuthenticateError::CredentialsNotFound) {
                        // Since we're authenticating these providers in the
                        // background for the purposes of populating the
                        // language selector, we don't care about providers
                        // where the credentials are not found.
                    } else {
                        // Some providers have noisy failure states that we
                        // don't want to spam the logs with every time the
                        // language model selector is initialized.
                        //
                        // Ideally these should have more clear failure modes
                        // that we know are safe to ignore here, like what we do
                        // with `CredentialsNotFound` above.
                        match provider_id.0.as_ref() {
                            "lmstudio" | "ollama" => {
                                // LM Studio and Ollama both make fetch requests to the local APIs to determine if they are "authenticated".
                                //
                                // These fail noisily, so we don't log them.
                            }
                            "copilot_chat" => {
                                // Copilot Chat returns an error if Copilot is not enabled, so we don't log those errors.
                            }
                            _ => {
                                log::error!(
                                    "Failed to authenticate provider: {}: {err}",
                                    provider_name.0
                                );
                            }
                        }
                    }
                }
            }
        })
    }

    pub fn active_model(&self, cx: &App) -> Option<ConfiguredModel> {
        (self.get_active_model)(cx)
    }
}

#[derive(Clone)]
struct GroupedModels {
    favorites: Vec<ModelInfo>,
    recommended: Vec<ModelInfo>,
    other: IndexMap<LanguageModelProviderId, Vec<ModelInfo>>,
}

impl GroupedModels {
    // TODO remove or keep?
    #[cfg(test)]
    pub fn new(other: Vec<ModelInfo>, recommended: Vec<ModelInfo>) -> Self {
        Self::with_favorites(other, recommended, Vec::new())
    }

    pub(crate) fn with_favorites(
        other: Vec<ModelInfo>,
        recommended: Vec<ModelInfo>,
        favorites: Vec<ModelInfo>,
    ) -> Self {
        let recommended_ids = recommended
            .iter()
            .map(|info| (info.model.provider_id(), info.model.id()))
            .collect::<HashSet<_>>();

        let favorite_ids = favorites
            .iter()
            .map(|info| (info.model.provider_id(), info.model.id()))
            .collect::<HashSet<_>>();

        let mut other_by_provider: IndexMap<_, Vec<ModelInfo>> = IndexMap::default();
        for model in other {
            if recommended_ids.contains(&(model.model.provider_id(), model.model.id()))
                || favorite_ids.contains(&(model.model.provider_id(), model.model.id()))
            {
                continue;
            }

            let provider = model.model.provider_id();
            if let Some(models) = other_by_provider.get_mut(&provider) {
                models.push(model);
            } else {
                other_by_provider.insert(provider, vec![model]);
            }
        }

        Self {
            favorites,
            recommended,
            other: other_by_provider,
        }
    }

    fn entries(&self) -> Vec<LanguageModelPickerEntry> {
        let mut entries = Vec::new();

        if !self.favorites.is_empty() {
            entries.push(LanguageModelPickerEntry::Separator("Favorites".into()));
            entries.extend(
                self.favorites
                    .iter()
                    .map(|info| LanguageModelPickerEntry::Model(info.clone())),
            );
        }

        if !self.recommended.is_empty() {
            entries.push(LanguageModelPickerEntry::Separator("Recommended".into()));
            entries.extend(
                self.recommended
                    .iter()
                    .map(|info| LanguageModelPickerEntry::Model(info.clone())),
            );
        }

        for models in self.other.values() {
            if models.is_empty() {
                continue;
            }
            entries.push(LanguageModelPickerEntry::Separator(
                models[0].model.provider_name().0,
            ));
            entries.extend(
                models
                    .iter()
                    .map(|info| LanguageModelPickerEntry::Model(info.clone())),
            );
        }
        entries
    }

    fn model_infos(&self) -> Vec<ModelInfo> {
        let other = self
            .other
            .values()
            .flat_map(|model| model.iter())
            .cloned()
            .collect::<Vec<_>>();
        self.favorites
            .iter()
            .chain(&self.recommended)
            .chain(&other)
            .cloned()
            .collect::<Vec<_>>()
    }
}

enum LanguageModelPickerEntry {
    Model(ModelInfo),
    Separator(SharedString),
}

struct ModelMatcher {
    models: Vec<ModelInfo>,
    bg_executor: BackgroundExecutor,
    candidates: Vec<StringMatchCandidate>,
}

impl ModelMatcher {
    fn new(models: Vec<ModelInfo>, bg_executor: BackgroundExecutor) -> ModelMatcher {
        let candidates = Self::make_match_candidates(&models);
        Self {
            models,
            bg_executor,
            candidates,
        }
    }

    pub fn fuzzy_search(&self, query: &str) -> Vec<ModelInfo> {
        let mut matches = self.bg_executor.block(match_strings(
            &self.candidates,
            &query,
            false,
            100,
            &Default::default(),
            self.bg_executor.clone(),
        ));

        let sorting_key = |mat: &StringMatch| {
            let candidate = &self.candidates[mat.candidate_id];
            (Reverse(OrderedFloat(mat.score)), candidate.id)
        };
        matches.sort_unstable_by_key(sorting_key);

        let matched_models: Vec<_> = matches
            .into_iter()
            .map(|mat| self.models[mat.candidate_id].clone())
            .collect();

        matched_models
    }

    pub fn exact_search(&self, query: &str) -> Vec<ModelInfo> {
        self.models
            .iter()
            .filter(|m| {
                m.model
                    .name()
                    .0
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    fn make_match_candidates(model_infos: &Vec<ModelInfo>) -> Vec<StringMatchCandidate> {
        model_infos
            .iter()
            .enumerate()
            .map(|(index, model)| {
                StringMatchCandidate::new(
                    index,
                    &format!(
                        "{}/{}",
                        &model.model.provider_name().0,
                        &model.model.name().0
                    ),
                )
            })
            .collect::<Vec<_>>()
    }
}

impl PickerDelegate for LanguageModelPickerDelegate {
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
            Some(LanguageModelPickerEntry::Model(_)) => true,
            Some(LanguageModelPickerEntry::Separator(_)) | None => false,
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
        let all_models = self.all_models.clone();
        let current_index = self.selected_index;
        let bg_executor = cx.background_executor();

        let language_model_registry = LanguageModelRegistry::global(cx);

        let configured_providers = language_model_registry
            .read(cx)
            .providers()
            .into_iter()
            .filter(|provider| provider.is_authenticated(cx))
            .collect::<Vec<_>>();

        let configured_provider_ids = configured_providers
            .iter()
            .map(|provider| provider.id())
            .collect::<Vec<_>>();

        let recommended_models = all_models
            .recommended
            .iter()
            .filter(|m| configured_provider_ids.contains(&m.model.provider_id()))
            .cloned()
            .collect::<Vec<_>>();

        let available_models = all_models
            .model_infos()
            .iter()
            .filter(|m| configured_provider_ids.contains(&m.model.provider_id()))
            .cloned()
            .collect::<Vec<_>>();

        let matcher_rec = ModelMatcher::new(recommended_models, bg_executor.clone());
        let matcher_all = ModelMatcher::new(available_models.clone(), bg_executor.clone());

        let recommended = matcher_rec.exact_search(&query);
        let all = matcher_all.fuzzy_search(&query);

        let favorites: Vec<_> = AgentSettings::get_global(cx)
            .favorite_models
            .iter()
            .filter_map(|selection| {
                available_models
                    .iter()
                    .find(|model| {
                        model.model.provider_id().0 == selection.provider.0
                            && model.model.id().0 == selection.model
                    })
                    .cloned()
            })
            .collect();
        let filtered_models = GroupedModels::with_favorites(all, recommended, favorites);

        cx.spawn_in(window, async move |this, cx| {
            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_entries = filtered_models.entries();
                // Preserve selection focus
                let new_index = if current_index >= this.delegate.filtered_entries.len() {
                    0
                } else {
                    current_index
                };
                this.set_selected_index(new_index, Some(picker::Direction::Down), true, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(LanguageModelPickerEntry::Model(model_info)) =
            self.filtered_entries.get(self.selected_index)
        {
            let model = model_info.model.clone();
            (self.on_model_changed)(model.clone(), cx);

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
            LanguageModelPickerEntry::Separator(title) => Some(
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
            LanguageModelPickerEntry::Model(model_info) => {
                let active_model = (self.get_active_model)(cx);
                let active_provider_id = active_model.as_ref().map(|m| m.provider.id());
                let active_model_id = active_model.map(|m| m.model.id());

                let is_selected = Some(model_info.model.provider_id()) == active_provider_id
                    && Some(model_info.model.id()) == active_model_id;

                let model_icon_color = if is_selected {
                    Color::Accent
                } else {
                    Color::Muted
                };

                let agent_settings = AgentSettings::get_global(cx);
                let is_favorite = agent_settings
                    .is_favorite_model(&model_info.model.provider_id().0, &model_info.model.id().0);
                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .start_slot(
                            Icon::new(model_info.icon)
                                .color(model_icon_color)
                                .size(IconSize::Small),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .pl_0p5()
                                .gap_1p5()
                                .w(px(240.))
                                .child(Label::new(model_info.model.name().0.clone()).truncate()),
                        )
                        .end_slot(
                            h_flex().gap_1().pr_3().child(
                                IconButton::new(
                                    ("favorite", ix),
                                    if is_selected {
                                        IconName::Check
                                    } else if is_favorite {
                                        IconName::StarFilled
                                    } else {
                                        IconName::Star
                                    },
                                )
                                .icon_color(if is_selected {
                                    Color::Accent
                                } else {
                                    Color::Disabled
                                })
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text(if is_favorite {
                                    "Unfavorite"
                                } else {
                                    "Favorite"
                                }))
                                .on_click({
                                    let provider_id = model_info.model.provider_id().0.to_string();
                                    let model_id = model_info.model.id().0.to_string();
                                    cx.listener(move |picker, _, window, cx| {
                                        picker.delegate.handle_toggle_favorite(
                                            &ToggleModelFavorite {
                                                provider_id: provider_id.clone(),
                                                model_id: model_id.clone(),
                                            },
                                            window,
                                            cx,
                                        );
                                    })
                                }),
                            ),
                        )
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        cx.on_action(
            std::any::TypeId::of::<ToggleFavoriteHotkey>(),
            window,
            |picker: &mut Picker<Self>, _action, phase, window, cx| {
                if matches!(phase, gpui::DispatchPhase::Bubble) {
                    picker.delegate.toggle_selected_favorite(window, cx);
                }
            },
        );

        use feature_flags::FeatureFlagAppExt;

        let plan = proto::Plan::ZedPro;

        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .gap_4()
                .justify_between()
                .when(cx.has_flag::<ZedProFeatureFlag>(), |this| {
                    this.child(match plan {
                        Plan::ZedPro => Button::new("zed-pro", "Zed Pro")
                            .icon(IconName::ZedAssistant)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .icon_position(IconPosition::Start)
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(Box::new(zed_actions::OpenAccountSettings), cx)
                            }),
                        Plan::Free | Plan::ZedProTrial => Button::new(
                            "try-pro",
                            if plan == Plan::ZedProTrial {
                                "Upgrade to Pro"
                            } else {
                                "Try Pro"
                            },
                        )
                        .on_click(|_, _, cx| cx.open_url(TRY_ZED_PRO_URL)),
                    })
                })
                .child(
                    Button::new("configure", "Configure")
                        .icon(IconName::Settings)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(
                                zed_actions::agent::OpenConfiguration.boxed_clone(),
                                cx,
                            );
                        }),
                )
                .into_any(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{future::BoxFuture, stream::BoxStream};
    use gpui::{AsyncApp, TestAppContext, http_client};
    use language_model::{
        LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId,
        LanguageModelName, LanguageModelProviderId, LanguageModelProviderName,
        LanguageModelRequest, LanguageModelToolChoice,
    };
    use ui::IconName;

    #[derive(Clone)]
    struct TestLanguageModel {
        name: LanguageModelName,
        id: LanguageModelId,
        provider_id: LanguageModelProviderId,
        provider_name: LanguageModelProviderName,
    }

    impl TestLanguageModel {
        fn new(name: &str, provider: &str) -> Self {
            Self {
                name: LanguageModelName::from(name.to_string()),
                id: LanguageModelId::from(name.to_string()),
                provider_id: LanguageModelProviderId::from(provider.to_string()),
                provider_name: LanguageModelProviderName::from(provider.to_string()),
            }
        }
    }

    impl LanguageModel for TestLanguageModel {
        fn id(&self) -> LanguageModelId {
            self.id.clone()
        }

        fn name(&self) -> LanguageModelName {
            self.name.clone()
        }

        fn provider_id(&self) -> LanguageModelProviderId {
            self.provider_id.clone()
        }

        fn provider_name(&self) -> LanguageModelProviderName {
            self.provider_name.clone()
        }

        fn supports_tools(&self) -> bool {
            false
        }

        fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
            false
        }

        fn supports_images(&self) -> bool {
            false
        }

        fn telemetry_id(&self) -> String {
            format!("{}/{}", self.provider_id.0, self.name.0)
        }

        fn max_token_count(&self) -> usize {
            1000
        }

        fn count_tokens(
            &self,
            _: LanguageModelRequest,
            _: &App,
        ) -> BoxFuture<'static, http_client::Result<usize>> {
            unimplemented!()
        }

        fn stream_completion(
            &self,
            _: LanguageModelRequest,
            _: &AsyncApp,
        ) -> BoxFuture<
            'static,
            Result<
                BoxStream<
                    'static,
                    Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                >,
                LanguageModelCompletionError,
            >,
        > {
            unimplemented!()
        }
    }

    fn create_models(model_specs: Vec<(&str, &str)>) -> Vec<ModelInfo> {
        model_specs
            .into_iter()
            .map(|(provider, name)| ModelInfo {
                model: Arc::new(TestLanguageModel::new(name, provider)),
                icon: IconName::Ai,
            })
            .collect()
    }

    fn assert_models_eq(result: Vec<ModelInfo>, expected: Vec<&str>) {
        assert_eq!(
            result.len(),
            expected.len(),
            "Number of models doesn't match"
        );

        for (i, expected_name) in expected.iter().enumerate() {
            assert_eq!(
                result[i].model.telemetry_id(),
                *expected_name,
                "Model at position {} doesn't match expected model",
                i
            );
        }
    }

    #[gpui::test]
    fn test_exact_match(cx: &mut TestAppContext) {
        let models = create_models(vec![
            ("zed", "Claude 3.7 Sonnet"),
            ("zed", "Claude 3.7 Sonnet Thinking"),
            ("zed", "gpt-4.1"),
            ("zed", "gpt-4.1-nano"),
            ("openai", "gpt-3.5-turbo"),
            ("openai", "gpt-4.1"),
            ("openai", "gpt-4.1-nano"),
            ("ollama", "mistral"),
            ("ollama", "deepseek"),
        ]);
        let matcher = ModelMatcher::new(models, cx.background_executor.clone());

        // The order of models should be maintained, case doesn't matter
        let results = matcher.exact_search("GPT-4.1");
        assert_models_eq(
            results,
            vec![
                "zed/gpt-4.1",
                "zed/gpt-4.1-nano",
                "openai/gpt-4.1",
                "openai/gpt-4.1-nano",
            ],
        );
    }

    #[gpui::test]
    fn test_fuzzy_match(cx: &mut TestAppContext) {
        let models = create_models(vec![
            ("zed", "Claude 3.7 Sonnet"),
            ("zed", "Claude 3.7 Sonnet Thinking"),
            ("zed", "gpt-4.1"),
            ("zed", "gpt-4.1-nano"),
            ("openai", "gpt-3.5-turbo"),
            ("openai", "gpt-4.1"),
            ("openai", "gpt-4.1-nano"),
            ("ollama", "mistral"),
            ("ollama", "deepseek"),
        ]);
        let matcher = ModelMatcher::new(models, cx.background_executor.clone());

        // Results should preserve models order whenever possible.
        // In the case below, `zed/gpt-4.1` and `openai/gpt-4.1` have identical
        // similarity scores, but `zed/gpt-4.1` was higher in the models list,
        // so it should appear first in the results.
        let results = matcher.fuzzy_search("41");
        assert_models_eq(
            results,
            vec![
                "zed/gpt-4.1",
                "openai/gpt-4.1",
                "zed/gpt-4.1-nano",
                "openai/gpt-4.1-nano",
            ],
        );

        // Model provider should be searchable as well
        let results = matcher.fuzzy_search("ol"); // meaning "ollama"
        assert_models_eq(results, vec!["ollama/mistral", "ollama/deepseek"]);

        // Fuzzy search
        let results = matcher.fuzzy_search("z4n");
        assert_models_eq(results, vec!["zed/gpt-4.1-nano"]);
    }

    #[gpui::test]
    fn test_exclude_recommended_models(_cx: &mut TestAppContext) {
        let recommended_models = create_models(vec![("zed", "claude")]);
        let all_models = create_models(vec![
            ("zed", "claude"), // Should be filtered out from "other"
            ("zed", "gemini"),
            ("copilot", "o3"),
        ]);

        let grouped_models = GroupedModels::new(all_models, recommended_models);

        let actual_other_models = grouped_models
            .other
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();

        // Recommended models should not appear in "other"
        assert_models_eq(actual_other_models, vec!["zed/gemini", "copilot/o3"]);
    }

    #[gpui::test]
    fn test_dont_exclude_models_from_other_providers(_cx: &mut TestAppContext) {
        let recommended_models = create_models(vec![("zed", "claude")]);
        let all_models = create_models(vec![
            ("zed", "claude"), // Should be filtered out from "other"
            ("zed", "gemini"),
            ("copilot", "claude"), // Should not be filtered out from "other"
        ]);

        let grouped_models = GroupedModels::new(all_models, recommended_models);

        let actual_other_models = grouped_models
            .other
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();

        // Recommended models should not appear in "other"
        assert_models_eq(actual_other_models, vec!["zed/gemini", "copilot/claude"]);
    }

    #[test]
    fn test_models_with_favorites() {
        let favorite_models = create_models(vec![("zed", "claude"), ("openai", "gpt-4")]);
        let recommended_models = create_models(vec![("zed", "gemini")]);
        let all_models = create_models(vec![
            ("zed", "claude"),
            ("zed", "gemini"),
            ("openai", "gpt-4"),
            ("openai", "gpt-3.5"),
        ]);
        let grouped_models =
            GroupedModels::with_favorites(all_models, recommended_models, favorite_models);

        assert_models_eq(
            grouped_models.favorites.clone(),
            vec!["zed/claude", "openai/gpt-4"],
        );

        assert_models_eq(grouped_models.recommended.clone(), vec!["zed/gemini"]);

        let actual_other_models = grouped_models
            .other
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        assert_models_eq(actual_other_models, vec!["openai/gpt-3.5"]);

        let entries = grouped_models.entries();
        let separator_count = entries
            .iter()
            .filter(|entry| matches!(entry, LanguageModelPickerEntry::Separator(_)))
            .count();
        assert_eq!(separator_count, 3);
    }
}
