use std::ops::Range;

use client::zed_urls;
use collections::{HashMap, HashSet};
use editor::{Editor, EditorElement, EditorStyle};
use fs::Fs;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, Focusable, KeyContext, ParentElement, Render,
    RenderOnce, SharedString, Styled, TextStyle, UniformListScrollHandle, Window, point,
    uniform_list,
};
use project::agent_server_store::{
    AgentId, AgentServerStore, AllAgentServersSettings, CustomAgentServerSettings,
};
use project::{AgentRegistryStore, RegistryAgent};
use semver::Version;
use settings::{Settings, SettingsStore, update_settings_file};
use theme_settings::ThemeSettings;
use ui::{
    ButtonStyle, ScrollableHandle, ToggleButtonGroup, ToggleButtonGroupSize,
    ToggleButtonGroupStyle, ToggleButtonSimple, Tooltip, WithScrollbar, prelude::*,
};
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
};

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
}

#[derive(Clone, Debug)]
enum RegistryInstalledVersion {
    Checking,
    Installed(Version),
    Missing,
    Error(SharedString),
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
                .min_h(rems_from_px(86_f32))
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
    agent_server_store: Entity<AgentServerStore>,
    list: UniformListScrollHandle,
    registry_agents: Vec<RegistryAgent>,
    filtered_registry_indices: Vec<usize>,
    installed_statuses: HashMap<String, RegistryInstallStatus>,
    installed_versions: HashMap<String, RegistryInstalledVersion>,
    updating_agents: HashSet<String>,
    update_errors: HashMap<String, SharedString>,
    query_editor: Entity<Editor>,
    filter: RegistryFilter,
    _subscriptions: Vec<gpui::Subscription>,
}

