use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::{Entity, FocusHandle, SharedString};
use language_model::LanguageModelRegistry;
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use settings::update_settings_file;
use std::sync::Arc;
use ui::{prelude::*, ButtonLike, PopoverMenuHandle, Tooltip};

use crate::ToggleModelSelector;

pub struct AssistantModelSelector {
    selector: Entity<LanguageModelSelector>,
    menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    focus_handle: FocusHandle,
}

impl AssistantModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<LanguageModelSelector>,
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
            menu_handle,
            focus_handle,
        }
    }
}

impl Render for AssistantModelSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_model = LanguageModelRegistry::read_global(cx).active_model();
        let focus_handle = self.focus_handle.clone();
        let model_name = match active_model {
            Some(model) => model.name().0,
            _ => SharedString::from("No model selected"),
        };

        LanguageModelSelectorPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            Label::new(model_name)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
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
