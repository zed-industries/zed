use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::FocusHandle;
use language_model_selector::{assistant_language_model_selector, LanguageModelSelector};
use settings::update_settings_file;
use std::sync::Arc;
use ui::{prelude::*, PopoverMenuHandle};

pub struct AssistantModelSelector {
    menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
}

impl AssistantModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        focus_handle: FocusHandle,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            fs,
            focus_handle,
            menu_handle: PopoverMenuHandle::default(),
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu_handle.toggle(window, cx);
    }
}

impl Render for AssistantModelSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fs_clone = self.fs.clone();
        assistant_language_model_selector(
            self.focus_handle.clone(),
            Some(self.menu_handle.clone()),
            cx,
            move |model, cx| {
                update_settings_file::<AssistantSettings>(
                    fs_clone.clone(),
                    cx,
                    move |settings, _| settings.set_model(model.clone()),
                );
            },
        )
    }
}
