use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anyhow::{Result, anyhow, bail};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{App, BackgroundExecutor, Entity, Task, WeakEntity};
use indexed_docs::{
    DocsDotRsProvider, IndexedDocsRegistry, IndexedDocsStore, LocalRustdocProvider, PackageName,
    ProviderId,
};
use language::{BufferSnapshot, LspAdapterDelegate};
use project::{Project, ProjectPath};
use ui::prelude::*;
use util::{ResultExt, maybe};
use workspace::Workspace;

pub struct DocsSlashCommand;

impl DocsSlashCommand {
    pub const NAME: &'static str = "docs";

    fn path_to_cargo_toml(project: Entity<Project>, cx: &mut App) -> Option<Arc<Path>> {
        let worktree = project.read(cx).worktrees(cx).next()?;
        let worktree = worktree.read(cx);
        let entry = worktree.entry_for_path("Cargo.toml")?;
        let path = ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path.clone(),
        };
        Some(Arc::from(
            project.read(cx).absolute_path(&path, cx)?.as_path(),
        ))
    }

    /// Ensures that the indexed doc providers for Rust are registered.
    ///
    /// Ideally we would do this sooner, but we need to wait until we're able to
    /// access the workspace so we can read the project.
    fn ensure_rust_doc_providers_are_registered(
        &self,
        workspace: Option<WeakEntity<Workspace>>,
        cx: &mut App,
    ) {
        let indexed_docs_registry = IndexedDocsRegistry::global(cx);
        if indexed_docs_registry
            .get_provider_store(LocalRustdocProvider::id())
            .is_none()
        {
            let index_provider_deps = maybe!({
                let workspace = workspace.clone().ok_or_else(|| anyhow!("no workspace"))?;
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace was dropped"))?;
                let project = workspace.read(cx).project().clone();
                let fs = project.read(cx).fs().clone();
                let cargo_workspace_root = Self::path_to_cargo_toml(project, cx)
                    .and_then(|path| path.parent().map(|path| path.to_path_buf()))
                    .ok_or_else(|| anyhow!("no Cargo workspace root found"))?;

                anyhow::Ok((fs, cargo_workspace_root))
            });

            if let Some((fs, cargo_workspace_root)) = index_provider_deps.log_err() {
                indexed_docs_registry.register_provider(Box::new(LocalRustdocProvider::new(
                    fs,
                    cargo_workspace_root,
                )));
            }
        }

        if indexed_docs_registry
            .get_provider_store(DocsDotRsProvider::id())
            .is_none()
        {
            let http_client = maybe!({
                let workspace = workspace.ok_or_else(|| anyhow!("no workspace"))?;
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace was dropped"))?;
                let project = workspace.read(cx).project().clone();
                anyhow::Ok(project.read(cx).client().http_client())
            });

            if let Some(http_client) = http_client.log_err() {
                indexed_docs_registry
                    .register_provider(Box::new(DocsDotRsProvider::new(http_client)));
            }
        }
    }

    /// Runs just-in-time indexing for a given package, in case the slash command
    /// is run without any entries existing in the index.
    fn run_just_in_time_indexing(
        store: Arc<IndexedDocsStore>,
        key: String,
        package: PackageName,
        executor: BackgroundExecutor,
    ) -> Task<()> {
        executor.clone().spawn(async move {
            let (prefix, needs_full_index) = if let Some((prefix, _)) = key.split_once('*') {
                // If we have a wildcard in the search, we want to wait until
                // we've completely finished indexing so we get a full set of
                // results for the wildcard.
                (prefix.to_string(), true)
            } else {
                (key, false)
            };

            // If we already have some entries, we assume that we've indexed the package before
            // and don't need to do it again.
            let has_any_entries = store
                .any_with_prefix(prefix.clone())
                .await
                .unwrap_or_default();
            if has_any_entries {
                return ();
            };

            let index_task = store.clone().index(package.clone());

            if needs_full_index {
                _ = index_task.await;
            } else {
                loop {
                    executor.timer(Duration::from_millis(200)).await;

                    if store
                        .any_with_prefix(prefix.clone())
                        .await
                        .unwrap_or_default()
                        || !store.is_indexing(&package)
                    {
                        break;
                    }
                }
            }
        })
    }
}

impl SlashCommand for DocsSlashCommand {
    fn name(&self) -> String {
        Self::NAME.into()
    }

    fn description(&self) -> String {
        "insert docs".into()
    }

