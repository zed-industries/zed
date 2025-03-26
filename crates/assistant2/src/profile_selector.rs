use std::sync::{Arc, LazyLock};

use anyhow::Result;
use assistant_settings::{AgentProfile, AssistantSettings};
use assistant_tool::{ToolSource, ToolWorkingSet};
use editor::scroll::Autoscroll;
use editor::Editor;
use gpui::{prelude::*, AsyncWindowContext, Entity, Subscription, WeakEntity};
use indexmap::IndexMap;
use regex::Regex;
use settings::{Settings as _, SettingsStore};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, PopoverMenu, Tooltip};
use workspace::{create_and_open_local_file, Workspace};

pub struct ProfileSelector {
    profiles: IndexMap<Arc<str>, AgentProfile>,
    tools: Arc<ToolWorkingSet>,
    _subscriptions: Vec<Subscription>,
}

impl ProfileSelector {
    pub fn new(tools: Arc<ToolWorkingSet>, cx: &mut Context<Self>) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            this.refresh_profiles(cx);
        });

        let mut this = Self {
            profiles: IndexMap::default(),
            tools,
            _subscriptions: vec![settings_subscription],
        };
        this.refresh_profiles(cx);

        this
    }

    fn refresh_profiles(&mut self, cx: &mut Context<Self>) {
        let settings = AssistantSettings::get_global(cx);

        self.profiles = settings.profiles.clone();
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let profiles = self.profiles.clone();
        let tool_set = self.tools.clone();
        ContextMenu::build_persistent(window, cx, move |mut menu, _window, _cx| {
            let icon_position = IconPosition::End;

            menu = menu.header("Profiles");
            for (_id, profile) in profiles.clone() {
                menu = menu.toggleable_entry(profile.name.clone(), false, icon_position, None, {
                    let tools = tool_set.clone();
                    move |_window, cx| {
                        tools.disable_all_tools(cx);

                        tools.enable(
                            ToolSource::Native,
                            &profile
                                .tools
                                .iter()
                                .filter_map(|(tool, enabled)| enabled.then(|| tool.clone()))
                                .collect::<Vec<_>>(),
                        );

                        for (context_server_id, preset) in &profile.context_servers {
                            tools.enable(
                                ToolSource::ContextServer {
                                    id: context_server_id.clone().into(),
                                },
                                &preset
                                    .tools
                                    .iter()
                                    .filter_map(|(tool, enabled)| enabled.then(|| tool.clone()))
                                    .collect::<Vec<_>>(),
                            )
                        }
                    }
                });
            }

            menu = menu.separator();
            menu = menu.item(
                ContextMenuEntry::new("Configure Profiles")
                    .icon(IconName::Pencil)
                    .icon_color(Color::Muted)
                    .handler(move |window, cx| {
                        if let Some(workspace) = window.root().flatten() {
                            let workspace = workspace.downgrade();
                            window
                                .spawn(cx, async |cx| {
                                    Self::open_profiles_setting_in_editor(workspace, cx).await
                                })
                                .detach_and_log_err(cx);
                        }
                    }),
            );

            menu
        })
    }

    async fn open_profiles_setting_in_editor(
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let settings_editor = workspace
            .update_in(cx, |_, window, cx| {
                create_and_open_local_file(paths::settings_file(), window, cx, || {
                    settings::initial_user_settings_content().as_ref().into()
                })
            })?
            .await?
            .downcast::<Editor>()
            .unwrap();

        settings_editor
            .downgrade()
            .update_in(cx, |editor, window, cx| {
                let text = editor.buffer().read(cx).snapshot(cx).text();

                let settings = cx.global::<SettingsStore>();

                let edits =
                    settings.edits_for_update::<AssistantSettings>(
                        &text,
                        |settings| match settings {
                            assistant_settings::AssistantSettingsContent::Versioned(settings) => {
                                match settings {
                                    assistant_settings::VersionedAssistantSettingsContent::V2(
                                        settings,
                                    ) => {
                                        settings.profiles.get_or_insert_with(IndexMap::default);
                                    }
                                    assistant_settings::VersionedAssistantSettingsContent::V1(
                                        _,
                                    ) => {}
                                }
                            }
                            assistant_settings::AssistantSettingsContent::Legacy(_) => {}
                        },
                    );

                if !edits.is_empty() {
                    editor.edit(edits.iter().cloned(), cx);
                }

                let text = editor.buffer().read(cx).snapshot(cx).text();

                static PROFILES_REGEX: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r#"(?P<key>"profiles":)\s*\{"#).unwrap());
                let range = PROFILES_REGEX.captures(&text).and_then(|captures| {
                    captures
                        .name("key")
                        .map(|inner_match| inner_match.start()..inner_match.end())
                });
                if let Some(range) = range {
                    editor.change_selections(
                        Some(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges(vec![range]);
                        },
                    );
                }
            })?;

        anyhow::Ok(())
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let this = cx.entity().clone();
        PopoverMenu::new("tool-selector")
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
            .trigger_with_tooltip(
                IconButton::new("tool-selector-button", IconName::SettingsAlt)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
                Tooltip::text("Customize Tools"),
            )
            .anchor(gpui::Corner::BottomLeft)
    }
}
