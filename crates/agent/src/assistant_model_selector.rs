use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::{Entity, FocusHandle, SharedString};
use language_model::LanguageModelRegistry;
use language_model_selector::{
    LanguageModelSelector, LanguageModelSelectorPopoverMenu, ToggleModelSelector,
};
use settings::update_settings_file;
use std::sync::Arc;
use ui::{ButtonLike, PopoverMenuHandle, Tooltip, prelude::*};

#[derive(Clone, Copy)]
pub enum ModelType {
    Default,
    InlineAssistant,
}

pub struct AssistantModelSelector {
    selector: Entity<LanguageModelSelector>,
    menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    focus_handle: FocusHandle,
    model_type: ModelType,
}

impl AssistantModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<LanguageModelSelector>,
        focus_handle: FocusHandle,
        model_type: ModelType,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            selector: cx.new(|cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    move |model, cx| {
                        let provider = model.provider_id().0.to_string();
                        let model_id = model.id().0.to_string();

                        match model_type {
                            ModelType::Default => {
                                update_settings_file::<AssistantSettings>(
                                    fs.clone(),
                                    cx,
                                    move |settings, _cx| {
                                        settings.set_model(model.clone());
                                    },
                                );
                            }
                            ModelType::InlineAssistant => {
                                update_settings_file::<AssistantSettings>(
                                    fs.clone(),
                                    cx,
                                    move |settings, _cx| {
                                        settings.set_inline_assistant_model(
                                            provider.clone(),
                                            model_id.clone(),
                                        );
                                    },
                                );
                            }
                        }
                    },
                    window,
                    cx,
                )
            }),
            menu_handle,
            focus_handle,
            model_type,
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu_handle.toggle(window, cx);
    }
}

impl Render for AssistantModelSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        let model_registry = LanguageModelRegistry::read_global(cx);
        let model = match self.model_type {
            ModelType::Default => model_registry.default_model(),
            ModelType::InlineAssistant => model_registry.inline_assistant_model(),
        };
        let (model_name, model_icon) = match model {
            Some(model) => (model.model.name().0, Some(model.provider.icon())),
            _ => (SharedString::from("No model selected"), None),
        };

        LanguageModelSelectorPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_0p5()
                        .children(
                            model_icon.map(|icon| {
                                Icon::new(icon).color(Color::Muted).size(IconSize::Small)
                            }),
                        )
                        .child(
                            Label::new(model_name)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .ml_1(),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                ),
            move |window, cx| {
                Tooltip::for_action_in(
                    "Change Model",
                    &ToggleModelSelector,
                    &focus_handle,
                    window,
                    cx,
                )
            },
            gpui::Corner::BottomRight,
        )
        .with_handle(self.menu_handle.clone())
    }
}
