use std::ops::Range;

use anyhow::Result;
use collections::HashMap;
use editor::{Editor, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
use gpui::{
    AsyncWindowContext, Entity, FocusHandle, Focusable as _, ReadGlobal as _, ScrollHandle,
    WeakEntity, WindowHandle, prelude::*,
};
use itertools::Itertools as _;
use project::agent_server_store::{AgentId, AgentServerStore, ExternalAgentSource};
use settings::{
    AgentConfigOptionValue, CustomAgentServerSettings, SettingsStore, update_settings_file,
};
use ui::{
    AiSettingItem, AiSettingItemSource, AiSettingItemStatus, ContextMenu, ContextMenuEntry,
    Divider, PopoverMenu, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{MultiWorkspace, Workspace, create_and_open_local_file};

use crate::SettingsWindow;

pub(crate) fn render_external_agents_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
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

    v_flex()
        .id("external-agents-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(Label::new("External Agents"))
        .child(
            Label::new("Agents connected through the Agent Client Protocol.")
                .size(LabelSize::Small)
                .color(Color::Muted),
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

/// Reads the raw, user-configured settings for a custom agent so the edit form
/// can be pre-filled. Reading the parsed settings (rather than the resolved
/// runtime server) keeps this resilient to malformed `settings.json`: the
/// settings layer drops individual bad fields instead of failing.
fn custom_agent_settings(id: &AgentId, cx: &App) -> Option<CustomAgentServerSettings> {
    SettingsStore::global(cx)
        .get_content_for_file(settings::SettingsFile::User)?
        .agent_servers
        .as_ref()?
        .get(id.0.as_ref())
        .cloned()
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
            || Divider::horizontal().into_any_element(),
        ))
        .into_any_element()
}

fn render_agent(
    id: AgentId,
    icon: Option<SharedString>,
    display_name: SharedString,
    source: ExternalAgentSource,
    cx: &mut Context<SettingsWindow>,
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

    // Only custom agents are editable here; registry agents are managed via the
    // ACP registry and only support removal.
    let configure_button = (source == ExternalAgentSource::Custom).then(|| {
        IconButton::new(format!("configure-{}", id_string), IconName::Settings)
            .icon_color(Color::Muted)
            .icon_size(IconSize::Small)
            .size(ButtonSize::Medium)
            .tab_index(0isize)
            .tooltip(Tooltip::text("Configure Agent"))
            .on_click(cx.listener({
                let id = id.clone();
                move |this, _event, window, cx| {
                    let existing =
                        custom_agent_settings(&id, cx).map(|settings| (id.clone(), settings));
                    open_custom_agent_form(this, existing, window, cx);
                }
            }))
    });

    let remove_tooltip = match source {
        ExternalAgentSource::Registry => "Remove Registry Agent",
        ExternalAgentSource::Custom => "Remove Custom Agent",
    };

    let remove_button = IconButton::new(format!("uninstall-{}", id_string), IconName::Trash)
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .size(ButtonSize::Medium)
        .tab_index(0isize)
        .tooltip(Tooltip::text(remove_tooltip))
        .on_click(move |_event, _window, cx| {
            remove_agent(&id, source, cx);
        });

    // The connection status of an external agent is tracked per agent-panel
    // session (via the agent panel's `AgentConnectionStore`), which isn't
    // available from the settings window. We therefore render a neutral status;
    // the row still shows the agent's source and supports configure/removal.
    AiSettingItem::new(
        id_string,
        display_name,
        AiSettingItemStatus::Stopped,
        source_kind,
    )
    .icon(icon)
    .when_some(configure_button, |this, button| this.action(button))
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
                    matches!(entry, CustomAgentServerSettings::Registry { .. })
                }
                ExternalAgentSource::Custom => {
                    matches!(entry, CustomAgentServerSettings::Custom { .. })
                }
            });
        if matches_source {
            agent_servers.remove(id.0.as_ref());
        }
    });
}

