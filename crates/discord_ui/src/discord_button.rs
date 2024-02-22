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
        match Discord::global(cx) {
            Some(discord) => {
                let icon = if discord.read(cx).running {
                    IconName::Discord
                } else {
                    IconName::DiscordDisabled
                };

                let this = cx.view().clone();

                div().child(
                    popover_menu("discord")
                        .menu(move |cx| {
                            Some(this.update(cx, |this, cx| this.build_discord_menu(cx)))
                        })
                        .anchor(AnchorCorner::BottomRight)
                        .trigger(
                            IconButton::new("discord-icon", icon)
                                .tooltip(|cx| Tooltip::text("Discord Rich Presence", cx)),
                        ),
                )
            }
            None => div(),
        }
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
        if let Some(discord) = Discord::global(cx) {
            cx.observe(&discord, |_, _, cx| cx.notify()).detach();
        }

        Self {
            editor_subscription: None,
            language: None,
            file: None,
        }
    }

    pub fn build_discord_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        match Discord::global(cx) {
            Some(discord) => {
                let running = discord.read(cx).running;

                ContextMenu::build(cx, move |menu, _| {
                    let action_text = if running {
                        "Stop Discord Rich Presence"
                    } else {
                        "Start Discord Rich Presence"
                    };
                    menu.entry(action_text, None, toggle_discord_rich_presence)
                })
            }
            None => ContextMenu::build(cx, |menu, _| {
                menu.entry(
                    "Start Discord Rich Presence",
                    None,
                    toggle_discord_rich_presence,
                )
            }),
        }
    }

    fn update_enabled(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let Some(discord) = Discord::global(cx) else {
            return;
        };

        // Do this initial check to prevent self.file from being saved
        // so that it causes the current file to be set as activity when rich presence is start
        if !discord.read(cx).running {
            // Resetting here in the event that a user starts,
            // then stops and happens to start it again on the same file
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
                        if discord.running {
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
        if discord.read(cx).running {
            discord.update(cx, |discord, _| discord.stop_discord_rpc());
        } else {
            discord.update(cx, |discord, _| discord.start_discord_rpc());
        }
    };
}
