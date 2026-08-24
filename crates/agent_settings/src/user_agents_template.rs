//! User-global `AGENTS.hbs` rules template support.
//!
//! When `~/.config/zed/AGENTS.hbs` (or the platform equivalent) exists, it is
//! rendered as a Handlebars template with the session context
//! ([`RulesTemplateContext`]) and the rendered markdown replaces the verbatim
//! `AGENTS.md` injection in the native agent's system prompt. This lets users
//! gate sections of their personal rules on the tools a session actually has
//! (`{{#if (contains available_tools 'es')}}...{{/if}}`) instead of
//! documenting tools the model cannot call.
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
//! Failure model: a template that fails to validate (syntax error, unknown
//! variable in strict mode, missing partial) is treated as if the file were
//! absent — the caller falls back to verbatim `AGENTS.md` — and the error is
//! exposed via [`UserAgentsTemplateState::Error`] so the host application can
//! surface it with the same UI it uses for settings/keymap errors.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BorrowAppContext, Global, SharedString, Task};
use handlebars::template::TemplateElement;
use handlebars::{Handlebars, RenderError, Template};
use serde::Serialize;
use util::ResultExt as _;

/// File name of the rules template entrypoint in the global config directory.
pub const AGENTS_TEMPLATE_FILE_NAME: &str = "AGENTS.hbs";

/// Partial name under which the `AGENTS.md` content is registered, so the
/// template can splice it in place with `{{> agents_md}}`.
pub const AGENTS_MD_PARTIAL_NAME: &str = "agents_md";

/// The content written to `AGENTS.hbs` when the user materializes it from the
/// agent menu. Its body is just `{{> agents_md}}` plus documentation comments
/// (stripped at render time), so materializing it never changes the prompt.
pub const DEFAULT_AGENTS_TEMPLATE: &str = include_str!("default_agents_template.hbs");

/// The session context available to `AGENTS.hbs` and tool guidance templates.
/// Mirrors the tool/platform context of the built-in system prompt template.
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

/// Renders a rules template with the given partials registered on an ad-hoc
/// registry — separate from Zed's embedded templates, so user partials can
/// never collide with built-ins.
pub fn render_rules_template(
    source: &str,
    partials: &BTreeMap<String, String>,
    context: &RulesTemplateContext,
) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("contains", Box::new(contains_helper));
    for (name, content) in partials {
        handlebars.register_partial(name, content.as_str())?;
    }
    let entrypoint = Template::compile(source)?;
    check_partial_graph(&entrypoint, partials)?;
    Ok(handlebars.render_template(source, context)?)
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
        return Err(RenderError::new(format!(
            "partial import cycle: {}",
            chain.join(" -> ")
        )));
    }
    let Some(content) = partials.get(name) else {
        return Err(RenderError::new(format!("unknown partial `{name}`")));
    };
    let template = Template::compile(content)
        .map_err(|err| RenderError::new(format!("invalid partial `{name}`: {err}")))?;
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

/// Renders the user's `AGENTS.hbs`, additionally registering the `AGENTS.md`
/// content under [`AGENTS_MD_PARTIAL_NAME`].
pub fn render_user_agents_template(
    source: &UserAgentsTemplateSource,
    agents_md: Option<&str>,
    context: &RulesTemplateContext,
) -> anyhow::Result<String> {
    let mut partials = (*source.partials).clone();
    // Registered last so the real `AGENTS.md` wins over a user partial that
    // happens to use the same stem.
    partials.insert(
        AGENTS_MD_PARTIAL_NAME.to_string(),
        agents_md.unwrap_or_default().to_string(),
    );
    render_rules_template(&source.source, &partials, context)
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
            handlebars::RenderError::new("contains: missing or invalid list parameter")
        })?;
    let query = h.param(1).map(|v| v.value()).ok_or_else(|| {
        handlebars::RenderError::new("contains: missing or invalid query parameter")
    })?;

    if list.contains(query) {
        out.write("true")?;
    }

    Ok(())
}

/// In-memory state of the user-global `AGENTS.hbs` file.
#[derive(Debug, Default, Clone)]
pub enum UserAgentsTemplateState {
    /// The file is missing, empty, or whitespace-only.
    #[default]
    Empty,
    /// The file was loaded and validated successfully.
    Loaded(UserAgentsTemplateSource),
    /// The file exists but could not be read or failed validation; carries
    /// the error message.
    Error(SharedString),
}

/// A validated `AGENTS.hbs` plus its importable partials.
#[derive(Debug, Clone)]
pub struct UserAgentsTemplateSource {
    /// The raw template text.
    pub source: SharedString,
    /// Partial name (file stem) → content, collected from the other `*.hbs`
    /// files in the config directory.
    pub partials: Arc<BTreeMap<String, String>>,
}

/// How the materialized `AGENTS.hbs` relates to the built-in default — the
/// three displayable states behind the agent menu's override indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserAgentsTemplateCustomization {
    /// No template file (or a whitespace-only one); plain `AGENTS.md`
    /// behavior applies.
    Absent,
    /// The file content equals the built-in default.
    Default,
    /// The file validates and differs from the built-in default.
    Overridden,
    /// The file exists but fails to read or validate; the session falls back
    /// to `AGENTS.md`.
    Invalid,
}

