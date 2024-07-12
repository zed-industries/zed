use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use gpui::{AppContext, Model, Task, WeakView};
use indexed_docs::{
    DocsDotRsProvider, IndexedDocsRegistry, IndexedDocsStore, LocalRustdocProvider, PackageName,
    ProviderId,
};
use language::LspAdapterDelegate;
use project::{Project, ProjectPath};
use ui::prelude::*;
use util::{maybe, ResultExt};
use workspace::Workspace;

pub(crate) struct DocsSlashCommand;

impl DocsSlashCommand {
    pub const NAME: &'static str = "docs";

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

    /// Ensures that the indexed doc providers for Rust are registered.
    ///
    /// Ideally we would do this sooner, but we need to wait until we're able to
    /// access the workspace so we can read the project.
    fn ensure_rust_doc_providers_are_registered(
        &self,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
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
                anyhow::Ok(project.read(cx).client().http_client().clone())
            });

            if let Some(http_client) = http_client.log_err() {
                indexed_docs_registry
                    .register_provider(Box::new(DocsDotRsProvider::new(http_client)));
            }
        }
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
        query: String,
        _cancel: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        self.ensure_rust_doc_providers_are_registered(workspace, cx);

        let indexed_docs_registry = IndexedDocsRegistry::global(cx);
        let args = DocsSlashCommandArgs::parse(&query);
        let store = args
            .provider()
            .ok_or_else(|| anyhow!("no docs provider specified"))
            .and_then(|provider| IndexedDocsStore::try_global(provider, cx));
        cx.background_executor().spawn(async move {
            fn build_completions(
                provider: ProviderId,
                items: Vec<String>,
            ) -> Vec<ArgumentCompletion> {
                items
                    .into_iter()
                    .map(|item| ArgumentCompletion {
                        label: item.clone(),
                        new_text: format!("{provider} {item}"),
                        run_command: true,
                    })
                    .collect()
            }

            match args {
                DocsSlashCommandArgs::NoProvider => {
                    let providers = indexed_docs_registry.list_providers();
                    if providers.is_empty() {
                        return Ok(vec![ArgumentCompletion {
                            label: "No available docs providers.".to_string(),
                            new_text: String::new(),
                            run_command: false,
                        }]);
                    }

                    Ok(providers
                        .into_iter()
                        .map(|provider| ArgumentCompletion {
                            label: provider.to_string(),
                            new_text: provider.to_string(),
                            run_command: false,
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
                        let _ = store.clone().index(package.as_str().into());
                    }

                    let items = store.search(package).await;
                    Ok(build_completions(provider, items))
                }
                DocsSlashCommandArgs::SearchItemDocs {
                    provider,
                    item_path,
                    ..
                } => {
                    let store = store?;
                    let items = store.search(item_path).await;
                    Ok(build_completions(provider, items))
                }
            }
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing argument")));
        };

        let args = DocsSlashCommandArgs::parse(argument);
        let text = cx.background_executor().spawn({
            let store = args
                .provider()
                .ok_or_else(|| anyhow!("no docs provider specified"))
                .and_then(|provider| IndexedDocsStore::try_global(provider, cx));
            async move {
                match args {
                    DocsSlashCommandArgs::NoProvider => bail!("no docs provider specified"),
                    DocsSlashCommandArgs::SearchPackageDocs {
                        provider, package, ..
                    } => {
                        let store = store?;
                        let item_docs = store.load(package.clone()).await?;

                        anyhow::Ok((provider, package, item_docs.to_string()))
                    }
                    DocsSlashCommandArgs::SearchItemDocs {
                        provider,
                        item_path,
                        ..
                    } => {
                        let store = store?;
                        let item_docs = store.load(item_path.clone()).await?;

                        anyhow::Ok((provider, item_path, item_docs.to_string()))
                    }
                }
            }
        });

        cx.foreground_executor().spawn(async move {
            let (provider, path, text) = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::FileDoc,
                    label: format!("docs ({provider}): {path}",).into(),
                }],
                run_commands_in_text: false,
            })
        })
    }
}

fn is_item_path_delimiter(char: char) -> bool {
    !char.is_alphanumeric() && char != '-' && char != '_'
}

#[derive(Debug, PartialEq)]
pub(crate) enum DocsSlashCommandArgs {
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
    pub fn parse(argument: &str) -> Self {
        let Some((provider, argument)) = argument.split_once(' ') else {
            return Self::NoProvider;
        };

        let provider = ProviderId(provider.into());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_docs_slash_command_args() {
        assert_eq!(
            DocsSlashCommandArgs::parse(""),
            DocsSlashCommandArgs::NoProvider
        );
        assert_eq!(
            DocsSlashCommandArgs::parse("rustdoc"),
            DocsSlashCommandArgs::NoProvider
        );

        assert_eq!(
            DocsSlashCommandArgs::parse("rustdoc "),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "".into(),
                index: false
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse("gleam "),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "".into(),
                index: false
            }
        );

        assert_eq!(
            DocsSlashCommandArgs::parse("rustdoc gpui"),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                index: false,
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse("gleam gleam_stdlib"),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                index: false
            }
        );

        // Adding an item path delimiter indicates we can start indexing.
        assert_eq!(
            DocsSlashCommandArgs::parse("rustdoc gpui:"),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                index: true,
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse("gleam gleam_stdlib/"),
            DocsSlashCommandArgs::SearchPackageDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                index: true
            }
        );

        assert_eq!(
            DocsSlashCommandArgs::parse("rustdoc gpui::foo::bar::Baz"),
            DocsSlashCommandArgs::SearchItemDocs {
                provider: ProviderId("rustdoc".into()),
                package: "gpui".into(),
                item_path: "gpui::foo::bar::Baz".into()
            }
        );
        assert_eq!(
            DocsSlashCommandArgs::parse("gleam gleam_stdlib/gleam/int"),
            DocsSlashCommandArgs::SearchItemDocs {
                provider: ProviderId("gleam".into()),
                package: "gleam_stdlib".into(),
                item_path: "gleam_stdlib/gleam/int".into()
            }
        );
    }
}
