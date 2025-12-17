use std::rc::Rc;
use std::sync::Arc;

use acp_thread::{AgentModelInfo, AgentModelSelector};
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use fs::Fs;
use gpui::{Entity, FocusHandle};
use picker::popover_menu::PickerPopoverMenu;
use settings::Settings as _;
use ui::{ButtonLike, KeyBinding, PopoverMenuHandle, TintColor, Tooltip, prelude::*};
use zed_actions::agent::ToggleModelSelector;

use crate::CycleFavoriteModels;
use crate::acp::{AcpModelSelector, model_selector::acp_model_selector};

pub struct AcpModelSelectorPopover {
    selector: Entity<AcpModelSelector>,
    menu_handle: PopoverMenuHandle<AcpModelSelector>,
    focus_handle: FocusHandle,
}

impl AcpModelSelectorPopover {
    pub(crate) fn new(
        selector: Rc<dyn AgentModelSelector>,
        agent_server: Rc<dyn AgentServer>,
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
        let model = self.selector.read(cx).delegate.active_model();
        let model_name = model
            .as_ref()
            .map(|model| model.name.clone())
            .unwrap_or_else(|| SharedString::from("Select a Model"));

        let model_icon = model.as_ref().and_then(|model| model.icon);

        let focus_handle = self.focus_handle.clone();

        let (color, icon) = if self.menu_handle.is_deployed() {
            (Color::Accent, IconName::ChevronUp)
        } else {
            (Color::Muted, IconName::ChevronDown)
        };

        let tooltip = Tooltip::element({
            move |_, cx| {
                let focus_handle = focus_handle.clone();
                let should_show_cycle_row = !AgentSettings::get_global(cx)
                    .favorite_model_ids()
                    .is_empty();

                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_between()
                            .child(Label::new("Change Model"))
                            .child(KeyBinding::for_action_in(
                                &ToggleModelSelector,
                                &focus_handle,
                                cx,
                            )),
                    )
                    .when(should_show_cycle_row, |this| {
                        this.child(
                            h_flex()
                                .pt_1()
                                .gap_2()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .justify_between()
                                .child(Label::new("Cycle Favorited Models"))
                                .child(KeyBinding::for_action_in(
                                    &CycleFavoriteModels,
                                    &focus_handle,
                                    cx,
                                )),
                        )
                    })
                    .into_any()
            }
        });

        PickerPopoverMenu::new(
            self.selector.clone(),
            ButtonLike::new("active-model")
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .when_some(model_icon, |this, icon| {
                    this.child(Icon::new(icon).color(color).size(IconSize::XSmall))
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
