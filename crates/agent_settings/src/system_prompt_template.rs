//! User-overridable `system_prompt.hbs` system prompt template support.
//!
//! When `~/.config/zed/system_prompt.hbs` (or the platform equivalent) exists,
//! it replaces the built-in system prompt template for the native agent. The
//! template is rendered with the full system prompt context and a set of
//! partials; the raw `AGENTS.md` content remains available both as the
//! `user_agents_md` context value and as the [`AGENTS_MD_PARTIAL_NAME`]
//! partial, so existing personal rules keep working.
//!
//! Composition uses the engine's native partial mechanism — no new syntax:
//! `AGENTS.md` is registered under the fixed partial name
//! [`AGENTS_MD_PARTIAL_NAME`], and every other `*.hbs` file under the config
//! directory (recursively) is importable by its relative path without the
//! extension, `/`-separated on every platform (`partials/style.hbs` →
//! `{{> partials/style}}`).
//!
//! Only files in the user-owned global config directory are ever rendered —
//! the same trust level as `settings.json`. Project rules files are
//! deliberately never rendered: a checked-in repository must not be able to
//! condition the session's behavior on its tool set.
//!
//! Load-time validation rejects malformed templates and missing/cyclic
//! partials, but strict-mode unknown variables can only be detected at render
//! time (the full context isn't available when the file is loaded). The
//! renderer therefore falls back to the built-in system prompt on error and
//! surfaces the failure via [`SystemPromptTemplateState::Error`] so the host
//! application can show it with the same UI it uses for settings/keymap
//! errors.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BorrowAppContext, Global, SharedString, Task};
use handlebars::template::TemplateElement;
use handlebars::{Handlebars, RenderError, RenderErrorReason, Template};
use serde::Serialize;
use util::ResultExt as _;

/// File name of the overridable system prompt template in the global config
/// directory.
pub const SYSTEM_PROMPT_TEMPLATE_FILE_NAME: &str = "system_prompt.hbs";

/// Partial name under which the `AGENTS.md` content is registered, so the
/// template can splice it in place with `{{> agents_md}}`.
pub const AGENTS_MD_PARTIAL_NAME: &str = "agents_md";

/// The session context available to tool guidance templates.
#[derive(Serialize)]
pub struct RulesTemplateContext<'a> {
    /// Names of the tools enabled for the session.
    pub available_tools: &'a [SharedString],
    pub model_name: Option<&'a str>,
    /// Today's date, `YYYY-MM-DD`.
    pub date: &'a str,
    pub is_linux: bool,
    pub is_windows: bool,
    pub is_macos: bool,
    /// Whether agent-run terminal commands are wrapped in an OS-level sandbox
    /// for this thread — the same gate the built-in system prompt applies to
    /// its sandbox section.
    pub sandboxing: bool,
}

/// Renders a Handlebars template with the given partials registered on an
/// ad-hoc registry — separate from Zed's embedded templates, so user partials
/// can never collide with built-ins.
pub fn render_template(
    source: &str,
    partials: &BTreeMap<String, String>,
    context: &impl Serialize,
) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("contains", Box::new(contains_helper));
    handlebars.register_helper("join", Box::new(join_helper));
    handlebars.register_helper("array", Box::new(ArrayHelper));
    for (name, content) in partials {
        handlebars.register_partial(name, content.as_str())?;
    }
    let entrypoint = Template::compile(source)?;
    check_partial_graph(&entrypoint, partials)?;
    Ok(handlebars.render_template(source, context)?)
}

/// Renders a tool guidance template with the given partials.
pub fn render_rules_template(
    source: &str,
    partials: &BTreeMap<String, String>,
    context: &RulesTemplateContext,
) -> anyhow::Result<String> {
    render_template(source, partials, context)
}

/// Handlebars renders a missing partial as empty output instead of failing,
/// and expands partials recursively without a depth limit — so a typo'd
/// partial name would silently drop content, and an import cycle would
/// overflow the stack mid-render. Reject both before rendering.
fn check_partial_graph(
    entrypoint: &Template,
    partials: &BTreeMap<String, String>,
) -> Result<(), RenderError> {
    let mut references = Vec::new();
    collect_partial_references(&entrypoint.elements, &mut references);
    let mut visiting = Vec::new();
    let mut done = std::collections::HashSet::new();
    for name in references {
        check_partial(&name, partials, &mut visiting, &mut done)?;
    }
    Ok(())
}

