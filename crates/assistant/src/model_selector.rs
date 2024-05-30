use std::sync::Arc;

use crate::{assistant_settings::AssistantSettings, CompletionProvider, ToggleModelSelector};
use fs::Fs;
use settings::update_settings_file;
use ui::{popover_menu, prelude::*, ButtonLike, ContextMenu, PopoverMenuHandle, Tooltip};

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
        popover_menu("model-switcher")
            .with_handle(self.handle)
            .menu(move |cx| {
                ContextMenu::build(cx, |mut menu, cx| {
                    for model in CompletionProvider::global(cx).available_models() {
                        menu = menu.custom_entry(
                            {
                                let model = model.clone();
                                move |_| Label::new(model.display_name()).into_any_element()
                            },
                            {
                                let fs = self.fs.clone();
                                let model = model.clone();
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
                                            CompletionProvider::global(cx).model().display_name(),
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
            .anchor(gpui::AnchorCorner::BottomRight)
    }
}
