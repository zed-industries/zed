use std::sync::Arc;

use crate::{
    assistant_settings::AssistantSettings, completion_provider::LanguageModelCompletionProvider,
    ToggleModelSelector,
};
use fs::Fs;
use gpui::ReadGlobal;
use language_model::registry::LanguageModelRegistry;
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
                    for available_model in LanguageModelRegistry::global(cx).available_models(cx) {
                        menu = menu.custom_entry(
                            {
                                let model_name = available_model.model.name.0.clone();
                                let provider = available_model.provider.0.clone();
                                move |_| {
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(Label::new(model_name.clone()))
                                        .child(div().ml_4().child(
                                            Label::new(provider.clone()).color(Color::Muted),
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
                                        move |settings| settings.set_model(model),
                                    );
                                }
                            },
                        );
                    }
                    menu
                })
                .into()
            })
            .trigger(
                ButtonLike::new("active-model")
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
                                            LanguageModelCompletionProvider::global(cx)
                                                .active_model()
                                                .map(|model| model.name().0.clone())
                                                .unwrap_or_default(),
                                        )
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    ),
                            )
                            .child(
                                div().child(
                                    Icon::new(IconName::ChevronDown)
                                        .color(Color::Muted)
                                        .size(IconSize::XSmall),
                                ),
                            ),
                    )
                    .style(ButtonStyle::Subtle)
                    .tooltip(move |cx| {
                        Tooltip::for_action("Change Model", &ToggleModelSelector, cx)
                    }),
            )
            .attach(gpui::AnchorCorner::BottomLeft)
    }
}
