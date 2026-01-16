use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use futures::lock::OwnedMutexGuard;
use gpui::{App, AppContext, AsyncApp, SharedString, Task};
use http_client::github::AssetKind;
use http_client::github::{GitHubLspBinaryVersion, latest_github_release};
use http_client::github_download::{GithubBinaryMetadata, download_server_binary};
pub use language::*;
use lsp::{InitializeParams, LanguageServerBinary, LanguageServerBinaryOptions};
use project::lsp_store::rust_analyzer_ext::CARGO_DIAGNOSTICS_SOURCE_NAME;
use project::project_settings::ProjectSettings;
use regex::Regex;
use serde_json::json;
use settings::Settings as _;
use smallvec::SmallVec;
use smol::fs::{self};
use std::cmp::Reverse;
use std::fmt::Display;
use std::ops::Range;
use std::process::Stdio;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};
use util::fs::{make_file_executable, remove_matching};
use util::merge_json_value_into;
use util::rel_path::RelPath;
use util::{ResultExt, maybe};

use crate::language_settings::language_settings;

pub struct RustLspAdapter;

#[cfg(target_os = "macos")]
impl RustLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Gz;
    const ARCH_SERVER_NAME: &str = "apple-darwin";
}

#[cfg(target_os = "linux")]
impl RustLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Gz;
    const ARCH_SERVER_NAME: &str = "unknown-linux";
}

#[cfg(target_os = "freebsd")]
impl RustLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Gz;
    const ARCH_SERVER_NAME: &str = "unknown-freebsd";
}

#[cfg(target_os = "windows")]
impl RustLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;
    const ARCH_SERVER_NAME: &str = "pc-windows-msvc";
}

const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("rust-analyzer");

#[cfg(target_os = "linux")]
enum LibcType {
    Gnu,
    Musl,
}

impl RustLspAdapter {
    fn convert_rust_analyzer_schema(raw_schema: &serde_json::Value) -> serde_json::Value {
        let Some(schema_array) = raw_schema.as_array() else {
            return raw_schema.clone();
        };

        let mut root_properties = serde_json::Map::new();

        for item in schema_array {
            if let Some(props) = item.get("properties").and_then(|p| p.as_object()) {
                for (key, value) in props {
                    let parts: Vec<&str> = key.split('.').collect();

                    if parts.is_empty() {
                        continue;
                    }

                    let parts_to_process = if parts.first() == Some(&"rust-analyzer") {
                        &parts[1..]
                    } else {
                        &parts[..]
                    };

                    if parts_to_process.is_empty() {
                        continue;
                    }

                    let mut current = &mut root_properties;

                    for (i, part) in parts_to_process.iter().enumerate() {
                        let is_last = i == parts_to_process.len() - 1;

                        if is_last {
                            current.insert(part.to_string(), value.clone());
                        } else {
                            let next_current = current
                                .entry(part.to_string())
                                .or_insert_with(|| {
                                    serde_json::json!({
                                        "type": "object",
                                        "properties": {}
                                    })
                                })
                                .as_object_mut()
                                .expect("should be an object")
                                .entry("properties")
                                .or_insert_with(|| serde_json::json!({}))
                                .as_object_mut()
                                .expect("properties should be an object");

                            current = next_current;
                        }
                    }
                }
            }
        }

        serde_json::json!({
            "type": "object",
            "properties": root_properties
        })
    }

    #[cfg(target_os = "linux")]
    async fn determine_libc_type() -> LibcType {
        use futures::pin_mut;

        async fn from_ldd_version() -> Option<LibcType> {
            use util::command::new_smol_command;

            let ldd_output = new_smol_command("ldd")
                .arg("--version")
                .output()
                .await
                .ok()?;
            let ldd_version = String::from_utf8_lossy(&ldd_output.stdout);

            if ldd_version.contains("GNU libc") || ldd_version.contains("GLIBC") {
                Some(LibcType::Gnu)
            } else if ldd_version.contains("musl") {
                Some(LibcType::Musl)
            } else {
                None
            }
        }

        if let Some(libc_type) = from_ldd_version().await {
            return libc_type;
        }

        let Ok(dir_entries) = smol::fs::read_dir("/lib").await else {
            // defaulting to gnu because nix doesn't have /lib files due to not following FHS
            return LibcType::Gnu;
        };
        let dir_entries = dir_entries.filter_map(async move |e| e.ok());
        pin_mut!(dir_entries);

        let mut has_musl = false;
        let mut has_gnu = false;

        while let Some(entry) = dir_entries.next().await {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if file_name.starts_with("ld-musl-") {
                has_musl = true;
            } else if file_name.starts_with("ld-linux-") {
                has_gnu = true;
            }
        }

        match (has_musl, has_gnu) {
            (true, _) => LibcType::Musl,
            (_, true) => LibcType::Gnu,
            _ => LibcType::Gnu,
        }
    }

    #[cfg(target_os = "linux")]
    async fn build_arch_server_name_linux() -> String {
        let libc = match Self::determine_libc_type().await {
            LibcType::Musl => "musl",
            LibcType::Gnu => "gnu",
        };

        format!("{}-{}", Self::ARCH_SERVER_NAME, libc)
    }

    async fn build_asset_name() -> String {
        let extension = match Self::GITHUB_ASSET_KIND {
            AssetKind::TarGz => "tar.gz",
            AssetKind::Gz => "gz",
            AssetKind::Zip => "zip",
        };

        #[cfg(target_os = "linux")]
        let arch_server_name = Self::build_arch_server_name_linux().await;
        #[cfg(not(target_os = "linux"))]
        let arch_server_name = Self::ARCH_SERVER_NAME.to_string();

        format!(
            "{}-{}-{}.{}",
            SERVER_NAME,
            std::env::consts::ARCH,
            &arch_server_name,
            extension
        )
    }
}

pub(crate) struct CargoManifestProvider;

impl ManifestProvider for CargoManifestProvider {
    fn name(&self) -> ManifestName {
        SharedString::new_static("Cargo.toml").into()
    }

    fn search(
        &self,
        ManifestQuery {
            path,
            depth,
            delegate,
        }: ManifestQuery,
    ) -> Option<Arc<RelPath>> {
        let mut outermost_cargo_toml = None;
        for path in path.ancestors().take(depth) {
            let p = path.join(RelPath::unix("Cargo.toml").unwrap());
            if delegate.exists(&p, Some(false)) {
                outermost_cargo_toml = Some(Arc::from(path));
            }
        }

        outermost_cargo_toml
    }
}

