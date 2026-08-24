//! Two-tier tool instructions: the guidance tier.
//!
//! Each tool's model-facing documentation is split into a non-overridable API
//! contract (the schema and description generated from the tool's
//! registration) and overridable default guidance (principles, examples,
//! pitfalls). Guidance for tool `<name>` lives in an embedded default file
//! `src/tool_guidance/<name>.hbs` and can be shadowed — skills-style — by a
//! `<name>.hbs` file in the user-global `tool_guidance` config directory.
//!
//! Guidance is rendered through the same Handlebars engine as the system
//! prompt ([`agent_settings::render_rules_template`]) with the same session
//! context. Every guidance file — built-in or user — is importable from the
//! others as a partial named by its relative path without the extension,
//! `/`-separated (`shared/editing.hbs` → `{{> shared/editing}}`), so shared
//! guidance can be factored out into files and subdirectories. Files in
//! subdirectories never map to a tool name, so they are partial-only.
//!
//! A tool's guidance reaches the model only when the tool itself is
//! available: guidance is appended to the tool's schema description when the
//! completion request is built, so gating needs no separate convention.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use agent_settings::RulesTemplateContext;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BorrowAppContext, Global, SharedString, Task};
use rust_embed::RustEmbed;
use util::ResultExt as _;

/// Built-in default guidance, embedded from `src/tool_guidance/**/*.hbs`.
///
/// The contract/guidance tier split of the existing tool docs has
/// intentionally not been done yet, so this set starts as raw dumps of the
/// current docs; see the `extract_builtin_tool_docs` utility test below for
/// the extraction harness.
#[derive(RustEmbed)]
#[folder = "src/tool_guidance"]
#[include = "*.hbs"]
struct BuiltinGuidance;

/// The built-in default guidance files: partial name (relative path without
/// extension) → content.
static BUILTIN_GUIDANCE: LazyLock<BTreeMap<String, String>> = LazyLock::new(|| {
    let mut files = BTreeMap::new();
    for path in BuiltinGuidance::iter() {
        let Some(name) = path.strip_suffix(".hbs") else {
            continue;
        };
        if let Some(content) = BuiltinGuidance::get(&path)
            .and_then(|content| String::from_utf8(content.data.into_owned()).log_err())
        {
            files.insert(name.to_string(), content);
        }
    }
    files
});

/// The built-in default guidance for a tool, if it has one.
pub fn builtin_guidance(tool_name: &str) -> Option<&'static str> {
    BUILTIN_GUIDANCE.get(tool_name).map(String::as_str)
}

/// The content written to a `tool_guidance/<tool>.hbs` override file when the
/// user materializes it for a tool that has no built-in default yet.
pub fn default_tool_guidance_stub(tool_name: &str) -> String {
    format!(
        "{{{{!--\n\
         Guidance for the `{tool_name}` tool, appended to the tool's model-facing\n\
         description whenever `{tool_name}` is available in the session. Text is\n\
         emitted verbatim; handlebars comments like this one are stripped and never reach\n\
         the model.\n\
         \n\
         This tool has no built-in default guidance — replace this comment with your own.\n\
         Context variables: available_tools, model_name, date, is_windows, is_linux,\n\
         sandboxing. Gate sections with {{{{#if (contains available_tools 'x')}}}}...{{{{/if}}}}.\n\
         Other guidance files here are importable as partials by relative path\n\
         (`shared/tips.hbs` → `{{{{> shared/tips}}}}`, `/`-separated on every platform).\n\
         --}}}}\n"
    )
}

/// How a tool's guidance override relates to the built-in default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolGuidanceOverrideState {
    /// No user override file.
    Absent,
    /// The override file's content equals the built-in default.
    Default,
    /// The override file exists and differs from the built-in default (or
    /// there is no built-in default to compare against).
    Overridden,
}

/// The user-global tool guidance overrides, kept up to date by a watcher.
pub struct ToolGuidanceStore {
    /// Partial name (relative path without extension) → content.
    user_files: BTreeMap<String, String>,
    _watcher: Task<()>,
}

impl Global for ToolGuidanceStore {}

