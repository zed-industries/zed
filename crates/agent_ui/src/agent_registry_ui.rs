use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::sync::OnceLock;

use client::zed_urls;
use collections::HashMap;
use editor::{Editor, EditorElement, EditorStyle};
use fs::Fs;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, Focusable, KeyContext, ParentElement, Render,
    RenderOnce, SharedString, Styled, TextStyle, UniformListScrollHandle, Window, point,
    uniform_list,
};
use project::agent_server_store::{AllAgentServersSettings, CustomAgentServerSettings};
use project::{AgentRegistryStore, RegistryAgent};
use settings::{Settings, SettingsStore, update_settings_file};
use theme::ThemeSettings;
use ui::{
    Banner, ButtonStyle, ScrollableHandle, Severity, ToggleButtonGroup, ToggleButtonGroupSize,
    ToggleButtonGroupStyle, ToggleButtonSimple, Tooltip, WithScrollbar, prelude::*,
};
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
};

/// Registry IDs for built-in agents that Zed already provides first-class support for.
/// These are filtered out of the ACP Agent Registry UI to avoid showing duplicates.
const BUILT_IN_REGISTRY_IDS: [&str; 4] = ["claude-acp", "claude-code-acp", "codex-acp", "gemini"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegistryFilter {
    All,
    Installed,
    NotInstalled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegistryInstallStatus {
    NotInstalled,
    InstalledRegistry,
    InstalledCustom,
    InstalledExtension,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BuiltInAgent {
    Claude,
    Codex,
    Gemini,
}

fn keywords_by_agent_feature() -> &'static BTreeMap<BuiltInAgent, Vec<&'static str>> {
    static KEYWORDS_BY_FEATURE: OnceLock<BTreeMap<BuiltInAgent, Vec<&'static str>>> =
        OnceLock::new();
    KEYWORDS_BY_FEATURE.get_or_init(|| {
        BTreeMap::from_iter([
            (BuiltInAgent::Claude, vec!["claude", "claude code"]),
            (BuiltInAgent::Codex, vec!["codex", "codex cli"]),
            (BuiltInAgent::Gemini, vec!["gemini", "gemini cli"]),
        ])
    })
}

#[derive(IntoElement)]
struct AgentRegistryCard {
    children: Vec<AnyElement>,
}

impl AgentRegistryCard {
    fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for AgentRegistryCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for AgentRegistryCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().child(
            v_flex()
                .p_3()
                .mt_4()
                .w_full()
                .min_h(rems_from_px(86.))
                .gap_2()
                .bg(cx.theme().colors().elevated_surface_background.opacity(0.5))
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .rounded_md()
                .children(self.children),
        )
    }
}

pub struct AgentRegistryPage {
    registry_store: Entity<AgentRegistryStore>,
    list: UniformListScrollHandle,
    registry_agents: Vec<RegistryAgent>,
    filtered_registry_indices: Vec<usize>,
    installed_statuses: HashMap<String, RegistryInstallStatus>,
    query_editor: Entity<Editor>,
    filter: RegistryFilter,
    upsells: BTreeSet<BuiltInAgent>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl AgentRegistryPage {
    pub fn new(
        _workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let registry_store = AgentRegistryStore::global(cx);
            let query_editor = cx.new(|cx| {
                let mut input = Editor::single_line(window, cx);
                input.set_placeholder_text("Search agents...", window, cx);
                input
            });
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let mut subscriptions = Vec::new();
            subscriptions.push(cx.observe(&registry_store, |this, _, cx| {
                this.reload_registry_agents(cx);
            }));
            subscriptions.push(cx.observe_global::<SettingsStore>(|this, cx| {
                this.filter_registry_agents(cx);
            }));

            let mut this = Self {
                registry_store,
                list: UniformListScrollHandle::new(),
                registry_agents: Vec::new(),
                filtered_registry_indices: Vec::new(),
                installed_statuses: HashMap::default(),
                query_editor,
                filter: RegistryFilter::All,
                upsells: BTreeSet::new(),
                _subscriptions: subscriptions,
            };

            this.reload_registry_agents(cx);
            this.registry_store
                .update(cx, |store, cx| store.refresh(cx));

            this
        })
    }

    fn reload_registry_agents(&mut self, cx: &mut Context<Self>) {
        self.registry_agents = self.registry_store.read(cx).agents().to_vec();
        self.registry_agents.sort_by(|left, right| {
            left.name()
                .as_ref()
                .cmp(right.name().as_ref())
                .then_with(|| left.id().as_ref().cmp(right.id().as_ref()))
        });
        self.filter_registry_agents(cx);
    }

    fn refresh_installed_statuses(&mut self, cx: &mut Context<Self>) {
        let settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None);
        self.installed_statuses.clear();
        for (id, settings) in &settings.custom {
            let status = match settings {
                CustomAgentServerSettings::Registry { .. } => {
                    RegistryInstallStatus::InstalledRegistry
                }
                CustomAgentServerSettings::Custom { .. } => RegistryInstallStatus::InstalledCustom,
                CustomAgentServerSettings::Extension { .. } => {
                    RegistryInstallStatus::InstalledExtension
                }
            };
            self.installed_statuses.insert(id.clone(), status);
        }
    }

    fn install_status(&self, id: &str) -> RegistryInstallStatus {
        self.installed_statuses
            .get(id)
            .copied()
            .unwrap_or(RegistryInstallStatus::NotInstalled)
    }

    fn search_query(&self, cx: &mut App) -> Option<String> {
        let search = self.query_editor.read(cx).text(cx);
        if search.trim().is_empty() {
            None
        } else {
            Some(search)
        }
    }

    fn filter_registry_agents(&mut self, cx: &mut Context<Self>) {
        self.refresh_installed_statuses(cx);
        self.refresh_feature_upsells(cx);
        let search = self.search_query(cx).map(|search| search.to_lowercase());
        let filter = self.filter;
        let installed_statuses = self.installed_statuses.clone();

        let filtered_indices = self
            .registry_agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| {
                // Filter out built-in agents since they already appear in the main
                // agent configuration UI and don't need to be installed from the registry.
                if BUILT_IN_REGISTRY_IDS.contains(&agent.id().as_ref()) {
                    return false;
                }

                let matches_search = search.as_ref().is_none_or(|query| {
                    let query = query.as_str();
                    agent.id().as_ref().to_lowercase().contains(query)
                        || agent.name().as_ref().to_lowercase().contains(query)
                        || agent.description().as_ref().to_lowercase().contains(query)
                });

                let install_status = installed_statuses
                    .get(agent.id().as_ref())
                    .copied()
                    .unwrap_or(RegistryInstallStatus::NotInstalled);
                let matches_filter = match filter {
                    RegistryFilter::All => true,
                    RegistryFilter::Installed => {
                        install_status != RegistryInstallStatus::NotInstalled
                    }
                    RegistryFilter::NotInstalled => {
                        install_status == RegistryInstallStatus::NotInstalled
                    }
                };

                matches_search && matches_filter
            })
            .map(|(index, _)| index)
            .collect();

        self.filtered_registry_indices = filtered_indices;

        cx.notify();
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    fn on_query_change(
        &mut self,
        _: Entity<Editor>,
        event: &editor::EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if let editor::EditorEvent::Edited { .. } = event {
            self.filter_registry_agents(cx);
            self.scroll_to_top(cx);
        }
    }

    fn refresh_feature_upsells(&mut self, cx: &mut Context<Self>) {
        let Some(search) = self.search_query(cx) else {
            self.upsells.clear();
            return;
        };

        let search = search.to_lowercase();
        let search_terms = search
            .split_whitespace()
            .map(|term| term.trim())
            .collect::<Vec<_>>();

        for (feature, keywords) in keywords_by_agent_feature() {
            if keywords
                .iter()
                .any(|keyword| search_terms.contains(keyword))
            {
                self.upsells.insert(*feature);
            } else {
                self.upsells.remove(feature);
            }
        }
    }

    fn render_feature_upsell_banner(
        &self,
        label: SharedString,
        docs_url: SharedString,
    ) -> impl IntoElement {
        let docs_url_button = Button::new("open_docs", "View Documentation")
            .icon(IconName::ArrowUpRight)
            .icon_size(IconSize::Small)
            .icon_position(IconPosition::End)
            .icon_color(Color::Muted)
            .on_click({
                move |_event, _window, cx| {
                    telemetry::event!(
                        "Documentation Viewed",
                        source = "Agent Registry Feature Upsell",
                        url = docs_url,
                    );
                    cx.open_url(&docs_url)
                }
            });

        div().pt_4().px_4().child(
            Banner::new()
                .severity(Severity::Success)
                .child(Label::new(label).mt_0p5())
                .action_slot(docs_url_button),
        )
    }

    fn render_feature_upsells(&self) -> impl IntoElement {
        let mut container = v_flex();

        for feature in &self.upsells {
            let banner = match feature {
                BuiltInAgent::Claude => self.render_feature_upsell_banner(
                    "Claude Code support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#claude-code".into(),
                ),
                BuiltInAgent::Codex => self.render_feature_upsell_banner(
                    "Codex CLI support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#codex-cli".into(),
                ),
                BuiltInAgent::Gemini => self.render_feature_upsell_banner(
                    "Gemini CLI support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#gemini-cli".into(),
                ),
            };
            container = container.child(banner);
        }

        container
    }

    fn render_search(&self, cx: &mut Context<Self>) -> Div {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");

        h_flex()
            .key_context(key_context)
            .h_8()
            .min_w(rems_from_px(384.))
            .flex_1()
            .pl_1p5()
            .pr_2()
            .gap_2()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.render_text_input(&self.query_editor, cx))
    }

    fn render_text_input(
        &self,
        editor: &Entity<Editor>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_search = self.search_query(cx).is_some();
        let registry_store = self.registry_store.read(cx);

        let message = if registry_store.is_fetching() {
            "Loading registry..."
        } else if registry_store.fetch_error().is_some() {
            "Failed to load the agent registry. Please check your connection and try again."
        } else {
            match self.filter {
                RegistryFilter::All => {
                    if has_search {
                        "No agents match your search."
                    } else {
                        "No agents available."
                    }
                }
                RegistryFilter::Installed => {
                    if has_search {
                        "No installed agents match your search."
                    } else {
                        "No installed agents."
                    }
                }
                RegistryFilter::NotInstalled => {
                    if has_search {
                        "No uninstalled agents match your search."
                    } else {
                        "No uninstalled agents."
                    }
                }
            }
        };

        h_flex()
            .py_4()
            .gap_1p5()
            .when(registry_store.fetch_error().is_some(), |this| {
                this.child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
            })
            .child(Label::new(message))
    }

    fn render_agents(
        &mut self,
        range: Range<usize>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AgentRegistryCard> {
        range
            .map(|index| {
                let Some(agent_index) = self.filtered_registry_indices.get(index).copied() else {
                    return self.render_missing_agent();
                };
                let Some(agent) = self.registry_agents.get(agent_index) else {
                    return self.render_missing_agent();
                };
                self.render_registry_agent(agent, cx)
            })
            .collect()
    }

    fn render_missing_agent(&self) -> AgentRegistryCard {
        AgentRegistryCard::new().child(
            Label::new("Missing registry entry.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }

    fn render_registry_agent(
        &self,
        agent: &RegistryAgent,
        cx: &mut Context<Self>,
    ) -> AgentRegistryCard {
        let install_status = self.install_status(agent.id().as_ref());
        let supports_current_platform = agent.supports_current_platform();

        let icon = match agent.icon_path() {
            Some(icon_path) => Icon::from_external_svg(icon_path.clone()),
            None => Icon::new(IconName::Sparkle),
        }
        .size(IconSize::Medium)
        .color(Color::Muted);

        let install_button =
            self.install_button(agent, install_status, supports_current_platform, cx);

        let repository_button = agent.repository().map(|repository| {
            let repository_for_tooltip: SharedString = repository.to_string().into();
            let repository_for_click = repository.to_string();

            IconButton::new(
                SharedString::from(format!("agent-repo-{}", agent.id())),
                IconName::Github,
            )
            .icon_size(IconSize::Small)
            .tooltip(move |_, cx| {
                Tooltip::with_meta(
                    "Visit Agent Repository",
                    None,
                    repository_for_tooltip.clone(),
                    cx,
                )
            })
            .on_click(move |_, _, cx| {
                cx.open_url(&repository_for_click);
            })
        });

        AgentRegistryCard::new()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(icon)
                            .child(Headline::new(agent.name().clone()).size(HeadlineSize::Small))
                            .child(Label::new(format!("v{}", agent.version())).color(Color::Muted))
                            .when(!supports_current_platform, |this| {
                                this.child(
                                    Label::new("Not supported on this platform")
                                        .size(LabelSize::Small)
                                        .color(Color::Warning),
                                )
                            }),
                    )
                    .child(install_button),
            )
            .child(
                h_flex()
                    .gap_2()
                    .justify_between()
                    .child(
                        Label::new(agent.description().clone())
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Label::new(format!("ID: {}", agent.id()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                            .when_some(repository_button, |this, button| this.child(button)),
                    ),
            )
    }

    fn install_button(
        &self,
        agent: &RegistryAgent,
        install_status: RegistryInstallStatus,
        supports_current_platform: bool,
        cx: &mut Context<Self>,
    ) -> Button {
        let button_id = SharedString::from(format!("install-agent-{}", agent.id()));

        if !supports_current_platform {
            return Button::new(button_id, "Unavailable")
                .style(ButtonStyle::OutlinedGhost)
                .disabled(true);
        }

        match install_status {
            RegistryInstallStatus::NotInstalled => {
                let fs = <dyn Fs>::global(cx);
                let agent_id = agent.id().to_string();
                Button::new(button_id, "Install")
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .icon(IconName::Download)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .on_click(move |_, _, cx| {
                        let agent_id = agent_id.clone();
                        update_settings_file(fs.clone(), cx, move |settings, _| {
                            let agent_servers = settings.agent_servers.get_or_insert_default();
                            agent_servers.custom.entry(agent_id).or_insert_with(|| {
                                settings::CustomAgentServerSettings::Registry {
                                    default_mode: None,
                                    default_model: None,
                                    env: Default::default(),
                                    favorite_models: Vec::new(),
                                    default_config_options: HashMap::default(),
                                    favorite_config_option_values: HashMap::default(),
                                }
                            });
                        });
                    })
            }
            RegistryInstallStatus::InstalledRegistry => {
                let fs = <dyn Fs>::global(cx);
                let agent_id = agent.id().to_string();
                Button::new(button_id, "Remove")
                    .style(ButtonStyle::OutlinedGhost)
                    .on_click(move |_, _, cx| {
                        let agent_id = agent_id.clone();
                        update_settings_file(fs.clone(), cx, move |settings, _| {
                            let Some(agent_servers) = settings.agent_servers.as_mut() else {
                                return;
                            };
                            if let Some(entry) = agent_servers.custom.get(agent_id.as_str())
                                && matches!(
                                    entry,
                                    settings::CustomAgentServerSettings::Registry { .. }
                                )
                            {
                                agent_servers.custom.remove(agent_id.as_str());
                            }
                        });
                    })
            }
            RegistryInstallStatus::InstalledCustom => Button::new(button_id, "Installed")
                .style(ButtonStyle::OutlinedGhost)
                .disabled(true),
            RegistryInstallStatus::InstalledExtension => Button::new(button_id, "Installed")
                .style(ButtonStyle::OutlinedGhost)
                .disabled(true),
        }
    }
}

impl Render for AgentRegistryPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .p_4()
                    .gap_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_1p5()
                            .justify_between()
                            .child(Headline::new("ACP Registry").size(HeadlineSize::Large))
                            .child(
                                Button::new("learn-more", "Learn More")
                                    .style(ButtonStyle::Outlined)
                                    .size(ButtonSize::Medium)
                                    .icon(IconName::ArrowUpRight)
                                    .icon_color(Color::Muted)
                                    .icon_size(IconSize::Small)
                                    .on_click(move |_, _, cx| {
                                        cx.open_url(&zed_urls::acp_registry_blog(cx))
                                    }),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .flex_wrap()
                            .gap_2()
                            .child(self.render_search(cx))
                            .child(
                                div().child(
                                    ToggleButtonGroup::single_row(
                                        "registry-filter-buttons",
                                        [
                                            ToggleButtonSimple::new(
                                                "All",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = RegistryFilter::All;
                                                    this.filter_registry_agents(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                            ToggleButtonSimple::new(
                                                "Installed",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = RegistryFilter::Installed;
                                                    this.filter_registry_agents(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                            ToggleButtonSimple::new(
                                                "Not Installed",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = RegistryFilter::NotInstalled;
                                                    this.filter_registry_agents(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                        ],
                                    )
                                    .style(ToggleButtonGroupStyle::Outlined)
                                    .size(ToggleButtonGroupSize::Custom(rems_from_px(30.)))
                                    .label_size(LabelSize::Default)
                                    .auto_width()
                                    .selected_index(match self.filter {
                                        RegistryFilter::All => 0,
                                        RegistryFilter::Installed => 1,
                                        RegistryFilter::NotInstalled => 2,
                                    })
                                    .into_any_element(),
                                ),
                            ),
                    ),
            )
            .child(self.render_feature_upsells())
            .child(v_flex().px_4().size_full().overflow_y_hidden().map(|this| {
                let count = self.filtered_registry_indices.len();
                let has_upsells = !self.upsells.is_empty();
                if count == 0 && !has_upsells {
                    this.child(self.render_empty_state(cx)).into_any_element()
                } else if count == 0 {
                    this.into_any_element()
                } else {
                    let scroll_handle = &self.list;
                    this.child(
                        uniform_list("registry-entries", count, cx.processor(Self::render_agents))
                            .flex_grow()
                            .pb_4()
                            .track_scroll(scroll_handle),
                    )
                    .vertical_scrollbar_for(scroll_handle, window, cx)
                    .into_any_element()
                }
            }))
    }
}

impl EventEmitter<ItemEvent> for AgentRegistryPage {}

impl Focusable for AgentRegistryPage {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.query_editor.read(cx).focus_handle(cx)
    }
}

impl Item for AgentRegistryPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "ACP Registry".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("ACP Registry Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
