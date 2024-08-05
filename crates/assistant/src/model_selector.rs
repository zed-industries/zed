use std::sync::Arc;

use crate::assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::SharedString;
use language_model::{LanguageModelAvailability, LanguageModelRegistry};
use proto::Plan;
use settings::update_settings_file;
use ui::{prelude::*, ContextMenu, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

#[derive(IntoElement)]
pub struct ModelSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<ContextMenu>>,
    fs: Arc<dyn Fs>,
    trigger: T,
    info_text: Option<SharedString>,
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

    pub fn with_handle(mut self, handle: PopoverMenuHandle<ContextMenu>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn with_info_text(mut self, text: impl Into<SharedString>) -> Self {
        self.info_text = Some(text.into());
        self
    }
}

impl<T: PopoverTrigger> RenderOnce for ModelSelector<T> {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let mut menu = PopoverMenu::new("model-switcher");
        if let Some(handle) = self.handle {
            menu = menu.with_handle(handle);
        }

        let info_text = self.info_text.clone();

        menu.menu(move |cx| {
            ContextMenu::build(cx, |mut menu, cx| {
                if let Some(info_text) = info_text.clone() {
                    menu = menu
                        .custom_row(move |_cx| {
                            Label::new(info_text.clone())
                                .color(Color::Muted)
                                .into_any_element()
                        })
                        .separator();
                }

                for (index, provider) in LanguageModelRegistry::global(cx)
                    .read(cx)
                    .providers()
                    .into_iter()
                    .enumerate()
                {
                    let provider_icon = provider.icon();
                    let provider_name = provider.name().0.clone();

                    if index > 0 {
                        menu = menu.separator();
                    }
                    menu = menu.custom_row(move |_| {
                        h_flex()
                            .pb_1()
                            .gap_1p5()
                            .w_full()
                            .child(
                                Icon::new(provider_icon)
                                    .color(Color::Muted)
                                    .size(IconSize::Small),
                            )
                            .child(Label::new(provider_name.clone()))
                            .into_any_element()
                    });

                    let available_models = provider.provided_models(cx);
                    if available_models.is_empty() {
                        menu = menu.custom_entry(
                            {
                                move |_| {
                                    h_flex()
                                        .w_full()
                                        .gap_1()
                                        .child(Icon::new(IconName::Settings))
                                        .child(Label::new("Configure"))
                                        .into_any()
                                }
                            },
                            {
                                let provider = provider.clone();
                                move |cx| {
                                    LanguageModelRegistry::global(cx).update(
                                        cx,
                                        |completion_provider, cx| {
                                            completion_provider
                                                .set_active_provider(Some(provider.clone()), cx);
                                        },
                                    );
                                }
                            },
                        );
                    }

                    let selected_provider = LanguageModelRegistry::read_global(cx)
                        .active_provider()
                        .map(|m| m.id());
                    let selected_model = LanguageModelRegistry::read_global(cx)
                        .active_model()
                        .map(|m| m.id());

                    for available_model in available_models {
                        menu = menu.custom_entry(
                            {
                                let id = available_model.id();
                                let provider_id = available_model.provider_id();
                                let model_name = available_model.name().0.clone();
                                let availability = available_model.availability();
                                let selected_model = selected_model.clone();
                                let selected_provider = selected_provider.clone();
                                move |cx| {
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .font_buffer(cx)
                                        .min_w(px(260.))
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(Label::new(model_name.clone()))
                                                .children(match availability {
                                                    LanguageModelAvailability::Public => None,
                                                    LanguageModelAvailability::RequiresPlan(
                                                        Plan::Free,
                                                    ) => None,
                                                    LanguageModelAvailability::RequiresPlan(
                                                        Plan::ZedPro,
                                                    ) => Some(
                                                        Label::new("Pro")
                                                            .size(LabelSize::XSmall)
                                                            .color(Color::Muted),
                                                    ),
                                                }),
                                        )
                                        .child(div().when(
                                            selected_model.as_ref() == Some(&id)
                                                && selected_provider.as_ref() == Some(&provider_id),
                                            |this| {
                                                this.child(
                                                    Icon::new(IconName::Check)
                                                        .color(Color::Accent)
                                                        .size(IconSize::Small),
                                                )
                                            },
                                        ))
                                        .into_any()
                                }
                            },
                            {
                                let fs = self.fs.clone();
                                let model = available_model.clone();
                                move |cx| {
                                    let model = model.clone();
                                    update_settings_file::<AssistantSettings>(
                                        fs.clone(),
                                        cx,
                                        move |settings, _| settings.set_model(model),
                                    );
                                }
                            },
                        );
                    }
                }
                menu
            })
            .into()
        })
        .trigger(self.trigger)
        .attach(gpui::AnchorCorner::BottomLeft)
    }
}
