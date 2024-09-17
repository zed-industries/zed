use feature_flags::ZedPro;
use gpui::Action;
use gpui::DismissEvent;

use language_model::{LanguageModel, LanguageModelAvailability, LanguageModelRegistry};
use proto::Plan;
use workspace::ShowConfiguration;

use std::sync::Arc;
use ui::ListItemSpacing;

use crate::assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::SharedString;
use gpui::Task;
use picker::{Picker, PickerDelegate};
use settings::update_settings_file;
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

const TRY_ZED_PRO_URL: &str = "https://zed.dev/pro";

#[derive(IntoElement)]
pub struct ModelSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<Picker<ModelPickerDelegate>>>,
    fs: Arc<dyn Fs>,
    trigger: T,
    info_text: Option<SharedString>,
}

pub struct ModelPickerDelegate {
    fs: Arc<dyn Fs>,
    all_models: Vec<ModelInfo>,
    filtered_models: Vec<ModelInfo>,
    selected_index: usize,
}

#[derive(Clone)]
struct ModelInfo {
    model: Arc<dyn LanguageModel>,
    icon: IconName,
    availability: LanguageModelAvailability,
    is_selected: bool,
}

impl<T: PopoverTrigger> ModelSelector<T> {
    pub fn new(fs: Arc<dyn Fs>, trigger: T) -> Self {
        ModelSelector {
            handle: None,
            fs,
            trigger,
            info_text: None,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<Picker<ModelPickerDelegate>>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn with_info_text(mut self, text: impl Into<SharedString>) -> Self {
        self.info_text = Some(text.into());
        self
    }
}

impl PickerDelegate for ModelPickerDelegate {
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
        cx.spawn(|this, mut cx| async move {
            let filtered_models = cx
                .background_executor()
                .spawn(async move {
                    if query.is_empty() {
                        all_models
                    } else {
                        all_models
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
            update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings, _| {
                settings.set_model(model.clone())
            });

            // Update the selection status
            let selected_model_id = model_info.model.id();
            let selected_provider_id = model_info.model.provider_id();
            for model in &mut self.all_models {
                model.is_selected = model.model.id() == selected_model_id
                    && model.model.provider_id() == selected_provider_id;
            }
            for model in &mut self.filtered_models {
                model.is_selected = model.model.id() == selected_model_id
                    && model.model.provider_id() == selected_provider_id;
            }

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        use feature_flags::FeatureFlagAppExt;
        let model_info = self.filtered_models.get(ix)?;
        let show_badges = cx.has_flag::<ZedPro>();
        let provider_name: String = model_info.model.provider_name().0.into();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .start_slot(
                    div().pr_1().child(
                        Icon::new(model_info.icon)
                            .color(Color::Muted)
                            .size(IconSize::Medium),
                    ),
                )
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .font_buffer(cx)
                        .min_w(px(240.))
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new(model_info.model.name().0.clone()))
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
                .end_slot(div().when(model_info.is_selected, |this| {
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
                .border_color(cx.theme().colors().border)
                .p_1()
                .gap_4()
                .justify_between()
                .when(cx.has_flag::<ZedPro>(), |this| {
                    this.child(match plan {
                        // Already a zed pro subscriber
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

impl<T: PopoverTrigger> RenderOnce for ModelSelector<T> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let selected_provider = LanguageModelRegistry::read_global(cx)
            .active_provider()
            .map(|m| m.id());
        let selected_model = LanguageModelRegistry::read_global(cx)
            .active_model()
            .map(|m| m.id());

        let all_models = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .flat_map(|provider| {
                let provider_id = provider.id();
                let icon = provider.icon();
                let selected_model = selected_model.clone();
                let selected_provider = selected_provider.clone();

                provider.provided_models(cx).into_iter().map(move |model| {
                    let model = model.clone();
                    let icon = model.icon().unwrap_or(icon);

                    ModelInfo {
                        model: model.clone(),
                        icon,
                        availability: model.availability(),
                        is_selected: selected_model.as_ref() == Some(&model.id())
                            && selected_provider.as_ref() == Some(&provider_id),
                    }
                })
            })
            .collect::<Vec<_>>();

        let delegate = ModelPickerDelegate {
            fs: self.fs.clone(),
            all_models: all_models.clone(),
            filtered_models: all_models,
            selected_index: 0,
        };

        let picker_view = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into()));
            picker
        });

        PopoverMenu::new("model-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(self.trigger)
            .attach(gpui::AnchorCorner::BottomLeft)
            .when_some(self.handle, |menu, handle| menu.with_handle(handle))
    }
}