fn check_partial(
    name: &str,
    partials: &BTreeMap<String, String>,
    visiting: &mut Vec<String>,
    done: &mut std::collections::HashSet<String>,
) -> Result<(), RenderError> {
    if done.contains(name) {
        return Ok(());
    }
    if visiting.iter().any(|visiting| visiting == name) {
        let mut chain = visiting.clone();
        chain.push(name.to_string());
        return Err(RenderErrorReason::Other(format!(
            "partial import cycle: {}",
            chain.join(" -> ")
        ))
        .into());
    }
    let Some(content) = partials.get(name) else {
        return Err(RenderErrorReason::Other(format!("unknown partial `{name}`")).into());
    };
    let template = Template::compile(content)
        .map_err(|err| RenderErrorReason::Other(format!("invalid partial `{name}`: {err}")))?;
    let mut references = Vec::new();
    collect_partial_references(&template.elements, &mut references);
    visiting.push(name.to_string());
    for reference in references {
        check_partial(&reference, partials, visiting, done)?;
    }
    visiting.pop();
    done.insert(name.to_string());
    Ok(())
}

fn collect_partial_references(elements: &[TemplateElement], out: &mut Vec<String>) {
    for element in elements {
        match element {
            TemplateElement::PartialExpression(decorator)
            | TemplateElement::PartialBlock(decorator) => {
                // Dynamic partial names (`{{> (which)}}`) can't be checked
                // statically; skip them.
                if let Some(name) = decorator.name.as_name() {
                    out.push(name.to_string());
                }
                if let Some(template) = &decorator.template {
                    collect_partial_references(&template.elements, out);
                }
            }
            TemplateElement::HelperBlock(helper) => {
                if let Some(template) = &helper.template {
                    collect_partial_references(&template.elements, out);
                }
                if let Some(inverse) = &helper.inverse {
                    collect_partial_references(&inverse.elements, out);
                }
            }
            _ => {}
        }
    }
}

/// Handlebars helper for checking if an item is in a list.
/// Also used by the built-in templates in the `agent` crate.
pub fn contains_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let list = h
        .param(0)
        .and_then(|v| v.value().as_array())
        .ok_or_else(|| {
            handlebars::RenderError::from(RenderErrorReason::Other(
                "contains: missing or invalid list parameter".to_string(),
            ))
        })?;
    let query = h.param(1).map(|v| v.value()).ok_or_else(|| {
        handlebars::RenderError::from(RenderErrorReason::Other(
            "contains: missing or invalid query parameter".to_string(),
        ))
    })?;

    if list.contains(query) {
        out.write("true")?;
    }

    Ok(())
}

/// Handlebars helper for joining a list into a string with a separator:
/// `{{join available_tools ", "}}`. Elements render like `{{this}}` would
/// render them. Also used by the built-in templates in the `agent` crate.
pub fn join_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let list = h
        .param(0)
        .and_then(|v| v.value().as_array())
        .ok_or_else(|| {
            handlebars::RenderError::from(RenderErrorReason::Other(
                "join: missing or invalid list parameter".to_string(),
            ))
        })?;
    let separator = h.param(1).and_then(|v| v.value().as_str()).ok_or_else(|| {
        handlebars::RenderError::from(RenderErrorReason::Other(
            "join: missing or invalid separator parameter".to_string(),
        ))
    })?;

    use handlebars::JsonRender as _;
    let joined = list
        .iter()
        .map(|value| value.render())
        .collect::<Vec<_>>()
        .join(separator);
    out.write(&joined)?;

    Ok(())
}

/// Handlebars helper that builds a JSON array from its parameters, letting
/// templates construct a list inline: `{{join (array "a" "b") ", "}}` or
/// `{{#if (contains (array "a" "b") "a")}}`. The crate has no array
/// literal syntax, so this is the only way to define a list in place.
///
/// Implemented via `call_inner` (rather than writing to `Output`) so the
/// result keeps its array type when used as a subexpression — output-writing
/// helpers degrade to strings there. Also used by the built-in templates in
/// the `agent` crate.
#[derive(Clone, Copy)]
pub struct ArrayHelper;

impl handlebars::HelperDef for ArrayHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, RenderError> {
        Ok(handlebars::ScopedJson::Derived(
            handlebars::JsonValue::Array(
                h.params()
                    .iter()
                    .map(|param| param.value().clone())
                    .collect(),
            ),
        ))
    }
}