    fn menu_text(&self) -> String {
        "Insert Documentation".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        workspace: Option<WeakEntity<Workspace>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        self.ensure_rust_doc_providers_are_registered(workspace, cx);

        let indexed_docs_registry = IndexedDocsRegistry::global(cx);
        let args = DocsSlashCommandArgs::parse(arguments);
        let store = args
            .provider()
            .ok_or_else(|| anyhow!("no docs provider specified"))
            .and_then(|provider| IndexedDocsStore::try_global(provider, cx));
        cx.background_spawn(async move {
            fn build_completions(items: Vec<String>) -> Vec<ArgumentCompletion> {
                items
                    .into_iter()
                    .map(|item| ArgumentCompletion {
                        label: item.clone().into(),
                        new_text: item.to_string(),
                        after_completion: assistant_slash_command::AfterCompletion::Run,
                        replace_previous_arguments: false,
                    })
                    .collect()
            }

            match args {
                DocsSlashCommandArgs::NoProvider => {
                    let providers = indexed_docs_registry.list_providers();
                    if providers.is_empty() {
                        return Ok(vec![ArgumentCompletion {
                            label: "No available docs providers.".into(),
                            new_text: String::new(),
                            after_completion: false.into(),
                            replace_previous_arguments: false,
                        }]);
                    }

                    Ok(providers
                        .into_iter()
                        .map(|provider| ArgumentCompletion {
                            label: provider.to_string().into(),
                            new_text: provider.to_string(),
                            after_completion: false.into(),
                            replace_previous_arguments: false,
                        })
                        .collect())
                }
                DocsSlashCommandArgs::SearchPackageDocs {
                    provider,
                    package,
                    index,
                } => {
                    let store = store?;

                    if index {
                        // We don't need to hold onto this task, as the `IndexedDocsStore` will hold it
                        // until it completes.
                        drop(store.clone().index(package.as_str().into()));
                    }

                    let suggested_packages = store.clone().suggest_packages().await?;
                    let search_results = store.search(package).await;

                    let mut items = build_completions(search_results);
                    let workspace_crate_completions = suggested_packages
                        .into_iter()
                        .filter(|package_name| {
                            !items
                                .iter()
                                .any(|item| item.label.text() == package_name.as_ref())
                        })
                        .map(|package_name| ArgumentCompletion {
                            label: format!("{package_name} (unindexed)").into(),
                            new_text: format!("{package_name}"),
                            after_completion: true.into(),
                            replace_previous_arguments: false,
                        })
                        .collect::<Vec<_>>();
                    items.extend(workspace_crate_completions);

                    if items.is_empty() {
                        return Ok(vec![ArgumentCompletion {
                            label: format!(
                                "Enter a {package_term} name.",
                                package_term = package_term(&provider)
                            )
                            .into(),
                            new_text: provider.to_string(),
                            after_completion: false.into(),
                            replace_previous_arguments: false,
                        }]);
                    }

                    Ok(items)
                }
                DocsSlashCommandArgs::SearchItemDocs { item_path, .. } => {
                    let store = store?;
                    let items = store.search(item_path).await;
                    Ok(build_completions(items))
                }
            }
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        if arguments.is_empty() {
            return Task::ready(Err(anyhow!("missing an argument")));
        };

        let args = DocsSlashCommandArgs::parse(arguments);
        let executor = cx.background_executor().clone();
        let task = cx.background_spawn({
            let store = args
                .provider()
                .ok_or_else(|| anyhow!("no docs provider specified"))
                .and_then(|provider| IndexedDocsStore::try_global(provider, cx));
            async move {
                let (provider, key) = match args.clone() {
                    DocsSlashCommandArgs::NoProvider => bail!("no docs provider specified"),
                    DocsSlashCommandArgs::SearchPackageDocs {
                        provider, package, ..
                    } => (provider, package),
                    DocsSlashCommandArgs::SearchItemDocs {
                        provider,
                        item_path,
                        ..
                    } => (provider, item_path),
                };

                if key.trim().is_empty() {
                    bail!(
                        "no {package_term} name provided",
                        package_term = package_term(&provider)
                    );
                }

                let store = store?;

                if let Some(package) = args.package() {
                    Self::run_just_in_time_indexing(store.clone(), key.clone(), package, executor)
                        .await;
                }

                let (text, ranges) = if let Some((prefix, _)) = key.split_once('*') {
                    let docs = store.load_many_by_prefix(prefix.to_string()).await?;

                    let mut text = String::new();
                    let mut ranges = Vec::new();

                    for (key, docs) in docs {
                        let prev_len = text.len();

                        text.push_str(&docs.0);
                        text.push_str("\n");
                        ranges.push((key, prev_len..text.len()));
                        text.push_str("\n");
                    }

                    (text, ranges)
                } else {
                    let item_docs = store.load(key.clone()).await?;
                    let text = item_docs.to_string();
                    let range = 0..text.len();

                    (text, vec![(key, range)])
                };

                anyhow::Ok((provider, text, ranges))
            }
        });

        cx.foreground_executor().spawn(async move {
            let (provider, text, ranges) = task.await?;
            Ok(SlashCommandOutput {
                text,
                sections: ranges
                    .into_iter()
                    .map(|(key, range)| SlashCommandOutputSection {
                        range,
                        icon: IconName::FileDoc,
                        label: format!("docs ({provider}): {key}",).into(),
                        metadata: None,
                    })
                    .collect(),
                run_commands_in_text: false,
            }
            .to_event_stream())
        })
    }
}

fn is_item_path_delimiter(char: char) -> bool {
    !char.is_alphanumeric() && char != '-' && char != '_'
}

#[derive(Debug, PartialEq, Clone)]
pub enum DocsSlashCommandArgs {
    NoProvider,
    SearchPackageDocs {
        provider: ProviderId,
        package: String,
        index: bool,
    },
    SearchItemDocs {
        provider: ProviderId,
        package: String,
        item_path: String,
    },
}

impl DocsSlashCommandArgs {
    pub fn parse(arguments: &[String]) -> Self {
        let Some(provider) = arguments
            .get(0)
            .cloned()
            .filter(|arg| !arg.trim().is_empty())
        else {
            return Self::NoProvider;
        };
        let provider = ProviderId(provider.into());
        let Some(argument) = arguments.get(1) else {
            return Self::NoProvider;
        };

        if let Some((package, rest)) = argument.split_once(is_item_path_delimiter) {
            if rest.trim().is_empty() {
                Self::SearchPackageDocs {
                    provider,
                    package: package.to_owned(),
                    index: true,
                }
            } else {
                Self::SearchItemDocs {
                    provider,
                    package: package.to_owned(),
                    item_path: argument.to_owned(),
                }
            }
        } else {
            Self::SearchPackageDocs {
                provider,
                package: argument.to_owned(),
                index: false,
            }
        }
    }

    pub fn provider(&self) -> Option<ProviderId> {
        match self {
            Self::NoProvider => None,
            Self::SearchPackageDocs { provider, .. } | Self::SearchItemDocs { provider, .. } => {
                Some(provider.clone())
            }
        }
    }

    pub fn package(&self) -> Option<PackageName> {
        match self {
            Self::NoProvider => None,
            Self::SearchPackageDocs { package, .. } | Self::SearchItemDocs { package, .. } => {
                Some(package.as_str().into())
            }
        }
    }
}

/// Returns the term used to refer to a package.
fn package_term(provider: &ProviderId) -> &'static str {
    if provider == &DocsDotRsProvider::id() || provider == &LocalRustdocProvider::id() {
        return "crate";
    }

    "package"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_docs_slash_command_args() {
        assert_eq!(
            DocsSlashCommandArgs::parse(&["".to_string()]),
            DocsSlashCommandArgs::NoProvider
        );
        assert_eq!(
            DocsSlashCommandArgs::parse(&["rustdoc".to_string()]),
            DocsSlashCommandArgs::NoProvider
        );

        assert_eq!(
            DocsSlashCommandArgs::parse(&["rustdoc".to_string(), "".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "".into(),
                index: false
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse(&["gleam".to_string(), "".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "".into(),
                index: false
            }
        );

        assert_eq!(
            DocsSlashCommandArgs::parse(&["rustdoc".to_string(), "gpui".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                index: false,
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse(&["gleam".to_string(), "gleam_stdlib".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                index: false
            }
        );

        // Adding an item path delimiter indicates we can start indexing.
        assert_eq!(
            DocsSlashCommandArgs::parse(&["rustdoc".to_string(), "gpui:".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                index: true,
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse(&["gleam".to_string(), "gleam_stdlib/".to_string()]),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                index: true
            }
        );

        assert_eq!(
            DocsSlashCommandArgs::parse(&[
                "rustdoc".to_string(),
                "gpui::foo::bar::Baz".to_string()
            ]),
            DocsSlashCommandArgs::SearchItemDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                item_path: "gpui::foo::bar::Baz".into()
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse(&[
                "gleam".to_string(),
                "gleam_stdlib/gleam/int".to_string()
            ]),
            DocsSlashCommandArgs::SearchItemDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                item_path: "gleam_stdlib/gleam/int".into()
            }
        );
    }
}
