use std::sync::Arc;

use assistant_settings::{AgentProfile, AssistantSettings};
use assistant_tool::{ToolSource, ToolWorkingSet};
use collections::HashMap;
use gpui::Entity;
use scripting_tool::ScriptingTool;
use settings::Settings as _;
use ui::{prelude::*, ContextMenu, PopoverMenu, Tooltip};

pub struct ToolSelector {
    profiles: HashMap<Arc<str>, AgentProfile>,
    tools: Arc<ToolWorkingSet>,
}

impl ToolSelector {
    pub fn new(tools: Arc<ToolWorkingSet>, cx: &mut Context<Self>) -> Self {
        let settings = AssistantSettings::get_global(cx);
        let mut profiles = settings.profiles.clone();

        let read_only = AgentProfile::read_only();
        if !profiles.contains_key(read_only.name.as_ref()) {
            profiles.insert(read_only.name.clone().into(), read_only);
        }

        let code_writer = AgentProfile::code_writer();
        if !profiles.contains_key(code_writer.name.as_ref()) {
            profiles.insert(code_writer.name.clone().into(), code_writer);
        }

        Self { profiles, tools }
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let profiles = self.profiles.clone();
        let tool_set = self.tools.clone();
        ContextMenu::build_persistent(window, cx, move |mut menu, _window, cx| {
            let icon_position = IconPosition::End;

            menu = menu.header("Profiles");
            for (_id, profile) in profiles.clone() {
                menu = menu.toggleable_entry(profile.name.clone(), false, icon_position, None, {
                    let tools = tool_set.clone();
                    move |_window, cx| {
                        tools.disable_source(ToolSource::Native, cx);
                        tools.enable(
                            ToolSource::Native,
                            &profile
                                .tools
                                .iter()
                                .filter_map(|(tool, enabled)| enabled.then(|| tool.clone()))
                                .collect::<Vec<_>>(),
                        );

                        if profile.tools.contains_key(ScriptingTool::NAME) {
                            tools.enable_scripting_tool();
                        }
                    }
                });
            }

            menu = menu.separator();

            let tools_by_source = tool_set.tools_by_source(cx);

            let all_tools_enabled = tool_set.are_all_tools_enabled();
            menu = menu.toggleable_entry("All Tools", all_tools_enabled, icon_position, None, {
                let tools = tool_set.clone();
                move |_window, cx| {
                    if all_tools_enabled {
                        tools.disable_all_tools(cx);
                    } else {
                        tools.enable_all_tools();
                    }
                }
            });

            for (source, tools) in tools_by_source {
                let mut tools = tools
                    .into_iter()
                    .map(|tool| {
                        let source = tool.source();
                        let name = tool.name().into();
                        let is_enabled = tool_set.is_enabled(&source, &name);

                        (source, name, is_enabled)
                    })
                    .collect::<Vec<_>>();

                if ToolSource::Native == source {
                    tools.push((
                        ToolSource::Native,
                        ScriptingTool::NAME.into(),
                        tool_set.is_scripting_tool_enabled(),
                    ));
                    tools.sort_by(|(_, name_a, _), (_, name_b, _)| name_a.cmp(name_b));
                }

                menu = match &source {
                    ToolSource::Native => menu.separator().header("Zed Tools"),
                    ToolSource::ContextServer { id } => {
                        let all_tools_from_source_enabled =
                            tool_set.are_all_tools_from_source_enabled(&source);

                        menu.separator().header(id).toggleable_entry(
                            "All Tools",
                            all_tools_from_source_enabled,
                            icon_position,
                            None,
                            {
                                let tools = tool_set.clone();
                                let source = source.clone();
                                move |_window, cx| {
                                    if all_tools_from_source_enabled {
                                        tools.disable_source(source.clone(), cx);
                                    } else {
                                        tools.enable_source(&source);
                                    }
                                }
                            },
                        )
                    }
                };

                for (source, name, is_enabled) in tools {
                    menu = menu.toggleable_entry(name.clone(), is_enabled, icon_position, None, {
                        let tools = tool_set.clone();
                        move |_window, _cx| {
                            if name.as_ref() == ScriptingTool::NAME {
                                if is_enabled {
                                    tools.disable_scripting_tool();
                                } else {
                                    tools.enable_scripting_tool();
                                }
                            } else {
                                if is_enabled {
                                    tools.disable(source.clone(), &[name.clone()]);
                                } else {
                                    tools.enable(source.clone(), &[name.clone()]);
                                }
                            }
                        }
                    });
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
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
                Tooltip::text("Customize Tools"),
            )
            .anchor(gpui::Corner::BottomLeft)
    }
}
