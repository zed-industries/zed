use gpui::{
    div, AnchorCorner, AppContext, IntoElement, ParentElement, Render, View, ViewContext,
    WindowContext,
};
use language::File;
use workspace::{
    item::ItemHandle,
    ui::{popover_menu, ButtonCommon, ContextMenu, IconButton, IconName, Tooltip},
    StatusItemView,
};

pub struct DiscordButton {
    display_presence: Option<bool>,
}

impl Render for DiscordButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let enabled = self.display_presence.unwrap_or_else(|| false);

        let icon = match enabled {
            true => IconName::Discord,
            false => IconName::DiscordDisabled,
        };

        let this = cx.view().clone();

        div().child(
            popover_menu("discord")
                .menu(move |cx| Some(this.update(cx, |this, cx| this.build_discord_menu(cx))))
                .anchor(AnchorCorner::BottomRight)
                .trigger(
                    IconButton::new("discord-icon", icon)
                        .tooltip(|cx| Tooltip::text("Discord Rich Presence", cx)),
                ),
        )
    }
}

impl StatusItemView for DiscordButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        println!("SetActivePaneItem has been called");
    }
}

impl DiscordButton {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            display_presence: None,
            file: None,
        }
    }

    pub fn build_discord_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let enabled = self.display_presence.unwrap_or_else(|| false);

        ContextMenu::build(cx, move |mut menu, _| match enabled {
            true => menu.entry(
                "Disable Discord Rich Presence",
                None,
                toggle_discord_rich_presence,
            ),
            false => menu.entry(
                "Enable Discord Rich Presence",
                None,
                toggle_discord_rich_presence,
            ),
        })
    }
}

fn toggle_discord_rich_presence(cx: &mut WindowContext) {
    println!("Toggling DRPC!");
}