#[async_trait(?Send)]
impl LspAdapter for RustLspAdapter {
    fn name(&self) -> LanguageServerName {
        SERVER_NAME
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec![CARGO_DIAGNOSTICS_SOURCE_NAME.to_owned()]
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        Some("rust-analyzer/flycheck".into())
    }

    fn process_diagnostics(
        &self,
        params: &mut lsp::PublishDiagnosticsParams,
        _: LanguageServerId,
        _: Option<&'_ Buffer>,
    ) {
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

    fn diagnostic_message_to_markdown(&self, message: &str) -> Option<String> {
        static REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?m)\n *").expect("Failed to create REGEX"));
        Some(REGEX.replace_all(message, "\n\n").to_string())
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        // rust-analyzer calls these detail left and detail right in terms of where it expects things to be rendered
        // this usually contains signatures of the thing to be completed
        let detail_right = completion
            .label_details
            .as_ref()
            .and_then(|detail| detail.description.as_ref())
            .or(completion.detail.as_ref())
            .map(|detail| detail.trim());
        // this tends to contain alias and import information
        let mut detail_left = completion
            .label_details
            .as_ref()
            .and_then(|detail| detail.detail.as_deref());
        let mk_label = |text: String, filter_range: &dyn Fn() -> Range<usize>, runs| {
            let filter_range = completion
                .filter_text
                .as_deref()
                .and_then(|filter| text.find(filter).map(|ix| ix..ix + filter.len()))
                .or_else(|| {
                    text.find(&completion.label)
                        .map(|ix| ix..ix + completion.label.len())
                })
                .unwrap_or_else(filter_range);

            CodeLabel::new(text, filter_range, runs)
        };
        let mut label = match (detail_right, completion.kind) {
            (Some(signature), Some(lsp::CompletionItemKind::FIELD)) => {
                let name = &completion.label;
                let text = format!("{name}: {signature}");
                let prefix = "struct S { ";
                let source = Rope::from_iter([prefix, &text, " }"]);
                let runs =
                    language.highlight_text(&source, prefix.len()..prefix.len() + text.len());
                mk_label(text, &|| 0..completion.label.len(), runs)
            }
            (
                Some(signature),
                Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE),
            ) if completion.insert_text_format != Some(lsp::InsertTextFormat::SNIPPET) => {
                let name = &completion.label;
                let text = format!("{name}: {signature}",);
                let prefix = "let ";
                let source = Rope::from_iter([prefix, &text, " = ();"]);
                let runs =
                    language.highlight_text(&source, prefix.len()..prefix.len() + text.len());
                mk_label(text, &|| 0..completion.label.len(), runs)
            }
            (
                function_signature,
                Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD),
            ) => {
                const FUNCTION_PREFIXES: [&str; 6] = [
                    "async fn",
                    "async unsafe fn",
                    "const fn",
                    "const unsafe fn",
                    "unsafe fn",
                    "fn",
                ];
                let fn_prefixed = FUNCTION_PREFIXES.iter().find_map(|&prefix| {
                    function_signature?
                        .strip_prefix(prefix)
                        .map(|suffix| (prefix, suffix))
                });
                let label = if let Some(label) = completion
                    .label
                    .strip_suffix("(…)")
                    .or_else(|| completion.label.strip_suffix("()"))
                {
                    label
                } else {
                    &completion.label
                };

                static FULL_SIGNATURE_REGEX: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"fn (.?+)\(").expect("Failed to create REGEX"));
                if let Some((function_signature, match_)) = function_signature
                    .filter(|it| it.contains(&label))
                    .and_then(|it| Some((it, FULL_SIGNATURE_REGEX.find(it)?)))
                {
                    let source = Rope::from(function_signature);
                    let runs = language.highlight_text(&source, 0..function_signature.len());
                    mk_label(
                        function_signature.to_owned(),
                        &|| match_.range().start - 3..match_.range().end - 1,
                        runs,
                    )
                } else if let Some((prefix, suffix)) = fn_prefixed {
                    let text = format!("{label}{suffix}");
                    let source = Rope::from_iter([prefix, " ", &text, " {}"]);
                    let run_start = prefix.len() + 1;
                    let runs = language.highlight_text(&source, run_start..run_start + text.len());
                    mk_label(text, &|| 0..label.len(), runs)
                } else if completion
                    .detail
                    .as_ref()
                    .is_some_and(|detail| detail.starts_with("macro_rules! "))
                {
                    let text = completion.label.clone();
                    let len = text.len();
                    let source = Rope::from(text.as_str());
                    let runs = language.highlight_text(&source, 0..len);
                    mk_label(text, &|| 0..completion.label.len(), runs)
                } else if detail_left.is_none() {
                    return None;
                } else {
                    mk_label(
                        completion.label.clone(),
                        &|| 0..completion.label.len(),
                        vec![],
                    )
                }
            }
            (_, kind) => {
                let mut label;
                let mut runs = vec![];

                if completion.insert_text_format == Some(lsp::InsertTextFormat::SNIPPET)
                    && let Some(
                        lsp::CompletionTextEdit::InsertAndReplace(lsp::InsertReplaceEdit {
                            new_text,
                            ..
                        })
                        | lsp::CompletionTextEdit::Edit(lsp::TextEdit { new_text, .. }),
                    ) = completion.text_edit.as_ref()
                    && let Ok(mut snippet) = snippet::Snippet::parse(new_text)
                    && snippet.tabstops.len() > 1
                {
                    label = String::new();

                    // we never display the final tabstop
                    snippet.tabstops.remove(snippet.tabstops.len() - 1);

                    let mut text_pos = 0;

                    let mut all_stop_ranges = snippet
                        .tabstops
                        .into_iter()
                        .flat_map(|stop| stop.ranges)
                        .collect::<SmallVec<[_; 8]>>();
                    all_stop_ranges.sort_unstable_by_key(|a| (a.start, Reverse(a.end)));

                    for range in &all_stop_ranges {
                        let start_pos = range.start as usize;
                        let end_pos = range.end as usize;

                        label.push_str(&snippet.text[text_pos..start_pos]);

                        if start_pos == end_pos {
                            let caret_start = label.len();
                            label.push('…');
                            runs.push((caret_start..label.len(), HighlightId::TABSTOP_INSERT_ID));
                        } else {
                            let label_start = label.len();
                            label.push_str(&snippet.text[start_pos..end_pos]);
                            let label_end = label.len();
                            runs.push((label_start..label_end, HighlightId::TABSTOP_REPLACE_ID));
                        }

                        text_pos = end_pos;
                    }

                    label.push_str(&snippet.text[text_pos..]);

                    if detail_left.is_some_and(|detail_left| detail_left == new_text) {
                        // We only include the left detail if it isn't the snippet again
                        detail_left.take();
                    }

                    runs.extend(language.highlight_text(&Rope::from(&label), 0..label.len()));
                } else {
                    let highlight_name = kind.and_then(|kind| match kind {
                        lsp::CompletionItemKind::STRUCT
                        | lsp::CompletionItemKind::INTERFACE
                        | lsp::CompletionItemKind::ENUM => Some("type"),
                        lsp::CompletionItemKind::ENUM_MEMBER => Some("variant"),
                        lsp::CompletionItemKind::KEYWORD => Some("keyword"),
                        lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
                            Some("constant")
                        }
                        _ => None,
                    });

                    label = completion.label.clone();

                    if let Some(highlight_name) = highlight_name {
                        let highlight_id =
                            language.grammar()?.highlight_id_for_name(highlight_name)?;
                        runs.push((
                            0..label.rfind('(').unwrap_or(completion.label.len()),
                            highlight_id,
                        ));
                    } else if detail_left.is_none()
                        && kind != Some(lsp::CompletionItemKind::SNIPPET)
                    {
                        return None;
                    }
                }

                let label_len = label.len();

                mk_label(label, &|| 0..label_len, runs)
            }
        };

        if let Some(detail_left) = detail_left {
            label.text.push(' ');
            if !detail_left.starts_with('(') {
                label.text.push('(');
            }
            label.text.push_str(detail_left);
            if !detail_left.ends_with(')') {
                label.text.push(')');
            }
        }

        Some(label)
    }

    async fn initialization_options_schema(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cached_binary: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        cx: &mut AsyncApp,
    ) -> Option<serde_json::Value> {
        let binary = self
            .get_language_server_command(
                delegate.clone(),
                None,
                LanguageServerBinaryOptions {
                    allow_path_lookup: true,
                    allow_binary_download: false,
                    pre_release: false,
                },
                cached_binary,
                cx.clone(),
            )
            .await
            .0
            .ok()?;

        let mut command = util::command::new_smol_command(&binary.path);
        command
            .arg("--print-config-schema")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let cmd = command
            .spawn()
            .map_err(|e| log::debug!("failed to spawn command {command:?}: {e}"))
            .ok()?;
        let output = cmd
            .output()
            .await
            .map_err(|e| log::debug!("failed to execute command {command:?}: {e}"))
            .ok()?;
        if !output.status.success() {
            return None;
        }

        let raw_schema: serde_json::Value = serde_json::from_slice(output.stdout.as_slice())
            .map_err(|e| log::debug!("failed to parse rust-analyzer's JSON schema output: {e}"))
            .ok()?;

        // Convert rust-analyzer's array-based schema format to nested JSON Schema
        let converted_schema = Self::convert_rust_analyzer_schema(&raw_schema);
        Some(converted_schema)
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (prefix, suffix) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => ("fn ", "();"),
            lsp::SymbolKind::STRUCT => ("struct ", ";"),
            lsp::SymbolKind::ENUM => ("enum ", "{}"),
            lsp::SymbolKind::ENUM_MEMBER => ("enum E{", "}"),
            lsp::SymbolKind::INTERFACE => ("trait ", "{}"),
            lsp::SymbolKind::CONSTANT => ("const ", ":()=();"),
            lsp::SymbolKind::MODULE => ("mod ", ";"),
            lsp::SymbolKind::PACKAGE => ("extern crate ", ";"),
            lsp::SymbolKind::TYPE_PARAMETER => ("type ", "=();"),
            _ => return None,
        };

        let filter_range = prefix.len()..prefix.len() + name.len();
        let display_range = 0..filter_range.end;
        Some(CodeLabel::new(
            format!("{prefix}{name}"),
            filter_range,
            language.highlight_text(&Rope::from_iter([prefix, name, suffix]), display_range),
        ))
    }

    fn prepare_initialize_params(
        &self,
        mut original: InitializeParams,
        cx: &App,
    ) -> Result<InitializeParams> {
        let enable_lsp_tasks = ProjectSettings::get_global(cx)
            .lsp
            .get(&SERVER_NAME)
            .is_some_and(|s| s.enable_lsp_tasks);
        if enable_lsp_tasks {
            let experimental = json!({
                "runnables": {
                    "kinds": [ "cargo", "shell" ],
                },
            });
            if let Some(original_experimental) = &mut original.capabilities.experimental {
                merge_json_value_into(experimental, original_experimental);
            } else {
                original.capabilities.experimental = Some(experimental);
            }
        }

        Ok(original)
    }
}

