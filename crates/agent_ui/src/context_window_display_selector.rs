use agent_settings::AgentSettings;
use fs::Fs;
use gpui::{Context, Entity, Window, prelude::*};
use settings::{ContextWindowDisplay, Settings as _, update_settings_file};
use std::sync::Arc;
use ui::{
    Button, ContextMenu, ContextMenuEntry, IconPosition, PopoverMenu, PopoverMenuHandle,
    prelude::*,
};

pub struct ContextWindowDisplaySelector {
    menu_handle: PopoverMenuHandle<ContextMenu>,
    fs: Arc<dyn Fs>,
}

impl ContextWindowDisplaySelector {
    pub fn new(fs: Arc<dyn Fs>) -> Self {
        Self {
            menu_handle: PopoverMenuHandle::default(),
            fs,
        }
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let current = AgentSettings::get_global(cx).context_window_display;
        let fs = self.fs.clone();

        ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
            let compact_selected = current == ContextWindowDisplay::Compact;
            menu.push_item(
                ContextMenuEntry::new("Compact")
                    .toggleable(IconPosition::End, compact_selected)
                    .handler({
                        let fs = fs.clone();
                        move |_window, cx| {
                            update_settings_file(fs.clone(), cx, move |settings, _| {
                                settings
                                    .agent
                                    .get_or_insert_default()
                                    .set_context_window_display(ContextWindowDisplay::Compact);
                            });
                        }
                    }),
            );

            let detailed_selected = current == ContextWindowDisplay::Detailed;
            menu.push_item(
                ContextMenuEntry::new("Detailed")
                    .toggleable(IconPosition::End, detailed_selected)
                    .handler({
                        let fs = fs.clone();
                        move |_window, cx| {
                            update_settings_file(fs.clone(), cx, move |settings, _| {
                                settings
                                    .agent
                                    .get_or_insert_default()
                                    .set_context_window_display(ContextWindowDisplay::Detailed);
                            });
                        }
                    }),
            );

            menu
        })
    }
}

impl Render for ContextWindowDisplaySelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = AgentSettings::get_global(cx).context_window_display;
        let label = match current {
            ContextWindowDisplay::Compact => "Compact",
            ContextWindowDisplay::Detailed => "Detailed",
        };

        let icon = if self.menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let this = cx.weak_entity();

        let trigger_button = Button::new("context-window-display-trigger", label)
            .label_size(LabelSize::Small)
            .color(Color::Muted)
            .icon(icon)
            .icon_size(IconSize::XSmall)
            .icon_position(IconPosition::End)
            .icon_color(Color::Muted);

        PopoverMenu::new("context-window-display-selector")
            .trigger(trigger_button)
            .anchor(gpui::Corner::BottomRight)
            .with_handle(self.menu_handle.clone())
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                this.update(cx, |this, cx| this.build_context_menu(window, cx))
                    .ok()
            })
    }
}
