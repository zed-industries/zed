use crate::{
    ModelUsageContext,
    language_model_selector::{LanguageModelSelector, language_model_selector},
};
use fs::Fs;
use gpui::{Entity, FocusHandle, SharedString};
use picker::popover_menu::PickerPopoverMenu;
use settings::update_settings_file;
use std::sync::Arc;
use ui::{ButtonLike, PopoverMenuHandle, TintColor, Tooltip, prelude::*};
use zed_actions::agent::ToggleModelSelector;

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
        model_usage_context: ModelUsageContext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selector: cx.new(move |cx| {
                let fs = fs.clone();
                language_model_selector(
                    {
                        let model_context = model_usage_context.clone();
                        move |cx| model_context.configured_model(cx)
                    },
                    move |model, cx| {
                        let provider = model.provider_id().0.to_string();
                        let model_id = model.id().0.to_string();
                        match &model_usage_context {
                            ModelUsageContext::InlineAssistant => {
                                update_settings_file(fs.clone(), cx, move |settings, _cx| {
                                    settings
                                        .agent
                                        .get_or_insert_default()
                                        .set_inline_assistant_model(provider.clone(), model_id);
                                });
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let model = self.selector.read(cx).delegate.active_model(cx);
        let model_name = model
            .as_ref()
            .map(|model| model.model.name().0)
            .unwrap_or_else(|| SharedString::from("Select a Model"));

        let provider_icon = model.as_ref().map(|model| model.provider.icon());
        let color = if self.menu_handle.is_deployed() {
            Color::Accent
        } else {
            Color::Muted
        };

        let focus_handle = self.focus_handle.clone();

        PickerPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .when_some(provider_icon, |this, icon| {
                    this.child(Icon::new(icon).color(color).size(IconSize::XSmall))
                })
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .child(
                    Label::new(model_name)
                        .color(color)
                        .size(LabelSize::Small)
                        .ml_0p5(),
                )
                .child(
                    Icon::new(IconName::ChevronDown)
                        .color(color)
                        .size(IconSize::XSmall),
                ),
            move |_window, cx| {
                Tooltip::for_action_in("Change Model", &ToggleModelSelector, &focus_handle, cx)
            },
            gpui::Corner::TopRight,
            cx,
        )
        .with_handle(self.menu_handle.clone())
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .render(window, cx)
    }
}