pub(crate) fn render_add_agent_popover(
    settings_window: &SettingsWindow,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let original_window = settings_window.original_window;
    // Stable handle so the button keeps focus state across renders and can show a
    // focus ring even when the page is opened (and the button auto-focused) via a
    // mouse click, where `focus_visible` styling is suppressed.
    let focus_handle = settings_window
        .external_agent_add_focus_handle
        .clone()
        .tab_index(0)
        .tab_stop(true);
    let border_color = focus_ring_color(&focus_handle, window, cx);
    let settings_window = cx.entity().downgrade();

    let popover = PopoverMenu::new("add-agent-server-popover")
        .trigger(
            Button::new("add-agent", "Add Agent")
                .style(ButtonStyle::Outlined)
                .track_focus(&focus_handle)
                .start_icon(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .label_size(LabelSize::Small),
        )
        .anchor(gpui::Anchor::TopRight)
        .menu(move |window, cx| {
            let settings_window = settings_window.clone();
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
                .entry("Add Custom Agent", None, move |window, cx| {
                    settings_window
                        .update(cx, |this, cx| {
                            open_custom_agent_form(this, None, window, cx);
                        })
                        .log_err();
                })
                .separator()
                .header("Learn More")
                .item(
                    ContextMenuEntry::new("ACP Docs")
                        .icon(IconName::ArrowUpRight)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::End)
                        .handler(|_window, cx| cx.open_url("https://agentclientprotocol.com/")),
                )
            }))
        });

    div()
        .rounded_md()
        .border_1()
        .border_color(border_color)
        .child(popover)
}

// === Custom external agent add/edit form ===

struct KeyValueRow {
    key: Entity<Editor>,
    value: Entity<Editor>,
}

/// Editor-backed state for the custom external agent add/edit form.
pub(crate) struct CustomAgentForm {
    /// `Some` when editing an existing agent (used to remove the old entry on rename).
    original_id: Option<AgentId>,
    name: Entity<Editor>,
    command: Entity<Editor>,
    args: Entity<Editor>,
    env: Vec<KeyValueRow>,
    /// Advanced fields not surfaced by the form. They're preserved verbatim so
    /// editing the basic settings doesn't drop a user's hand-written config.
    default_mode: Option<String>,
    default_config_options: HashMap<String, AgentConfigOptionValue>,
    favorite_config_option_values: HashMap<String, Vec<String>>,
    /// Stable handles for the Cancel/Save buttons so they can render a focus
    /// ring. `Filled`/`Subtle` buttons only get a subtle `focus_visible`
    /// background change otherwise, which is hard to see.
    cancel_focus_handle: FocusHandle,
    save_focus_handle: FocusHandle,
    error: Option<SharedString>,
}

