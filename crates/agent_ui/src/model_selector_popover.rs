use std::rc::Rc;
use std::sync::Arc;

use acp_thread::{AgentModelIcon, AgentModelInfo, AgentModelSelector};
use fs::Fs;
use gpui::{AnyView, Entity, FocusHandle};
use picker::popover_menu::PickerPopoverMenu;
use ui::{ButtonLike, PopoverMenuHandle, TintColor, Tooltip, prelude::*};

use crate::ui::ModelSelectorTooltip;
use crate::{ModelSelector, model_selector::acp_model_selector};

pub struct ModelSelectorPopover {
    selector: Entity<ModelSelector>,
    menu_handle: PopoverMenuHandle<ModelSelector>,
    disabled: bool,
}

impl ModelSelectorPopover {
    pub(crate) fn new(
        selector: Rc<dyn AgentModelSelector>,
        agent_server: Rc<dyn agent_servers::AgentServer>,
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<ModelSelector>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selector: cx.new(move |cx| {
                acp_model_selector(selector, agent_server, fs, focus_handle.clone(), window, cx)
            }),
            menu_handle,
            disabled: false,
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.menu_handle.toggle(window, cx);
    }

    pub fn active_model<'a>(&self, cx: &'a App) -> Option<&'a AgentModelInfo> {
        self.selector.read(cx).delegate.active_model()
    }

    pub fn cycle_favorite_models(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.selector.update(cx, |selector, cx| {
            selector.delegate.cycle_favorite_models(window, cx);
        });
    }
}

impl Render for ModelSelectorPopover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selector = self.selector.read(cx);
        let model = selector.delegate.active_model();
        let model_name = model
            .as_ref()
            .map(|model| model.name.clone())
            .unwrap_or_else(|| SharedString::from("Select a Model"));

        let model_icon = model.as_ref().and_then(|model| model.icon.clone());

        let (color, icon) = if self.menu_handle.is_deployed() {
            (Color::Accent, IconName::ChevronUp)
        } else if self.disabled {
            (Color::Disabled, IconName::ChevronDown)
        } else {
            (Color::Muted, IconName::ChevronDown)
        };

        let show_cycle_row = selector.delegate.favorites_count() > 1;
        let disabled = self.disabled;

        let tooltip: Box<dyn Fn(&mut Window, &mut App) -> AnyView> = if disabled {
            Box::new(Tooltip::text("Disabled until generation is done"))
        } else {
            Box::new(Tooltip::element({
                move |_, _cx| {
                    ModelSelectorTooltip::new()
                        .show_cycle_row(show_cycle_row)
                        .into_any_element()
                }
            }))
        };

        PickerPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .disabled(self.disabled)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .when_some(model_icon, |this, icon| {
                    this.child(
                        match icon {
                            AgentModelIcon::Path(path) => Icon::from_external_svg(path),
                            AgentModelIcon::Named(icon_name) => Icon::new(icon_name),
                        }
                        .color(color)
                        .size(IconSize::XSmall),
                    )
                })
                .child(
                    Label::new(model_name)
                        .color(color)
                        .size(LabelSize::Small)
                        .ml_0p5(),
                )
                .child(
                    Icon::new(icon)
                        .map(|this| {
                            if self.disabled {
                                this.color(Color::Disabled)
                            } else {
                                this.color(Color::Muted)
                            }
                        })
                        .size(IconSize::XSmall),
                ),
            tooltip,
            gpui::Corner::BottomRight,
            cx,
        )
        .with_handle(self.menu_handle.clone())
        .render(window, cx)
    }
}
