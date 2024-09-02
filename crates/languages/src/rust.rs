use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
use gpui::{AppContext, AsyncAppContext};
use http_client::github::{latest_github_release, GitHubLspBinaryVersion};
pub use language::*;
use language_settings::all_language_settings;
use lsp::LanguageServerBinary;
use project::project_settings::{BinarySettings, ProjectSettings};
use regex::Regex;
use settings::Settings;
use smol::fs::{self, File};
use std::{
    any::Any,
    borrow::Cow,
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
    sync::LazyLock,
};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};
use util::{fs::remove_matching, maybe, ResultExt};

pub struct RustLspAdapter;

impl RustLspAdapter {
    const SERVER_NAME: &'static str = "rust-analyzer";
}

#[async_trait(?Send)]
impl LspAdapter for RustLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
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

        match configured_binary {
            Ok(Some(BinarySettings {
                path,
                arguments,
                path_lookup,
            })) => {
                let (path, env) = match (path, path_lookup) {
                    (Some(path), lookup) => {
                        if lookup.is_some() {
                            log::warn!(
                                "Both `path` and `path_lookup` are set, ignoring `path_lookup`"
                            );
                        }
                        (Some(path.into()), None)
                    }
                    (None, Some(true)) => {
                        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
                        let env = delegate.shell_env().await;
                        (Some(path), Some(env))
                    }
                    (None, Some(false)) | (None, None) => (None, None),
                };
                path.map(|path| LanguageServerBinary {
                    path,
                    arguments: arguments
                        .unwrap_or_default()
                        .iter()
                        .map(|arg| arg.into())
                        .collect(),
                    env,
                })
            }
            _ => None,
        }
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release(
            "rust-lang/rust-analyzer",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let os = match consts::OS {
            "macos" => "apple-darwin",
            "linux" => "unknown-linux-gnu",
            "windows" => "pc-windows-msvc",
            other => bail!("Running on unsupported os: {other}"),
        };
        let asset_name = format!("rust-analyzer-{}-{os}.gz", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!("rust-analyzer-{}", version.name));

        if fs::metadata(&destination_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = File::create(&destination_path).await?;
            futures::io::copy(decompressed_bytes, &mut file).await?;
            // todo("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &destination_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }

            remove_matching(&container_dir, |entry| entry != destination_path).await;
        }

        Ok(LanguageServerBinary {
            path: destination_path,
            env: None,
            arguments: Default::default(),
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

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec!["rustc".into()]
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        Some("rust-analyzer/flycheck".into())
    }

    fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
        static REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?m)`([^`]+)\n`$").expect("Failed to create REGEX"));

        for diagnostic in &mut params.diagnostics {
            for message in diagnostic
                .related_information
                .iter_mut()
                .flatten()
                .map(|info| &mut info.message)
                .chain([&mut diagnostic.message])
            {
                if let Cow::Owned(sanitized) = REGEX.replace_all(message, "`$1`") {
                    *message = sanitized;
                }
            }
        }
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let detail = completion
            .label_details
            .as_ref()
            .and_then(|detail| detail.detail.as_ref())
            .or(completion.detail.as_ref())
            .map(ToOwned::to_owned);
        let function_signature = completion
            .label_details
            .as_ref()
            .and_then(|detail| detail.description.as_ref())
            .or(completion.detail.as_ref())
            .map(ToOwned::to_owned);
        match completion.kind {
            Some(lsp::CompletionItemKind::FIELD) if detail.is_some() => {
                let name = &completion.label;
                let text = format!("{}: {}", name, detail.unwrap());
                let source = Rope::from(format!("struct S {{ {} }}", text).as_str());
                let runs = language.highlight_text(&source, 11..11 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                });
            }
            Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE)
                if detail.is_some()
                    && completion.insert_text_format != Some(lsp::InsertTextFormat::SNIPPET) =>
            {
                let name = &completion.label;
                let text = format!(
                    "{}: {}",
                    name,
                    completion.detail.as_ref().or(detail.as_ref()).unwrap()
                );
                let source = Rope::from(format!("let {} = ();", text).as_str());
                let runs = language.highlight_text(&source, 4..4 + text.len());
                return Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                });
            }
            Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD)
                if detail.is_some() =>
            {
                static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("\\(…?\\)").unwrap());

                let detail = detail.unwrap();
                const FUNCTION_PREFIXES: [&'static str; 6] = [
                    "async fn",
                    "async unsafe fn",
                    "const fn",
                    "const unsafe fn",
                    "unsafe fn",
                    "fn",
                ];
                // Is it function `async`?
                let fn_keyword = FUNCTION_PREFIXES.iter().find_map(|prefix| {
                    function_signature.as_ref().and_then(|signature| {
                        signature
                            .strip_prefix(*prefix)
                            .map(|suffix| (*prefix, suffix))
                    })
                });
                // fn keyword should be followed by opening parenthesis.
                if let Some((prefix, suffix)) = fn_keyword {
                    let mut text = REGEX.replace(&completion.label, suffix).to_string();
                    let source = Rope::from(format!("{prefix} {} {{}}", text).as_str());
                    let run_start = prefix.len() + 1;
                    let runs = language.highlight_text(&source, run_start..run_start + text.len());
                    if detail.starts_with(" (") {
                        text.push_str(&detail);
                    }

                    return Some(CodeLabel {
                        filter_range: 0..completion.label.find('(').unwrap_or(text.len()),
                        text,
                        runs,
                    });
                } else if completion
                    .detail
                    .as_ref()
                    .map_or(false, |detail| detail.starts_with("macro_rules! "))
                {
                    let source = Rope::from(completion.label.as_str());
                    let runs = language.highlight_text(&source, 0..completion.label.len());

                    return Some(CodeLabel {
                        filter_range: 0..completion.label.len(),
                        text: completion.label.clone(),
                        runs,
                    });
                }
            }
            Some(kind) => {
                let highlight_name = match kind {
                    lsp::CompletionItemKind::STRUCT
                    | lsp::CompletionItemKind::INTERFACE
                    | lsp::CompletionItemKind::ENUM => Some("type"),
                    lsp::CompletionItemKind::ENUM_MEMBER => Some("variant"),
                    lsp::CompletionItemKind::KEYWORD => Some("keyword"),
                    lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
                        Some("constant")
                    }
                    _ => None,
                };

                let mut label = completion.label.clone();
                if let Some(detail) = detail.filter(|detail| detail.starts_with(" (")) {
                    use std::fmt::Write;
                    write!(label, "{detail}").ok()?;
                }
                let mut label = CodeLabel::plain(label, None);
                if let Some(highlight_name) = highlight_name {
                    let highlight_id = language.grammar()?.highlight_id_for_name(highlight_name)?;
                    label.runs.push((
                        0..label.text.rfind('(').unwrap_or(completion.label.len()),
                        highlight_id,
                    ));
                }

                return Some(label);
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
                let text = format!("fn {} () {{}}", name);
                let filter_range = 3..3 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::STRUCT => {
                let text = format!("struct {} {{}}", name);
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::ENUM => {
                let text = format!("enum {} {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::INTERFACE => {
                let text = format!("trait {} {{}}", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const {}: () = ();", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("mod {} {{}}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::TYPE_PARAMETER => {
                let text = format!("type {} {{}}", name);
                let filter_range = 5..5 + name.len();
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

pub(crate) struct RustContextProvider;

const RUST_PACKAGE_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_PACKAGE"));

/// The bin name corresponding to the current file in Cargo.toml
const RUST_BIN_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_BIN_NAME"));

const RUST_MAIN_FUNCTION_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("_rust_main_function_end"));

impl ContextProvider for RustContextProvider {
    fn build_context(
        &self,
        task_variables: &TaskVariables,
        location: &Location,
        cx: &mut gpui::AppContext,
    ) -> Result<TaskVariables> {
        let local_abs_path = location
            .buffer
            .read(cx)
            .file()
            .and_then(|file| Some(file.as_local()?.abs_path(cx)));

        let local_abs_path = local_abs_path.as_deref();

        let is_main_function = task_variables
            .get(&RUST_MAIN_FUNCTION_TASK_VARIABLE)
            .is_some();

        if is_main_function {
            if let Some((package_name, bin_name)) = local_abs_path
                .and_then(|local_abs_path| package_name_and_bin_name_from_abs_path(local_abs_path))
            {
                return Ok(TaskVariables::from_iter([
                    (RUST_PACKAGE_TASK_VARIABLE.clone(), package_name),
                    (RUST_BIN_NAME_TASK_VARIABLE.clone(), bin_name),
                ]));
            }
        }

        if let Some(package_name) = local_abs_path
            .and_then(|local_abs_path| local_abs_path.parent())
            .and_then(human_readable_package_name)
        {
            return Ok(TaskVariables::from_iter([(
                RUST_PACKAGE_TASK_VARIABLE.clone(),
                package_name,
            )]));
        }

        Ok(TaskVariables::default())
    }

    fn associated_tasks(
        &self,
        file: Option<Arc<dyn language::File>>,
        cx: &AppContext,
    ) -> Option<TaskTemplates> {
        const DEFAULT_RUN_NAME_STR: &'static str = "RUST_DEFAULT_PACKAGE_RUN";
        let package_to_run = all_language_settings(file.as_ref(), cx)
            .language(Some("Rust"))
            .tasks
            .variables
            .get(DEFAULT_RUN_NAME_STR);
        let run_task_args = if let Some(package_to_run) = package_to_run {
            vec!["run".into(), "-p".into(), package_to_run.clone()]
        } else {
            vec!["run".into()]
        };
        Some(TaskTemplates(vec![
            TaskTemplate {
                label: format!(
                    "cargo check -p {}",
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "check".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "cargo check --workspace --all-targets".into(),
                command: "cargo".into(),
                args: vec!["check".into(), "--workspace".into(), "--all-targets".into()],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "cargo test -p {} {} -- --nocapture",
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Symbol.template_value(),
                    "--".into(),
                    "--nocapture".into(),
                ],
                tags: vec!["rust-test".to_owned()],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "cargo test -p {} {}",
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Stem.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    VariableName::Stem.template_value(),
                ],
                tags: vec!["rust-mod-test".to_owned()],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "cargo run -p {} --bin {}",
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    RUST_BIN_NAME_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "run".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    "--bin".into(),
                    RUST_BIN_NAME_TASK_VARIABLE.template_value(),
                ],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                tags: vec!["rust-main".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "cargo test -p {}",
                    RUST_PACKAGE_TASK_VARIABLE.template_value()
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "cargo run".into(),
                command: "cargo".into(),
                args: run_task_args,
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "cargo clean".into(),
                command: "cargo".into(),
                args: vec!["clean".into()],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
        ]))
    }
}

/// Part of the data structure of Cargo metadata
#[derive(serde::Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(serde::Deserialize)]
struct CargoPackage {
    id: String,
    targets: Vec<CargoTarget>,
}

#[derive(serde::Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
    src_path: String,
}

fn package_name_and_bin_name_from_abs_path(abs_path: &Path) -> Option<(String, String)> {
    let output = std::process::Command::new("cargo")
        .current_dir(abs_path.parent()?)
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1")
        .output()
        .log_err()?
        .stdout;

    let metadata: CargoMetadata = serde_json::from_slice(&output).log_err()?;

    retrieve_package_id_and_bin_name_from_metadata(metadata, abs_path).and_then(
        |(package_id, bin_name)| {
            let package_name = package_name_from_pkgid(&package_id);

            package_name.map(|package_name| (package_name.to_owned(), bin_name))
        },
    )
}

fn retrieve_package_id_and_bin_name_from_metadata(
    metadata: CargoMetadata,
    abs_path: &Path,
) -> Option<(String, String)> {
    for package in metadata.packages {
        for target in package.targets {
            let is_bin = target.kind.iter().any(|kind| kind == "bin");
            let target_path = PathBuf::from(target.src_path);
            if target_path == abs_path && is_bin {
                return Some((package.id, target.name));
            }
        }
    }

    None
}

fn human_readable_package_name(package_directory: &Path) -> Option<String> {
    let pkgid = String::from_utf8(
        std::process::Command::new("cargo")
            .current_dir(package_directory)
            .arg("pkgid")
            .output()
            .log_err()?
            .stdout,
    )
    .ok()?;
    Some(package_name_from_pkgid(&pkgid)?.to_owned())
}

// For providing local `cargo check -p $pkgid` task, we do not need most of the information we have returned.
// Output example in the root of Zed project:
// ```sh
// ❯ cargo pkgid zed
// path+file:///absolute/path/to/project/zed/crates/zed#0.131.0
// ```
// Another variant, if a project has a custom package name or hyphen in the name:
// ```
// path+file:///absolute/path/to/project/custom-package#my-custom-package@0.1.0
// ```
//
// Extracts the package name from the output according to the spec:
// https://doc.rust-lang.org/cargo/reference/pkgid-spec.html#specification-grammar
fn package_name_from_pkgid(pkgid: &str) -> Option<&str> {
    fn split_off_suffix(input: &str, suffix_start: char) -> &str {
        match input.rsplit_once(suffix_start) {
            Some((without_suffix, _)) => without_suffix,
            None => input,
        }
    }

    let (version_prefix, version_suffix) = pkgid.trim().rsplit_once('#')?;
    let package_name = match version_suffix.rsplit_once('@') {
        Some((custom_package_name, _version)) => custom_package_name,
        None => {
            let host_and_path = split_off_suffix(version_prefix, '?');
            let (_, package_name) = host_and_path.rsplit_once('/')?;
            package_name
        }
    };
    Some(package_name)
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        anyhow::Ok(LanguageServerBinary {
            path: last.ok_or_else(|| anyhow!("no cached binary"))?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::language;
    use gpui::{BorrowAppContext, Context, Hsla, TestAppContext};
    use language::language_settings::AllLanguageSettings;
    use lsp::CompletionItemLabelDetails;
    use settings::SettingsStore;
    use theme::SyntaxTheme;

    #[gpui::test]
    async fn test_process_rust_diagnostics() {
        let mut params = lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a").unwrap(),
            version: None,
            diagnostics: vec![
                // no newlines
                lsp::Diagnostic {
                    message: "use of moved value `a`".to_string(),
                    ..Default::default()
                },
                // newline at the end of a code span
                lsp::Diagnostic {
                    message: "consider importing this struct: `use b::c;\n`".to_string(),
                    ..Default::default()
                },
                // code span starting right after a newline
                lsp::Diagnostic {
                    message: "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
                        .to_string(),
                    ..Default::default()
                },
            ],
        };
        RustLspAdapter.process_diagnostics(&mut params);

        assert_eq!(params.diagnostics[0].message, "use of moved value `a`");

        // remove trailing newline from code span
        assert_eq!(
            params.diagnostics[1].message,
            "consider importing this struct: `use b::c;`"
        );

        // do not remove newline before the start of code span
        assert_eq!(
            params.diagnostics[2].message,
            "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
        );
    }

    #[gpui::test]
    async fn test_rust_label_for_completion() {
        let adapter = Arc::new(RustLspAdapter);
        let language = language("rust", tree_sitter_rust::language());
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new_test([
            ("type", Hsla::default()),
            ("keyword", Hsla::default()),
            ("function", Hsla::default()),
            ("property", Hsla::default()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();
        let highlight_field = grammar.highlight_id_for_name("property").unwrap();

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some(" (use crate::foo)".into()),
                            description: Some("fn(&mut Option<T>) -> Vec<T>".to_string())
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            })
        );
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some(" (use crate::foo)".into()),
                            description: Some("async fn(&mut Option<T>) -> Vec<T>".to_string()),
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            })
        );
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FIELD),
                        label: "len".to_string(),
                        detail: Some("usize".to_string()),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "len: usize".to_string(),
                filter_range: 0..3,
                runs: vec![(0..3, highlight_field), (5..10, highlight_type),],
            })
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some(" (use crate::foo)".to_string()),
                            description: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                        }),

                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel {
                text: "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                filter_range: 0..5,
                runs: vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            })
        );
    }

    #[gpui::test]
    async fn test_rust_label_for_symbol() {
        let adapter = Arc::new(RustLspAdapter);
        let language = language("rust", tree_sitter_rust::language());
        let grammar = language.grammar().unwrap();
        let theme = SyntaxTheme::new_test([
            ("type", Hsla::default()),
            ("keyword", Hsla::default()),
            ("function", Hsla::default()),
            ("property", Hsla::default()),
        ]);

        language.set_theme(&theme);

        let highlight_function = grammar.highlight_id_for_name("function").unwrap();
        let highlight_type = grammar.highlight_id_for_name("type").unwrap();
        let highlight_keyword = grammar.highlight_id_for_name("keyword").unwrap();

        assert_eq!(
            adapter
                .label_for_symbol("hello", lsp::SymbolKind::FUNCTION, &language)
                .await,
            Some(CodeLabel {
                text: "fn hello".to_string(),
                filter_range: 3..8,
                runs: vec![(0..2, highlight_keyword), (3..8, highlight_function)],
            })
        );

        assert_eq!(
            adapter
                .label_for_symbol("World", lsp::SymbolKind::TYPE_PARAMETER, &language)
                .await,
            Some(CodeLabel {
                text: "type World".to_string(),
                filter_range: 5..10,
                runs: vec![(0..4, highlight_keyword), (5..10, highlight_type)],
            })
        );
    }

    #[gpui::test]
    async fn test_rust_autoindent(cx: &mut TestAppContext) {
        // cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            language::init(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |s| {
                    s.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });

        let language = crate::language("rust", tree_sitter_rust::language());

        cx.new_model(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            // indent between braces
            buffer.set_text("fn a() {}", cx);
            let ix = buffer.len() - 1;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  \n}");

            // indent between braces, even after empty lines
            buffer.set_text("fn a() {\n\n\n}", cx);
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n\n\n  \n}");

            // indent a line that continues a field expression
            buffer.set_text("fn a() {\n  \n}", cx);
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "b\n.c")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  b\n    .c\n}");

            // indent further lines that continue the field expression, even after empty lines
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, "\n\n.d")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "fn a() {\n  b\n    .c\n    \n    .d\n}");

            // dedent the line after the field expression
            let ix = buffer.len() - 2;
            buffer.edit([(ix..ix, ";\ne")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "fn a() {\n  b\n    .c\n    \n    .d;\n  e\n}"
            );

            // indent inside a struct within a call
            buffer.set_text("const a: B = c(D {});", cx);
            let ix = buffer.len() - 3;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "const a: B = c(D {\n  \n});");

            // indent further inside a nested call
            let ix = buffer.len() - 4;
            buffer.edit([(ix..ix, "e: f(\n\n)")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(buffer.text(), "const a: B = c(D {\n  e: f(\n    \n  )\n});");

            // keep that indent after an empty line
            let ix = buffer.len() - 8;
            buffer.edit([(ix..ix, "\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "const a: B = c(D {\n  e: f(\n    \n    \n  )\n});"
            );

            buffer
        });
    }

    #[test]
    fn test_package_name_from_pkgid() {
        for (input, expected) in [
            (
                "path+file:///absolute/path/to/project/zed/crates/zed#0.131.0",
                "zed",
            ),
            (
                "path+file:///absolute/path/to/project/custom-package#my-custom-package@0.1.0",
                "my-custom-package",
            ),
        ] {
            assert_eq!(package_name_from_pkgid(input), Some(expected));
        }
    }

    #[test]
    fn test_retrieve_package_id_and_bin_name_from_metadata() {
        for (input, absolute_path, expected) in [
            (
                r#"{"packages":[{"id":"path+file:///path/to/zed/crates/zed#0.131.0","targets":[{"name":"zed","kind":["bin"],"src_path":"/path/to/zed/src/main.rs"}]}]}"#,
                "/path/to/zed/src/main.rs",
                Some(("path+file:///path/to/zed/crates/zed#0.131.0", "zed")),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","targets":[{"name":"my-custom-bin","kind":["bin"],"src_path":"/path/to/custom-package/src/main.rs"}]}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((
                    "path+file:///path/to/custom-package#my-custom-package@0.1.0",
                    "my-custom-bin",
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","targets":[{"name":"my-custom-package","kind":["lib"],"src_path":"/path/to/custom-package/src/main.rs"}]}]}"#,
                "/path/to/custom-package/src/main.rs",
                None,
            ),
        ] {
            let metadata: CargoMetadata = serde_json::from_str(input).unwrap();

            let absolute_path = Path::new(absolute_path);

            assert_eq!(
                retrieve_package_id_and_bin_name_from_metadata(metadata, absolute_path),
                expected.map(|(pkgid, bin)| (pkgid.to_owned(), bin.to_owned()))
            );
        }
    }
}
