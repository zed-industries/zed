use discord::Discord;
use editor::Editor;
use gpui::{
    div, AnchorCorner, Entity, IntoElement, ParentElement, Render, Subscription, View, ViewContext,
    WindowContext,
};
use language::{File, Language};
use std::path::Path;
use std::sync::Arc;
use workspace::{
    item::ItemHandle,
    ui::{popover_menu, ButtonCommon, ContextMenu, IconButton, IconName, Tooltip},
    StatusItemView,
};

pub struct DiscordButton {
    editor_subscription: Option<(Subscription, usize)>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
}

impl Render for DiscordButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(discord) = Discord::global(cx) else {
            return div();
        };

        let running = discord.read(cx).running.unwrap_or_else(|| false);

        let icon = match running {
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
        if let Some(editor) = item.map(|item| item.act_as::<Editor>(cx)).flatten() {
            self.editor_subscription = Some((
                cx.observe(&editor, Self::update_enabled),
                editor.entity_id().as_u64() as usize,
            ));
            self.update_enabled(editor, cx);
        } else {
            self.editor_subscription = None;
            self.language = None;
        }
        cx.notify();
    }
}

impl DiscordButton {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Discord::global(cx).map(|discord| cx.observe(&discord, |_, _, cx| cx.notify()).detach());

        Self {
            editor_subscription: None,
            language: None,
            file: None,
        }
    }

    pub fn build_discord_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let Some(discord) = Discord::global(cx) else {
            return ContextMenu::build(cx, move |menu, _| {
                menu.entry(
                    "Start Discord Rich Presence",
                    None,
                    toggle_discord_rich_presence,
                )
            });
        };

        let running = discord.read(cx).running.unwrap_or_else(|| false);

        ContextMenu::build(cx, move |menu, _| match running {
            true => menu.entry(
                "Stop Discord Rich Presence",
                None,
                toggle_discord_rich_presence,
            ),
            false => menu.entry(
                "Start Discord Rich Presence",
                None,
                toggle_discord_rich_presence,
            ),
        })
    }

    pub fn update_enabled(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let Some(discord) = Discord::global(cx) else {
            return;
        };

        let running = discord.read(cx).running.unwrap_or_else(|| false);

        // Do this initial check to prevent self.file from being saved
        // so that it causes the current file to be set as activity when rich presence is start
        if !running {
            // Resetting here in the event that a user starts, then stops and happens to start it again on the same file
            self.file = None;
            return;
        }

        let editor = editor.read(cx);
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let suggestion_anchor = editor.selections.newest_anchor().start;
        let language = snapshot.language_at(suggestion_anchor);
        let file = snapshot.file_at(suggestion_anchor).cloned();

        if self.file.as_ref().map(Arc::as_ptr) != file.as_ref().map(Arc::as_ptr) {
            if let Some(file) = file.as_ref() {
                let fullpath = file.full_path(cx).to_str().unwrap_or_default().to_string();
                // Not sure if there's a better way to get the project name here
                let path = Path::new(&fullpath);
                let project = path
                    .iter()
                    .next()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let filepath = file.path().to_str().unwrap_or_default().to_string();
                let file_language = language
                    .as_ref()
                    .map_or_else(|| "".to_string(), |lang| lang.name().to_string());
                if let Some(discord) = Discord::global(cx) {
                    discord.update(cx, |discord, _cx| {
                        if discord.running.unwrap_or_else(|| false) {
                            discord.set_activity(filepath, file_language, project);
                        }
                    })
                }
            }
        }

        self.language = language.cloned();
        self.file = file;

        cx.notify()
    }
}

fn toggle_discord_rich_presence(cx: &mut WindowContext) {
    if let Some(discord) = Discord::global(cx) {
        let running = discord.read(cx).running.unwrap_or_else(|| false);

        match running {
            false => discord.update(cx, |discord, _cx| {
                discord.start_discord_rpc();
            }),
            true => discord.update(cx, |discord, _cx| {
                discord.stop_discord_rpc();
            }),
        }
    };
}
