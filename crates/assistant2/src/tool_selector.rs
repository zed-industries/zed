use std::sync::Arc;

use assistant_tool::{ToolSource, ToolWorkingSet};
use gpui::Entity;
use ui::{prelude::*, ContextMenu, IconButtonShape, PopoverMenu, Tooltip};

pub struct ToolSelector {
    tools: Arc<ToolWorkingSet>,
}

impl ToolSelector {
    pub fn new(tools: Arc<ToolWorkingSet>, _cx: &mut Context<Self>) -> Self {
        Self { tools }
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, window, cx| {
            let tools_by_source = self.tools.tools_by_source(cx);

            for (source, tools) in tools_by_source {
                menu = match source {
                    ToolSource::Native => menu.header("Zed"),
                    ToolSource::ContextServer { id } => menu.separator().header(id),
                };

                for tool in tools {
                    menu = menu.toggleable_entry(
                        tool.name(),
                        false,
                        IconPosition::End,
                        None,
                        |_window, _cx| {},
                    );
                }
            }

            menu
        })
    }
}

impl Render for ToolSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let this = cx.entity().clone();
        PopoverMenu::new("tool-selector")
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
            .trigger_with_tooltip(
                IconButton::new("tool-selector-button", IconName::SettingsAlt)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
                Tooltip::text("Customize Tools"),
            )
            .anchor(gpui::Corner::BottomRight)
    }
}
