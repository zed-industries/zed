use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gpui::{AsyncAppContext, Task};
use language::{CodeLabel, Language};
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use util::github::latest_github_release;

pub struct JuliaLspAdapter;

const PROJECT_FILE: &'static str = "Project.toml";

#[async_trait]
impl LspAdapter for JuliaLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("LanguageServer.jl".into())
    }

    fn short_name(&self) -> &'static str {
        "julials"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release(
            "julia-vscode/LanguageServer.jl",
            false,
            false,
            delegate.http_client(),
        )
        .await?;
        let version = release.tag_name;
        return Ok(Box::new(version) as Box<_>);
    }

    fn will_fetch_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        static DID_SHOW_NOTIFICATION: AtomicBool = AtomicBool::new(false);

        const NOTIFICATION_MESSAGE: &str = "Could not run the julia language server, `LanguageServer.jl`, because `julia` was not found.";

        let delegate = delegate.clone();
        Some(cx.spawn(|cx| async move {
            let julia_output = smol::process::Command::new("julia")
                .args(["--version"])
                .output()
                .await;
            if julia_output.is_err() {
                if DID_SHOW_NOTIFICATION
                    .compare_exchange(false, true, SeqCst, SeqCst)
                    .is_ok()
                {
                    cx.update(|cx| {
                        delegate.show_notification(NOTIFICATION_MESSAGE, cx);
                    })?
                }
                return Err(anyhow!("cannot install LanguageServer.jl"));
            }

            Ok(())
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let Ok(Some(version)) = version
            .downcast::<String>()
            .map(|v| (*v).to_owned().strip_prefix('v').map(|s| s.to_owned()))
        else {
            return Err(anyhow!("could not get version for LanguageServer.jl"));
        };

        if version.chars().any(|c| !(c.is_numeric() || c == '.')) {
            return Err(anyhow!("malformed LanguageServer.jl version '{version}'"));
        }

        if fs::metadata(&container_dir.join(PROJECT_FILE))
            .await
            .is_err()
        {
            let Some(project_dir) = container_dir.to_str() else {
                return Err(anyhow!("cannot install LanguageServer.jl"));
            };

            let julia_output = smol::process::Command::new("julia")
            .args([
                "--history-file=no",
                "--startup-file=no",
                &format!("--project={project_dir}"),
                "-e",
                &format!("import Pkg; Pkg.add(Pkg.PackageSpec(; name=\"LanguageServer\", version=v\"{version}\"))"),
            ])
            .output()
            .await;

            if julia_output.is_err() {
                return Err(anyhow!("cannot install LanguageServer.jl"));
            };
        }

        Ok(get_lsp_binary(container_dir, Some(&version), None))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        if fs::metadata(&container_dir.join(PROJECT_FILE))
            .await
            .is_err()
        {
            return None;
        }

        return Some(get_lsp_binary(container_dir, None, None));
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        // Just load LanguageServer to make sure it is installed.
        Some(get_lsp_binary(
            container_dir,
            None,
            Some("import LanguageServer".into()),
        ))
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind {
            Some(lsp::CompletionItemKind::UNIT) if completion.label.starts_with('\\') => {
                let text = &completion.label;
                let filter_range = if text.starts_with("\\:") && text.ends_with(":") {
                    // Completions such as \:pizza:
                    2..text.len() - 1
                } else {
                    // Unicode completions such as \lambda
                    1..text.len()
                };
                Some(CodeLabel {
                    text: text.clone(),
                    runs: Default::default(),
                    filter_range,
                })
            }
            Some(kind) => {
                let highlight_name = match kind {
                    lsp::CompletionItemKind::STRUCT | lsp::CompletionItemKind::ENUM => Some("type"),
                    lsp::CompletionItemKind::KEYWORD => Some("keyword"),
                    lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
                        Some("constant")
                    }
                    _ => None,
                };
                let highlight_id = language.grammar()?.highlight_id_for_name(highlight_name?)?;
                let mut label = CodeLabel::plain(completion.label.clone(), None);
                label.runs.push((0..label.text.len(), highlight_id));
                Some(label)
            }
            _ => None,
        }
    }
}

fn get_lsp_binary(
    container_dir: PathBuf,
    version: Option<&str>,
    julia_code: Option<OsString>,
) -> LanguageServerBinary {
    let mut project_cmd = OsString::from("--project=");
    project_cmd.push(container_dir.as_os_str());

    let julia_code = julia_code.unwrap_or_else(|| {
        format!(
            "
                import LanguageServer, Pkg

                # if the current version of LanguageServer.jl is not the latest one, update it in the current environment.
                # this will only be effective on the next launch.

                version = v\"{}\"
                if pkgversion(LanguageServer) != version
                    Pkg.update(Pkg.PackageSpec(; name=\"LanguageServer\", version=version))
                end

                LanguageServer.runserver()
                ",
            version.unwrap_or("0.0.0")
        )
        .into()
    });

    LanguageServerBinary {
        path: "julia".into(),
        arguments: vec![
            "--history-file=no".into(),
            "--startup-file=no".into(),
            project_cmd,
            "-e".into(),
            julia_code.into(),
        ],
        env: None,
    }
}
