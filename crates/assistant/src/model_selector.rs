use std::sync::Arc;

use crate::{assistant_settings::AssistantSettings, LanguageModelCompletionProvider};
use fs::Fs;
use language_model::LanguageModelRegistry;
use settings::update_settings_file;
use ui::{prelude::*, ContextMenu, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

#[derive(IntoElement)]
pub struct ModelSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<ContextMenu>>,
    fs: Arc<dyn Fs>,
    trigger: T,
}

impl<T: PopoverTrigger> ModelSelector<T> {
    pub fn new(fs: Arc<dyn Fs>, trigger: T) -> Self {
        ModelSelector {
            handle: None,
            fs,
            trigger,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<ContextMenu>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl<T: PopoverTrigger> RenderOnce for ModelSelector<T> {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        let mut menu = PopoverMenu::new("model-switcher");
        if let Some(handle) = self.handle {
            menu = menu.with_handle(handle);
        }

        menu.menu(move |cx| {
            ContextMenu::build(cx, |mut menu, cx| {
                for (index, provider) in LanguageModelRegistry::global(cx)
                    .read(cx)
                    .providers()
                    .enumerate()
                {
                    if index > 0 {
                        menu = menu.separator();
                    }
                    menu = menu.header(provider.name().0);

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
                                let provider = provider.id();
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

                    let selected_model = LanguageModelCompletionProvider::read_global(cx)
                        .active_model()
                        .map(|m| m.id());
                    let selected_provider = LanguageModelCompletionProvider::read_global(cx)
                        .active_provider()
                        .map(|m| m.id());

                    for available_model in available_models {
                        menu = menu.custom_entry(
                            {
                                let id = available_model.id();
                                let provider_id = available_model.provider_id();
                                let model_name = available_model.name().0.clone();
                                let selected_model = selected_model.clone();
                                let selected_provider = selected_provider.clone();
                                move |_| {
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(Label::new(model_name.clone()))
                                        .when(
                                            selected_model.as_ref() == Some(&id)
                                                && selected_provider.as_ref() == Some(&provider_id),
                                            |this| this.child(Icon::new(IconName::Check)),
                                        )
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
