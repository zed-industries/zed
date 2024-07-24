use std::sync::Arc;

use crate::{
    assistant_settings::AssistantSettings, LanguageModelCompletionProvider, ToggleModelSelector,
};
use fs::Fs;
use language_model::LanguageModelRegistry;
use settings::update_settings_file;
use ui::{prelude::*, ButtonLike, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip};

#[derive(IntoElement)]
pub struct ModelSelector {
    handle: PopoverMenuHandle<ContextMenu>,
    fs: Arc<dyn Fs>,
}

impl ModelSelector {
    pub fn new(handle: PopoverMenuHandle<ContextMenu>, fs: Arc<dyn Fs>) -> Self {
        ModelSelector { handle, fs }
    }
}

impl RenderOnce for ModelSelector {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        PopoverMenu::new("model-switcher")
            .with_handle(self.handle)
            .menu(move |cx| {
                ContextMenu::build(cx, |mut menu, cx| {
                    for (provider, available_models) in LanguageModelRegistry::global(cx)
                        .read(cx)
                        .available_models_grouped_by_provider(cx)
                    {
                        menu = menu.header(provider.0.clone());

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
                                        LanguageModelCompletionProvider::global(cx).update(
                                            cx,
                                            |completion_provider, cx| {
                                                completion_provider
                                                    .set_active_provider(provider.clone(), cx)
                                            },
                                        );
                                    }
                                },
                            );
                        }

                        for available_model in available_models {
                            menu = menu.custom_entry(
                                {
                                    let model_name = available_model.name().0.clone();
                                    move |_| {
                                        h_flex()
                                            .w_full()
                                            .child(Label::new(model_name.clone()))
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
            .trigger(
                ButtonLike::new("active-model")
                    .style(ButtonStyle::Subtle)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_0p5()
                            .child(
                                div()
                                    .overflow_x_hidden()
                                    .flex_grow()
                                    .whitespace_nowrap()
                                    .child(
                                        Label::new(
                                            LanguageModelCompletionProvider::read_global(cx)
                                                .active_model()
                                                .map(|model| model.name().0)
                                                .unwrap_or_else(|| "No model selected".into()),
                                        )
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    ),
                            )
                            .child(
                                Icon::new(IconName::ChevronDown)
                                    .color(Color::Muted)
                                    .size(IconSize::XSmall),
                            ),
                    )
                    .tooltip(move |cx| {
                        Tooltip::for_action("Change Model", &ToggleModelSelector, cx)
                    }),
            )
            .attach(gpui::AnchorCorner::BottomLeft)
    }
}
