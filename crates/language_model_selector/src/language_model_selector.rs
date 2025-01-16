use std::sync::Arc;

use feature_flags::ZedPro;
use gpui::{
    Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    Subscription, Task, View, WeakView,
};
use language_model::{LanguageModel, LanguageModelAvailability, LanguageModelRegistry};
use picker::{Picker, PickerDelegate};
use proto::Plan;
use ui::{prelude::*, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger};
use workspace::ShowConfiguration;

const TRY_ZED_PRO_URL: &str = "https://zed.dev/pro";

type OnModelChanged = Arc<dyn Fn(Arc<dyn LanguageModel>, &AppContext) + 'static>;

pub struct LanguageModelSelector {
    picker: View<Picker<LanguageModelPickerDelegate>>,
    /// The task used to update the picker's matches when there is a change to
    /// the language model registry.
    update_matches_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl LanguageModelSelector {
    pub fn new(
        on_model_changed: impl Fn(Arc<dyn LanguageModel>, &AppContext) + 'static,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let on_model_changed = Arc::new(on_model_changed);

        let all_models = Self::all_models(cx);
        let delegate = LanguageModelPickerDelegate {
            language_model_selector: cx.view().downgrade(),
            on_model_changed: on_model_changed.clone(),
            all_models: all_models.clone(),
            filtered_models: all_models,
            selected_index: 0,
        };

        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into())));

        LanguageModelSelector {
            picker,
            update_matches_task: None,
            _subscriptions: vec![cx.subscribe(
                &LanguageModelRegistry::global(cx),
                Self::handle_language_model_registry_event,
            )],
        }
    }

    fn handle_language_model_registry_event(
        &mut self,
        _registry: Model<LanguageModelRegistry>,
        event: &language_model::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            language_model::Event::ProviderStateChanged
            | language_model::Event::AddedProvider(_)
            | language_model::Event::RemovedProvider(_) => {
                let task = self.picker.update(cx, |this, cx| {
                    let query = this.query(cx);
                    this.delegate.all_models = Self::all_models(cx);
                    this.delegate.update_matches(query, cx)
                });
                self.update_matches_task = Some(task);
            }
            _ => {}
        }
    }

    fn all_models(cx: &AppContext) -> Vec<ModelInfo> {
        LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .flat_map(|provider| {
                let icon = provider.icon();

                provider.provided_models(cx).into_iter().map(move |model| {
                    let model = model.clone();
                    let icon = model.icon().unwrap_or(icon);

                    ModelInfo {
                        model: model.clone(),
                        icon,
                        availability: model.availability(),
                    }
                })
            })
            .collect::<Vec<_>>()
    }
}

impl EventEmitter<DismissEvent> for LanguageModelSelector {}

impl FocusableView for LanguageModelSelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for LanguageModelSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(IntoElement)]
pub struct LanguageModelSelectorPopoverMenu<T>
where
    T: PopoverTrigger,
{
    language_model_selector: View<LanguageModelSelector>,
    trigger: T,
    handle: Option<PopoverMenuHandle<LanguageModelSelector>>,
}