impl AgentRegistryPage {
    pub fn new(
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let agent_server_store = workspace.project().read(cx).agent_server_store().clone();
        cx.new(move |cx| {
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
            subscriptions.push(cx.observe(&agent_server_store, |this, _, cx| {
                this.installed_versions.clear();
                this.refresh_installed_versions(cx);
            }));
            subscriptions.push(cx.observe_global::<SettingsStore>(|this, cx| {
                this.filter_registry_agents(cx);
                this.refresh_installed_versions(cx);
            }));

            let mut this = Self {
                registry_store,
                agent_server_store,
                list: UniformListScrollHandle::new(),
                registry_agents: Vec::new(),
                filtered_registry_indices: Vec::new(),
                installed_statuses: HashMap::default(),
                installed_versions: HashMap::default(),
                updating_agents: HashSet::default(),
                update_errors: HashMap::default(),
                query_editor,
                filter: RegistryFilter::All,
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
                .to_lowercase()
                .cmp(&right.name().as_ref().to_lowercase())
                .then_with(|| {
                    left.id()
                        .as_ref()
                        .to_lowercase()
                        .cmp(&right.id().as_ref().to_lowercase())
                })
        });
        self.filter_registry_agents(cx);
        self.refresh_installed_versions(cx);
    }

    fn refresh_installed_statuses(&mut self, cx: &mut Context<Self>) {
        let settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None);
        self.installed_statuses.clear();
        for (id, settings) in settings.iter() {
            let status = match settings {
                CustomAgentServerSettings::Registry { .. } => {
                    RegistryInstallStatus::InstalledRegistry
                }
                CustomAgentServerSettings::Custom { .. } => RegistryInstallStatus::InstalledCustom,
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

    fn refresh_installed_versions(&mut self, cx: &mut Context<Self>) {
        let installed_ids = self
            .installed_statuses
            .iter()
            .filter_map(|(id, status)| {
                matches!(status, RegistryInstallStatus::InstalledRegistry).then(|| id.clone())
            })
            .collect::<HashSet<_>>();

        self.installed_versions
            .retain(|id, _| installed_ids.contains(id));
        self.update_errors
            .retain(|id, _| installed_ids.contains(id));

        for agent_id in installed_ids {
            if self.installed_versions.contains_key(&agent_id) {
                continue;
            }

            self.installed_versions
                .insert(agent_id.clone(), RegistryInstalledVersion::Checking);

            let task = self.agent_server_store.update(cx, |store, cx| {
                store.registry_agent_installed_version(
                    &AgentId::new(agent_id.clone()),
                    &mut cx.to_async(),
                )
            });
            let Some(task) = task else {
                self.installed_versions.insert(
                    agent_id.clone(),
                    RegistryInstalledVersion::Error(
                        "The registry agent is not available in this project.".into(),
                    ),
                );
                continue;
            };

            let this = cx.entity().downgrade();
            cx.spawn(async move |_, cx| {
                let result = task.await;
                this.update(cx, |this, cx| {
                    let status = match result {
                        Ok(Some(version)) => RegistryInstalledVersion::Installed(version),
                        Ok(None) => RegistryInstalledVersion::Missing,
                        Err(error) => RegistryInstalledVersion::Error(format!("{error:#}").into()),
                    };
                    this.installed_versions.insert(agent_id, status);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    fn has_update(
        &self,
        agent: &RegistryAgent,
        installed_version: Option<&RegistryInstalledVersion>,
    ) -> bool {
        let Some(RegistryInstalledVersion::Installed(installed_version)) = installed_version else {
            return false;
        };
        Version::parse(agent.version())
            .is_ok_and(|available_version| available_version.cmp(installed_version).is_gt())
    }

    fn installed_version_label(
        &self,
        agent: &RegistryAgent,
        install_status: RegistryInstallStatus,
    ) -> Option<SharedString> {
        if install_status != RegistryInstallStatus::InstalledRegistry {
            return None;
        }

        match self.installed_versions.get(agent.id().as_ref()) {
            Some(RegistryInstalledVersion::Checking) => Some("Checking installed version…".into()),
            Some(RegistryInstalledVersion::Installed(version)) => {
                Some(format!("Installed v{version}").into())
            }
            Some(RegistryInstalledVersion::Missing) => {
                Some("Installed package not found; repair required".into())
            }
            Some(RegistryInstalledVersion::Error(error)) => {
                Some(format!("Could not read installed version: {error}").into())
            }
            None => None,
        }
    }

    fn update_agent(&mut self, agent_id: String, cx: &mut Context<Self>) {
        if !self.updating_agents.insert(agent_id.clone()) {
            return;
        }
        self.update_errors.remove(&agent_id);
        cx.notify();

        let task = self.agent_server_store.update(cx, |store, cx| {
            store.install_registry_agent(&AgentId::new(agent_id.clone()), &mut cx.to_async())
        });
        let Some(task) = task else {
            self.updating_agents.remove(&agent_id);
            self.update_errors.insert(
                agent_id,
                "The registry agent is not available in this project.".into(),
            );
            cx.notify();
            return;
        };

        let this = cx.entity().downgrade();
        cx.spawn(async move |_, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.updating_agents.remove(&agent_id);
                match result {
                    Ok(_) => {
                        this.installed_versions.remove(&agent_id);
                        this.update_errors.remove(&agent_id);
                        this.refresh_installed_versions(cx);
                    }
                    Err(error) => {
                        this.update_errors
                            .insert(agent_id.clone(), format!("{error:#}").into());
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
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
        let search = self.search_query(cx).map(|search| search.to_lowercase());
        let filter = self.filter;
        let installed_statuses = self.installed_statuses.clone();

        let filtered_indices = self
            .registry_agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| {
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

    fn render_search(&self, cx: &mut Context<Self>) -> Div {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");

        h_flex()
            .key_context(key_context)
            .h_8()
            .min_w(rems_from_px(384_f32))
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
        let is_fetching = registry_store.is_fetching();
        let fetch_error = registry_store.fetch_error();

        let message = if is_fetching {
            "Loading registry..."
        } else if fetch_error.is_some() {
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
            .min_w_0()
            .w_full()
            .gap_1p5()
            .items_start()
            .when(fetch_error.is_some(), |this| {
                this.child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
            })
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_1()
                    .child(Label::new(message))
                    .when_some(fetch_error.clone(), |this, fetch_error| {
                        this.child(
                            Label::new(fetch_error)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .when_some(fetch_error, |this, _| {
                let registry_store = self.registry_store.clone();
                this.child(
                    Button::new("retry-agent-registry", "Retry")
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Compact)
                        .on_click(move |_, _, cx| {
                            registry_store.update(cx, |store, cx| store.refresh(cx));
                        }),
                )
            })
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
        let installed_version = self.installed_versions.get(agent.id().as_ref());
        let supports_current_platform = agent.supports_current_platform();

        let icon = match agent.icon_path() {
            Some(icon_path) => Icon::from_external_svg(icon_path.clone()),
            None => Icon::new(IconName::Sparkle),
        }
        .size(IconSize::Medium)
        .color(Color::Muted);

        let install_button = self.install_button(
            agent,
            install_status,
            installed_version,
            supports_current_platform,
            cx,
        );
        let installed_version_label = self.installed_version_label(agent, install_status);
        let update_error = self.update_errors.get(agent.id().as_ref()).cloned();

        let repository_button = agent.repository().map(|repository| {
            let repository_for_tooltip = repository.clone();
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

        let website_button = agent.website().map(|website| {
            let website = website.clone();
            let website_for_click = website.clone();
            IconButton::new(
                SharedString::from(format!("agent-website-{}", agent.id())),
                IconName::Link,
            )
            .icon_size(IconSize::Small)
            .tooltip(move |_, cx| {
                Tooltip::with_meta("Visit Agent Website", None, website.clone(), cx)
            })
            .on_click(move |_, _, cx| {
                cx.open_url(&website_for_click);
            })
        });

        let license_button = agent.license_url().map(|license_url| {
            let license_url = license_url.clone();
            let license_url_for_click = license_url.clone();
            IconButton::new(
                SharedString::from(format!("agent-license-{}", agent.id())),
                IconName::FileTextOutlined,
            )
            .icon_size(IconSize::Small)
            .tooltip(move |_, cx| {
                Tooltip::with_meta(
                    "View Agent License or Terms of Service",
                    None,
                    license_url.clone(),
                    cx,
                )
            })
            .on_click(move |_, _, cx| {
                cx.open_url(&license_url_for_click);
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
                            .child(
                                Label::new(format!("Available v{}", agent.version()))
                                    .color(Color::Muted),
                            )
                            .when_some(installed_version_label, |this, label| {
                                this.child(Label::new(label).size(LabelSize::Small).color(
                                    if self.has_update(agent, installed_version) {
                                        Color::Warning
                                    } else {
                                        Color::Muted
                                    },
                                ))
                            })
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
                    .when_some(update_error, |this, error| {
                        this.child(
                            Label::new(format!("Update failed: {error}"))
                                .size(LabelSize::Small)
                                .color(Color::Error),
                        )
                    })
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Label::new(format!("ID: {}", agent.id()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                            .when_some(repository_button, |this, button| this.child(button))
                            .when_some(website_button, |this, button| this.child(button))
                            .when_some(license_button, |this, button| this.child(button)),
                    ),
            )
    }

    fn install_button(
        &self,
        agent: &RegistryAgent,
        install_status: RegistryInstallStatus,
        installed_version: Option<&RegistryInstalledVersion>,
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
                    .start_icon(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click(move |_, window, cx| {
                        update_settings_file(fs.clone(), cx, {
                            let agent_id = agent_id.clone();
                            move |settings, _| {
                                let agent_servers = settings.agent_servers.get_or_insert_default();
                                agent_servers.entry(agent_id).or_insert_with(|| {
                                    settings::CustomAgentServerSettings::Registry {
                                        default_mode: None,
                                        env: Default::default(),
                                        default_config_options: HashMap::default(),
                                        favorite_config_option_values: HashMap::default(),
                                    }
                                });
                            }
                        });
                        window.dispatch_action(
                            Box::new(zed_actions::agent::SelectAgent {
                                agent: agent_id.clone(),
                            }),
                            cx,
                        );
                    })
            }
            RegistryInstallStatus::InstalledRegistry => {
                if self.updating_agents.contains(agent.id().as_ref()) {
                    return Button::new(button_id, "Updating…")
                        .style(ButtonStyle::OutlinedGhost)
                        .disabled(true);
                }

                if self.has_update(agent, installed_version)
                    || matches!(
                        installed_version,
                        Some(
                            RegistryInstalledVersion::Missing | RegistryInstalledVersion::Error(_)
                        )
                    )
                {
                    let agent_id = agent.id().to_string();
                    let button_label = if matches!(
                        installed_version,
                        Some(
                            RegistryInstalledVersion::Missing | RegistryInstalledVersion::Error(_)
                        )
                    ) {
                        "Repair"
                    } else {
                        "Update"
                    };
                    return Button::new(button_id, button_label)
                        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.update_agent(agent_id.clone(), cx);
                        }));
                }

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
                            if let Some(entry) = agent_servers.get(agent_id.as_str())
                                && matches!(
                                    entry,
                                    settings::CustomAgentServerSettings::Registry { .. }
                                )
                            {
                                agent_servers.remove(agent_id.as_str());
                            }
                        });
                    })
            }
            RegistryInstallStatus::InstalledCustom => Button::new(button_id, "Installed")
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
                                    .end_icon(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
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
                                    .size(ToggleButtonGroupSize::Custom(rems_from_px(30_f32)))
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
            .child(v_flex().px_4().size_full().overflow_y_hidden().map(|this| {
                let count = self.filtered_registry_indices.len();
                if count == 0 {
                    this.child(self.render_empty_state(cx)).into_any_element()
                } else {
                    let scroll_handle = &self.list;
                    this.child(
                        uniform_list("registry-entries", count, cx.processor(Self::render_agents))
                            .flex_grow_1()
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

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
