//! Native dbt support: the "dbt SQL" language (Jinja host grammar with combined
//! SQL injection), a hidden "SQL (dbt)" injection-target language, a language
//! server adapter (dbt Fusion's `dbt lsp` preferred, community Go
//! `dbt-language-server` as fallback with auto-download), dbt_project.yml root
//! detection, and dbt task templates.

use std::{ffi::OsStr, path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{App, AsyncApp, SharedString};
use http_client::github::{GitHubLspBinaryVersion, latest_github_release};
use language::{
    LanguageRegistry, LspAdapter, LspAdapterDelegate, LspInstaller, ManifestName,
    ManifestProvider, ManifestQuery, Toolchain,
};
use lsp::{LanguageServerBinary, LanguageServerName};
use project::ContextProviderWithTasks;
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{ResultExt, fs::remove_matching, maybe, rel_path::RelPath};

const GO_LSP_REPO: &str = "j-clemons/dbt-language-server";
const GO_LSP_BINARY: &str = "dbt-language-server";

pub(crate) struct DbtLspAdapter;

impl DbtLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("dbt-lsp");

    fn github_asset_name() -> Result<String> {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            other => anyhow::bail!(
                "no prebuilt {GO_LSP_BINARY} for {other}; install it manually (e.g. `go install github.com/{GO_LSP_REPO}@latest`)"
            ),
        };
        let arch = match std::env::consts::ARCH {
            "aarch64" => "arm64",
            "x86_64" => "amd64",
            other => anyhow::bail!(
                "no prebuilt {GO_LSP_BINARY} for {other}; install it manually (e.g. `go install github.com/{GO_LSP_REPO}@latest`)"
            ),
        };
        Ok(format!("{GO_LSP_BINARY}-{os}-{arch}"))
    }
}

impl LspInstaller for DbtLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        // Historic standalone Fusion LSP binary.
        if let Some(path) = delegate.which(OsStr::new("dbt-lsp")).await {
            return Some(LanguageServerBinary {
                path,
                arguments: Vec::new(),
                env: None,
            });
        }
        // dbt Fusion CLI ships the language server as the `lsp` subcommand.
        // NOTE: a dbt Core (Python) `dbt` on PATH has no `lsp` subcommand; users
        // in that situation should point `lsp.dbt-lsp.binary.path` at the Go
        // language server or remove Core's `dbt` from PATH.
        if let Some(path) = delegate.which(OsStr::new("dbt")).await {
            return Some(LanguageServerBinary {
                path,
                arguments: vec!["lsp".into()],
                env: None,
            });
        }
        // Community Go language server.
        let path = delegate.which(OsStr::new(GO_LSP_BINARY)).await?;
        Some(LanguageServerBinary {
            path,
            arguments: Vec::new(),
            env: None,
        })
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _pre_release: bool,
        _cx: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release =
            latest_github_release(GO_LSP_REPO, true, false, delegate.http_client()).await?;
        let asset_name = Self::github_asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| {
                format!(
                    "no asset {asset_name} in {GO_LSP_REPO} release {}",
                    release.tag_name
                )
            })?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
            digest: asset.digest.clone(),
        })
    }

    fn fetch_server_binary(
        &self,
        version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>> + use<> {
        let delegate = delegate.clone();
        async move {
            let destination_path =
                container_dir.join(format!("{GO_LSP_BINARY}-{}", version.name));

            if smol::fs::metadata(&destination_path).await.is_err() {
                let mut response = delegate
                    .http_client()
                    .get(&version.url, Default::default(), true)
                    .await
                    .context("downloading dbt-language-server release asset")?;
                anyhow::ensure!(
                    response.status().is_success(),
                    "downloading dbt-language-server failed with status {}",
                    response.status()
                );
                let mut file = smol::fs::File::create(&destination_path).await?;
                futures::io::copy(response.body_mut(), &mut file).await?;

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt as _;
                    smol::fs::set_permissions(
                        &destination_path,
                        std::fs::Permissions::from_mode(0o755),
                    )
                    .await?;
                }

                remove_matching(&container_dir, |path| path != destination_path).await;
            }

            Ok(LanguageServerBinary {
                path: destination_path,
                arguments: Vec::new(),
                env: None,
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let mut latest = None;
            let mut entries = smol::fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let path = entry?.path();
                if path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .is_some_and(|name| name.starts_with(GO_LSP_BINARY))
                {
                    latest = Some(path);
                }
            }
            latest
                .map(|path| LanguageServerBinary {
                    path,
                    arguments: Vec::new(),
                    env: None,
                })
                .context("no cached dbt-language-server binary")
        })
        .await
        .log_err()
    }
}