impl<T: PopoverTrigger> LanguageModelSelectorPopoverMenu<T> {
    pub fn new(language_model_selector: View<LanguageModelSelector>, trigger: T) -> Self {
        Self {
            language_model_selector,
            trigger,
            handle: None,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<LanguageModelSelector>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl<T: PopoverTrigger> RenderOnce for LanguageModelSelectorPopoverMenu<T> {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let language_model_selector = self.language_model_selector.clone();

        PopoverMenu::new("model-switcher")
            .menu(move |_cx| Some(language_model_selector.clone()))
            .trigger(self.trigger)
            .attach(gpui::Corner::BottomLeft)
            .when_some(self.handle.clone(), |menu, handle| menu.with_handle(handle))
    }
}

#[derive(Clone)]
struct ModelInfo {
    model: Arc<dyn LanguageModel>,
    icon: IconName,
    availability: LanguageModelAvailability,
}

pub struct LanguageModelPickerDelegate {
    language_model_selector: WeakView<LanguageModelSelector>,
    on_model_changed: OnModelChanged,
    all_models: Vec<ModelInfo>,
    filtered_models: Vec<ModelInfo>,
    selected_index: usize,
}

impl PickerDelegate for LanguageModelPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_models.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_models.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a model...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_models = self.all_models.clone();

        let llm_registry = LanguageModelRegistry::global(cx);

        let configured_providers = llm_registry
            .read(cx)
            .providers()
            .iter()
            .filter(|provider| provider.is_authenticated(cx))
            .map(|provider| provider.id())
            .collect::<Vec<_>>();

        cx.spawn(|this, mut cx| async move {
            let filtered_models = cx
                .background_executor()
                .spawn(async move {
                    let displayed_models = if configured_providers.is_empty() {
                        all_models
                    } else {
                        all_models
                            .into_iter()
                            .filter(|model_info| {
                                configured_providers.contains(&model_info.model.provider_id())
                            })
                            .collect::<Vec<_>>()
                    };

                    if query.is_empty() {
                        displayed_models
                    } else {
                        displayed_models
                            .into_iter()
                            .filter(|model_info| {
                                model_info
                                    .model
                                    .name()
                                    .0
                                    .to_lowercase()
                                    .contains(&query.to_lowercase())
                            })
                            .collect()
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.delegate.filtered_models = filtered_models;
                this.delegate.set_selected_index(0, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(model_info) = self.filtered_models.get(self.selected_index) {
            let model = model_info.model.clone();
            (self.on_model_changed)(model.clone(), cx);

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.language_model_selector
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_header(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        let configured_models_count = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .filter(|provider| provider.is_authenticated(cx))
            .count();

        if configured_models_count > 0 {
            Some(
                Label::new("Configured Models")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1()
                    .mb_0p5()
                    .ml_3()
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        use feature_flags::FeatureFlagAppExt;
        let show_badges = cx.has_flag::<ZedPro>();

        let model_info = self.filtered_models.get(ix)?;
        let provider_name: String = model_info.model.provider_name().0.clone().into();

        let active_provider_id = LanguageModelRegistry::read_global(cx)
            .active_provider()
            .map(|m| m.id());

        let active_model_id = LanguageModelRegistry::read_global(cx)
            .active_model()
            .map(|m| m.id());

        let is_selected = Some(model_info.model.provider_id()) == active_provider_id
            && Some(model_info.model.id()) == active_model_id;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(
                    div().pr_0p5().child(
                        Icon::new(model_info.icon)
                            .color(Color::Muted)
                            .size(IconSize::Medium),
                    ),
                )
                .child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .gap_1p5()
                        .min_w(px(200.))
                        .child(Label::new(model_info.model.name().0.clone()))
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    Label::new(provider_name)
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                                .children(match model_info.availability {
                                    LanguageModelAvailability::Public => None,
                                    LanguageModelAvailability::RequiresPlan(Plan::Free) => None,
                                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro) => {
                                        show_badges.then(|| {
                                            Label::new("Pro")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted)
                                        })
                                    }
                                }),
                        ),
                )
                .end_slot(div().when(is_selected, |this| {
                    this.child(
                        Icon::new(IconName::Check)
                            .color(Color::Accent)
                            .size(IconSize::Small),
                    )
                })),
        )
    }

    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        use feature_flags::FeatureFlagAppExt;

        let plan = proto::Plan::ZedPro;
        let is_trial = false;

        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .gap_4()
                .justify_between()
                .when(cx.has_flag::<ZedPro>(), |this| {
                    this.child(match plan {
                        // Already a Zed Pro subscriber
                        Plan::ZedPro => Button::new("zed-pro", "Zed Pro")
                            .icon(IconName::ZedAssistant)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .icon_position(IconPosition::Start)
                            .on_click(|_, cx| {
                                cx.dispatch_action(Box::new(zed_actions::OpenAccountSettings))
                            }),
                        // Free user
                        Plan::Free => Button::new(
                            "try-pro",
                            if is_trial {
                                "Upgrade to Pro"
                            } else {
                                "Try Pro"
                            },
                        )
                        .on_click(|_, cx| cx.open_url(TRY_ZED_PRO_URL)),
                    })
                })
                .child(
                    Button::new("configure", "Configure")
                        .icon(IconName::Settings)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .on_click(|_, cx| {
                            cx.dispatch_action(ShowConfiguration.boxed_clone());
                        }),
                )
                .into_any(),
        )
    }
}