impl LspInstaller for RustLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;
    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which("rust-analyzer".as_ref()).await?;
        let env = delegate.shell_env().await;

        // It is surprisingly common for ~/.cargo/bin/rust-analyzer to be a symlink to
        // /usr/bin/rust-analyzer that fails when you run it; so we need to test it.
        log::debug!("found rust-analyzer in PATH. trying to run `rust-analyzer --help`");
        let result = delegate
            .try_exec(LanguageServerBinary {
                path: path.clone(),
                arguments: vec!["--help".into()],
                env: Some(env.clone()),
            })
            .await;
        if let Err(err) = result {
            log::debug!(
                "failed to run rust-analyzer after detecting it in PATH: binary: {:?}: {}",
                path,
                err
            );
            return None;
        }

        Some(LanguageServerBinary {
            path,
            env: Some(env),
            arguments: vec![],
        })
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        pre_release: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release = latest_github_release(
            "rust-lang/rust-analyzer",
            true,
            pre_release,
            delegate.http_client(),
        )
        .await?;
        let asset_name = Self::build_asset_name().await;
        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url,
            digest: asset.digest,
        })
    }

    async fn fetch_server_binary(
        &self,
        version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let GitHubLspBinaryVersion {
            name,
            url,
            digest: expected_digest,
        } = version;
        let destination_path = container_dir.join(format!("rust-analyzer-{name}"));
        let server_path = match Self::GITHUB_ASSET_KIND {
            AssetKind::TarGz | AssetKind::Gz => destination_path.clone(), // Tar and gzip extract in place.
            AssetKind::Zip => destination_path.clone().join("rust-analyzer.exe"), // zip contains a .exe
        };

        let binary = LanguageServerBinary {
            path: server_path.clone(),
            env: None,
            arguments: Default::default(),
        };

        let metadata_path = destination_path.with_extension("metadata");
        let metadata = GithubBinaryMetadata::read_from_file(&metadata_path)
            .await
            .ok();
        if let Some(metadata) = metadata {
            let validity_check = async || {
                delegate
                    .try_exec(LanguageServerBinary {
                        path: server_path.clone(),
                        arguments: vec!["--version".into()],
                        env: None,
                    })
                    .await
                    .inspect_err(|err| {
                        log::warn!("Unable to run {server_path:?} asset, redownloading: {err:#}",)
                    })
            };
            if let (Some(actual_digest), Some(expected_digest)) =
                (&metadata.digest, &expected_digest)
            {
                if actual_digest == expected_digest {
                    if validity_check().await.is_ok() {
                        return Ok(binary);
                    }
                } else {
                    log::info!(
                        "SHA-256 mismatch for {destination_path:?} asset, downloading new asset. Expected: {expected_digest}, Got: {actual_digest}"
                    );
                }
            } else if validity_check().await.is_ok() {
                return Ok(binary);
            }
        }

        download_server_binary(
            &*delegate.http_client(),
            &url,
            expected_digest.as_deref(),
            &destination_path,
            Self::GITHUB_ASSET_KIND,
        )
        .await?;
        make_file_executable(&server_path).await?;
        remove_matching(&container_dir, |path| path != destination_path).await;
        GithubBinaryMetadata::write_to_file(
            &GithubBinaryMetadata {
                metadata_version: 1,
                digest: expected_digest,
            },
            &metadata_path,
        )
        .await?;

        Ok(LanguageServerBinary {
            path: server_path,
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
}

pub(crate) struct RustContextProvider;

const RUST_PACKAGE_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_PACKAGE"));

/// The bin name corresponding to the current file in Cargo.toml
const RUST_BIN_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_BIN_NAME"));

/// The bin kind (bin/example) corresponding to the current file in Cargo.toml
const RUST_BIN_KIND_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_BIN_KIND"));

/// The flag to list required features for executing a bin, if any
const RUST_BIN_REQUIRED_FEATURES_FLAG_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_BIN_REQUIRED_FEATURES_FLAG"));

/// The list of required features for executing a bin, if any
const RUST_BIN_REQUIRED_FEATURES_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_BIN_REQUIRED_FEATURES"));

const RUST_TEST_FRAGMENT_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_TEST_FRAGMENT"));

const RUST_DOC_TEST_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_DOC_TEST_NAME"));

const RUST_TEST_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_TEST_NAME"));

const RUST_MANIFEST_DIRNAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("RUST_MANIFEST_DIRNAME"));

impl ContextProvider for RustContextProvider {
    fn build_context(
        &self,
        task_variables: &TaskVariables,
        location: ContextLocation<'_>,
        project_env: Option<HashMap<String, String>>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut gpui::App,
    ) -> Task<Result<TaskVariables>> {
        let local_abs_path = location
            .file_location
            .buffer
            .read(cx)
            .file()
            .and_then(|file| Some(file.as_local()?.abs_path(cx)));

        let mut variables = TaskVariables::default();

        if let (Some(path), Some(stem)) = (&local_abs_path, task_variables.get(&VariableName::Stem))
        {
            let fragment = test_fragment(&variables, path, stem);
            variables.insert(RUST_TEST_FRAGMENT_TASK_VARIABLE, fragment);
        };
        if let Some(test_name) =
            task_variables.get(&VariableName::Custom(Cow::Borrowed("_test_name")))
        {
            variables.insert(RUST_TEST_NAME_TASK_VARIABLE, test_name.into());
        }
        if let Some(doc_test_name) =
            task_variables.get(&VariableName::Custom(Cow::Borrowed("_doc_test_name")))
        {
            variables.insert(RUST_DOC_TEST_NAME_TASK_VARIABLE, doc_test_name.into());
        }
        cx.background_spawn(async move {
            if let Some(path) = local_abs_path
                .as_deref()
                .and_then(|local_abs_path| local_abs_path.parent())
                && let Some(package_name) =
                    human_readable_package_name(path, project_env.as_ref()).await
            {
                variables.insert(RUST_PACKAGE_TASK_VARIABLE.clone(), package_name);
            }
            if let Some(path) = local_abs_path.as_ref()
                && let Some((target, manifest_path)) =
                    target_info_from_abs_path(path, project_env.as_ref()).await
            {
                if let Some(target) = target {
                    variables.extend(TaskVariables::from_iter([
                        (RUST_PACKAGE_TASK_VARIABLE.clone(), target.package_name),
                        (RUST_BIN_NAME_TASK_VARIABLE.clone(), target.target_name),
                        (
                            RUST_BIN_KIND_TASK_VARIABLE.clone(),
                            target.target_kind.to_string(),
                        ),
                    ]));
                    if target.required_features.is_empty() {
                        variables.insert(RUST_BIN_REQUIRED_FEATURES_FLAG_TASK_VARIABLE, "".into());
                        variables.insert(RUST_BIN_REQUIRED_FEATURES_TASK_VARIABLE, "".into());
                    } else {
                        variables.insert(
                            RUST_BIN_REQUIRED_FEATURES_FLAG_TASK_VARIABLE.clone(),
                            "--features".to_string(),
                        );
                        variables.insert(
                            RUST_BIN_REQUIRED_FEATURES_TASK_VARIABLE.clone(),
                            target.required_features.join(","),
                        );
                    }
                }
                variables.extend(TaskVariables::from_iter([(
                    RUST_MANIFEST_DIRNAME_TASK_VARIABLE.clone(),
                    manifest_path.to_string_lossy().into_owned(),
                )]));
            }
            Ok(variables)
        })
    }

    fn associated_tasks(
        &self,
        file: Option<Arc<dyn language::File>>,
        cx: &App,
    ) -> Task<Option<TaskTemplates>> {
        const DEFAULT_RUN_NAME_STR: &str = "RUST_DEFAULT_PACKAGE_RUN";
        const CUSTOM_TARGET_DIR: &str = "RUST_TARGET_DIR";

        let language_sets = language_settings(Some("Rust".into()), file.as_ref(), cx);
        let package_to_run = language_sets
            .tasks
            .variables
            .get(DEFAULT_RUN_NAME_STR)
            .cloned();
        let custom_target_dir = language_sets
            .tasks
            .variables
            .get(CUSTOM_TARGET_DIR)
            .cloned();
        let run_task_args = if let Some(package_to_run) = package_to_run {
            vec!["run".into(), "-p".into(), package_to_run]
        } else {
            vec!["run".into()]
        };
        let mut task_templates = vec![
            TaskTemplate {
                label: format!(
                    "Check (package: {})",
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
                label: "Check all targets (workspace)".into(),
                command: "cargo".into(),
                args: vec!["check".into(), "--workspace".into(), "--all-targets".into()],
                cwd: Some("$ZED_DIRNAME".to_owned()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "Test '{}' (package: {})",
                    RUST_TEST_NAME_TASK_VARIABLE.template_value(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    "--".into(),
                    "--nocapture".into(),
                    "--include-ignored".into(),
                    RUST_TEST_NAME_TASK_VARIABLE.template_value(),
                ],
                tags: vec!["rust-test".to_owned()],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "Doc test '{}' (package: {})",
                    RUST_DOC_TEST_NAME_TASK_VARIABLE.template_value(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "--doc".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    "--".into(),
                    "--nocapture".into(),
                    "--include-ignored".into(),
                    RUST_DOC_TEST_NAME_TASK_VARIABLE.template_value(),
                ],
                tags: vec!["rust-doc-test".to_owned()],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "Test mod '{}' (package: {})",
                    VariableName::Stem.template_value(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    "--".into(),
                    RUST_TEST_FRAGMENT_TASK_VARIABLE.template_value(),
                ],
                tags: vec!["rust-mod-test".to_owned()],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "Run {} {} (package: {})",
                    RUST_BIN_KIND_TASK_VARIABLE.template_value(),
                    RUST_BIN_NAME_TASK_VARIABLE.template_value(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ),
                command: "cargo".into(),
                args: vec![
                    "run".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                    format!("--{}", RUST_BIN_KIND_TASK_VARIABLE.template_value()),
                    RUST_BIN_NAME_TASK_VARIABLE.template_value(),
                    RUST_BIN_REQUIRED_FEATURES_FLAG_TASK_VARIABLE.template_value(),
                    RUST_BIN_REQUIRED_FEATURES_TASK_VARIABLE.template_value(),
                ],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                tags: vec!["rust-main".to_owned()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!(
                    "Test (package: {})",
                    RUST_PACKAGE_TASK_VARIABLE.template_value()
                ),
                command: "cargo".into(),
                args: vec![
                    "test".into(),
                    "-p".into(),
                    RUST_PACKAGE_TASK_VARIABLE.template_value(),
                ],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "Run".into(),
                command: "cargo".into(),
                args: run_task_args,
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "Clean".into(),
                command: "cargo".into(),
                args: vec!["clean".into()],
                cwd: Some(RUST_MANIFEST_DIRNAME_TASK_VARIABLE.template_value()),
                ..TaskTemplate::default()
            },
        ];

        if let Some(custom_target_dir) = custom_target_dir {
            task_templates = task_templates
                .into_iter()
                .map(|mut task_template| {
                    let mut args = task_template.args.split_off(1);
                    task_template.args.append(&mut vec![
                        "--target-dir".to_string(),
                        custom_target_dir.clone(),
                    ]);
                    task_template.args.append(&mut args);

                    task_template
                })
                .collect();
        }

        Task::ready(Some(TaskTemplates(task_templates)))
    }

    fn lsp_task_source(&self) -> Option<LanguageServerName> {
        Some(SERVER_NAME)
    }
}

/// Part of the data structure of Cargo metadata
#[derive(Debug, serde::Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Debug, serde::Deserialize)]
struct CargoPackage {
    id: String,
    targets: Vec<CargoTarget>,
    manifest_path: Arc<Path>,
}

#[derive(Debug, serde::Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
    src_path: String,
    #[serde(rename = "required-features", default)]
    required_features: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum TargetKind {
    Bin,
    Example,
}

impl Display for TargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetKind::Bin => write!(f, "bin"),
            TargetKind::Example => write!(f, "example"),
        }
    }
}

impl TryFrom<&str> for TargetKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "bin" => Ok(Self::Bin),
            "example" => Ok(Self::Example),
            _ => Err(()),
        }
    }
}
/// Which package and binary target are we in?
#[derive(Debug, PartialEq)]
struct TargetInfo {
    package_name: String,
    target_name: String,
    target_kind: TargetKind,
    required_features: Vec<String>,
}