#[async_trait(?Send)]
impl LspAdapter for DbtLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

pub(crate) struct DbtProjectManifestProvider;

impl ManifestProvider for DbtProjectManifestProvider {
    fn name(&self) -> ManifestName {
        SharedString::new_static("dbt_project.yml").into()
    }

    fn search(
        &self,
        ManifestQuery {
            path,
            depth,
            delegate,
        }: ManifestQuery,
    ) -> Option<Arc<RelPath>> {
        let mut outermost_dbt_project = None;
        for path in path.ancestors().take(depth) {
            let p = path.join(RelPath::from_unix_str("dbt_project.yml").unwrap());
            if delegate.exists(&p, Some(false)) {
                outermost_dbt_project = Some(Arc::from(path));
            }
        }

        outermost_dbt_project
    }
}

fn dbt_task_context() -> ContextProviderWithTasks {
    let model = VariableName::Stem.template_value();
    let worktree = VariableName::WorktreeRoot.template_value();
    let dbt_task = |label: String, args: Vec<String>, tags: Vec<String>| TaskTemplate {
        label,
        command: "dbt".to_owned(),
        args,
        cwd: Some(worktree.clone()),
        tags,
        ..TaskTemplate::default()
    };
    ContextProviderWithTasks::new(TaskTemplates(vec![
        dbt_task(
            format!("dbt run {model}"),
            vec!["run".into(), "--select".into(), model.clone()],
            vec!["dbt-model".to_owned()],
        ),
        dbt_task(
            format!("dbt test {model}"),
            vec!["test".into(), "--select".into(), model.clone()],
            vec!["dbt-model".to_owned()],
        ),
        dbt_task(
            format!("dbt build {model}+"),
            vec!["build".into(), "--select".into(), format!("{model}+")],
            vec!["dbt-model".to_owned()],
        ),
        dbt_task(
            format!("dbt compile {model}"),
            vec!["compile".into(), "--select".into(), model.clone()],
            vec!["dbt-model".to_owned()],
        ),
        dbt_task(
            format!("dbt show {model}"),
            vec![
                "show".into(),
                "--select".into(),
                model.clone(),
                "--limit".into(),
                "50".into(),
            ],
            vec!["dbt-model".to_owned()],
        ),
        dbt_task("dbt run".to_owned(), vec!["run".into()], Vec::new()),
        dbt_task("dbt test".to_owned(), vec!["test".into()], Vec::new()),
        dbt_task("dbt build".to_owned(), vec!["build".into()], Vec::new()),
        dbt_task("dbt parse".to_owned(), vec!["parse".into()], Vec::new()),
        dbt_task(
            "dbt docs generate".to_owned(),
            vec!["docs".into(), "generate".into()],
            Vec::new(),
        ),
    ]))
}

pub(crate) fn register(languages: &LanguageRegistry, cx: &mut App) {
    project::ManifestProvidersStore::global(cx).register(Arc::from(DbtProjectManifestProvider));
    crate::register_language(
        languages,
        "dbt",
        vec![Arc::new(DbtLspAdapter)],
        Some(Arc::new(dbt_task_context())),
        None,
        Some(SharedString::new_static("dbt_project.yml").into()),
        None,
        cx,
    );
    crate::register_language(languages, "sql", Vec::new(), None, None, None, None, cx);
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};

    #[test]
    fn test_dbt_configs_and_queries_load() {
        crate::language("dbt", tree_sitter_jinja2::LANGUAGE.into());
        crate::language("sql", tree_sitter_sequel::LANGUAGE.into());
    }

    #[gpui::test]
    fn test_dbt_model_is_runnable(cx: &mut TestAppContext) {
        let language = crate::language("dbt", tree_sitter_jinja2::LANGUAGE.into());
        let model = "{{ config(materialized='table') }}\n\nselect 1 as id\n";

        let buffer =
            cx.new(|cx| crate::Buffer::local(model, cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.runnable_ranges(0..model.len()).collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();
        assert!(
            tag_strings.contains(&"dbt-model".to_string()),
            "expected a dbt-model runnable tag, found: {:?}",
            tag_strings
        );
    }
}
