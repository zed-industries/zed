use std::rc::Rc;

use acp_thread::AgentModelSelector;
use agent_client_protocol as acp;
use gpui::{Entity, FocusHandle};
use picker::popover_menu::PickerPopoverMenu;
use ui::{
    ButtonLike, Context, IntoElement, PopoverMenuHandle, SharedString, Tooltip, Window, prelude::*,
};
use zed_actions::agent::ToggleModelSelector;

use crate::acp::{AcpModelSelector, model_selector::acp_model_selector};

pub struct AcpModelSelectorPopover {
    selector: Entity<AcpModelSelector>,
    menu_handle: PopoverMenuHandle<AcpModelSelector>,
    focus_handle: FocusHandle,
}

impl AcpModelSelectorPopover {
    pub(crate) fn new(
        session_id: acp::SessionId,
        selector: Rc<dyn AgentModelSelector>,
        menu_handle: PopoverMenuHandle<AcpModelSelector>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selector: cx.new(move |cx| acp_model_selector(session_id, selector, window, cx)),
            menu_handle,
            focus_handle,
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu_handle.toggle(window, cx);
    }

    pub fn active_model_name(&self, cx: &App) -> Option<SharedString> {
        self.selector
            .read(cx)
            .delegate
            .active_model()
            .map(|model| model.name.clone())
    }
}

impl Render for AcpModelSelectorPopover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let model = self.selector.read(cx).delegate.active_model();
        let model_name = model
            .as_ref()
            .map(|model| model.name.clone())
            .unwrap_or_else(|| SharedString::from("Select a Model"));

        let model_icon = model.as_ref().and_then(|model| model.icon);

        let focus_handle = self.focus_handle.clone();

        PickerPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .when_some(model_icon, |this, icon| {
                    this.child(Icon::new(icon).color(Color::Muted).size(IconSize::XSmall))
                })
                .child(
                    Label::new(model_name)
                        .color(Color::Muted)
                        .size(LabelSize::Small)
                        .ml_0p5(),
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
            cx,
        )
        .with_handle(self.menu_handle.clone())
        .render(window, cx)
    }
}