async fn target_info_from_abs_path(
    abs_path: &Path,
    project_env: Option<&HashMap<String, String>>,
) -> Option<(Option<TargetInfo>, Arc<Path>)> {
    let mut command = util::command::new_smol_command("cargo");
    if let Some(envs) = project_env {
        command.envs(envs);
    }
    let output = command
        .current_dir(abs_path.parent()?)
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1")
        .output()
        .await
        .log_err()?
        .stdout;

    let metadata: CargoMetadata = serde_json::from_slice(&output).log_err()?;
    target_info_from_metadata(metadata, abs_path)
}

fn target_info_from_metadata(
    metadata: CargoMetadata,
    abs_path: &Path,
) -> Option<(Option<TargetInfo>, Arc<Path>)> {
    let mut manifest_path = None;
    for package in metadata.packages {
        let Some(manifest_dir_path) = package.manifest_path.parent() else {
            continue;
        };

        let Some(path_from_manifest_dir) = abs_path.strip_prefix(manifest_dir_path).ok() else {
            continue;
        };
        let candidate_path_length = path_from_manifest_dir.components().count();
        // Pick the most specific manifest path
        if let Some((path, current_length)) = &mut manifest_path {
            if candidate_path_length > *current_length {
                *path = Arc::from(manifest_dir_path);
                *current_length = candidate_path_length;
            }
        } else {
            manifest_path = Some((Arc::from(manifest_dir_path), candidate_path_length));
        };

        for target in package.targets {
            let Some(bin_kind) = target
                .kind
                .iter()
                .find_map(|kind| TargetKind::try_from(kind.as_ref()).ok())
            else {
                continue;
            };
            let target_path = PathBuf::from(target.src_path);
            if target_path == abs_path {
                return manifest_path.map(|(path, _)| {
                    (
                        package_name_from_pkgid(&package.id).map(|package_name| TargetInfo {
                            package_name: package_name.to_owned(),
                            target_name: target.name,
                            required_features: target.required_features,
                            target_kind: bin_kind,
                        }),
                        path,
                    )
                });
            }
        }
    }

    manifest_path.map(|(path, _)| (None, path))
}

