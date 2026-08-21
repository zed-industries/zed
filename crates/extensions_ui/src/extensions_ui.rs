mod components;
mod extension_suggest;
mod extension_version_selector;

use std::sync::OnceLock;
use std::time::Duration;
use std::{any::TypeId, ops::Range, sync::Arc};

use anyhow::Context as _;
use cloud_api_types::{ExtensionMetadata, ExtensionProvides};
use collections::{BTreeMap, BTreeSet};
use command_palette_hooks::CommandPaletteFilter;
use editor::{Editor, EditorElement, EditorStyle};
use extension_host::{ExtensionIndexEntry, ExtensionManifest, ExtensionStore};
use futures::future::OptionFuture;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use git::{GitHostingProviderRegistry, parse_git_remote_url};
use gpui::{
    Action, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, Focusable,
    InteractiveElement, KeyContext, ParentElement, Render, Styled, Task, TaskExt, TextStyle,
    UniformListScrollHandle, WeakEntity, Window, actions, point, uniform_list,
};
use itertools::Itertools;
use picker::{Picker, PickerDelegate};
use project::DirectoryLister;

use schemars::JsonSchema;
use serde::Deserialize;
use settings::{Settings, SettingsContent};
use strum::IntoEnumIterator as _;
use theme_settings::ThemeSettings;
use ui::{
    Banner, ContextMenu, Divider, ListItem, ListItemSpacing, ScrollableHandle, Switch,
    ToggleButtonGroup, ToggleButtonGroupSize, ToggleButtonGroupStyle, ToggleButtonSimple,
    WithScrollbar, prelude::*,
};
use util::ResultExt;
use vim_mode_setting::VimModeSetting;
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
    workspace_error::{ErrorAction, ErrorSeverity, WorkspaceError},
};
use zed_actions::ExtensionCategoryFilter;

use crate::components::{ExtensionCard, extension_provides_label};
use crate::extension_version_selector::{
    ExtensionVersionSelector, ExtensionVersionSelectorDelegate,
};

actions!(
    zed,
    [
        /// Installs an extension from a local directory for development.
        InstallDevExtension,
    ]
);

/// Rebuilds an installed dev extension.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, gpui::Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct RebuildDevExtension {
    /// The ID of the dev extension to rebuild.
    ///
    /// Default: opens a picker if multiple dev extensions are installed.
    #[serde(default)]
    pub extension_id: Option<String>,
}

#[derive(Default)]
struct DevExtensionNotInstalledError {
    extension_id: Option<SharedString>,
}

impl WorkspaceError for DevExtensionNotInstalledError {
    fn primary_message(&self) -> SharedString {
        match &self.extension_id {
            Some(extension_id) => {
                format!("Dev extension '{extension_id}' is not installed.").into()
            }
            None => "No dev extensions are installed.".into(),
        }
    }

    fn primary_action(&self) -> ErrorAction {
        ErrorAction::new("Install Dev Extension", InstallDevExtension)
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Warning
    }
}

fn update_rebuild_dev_extension_visibility(store: &Entity<ExtensionStore>, cx: &mut App) {
    let has_dev_extensions = store.read(cx).dev_extensions().next().is_some();
    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        if has_dev_extensions {
            filter.show_action_types(&[TypeId::of::<RebuildDevExtension>()]);
        } else {
            filter.hide_action_types(&[TypeId::of::<RebuildDevExtension>()]);
        }
    });
}

