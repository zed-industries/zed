use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{AsyncAppContext, Task};
use http::github::latest_github_release;
pub use language::*;
use lazy_static::lazy_static;
use lsp::LanguageServerBinary;
use project::project_settings::{BinarySettings, ProjectSettings};
use regex::Regex;
use serde_json::json;
use settings::Settings;
use smol::{fs, process};
use std::{
    any::Any,
    borrow::Cow,
    ffi::{OsStr, OsString},
    ops::Range,
    path::PathBuf,
    str,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};
use util::{fs::remove_matching, maybe, ResultExt};

fn server_binary_arguments() -> Vec<OsString> {
    vec!["-mode=stdio".into()]
}

#[derive(Copy, Clone)]
pub struct GoLspAdapter;

impl GoLspAdapter {
    const SERVER_NAME: &'static str = "gopls";
}

lazy_static! {
    static ref GOPLS_VERSION_REGEX: Regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    static ref GO_EXTRACT_SUBTEST_NAME_REGEX: Regex =
        Regex::new(r#".*t\.Run\("([^"]*)".*"#).unwrap();
    static ref GO_ESCAPE_SUBTEST_NAME_REGEX: Regex = Regex::new(r#"[.*+?^${}()|\[\]\\]"#).unwrap();
}

#[async_trait(?Send)]
impl super::LspAdapter for GoLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("golang/tools", false, false, delegate.http_client()).await?;
        let version: Option<String> = release.tag_name.strip_prefix("gopls/v").map(str::to_string);
        if version.is_none() {
            log::warn!(
                "couldn't infer gopls version from GitHub release tag name '{}'",
                release.tag_name
            );
        }
        Ok(Box::new(version) as Box<_>)
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        cx: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        let configured_binary = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(Self::SERVER_NAME)
                .and_then(|s| s.binary.clone())
        });

        if let Ok(Some(BinarySettings {
            path: Some(path),
            arguments,
        })) = configured_binary
        {
            Some(LanguageServerBinary {
                path: path.into(),
                arguments: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.into())
                    .collect(),
                env: None,
            })
        } else {
            let env = delegate.shell_env().await;
            let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
            Some(LanguageServerBinary {
                path,
                arguments: server_binary_arguments(),
                env: Some(env),
            })
        }
    }

    fn will_fetch_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        static DID_SHOW_NOTIFICATION: AtomicBool = AtomicBool::new(false);

        const NOTIFICATION_MESSAGE: &str =
            "Could not install the Go language server `gopls`, because `go` was not found.";

        let delegate = delegate.clone();
        Some(cx.spawn(|cx| async move {
            let install_output = process::Command::new("go").args(["version"]).output().await;
            if install_output.is_err() {
                if DID_SHOW_NOTIFICATION
                    .compare_exchange(false, true, SeqCst, SeqCst)
                    .is_ok()
                {
                    cx.update(|cx| {
                        delegate.show_notification(NOTIFICATION_MESSAGE, cx);
                    })?
                }
                return Err(anyhow!("cannot install gopls"));
            }
            Ok(())
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<Option<String>>().unwrap();
        let this = *self;

        if let Some(version) = *version {
            let binary_path = container_dir.join(&format!("gopls_{version}"));
            if let Ok(metadata) = fs::metadata(&binary_path).await {
                if metadata.is_file() {
                    remove_matching(&container_dir, |entry| {
                        entry != binary_path && entry.file_name() != Some(OsStr::new("gobin"))
                    })
                    .await;

                    return Ok(LanguageServerBinary {
                        path: binary_path.to_path_buf(),
                        arguments: server_binary_arguments(),
                        env: None,
                    });
                }
            }
        } else if let Some(path) = this
            .cached_server_binary(container_dir.clone(), delegate)
            .await
        {
            return Ok(path);
        }

        let gobin_dir = container_dir.join("gobin");
        fs::create_dir_all(&gobin_dir).await?;
        let install_output = process::Command::new("go")
            .env("GO111MODULE", "on")
            .env("GOBIN", &gobin_dir)
            .args(["install", "golang.org/x/tools/gopls@latest"])
            .output()
            .await?;

        if !install_output.status.success() {
            log::error!(
                "failed to install gopls via `go install`. stdout: {:?}, stderr: {:?}",
                String::from_utf8_lossy(&install_output.stdout),
                String::from_utf8_lossy(&install_output.stderr)
            );

            return Err(anyhow!("failed to install gopls with `go install`. Is `go` installed and in the PATH? Check logs for more information."));
        }

        let installed_binary_path = gobin_dir.join("gopls");
        let version_output = process::Command::new(&installed_binary_path)
            .arg("version")
            .output()
            .await
            .context("failed to run installed gopls binary")?;
        let version_stdout = str::from_utf8(&version_output.stdout)
            .context("gopls version produced invalid utf8 output")?;
        let version = GOPLS_VERSION_REGEX
            .find(version_stdout)
            .with_context(|| format!("failed to parse golps version output '{version_stdout}'"))?
            .as_str();
        let binary_path = container_dir.join(&format!("gopls_{version}"));
        fs::rename(&installed_binary_path, &binary_path).await?;

        Ok(LanguageServerBinary {
            path: binary_path.to_path_buf(),
            arguments: server_binary_arguments(),
            env: None,
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "usePlaceholders": true,
            "hints": {
                "assignVariableTypes": true,
                "compositeLiteralFields": true,
                "compositeLiteralTypes": true,
                "constantValues": true,
                "functionTypeParameters": true,
                "parameterNames": true,
                "rangeVariableTypes": true
            }
        })))
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let label = &completion.label;

        // Gopls returns nested fields and methods as completions.
        // To syntax highlight these, combine their final component
        // with their detail.
        let name_offset = label.rfind('.').unwrap_or(0);

        match completion.kind.zip(completion.detail.as_ref()) {
            Some((lsp::CompletionItemKind::MODULE, detail)) => {
                let text = format!("{label} {detail}");
                let source = Rope::from(format!("import {text}").as_str());
                let runs = language.highlight_text(&source, 7..7 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((
                lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE,
                detail,
            )) => {
                let text = format!("{label} {detail}");
                let source =
                    Rope::from(format!("var {} {}", &text[name_offset..], detail).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 4..4 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::STRUCT, _)) => {
                let text = format!("{label} struct {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::INTERFACE, _)) => {
                let text = format!("{label} interface {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::FIELD, detail)) => {
                let text = format!("{label} {detail}");
                let source =
                    Rope::from(format!("type T struct {{ {} }}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 16..16 + text.len()),
                );
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..label.len(),
                });
            }
            Some((lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD, detail)) => {
                if let Some(signature) = detail.strip_prefix("func") {
                    let text = format!("{label}{signature}");
                    let source = Rope::from(format!("func {} {{}}", &text[name_offset..]).as_str());
                    let runs = adjust_runs(
                        name_offset,
                        language.highlight_text(&source, 5..5 + text.len()),
                    );
                    return Some(CodeLabel {
                        filter_range: 0..label.len(),
                        text,
                        runs,
                    });
                }
            }
            _ => {}
        }
        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("func {} () {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::STRUCT => {
                let text = format!("type {} struct {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..text.len();
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::INTERFACE => {
                let text = format!("type {} interface {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..text.len();
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("type {} T", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const {} = nil", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::VARIABLE => {
                let text = format!("var {} = nil", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("package {}", name);
                let filter_range = 8..8 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last_binary_path = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |name| name.starts_with("gopls_"))
            {
                last_binary_path = Some(entry.path());
            }
        }

        if let Some(path) = last_binary_path {
            Ok(LanguageServerBinary {
                path,
                arguments: server_binary_arguments(),
                env: None,
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}

fn adjust_runs(
    delta: usize,
    mut runs: Vec<(Range<usize>, HighlightId)>,
) -> Vec<(Range<usize>, HighlightId)> {
    for (range, _) in &mut runs {
        range.start += delta;
        range.end += delta;
    }
    runs
}

pub(crate) struct GoContextProvider;

const GO_PACKAGE_TASK_VARIABLE: VariableName = VariableName::Custom(Cow::Borrowed("GO_PACKAGE"));
const GO_SUBTEST_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("GO_SUBTEST_NAME"));

impl ContextProvider for GoContextProvider {
    fn build_context(
        &self,
        variables: &TaskVariables,
        location: &Location,
        cx: &mut gpui::AppContext,
    ) -> Result<TaskVariables> {
        let local_abs_path = location
            .buffer
            .read(cx)
            .file()
            .and_then(|file| Some(file.as_local()?.abs_path(cx)));

        let go_package_variable = local_abs_path
            .as_deref()
            .and_then(|local_abs_path| local_abs_path.parent())
            .map(|buffer_dir| {
                // Prefer the relative form `./my-nested-package/is-here` over
                // absolute path, because it's more readable in the modal, but
                // the absolute path also works.
                let package_name = variables
                    .get(&VariableName::WorktreeRoot)
                    .and_then(|worktree_abs_path| buffer_dir.strip_prefix(worktree_abs_path).ok())
                    .map(|relative_pkg_dir| {
                        if relative_pkg_dir.as_os_str().is_empty() {
                            ".".into()
                        } else {
                            format!("./{}", relative_pkg_dir.to_string_lossy())
                        }
                    })
                    .unwrap_or_else(|| format!("{}", buffer_dir.to_string_lossy()));

                (GO_PACKAGE_TASK_VARIABLE.clone(), package_name.to_string())
            });

        let _subtest_name = variables.get(&VariableName::Custom(Cow::Borrowed("_subtest_name")));

        let go_subtest_variable = extract_subtest_name(_subtest_name.unwrap_or(""))
            .map(|subtest_name| (GO_SUBTEST_NAME_TASK_VARIABLE.clone(), subtest_name));

        Ok(TaskVariables::from_iter(
            [go_package_variable, go_subtest_variable]
                .into_iter()
                .flatten(),
        ))
    }

    fn associated_tasks(&self) -> Option<TaskTemplates> {
        Some(TaskTemplates(vec![
            TaskTemplate {
                label: format!(
                    "go test {} -run {}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    "-run".into(),
                    VariableName::Symbol.template_value(),
                ],
                tags: vec!["go-test".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("go test {}", GO_PACKAGE_TASK_VARIABLE.template_value()),
                command: "go".into(),
                args: vec!["test".into(), GO_PACKAGE_TASK_VARIABLE.template_value()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "go test ./...".into(),
                command: "go".into(),
                args: vec!["test".into(), "./...".into()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "go test {} -run {}/{}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                    GO_SUBTEST_NAME_TASK_VARIABLE.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    "-v".into(),
                    "-run".into(),
                    format!(
                        "^{}$/^{}$",
                        VariableName::Symbol.template_value(),
                        GO_SUBTEST_NAME_TASK_VARIABLE.template_value(),
                    ),
                ],
                tags: vec!["go-subtest".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "go test {} -bench {}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value()
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    "-benchmem".into(),
                    "-run=^$".into(),
                    "-bench".into(),
                    format!("^{}$", VariableName::Symbol.template_value()),
                ],
                tags: vec!["go-benchmark".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("go run {}", GO_PACKAGE_TASK_VARIABLE.template_value(),),
                command: "go".into(),
                args: vec!["run".into(), GO_PACKAGE_TASK_VARIABLE.template_value()],
                tags: vec!["go-main".to_owned()],
                ..TaskTemplate::default()
            },
        ]))
    }
}

fn extract_subtest_name(input: &str) -> Option<String> {
    let replaced_spaces = input.trim_matches('"').replace(' ', "_");

    Some(
        GO_ESCAPE_SUBTEST_NAME_REGEX
            .replace_all(&replaced_spaces, |caps: &regex::Captures| {
                format!("\\{}", &caps[0])
            })
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language;
    use gpui::Hsla;
    use theme::SyntaxTheme;

    #[gpui::test]
    async fn test_go_label_for_completion() {
        let adapter = Arc::new(GoLspAdapter);
        let language = language("go", tree_sitter_go::language());

        let theme = SyntaxTheme::new_test([
            ("type", Hsla::default()),
            ("keyword", Hsla::default()),
            ("function", Hsla::default()),
            ("number", Hsla::default()),
            ("property", Hsla::default()),
        ]);
        language.set_theme(&theme);

        let grammar = language.grammar().unwrap();
        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();
        let highlight_number = grammar.highlight_id_for_name("number").unwrap();

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "Hello".to_string(),
                        detail: Some("func(a B) c.D".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "Hello(a B) c.D".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (8..9, highlight_type),
                    (13..14, highlight_type),
                ],
            })
        );

        // Nested methods
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::METHOD),
                        label: "one.two.Three".to_string(),
                        detail: Some("func() [3]interface{}".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "one.two.Three() [3]interface{}".to_string(),
                filter_range: 0..13,
                runs: vec![
                    (8..13, highlight_function),
                    (17..18, highlight_number),
                    (19..28, highlight_keyword),
                ],
            })
        );

        // Nested fields
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FIELD),
                        label: "two.Three".to_string(),
                        detail: Some("a.Bcd".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "two.Three a.Bcd".to_string(),
                filter_range: 0..9,
                runs: vec![(12..15, highlight_type)],
            })
        );
    }
}