async fn human_readable_package_name(
    package_directory: &Path,
    project_env: Option<&HashMap<String, String>>,
) -> Option<String> {
    let mut command = util::command::new_smol_command("cargo");
    if let Some(envs) = project_env {
        command.envs(envs);
    }
    let pkgid = String::from_utf8(
        command
            .current_dir(package_directory)
            .arg("pkgid")
            .output()
            .await
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
    let binary_result = maybe!(async {
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir)
            .await
            .with_context(|| format!("listing {container_dir:?}"))?;
        while let Some(entry) = entries.next().await {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "metadata") {
                continue;
            }
            last = Some(path);
        }

        let path = match last {
            Some(last) => last,
            None => return Ok(None),
        };
        let path = match RustLspAdapter::GITHUB_ASSET_KIND {
            AssetKind::TarGz | AssetKind::Gz => path, // Tar and gzip extract in place.
            AssetKind::Zip => path.join("rust-analyzer.exe"), // zip contains a .exe
        };

        anyhow::Ok(Some(LanguageServerBinary {
            path,
            env: None,
            arguments: Vec::new(),
        }))
    })
    .await;

    match binary_result {
        Ok(Some(binary)) => Some(binary),
        Ok(None) => {
            log::info!("No cached rust-analyzer binary found");
            None
        }
        Err(e) => {
            log::error!("Failed to look up cached rust-analyzer binary: {e:#}");
            None
        }
    }
}