impl ToolGuidanceStore {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<ToolGuidanceStore>()
    }

    pub fn override_state(&self, tool_name: &str) -> ToolGuidanceOverrideState {
        let Some(user_content) = self.user_files.get(tool_name) else {
            return ToolGuidanceOverrideState::Absent;
        };
        match builtin_guidance(tool_name) {
            Some(default) if user_content.trim() == default.trim() => {
                ToolGuidanceOverrideState::Default
            }
            _ => ToolGuidanceOverrideState::Overridden,
        }
    }

    /// Renders the guidance for a single tool, if it has one.
    /// Render failures return `None` rather than failing the request build.
    pub fn render_guidance(
        &self,
        tool_name: &str,
        context: &RulesTemplateContext,
    ) -> Option<SharedString> {
        // User overrides shadow same-named built-in defaults.
        let mut files = BUILTIN_GUIDANCE.clone();
        files.extend(
            self.user_files
                .iter()
                .map(|(name, content)| (name.clone(), content.clone())),
        );

        let source = files.get(tool_name)?;
        match agent_settings::render_rules_template(source, &files, context) {
            Ok(rendered) => {
                let rendered = rendered.trim();
                if rendered.is_empty() {
                    None
                } else {
                    Some(SharedString::from(rendered.to_string()))
                }
            }
            Err(err) => {
                log::error!("Failed to render tool guidance for `{tool_name}`: {err:#}");
                None
            }
        }
    }
}

/// Initialize the tool guidance store by scanning the user-global
/// `tool_guidance` directory for overrides, keeping it up to date as override
/// files change.
pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    if cx.has_global::<ToolGuidanceStore>() {
        return;
    }
    let watcher = spawn_watcher(fs, cx);
    cx.set_global(ToolGuidanceStore {
        user_files: BTreeMap::new(),
        _watcher: watcher,
    });
}

fn spawn_watcher(fs: Arc<dyn Fs>, cx: &mut App) -> Task<()> {
    let guidance_dir = paths::tool_guidance_dir().clone();

    cx.spawn(async move |cx| {
        // `events` holds a watcher reference, so registrations outlive this
        // handle; we keep it to register newly discovered subdirectories —
        // directory watches are not recursive on Linux. `FsWatcher` polls for
        // the directory to appear if it doesn't exist yet.
        let (events, watcher) = fs.watch(&guidance_dir, Duration::from_millis(100)).await;
        futures::pin_mut!(events);

        let (mut user_files, mut scanned_dirs) =
            load_user_overrides(fs.as_ref(), &guidance_dir).await;
        loop {
            for dir in &scanned_dirs {
                watcher.add(dir).log_err();
            }
            cx.update(|cx| {
                cx.update_global::<ToolGuidanceStore, _>(|store, _| {
                    store.user_files = user_files.clone();
                });
            });

            if events.next().await.is_none() {
                // Watcher ended; nothing more to do.
                return;
            }
            (user_files, scanned_dirs) = load_user_overrides(fs.as_ref(), &guidance_dir).await;
        }
    })
}

/// Every `*.hbs` file under the guidance directory, named by its relative
/// path without the extension, `/`-separated on every platform (handlebars
/// template syntax cannot contain `\`). Also returns every directory that was
/// scanned, so the watcher can register them.
async fn load_user_overrides(
    fs: &dyn Fs,
    guidance_dir: &Path,
) -> (BTreeMap<String, String>, Vec<PathBuf>) {
    let mut files = BTreeMap::new();
    let mut dirs = Vec::new();
    let Ok(items) = fs::read_dir_items(fs, guidance_dir).await else {
        return (files, dirs);
    };
    for (path, is_dir) in items {
        if is_dir {
            dirs.push(path);
            continue;
        }
        let Ok(relative) = path.strip_prefix(guidance_dir) else {
            continue;
        };
        let Some(name) = relative.to_str().map(|name| name.replace('\\', "/")) else {
            continue;
        };
        let Some(name) = name.strip_suffix(".hbs") else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        match fs.load(&path).await {
            Ok(content) => {
                files.insert(name.to_string(), content);
            }
            Err(err) => {
                log::warn!("Failed to load tool guidance {}: {err:#}", path.display());
            }
        }
    }
    (files, dirs)
}

#[cfg(test)]
mod tests {
    /// Extraction utility for the contract/guidance tier split: dumps each
    /// built-in tool's current model-facing documentation into
    /// `src/tool_guidance/<tool>.hbs`, where the files become embedded
    /// built-in guidance defaults on the next build.
    ///
    /// Skipped by default; run manually with:
    ///
    /// ```sh
    /// cargo test -p agent extract_builtin_tool_docs -- --ignored
    /// ```
    ///
    /// The dump is a starting point for curation, not the split itself:
    /// deciding what stays in the schema description (the API contract tier)
    /// versus what moves to the guidance file is a per-tool editorial pass,
    /// and doc text containing `{{` must be escaped for Handlebars. Until a
    /// tool's doc comment is slimmed to its contract, extracting it verbatim
    /// would send the same text twice (schema description + guidance
    /// section).
    #[test]
    #[ignore = "extraction utility, not a test"]
    fn extract_builtin_tool_docs() {
        let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tool_guidance");
        for tool in crate::tools::built_in_tools() {
            if tool.description.trim().is_empty() {
                continue;
            }
            let path = out_dir.join(format!("{}.hbs", tool.name));
            std::fs::write(&path, &tool.description)
                .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
        }
    }
}
