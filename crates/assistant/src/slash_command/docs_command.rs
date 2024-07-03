use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use fs::Fs;
use futures::AsyncReadExt;
use gpui::{AppContext, Model, Task, WeakView};
use http::{AsyncBody, HttpClient, HttpClientWithUrl};
use indexed_docs::{
    convert_rustdoc_to_markdown, IndexedDocsRegistry, IndexedDocsStore, LocalProvider, PackageName,
    ProviderId, RustdocIndexer, RustdocSource,
};
use language::LspAdapterDelegate;
use project::{Project, ProjectPath};
use ui::prelude::*;
use util::{maybe, ResultExt};
use workspace::Workspace;

fn is_item_path_delimiter(char: char) -> bool {
    !char.is_alphanumeric() && char != '-' && char != '_'
}

#[derive(Debug)]
enum DocsSlashCommandArgs {
    NoProvider,
    ProviderSelected {
        provider: ProviderId,
    },
    SearchPackageDocs {
        provider: ProviderId,
        package: String,
    },
    SearchItemDocs {
        provider: ProviderId,
        item_path: String,
    },
}

impl DocsSlashCommandArgs {
    pub fn parse(argument: &str) -> Self {
        let Some((provider, argument)) = argument.split_once(' ') else {
            return Self::NoProvider;
        };

        let provider = ProviderId(provider.into());

        let Some((package, rest)) = argument.split_once(is_item_path_delimiter) else {
            return Self::ProviderSelected { provider };
        };

        if rest.trim().is_empty() {
            return Self::SearchPackageDocs {
                provider,
                package: package.to_owned(),
            };
        }

        let item_path = argument.trim_start_matches(provider.as_ref()).trim();

        Self::SearchItemDocs {
            provider,
            item_path: item_path.to_owned(),
        }
    }

    pub fn provider(&self) -> Option<ProviderId> {
        match self {
            Self::NoProvider => None,
            Self::ProviderSelected { provider }
            | Self::SearchPackageDocs { provider, .. }
            | Self::SearchItemDocs { provider, .. } => Some(provider.clone()),
        }
    }
}

pub(crate) struct DocsSlashCommand;

impl DocsSlashCommand {
    fn path_to_cargo_toml(project: Model<Project>, cx: &mut AppContext) -> Option<Arc<Path>> {
        let worktree = project.read(cx).worktrees().next()?;
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

    /// Ensures that the rustdoc provider is registered.
    ///
    /// Ideally we would do this sooner, but we need to wait until we're able to
    /// access the workspace so we can read the project.
    fn ensure_rustdoc_provider_is_registered(
        &self,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) {
        let indexed_docs_registry = IndexedDocsRegistry::global(cx);
        if indexed_docs_registry
            .get_provider_store(ProviderId::rustdoc())
            .is_none()
        {
            let index_provider_deps = maybe!({
                let workspace = workspace.ok_or_else(|| anyhow!("no workspace"))?;
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
                indexed_docs_registry.register_provider(Box::new(RustdocIndexer::new(Box::new(
                    LocalProvider::new(fs, cargo_workspace_root),
                ))));
            }
        }
    }
}

impl SlashCommand for DocsSlashCommand {
    fn name(&self) -> String {
        "docs".into()
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
        query: String,
        _cancel: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        self.ensure_rustdoc_provider_is_registered(workspace, cx);

        let args = DocsSlashCommandArgs::parse(&query);

        dbg!(&args);

        let indexed_docs_registry = IndexedDocsRegistry::global(cx);
        let store = args
            .provider()
            .ok_or_else(|| anyhow!("no provider specified"))
            .and_then(|provider| IndexedDocsStore::try_global(provider, cx));
        cx.background_executor().spawn(async move {
            match args {
                DocsSlashCommandArgs::NoProvider => {
                    let providers = indexed_docs_registry.list_providers();
                    Ok(providers
                        .into_iter()
                        .map(|provider| provider.to_string())
                        .collect())
                }
                DocsSlashCommandArgs::ProviderSelected { .. } => {
                    let store = store?;
                    let items = store.search(String::new()).await;
                    Ok(items)
                }
                DocsSlashCommandArgs::SearchPackageDocs { package, .. } => {
                    let store = store?;

                    // We don't need to hold onto this task, as the `IndexedDocsStore` will hold it
                    // until it completes.
                    let _ = store.clone().index(package.as_str().into());

                    let items = store.search(package).await;
                    Ok(items)
                }
                DocsSlashCommandArgs::SearchItemDocs { item_path, .. } => {
                    let store = store?;
                    let items = store.search(item_path).await;
                    Ok(items)
                }
            }
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing argument")));
        };
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let mut args = argument.split(' ');
        let Some(provider) = args.next() else {
            return Task::ready(Err(anyhow!("missing provider")));
        };

        let mut path_components = argument.split("::");
        let crate_name = match path_components
            .next()
            .ok_or_else(|| anyhow!("missing crate name"))
        {
            Ok(crate_name) => PackageName::from(crate_name),
            Err(err) => return Task::ready(Err(err)),
        };
        let item_path = path_components.map(ToString::to_string).collect::<Vec<_>>();

        let text = cx.background_executor().spawn({
            let rustdoc_store = IndexedDocsStore::try_global(ProviderId::rustdoc(), cx);
            let crate_name = crate_name.clone();
            let item_path = item_path.clone();
            async move {
                let rustdoc_store = rustdoc_store?;
                let item_docs = rustdoc_store
                    .load(
                        crate_name.clone(),
                        if item_path.is_empty() {
                            None
                        } else {
                            Some(item_path.join("::"))
                        },
                    )
                    .await?;

                anyhow::Ok((RustdocSource::Index, item_docs.to_string()))
            }
        });

        let module_path = if item_path.is_empty() {
            None
        } else {
            Some(SharedString::from(item_path.join("::")))
        };
        cx.foreground_executor().spawn(async move {
            let (source, text) = text.await?;
            let range = 0..text.len();
            let crate_path = module_path
                .map(|module_path| format!("{}::{}", crate_name, module_path))
                .unwrap_or_else(|| crate_name.to_string());
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::FileRust,
                    label: format!(
                        "rustdoc ({source}): {crate_path}",
                        source = match source {
                            RustdocSource::Index => "index",
                            RustdocSource::Local => "local",
                            RustdocSource::DocsDotRs => "docs.rs",
                        }
                    )
                    .into(),
                }],
                run_commands_in_text: false,
            })
        })
    }
}
