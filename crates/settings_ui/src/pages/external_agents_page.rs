use std::ops::Range;

use anyhow::Result;
use collections::HashMap;
use editor::{Editor, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
use gpui::{AsyncWindowContext, Entity, ScrollHandle, WeakEntity, WindowHandle, prelude::*};
use itertools::Itertools as _;
use project::agent_server_store::{AgentId, AgentServerStore, ExternalAgentSource};
use settings::{SettingsStore, update_settings_file};
use ui::{
    AiSettingItem, AiSettingItemSource, AiSettingItemStatus, ContextMenu, ContextMenuEntry,
    Divider, DividerColor, PopoverMenu, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{MultiWorkspace, Workspace, create_and_open_local_file};
use zed_actions::OpenBrowser;

use crate::SettingsWindow;

pub(crate) fn render_external_agents_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let agent_server_store = get_agent_server_store(settings_window, cx);

    let agent_list = if let Some(store) = agent_server_store.as_ref() {
        let agents = collect_agents(store, cx);
        if agents.is_empty() {
            render_empty_state(cx)
        } else {
            render_agent_list(agents, cx)
        }
    } else {
        render_no_project_state(cx)
    };

    let add_agent_popover = render_add_agent_popover(settings_window, window, cx);

    v_flex()
        .id("external-agents-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(
            h_flex()
                .w_full()
                .justify_between()
                .items_center()
                .mb_4()
                .child(
                    v_flex()
                        .child(Label::new("External Agents").size(LabelSize::Large))
                        .child(
                            Label::new("All agents connected through the Agent Client Protocol.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(add_agent_popover),
        )
        .child(agent_list)
        .into_any_element()
}

fn get_agent_server_store(
    settings_window: &SettingsWindow,
    cx: &App,
) -> Option<Entity<AgentServerStore>> {
    let original_window = settings_window.original_window.as_ref()?;
    let multi_workspace = original_window.read(cx).ok()?;
    let workspace = multi_workspace.workspaces().next()?;
    let project = workspace.read(cx).project().clone();
    Some(project.read(cx).agent_server_store().clone())
}

/// An external agent listed on the page, paired with the data needed to render
/// its row: the optional extension-provided icon path, a human-readable name,
/// and where the agent came from.
type AgentRow = (
    AgentId,
    Option<SharedString>,
    SharedString,
    ExternalAgentSource,
);

fn collect_agents(store: &Entity<AgentServerStore>, cx: &App) -> Vec<AgentRow> {
    let store = store.read(cx);
    store
        .external_agents()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|name| {
            let icon = store.agent_icon(&name);
            let display_name = store
                .agent_display_name(&name)
                .unwrap_or_else(|| name.0.clone());
            let source = store.agent_source(&name).unwrap_or_default();
            (name, icon, display_name, source)
        })
        .sorted_unstable_by_key(|(_, _, display_name, _)| display_name.to_lowercase())
        .collect()
}

fn render_empty_state(cx: &App) -> AnyElement {
    h_flex()
        .p_4()
        .justify_center()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .rounded_sm()
        .child(
            Label::new("No external agents added yet. Click \"Add Agent\" to get started.")
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .into_any_element()
}

fn render_no_project_state(cx: &App) -> AnyElement {
    h_flex()
        .p_4()
        .justify_center()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .rounded_sm()
        .child(
            Label::new("No active project found. Open a workspace to manage external agents.")
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .into_any_element()
}

fn render_agent_list(agents: Vec<AgentRow>, cx: &mut Context<SettingsWindow>) -> AnyElement {
    v_flex()
        .w_full()
        .gap_1()
        .children(itertools::intersperse_with(
            agents.into_iter().map(|(id, icon, display_name, source)| {
                render_agent(id, icon, display_name, source, cx).into_any_element()
            }),
            || {
                Divider::horizontal()
                    .color(DividerColor::BorderFaded)
                    .into_any_element()
            },
        ))
        .into_any_element()
}

fn render_agent(
    id: AgentId,
    icon: Option<SharedString>,
    display_name: SharedString,
    source: ExternalAgentSource,
    _cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let id_string = id.0.clone();

    let icon = match icon {
        Some(icon_path) => Icon::from_external_svg(icon_path),
        None => Icon::new(IconName::Sparkle),
    }
    .size(IconSize::Small)
    .color(Color::Muted);

    let source_kind = match source {
        ExternalAgentSource::Registry => AiSettingItemSource::Registry,
        ExternalAgentSource::Custom => AiSettingItemSource::Custom,
    };

    let remove_tooltip = match source {
        ExternalAgentSource::Registry => "Remove Registry Agent",
        ExternalAgentSource::Custom => "Remove Custom Agent",
    };

    let remove_button = IconButton::new(
        SharedString::from(format!("uninstall-{}", id_string)),
        IconName::Trash,
    )
    .icon_color(Color::Muted)
    .icon_size(IconSize::Small)
    .tab_index(0isize)
    .tooltip(Tooltip::text(remove_tooltip))
    .on_click(move |_event, _window, cx| {
        remove_agent(&id, source, cx);
    });

    // The connection status of an external agent is tracked per agent-panel
    // session (via the agent panel's `AgentConnectionStore`), which isn't
    // available from the settings window. We therefore render a neutral status;
    // the row still shows the agent's source and supports removal.
    AiSettingItem::new(
        id_string,
        display_name,
        AiSettingItemStatus::Stopped,
        source_kind,
    )
    .icon(icon)
    .action(remove_button)
}

fn remove_agent(id: &AgentId, source: ExternalAgentSource, cx: &mut App) {
    let fs = <dyn fs::Fs>::global(cx);
    let id = id.clone();
    update_settings_file(fs, cx, move |settings, _| {
        let Some(agent_servers) = settings.agent_servers.as_mut() else {
            return;
        };
        // Only remove the entry if it still matches the source we rendered, so a
        // stale row can't clobber an entry that was changed in the meantime.
        let matches_source = agent_servers
            .get(id.0.as_ref())
            .is_some_and(|entry| match source {
                ExternalAgentSource::Registry => {
                    matches!(entry, settings::CustomAgentServerSettings::Registry { .. })
                }
                ExternalAgentSource::Custom => {
                    matches!(entry, settings::CustomAgentServerSettings::Custom { .. })
                }
            });
        if matches_source {
            agent_servers.remove(id.0.as_ref());
        }
    });
}

fn render_add_agent_popover(
    settings_window: &SettingsWindow,
    _window: &mut Window,
    _cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let original_window = settings_window.original_window;

    PopoverMenu::new("add-agent-server-popover")
        .trigger(
            Button::new("add-agent", "Add Agent")
                .style(ButtonStyle::Outlined)
                .start_icon(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .label_size(LabelSize::Small),
        )
        .anchor(gpui::Anchor::TopRight)
        .menu(move |window, cx| {
            Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                menu.entry("Install from Registry", None, move |_window, cx| {
                    if let Some(original_window) = original_window {
                        cx.activate(true);
                        original_window
                            .update(cx, |_, window, cx| {
                                window.activate_window();
                                window.dispatch_action(Box::new(zed_actions::AcpRegistry), cx);
                            })
                            .log_err();
                    }
                })
                .entry("Add Custom Agent", None, move |_window, cx| {
                    if let Some(original_window) = original_window {
                        open_new_custom_agent_in_settings(original_window, cx);
                    }
                })
                .separator()
                .header("Learn More")
                .item(
                    ContextMenuEntry::new("ACP Docs")
                        .icon(IconName::ArrowUpRight)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::End)
                        .handler(move |window, cx| {
                            window.dispatch_action(
                                Box::new(OpenBrowser {
                                    url: "https://agentclientprotocol.com/".into(),
                                }),
                                cx,
                            );
                        }),
                )
            }))
        })
}

/// Opens the user's `settings.json` in the original (editor) window, inserts a
/// scaffold `agent_servers` entry, and selects its name so the user can fill in
/// the executable path. Mirrors the agent panel's "Add Custom Agent" flow.
fn open_new_custom_agent_in_settings(original_window: WindowHandle<MultiWorkspace>, cx: &mut App) {
    cx.activate(true);
    original_window
        .update(cx, |multi_workspace, window, cx| {
            // Use the workspace handed to us by the update closure rather than
            // `Workspace::for_window`, which would read the `MultiWorkspace`
            // entity that this closure is already updating (a double borrow).
            let Some(workspace) = multi_workspace.workspaces().next() else {
                return;
            };
            let workspace = workspace.downgrade();
            window.activate_window();
            window
                .spawn(cx, async move |cx| {
                    add_custom_agent_settings_entry(workspace, cx).await
                })
                .detach_and_log_err(cx);
        })
        .log_err();
}

async fn add_custom_agent_settings_entry(
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let item = workspace
        .update_in(cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?;

    let Some(settings_editor) = item.downcast::<Editor>() else {
        return Ok(());
    };

    settings_editor
        .downgrade()
        .update_in(cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();

            let settings = cx.global::<SettingsStore>();

            let mut unique_server_name = None;
            let Some(edits) = settings
                .edits_for_update(&text, |settings| {
                    let server_name: Option<String> = (0..u8::MAX)
                        .map(|i| {
                            if i == 0 {
                                "your_agent".to_string()
                            } else {
                                format!("your_agent_{}", i)
                            }
                        })
                        .find(|name| {
                            !settings
                                .agent_servers
                                .as_ref()
                                .is_some_and(|agent_servers| {
                                    agent_servers.contains_key(name.as_str())
                                })
                        });
                    if let Some(server_name) = server_name {
                        unique_server_name = Some(SharedString::from(server_name.clone()));
                        settings.agent_servers.get_or_insert_default().insert(
                            server_name,
                            settings::CustomAgentServerSettings::Custom {
                                path: "path_to_executable".into(),
                                args: vec![],
                                env: HashMap::default(),
                                default_mode: None,
                                default_config_options: Default::default(),
                                favorite_config_option_values: Default::default(),
                            },
                        );
                    }
                })
                .log_err()
            else {
                return;
            };

            if edits.is_empty() {
                return;
            }

            let ranges = edits
                .iter()
                .map(|(range, _)| range.clone())
                .collect::<Vec<_>>();

            item.edit(
                edits.into_iter().map(|(range, s)| {
                    (
                        MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                        s,
                    )
                }),
                cx,
            );

            if let Some((unique_server_name, buffer)) =
                unique_server_name.zip(item.buffer().read(cx).as_singleton())
            {
                let snapshot = buffer.read(cx).snapshot();
                if let Some(range) =
                    find_text_in_buffer(&unique_server_name, ranges[0].start, &snapshot)
                {
                    item.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges(vec![
                                MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                            ]);
                        },
                    );
                }
            }
        })
        .log_err();

    Ok(())
}

fn find_text_in_buffer(
    text: &str,
    start: usize,
    snapshot: &language::BufferSnapshot,
) -> Option<Range<usize>> {
    let chars = text.chars().collect::<Vec<char>>();

    let mut offset = start;
    let mut char_offset = 0;
    for c in snapshot.chars_at(start) {
        if char_offset >= chars.len() {
            break;
        }
        offset += 1;

        if c == chars[char_offset] {
            char_offset += 1;
        } else {
            char_offset = 0;
        }
    }

    if char_offset == chars.len() {
        Some(offset.saturating_sub(chars.len())..offset)
    } else {
        None
    }
}