/// In-memory state of the user-global `system_prompt.hbs` file.
#[derive(Debug, Default, Clone)]
pub enum SystemPromptTemplateState {
    /// The file is missing, empty, or whitespace-only.
    #[default]
    Empty,
    /// The file was loaded and validated successfully.
    Loaded(SystemPromptTemplateSource),
    /// The file exists but could not be read or failed validation; carries
    /// the error message.
    Error(SharedString),
}

/// A validated `system_prompt.hbs` plus its importable partials.
#[derive(Debug, Clone)]
pub struct SystemPromptTemplateSource {
    /// The raw template text.
    pub source: SharedString,
    /// Partial name (relative path without extension) → content, collected
    /// from the other `*.hbs` files in the config directory.
    pub partials: Arc<BTreeMap<String, String>>,
}

/// Global wrapper that owns the current [`SystemPromptTemplateState`] plus the
/// watcher task responsible for keeping it up to date.
pub struct SystemPromptTemplate {
    state: SystemPromptTemplateState,
    _watcher: Task<()>,
}

impl Global for SystemPromptTemplate {}

impl SystemPromptTemplate {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<SystemPromptTemplate>()
    }

    pub fn state(&self) -> &SystemPromptTemplateState {
        &self.state
    }

    /// The validated template source, if loaded.
    pub fn source(&self) -> Option<&SystemPromptTemplateSource> {
        match &self.state {
            SystemPromptTemplateState::Loaded(source) => Some(source),
            SystemPromptTemplateState::Empty | SystemPromptTemplateState::Error(_) => None,
        }
    }
}

/// Initialize the user-global `system_prompt.hbs` watcher.
///
/// Watches the config directory for changes to `system_prompt.hbs` or any
/// sibling `*.hbs` partial and updates the [`SystemPromptTemplate`] global
/// accordingly. The `on_change` callback is invoked on the foreground thread
/// whenever a new load completes, so callers can show or dismiss notifications
/// matching the settings/keymap-error UI.
pub fn init(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&SystemPromptTemplateState, &mut App) + 'static,
) {
    let watcher = spawn_watcher(fs, cx, on_change);
    cx.set_global(SystemPromptTemplate {
        state: SystemPromptTemplateState::default(),
        _watcher: watcher,
    });
}

fn spawn_watcher(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&SystemPromptTemplateState, &mut App) + 'static,
) -> Task<()> {
    let config_dir = paths::system_prompt_template_file()
        .parent()
        .expect("system_prompt.hbs path should have a parent")
        .to_path_buf();

    cx.spawn(async move |cx| {
        let config_dir = fs.canonicalize(&config_dir).await.unwrap_or(config_dir);
        // `events` holds a watcher reference, so registrations outlive this
        // handle; we keep it to register newly discovered subdirectories —
        // directory watches are not recursive on Linux.
        let (events, watcher) = fs.watch(&config_dir, Duration::from_millis(100)).await;
        futures::pin_mut!(events);

        let (mut state, mut scanned_dirs) = load_template_state(&fs).await;
        loop {
            for dir in &scanned_dirs {
                watcher.add(dir).log_err();
            }
            cx.update(|cx| {
                cx.update_global::<SystemPromptTemplate, _>(|template, _| {
                    template.state = state.clone();
                });
                on_change(&state, cx);
            });

            let mut reload = false;
            while !reload {
                let Some(batch) = events.next().await else {
                    // Watcher ended; nothing more to do.
                    return;
                };
                // Any `*.hbs` change under the config dir can affect the
                // entrypoint or the partial set.
                reload = batch.iter().any(|event| {
                    event.path.starts_with(&config_dir)
                        && event.path.extension().and_then(|ext| ext.to_str()) == Some("hbs")
                });
            }
            (state, scanned_dirs) = load_template_state(&fs).await;
        }
    })
}

async fn load_template_state(fs: &Arc<dyn Fs>) -> (SystemPromptTemplateState, Vec<PathBuf>) {
    let (partials, scanned_dirs) = load_partials(fs.as_ref()).await;
    let state = 'state: {
        let raw = match fs.load(paths::system_prompt_template_file()).await {
            Ok(raw) => raw,
            Err(err) => {
                if let Some(io_err) = err.downcast_ref::<std::io::Error>()
                    && io_err.kind() == std::io::ErrorKind::NotFound
                {
                    break 'state SystemPromptTemplateState::Empty;
                }
                break 'state SystemPromptTemplateState::Error(SharedString::from(format!(
                    "{err:#}"
                )));
            }
        };
        if raw.trim().is_empty() {
            break 'state SystemPromptTemplateState::Empty;
        }
        match validate_template(&raw, &partials) {
            Ok(()) => SystemPromptTemplateState::Loaded(SystemPromptTemplateSource {
                source: SharedString::from(raw),
                partials: Arc::new(partials),
            }),
            Err(err) => SystemPromptTemplateState::Error(SharedString::from(format!("{err:#}"))),
        }
    };
    (state, scanned_dirs)
}

