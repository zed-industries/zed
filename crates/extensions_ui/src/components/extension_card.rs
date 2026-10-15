use std::{collections::BTreeSet, sync::Arc};

use cloud_api_types::{ExtensionApiManifest, ExtensionMetadata, ExtensionProvides};
use extension::{ExtensionManifest, SchemaVersion};
use extension_host::{ExtensionOperation, ExtensionStatus, ExtensionStore};
use gpui::{Anchor, ElementId, Entity, Point, SharedString, prelude::*};
use release_channel::ReleaseChannel;
use ui::{Chip, ContextMenu, PopoverMenu, Tooltip, prelude::*, utils::format_number_with_commas};

type ContextMenuBuilder = Box<
    dyn Fn(Arc<str>, SharedString, &mut Window, &mut App) -> Option<Entity<ContextMenu>> + 'static,
>;
type ExtensionCardActions = [Option<Button>; 3];

/// Local state of a published extension, derived from the `ExtensionStore`.
#[derive(Clone)]
enum PublishedExtensionState {
    /// A dev extension with the same id is installed locally and takes
    /// precedence, so the published version can't be installed or managed.
    /// This replaces the store status, which would describe the dev extension.
    OverriddenByDevExtension,
    /// Managed by the store like any other extension.
    Store(ExtensionStatus),
}

