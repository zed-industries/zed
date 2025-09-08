use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use gpui::{App, AsyncApp, Task};
use http_client::github::latest_github_release;
pub use language::*;
use lsp::{LanguageServerBinary, LanguageServerName};
use project::Fs;
use regex::Regex;
use serde_json::json;
use smol::fs;
use std::{
    any::Any,
    borrow::Cow,
    ffi::{OsStr, OsString},
    ops::Range,
    path::PathBuf,
    process::Output,
    str,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};
use util::{ResultExt, fs::remove_matching, maybe};

fn server_binary_arguments() -> Vec<OsString> {
    vec!["-mode=stdio".into()]
}

#[derive(Copy, Clone)]
pub struct GoLspAdapter;

impl GoLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("gopls");
}

static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+\.\d+\.\d+").expect("Failed to create VERSION_REGEX"));

static GO_ESCAPE_SUBTEST_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[.*+?^${}()|\[\]\\"']"#).expect("Failed to create GO_ESCAPE_SUBTEST_NAME_REGEX")
});

const BINARY: &str = if cfg!(target_os = "windows") {
    "gopls.exe"
} else {
    "gopls"
};

#[async_trait(?Send)]
impl super::LspAdapter for GoLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: &AsyncApp,
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
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            arguments: server_binary_arguments(),
            env: None,
        })
    }

    fn will_fetch_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncApp,
    ) -> Option<Task<Result<()>>> {
        static DID_SHOW_NOTIFICATION: AtomicBool = AtomicBool::new(false);

        const NOTIFICATION_MESSAGE: &str =
            "Could not install the Go language server `gopls`, because `go` was not found.";

        let delegate = delegate.clone();
        Some(cx.spawn(async move |cx| {
            if delegate.which("go".as_ref()).await.is_none() {
                if DID_SHOW_NOTIFICATION
                    .compare_exchange(false, true, SeqCst, SeqCst)
                    .is_ok()
                {
                    cx.update(|cx| {
                        delegate.show_notification(NOTIFICATION_MESSAGE, cx);
                    })?
                }
                anyhow::bail!("cannot install gopls");
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
        let go = delegate.which("go".as_ref()).await.unwrap_or("go".into());
        let go_version_output = util::command::new_smol_command(&go)
            .args(["version"])
            .output()
            .await
            .context("failed to get go version via `go version` command`")?;
        let go_version = parse_version_output(&go_version_output)?;
        let version = version.downcast::<Option<String>>().unwrap();
        let this = *self;

        if let Some(version) = *version {
            let binary_path = container_dir.join(format!("gopls_{version}_go_{go_version}"));
            if let Ok(metadata) = fs::metadata(&binary_path).await
                && metadata.is_file()
            {
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
        } else if let Some(path) = this
            .cached_server_binary(container_dir.clone(), delegate)
            .await
        {
            return Ok(path);
        }

        let gobin_dir = container_dir.join("gobin");
        fs::create_dir_all(&gobin_dir).await?;
        let install_output = util::command::new_smol_command(go)
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
            anyhow::bail!(
                "failed to install gopls with `go install`. Is `go` installed and in the PATH? Check logs for more information."
            );
        }

        let installed_binary_path = gobin_dir.join(BINARY);
        let version_output = util::command::new_smol_command(&installed_binary_path)
            .arg("version")
            .output()
            .await
            .context("failed to run installed gopls binary")?;
        let gopls_version = parse_version_output(&version_output)?;
        let binary_path = container_dir.join(format!("gopls_{gopls_version}_go_{go_version}"));
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

    async fn initialization_options(
        self: Arc<Self>,
        _: &dyn Fs,
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
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(0..label.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range,
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
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(0..label.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range,
                });
            }
            Some((lsp::CompletionItemKind::STRUCT, _)) => {
                let text = format!("{label} struct {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(0..label.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range,
                });
            }
            Some((lsp::CompletionItemKind::INTERFACE, _)) => {
                let text = format!("{label} interface {{}}");
                let source = Rope::from(format!("type {}", &text[name_offset..]).as_str());
                let runs = adjust_runs(
                    name_offset,
                    language.highlight_text(&source, 5..5 + text.len()),
                );
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(0..label.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range,
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
                let filter_range = completion
                    .filter_text
                    .as_deref()
                    .and_then(|filter_text| {
                        text.find(filter_text)
                            .map(|start| start..start + filter_text.len())
                    })
                    .unwrap_or(0..label.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range,
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
                    let filter_range = completion
                        .filter_text
                        .as_deref()
                        .and_then(|filter_text| {
                            text.find(filter_text)
                                .map(|start| start..start + filter_text.len())
                        })
                        .unwrap_or(0..label.len());
                    return Some(CodeLabel {
                        filter_range,
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

    fn diagnostic_message_to_markdown(&self, message: &str) -> Option<String> {
        static REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?m)\n\s*").expect("Failed to create REGEX"));
        Some(REGEX.replace_all(message, "\n\n").to_string())
    }
}

fn parse_version_output(output: &Output) -> Result<&str> {
    let version_stdout =
        str::from_utf8(&output.stdout).context("version command produced invalid utf8 output")?;

    let version = VERSION_REGEX
        .find(version_stdout)
        .with_context(|| format!("failed to parse version output '{version_stdout}'"))?
        .as_str();

    Ok(version)
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
                    .is_some_and(|name| name.starts_with("gopls_"))
            {
                last_binary_path = Some(entry.path());
            }
        }

        let path = last_binary_path.context("no cached binary")?;
        anyhow::Ok(LanguageServerBinary {
            path,
            arguments: server_binary_arguments(),
            env: None,
        })
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
const GO_MODULE_ROOT_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("GO_MODULE_ROOT"));
const GO_SUBTEST_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("GO_SUBTEST_NAME"));
const GO_TABLE_TEST_CASE_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("GO_TABLE_TEST_CASE_NAME"));

impl ContextProvider for GoContextProvider {
    fn build_context(
        &self,
        variables: &TaskVariables,
        location: ContextLocation<'_>,
        _: Option<HashMap<String, String>>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut gpui::App,
    ) -> Task<Result<TaskVariables>> {
        let local_abs_path = location
            .file_location
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

                (GO_PACKAGE_TASK_VARIABLE.clone(), package_name)
            });

        let go_module_root_variable = local_abs_path
            .as_deref()
            .and_then(|local_abs_path| local_abs_path.parent())
            .map(|buffer_dir| {
                // Walk dirtree up until getting the first go.mod file
                let module_dir = buffer_dir
                    .ancestors()
                    .find(|dir| dir.join("go.mod").is_file())
                    .map(|dir| dir.to_string_lossy().to_string())
                    .unwrap_or_else(|| ".".to_string());

                (GO_MODULE_ROOT_TASK_VARIABLE.clone(), module_dir)
            });

        let _subtest_name = variables.get(&VariableName::Custom(Cow::Borrowed("_subtest_name")));

        let go_subtest_variable = extract_subtest_name(_subtest_name.unwrap_or(""))
            .map(|subtest_name| (GO_SUBTEST_NAME_TASK_VARIABLE.clone(), subtest_name));

        let table_test_case_name = variables.get(&VariableName::Custom(Cow::Borrowed(
            "_table_test_case_name",
        )));

        let go_table_test_case_variable = table_test_case_name
            .and_then(extract_subtest_name)
            .map(|case_name| (GO_TABLE_TEST_CASE_NAME_TASK_VARIABLE.clone(), case_name));

        Task::ready(Ok(TaskVariables::from_iter(
            [
                go_package_variable,
                go_subtest_variable,
                go_table_test_case_variable,
                go_module_root_variable,
            ]
            .into_iter()
            .flatten(),
        )))
    }

    fn associated_tasks(
        &self,
        _: Arc<dyn Fs>,
        _: Option<Arc<dyn File>>,
        _: &App,
    ) -> Task<Option<TaskTemplates>> {
        let package_cwd = if GO_PACKAGE_TASK_VARIABLE.template_value() == "." {
            None
        } else {
            Some("$ZED_DIRNAME".to_string())
        };
        let module_cwd = Some(GO_MODULE_ROOT_TASK_VARIABLE.template_value());

        Task::ready(Some(TaskTemplates(vec![
            TaskTemplate {
                label: format!(
                    "go test {} -v -run {}/{}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                    GO_TABLE_TEST_CASE_NAME_TASK_VARIABLE.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    "-v".into(),
                    "-run".into(),
                    format!(
                        "\\^{}\\$/\\^{}\\$",
                        VariableName::Symbol.template_value(),
                        GO_TABLE_TEST_CASE_NAME_TASK_VARIABLE.template_value(),
                    ),
                ],
                cwd: package_cwd.clone(),
                tags: vec!["go-table-test-case".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "go test {} -run {}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    "-run".into(),
                    format!("\\^{}\\$", VariableName::Symbol.template_value(),),
                ],
                tags: vec!["go-test".to_owned()],
                cwd: package_cwd.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("go test {}", GO_PACKAGE_TASK_VARIABLE.template_value()),
                command: "go".into(),
                args: vec!["test".into()],
                cwd: package_cwd.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "go test ./...".into(),
                command: "go".into(),
                args: vec!["test".into(), "./...".into()],
                cwd: module_cwd.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "go test {} -v -run {}/{}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                    GO_SUBTEST_NAME_TASK_VARIABLE.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    "-v".into(),
                    "-run".into(),
                    format!(
                        "\\^{}\\$/\\^{}\\$",
                        VariableName::Symbol.template_value(),
                        GO_SUBTEST_NAME_TASK_VARIABLE.template_value(),
                    ),
                ],
                cwd: package_cwd.clone(),
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
                    "-benchmem".into(),
                    "-run='^$'".into(),
                    "-bench".into(),
                    format!("\\^{}\\$", VariableName::Symbol.template_value()),
                ],
                cwd: package_cwd.clone(),
                tags: vec!["go-benchmark".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "go test {} -fuzz=Fuzz -run {}",
                    GO_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                ),
                command: "go".into(),
                args: vec![
                    "test".into(),
                    "-fuzz=Fuzz".into(),
                    "-run".into(),
                    format!("\\^{}\\$", VariableName::Symbol.template_value(),),
                ],
                tags: vec!["go-fuzz".to_owned()],
                cwd: package_cwd.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("go run {}", GO_PACKAGE_TASK_VARIABLE.template_value(),),
                command: "go".into(),
                args: vec!["run".into(), ".".into()],
                cwd: package_cwd.clone(),
                tags: vec!["go-main".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("go generate {}", GO_PACKAGE_TASK_VARIABLE.template_value()),
                command: "go".into(),
                args: vec!["generate".into()],
                cwd: package_cwd,
                tags: vec!["go-generate".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "go generate ./...".into(),
                command: "go".into(),
                args: vec!["generate".into(), "./...".into()],
                cwd: module_cwd,
                ..TaskTemplate::default()
            },
        ])))
    }
}

fn extract_subtest_name(input: &str) -> Option<String> {
    let content = if input.starts_with('`') && input.ends_with('`') {
        input.trim_matches('`')
    } else {
        input.trim_matches('"')
    };

    let processed = content
        .chars()
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect::<String>();

    Some(
        GO_ESCAPE_SUBTEST_NAME_REGEX
            .replace_all(&processed, |caps: &regex::Captures| {
                format!("\\{}", &caps[0])
            })
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language;
    use gpui::{AppContext, Hsla, TestAppContext};
    use theme::SyntaxTheme;

    #[gpui::test]
    async fn test_go_label_for_completion() {
        let adapter = Arc::new(GoLspAdapter);
        let language = language("go", tree_sitter_go::LANGUAGE.into());

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
        let highlight_field = grammar.highlight_id_for_name("property").unwrap();

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
                runs: vec![(4..9, highlight_field), (12..15, highlight_type)],
            })
        );
    }

    #[gpui::test]
    fn test_go_runnable_detection(cx: &mut TestAppContext) {
        let language = language("go", tree_sitter_go::LANGUAGE.into());

        let interpreted_string_subtest = r#"
        package main

        import "testing"

        func TestExample(t *testing.T) {
            t.Run("subtest with double quotes", func(t *testing.T) {
                // test code
            })
        }
        "#;

        let raw_string_subtest = r#"
        package main

        import "testing"

        func TestExample(t *testing.T) {
            t.Run(`subtest with
            multiline
            backticks`, func(t *testing.T) {
                // test code
            })
        }
        "#;

        let buffer = cx.new(|cx| {
            crate::Buffer::local(interpreted_string_subtest, cx).with_language(language.clone(), cx)
        });
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot
                .runnable_ranges(0..interpreted_string_subtest.len())
                .collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            tag_strings.contains(&"go-subtest".to_string()),
            "Should find go-subtest tag, found: {:?}",
            tag_strings
        );

        let buffer = cx.new(|cx| {
            crate::Buffer::local(raw_string_subtest, cx).with_language(language.clone(), cx)
        });
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot
                .runnable_ranges(0..raw_string_subtest.len())
                .collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            tag_strings.contains(&"go-subtest".to_string()),
            "Should find go-subtest tag, found: {:?}",
            tag_strings
        );
    }

    #[gpui::test]
    fn test_go_table_test_slice_detection(cx: &mut TestAppContext) {
        let language = language("go", tree_sitter_go::LANGUAGE.into());

        let table_test = r#"
        package main

        import "testing"

        func TestExample(t *testing.T) {
            _ = "some random string"

            testCases := []struct{
                name string
                anotherStr string
            }{
                {
                    name: "test case 1",
                    anotherStr: "foo",
                },
                {
                    name: "test case 2",
                    anotherStr: "bar",
                },
            }

            notATableTest := []struct{
                name string
            }{
                {
                    name: "some string",
                },
                {
                    name: "some other string",
                },
            }

            for _, tc := range testCases {
                t.Run(tc.name, func(t *testing.T) {
                    // test code here
                })
            }
        }
        "#;

        let buffer =
            cx.new(|cx| crate::Buffer::local(table_test, cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.runnable_ranges(0..table_test.len()).collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            tag_strings.contains(&"go-table-test-case".to_string()),
            "Should find go-table-test-case tag, found: {:?}",
            tag_strings
        );

        let go_test_count = tag_strings.iter().filter(|&tag| tag == "go-test").count();
        let go_table_test_count = tag_strings
            .iter()
            .filter(|&tag| tag == "go-table-test-case")
            .count();

        assert!(
            go_test_count == 1,
            "Should find exactly 1 go-test, found: {}",
            go_test_count
        );
        assert!(
            go_table_test_count == 2,
            "Should find exactly 2 go-table-test-case, found: {}",
            go_table_test_count
        );
    }

    #[gpui::test]
    fn test_go_table_test_slice_ignored(cx: &mut TestAppContext) {
        let language = language("go", tree_sitter_go::LANGUAGE.into());

        let table_test = r#"
        package main

        func Example() {
            _ = "some random string"

            notATableTest := []struct{
                name string
            }{
                {
                    name: "some string",
                },
                {
                    name: "some other string",
                },
            }
        }
        "#;

        let buffer =
            cx.new(|cx| crate::Buffer::local(table_test, cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.runnable_ranges(0..table_test.len()).collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            !tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            !tag_strings.contains(&"go-table-test-case".to_string()),
            "Should find go-table-test-case tag, found: {:?}",
            tag_strings
        );
    }

    #[gpui::test]
    fn test_go_table_test_map_detection(cx: &mut TestAppContext) {
        let language = language("go", tree_sitter_go::LANGUAGE.into());

        let table_test = r#"
        package main

        import "testing"

        func TestExample(t *testing.T) {
            _ = "some random string"

           	testCases := map[string]struct {
          		someStr string
          		fail    bool
           	}{
          		"test failure": {
         			someStr: "foo",
         			fail:    true,
          		},
          		"test success": {
         			someStr: "bar",
         			fail:    false,
          		},
           	}

           	notATableTest := map[string]struct {
          		someStr string
           	}{
          		"some string": {
         			someStr: "foo",
          		},
          		"some other string": {
         			someStr: "bar",
          		},
           	}

            for name, tc := range testCases {
                t.Run(name, func(t *testing.T) {
                    // test code here
                })
            }
        }
        "#;

        let buffer =
            cx.new(|cx| crate::Buffer::local(table_test, cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.runnable_ranges(0..table_test.len()).collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            tag_strings.contains(&"go-table-test-case".to_string()),
            "Should find go-table-test-case tag, found: {:?}",
            tag_strings
        );

        let go_test_count = tag_strings.iter().filter(|&tag| tag == "go-test").count();
        let go_table_test_count = tag_strings
            .iter()
            .filter(|&tag| tag == "go-table-test-case")
            .count();

        assert!(
            go_test_count == 1,
            "Should find exactly 1 go-test, found: {}",
            go_test_count
        );
        assert!(
            go_table_test_count == 2,
            "Should find exactly 2 go-table-test-case, found: {}",
            go_table_test_count
        );
    }

    #[gpui::test]
    fn test_go_table_test_map_ignored(cx: &mut TestAppContext) {
        let language = language("go", tree_sitter_go::LANGUAGE.into());

        let table_test = r#"
        package main

        func Example() {
            _ = "some random string"

           	notATableTest := map[string]struct {
          		someStr string
           	}{
          		"some string": {
         			someStr: "foo",
          		},
          		"some other string": {
         			someStr: "bar",
          		},
           	}
        }
        "#;

        let buffer =
            cx.new(|cx| crate::Buffer::local(table_test, cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.runnable_ranges(0..table_test.len()).collect()
        });

        let tag_strings: Vec<String> = runnables
            .iter()
            .flat_map(|r| &r.runnable.tags)
            .map(|tag| tag.0.to_string())
            .collect();

        assert!(
            !tag_strings.contains(&"go-test".to_string()),
            "Should find go-test tag, found: {:?}",
            tag_strings
        );
        assert!(
            !tag_strings.contains(&"go-table-test-case".to_string()),
            "Should find go-table-test-case tag, found: {:?}",
            tag_strings
        );
    }

    #[test]
    fn test_extract_subtest_name() {
        // Interpreted string literal
        let input_double_quoted = r#""subtest with double quotes""#;
        let result = extract_subtest_name(input_double_quoted);
        assert_eq!(result, Some(r#"subtest_with_double_quotes"#.to_string()));

        let input_double_quoted_with_backticks = r#""test with `backticks` inside""#;
        let result = extract_subtest_name(input_double_quoted_with_backticks);
        assert_eq!(result, Some(r#"test_with_`backticks`_inside"#.to_string()));

        // Raw string literal
        let input_with_backticks = r#"`subtest with backticks`"#;
        let result = extract_subtest_name(input_with_backticks);
        assert_eq!(result, Some(r#"subtest_with_backticks"#.to_string()));

        let input_raw_with_quotes = r#"`test with "quotes" and other chars`"#;
        let result = extract_subtest_name(input_raw_with_quotes);
        assert_eq!(
            result,
            Some(r#"test_with_\"quotes\"_and_other_chars"#.to_string())
        );

        let input_multiline = r#"`subtest with
        multiline
        backticks`"#;
        let result = extract_subtest_name(input_multiline);
        assert_eq!(
            result,
            Some(r#"subtest_with_________multiline_________backticks"#.to_string())
        );

        let input_with_double_quotes = r#"`test with "double quotes"`"#;
        let result = extract_subtest_name(input_with_double_quotes);
        assert_eq!(result, Some(r#"test_with_\"double_quotes\""#.to_string()));
    }
}