impl CustomAgentForm {
    fn new(
        existing: Option<(AgentId, CustomAgentServerSettings)>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> Self {
        let original_id = existing.as_ref().map(|(id, _)| id.clone());
        let name_initial = original_id.as_ref().map(|id| id.0.to_string());

        let mut command_initial = None;
        let mut args_initial = None;
        let mut env = Vec::new();
        let mut default_mode = None;
        let mut default_config_options = HashMap::default();
        let mut favorite_config_option_values = HashMap::default();

        // Pre-fill from the raw settings so invalid values typed directly into
        // settings.json still load into the form for correction.
        if let Some((_, settings)) = existing.as_ref() {
            match settings {
                CustomAgentServerSettings::Custom {
                    path,
                    args,
                    env: env_map,
                    default_mode: mode,
                    default_config_options: config_options,
                    favorite_config_option_values: favorites,
                } => {
                    command_initial = Some(path.to_string_lossy().to_string());
                    if !args.is_empty() {
                        args_initial = Some(args.join(" "));
                    }
                    for (key, value) in sorted_pairs(env_map) {
                        env.push(new_kv_row(Some(&key), Some(&value), window, cx));
                    }
                    default_mode = mode.clone();
                    default_config_options = config_options.clone();
                    favorite_config_option_values = favorites.clone();
                }
                CustomAgentServerSettings::Registry {
                    env: env_map,
                    default_mode: mode,
                    default_config_options: config_options,
                    favorite_config_option_values: favorites,
                } => {
                    for (key, value) in sorted_pairs(env_map) {
                        env.push(new_kv_row(Some(&key), Some(&value), window, cx));
                    }
                    default_mode = mode.clone();
                    default_config_options = config_options.clone();
                    favorite_config_option_values = favorites.clone();
                }
            }
        }

        Self {
            original_id,
            name: new_input("my-agent", name_initial.as_deref(), window, cx),
            command: new_input("/path/to/agent", command_initial.as_deref(), window, cx),
            args: new_input("--flag value", args_initial.as_deref(), window, cx),
            env,
            default_mode,
            default_config_options,
            favorite_config_option_values,
            cancel_focus_handle: cx.focus_handle(),
            save_focus_handle: cx.focus_handle(),
            error: None,
        }
    }
}

fn sorted_pairs(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

fn new_input(
    placeholder: &str,
    initial: Option<&str>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> Entity<Editor> {
    let placeholder = placeholder.to_string();
    let initial = initial.map(|text| text.to_string());
    cx.new(|cx| {
        let mut editor = Editor::single_line(window, cx);
        editor.set_placeholder_text(placeholder.as_str(), window, cx);
        if let Some(text) = initial {
            editor.set_text(text, window, cx);
        }
        editor
    })
}

fn new_kv_row(
    key: Option<&str>,
    value: Option<&str>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> KeyValueRow {
    KeyValueRow {
        key: new_input("Key", key, window, cx),
        value: new_input("Value", value, window, cx),
    }
}

/// Creates the form state and pushes the form sub-page onto the stack.
pub(crate) fn open_custom_agent_form(
    settings_window: &mut SettingsWindow,
    existing: Option<(AgentId, CustomAgentServerSettings)>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let is_edit = existing.is_some();
    settings_window.custom_agent_form = Some(CustomAgentForm::new(existing, window, cx));

    let title = if is_edit {
        "Configure External Agent"
    } else {
        "Add Custom Agent"
    };

    settings_window.push_dynamic_sub_page(
        title,
        "Agent Configuration",
        Some("agent_servers"),
        false,
        render_custom_agent_form_page,
        window,
        cx,
    );
}

fn render_custom_agent_form_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(form) = settings_window.custom_agent_form.as_ref() else {
        return div().into_any_element();
    };
    let error = form.error.clone();

    let fields = v_flex()
        .w_full()
        .gap_4()
        .child(
            crate::render_settings_item_layout(
                settings_window,
                "Agent Name",
                "Required. A unique name used to identify this agent.",
                input_box(&form.name, cx).into_any_element(),
                None,
                None,
                None,
                false,
                cx,
            )
            .into_any_element(),
        )
        .child(
            crate::render_settings_item_layout(
                settings_window,
                "Command",
                "Required. Path to the executable that launches the agent.",
                input_box(&form.command, cx).into_any_element(),
                None,
                None,
                None,
                false,
                cx,
            )
            .into_any_element(),
        )
        .child(
            crate::render_settings_item_layout(
                settings_window,
                "Arguments",
                "Space-separated arguments passed to the command.",
                input_box(&form.args, cx).into_any_element(),
                None,
                None,
                None,
                false,
                cx,
            )
            .into_any_element(),
        )
        .child(render_env_section(settings_window, &form.env, cx))
        .when_some(error, |this, error| this.child(render_form_error(error)))
        .child(render_form_actions(form, window, cx));

    v_flex()
        .id("custom-agent-form-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(fields)
        .into_any_element()
}

fn input_box(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors();
    // All form inputs share tab index 0, so tab order follows render (insertion)
    // order. Tracking the editor's focus handle makes the field a tab stop and
    // routes keyboard focus into the editor when tabbed to.
    let focus_handle = editor.focus_handle(cx).tab_index(0).tab_stop(true);
    h_flex()
        .min_w_64()
        .py_1()
        .px_2()
        .h_8()
        .rounded_md()
        .border_1()
        .border_color(colors.border)
        .bg(colors.editor_background)
        .track_focus(&focus_handle)
        .focus(|style| style.border_color(colors.border_focused))
        .child(editor.clone())
}

fn render_env_section(
    settings_window: &SettingsWindow,
    rows: &[KeyValueRow],
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    // The right-hand control column is narrower than a full row, so each
    // variable stacks its key above its value (with the remove affordance next
    // to the value) to stay readable.
    let control = v_flex()
        .min_w_64()
        .gap_2()
        .children(rows.iter().enumerate().map(|(ix, row)| {
            v_flex().gap_1().child(input_box(&row.key, cx)).child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(input_box(&row.value, cx))
                    .child(
                        IconButton::new(("custom-agent-env-remove", ix), IconName::Close)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tab_index(0isize)
                            .tooltip(Tooltip::text("Remove"))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                if let Some(form) = this.custom_agent_form.as_mut()
                                    && ix < form.env.len()
                                {
                                    form.env.remove(ix);
                                }
                                cx.notify();
                            })),
                    ),
            )
        }))
        .child(
            Button::new("custom-agent-env-add", "Add")
                .style(ButtonStyle::Outlined)
                .label_size(LabelSize::Small)
                .tab_index(0isize)
                .start_icon(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    let row = new_kv_row(None, None, window, cx);
                    // Focus the new key so the user can type immediately and tab
                    // through the new row (key -> value -> ... -> Add button).
                    let key_handle = row.key.focus_handle(cx);
                    if let Some(form) = this.custom_agent_form.as_mut() {
                        form.env.push(row);
                    }
                    key_handle.focus(window, cx);
                    cx.notify();
                })),
        )
        .into_any_element();

    crate::render_settings_item_layout(
        settings_window,
        "Environment Variables",
        "Environment variables provided to the agent process.",
        control,
        None,
        None,
        None,
        false,
        cx,
    )
    .into_any_element()
}

