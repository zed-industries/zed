use crate::{
    ModelUsageContext,
    language_model_selector::{LanguageModelSelector, language_model_selector},
    ui::ModelSelectorTooltip,
};
use fs::Fs;
use gpui::{Entity, FocusHandle, SharedString};
use language_model::IconOrSvg;
use picker::popover_menu::PickerPopoverMenu;
use settings::update_settings_file;
use std::sync::Arc;
use ui::{ButtonLike, PopoverMenuHandle, TintColor, Tooltip, prelude::*};

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
        let focus_handle_clone = focus_handle.clone();

        Self {
            selector: cx.new(move |cx| {
                language_model_selector(
                    {
                        let model_context = model_usage_context.clone();
                        move |cx| model_context.configured_model(cx)
                    },
                    {
                        let fs = fs.clone();
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
                        }
                    },
                    {
                        let fs = fs.clone();
                        move |model, should_be_favorite, cx| {
                            crate::favorite_models::toggle_in_settings(
                                model,
                                should_be_favorite,
                                fs.clone(),
                                cx,
                            );
                        }
                    },
                    true, // Use popover styles for picker
                    focus_handle_clone,
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

    pub fn active_model(&self, cx: &App) -> Option<language_model::ConfiguredModel> {
        self.selector.read(cx).delegate.active_model(cx)
    }

    pub fn cycle_favorite_models(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.selector.update(cx, |selector, cx| {
            selector.delegate.cycle_favorite_models(window, cx);
        });
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

        let show_cycle_row = self.selector.read(cx).delegate.favorites_count() > 1;

        let focus_handle = self.focus_handle.clone();

        let tooltip = Tooltip::element({
            move |_, _cx| {
                ModelSelectorTooltip::new(focus_handle.clone())
                    .show_cycle_row(show_cycle_row)
                    .into_any_element()
            }
        });

        PickerPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .when_some(provider_icon, |this, icon| {
                    this.child(
                        match icon {
                            IconOrSvg::Svg(path) => Icon::from_external_svg(path),
                            IconOrSvg::Icon(name) => Icon::new(name),
                        }
                        .color(color)
                        .size(IconSize::XSmall),
                    )
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
            tooltip,
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
