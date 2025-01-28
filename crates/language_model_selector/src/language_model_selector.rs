use std::sync::Arc;

use feature_flags::ZedPro;
use gpui::{
    Action, AnyElement, App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, WeakEntity,
};
use language_model::{LanguageModel, LanguageModelAvailability, LanguageModelRegistry};
use picker::{Picker, PickerDelegate};
use proto::Plan;
use ui::{prelude::*, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger};
use workspace::ShowConfiguration;

const TRY_ZED_PRO_URL: &str = "https://zed.dev/pro";

type OnModelChanged = Arc<dyn Fn(Arc<dyn LanguageModel>, &App) + 'static>;

pub struct LanguageModelSelector {
    picker: Entity<Picker<LanguageModelPickerDelegate>>,
    /// The task used to update the picker's matches when there is a change to
    /// the language model registry.
    update_matches_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl LanguageModelSelector {
    pub fn new(
        on_model_changed: impl Fn(Arc<dyn LanguageModel>, &App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let on_model_changed = Arc::new(on_model_changed);

        let all_models = Self::all_models(cx);
        let delegate = LanguageModelPickerDelegate {
            language_model_selector: cx.entity().downgrade(),
            on_model_changed: on_model_changed.clone(),
            all_models: all_models.clone(),
            filtered_models: all_models,
            selected_index: 0,
        };

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).max_height(Some(rems(20.).into()))
        });

        LanguageModelSelector {
            picker,
            update_matches_task: None,
            _subscriptions: vec![cx.subscribe_in(
                &LanguageModelRegistry::global(cx),
                window,
                Self::handle_language_model_registry_event,
            )],
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
                let task = self.picker.update(cx, |this, cx| {
                    let query = this.query(cx);
                    this.delegate.all_models = Self::all_models(cx);
                    this.delegate.update_matches(query, window, cx)
                });
                self.update_matches_task = Some(task);
            }
            _ => {}
        }
    }

    fn all_models(cx: &App) -> Vec<ModelInfo> {
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
pub struct LanguageModelSelectorPopoverMenu<T>
where
    T: PopoverTrigger,
{
    language_model_selector: Entity<LanguageModelSelector>,
    trigger: T,
    handle: Option<PopoverMenuHandle<LanguageModelSelector>>,
}

impl<T: PopoverTrigger> LanguageModelSelectorPopoverMenu<T> {
    pub fn new(language_model_selector: Entity<LanguageModelSelector>, trigger: T) -> Self {
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let language_model_selector = self.language_model_selector.clone();

        PopoverMenu::new("model-switcher")
            .menu(move |_window, _cx| Some(language_model_selector.clone()))
            .trigger(self.trigger)
            .anchor(gpui::Corner::BottomRight)
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
    availability: LanguageModelAvailability,
}

pub struct LanguageModelPickerDelegate {
    language_model_selector: WeakEntity<LanguageModelSelector>,
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

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_models.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a model...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_models = self.all_models.clone();
        let current_index = self.selected_index;

        let llm_registry = LanguageModelRegistry::global(cx);

        let configured_providers = llm_registry
            .read(cx)
            .providers()
            .iter()
            .filter(|provider| provider.is_authenticated(cx))
            .map(|provider| provider.id())
            .collect::<Vec<_>>();

        cx.spawn_in(window, |this, mut cx| async move {
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

            this.update_in(&mut cx, |this, window, cx| {
                this.delegate.filtered_models = filtered_models;
                // Preserve selection focus
                let new_index = if current_index >= this.delegate.filtered_models.len() {
                    0
                } else {
                    current_index
                };
                this.delegate.set_selected_index(new_index, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(model_info) = self.filtered_models.get(self.selected_index) {
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

    fn render_header(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
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
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
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

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
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
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(Box::new(zed_actions::OpenAccountSettings), cx)
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
                            window.dispatch_action(ShowConfiguration.boxed_clone(), cx);
                        }),
                )
                .into_any(),
        )
    }
}