fn render_form_error(error: SharedString) -> impl IntoElement {
    h_flex()
        .w_full()
        .gap_2()
        .items_start()
        .child(
            Icon::new(IconName::XCircle)
                .size(IconSize::Small)
                .color(Color::Error),
        )
        .child(Label::new(error).size(LabelSize::Small).color(Color::Error))
}

fn render_form_actions(
    form: &CustomAgentForm,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let cancel_handle = form.cancel_focus_handle.clone().tab_index(0).tab_stop(true);
    let save_handle = form.save_focus_handle.clone().tab_index(0).tab_stop(true);
    let cancel_border = focus_ring_color(&cancel_handle, window, cx);
    let save_border = focus_ring_color(&save_handle, window, cx);

    h_flex()
        .w_full()
        .gap_2()
        .justify_end()
        .pt_2()
        .child(
            div()
                .rounded_md()
                .border_1()
                .border_color(cancel_border)
                .child(
                    Button::new("custom-agent-form-cancel", "Cancel")
                        .style(ButtonStyle::Subtle)
                        .track_focus(&cancel_handle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.custom_agent_form = None;
                            this.pop_sub_page(window, cx);
                        })),
                ),
        )
        .child(
            div()
                .rounded_md()
                .border_1()
                .border_color(save_border)
                .child(
                    Button::new("custom-agent-form-save", "Save")
                        .style(ButtonStyle::Filled)
                        .track_focus(&save_handle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            save_custom_agent_form(this, window, cx);
                        })),
                ),
        )
}

/// Returns the border color for a button's focus ring: visible when focused
/// (keyboard or programmatic), transparent otherwise.
fn focus_ring_color(handle: &FocusHandle, window: &Window, cx: &App) -> gpui::Hsla {
    if handle.is_focused(window) {
        cx.theme().colors().border_focused
    } else {
        gpui::transparent_black()
    }
}