/// Collects the importable partials: every `*.hbs` file under the config
/// directory except the top-level `system_prompt.hbs` entrypoint, named by its
/// relative path without the extension, `/`-separated on every platform
/// (handlebars template syntax cannot contain `\`). Also returns every
/// directory that was scanned, so the watcher can register them.
async fn load_partials(fs: &dyn Fs) -> (BTreeMap<String, String>, Vec<PathBuf>) {
    let mut partials = BTreeMap::new();
    let mut dirs = Vec::new();
    let Some(config_dir) = paths::system_prompt_template_file().parent() else {
        return (partials, dirs);
    };
    let Ok(items) = fs::read_dir_items(fs, config_dir).await else {
        return (partials, dirs);
    };
    for (path, is_dir) in items {
        if is_dir {
            dirs.push(path);
            continue;
        }
        let Ok(relative) = path.strip_prefix(config_dir) else {
            continue;
        };
        if relative == Path::new(SYSTEM_PROMPT_TEMPLATE_FILE_NAME) {
            continue;
        }
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
                partials.insert(name.to_string(), content);
            }
            Err(err) => {
                log::warn!(
                    "Failed to load system prompt partial {}: {err:#}",
                    path.display()
                );
            }
        }
    }
    (partials, dirs)
}

/// Rejects syntax errors, missing partials, and partial import cycles at load
/// time. Strict-mode unknown-variable errors can only be caught when the full
/// session context is available, so those fall through to the renderer's
/// built-in-template fallback.
fn validate_template(source: &str, partials: &BTreeMap<String, String>) -> anyhow::Result<()> {
    let entrypoint = Template::compile(source)?;
    check_partial_graph(&entrypoint, partials)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(available_tools: &[SharedString]) -> RulesTemplateContext<'_> {
        RulesTemplateContext {
            available_tools,
            model_name: None,
            date: "",
            is_linux: false,
            is_windows: false,
            is_macos: false,
            sandboxing: false,
        }
    }

    #[test]
    fn test_join_helper() {
        let tools = vec![SharedString::from("grep"), SharedString::from("read_file")];
        let rendered = render_rules_template(
            "{{join available_tools \", \"}}",
            &BTreeMap::new(),
            &context(&tools),
        )
        .unwrap();
        assert_eq!(rendered, "grep, read_file");
    }

    #[test]
    fn test_join_helper_empty_list() {
        let rendered = render_rules_template(
            "{{join available_tools \", \"}}",
            &BTreeMap::new(),
            &context(&[]),
        )
        .unwrap();
        assert_eq!(rendered, "");
    }

    #[test]
    fn test_join_helper_invalid_parameters() {
        let tools = vec![SharedString::from("grep")];
        // Non-list first parameter.
        assert!(
            render_rules_template("{{join date \", \"}}", &BTreeMap::new(), &context(&tools))
                .is_err()
        );
        // Missing separator.
        assert!(
            render_rules_template(
                "{{join available_tools}}",
                &BTreeMap::new(),
                &context(&tools)
            )
            .is_err()
        );
    }

    #[test]
    fn test_array_helper_inline_list() {
        let rendered = render_rules_template(
            "{{join (array \"linux\" \"windows\" \"macos\") \" / \"}}",
            &BTreeMap::new(),
            &context(&[]),
        )
        .unwrap();
        assert_eq!(rendered, "linux / windows / macos");
    }

    #[test]
    fn test_array_helper_composes_with_contains() {
        let rendered = render_rules_template(
            "{{#if (contains (array \"grep\" \"read_file\") \"grep\")}}yes{{else}}no{{/if}}",
            &BTreeMap::new(),
            &context(&[]),
        )
        .unwrap();
        assert_eq!(rendered, "yes");
    }

    #[test]
    fn test_array_helper_mixed_with_context_values() {
        let tools = vec![SharedString::from("grep")];
        let rendered = render_rules_template(
            "{{#each (array \"shell\" \"glob\")}}{{this}},{{/each}}{{#each available_tools}}{{this}}{{/each}}",
            &BTreeMap::new(),
            &context(&tools),
        )
        .unwrap();
        assert_eq!(rendered, "shell,glob,grep");
    }
}
