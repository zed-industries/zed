use std::sync::Arc;

use collections::{HashSet, IndexMap};
use feature_flags::ZedProFeatureFlag;
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, AnyView, App, BackgroundExecutor, Corner, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
    action_with_deprecated_aliases,
};
use language_model::{
    AuthenticateError, ConfiguredModel, LanguageModel, LanguageModelProviderId,
    LanguageModelRegistry,
};
use picker::{Picker, PickerDelegate};
use proto::Plan;
use ui::{ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger, prelude::*};

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

pub struct LanguageModelSelector {
    picker: Entity<Picker<LanguageModelPickerDelegate>>,
    _authenticate_all_providers_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl LanguageModelSelector {
    pub fn new(
        get_active_model: impl Fn(&App) -> Option<ConfiguredModel> + 'static,
        on_model_changed: impl Fn(Arc<dyn LanguageModel>, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let on_model_changed = Arc::new(on_model_changed);

        let all_models = Self::all_models(cx);
        let entries = all_models.entries();

        let delegate = LanguageModelPickerDelegate {
            language_model_selector: cx.entity().downgrade(),
            on_model_changed: on_model_changed.clone(),
            all_models: Arc::new(all_models),
            selected_index: Self::get_active_model_index(&entries, get_active_model(cx)),
            filtered_entries: entries,
            get_active_model: Arc::new(get_active_model),
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .show_scrollbar(true)
                .width(rems(20.))
                .max_height(Some(rems(20.).into()))
        });

        let subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));

        LanguageModelSelector {
            picker,
            _authenticate_all_providers_task: Self::authenticate_all_providers(cx),
            _subscriptions: vec![
                cx.subscribe_in(
                    &LanguageModelRegistry::global(cx),
                    window,
                    Self::handle_language_model_registry_event,
                ),
                subscription,
            ],
        }
    }

    fn handle_language_model_registry_event(
        &mut self,
        _registry: &Entity<LanguageModelRegistry>,
        event: &language_model::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            language_model::Event::ProviderStateChanged
            | language_model::Event::AddedProvider(_)
            | language_model::Event::RemovedProvider(_) => {
                self.picker.update(cx, |this, cx| {
                    let query = this.query(cx);
                    this.delegate.all_models = Arc::new(Self::all_models(cx));
                    // Update matches will automatically drop the previous task
                    // if we get a provider event again
                    this.update_matches(query, window, cx)
                });
            }
            _ => {}
        }
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

    fn all_models(cx: &App) -> GroupedModels {
        let mut recommended = Vec::new();
        let mut recommended_set = HashSet::default();
        for provider in LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
        {
            let models = provider.recommended_models(cx);
            recommended_set.extend(models.iter().map(|model| (model.provider_id(), model.id())));
            recommended.extend(
                provider
                    .recommended_models(cx)
                    .into_iter()
                    .map(move |model| ModelInfo {
                        model: model.clone(),
                        icon: provider.icon(),
                    }),
            );
        }

        let other_models = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .map(|provider| {
                (
                    provider.id(),
                    provider
                        .provided_models(cx)
                        .into_iter()
                        .filter_map(|model| {
                            let not_included =
                                !recommended_set.contains(&(model.provider_id(), model.id()));
                            not_included.then(|| ModelInfo {
                                model: model.clone(),
                                icon: provider.icon(),
                            })
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<IndexMap<_, _>>();

        GroupedModels {
            recommended,
            other: other_models,
        }
    }

    pub fn active_model(&self, cx: &App) -> Option<ConfiguredModel> {
        (self.picker.read(cx).delegate.get_active_model)(cx)
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
}

impl EventEmitter<DismissEvent> for LanguageModelSelector {}

impl Focusable for LanguageModelSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for LanguageModelSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(IntoElement)]
pub struct LanguageModelSelectorPopoverMenu<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    language_model_selector: Entity<LanguageModelSelector>,
    trigger: T,
    tooltip: TT,
    handle: Option<PopoverMenuHandle<LanguageModelSelector>>,
    anchor: Corner,
}

impl<T, TT> LanguageModelSelectorPopoverMenu<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    pub fn new(
        language_model_selector: Entity<LanguageModelSelector>,
        trigger: T,
        tooltip: TT,
        anchor: Corner,
    ) -> Self {
        Self {
            language_model_selector,
            trigger,
            tooltip,
            handle: None,
            anchor,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<LanguageModelSelector>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl<T, TT> RenderOnce for LanguageModelSelectorPopoverMenu<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let language_model_selector = self.language_model_selector.clone();

        PopoverMenu::new("model-switcher")
            .menu(move |_window, _cx| Some(language_model_selector.clone()))
            .trigger_with_tooltip(self.trigger, self.tooltip)
            .anchor(self.anchor)
            .when_some(self.handle.clone(), |menu, handle| menu.with_handle(handle))
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
    }
}

#[derive(Clone)]
struct ModelInfo {
    model: Arc<dyn LanguageModel>,
    icon: IconName,
}

pub struct LanguageModelPickerDelegate {
    language_model_selector: WeakEntity<LanguageModelSelector>,
    on_model_changed: OnModelChanged,
    get_active_model: GetActiveModel,
    all_models: Arc<GroupedModels>,
    filtered_entries: Vec<LanguageModelPickerEntry>,
    selected_index: usize,
}

struct GroupedModels {
    recommended: Vec<ModelInfo>,
    other: IndexMap<LanguageModelProviderId, Vec<ModelInfo>>,
}

impl GroupedModels {
    pub fn new(other: Vec<ModelInfo>, recommended: Vec<ModelInfo>) -> Self {
        let mut other_by_provider: IndexMap<_, Vec<ModelInfo>> = IndexMap::default();
        for model in other {
            let provider = model.model.provider_id();
            if let Some(models) = other_by_provider.get_mut(&provider) {
                models.push(model);
            } else {
                other_by_provider.insert(provider, vec![model]);
            }
        }

        Self {
            recommended,
            other: other_by_provider,
        }
    }

    fn entries(&self) -> Vec<LanguageModelPickerEntry> {
        let mut entries = Vec::new();

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
        self.recommended
            .iter()
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
        let matches = self.bg_executor.block(match_strings(
            &self.candidates,
            &query,
            false,
            100,
            &Default::default(),
            self.bg_executor.clone(),
        ));

        let matched_models: Vec<_> = matches
            .into_iter()
            .map(|m| self.models[m.candidate_id].clone())
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
                    &format!("{}/{}", &model.model.provider_id().0, &model.model.name().0),
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
        let matcher_all = ModelMatcher::new(available_models, bg_executor.clone());

        let recommended = matcher_rec.exact_search(&query);
        let all = matcher_all.fuzzy_search(&query);

        let filtered_models = GroupedModels::new(all, recommended);

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
        self.language_model_selector
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
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
