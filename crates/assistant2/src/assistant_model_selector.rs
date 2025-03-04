use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::{Entity, FocusHandle};
use language_model_selector::{AssistantLanguageModelSelector, LanguageModelSelector};
use settings::update_settings_file;
use std::sync::Arc;
use ui::prelude::*;

pub struct AssistantModelSelector {
    pub selector: Entity<LanguageModelSelector>,
    focus_handle: FocusHandle,
}

impl AssistantModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            selector: cx.new(|cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _cx| settings.set_model(model.clone()),
                        );
                    },
                    window,
                    cx,
                )
            }),
            focus_handle,
        }
    }
}

impl Render for AssistantModelSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        AssistantLanguageModelSelector::new(self.focus_handle.clone(), self.selector.clone())
            .render(window, cx)
    }
}
