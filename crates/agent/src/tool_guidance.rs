//! Per-tool guidance templates: a tool's model-facing description.
//!
//! A tool's model-facing description is an embedded Handlebars template
//! `src/tool_guidance/<tool>.hbs`, rendered against the session context
//! ([`agent_settings::ToolGuidanceContext`]) when the completion request is
//! built (`Thread::build_completion_request`) and used as the tool's
//! description, replacing the Rust doc comment. This lets the description
//! condition on the session's tool set, platform, and sandboxing state.
//!
//! Because rendering happens at request-build time and the description rides
//! on the tool's own entry in the request's `tools` array, it reaches the
//! model exactly when the tool itself is available — sub-agents and reduced
//! profiles included — and tools without a guidance template cost nothing
//! (their Rust doc comment is used as-is).

use std::collections::BTreeMap;
use std::sync::LazyLock;

use agent_settings::ToolGuidanceContext;
use gpui::SharedString;
use rust_embed::RustEmbed;
use util::ResultExt as _;

/// Built-in guidance templates, embedded from `src/tool_guidance/*.hbs`.
#[derive(RustEmbed)]
#[folder = "src/tool_guidance"]
#[include = "*.hbs"]
struct BuiltinGuidance;

/// Tool name (template file stem) → template source.
static BUILTIN_GUIDANCE: LazyLock<BTreeMap<String, String>> = LazyLock::new(|| {
    let mut templates = BTreeMap::new();
    for path in BuiltinGuidance::iter() {
        let Some(tool_name) = path.strip_suffix(".hbs") else {
            continue;
        };
        if let Some(source) = BuiltinGuidance::get(&path)
            .and_then(|file| String::from_utf8(file.data.into_owned()).log_err())
        {
            templates.insert(tool_name.to_string(), source);
        }
    }
    templates
});

/// Renders the guidance template for `tool_name` against the session context,
/// returning the tool's full model-facing description. Returns `None` when the
/// tool has no guidance template or the template renders empty. A render
/// failure is logged and returns `None` — a broken template must not break the
/// request build.
pub fn render(tool_name: &str, context: &ToolGuidanceContext) -> Option<SharedString> {
    let source = BUILTIN_GUIDANCE.get(tool_name)?;
    match agent_settings::render_template(source, context) {
        Ok(rendered) => {
            let rendered = rendered.trim();
            if rendered.is_empty() {
                None
            } else {
                Some(SharedString::from(rendered.to_string()))
            }
        }
        Err(error) => {
            log::error!("Failed to render tool guidance for `{tool_name}`: {error:#}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(sandboxing: bool, is_linux: bool, is_macos: bool) -> ToolGuidanceContext<'static> {
        ToolGuidanceContext {
            available_tools: &[],
            model_name: Some("test-model"),
            date: "2026-01-01",
            is_linux,
            is_windows: !is_linux && !is_macos,
            is_macos,
            sandboxing,
        }
    }

    #[test]
    fn test_unknown_tool_has_no_guidance() {
        assert_eq!(render("no_such_tool", &context(true, true, false)), None);
    }

    #[test]
    fn test_fetch_guidance_gated_on_sandboxing() {
        let sandboxed = render("fetch", &context(true, true, false))
            .expect("fetch guidance should render when sandboxing");
        assert!(sandboxed.contains("Fetches a URL and returns the content as Markdown."));
        assert!(sandboxed.contains("granted network access"));

        // The stable description renders unsandboxed too; only the host-grant
        // text is omitted when the runtime paths it describes are disabled.
        let unsandboxed = render("fetch", &context(false, true, false))
            .expect("fetch guidance should render without sandboxing");
        assert!(unsandboxed.contains("Fetches a URL and returns the content as Markdown."));
        assert!(!unsandboxed.contains("granted network access"));
    }

    #[test]
    fn test_create_directory_guidance_gated_on_sandboxing_and_platform() {
        let linux = render("create_directory", &context(true, true, false))
            .expect("create_directory guidance should render when sandboxing on Linux");
        assert!(linux.contains("Creates a new directory"));
        assert!(linux.contains("directory **outside** the project"));
        assert!(linux.contains("`reason`"));
        assert!(linux.contains("The only other supported path outside the project"));

        let macos = render("create_directory", &context(true, false, true))
            .expect("create_directory guidance should render when sandboxing on macOS");
        assert!(macos.contains("directory **outside** the project"));

        // Out-of-project creation grants aren't supported on Windows, so the
        // description must not advertise them there.
        let windows = render("create_directory", &context(true, false, false))
            .expect("create_directory guidance should render on Windows");
        assert!(windows.contains("Creates a new directory"));
        assert!(!windows.contains("directory **outside** the project"));
        assert!(windows.contains("The only supported path outside the project"));

        // Unsandboxed: the stable description renders, without the
        // out-of-project grant behavior.
        let unsandboxed = render("create_directory", &context(false, true, false))
            .expect("create_directory guidance should render without sandboxing");
        assert!(unsandboxed.contains("Creates a new directory"));
        assert!(!unsandboxed.contains("directory **outside** the project"));
        assert!(unsandboxed.contains("The only supported path outside the project"));
    }
}