/// Global wrapper that owns the current [`UserAgentsTemplateState`] plus the
/// watcher task responsible for keeping it up to date.
pub struct UserAgentsTemplate {
    state: UserAgentsTemplateState,
    _watcher: Task<()>,
}

impl Global for UserAgentsTemplate {}

impl UserAgentsTemplate {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<UserAgentsTemplate>()
    }

    pub fn state(&self) -> &UserAgentsTemplateState {
        &self.state
    }

    /// The validated template source, if loaded.
    pub fn source(&self) -> Option<&UserAgentsTemplateSource> {
        match &self.state {
            UserAgentsTemplateState::Loaded(source) => Some(source),
            UserAgentsTemplateState::Empty | UserAgentsTemplateState::Error(_) => None,
        }
    }

    pub fn customization(&self) -> UserAgentsTemplateCustomization {
        match &self.state {
            UserAgentsTemplateState::Empty => UserAgentsTemplateCustomization::Absent,
            UserAgentsTemplateState::Error(_) => UserAgentsTemplateCustomization::Invalid,
            UserAgentsTemplateState::Loaded(source) => {
                if source.source.trim() == DEFAULT_AGENTS_TEMPLATE.trim() {
                    UserAgentsTemplateCustomization::Default
                } else {
                    UserAgentsTemplateCustomization::Overridden
                }
            }
        }
    }
}

/// Initialize the user-global `AGENTS.hbs` watcher.
///
/// Watches the config directory for changes to `AGENTS.hbs` or any sibling
/// `*.hbs` partial and updates the [`UserAgentsTemplate`] global accordingly.
/// The `on_change` callback is invoked on the foreground thread whenever a new
/// load completes, so callers can show or dismiss notifications matching the
/// settings/keymap-error UI.
pub fn init(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&UserAgentsTemplateState, &mut App) + 'static,
) {
    let watcher = spawn_watcher(fs, cx, on_change);
    cx.set_global(UserAgentsTemplate {
        state: UserAgentsTemplateState::default(),
        _watcher: watcher,
    });
}

fn spawn_watcher(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&UserAgentsTemplateState, &mut App) + 'static,
) -> Task<()> {
    let config_dir = paths::agents_template_file()
        .parent()
        .expect("AGENTS.hbs path should have a parent")
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
                cx.update_global::<UserAgentsTemplate, _>(|template, _| {
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

async fn load_template_state(fs: &Arc<dyn Fs>) -> (UserAgentsTemplateState, Vec<PathBuf>) {
    let (partials, scanned_dirs) = load_partials(fs.as_ref()).await;
    let state = 'state: {
        let raw = match fs.load(paths::agents_template_file()).await {
            Ok(raw) => raw,
            Err(err) => {
                if let Some(io_err) = err.downcast_ref::<std::io::Error>()
                    && io_err.kind() == std::io::ErrorKind::NotFound
                {
                    break 'state UserAgentsTemplateState::Empty;
                }
                break 'state UserAgentsTemplateState::Error(SharedString::from(format!(
                    "{err:#}"
                )));
            }
        };
        if raw.trim().is_empty() {
            break 'state UserAgentsTemplateState::Empty;
        }
        match validate_template(&raw, &partials) {
            Ok(()) => UserAgentsTemplateState::Loaded(UserAgentsTemplateSource {
                source: SharedString::from(raw),
                partials: Arc::new(partials),
            }),
            Err(err) => UserAgentsTemplateState::Error(SharedString::from(format!("{err:#}"))),
        }
    };
    (state, scanned_dirs)
}

/// Collects the importable partials: every `*.hbs` file under the config
/// directory except the top-level `AGENTS.hbs` entrypoint, named by its
/// relative path without the extension, `/`-separated on every platform
/// (handlebars template syntax cannot contain `\`). Also returns every
/// directory that was scanned, so the watcher can register them.
async fn load_partials(fs: &dyn Fs) -> (BTreeMap<String, String>, Vec<PathBuf>) {
    let mut partials = BTreeMap::new();
    let mut dirs = Vec::new();
    let Some(config_dir) = paths::agents_template_file().parent() else {
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
        if relative == Path::new(AGENTS_TEMPLATE_FILE_NAME) {
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
                log::warn!("Failed to load rules partial {}: {err:#}", path.display());
            }
        }
    }
    (partials, dirs)
}

/// Probe-renders the template so syntax errors, unknown variables (strict
/// mode), and missing partials are reported at load time rather than failing
/// a session's prompt build.
fn validate_template(source: &str, partials: &BTreeMap<String, String>) -> anyhow::Result<()> {
    let mut partials = partials.clone();
    partials.insert(AGENTS_MD_PARTIAL_NAME.to_string(), String::new());
    let probe = RulesTemplateContext {
        available_tools: &[],
        model_name: None,
        date: "",
        is_linux: false,
        is_windows: false,
        is_macos: false,
        sandboxing: false,
    };
    render_rules_template(source, &partials, &probe).map(|_| ())
}