pub fn init(cx: &mut App) {
    let store = ExtensionStore::global(cx);
    update_rebuild_dev_extension_visibility(&store, cx);
    cx.observe(&store, |store, cx| {
        update_rebuild_dev_extension_visibility(&store, cx);
    })
    .detach();

    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        workspace
            .register_action(
                move |workspace, action: &zed_actions::Extensions, window, cx| {
                    let provides_filter = action.category_filter.map(|category| match category {
                        ExtensionCategoryFilter::Themes => ExtensionProvides::Themes,
                        ExtensionCategoryFilter::IconThemes => ExtensionProvides::IconThemes,
                        ExtensionCategoryFilter::Languages => ExtensionProvides::Languages,
                        ExtensionCategoryFilter::Grammars => ExtensionProvides::Grammars,
                        ExtensionCategoryFilter::LanguageServers => {
                            ExtensionProvides::LanguageServers
                        }
                        ExtensionCategoryFilter::ContextServers => {
                            ExtensionProvides::ContextServers
                        }
                        ExtensionCategoryFilter::Snippets => ExtensionProvides::Snippets,
                        ExtensionCategoryFilter::DebugAdapters => ExtensionProvides::DebugAdapters,
                    });

                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<ExtensionsPage>());

                    if let Some(existing) = existing {
                        existing.update(cx, |extensions_page, cx| {
                            if provides_filter.is_some() {
                                extensions_page.change_provides_filter(provides_filter, cx);
                            }
                            if let Some(id) = action.id.as_ref() {
                                extensions_page.focus_extension(id, window, cx);
                            }
                        });

                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let extensions_page = ExtensionsPage::new(
                            workspace,
                            provides_filter,
                            action.id.as_deref(),
                            window,
                            cx,
                        );
                        workspace.add_item_to_active_pane(
                            Box::new(extensions_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                },
            )
            .register_action(move |workspace, _: &InstallDevExtension, window, cx| {
                let store = ExtensionStore::global(cx);
                let prompt = workspace.prompt_for_open_path(
                    gpui::PathPromptOptions {
                        files: false,
                        directories: true,
                        multiple: false,
                        prompt: None,
                    },
                    DirectoryLister::Local(
                        workspace.project().clone(),
                        workspace.app_state().fs.clone(),
                    ),
                    window,
                    cx,
                );

                let workspace_handle = cx.entity().downgrade();
                window
                    .spawn(cx, async move |cx| {
                        let extension_path = match prompt.await.map_err(anyhow::Error::from) {
                            Ok(Some(mut paths)) => paths.pop()?,
                            Ok(None) => return None,
                            Err(err) => {
                                workspace_handle
                                    .update(cx, |workspace, cx| {
                                        workspace.show_error(
                                            workspace::workspace_error::PortalError::new(
                                                err.to_string(),
                                            ),
                                            cx,
                                        );
                                    })
                                    .ok();
                                return None;
                            }
                        };

                        let install_task = store.update(cx, |store, cx| {
                            store.install_dev_extension(extension_path, cx)
                        });

                        match install_task.await {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Failed to install dev extension: {:?}", err);
                                workspace_handle
                                    .update(cx, |workspace, cx| {
                                        // NOTE: using `anyhow::context` here ends up not printing
                                        // the error
                                        workspace.show_error(
                                            format!("Failed to install dev extension: {}", err),
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }

                        Some(())
                    })
                    .detach();
            })
            .register_action(move |workspace, action: &RebuildDevExtension, window, cx| {
                if let Some(target_id) = action.extension_id.as_deref() {
                    let extension_id = ExtensionStore::global(cx)
                        .read(cx)
                        .dev_extensions()
                        .find_map(|m| {
                            if m.id.as_ref() == target_id {
                                Some(m.id.clone())
                            } else {
                                None
                            }
                        });
                    if let Some(extension_id) = extension_id {
                        ExtensionStore::global(cx).update(cx, |store, cx| {
                            store.rebuild_dev_extension(extension_id, cx);
                        });
                    } else {
                        workspace.show_error(
                            DevExtensionNotInstalledError {
                                extension_id: Some(SharedString::from(target_id.to_owned())),
                            },
                            cx,
                        );
                    }
                    return;
                }

                let dev_extensions = ExtensionStore::global(cx)
                    .read(cx)
                    .dev_extensions()
                    .cloned()
                    .collect::<Vec<_>>();

                match dev_extensions.len() {
                    0 => {
                        workspace.show_error(DevExtensionNotInstalledError::default(), cx);
                    }
                    1 => {
                        let extension_id = dev_extensions[0].id.clone();
                        ExtensionStore::global(cx).update(cx, |store, cx| {
                            store.rebuild_dev_extension(extension_id, cx);
                        });
                    }
                    _ => {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            let delegate = DevExtensionRebuildPickerDelegate::new(dev_extensions);
                            Picker::uniform_list(delegate, window, cx)
                        });
                    }
                }
            });

        cx.subscribe_in(workspace.project(), window, |_, _, event, window, cx| {
            if let project::Event::LanguageNotFound(buffer) = event {
                extension_suggest::suggest(buffer.clone(), window, cx);
            }
        })
        .detach();
    })
    .detach();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ExtensionFilter {
    All,
    Installed,
    NotInstalled,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Feature {
    AgentClaude,
    AgentCodex,
    AgentGemini,
    ExtensionBasedpyright,
    ExtensionRuff,
    ExtensionTailwind,
    ExtensionTy,
    Git,
    LanguageBash,
    LanguageC,
    LanguageCpp,
    LanguageGo,
    LanguagePython,
    LanguageReact,
    LanguageRust,
    LanguageTypescript,
    OpenIn,
    Vim,
}

fn keywords_by_feature() -> &'static BTreeMap<Feature, Vec<&'static str>> {
    static KEYWORDS_BY_FEATURE: OnceLock<BTreeMap<Feature, Vec<&'static str>>> = OnceLock::new();
    KEYWORDS_BY_FEATURE.get_or_init(|| {
        BTreeMap::from_iter([
            (
                Feature::AgentClaude,
                vec!["claude", "claude code", "claude agent"],
            ),
            (Feature::AgentCodex, vec!["codex", "codex cli"]),
            (Feature::AgentGemini, vec!["gemini", "gemini cli"]),
            (
                Feature::ExtensionBasedpyright,
                vec!["basedpyright", "pyright"],
            ),
            (Feature::ExtensionRuff, vec!["ruff"]),
            (Feature::ExtensionTailwind, vec!["tail", "tailwind"]),
            (Feature::ExtensionTy, vec!["ty"]),
            (Feature::Git, vec!["git"]),
            (Feature::LanguageBash, vec!["sh", "bash"]),
            (Feature::LanguageC, vec!["c", "clang"]),
            (Feature::LanguageCpp, vec!["c++", "cpp", "clang"]),
            (Feature::LanguageGo, vec!["go", "golang"]),
            (Feature::LanguagePython, vec!["python", "py"]),
            (Feature::LanguageReact, vec!["react"]),
            (Feature::LanguageRust, vec!["rust", "rs"]),
            (
                Feature::LanguageTypescript,
                vec!["type", "typescript", "ts"],
            ),
            (
                Feature::OpenIn,
                vec![
                    "github",
                    "gitlab",
                    "bitbucket",
                    "codeberg",
                    "sourcehut",
                    "permalink",
                    "link",
                    "open in",
                ],
            ),
            (Feature::Vim, vec!["vim"]),
        ])
    })
}

/// Everything we know about a single extension, consolidated from the local
/// extension store and remote responses.
#[derive(Clone, Default)]
struct ExtensionEntry {
    manifest: Option<Arc<ExtensionManifest>>,
    dev: bool,
    /// The most recent remote metadata we have seen for this extension, kept
    /// across searches. Note that `None` does not mean the extension is
    /// unpublished: the `/extensions` endpoint returns a bounded, filtered set,
    /// so we cannot currently surface a "no longer published" state.
    metadata: Option<Arc<ExtensionMetadata>>,
}

impl ExtensionEntry {
    fn is_installed(&self) -> bool {
        self.manifest.is_some()
    }

    fn provides(&self, provides: ExtensionProvides) -> bool {
        if let Some(manifest) = &self.manifest {
            manifest.provides().contains(&provides)
        } else if let Some(metadata) = &self.metadata {
            metadata.manifest.provides.contains(&provides)
        } else {
            false
        }
    }
}

/// A row in the extensions list, referencing an [`ExtensionEntry`] together with
/// the view to render it as. A dev extension and its published counterpart can
/// both be shown, as separate rows over the same entry.
#[derive(Clone)]
enum DisplayedExtension {
    Local(Arc<str>),
    Remote(Arc<str>),
}

/// The consolidated extension state backing the list: everything we know about
/// each extension, the current local and remote search results, and the rows
/// composed from them.
#[derive(Default)]
struct ExtensionList {
    entries: BTreeMap<Arc<str>, ExtensionEntry>,
    /// Ids of the installed extensions matching the current search, sorted by name.
    local_search_results: Vec<Arc<str>>,
    /// Ids of the current remote search results, in server order.
    remote_search_results: Vec<Arc<str>>,
    displayed: Vec<DisplayedExtension>,
}

impl ExtensionList {
    fn get(&self, extension_id: &str) -> Option<&ExtensionEntry> {
        self.entries.get(extension_id)
    }

    /// Syncs the installed state into the entries and recomputes the local search
    /// results. `matching_extension_ids` of `None` means there is no search query.
    fn update_installed_extensions(
        &mut self,
        installed_extensions: &BTreeMap<Arc<str>, ExtensionIndexEntry>,
        matching_extension_ids: Option<&BTreeSet<Arc<str>>>,
    ) {
        for entry in self.entries.values_mut() {
            entry.manifest = None;
            entry.dev = false;
        }
        for (extension_id, installed_extension) in installed_extensions {
            let entry = self.entries.entry(extension_id.clone()).or_default();
            entry.manifest = Some(installed_extension.manifest.clone());
            entry.dev = installed_extension.dev;
        }

        self.local_search_results = installed_extensions
            .keys()
            .filter(|extension_id| {
                matching_extension_ids
                    .is_none_or(|matching_ids| matching_ids.contains(*extension_id))
            })
            .cloned()
            .sorted_by_cached_key(|extension_id| {
                installed_extensions
                    .get(extension_id)
                    .map(|extension| extension.manifest.name.to_ascii_lowercase())
            })
            .collect();
    }

    fn set_remote_search_results(&mut self, remote_extensions: Vec<ExtensionMetadata>) {
        self.remote_search_results = remote_extensions
            .into_iter()
            .map(|metadata| {
                let metadata = Arc::new(metadata);
                let extension_id = metadata.id.clone();
                self.entries
                    .entry(extension_id.clone())
                    .or_default()
                    .metadata = Some(metadata);
                extension_id
            })
            .collect();
    }

    fn rebuild_displayed(
        &mut self,
        filter: ExtensionFilter,
        provides_filter: Option<ExtensionProvides>,
    ) {
        // The server already applies the provides filter to remote results, but
        // they can be stale for one fetch cycle after the filter changed, so apply
        // it locally to all rows to avoid briefly showing another category.
        let matches_provides = |extension_id: &Arc<str>| {
            provides_filter.is_none_or(|provides| {
                self.entries
                    .get(extension_id)
                    .is_some_and(|entry| entry.provides(provides))
            })
        };

        let local_ids = self
            .local_search_results
            .iter()
            .filter(|extension_id| matches_provides(extension_id));
        let remote_ids = self
            .remote_search_results
            .iter()
            .filter(|extension_id| matches_provides(extension_id));

        self.displayed = match filter {
            ExtensionFilter::All => local_ids
                .filter(|extension_id| {
                    self.entries
                        .get(*extension_id)
                        .is_some_and(|entry| entry.dev)
                })
                .cloned()
                .map(DisplayedExtension::Local)
                .chain(remote_ids.cloned().map(DisplayedExtension::Remote))
                .collect(),
            ExtensionFilter::Installed => {
                local_ids.cloned().map(DisplayedExtension::Local).collect()
            }
            ExtensionFilter::NotInstalled => remote_ids
                .filter(|extension_id| {
                    self.entries
                        .get(*extension_id)
                        .is_none_or(|entry| !entry.is_installed())
                })
                .cloned()
                .map(DisplayedExtension::Remote)
                .collect(),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExtensionFetchState {
    Fetching,
    Succeeded,
    Failed,
}

impl ExtensionFetchState {
    fn is_fetching(self) -> bool {
        self == Self::Fetching
    }

    fn failed(self) -> bool {
        self == Self::Failed
    }
}

pub struct ExtensionsPage {
    workspace: WeakEntity<Workspace>,
    provider_registry: Arc<GitHostingProviderRegistry>,
    list: UniformListScrollHandle,
    fetch_state: ExtensionFetchState,
    fetch_generation: usize,
    filter: ExtensionFilter,
    extensions: ExtensionList,
    query_editor: Entity<Editor>,
    query_contains_error: bool,
    provides_filter: Option<ExtensionProvides>,
    _subscriptions: [gpui::Subscription; 2],
    extension_fetch_task: Option<Task<()>>,
    local_search_task: Option<Task<()>>,
    upsells: BTreeSet<Feature>,
}

impl ExtensionsPage {
    pub fn new(
        workspace: &Workspace,
        provides_filter: Option<ExtensionProvides>,
        focus_extension_id: Option<&str>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let store = ExtensionStore::global(cx);
            let workspace_handle = workspace.weak_handle();
            let subscriptions = [
                cx.observe(&store, |_: &mut Self, _, cx| cx.notify()),
                cx.subscribe_in(
                    &store,
                    window,
                    move |this, _, event, window, cx| match event {
                        extension_host::Event::ExtensionsUpdated => {
                            this.fetch_extensions_debounced(None, cx)
                        }
                        extension_host::Event::ExtensionInstalled(extension_id) => this
                            .on_extension_installed(
                                workspace_handle.clone(),
                                extension_id,
                                window,
                                cx,
                            ),
                        _ => {}
                    },
                ),
            ];

            let query_editor = cx.new(|cx| {
                let mut input = Editor::single_line(window, cx);
                input.set_placeholder_text("Search extensions...", window, cx);
                if let Some(id) = focus_extension_id {
                    input.set_text(format!("id:{id}"), window, cx);
                }
                input
            });
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let scroll_handle = UniformListScrollHandle::new();
            let provider_registry = GitHostingProviderRegistry::default_global(cx);

            let mut this = Self {
                workspace: workspace.weak_handle(),
                provider_registry,
                list: scroll_handle,
                fetch_state: ExtensionFetchState::Fetching,
                fetch_generation: 0,
                filter: ExtensionFilter::All,
                extensions: ExtensionList::default(),
                query_contains_error: false,
                provides_filter,
                extension_fetch_task: None,
                local_search_task: None,
                _subscriptions: subscriptions,
                query_editor,
                upsells: BTreeSet::default(),
            };
            this.update_local_search_results(cx);
            this.fetch_extensions(
                this.search_query(cx),
                Some(BTreeSet::from_iter(this.provides_filter)),
                None,
                cx,
            );
            this
        })
    }

    fn get_repository_icon(&self, repository_url: &str) -> IconName {
        parse_git_remote_url(Arc::clone(&self.provider_registry), repository_url)
            .map(|(provider, _)| ui::git_hosting_provider_icon(provider.name().as_str()))
            .unwrap_or(IconName::Link)
    }

    fn on_extension_installed(
        &mut self,
        workspace: WeakEntity<Workspace>,
        extension_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let extension_store = ExtensionStore::global(cx).read(cx);
        let themes = extension_store
            .extension_themes(extension_id)
            .map(|name| name.to_string())
            .collect::<Vec<_>>();
        if !themes.is_empty() {
            workspace
                .update(cx, |_workspace, cx| {
                    window.dispatch_action(
                        zed_actions::theme_selector::Toggle {
                            themes_filter: Some(themes),
                        }
                        .boxed_clone(),
                        cx,
                    );
                })
                .ok();
            return;
        }

        let icon_themes = extension_store
            .extension_icon_themes(extension_id)
            .map(|name| name.to_string())
            .collect::<Vec<_>>();
        if !icon_themes.is_empty() {
            workspace
                .update(cx, |_workspace, cx| {
                    window.dispatch_action(
                        zed_actions::icon_theme_selector::Toggle {
                            themes_filter: Some(icon_themes),
                        }
                        .boxed_clone(),
                        cx,
                    );
                })
                .ok();
        }
    }

    /// Runs the search against the locally installed extensions, independently of
    /// the remote fetch, so local results show up without waiting for the server.
    fn update_local_search_results(&mut self, cx: &mut Context<Self>) {
        let search = self.search_query(cx);
        let installed_extensions = ExtensionStore::global(cx)
            .read(cx)
            .installed_extensions()
            .clone();

        self.local_search_task = Some(cx.spawn(async move |this, cx| {
            let matching_extension_ids: OptionFuture<_> = search
                .as_ref()
                .map(async |search| {
                    if let Some(extension_id) = search.strip_prefix("id:") {
                        installed_extensions
                            .contains_key(extension_id)
                            .then(|| BTreeSet::from([Arc::<str>::from(extension_id)]))
                            .unwrap_or_default()
                    } else {
                        let installed = installed_extensions.iter().collect::<Vec<_>>();
                        let match_candidates = installed
                            .iter()
                            .enumerate()
                            .map(|(index, (_, extension))| {
                                StringMatchCandidate::new(index, &extension.manifest.name)
                            })
                            .collect::<Vec<_>>();

                        let matches = match_strings(
                            &match_candidates,
                            search,
                            false,
                            true,
                            match_candidates.len(),
                            &Default::default(),
                            cx.background_executor().clone(),
                        )
                        .await;
                        matches
                            .into_iter()
                            .filter_map(|matched| {
                                installed
                                    .get(matched.candidate_id)
                                    .map(|(extension_id, _)| (*extension_id).clone())
                            })
                            .collect()
                    }
                })
                .into();

            let matching_extension_ids = matching_extension_ids.await;

            this.update(cx, |this, cx| {
                this.extensions.update_installed_extensions(
                    &installed_extensions,
                    matching_extension_ids.as_ref(),
                );
                this.rebuild_displayed_extensions(cx);
            })
            .ok();
        }));
    }

    fn rebuild_displayed_extensions(&mut self, cx: &mut Context<Self>) {
        self.extensions
            .rebuild_displayed(self.filter, self.provides_filter);
        cx.notify();
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    fn fetch_extensions(
        &mut self,
        search: Option<String>,
        provides_filter: Option<BTreeSet<ExtensionProvides>>,
        on_complete: Option<Box<dyn FnOnce(&mut Self, &mut Context<Self>) + Send>>,
        cx: &mut Context<Self>,
    ) {
        self.fetch_state = ExtensionFetchState::Fetching;
        self.fetch_generation = self.fetch_generation.wrapping_add(1);
        let fetch_generation = self.fetch_generation;
        cx.notify();

        let extension_store = ExtensionStore::global(cx);

        let remote_extensions = if let Some(id) = search
            .as_ref()
            .and_then(|search| search.strip_prefix("id:"))
        {
            let versions =
                extension_store.update(cx, |store, cx| store.fetch_extension_versions(id, cx));
            cx.foreground_executor().spawn(async move {
                let versions = versions.await?;
                let latest = versions
                    .into_iter()
                    .max_by_key(|version| version.published_at)
                    .context("no extension found")?;
                Ok(vec![latest])
            })
        } else {
            extension_store.update(cx, |store, cx| {
                store.fetch_extensions(search.as_deref(), provides_filter.as_ref(), cx)
            })
        };

        cx.spawn(async move |this, cx| {
            let fetch_result = remote_extensions.await;

            let result = this.update(cx, |this, cx| {
                if fetch_generation != this.fetch_generation {
                    return Ok(());
                }

                match fetch_result {
                    Ok(remote_extensions) => {
                        this.fetch_state = ExtensionFetchState::Succeeded;
                        this.extensions.set_remote_search_results(remote_extensions);
                        this.rebuild_displayed_extensions(cx);
                        if let Some(callback) = on_complete {
                            callback(this, cx);
                        }
                        Ok(())
                    }
                    Err(error) => {
                        // Keep the last successfully fetched remote extensions so that
                        // going offline doesn't wipe the list; the failure banner is
                        // shown instead.
                        this.fetch_state = ExtensionFetchState::Failed;
                        this.rebuild_displayed_extensions(cx);
                        Err(error)
                    }
                }
            });

            result?
        })
        .detach_and_log_err(cx);
    }

    fn render_extensions(
        &mut self,
        range: Range<usize>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<ExtensionCard> {
        range
            .filter_map(|index| {
                let row = self.extensions.displayed.get(index)?.clone();
                self.render_extension(&row, cx)
            })
            .collect()
    }

    fn render_extension(
        &self,
        row: &DisplayedExtension,
        cx: &mut Context<Self>,
    ) -> Option<ExtensionCard> {
        let entry = match row {
            DisplayedExtension::Local(extension_id) | DisplayedExtension::Remote(extension_id) => {
                self.extensions.get(extension_id)?
            }
        };
        let (card, repository_url, is_remote_card) = match row {
            DisplayedExtension::Local(_) => {
                let manifest = entry.manifest.as_ref()?;
                if entry.dev {
                    (
                        ExtensionCard::for_dev(manifest.clone(), cx),
                        manifest.repository.as_deref(),
                        false,
                    )
                } else if let Some(metadata) = &entry.metadata {
                    (
                        ExtensionCard::for_remote(metadata, cx),
                        Some(metadata.manifest.repository.as_str()),
                        true,
                    )
                } else {
                    (
                        ExtensionCard::for_installed(manifest.clone(), cx),
                        manifest.repository.as_deref(),
                        false,
                    )
                }
            }
            DisplayedExtension::Remote(_) => {
                let metadata = entry.metadata.as_ref()?;
                (
                    ExtensionCard::for_remote(metadata, cx),
                    Some(metadata.manifest.repository.as_str()),
                    true,
                )
            }
        };

        let card = match repository_url {
            Some(repository_url) => card.repository_icon(self.get_repository_icon(repository_url)),
            None => card,
        };
        if !is_remote_card {
            return Some(card);
        }

        let weak_self = cx.weak_entity();
        Some(card.context_menu(move |extension_id, authors, window, cx| {
            let this = weak_self.upgrade()?;
            Some(Self::render_remote_extension_context_menu(
                &this,
                extension_id,
                authors,
                window,
                cx,
            ))
        }))
    }

    fn render_remote_extension_context_menu(
        this: &Entity<Self>,
        extension_id: Arc<str>,
        authors: SharedString,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |context_menu, window, _| {
            context_menu
                .entry(
                    "Install Another Version...",
                    None,
                    window.handler_for(this, {
                        let extension_id = extension_id.clone();
                        move |this, window, cx| {
                            this.show_extension_version_list(extension_id.clone(), window, cx)
                        }
                    }),
                )
                .entry("Copy Extension ID", None, {
                    let extension_id = extension_id.clone();
                    move |_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(extension_id.to_string()));
                    }
                })
                .entry("Copy Author Info", None, move |_, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(authors.to_string()));
                })
        })
    }

    fn show_extension_version_list(
        &mut self,
        extension_id: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let extension_versions_task = this.update(cx, |_, cx| {
                let extension_store = ExtensionStore::global(cx);

                extension_store.update(cx, |store, cx| {
                    store.fetch_extension_versions(&extension_id, cx)
                })
            })?;

            let extension_versions = extension_versions_task.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let fs = workspace.project().read(cx).fs().clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let delegate = ExtensionVersionSelectorDelegate::new(
                        fs,
                        cx.entity().downgrade(),
                        extension_versions,
                    );

                    ExtensionVersionSelector::new(delegate, window, cx)
                });
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn render_search(&self, cx: &mut Context<Self>) -> Div {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");

        let editor_border = if self.query_contains_error {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        };

        h_flex()
            .key_context(key_context)
            .h_8()
            .min_w(rems_from_px(384_f32))
            .flex_1()
            .pl_1p5()
            .pr_2()
            .gap_2()
            .border_1()
            .border_color(editor_border)
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

    fn on_query_change(
        &mut self,
        _: Entity<Editor>,
        event: &editor::EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if let editor::EditorEvent::Edited { .. } = event {
            self.query_contains_error = false;
            self.refresh_search(cx);
        }
    }

    fn refresh_search(&mut self, cx: &mut Context<Self>) {
        self.fetch_extensions_debounced(
            Some(Box::new(|this, cx| {
                this.scroll_to_top(cx);
            })),
            cx,
        );
        self.refresh_feature_upsells(cx);
    }

    pub fn focus_extension(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |editor, cx| {
            editor.set_text(format!("id:{id}"), window, cx)
        });
        self.refresh_search(cx);
    }

    pub fn change_provides_filter(
        &mut self,
        provides_filter: Option<ExtensionProvides>,
        cx: &mut Context<Self>,
    ) {
        self.provides_filter = provides_filter;
        self.refresh_search(cx);
    }

    fn fetch_extensions_debounced(
        &mut self,
        on_complete: Option<Box<dyn FnOnce(&mut Self, &mut Context<Self>) + Send>>,
        cx: &mut Context<ExtensionsPage>,
    ) {
        // Local results don't need to wait for the debounce or the remote fetch.
        self.update_local_search_results(cx);

        self.fetch_state = ExtensionFetchState::Fetching;
        self.fetch_generation = self.fetch_generation.wrapping_add(1);
        cx.notify();

        self.extension_fetch_task = Some(cx.spawn(async move |this, cx| {
            let search = this
                .update(cx, |this, cx| this.search_query(cx))
                .ok()
                .flatten();

            // Only debounce the fetching of extensions if we have a search
            // query.
            //
            // If the search was just cleared then we can just reload the list
            // of extensions without a debounce, which allows us to avoid seeing
            // an intermittent flash of a "no extensions" state.
            if search.is_some() {
                cx.background_executor()
                    .timer(Duration::from_millis(250))
                    .await;
            };

            this.update(cx, |this, cx| {
                this.fetch_extensions(
                    search,
                    Some(BTreeSet::from_iter(this.provides_filter)),
                    on_complete,
                    cx,
                );
            })
            .ok();
        }));
    }

    pub fn search_query(&self, cx: &mut App) -> Option<String> {
        let search = self.query_editor.read(cx).text(cx);
        if search.trim().is_empty() {
            None
        } else {
            Some(search)
        }
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_search = self.search_query(cx).is_some();

        // The `Installed` filter is fully local, so fetch progress and failure are
        // only relevant to the other filters.
        let fetch_is_relevant = self.filter != ExtensionFilter::Installed;
        let fetch_failure_is_relevant = self.fetch_state.failed() && fetch_is_relevant;
        let message = if self.fetch_state.is_fetching() && fetch_is_relevant {
            "Loading extensions…"
        } else if fetch_failure_is_relevant {
            "Failed to load extensions. Please check your connection and try again."
        } else {
            match self.filter {
                ExtensionFilter::All => {
                    if has_search {
                        "No extensions that match your search."
                    } else {
                        "No extensions."
                    }
                }
                ExtensionFilter::Installed => {
                    if has_search {
                        "No installed extensions that match your search."
                    } else {
                        "No installed extensions."
                    }
                }
                ExtensionFilter::NotInstalled => {
                    if has_search {
                        "No not installed extensions that match your search."
                    } else {
                        "No not installed extensions."
                    }
                }
            }
        };

        h_flex()
            .py_4()
            .gap_1p5()
            .when(fetch_failure_is_relevant, |this| {
                this.child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
            })
            .child(Label::new(message))
    }

    fn update_settings(
        &mut self,
        selection: &ToggleState,
        cx: &mut Context<Self>,
        callback: impl 'static + Send + Fn(&mut SettingsContent, bool),
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            let selection = *selection;
            settings::update_settings_file(fs, cx, move |settings, _| {
                let value = match selection {
                    ToggleState::Unselected => false,
                    ToggleState::Selected => true,
                    _ => return,
                };

                callback(settings, value)
            });
        }
    }

    fn refresh_feature_upsells(&mut self, cx: &mut Context<Self>) {
        let Some(search) = self.search_query(cx) else {
            self.upsells.clear();
            return;
        };

        if let Some(id) = search.strip_prefix("id:") {
            self.upsells.clear();

            let upsell = match id.to_lowercase().as_str() {
                "ruff" => Some(Feature::ExtensionRuff),
                "basedpyright" => Some(Feature::ExtensionBasedpyright),
                "ty" => Some(Feature::ExtensionTy),
                _ => None,
            };

            if let Some(upsell) = upsell {
                self.upsells.insert(upsell);
            }

            return;
        }

        let search = search.to_lowercase();
        let search_terms = search
            .split_whitespace()
            .map(|term| term.trim())
            .collect::<Vec<_>>();

        for (feature, keywords) in keywords_by_feature() {
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
        vim: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let docs_url_button = Button::new("open_docs", "View Documentation")
            .end_icon(Icon::new(IconName::ArrowUpRight).size(IconSize::Small))
            .on_click({
                move |_event, _window, cx| {
                    telemetry::event!(
                        "Documentation Viewed",
                        source = "Feature Upsell",
                        url = docs_url,
                    );
                    cx.open_url(&docs_url)
                }
            });

        div()
            .pt_4()
            .px_4()
            .child(
                Banner::new()
                    .severity(Severity::Success)
                    .child(Label::new(label).mt_0p5())
                    .map(|this| {
                        if vim {
                            this.action_slot(
                                h_flex()
                                    .gap_1()
                                    .child(docs_url_button)
                                    .child(Divider::vertical().color(ui::DividerColor::Border))
                                    .child(
                                        h_flex()
                                            .pl_1()
                                            .gap_1()
                                            .child(Label::new("Enable Vim mode"))
                                            .child(
                                                Switch::new(
                                                    "enable-vim",
                                                    if VimModeSetting::get_global(cx).0 {
                                                        ui::ToggleState::Selected
                                                    } else {
                                                        ui::ToggleState::Unselected
                                                    },
                                                )
                                                .on_click(cx.listener(
                                                    move |this, selection, _, cx| {
                                                        telemetry::event!(
                                                            "Vim Mode Toggled",
                                                            source = "Feature Upsell"
                                                        );
                                                        this.update_settings(
                                                            selection,
                                                            cx,
                                                            |setting, value| {
                                                                setting.vim_mode = Some(value)
                                                            },
                                                        );
                                                    },
                                                )),
                                            ),
                                    ),
                            )
                        } else {
                            this.action_slot(docs_url_button)
                        }
                    }),
            )
            .into_any_element()
    }

    fn render_feature_upsells(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = v_flex();

        for feature in &self.upsells {
            let banner = match feature {
                Feature::AgentClaude => self.render_feature_upsell_banner(
                    "Claude Agent support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#claude-agent".into(),
                    false,
                    cx,
                ),
                Feature::AgentCodex => self.render_feature_upsell_banner(
                    "Codex CLI support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#codex-cli".into(),
                    false,
                    cx,
                ),
                Feature::AgentGemini => self.render_feature_upsell_banner(
                    "Gemini CLI support is built-in to Zed!".into(),
                    "https://zed.dev/docs/ai/external-agents#gemini-cli".into(),
                    false,
                    cx,
                ),
                Feature::ExtensionBasedpyright => self.render_feature_upsell_banner(
                    "Basedpyright (Python language server) support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/python#basedpyright".into(),
                    false,
                    cx,
                ),
                Feature::ExtensionRuff => self.render_feature_upsell_banner(
                    "Ruff (linter for Python) support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/python#code-formatting--linting".into(),
                    false,
                    cx,
                ),
                Feature::ExtensionTailwind => self.render_feature_upsell_banner(
                    "Tailwind CSS support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/tailwindcss".into(),
                    false,
                    cx,
                ),
                Feature::ExtensionTy => self.render_feature_upsell_banner(
                    "Ty (Python language server) support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/python".into(),
                    false,
                    cx,
                ),
                Feature::Git => self.render_feature_upsell_banner(
                    "Zed comes with basic Git support—more features are coming in the future."
                        .into(),
                    "https://zed.dev/docs/git".into(),
                    false,
                    cx,
                ),
                Feature::LanguageBash => self.render_feature_upsell_banner(
                    "Shell support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/bash".into(),
                    false,
                    cx,
                ),
                Feature::LanguageC => self.render_feature_upsell_banner(
                    "C support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/c".into(),
                    false,
                    cx,
                ),
                Feature::LanguageCpp => self.render_feature_upsell_banner(
                    "C++ support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/cpp".into(),
                    false,
                    cx,
                ),
                Feature::LanguageGo => self.render_feature_upsell_banner(
                    "Go support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/go".into(),
                    false,
                    cx,
                ),
                Feature::LanguagePython => self.render_feature_upsell_banner(
                    "Python support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/python".into(),
                    false,
                    cx,
                ),
                Feature::LanguageReact => self.render_feature_upsell_banner(
                    "React support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/typescript".into(),
                    false,
                    cx,
                ),
                Feature::LanguageRust => self.render_feature_upsell_banner(
                    "Rust support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/rust".into(),
                    false,
                    cx,
                ),
                Feature::LanguageTypescript => self.render_feature_upsell_banner(
                    "Typescript support is built-in to Zed!".into(),
                    "https://zed.dev/docs/languages/typescript".into(),
                    false,
                    cx,
                ),
                Feature::OpenIn => self.render_feature_upsell_banner(
                    "Zed supports linking to a source line on GitHub and others.".into(),
                    "https://zed.dev/docs/git#git-integrations".into(),
                    false,
                    cx,
                ),
                Feature::Vim => self.render_feature_upsell_banner(
                    "Vim support is built-in to Zed!".into(),
                    "https://zed.dev/docs/vim".into(),
                    true,
                    cx,
                ),
            };
            container = container.child(banner);
        }

        container
    }
}

