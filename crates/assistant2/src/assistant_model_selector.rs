use assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::{FocusHandle, View};
use language_model::LanguageModelRegistry;
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use settings::update_settings_file;
use std::sync::Arc;
use ui::{prelude::*, ButtonLike, PopoverMenuHandle, Tooltip};

use crate::ToggleModelSelector;

pub struct AssistantModelSelector {
    selector: View<LanguageModelSelector>,
    menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    focus_handle: FocusHandle,
}

impl AssistantModelSelector {
    pub(crate) fn new(
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<LanguageModelSelector>,
        focus_handle: FocusHandle,
        cx: &mut WindowContext,
    ) -> Self {
        Self {
            selector: cx.new_view(|cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _cx| settings.set_model(model.clone()),
                        );
                    },
                    cx,
                )
            }),
            menu_handle,
            focus_handle,
        }
    }
}

impl Render for AssistantModelSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_model = LanguageModelRegistry::read_global(cx).active_model();
        let focus_handle = self.focus_handle.clone();

        LanguageModelSelectorPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            div()
                                .overflow_x_hidden()
                                .flex_grow()
                                .whitespace_nowrap()
                                .child(match active_model {
                                    Some(model) => h_flex()
                                        .child(
                                            Label::new(model.name().0)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element(),
                                    _ => Label::new("No model selected")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element(),
                                }),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                )
                .tooltip(move |cx| {
                    Tooltip::for_action_in("Change Model", &ToggleModelSelector, &focus_handle, cx)
                }),
        )
        .with_handle(self.menu_handle.clone())
    }
}