fn test_fragment(variables: &TaskVariables, path: &Path, stem: &str) -> String {
    let fragment = if stem == "lib" {
        // This isn't quite right---it runs the tests for the entire library, rather than
        // just for the top-level `mod tests`. But we don't really have the means here to
        // filter out just that module.
        Some("--lib".to_owned())
    } else if stem == "mod" {
        maybe!({ Some(path.parent()?.file_name()?.to_string_lossy().into_owned()) })
    } else if stem == "main" {
        if let (Some(bin_name), Some(bin_kind)) = (
            variables.get(&RUST_BIN_NAME_TASK_VARIABLE),
            variables.get(&RUST_BIN_KIND_TASK_VARIABLE),
        ) {
            Some(format!("--{bin_kind}={bin_name}"))
        } else {
            None
        }
    } else {
        Some(stem.to_owned())
    };
    fragment.unwrap_or_else(|| "--".to_owned())
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::language;
    use gpui::{BorrowAppContext, Hsla, TestAppContext};
    use lsp::CompletionItemLabelDetails;
    use settings::SettingsStore;
    use theme::SyntaxTheme;
    use util::path;

    #[gpui::test]
    async fn test_process_rust_diagnostics() {
        let mut params = lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a")).unwrap(),
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
        RustLspAdapter.process_diagnostics(&mut params, LanguageServerId(0), None);

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
        let language = language("rust", tree_sitter_rust::LANGUAGE.into());
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
                            detail: Some("(use crate::foo)".into()),
                            description: Some("fn(&mut Option<T>) -> Vec<T>".to_string())
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                0..5,
                vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            ))
        );
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some("(use crate::foo)".into()),
                            description: Some("async fn(&mut Option<T>) -> Vec<T>".to_string()),
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                0..5,
                vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            ))
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
            Some(CodeLabel::new(
                "len: usize".to_string(),
                0..3,
                vec![(0..3, highlight_field), (5..10, highlight_type),],
            ))
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello(…)".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some("(use crate::foo)".to_string()),
                            description: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                        }),

                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                0..5,
                vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            ))
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FUNCTION),
                        label: "hello".to_string(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: Some("(use crate::foo)".to_string()),
                            description: Some("fn(&mut Option<T>) -> Vec<T>".to_string()),
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "hello(&mut Option<T>) -> Vec<T> (use crate::foo)".to_string(),
                0..5,
                vec![
                    (0..5, highlight_function),
                    (7..10, highlight_keyword),
                    (11..17, highlight_type),
                    (18..19, highlight_type),
                    (25..28, highlight_type),
                    (29..30, highlight_type),
                ],
            ))
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::METHOD),
                        label: "await.as_deref_mut()".to_string(),
                        filter_text: Some("as_deref_mut".to_string()),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: None,
                            description: Some("fn(&mut self) -> IterMut<'_, T>".to_string()),
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "await.as_deref_mut(&mut self) -> IterMut<'_, T>".to_string(),
                6..18,
                vec![
                    (6..18, HighlightId(2)),
                    (20..23, HighlightId(1)),
                    (33..40, HighlightId(0)),
                    (45..46, HighlightId(0))
                ],
            ))
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::METHOD),
                        label: "as_deref_mut()".to_string(),
                        filter_text: Some("as_deref_mut".to_string()),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: None,
                            description: Some(
                                "pub fn as_deref_mut(&mut self) -> IterMut<'_, T>".to_string()
                            ),
                        }),
                        ..Default::default()
                    },
                    &language
                )
                .await,
            Some(CodeLabel::new(
                "pub fn as_deref_mut(&mut self) -> IterMut<'_, T>".to_string(),
                7..19,
                vec![
                    (0..3, HighlightId(1)),
                    (4..6, HighlightId(1)),
                    (7..19, HighlightId(2)),
                    (21..24, HighlightId(1)),
                    (34..41, HighlightId(0)),
                    (46..47, HighlightId(0))
                ],
            ))
        );

        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::FIELD),
                        label: "inner_value".to_string(),
                        filter_text: Some("value".to_string()),
                        detail: Some("String".to_string()),
                        ..Default::default()
                    },
                    &language,
                )
                .await,
            Some(CodeLabel::new(
                "inner_value: String".to_string(),
                6..11,
                vec![(0..11, HighlightId(3)), (13..19, HighlightId(0))],
            ))
        );

        // Snippet with insert tabstop (empty placeholder)
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::SNIPPET),
                        label: "println!".to_string(),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::default(),
                            new_text: "println!(\"$1\", $2)$0".to_string(),
                        })),
                        ..Default::default()
                    },
                    &language,
                )
                .await,
            Some(CodeLabel::new(
                "println!(\"…\", …)".to_string(),
                0..8,
                vec![
                    (10..13, HighlightId::TABSTOP_INSERT_ID),
                    (16..19, HighlightId::TABSTOP_INSERT_ID),
                    (0..7, HighlightId(2)),
                    (7..8, HighlightId(2)),
                ],
            ))
        );

        // Snippet with replace tabstop (placeholder with default text)
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::SNIPPET),
                        label: "vec!".to_string(),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::default(),
                            new_text: "vec![${1:elem}]$0".to_string(),
                        })),
                        ..Default::default()
                    },
                    &language,
                )
                .await,
            Some(CodeLabel::new(
                "vec![elem]".to_string(),
                0..4,
                vec![
                    (5..9, HighlightId::TABSTOP_REPLACE_ID),
                    (0..3, HighlightId(2)),
                    (3..4, HighlightId(2)),
                ],
            ))
        );

        // Snippet with tabstop appearing more than once
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::SNIPPET),
                        label: "if let".to_string(),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::default(),
                            new_text: "if let ${1:pat} = $1 {\n    $0\n}".to_string(),
                        })),
                        ..Default::default()
                    },
                    &language,
                )
                .await,
            Some(CodeLabel::new(
                "if let pat = … {\n    \n}".to_string(),
                0..6,
                vec![
                    (7..10, HighlightId::TABSTOP_REPLACE_ID),
                    (13..16, HighlightId::TABSTOP_INSERT_ID),
                    (0..2, HighlightId(1)),
                    (3..6, HighlightId(1)),
                ],
            ))
        );

        // Snippet with tabstops not in left-to-right order
        assert_eq!(
            adapter
                .label_for_completion(
                    &lsp::CompletionItem {
                        kind: Some(lsp::CompletionItemKind::SNIPPET),
                        label: "for".to_string(),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::default(),
                            new_text: "for ${2:item} in ${1:iter} {\n    $0\n}".to_string(),
                        })),
                        ..Default::default()
                    },
                    &language,
                )
                .await,
            Some(CodeLabel::new(
                "for item in iter {\n    \n}".to_string(),
                0..3,
                vec![
                    (4..8, HighlightId::TABSTOP_REPLACE_ID),
                    (12..16, HighlightId::TABSTOP_REPLACE_ID),
                    (0..3, HighlightId(1)),
                    (9..11, HighlightId(1)),
                ],
            ))
        );

        // Postfix completion without actual tabstops (only implicit final $0)
        // The label should use completion.label so it can be filtered by "ref"
        let ref_completion = adapter
            .label_for_completion(
                &lsp::CompletionItem {
                    kind: Some(lsp::CompletionItemKind::SNIPPET),
                    label: "ref".to_string(),
                    filter_text: Some("ref".to_string()),
                    label_details: Some(CompletionItemLabelDetails {
                        detail: None,
                        description: Some("&expr".to_string()),
                    }),
                    detail: Some("&expr".to_string()),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        range: lsp::Range::default(),
                        new_text: "&String::new()".to_string(),
                    })),
                    ..Default::default()
                },
                &language,
            )
            .await;
        assert!(
            ref_completion.is_some(),
            "ref postfix completion should have a label"
        );
        let ref_label = ref_completion.unwrap();
        let filter_text = &ref_label.text[ref_label.filter_range.clone()];
        assert!(
            filter_text.contains("ref"),
            "filter range text '{filter_text}' should contain 'ref' for filtering to work",
        );

        // Test for correct range calculation with mixed empty and non-empty tabstops.(See https://github.com/zed-industries/zed/issues/44825)
        let res = adapter
            .label_for_completion(
                &lsp::CompletionItem {
                    kind: Some(lsp::CompletionItemKind::STRUCT),
                    label: "Particles".to_string(),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        range: lsp::Range::default(),
                        new_text: "Particles { pos_x: $1, pos_y: $2, vel_x: $3, vel_y: $4, acc_x: ${5:()}, acc_y: ${6:()}, mass: $7 }$0".to_string(),
                    })),
                    ..Default::default()
                },
                &language,
            )
            .await
            .unwrap();

        assert_eq!(
            res,
            CodeLabel::new(
                "Particles { pos_x: …, pos_y: …, vel_x: …, vel_y: …, acc_x: (), acc_y: (), mass: … }".to_string(),
                0..9,
                vec![
                    (19..22, HighlightId::TABSTOP_INSERT_ID),
                    (31..34, HighlightId::TABSTOP_INSERT_ID),
                    (43..46, HighlightId::TABSTOP_INSERT_ID),
                    (55..58, HighlightId::TABSTOP_INSERT_ID),
                    (67..69, HighlightId::TABSTOP_REPLACE_ID),
                    (78..80, HighlightId::TABSTOP_REPLACE_ID),
                    (88..91, HighlightId::TABSTOP_INSERT_ID),
                    (0..9, highlight_type),
                    (60..65, highlight_field),
                    (71..76, highlight_field),
                ],
            )
        );
    }

    #[gpui::test]
    async fn test_rust_label_for_symbol() {
        let adapter = Arc::new(RustLspAdapter);
        let language = language("rust", tree_sitter_rust::LANGUAGE.into());
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
            Some(CodeLabel::new(
                "fn hello".to_string(),
                3..8,
                vec![(0..2, highlight_keyword), (3..8, highlight_function)],
            ))
        );

        assert_eq!(
            adapter
                .label_for_symbol("World", lsp::SymbolKind::TYPE_PARAMETER, &language)
                .await,
            Some(CodeLabel::new(
                "type World".to_string(),
                5..10,
                vec![(0..4, highlight_keyword), (5..10, highlight_type)],
            ))
        );

        assert_eq!(
            adapter
                .label_for_symbol("zed", lsp::SymbolKind::PACKAGE, &language)
                .await,
            Some(CodeLabel::new(
                "extern crate zed".to_string(),
                13..16,
                vec![(0..6, highlight_keyword), (7..12, highlight_keyword),],
            ))
        );
    }

    #[gpui::test]
    async fn test_rust_autoindent(cx: &mut TestAppContext) {
        // cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });

        let language = crate::language("rust", tree_sitter_rust::LANGUAGE.into());

        cx.new(|cx| {
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
    fn test_target_info_from_metadata() {
        for (input, absolute_path, expected) in [
            (
                r#"{"packages":[{"id":"path+file:///absolute/path/to/project/zed/crates/zed#0.131.0","manifest_path":"/path/to/zed/Cargo.toml","targets":[{"name":"zed","kind":["bin"],"src_path":"/path/to/zed/src/main.rs"}]}]}"#,
                "/path/to/zed/src/main.rs",
                Some((
                    Some(TargetInfo {
                        package_name: "zed".into(),
                        target_name: "zed".into(),
                        required_features: Vec::new(),
                        target_kind: TargetKind::Bin,
                    }),
                    Arc::from("/path/to/zed".as_ref()),
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","manifest_path":"/path/to/custom-package/Cargo.toml","targets":[{"name":"my-custom-bin","kind":["bin"],"src_path":"/path/to/custom-package/src/main.rs"}]}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((
                    Some(TargetInfo {
                        package_name: "my-custom-package".into(),
                        target_name: "my-custom-bin".into(),
                        required_features: Vec::new(),
                        target_kind: TargetKind::Bin,
                    }),
                    Arc::from("/path/to/custom-package".as_ref()),
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","targets":[{"name":"my-custom-bin","kind":["example"],"src_path":"/path/to/custom-package/src/main.rs"}],"manifest_path":"/path/to/custom-package/Cargo.toml"}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((
                    Some(TargetInfo {
                        package_name: "my-custom-package".into(),
                        target_name: "my-custom-bin".into(),
                        required_features: Vec::new(),
                        target_kind: TargetKind::Example,
                    }),
                    Arc::from("/path/to/custom-package".as_ref()),
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","manifest_path":"/path/to/custom-package/Cargo.toml","targets":[{"name":"my-custom-bin","kind":["example"],"src_path":"/path/to/custom-package/src/main.rs","required-features":["foo","bar"]}]}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((
                    Some(TargetInfo {
                        package_name: "my-custom-package".into(),
                        target_name: "my-custom-bin".into(),
                        required_features: vec!["foo".to_owned(), "bar".to_owned()],
                        target_kind: TargetKind::Example,
                    }),
                    Arc::from("/path/to/custom-package".as_ref()),
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","targets":[{"name":"my-custom-bin","kind":["example"],"src_path":"/path/to/custom-package/src/main.rs","required-features":[]}],"manifest_path":"/path/to/custom-package/Cargo.toml"}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((
                    Some(TargetInfo {
                        package_name: "my-custom-package".into(),
                        target_name: "my-custom-bin".into(),
                        required_features: vec![],
                        target_kind: TargetKind::Example,
                    }),
                    Arc::from("/path/to/custom-package".as_ref()),
                )),
            ),
            (
                r#"{"packages":[{"id":"path+file:///path/to/custom-package#my-custom-package@0.1.0","targets":[{"name":"my-custom-package","kind":["lib"],"src_path":"/path/to/custom-package/src/main.rs"}],"manifest_path":"/path/to/custom-package/Cargo.toml"}]}"#,
                "/path/to/custom-package/src/main.rs",
                Some((None, Arc::from("/path/to/custom-package".as_ref()))),
            ),
        ] {
            let metadata: CargoMetadata = serde_json::from_str(input).context(input).unwrap();

            let absolute_path = Path::new(absolute_path);

            assert_eq!(target_info_from_metadata(metadata, absolute_path), expected);
        }
    }

    #[test]
    fn test_rust_test_fragment() {
        #[track_caller]
        fn check(
            variables: impl IntoIterator<Item = (VariableName, &'static str)>,
            path: &str,
            expected: &str,
        ) {
            let path = Path::new(path);
            let found = test_fragment(
                &TaskVariables::from_iter(variables.into_iter().map(|(k, v)| (k, v.to_owned()))),
                path,
                path.file_stem().unwrap().to_str().unwrap(),
            );
            assert_eq!(expected, found);
        }

        check([], "/project/src/lib.rs", "--lib");
        check([], "/project/src/foo/mod.rs", "foo");
        check(
            [
                (RUST_BIN_KIND_TASK_VARIABLE.clone(), "bin"),
                (RUST_BIN_NAME_TASK_VARIABLE, "x"),
            ],
            "/project/src/main.rs",
            "--bin=x",
        );
        check([], "/project/src/main.rs", "--");
    }

    #[test]
    fn test_convert_rust_analyzer_schema() {
        let raw_schema = serde_json::json!([
            {
                "title": "Assist",
                "properties": {
                    "rust-analyzer.assist.emitMustUse": {
                        "markdownDescription": "Insert #[must_use] when generating `as_` methods for enum variants.",
                        "default": false,
                        "type": "boolean"
                    }
                }
            },
            {
                "title": "Assist",
                "properties": {
                    "rust-analyzer.assist.expressionFillDefault": {
                        "markdownDescription": "Placeholder expression to use for missing expressions in assists.",
                        "default": "todo",
                        "type": "string"
                    }
                }
            },
            {
                "title": "Cache Priming",
                "properties": {
                    "rust-analyzer.cachePriming.enable": {
                        "markdownDescription": "Warm up caches on project load.",
                        "default": true,
                        "type": "boolean"
                    }
                }
            }
        ]);

        let converted = RustLspAdapter::convert_rust_analyzer_schema(&raw_schema);

        assert_eq!(
            converted.get("type").and_then(|v| v.as_str()),
            Some("object")
        );

        let properties = converted
            .pointer("/properties")
            .expect("should have properties")
            .as_object()
            .expect("properties should be object");

        assert!(properties.contains_key("assist"));
        assert!(properties.contains_key("cachePriming"));
        assert!(!properties.contains_key("rust-analyzer"));

        let assist_props = properties
            .get("assist")
            .expect("should have assist")
            .pointer("/properties")
            .expect("assist should have properties")
            .as_object()
            .expect("assist properties should be object");

        assert!(assist_props.contains_key("emitMustUse"));
        assert!(assist_props.contains_key("expressionFillDefault"));

        let emit_must_use = assist_props
            .get("emitMustUse")
            .expect("should have emitMustUse");
        assert_eq!(
            emit_must_use.get("type").and_then(|v| v.as_str()),
            Some("boolean")
        );
        assert_eq!(
            emit_must_use.get("default").and_then(|v| v.as_bool()),
            Some(false)
        );

        let cache_priming_props = properties
            .get("cachePriming")
            .expect("should have cachePriming")
            .pointer("/properties")
            .expect("cachePriming should have properties")
            .as_object()
            .expect("cachePriming properties should be object");

        assert!(cache_priming_props.contains_key("enable"));
    }
}