struct DevExtensionRebuildPickerDelegate {
    entries: Vec<Arc<ExtensionManifest>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl DevExtensionRebuildPickerDelegate {
    fn new(manifests: Vec<Arc<ExtensionManifest>>) -> Self {
        let matches = manifests
            .iter()
            .enumerate()
            .map(|(ix, manifest)| StringMatch {
                candidate_id: ix,
                score: 0.0,
                positions: Vec::new(),
                string: manifest.name.clone(),
            })
            .collect();

        Self {
            entries: manifests,
            matches,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for DevExtensionRebuildPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "dev-extension-rebuild"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn selected_index_changed(
        &self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut Window, &mut App) + 'static>> {
        None
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .entries
            .iter()
            .enumerate()
            .map(|(ix, manifest)| StringMatchCandidate::new(ix, manifest.name.as_ref()))
            .collect::<Vec<_>>();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };

        let extension_id = self.entries[mat.candidate_id].id.clone();
        ExtensionStore::global(cx).update(cx, |store, cx| {
            store.rebuild_dev_extension(extension_id, cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from("Rebuild dev extension…")
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let entry = self.entries.get(mat.candidate_id)?;

        let item = ListItem::new(("dev-extension-list-item", mat.candidate_id))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .child(
                h_flex()
                    .w_full()
                    .py_px()
                    .justify_between()
                    .gap_2()
                    .child(Label::new(entry.name.clone()))
                    .child(
                        Label::new(format!("{} • v{}", entry.id, entry.version))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            );

        Some(item)
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No dev extensions found".into())
    }
}

impl Render for ExtensionsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .gap_4()
                    .pt_4()
                    .px_4()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_1p5()
                            .justify_between()
                            .child(Headline::new("Extensions").size(HeadlineSize::Large))
                            .child(
                                Button::new("install-dev-extension", "Install Dev Extension")
                                    .style(ButtonStyle::Outlined)
                                    .size(ButtonSize::Medium)
                                    .on_click(|_event, window, cx| {
                                        window.dispatch_action(Box::new(InstallDevExtension), cx)
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
                                        "filter-buttons",
                                        [
                                            ToggleButtonSimple::new(
                                                "All",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = ExtensionFilter::All;
                                                    this.rebuild_displayed_extensions(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                            ToggleButtonSimple::new(
                                                "Installed",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = ExtensionFilter::Installed;
                                                    this.rebuild_displayed_extensions(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                            ToggleButtonSimple::new(
                                                "Not Installed",
                                                cx.listener(|this, _event, _, cx| {
                                                    this.filter = ExtensionFilter::NotInstalled;
                                                    this.rebuild_displayed_extensions(cx);
                                                    this.scroll_to_top(cx);
                                                }),
                                            ),
                                        ],
                                    )
                                    .style(ToggleButtonGroupStyle::Outlined)
                                    .size(ToggleButtonGroupSize::Custom(rems_from_px(30_f32))) // Perfectly matches the input
                                    .label_size(LabelSize::Default)
                                    .auto_width()
                                    .selected_index(match self.filter {
                                        ExtensionFilter::All => 0,
                                        ExtensionFilter::Installed => 1,
                                        ExtensionFilter::NotInstalled => 2,
                                    })
                                    .into_any_element(),
                                ),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .id("filter-row")
                    .gap_2()
                    .py_2p5()
                    .px_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .overflow_x_scroll()
                    .child(
                        Button::new("filter-all-categories", "All")
                            .when(self.provides_filter.is_none(), |button| {
                                button.style(ButtonStyle::Filled)
                            })
                            .when(self.provides_filter.is_some(), |button| {
                                button.style(ButtonStyle::Subtle)
                            })
                            .toggle_state(self.provides_filter.is_none())
                            .on_click(cx.listener(|this, _event, _, cx| {
                                this.change_provides_filter(None, cx);
                            })),
                    )
                    .children(
                        ExtensionProvides::iter()
                            .filter(|provides| match provides {
                                ExtensionProvides::AgentServers
                                | ExtensionProvides::Grammars // grammars do not add anything of value to users currently
                                | ExtensionProvides::IndexedDocsProviders
                                | ExtensionProvides::SlashCommands => false,
                                _ => true,
                            })
                            .map(|provides| {
                                let label = extension_provides_label(provides);
                                let button_id =
                                    SharedString::from(format!("filter-category-{}", label));

                                Button::new(button_id, label)
                                    .style(if self.provides_filter == Some(provides) {
                                        ButtonStyle::Filled
                                    } else {
                                        ButtonStyle::Subtle
                                    })
                                    .toggle_state(self.provides_filter == Some(provides))
                                    .on_click({
                                        cx.listener(move |this, _event, _, cx| {
                                            this.change_provides_filter(Some(provides), cx);
                                        })
                                    })
                            }),
                    ),
            )
            .child(self.render_feature_upsells(cx))
            .child(v_flex().px_4().size_full().overflow_y_hidden().map(|this| {
                let count = self.extensions.displayed.len();

                if count == 0 {
                    this.child(self.render_empty_state(cx)).into_any_element()
                } else {
                    let scroll_handle = &self.list;
                    this.child(
                        uniform_list("entries", count, cx.processor(Self::render_extensions))
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

impl EventEmitter<ItemEvent> for ExtensionsPage {}

impl Focusable for ExtensionsPage {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.query_editor.read(cx).focus_handle(cx)
    }
}

impl Item for ExtensionsPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Extensions".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Extensions Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