fn save_custom_agent_form(
    settings_window: &mut SettingsWindow,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let built = {
        let Some(form) = settings_window.custom_agent_form.as_ref() else {
            return;
        };
        build_settings_from_form(form, cx)
    };

    let (id, original_id, content) = match built {
        Ok(value) => value,
        Err(error) => {
            if let Some(form) = settings_window.custom_agent_form.as_mut() {
                form.error = Some(error);
            }
            cx.notify();
            return;
        }
    };

    // Reject names that would collide with a *different* existing agent. This
    // covers both adding a new agent and renaming an existing one.
    let collides_with_other_agent =
        get_agent_server_store(settings_window, cx).is_some_and(|store| {
            let existing_ids = store
                .read(cx)
                .external_agents()
                .cloned()
                .collect::<Vec<_>>();
            name_collides_with_other_agent(&id, original_id.as_ref(), &existing_ids)
        });
    if collides_with_other_agent {
        if let Some(form) = settings_window.custom_agent_form.as_mut() {
            form.error = Some(format!("An agent named \"{}\" already exists.", id.0).into());
        }
        cx.notify();
        return;
    }

    let fs = <dyn fs::Fs>::global(cx);
    update_settings_file(fs, cx, move |settings, _| {
        let agent_servers = settings.agent_servers.get_or_insert_default();
        if let Some(original_id) = &original_id
            && original_id.0 != id.0
        {
            agent_servers.remove(original_id.0.as_ref());
        }
        agent_servers.insert(id.0.to_string(), content);
    });

    settings_window.custom_agent_form = None;
    settings_window.pop_sub_page(window, cx);
}

/// Plain (editor-free) snapshot of the form's contents, so the validation /
/// build logic can be exercised without a GPUI context.
struct CustomAgentFormValues {
    original_id: Option<AgentId>,
    name: String,
    command: String,
    args: String,
    env: Vec<(String, String)>,
    default_mode: Option<String>,
    default_config_options: HashMap<String, AgentConfigOptionValue>,
    favorite_config_option_values: HashMap<String, Vec<String>>,
}

fn build_settings_from_form(
    form: &CustomAgentForm,
    cx: &App,
) -> Result<(AgentId, Option<AgentId>, CustomAgentServerSettings), SharedString> {
    let values = CustomAgentFormValues {
        original_id: form.original_id.clone(),
        name: form.name.read(cx).text(cx),
        command: form.command.read(cx).text(cx),
        args: form.args.read(cx).text(cx),
        env: read_kv(&form.env, cx),
        default_mode: form.default_mode.clone(),
        default_config_options: form.default_config_options.clone(),
        favorite_config_option_values: form.favorite_config_option_values.clone(),
    };
    build_settings_from_values(values)
}

fn read_kv(rows: &[KeyValueRow], cx: &App) -> Vec<(String, String)> {
    rows.iter()
        .map(|row| (row.key.read(cx).text(cx), row.value.read(cx).text(cx)))
        .collect()
}

fn build_settings_from_values(
    values: CustomAgentFormValues,
) -> Result<(AgentId, Option<AgentId>, CustomAgentServerSettings), SharedString> {
    let name = values.name.trim().to_string();
    if name.is_empty() {
        return Err("Agent name is required.".into());
    }

    let command = values.command.trim().to_string();
    if command.is_empty() {
        return Err("Command is required.".into());
    }

    let args = values
        .args
        .split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>();
    let env = collect_kv(&values.env, "environment variable")?;

    let content = CustomAgentServerSettings::Custom {
        path: command.into(),
        args,
        env,
        default_mode: values.default_mode,
        default_config_options: values.default_config_options,
        favorite_config_option_values: values.favorite_config_option_values,
    };

    Ok((AgentId(name.into()), values.original_id, content))
}

/// Returns whether saving under `id` would overwrite a *different* existing
/// agent. Editing an agent in place (`id == original_id`) is allowed.
fn name_collides_with_other_agent(
    id: &AgentId,
    original_id: Option<&AgentId>,
    existing_ids: &[AgentId],
) -> bool {
    original_id.is_none_or(|original| original.0 != id.0)
        && existing_ids.iter().any(|existing| existing.0 == id.0)
}

fn collect_kv(
    rows: &[(String, String)],
    label: &str,
) -> Result<HashMap<String, String>, SharedString> {
    let mut map = HashMap::default();
    for (key, value) in rows {
        let key = key.trim().to_string();
        if key.is_empty() {
            continue;
        }
        if map.contains_key(&key) {
            return Err(format!("Duplicate {label} \"{key}\".").into());
        }
        map.insert(key, value.clone());
    }
    Ok(map)
}

// === Open settings.json at the agent's position ===
//
// Retained for an upcoming "Edit in settings.json" affordance that jumps the
// user to the relevant `agent_servers` entry. Not currently wired to any UI.

