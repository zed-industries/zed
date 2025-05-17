use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::{Entity, FocusHandle, SharedString};

use crate::Thread;
use language_model::{ConfiguredModel, LanguageModelRegistry};
use language_model_selector::{
    LanguageModelSelector, LanguageModelSelectorPopoverMenu, ToggleModelSelector,
};
use settings::update_settings_file;
use std::sync::Arc;
use ui::{ButtonLike, PopoverMenuHandle, Tooltip, prelude::*};

#[derive(Clone)]
pub enum ModelType {
    Default(Entity<Thread>),
    InlineAssistant,
}

pub struct AgentModelSelector {
    selector: Entity<LanguageModelSelector>,
    menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    focus_handle: FocusHandle,
}

impl AgentModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<LanguageModelSelector>,
        focus_handle: FocusHandle,
        model_type: ModelType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selector: cx.new(move |cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    {
                        let model_type = model_type.clone();
                        move |cx| match &model_type {
                            ModelType::Default(thread) => thread.read(cx).configured_model(),
                            ModelType::InlineAssistant => {
                                LanguageModelRegistry::read_global(cx).inline_assistant_model()
                            }
                        }
                    },
                    move |model, cx| {
                        let provider = model.provider_id().0.to_string();
                        let model_id = model.id().0.to_string();
                        match &model_type {
                            ModelType::Default(thread) => {
                                thread.update(cx, |thread, cx| {
                                    let registry = LanguageModelRegistry::read_global(cx);
                                    if let Some(provider) = registry.provider(&model.provider_id())
                                    {
                                        thread.set_configured_model(
                                            Some(ConfiguredModel {
                                                provider,
                                                model: model.clone(),
                                            }),
                                            cx,
                                        );
                                    }
                                });
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
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu_handle.toggle(window, cx);
    }
}

impl Render for AgentModelSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        let model = self.selector.read(cx).active_model(cx);
        let model_name = model
            .as_ref()
            .map(|model| model.model.name().0)
            .unwrap_or_else(|| SharedString::from("No model selected"));
        let provider_icon = model
            .as_ref()
            .map(|model| model.provider.icon())
            .unwrap_or_else(|| IconName::Ai);

        LanguageModelSelectorPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .child(
                    Icon::new(provider_icon)
                        .color(Color::Accent)
                        .size(IconSize::Small),
                )
                .child(
                    Label::new(model_name)
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .child(
                    Icon::new(IconName::ChevronDown)
                        .color(Color::Muted)
                        .size(IconSize::XSmall),
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
