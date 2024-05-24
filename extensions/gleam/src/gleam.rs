use std::fs;
use zed::lsp::CompletionKind;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId, SlashCommand};
use zed_extension_api::{self as zed, Result};

struct GleamExtension {
    cached_binary_path: Option<String>,
}

impl GleamExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("gleam") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "gleam-lang/gleam",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "gleam-{version}-{arch}-{os}.tar.gz",
            version = release.version,
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-musl",
                zed::Os::Windows => "pc-windows-msvc",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("gleam-{}", release.version);
        let binary_path = format!("{version_dir}/gleam");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for GleamExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        let name = &completion.label;
        let ty = strip_newlines_from_detail(&completion.detail?);
        let let_binding = "let a";
        let colon = ": ";
        let assignment = " = ";
        let call = match completion.kind? {
            CompletionKind::Function | CompletionKind::Constructor => "()",
            _ => "",
        };
        let code = format!("{let_binding}{colon}{ty}{assignment}{name}{call}");

        Some(CodeLabel {
            spans: vec![
                CodeLabelSpan::code_range({
                    let start = let_binding.len() + colon.len() + ty.len() + assignment.len();
                    start..start + name.len()
                }),
                CodeLabelSpan::code_range({
                    let start = let_binding.len();
                    start..start + colon.len()
                }),
                CodeLabelSpan::code_range({
                    let start = let_binding.len() + colon.len();
                    start..start + ty.len()
                }),
            ],
            filter_range: (0..name.len()).into(),
            code,
        })
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _argument: Option<String>,
        worktree: &zed::Worktree,
    ) -> Result<Option<String>, String> {
        match command.name.as_str() {
            "gleam-project" => {
                let mut message = String::new();
                message.push_str("You are in a Gleam project.\n");

                if let Some(gleam_toml) = worktree.read_text_file("gleam.toml").ok() {
                    message.push_str("The `gleam.toml` is as follows:\n");
                    message.push_str(&gleam_toml);
                }

                Ok(Some(message))
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(GleamExtension);

/// Removes newlines from the completion detail.
///
/// The Gleam LSP can return types containing newlines, which causes formatting
/// issues within the Zed completions menu.
fn strip_newlines_from_detail(detail: &str) -> String {
    let without_newlines = detail
        .replace("->\n  ", "-> ")
        .replace("\n  ", "")
        .replace(",\n", "");

    let comma_delimited_parts = without_newlines.split(',');
    comma_delimited_parts
        .map(|part| part.trim())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use crate::strip_newlines_from_detail;

    #[test]
    fn test_strip_newlines_from_detail() {
        let detail = "fn(\n  Selector(a),\n  b,\n  fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a,\n) -> Selector(a)";
        let expected = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) -> Selector(a)";
        assert_eq!(strip_newlines_from_detail(detail), expected);

        let detail = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) ->\n  Selector(a)";
        let expected = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) -> Selector(a)";
        assert_eq!(strip_newlines_from_detail(detail), expected);

        let detail = "fn(\n  Method,\n  List(#(String, String)),\n  a,\n  Scheme,\n  String,\n  Option(Int),\n  String,\n  Option(String),\n) -> Request(a)";
        let expected = "fn(Method, List(#(String, String)), a, Scheme, String, Option(Int), String, Option(String)) -> Request(a)";
        assert_eq!(strip_newlines_from_detail(&detail), expected);
    }
}