/// Opens the user's `settings.json` in the original (editor) window, inserts a
/// scaffold `agent_servers` entry, and selects its name so the user can fill in
/// the executable path.
#[allow(dead_code)]
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

#[allow(dead_code)]
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
                            CustomAgentServerSettings::Custom {
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn values() -> CustomAgentFormValues {
        CustomAgentFormValues {
            original_id: None,
            name: "my-agent".into(),
            command: "/usr/bin/agent".into(),
            args: String::new(),
            env: Vec::new(),
            default_mode: None,
            default_config_options: HashMap::default(),
            favorite_config_option_values: HashMap::default(),
        }
    }

    fn id(name: &str) -> AgentId {
        AgentId(name.into())
    }

    #[test]
    fn requires_agent_name() {
        let mut values = values();
        values.name = "   ".into();
        assert_eq!(
            build_settings_from_values(values).unwrap_err().as_ref(),
            "Agent name is required."
        );
    }

    #[test]
    fn requires_command() {
        let mut values = values();
        values.command = "   ".into();
        assert_eq!(
            build_settings_from_values(values).unwrap_err().as_ref(),
            "Command is required."
        );
    }

    #[test]
    fn rejects_duplicate_environment_variables() {
        let mut values = values();
        values.env = vec![("FOO".into(), "1".into()), ("FOO".into(), "2".into())];
        assert_eq!(
            build_settings_from_values(values).unwrap_err().as_ref(),
            "Duplicate environment variable \"FOO\"."
        );
    }

    #[test]
    fn builds_custom_agent() {
        let mut values = values();
        values.name = "  my-agent  ".into();
        values.command = "/usr/bin/agent".into();
        values.args = "--flag  value".into();
        // Empty values are kept, but rows with a blank key are ignored.
        values.env = vec![
            ("KEY".into(), "VALUE".into()),
            ("EMPTY".into(), String::new()),
            ("   ".into(), "ignored".into()),
        ];

        let (id, original_id, content) = build_settings_from_values(values).unwrap();
        assert_eq!(id.0.as_ref(), "my-agent");
        assert_eq!(original_id, None);

        let expected_env = HashMap::from_iter([
            ("KEY".to_string(), "VALUE".to_string()),
            ("EMPTY".to_string(), String::new()),
        ]);
        assert_eq!(
            content,
            CustomAgentServerSettings::Custom {
                path: "/usr/bin/agent".into(),
                args: vec!["--flag".into(), "value".into()],
                env: expected_env,
                default_mode: None,
                default_config_options: HashMap::default(),
                favorite_config_option_values: HashMap::default(),
            }
        );
    }

    #[test]
    fn preserves_advanced_fields() {
        let mut values = values();
        values.default_mode = Some("ask".into());
        values.default_config_options =
            HashMap::from_iter([("opt".to_string(), AgentConfigOptionValue::from("val"))]);

        let (_, _, content) = build_settings_from_values(values).unwrap();
        match content {
            CustomAgentServerSettings::Custom {
                default_mode,
                default_config_options,
                ..
            } => {
                assert_eq!(default_mode.as_deref(), Some("ask"));
                assert_eq!(
                    default_config_options
                        .get("opt")
                        .and_then(AgentConfigOptionValue::as_value_id),
                    Some("val"),
                );
            }
            _ => panic!("expected a custom agent"),
        }
    }

    #[test]
    fn name_collision_covers_new_and_rename() {
        let existing = vec![id("foo"), id("bar")];

        // New agent taking an existing name collides.
        assert!(name_collides_with_other_agent(&id("foo"), None, &existing));
        // New agent with a free name is fine.
        assert!(!name_collides_with_other_agent(&id("baz"), None, &existing));
        // Editing an agent in place is allowed even though the name "exists".
        assert!(!name_collides_with_other_agent(
            &id("foo"),
            Some(&id("foo")),
            &existing
        ));
        // Renaming onto a different agent's name collides.
        assert!(name_collides_with_other_agent(
            &id("bar"),
            Some(&id("foo")),
            &existing
        ));
    }
}