impl PublishedExtensionState {
    fn new(extension_id: &str, extension_store: &ExtensionStore) -> Self {
        if extension_store.is_dev_extension(extension_id) {
            Self::OverriddenByDevExtension
        } else {
            Self::Store(extension_store.extension_status(extension_id))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LocalExtensionKind {
    Dev,
    Installed,
}

struct ExtensionCardDetails {
    id: Arc<str>,
    name: SharedString,
    version: Arc<str>,
    description: Option<SharedString>,
    authors: SharedString,
    repository_url: Option<SharedString>,
    repository_icon: IconName,
    provided_features: Vec<&'static str>,
    source: ExtensionCardSource,
}

#[derive(Clone)]
enum ExtensionCardSource {
    Dev,
    Installed,
    Remote {
        state: PublishedExtensionState,
        download_count: u64,
    },
}

impl ExtensionCardSource {
    fn installed_version(&self, latest_version: &Arc<str>) -> Option<Arc<str>> {
        match self {
            Self::Remote {
                state: PublishedExtensionState::Store(ExtensionStatus::Installed(installed_version)),
                ..
            } if installed_version != latest_version => Some(installed_version.clone()),
            _ => None,
        }
    }

    fn download_count(&self) -> Option<u64> {
        match self {
            Self::Remote { download_count, .. } => Some(*download_count),
            Self::Dev | Self::Installed => None,
        }
    }

    fn is_dev(&self) -> bool {
        matches!(self, Self::Dev)
    }

    fn is_overridden(&self) -> bool {
        matches!(
            self,
            Self::Remote {
                state: PublishedExtensionState::OverriddenByDevExtension,
                ..
            }
        )
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct ExtensionCard {
    details: ExtensionCardDetails,
    actions: ExtensionCardActions,
    context_menu: Option<ContextMenuBuilder>,
}

impl ExtensionCard {
    pub fn for_dev(extension: Arc<ExtensionManifest>, extension_store: &ExtensionStore) -> Self {
        let status = extension_store.extension_status(&extension.id);
        Self::manifest(extension, status, LocalExtensionKind::Dev)
    }

    pub fn for_installed(
        extension: Arc<ExtensionManifest>,
        extension_store: &ExtensionStore,
    ) -> Self {
        let status = extension_store.extension_status(&extension.id);
        Self::manifest(extension, status, LocalExtensionKind::Installed)
    }

    pub fn for_remote(
        extension: &ExtensionMetadata,
        extension_store: &ExtensionStore,
        cx: &App,
    ) -> Self {
        let state = PublishedExtensionState::new(&extension.id, extension_store);
        Self::remote(extension, state, cx)
    }

    fn manifest(
        extension: Arc<ExtensionManifest>,
        status: ExtensionStatus,
        kind: LocalExtensionKind,
    ) -> Self {
        let actions = Self::actions_for_manifest_extension(&extension, &status, kind);
        let details = ExtensionCardDetails {
            id: extension.id.clone(),
            name: extension.name.clone().into(),
            version: extension.version.clone(),
            description: extension.description.clone().map(Into::into),
            authors: extension.authors.join(", ").into(),
            repository_url: extension.repository.clone().map(Into::into),
            repository_icon: IconName::Link,
            provided_features: provided_feature_labels(extension.provides()),
            source: match kind {
                LocalExtensionKind::Dev => ExtensionCardSource::Dev,
                LocalExtensionKind::Installed => ExtensionCardSource::Installed,
            },
        };

        Self {
            details,
            actions,
            context_menu: None,
        }
    }

    fn remote(extension: &ExtensionMetadata, state: PublishedExtensionState, cx: &App) -> Self {
        let actions = Self::actions_for_remote_extension(extension, &state, cx);
        let details = ExtensionCardDetails {
            id: extension.id.clone(),
            name: extension.manifest.name.clone().into(),
            version: extension.manifest.version.clone(),
            description: extension.manifest.description.clone().map(Into::into),
            authors: extension.manifest.authors.join(", ").into(),
            repository_url: Some(extension.manifest.repository.clone().into()),
            repository_icon: IconName::Link,
            provided_features: provided_feature_labels(extension.manifest.provides.iter().copied()),
            source: ExtensionCardSource::Remote {
                state,
                download_count: extension.download_count,
            },
        };

        Self {
            details,
            actions,
            context_menu: None,
        }
    }

    fn disables_actions(status: &ExtensionStatus) -> bool {
        match status {
            ExtensionStatus::Installing
            | ExtensionStatus::Upgrading
            | ExtensionStatus::Removing => true,
            ExtensionStatus::NotInstalled | ExtensionStatus::Installed(_) => false,
        }
    }

    fn button_id(extension_id: &Arc<str>, operation: ExtensionOperation) -> ElementId {
        (SharedString::from(extension_id.clone()), operation as usize).into()
    }

    fn uninstall_button(extension_id: &Arc<str>, is_dev: bool) -> Button {
        Button::new(
            Self::button_id(extension_id, ExtensionOperation::Remove),
            "Uninstall",
        )
        .on_click({
            let extension_id = extension_id.clone();
            move |_, _, cx| {
                if !is_dev {
                    telemetry::event!("Extension Uninstalled", extension_id);
                }
                ExtensionStore::global(cx).update(cx, |store, cx| {
                    store
                        .uninstall_extension(extension_id.clone(), cx)
                        .detach_and_log_err(cx);
                });
            }
        })
    }

    fn configure_button(
        extension_id: &Arc<str>,
        manifest: Option<Arc<ExtensionManifest>>,
    ) -> Button {
        Button::new(
            SharedString::from(format!("configure-{extension_id}")),
            "Configure",
        )
        .on_click({
            let extension_id = extension_id.clone();
            move |_, _, cx| {
                let manifest = manifest.clone().or_else(|| {
                    ExtensionStore::global(cx)
                        .read(cx)
                        .extension_manifest_for_id(&extension_id)
                        .cloned()
                });
                if let Some(manifest) = manifest
                    && let Some(events) = extension::ExtensionEvents::try_global(cx)
                {
                    events.update(cx, |this, cx| {
                        this.emit(extension::Event::ConfigureExtensionRequested(manifest), cx)
                    });
                }
            }
        })
    }

    fn actions_for_manifest_extension(
        extension: &Arc<ExtensionManifest>,
        status: &ExtensionStatus,
        kind: LocalExtensionKind,
    ) -> ExtensionCardActions {
        let is_dev = kind == LocalExtensionKind::Dev;
        let rebuild = is_dev.then(|| {
            Button::new(
                SharedString::from(format!("rebuild-{}", extension.id)),
                "Rebuild",
            )
            .color(Color::Accent)
            .disabled(Self::disables_actions(status))
            .on_click({
                let extension_id = extension.id.clone();
                move |_, _, cx| {
                    ExtensionStore::global(cx).update(cx, |store, cx| {
                        store.rebuild_dev_extension(extension_id.clone(), cx)
                    });
                }
            })
        });
        let uninstall = Self::uninstall_button(&extension.id, is_dev)
            .when_else(
                is_dev,
                |button| button.color(Color::Accent),
                |button| button.style(ButtonStyle::OutlinedGhost),
            )
            .disabled(Self::disables_actions(status));
        let configure = (!extension.context_servers.is_empty()).then(|| {
            Self::configure_button(&extension.id, Some(extension.clone()))
                .when_else(
                    is_dev,
                    |button| button.color(Color::Accent),
                    |button| button.style(ButtonStyle::OutlinedGhost),
                )
                .disabled(Self::disables_actions(status))
        });

        if is_dev {
            [rebuild, Some(uninstall), configure]
        } else {
            [None, configure, Some(uninstall)]
        }
    }

    fn install_button(extension_id: &Arc<str>) -> Button {
        Button::new(
            Self::button_id(extension_id, ExtensionOperation::Install),
            "Install",
        )
        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
        .start_icon(
            Icon::new(IconName::Download)
                .size(IconSize::Small)
                .color(Color::Muted),
        )
        .on_click({
            let extension_id = extension_id.clone();
            move |_, _, cx| {
                telemetry::event!("Extension Installed");
                ExtensionStore::global(cx).update(cx, |store, cx| {
                    store.install_latest_extension(extension_id.clone(), cx)
                });
            }
        })
    }

    fn actions_for_remote_extension(
        extension: &ExtensionMetadata,
        state: &PublishedExtensionState,
        cx: &App,
    ) -> ExtensionCardActions {
        let is_configurable = extension
            .manifest
            .provides
            .contains(&ExtensionProvides::ContextServers);

        let status = match state {
            PublishedExtensionState::OverriddenByDevExtension => {
                return [
                    None,
                    None,
                    Some(Self::install_button(&extension.id).disabled(true)),
                ];
            }
            PublishedExtensionState::Store(status) => status,
        };

        match status {
            ExtensionStatus::NotInstalled | ExtensionStatus::Installing => [
                None,
                None,
                Some(Self::install_button(&extension.id).disabled(Self::disables_actions(status))),
            ],
            ExtensionStatus::Upgrading | ExtensionStatus::Removing => {
                let uninstall = Self::uninstall_button(&extension.id, false)
                    .style(ButtonStyle::OutlinedGhost)
                    .disabled(Self::disables_actions(status));
                let upgrade = matches!(status, ExtensionStatus::Upgrading).then(|| {
                    Button::new(
                        Self::button_id(&extension.id, ExtensionOperation::Upgrade),
                        "Upgrade",
                    )
                    .disabled(Self::disables_actions(status))
                });
                let configure = is_configurable.then(|| {
                    Self::configure_button(&extension.id, None)
                        .disabled(Self::disables_actions(status))
                });

                [upgrade, configure, Some(uninstall)]
            }
            ExtensionStatus::Installed(installed_version) => {
                let uninstall =
                    Self::uninstall_button(&extension.id, false).style(ButtonStyle::OutlinedGhost);
                let upgrade = (installed_version != &extension.manifest.version).then(|| {
                    let is_compatible = extension_host::is_version_compatible(
                        ReleaseChannel::global(cx),
                        extension,
                    );
                    Button::new(
                        Self::button_id(&extension.id, ExtensionOperation::Upgrade),
                        "Upgrade",
                    )
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .when(!is_compatible, |button| {
                        button.tooltip({
                            let version = extension.manifest.version.clone();
                            move |_, cx| {
                                Tooltip::simple(
                                    format!(
                                        "v{version} is not compatible with this version of Zed."
                                    ),
                                    cx,
                                )
                            }
                        })
                    })
                    .disabled(!is_compatible)
                    .on_click({
                        let extension_id = extension.id.clone();
                        let version = extension.manifest.version.clone();
                        move |_, _, cx| {
                            telemetry::event!("Extension Installed", extension_id, version);
                            ExtensionStore::global(cx).update(cx, |store, cx| {
                                store
                                    .upgrade_extension(extension_id.clone(), version.clone(), cx)
                                    .detach_and_log_err(cx)
                            });
                        }
                    })
                });
                let configure = is_configurable.then(|| {
                    Self::configure_button(&extension.id, None).style(ButtonStyle::OutlinedGhost)
                });

                [upgrade, configure, Some(uninstall)]
            }
        }
    }

    pub fn repository_icon(mut self, icon: IconName) -> Self {
        self.details.repository_icon = icon;
        self
    }

    pub fn context_menu(
        mut self,
        builder: impl Fn(Arc<str>, SharedString, &mut Window, &mut App) -> Option<Entity<ContextMenu>>
        + 'static,
    ) -> Self {
        self.context_menu = Some(Box::new(builder));
        self
    }
}

fn provided_feature_labels(
    provides: impl IntoIterator<Item = ExtensionProvides>,
) -> Vec<&'static str> {
    provides
        .into_iter()
        .filter(|provides| !provides.is_deprecated())
        .map(extension_provides_label)
        .collect()
}

pub(crate) fn extension_provides_label(provides: ExtensionProvides) -> &'static str {
    match provides {
        ExtensionProvides::Themes => "Themes",
        ExtensionProvides::IconThemes => "Icon Themes",
        ExtensionProvides::Languages => "Languages",
        ExtensionProvides::Grammars => "Grammars",
        ExtensionProvides::LanguageServers => "Language Servers",
        ExtensionProvides::ContextServers => "MCP Servers",
        ExtensionProvides::AgentServers => "Agent Servers",
        ExtensionProvides::SlashCommands => "Slash Commands",
        ExtensionProvides::IndexedDocsProviders => "Indexed Docs Providers",
        ExtensionProvides::Snippets => "Snippets",
        ExtensionProvides::DebugAdapters => "Debug Adapters",
    }
}

impl Component for ExtensionCard {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> &'static str {
        "A card that displays an extension's details, installation state, and available actions."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        fn remote_extension(
            id: &'static str,
            name: &'static str,
            version: &'static str,
            description: &'static str,
            download_count: u64,
            provides: impl IntoIterator<Item = ExtensionProvides>,
        ) -> ExtensionMetadata {
            ExtensionMetadata {
                id: id.into(),
                manifest: ExtensionApiManifest {
                    name: name.to_owned(),
                    version: version.into(),
                    description: Some(description.to_owned()),
                    authors: vec!["Zed Industries".to_owned()],
                    repository: "https://github.com/zed-industries/zed".to_owned(),
                    schema_version: Some(1),
                    wasm_api_version: None,
                    provides: BTreeSet::from_iter(provides),
                },
                published_at: Default::default(),
                download_count,
            }
        }

        fn dev_extension() -> Arc<ExtensionManifest> {
            Arc::new(ExtensionManifest {
                // Fake ID which will never be accepted during publishing
                id: "zed-demo-theme".into(),
                name: "Local Theme".to_owned(),
                version: "0.1.0".into(),
                schema_version: SchemaVersion::ZERO,
                description: Some("A locally installed extension under development.".to_owned()),
                repository: Some("https://github.com/zed-industries/zed".to_owned()),
                authors: vec!["Extension Developer".to_owned()],
                lib: Default::default(),
                themes: Vec::new(),
                icon_themes: Vec::new(),
                languages: Vec::new(),
                grammars: Default::default(),
                language_servers: Default::default(),
                context_servers: Default::default(),
                slash_commands: Default::default(),
                snippets: None,
                capabilities: Vec::new(),
                debug_adapters: Default::default(),
                debug_locators: Default::default(),
                language_model_providers: Default::default(),
            })
        }

        let examples = vec![
            single_example(
                "Available to Install",
                ExtensionCard::remote(
                    &remote_extension(
                        "preview-toml",
                        "TOML",
                        "0.6.2",
                        "TOML language support.",
                        482_391,
                        [ExtensionProvides::Languages],
                    ),
                    PublishedExtensionState::Store(ExtensionStatus::NotInstalled),
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Installed",
                ExtensionCard::remote(
                    &remote_extension(
                        "preview-python",
                        "Python",
                        "0.5.1",
                        "Python language support powered by basedpyright.",
                        1_284_613,
                        [
                            ExtensionProvides::Languages,
                            ExtensionProvides::LanguageServers,
                            ExtensionProvides::ContextServers,
                        ],
                    ),
                    PublishedExtensionState::Store(ExtensionStatus::Installed("0.5.1".into())),
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Update Available",
                ExtensionCard::remote(
                    &remote_extension(
                        "preview-rust",
                        "Rust",
                        "0.4.0",
                        "Rust language support powered by rust-analyzer.",
                        2_947_028,
                        [
                            ExtensionProvides::Languages,
                            ExtensionProvides::LanguageServers,
                        ],
                    ),
                    PublishedExtensionState::Store(ExtensionStatus::Installed("0.3.1".into())),
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Development Extension",
                ExtensionCard::manifest(
                    dev_extension(),
                    ExtensionStatus::Installed("0.1.0".into()),
                    LocalExtensionKind::Dev,
                )
                .into_any_element(),
            ),
            single_example(
                "Overridden by Development Extension",
                ExtensionCard::remote(
                    &remote_extension(
                        "preview-overridden-theme",
                        "Local Theme",
                        "1.3.0",
                        "The published version of a locally developed extension.",
                        36_512,
                        [ExtensionProvides::Themes],
                    ),
                    PublishedExtensionState::OverriddenByDevExtension,
                    cx,
                )
                .into_any_element(),
            ),
        ];

        div()
            .w_128()
            .child(example_group(examples).vertical())
            .into_any_element()
    }
}

impl RenderOnce for ExtensionCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            details,
            actions,
            context_menu,
        } = self;
        let ExtensionCardDetails {
            id,
            name,
            version,
            description,
            authors,
            repository_url,
            repository_icon,
            provided_features,
            source,
        } = details;
        let installed_version = source.installed_version(&version);
        let download_count = source.download_count();
        let is_dev = source.is_dev();
        let is_overridden = source.is_overridden();
        let repository_button_id = SharedString::from(format!("repository-{id}"));
        let context_menu_button_id = SharedString::from(format!("more-{id}"));
        let context_menu_extension_id = id;
        let context_menu_authors = authors.clone();

        div().w_full().child(
            v_flex()
                .mt_4()
                .w_full()
                .h(rems_from_px(110_f32))
                .p_3()
                .gap_2()
                .bg(cx.theme().colors().elevated_surface_background.opacity(0.5))
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .rounded_md()
                .child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(
                            h_flex()
                                .flex_shrink_1()
                                .min_w_0()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .gap_2()
                                .child(Headline::new(name).size(HeadlineSize::Small))
                                .child(
                                    Headline::new(if is_dev {
                                        format!("v{version} (dev)")
                                    } else {
                                        format!("v{version}")
                                    })
                                    .size(HeadlineSize::XSmall)
                                    .color(Color::Muted),
                                )
                                .children(installed_version.map(|installed_version| {
                                    Headline::new(format!("(v{installed_version} installed)"))
                                        .size(HeadlineSize::XSmall)
                                }))
                                .when(!provided_features.is_empty(), |parent| {
                                    parent.child(
                                        h_flex()
                                            .gap_1()
                                            .children(provided_features.into_iter().map(Chip::new)),
                                    )
                                }),
                        )
                        .child(
                            h_flex()
                                .flex_shrink_0()
                                .gap_1()
                                .children(actions.into_iter().flatten()),
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .children(description.map(|description| {
                            Label::new(description)
                                .size(LabelSize::Small)
                                .color(Color::Default)
                                .truncate()
                        }))
                        .children(download_count.map(|download_count| {
                            Label::new(format!(
                                "Downloads: {}",
                                format_number_with_commas(u128::from(download_count))
                            ))
                            .size(LabelSize::Small)
                        })),
                )
                .child(
                    h_flex()
                        .min_w_0()
                        .w_full()
                        .justify_between()
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_1()
                                .child(
                                    Icon::new(IconName::Person)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Label::new(authors)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate(),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .flex_shrink_0()
                                .when_some(repository_url, |this, repository_url| {
                                    let repository_url_for_tooltip = repository_url.clone();
                                    this.child(
                                        IconButton::new(repository_button_id, repository_icon)
                                            .icon_size(IconSize::Small)
                                            .tooltip(move |_, cx| {
                                                Tooltip::with_meta(
                                                    "Visit Extension Repository",
                                                    None,
                                                    repository_url_for_tooltip.clone(),
                                                    cx,
                                                )
                                            })
                                            .on_click(move |_, _, cx| {
                                                cx.open_url(&repository_url);
                                            }),
                                    )
                                })
                                .when_some(context_menu, |this, context_menu| {
                                    this.child(
                                        PopoverMenu::new(context_menu_button_id.clone())
                                            .trigger(
                                                IconButton::new(
                                                    context_menu_button_id,
                                                    IconName::Ellipsis,
                                                )
                                                .icon_size(IconSize::Small),
                                            )
                                            .anchor(Anchor::TopRight)
                                            .offset(Point {
                                                x: px(0.0),
                                                y: px(2.0),
                                            })
                                            .menu(move |window, cx| {
                                                context_menu(
                                                    context_menu_extension_id.clone(),
                                                    context_menu_authors.clone(),
                                                    window,
                                                    cx,
                                                )
                                            }),
                                    )
                                }),
                        ),
                )
                .when(is_overridden, |card| {
                    card.child(
                        h_flex()
                            .absolute()
                            .top_0()
                            .left_0()
                            .block_mouse_except_scroll()
                            .cursor_default()
                            .size_full()
                            .justify_center()
                            .bg(cx.theme().colors().elevated_surface_background.alpha(0.8))
                            .child(Label::new("Overridden by dev extension.")),
                    )
                }),
        )
    }
}
