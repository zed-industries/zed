use std::rc::Rc;
use std::sync::Arc;

use acp_thread::{AgentModelIcon, AgentModelInfo, AgentModelSelector};
use fs::Fs;
use gpui::{Entity, FocusHandle};
use picker::popover_menu::PickerPopoverMenu;
use ui::{ButtonLike, PopoverMenuHandle, TintColor, Tooltip, prelude::*};

use crate::acp::{AcpModelSelector, model_selector::acp_model_selector};
use crate::ui::ModelSelectorTooltip;

pub struct AcpModelSelectorPopover {
    selector: Entity<AcpModelSelector>,
    menu_handle: PopoverMenuHandle<AcpModelSelector>,
    focus_handle: FocusHandle,
}

impl AcpModelSelectorPopover {
    pub(crate) fn new(
        selector: Rc<dyn AgentModelSelector>,
        agent_server: Rc<dyn agent_servers::AgentServer>,
        fs: Arc<dyn Fs>,
        menu_handle: PopoverMenuHandle<AcpModelSelector>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle_clone = focus_handle.clone();
        Self {
            selector: cx.new(move |cx| {
                acp_model_selector(
                    selector,
                    agent_server,
                    fs,
                    focus_handle_clone.clone(),
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

    pub fn active_model<'a>(&self, cx: &'a App) -> Option<&'a AgentModelInfo> {
        self.selector.read(cx).delegate.active_model()
    }

    pub fn cycle_favorite_models(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.selector.update(cx, |selector, cx| {
            selector.delegate.cycle_favorite_models(window, cx);
        });
    }
}

impl Render for AcpModelSelectorPopover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selector = self.selector.read(cx);
        let model = selector.delegate.active_model();
        let model_name = model
            .as_ref()
            .map(|model| model.name.clone())
            .unwrap_or_else(|| SharedString::from("Select a Model"));

        let model_icon = model.as_ref().and_then(|model| model.icon.clone());

        let focus_handle = self.focus_handle.clone();

        let (color, icon) = if self.menu_handle.is_deployed() {
            (Color::Accent, IconName::ChevronUp)
        } else {
            (Color::Muted, IconName::ChevronDown)
        };

        let show_cycle_row = selector.delegate.favorites_count() > 1;

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
                .child(Icon::new(icon).color(Color::Muted).size(IconSize::XSmall)),
            tooltip,
            gpui::Corner::BottomRight,
            cx,
        )
        .with_handle(self.menu_handle.clone())
        .render(window, cx)
    }
}
