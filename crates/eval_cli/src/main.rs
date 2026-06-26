//! Headless CLI binary for running Zed's agent in evaluation/benchmark environments.
//!
//! Designed to work inside containerized environments (like Harbor/termbench) where:
//! - The repository is already checked out at the working directory
//! - The model API key is provided via environment variables
//! - Results are written to an output directory (default: `/logs/agent/`)
//!
//! ## Usage
//!
//! ```text
//! eval-cli --workdir /testbed --model anthropic/claude-sonnet-4-6-latest \
//!          --instruction "Fix the bug described in..." --timeout 600
//! ```
//!
//! ## Output
//!
//! Writes to `--output-dir` (default `/logs/agent/`):
//!   - `result.json`  — structured result with status, timing, token usage,
//!     step count, and tool-call counts (total and per tool)
//!   - `thread.md`    — full conversation as markdown
//!   - `thread.json`  — raw thread state as JSON
//!
//! ## Exit codes
//!
//! | Code | Meaning |
//! |------|---------|
//! | 0    | Agent finished |
//! | 1    | Error (model/auth/runtime failure) |
//! | 2    | Timeout |
//! | 3    | Interrupted (SIGTERM/SIGINT) |

mod headless;

use std::path::PathBuf;
use std::process;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use acp_thread::AgentConnection as _;
use agent::{NativeAgent, NativeAgentConnection, Templates, ThreadStore};
use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context, Result};
use clap::Parser;
use feature_flags::FeatureFlagAppExt as _;

use futures::{FutureExt, select_biased};
use gpui::{AppContext as _, AsyncApp, Entity, UpdateGlobal};
use language_model::{
    ANTHROPIC_PROVIDER_ID, LanguageModel, LanguageModelId, LanguageModelProviderId,
    LanguageModelRegistry, SelectedModel,
};
use project::Project;
use settings::SettingsStore;
use util::path_list::PathList;

use crate::headless::AgentCliAppState;

#[derive(Parser, Debug)]
#[command(
    name = "eval-cli",
    about = "Run Zed's agent headlessly in evaluation/benchmark environments"
)]
struct Args {
    /// Output current environment variables as JSON to stdout.
    /// Used internally by Zed's shell environment capture.
    #[arg(long, hide = true)]
    printenv: bool,

    /// Path to the repository working directory. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    workdir: PathBuf,

    /// Instruction/prompt text. If omitted, read from stdin.
    #[arg(long, allow_hyphen_values = true)]
    instruction: Option<String>,

    /// File containing additional instruction text appended after the task prompt.
    #[arg(long)]
    instruction_suffix_file: Option<PathBuf>,

    /// Language model to use, in `provider/model` format.
    #[arg(long, default_value = "anthropic/claude-sonnet-4-6-latest")]
    model: String,

    /// Maximum wall-clock time in seconds for the agent run.
    #[arg(long)]
    timeout: Option<u64>,

    /// Directory for output artifacts (result.json, thread.md, thread.json).
    #[arg(long, default_value = ".")]
    output_dir: PathBuf,

    /// Disable staff mode (staff mode is enabled by default).
    #[arg(long)]
    no_staff: bool,

    /// Reasoning effort level for models that support thinking (low, medium, high).
    /// Defaults to "high" for thinking-capable models.
    #[arg(long)]
    reasoning_effort: Option<String>,

    /// Enable or disable extended thinking. Defaults to model auto-detection if omitted.
    #[arg(long)]
    thinking: Option<bool>,
}

enum AgentOutcome {
    Completed,
    Timeout { seconds: u64 },
    Interrupted,
}

#[derive(serde::Serialize)]
struct EvalResult {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    duration_secs: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_secs: Option<u64>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_creation_input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_read_input_tokens: Option<u64>,
    /// Number of agent (assistant) turns, i.e. model round-trips in the agentic
    /// loop. Reported as "steps" by the eval harness.
    #[serde(skip_serializing_if = "Option::is_none")]
    step_count: Option<u64>,
    /// Total number of tool calls across all steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_count: Option<u64>,
    /// Tool calls broken down by tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<std::collections::BTreeMap<String, u64>>,
}

/// Per-run statistics collected from the finished thread, written into
/// `result.json` so the post-hoc report can compute success-conditioned metrics.
#[derive(Default)]
struct RunStats {
    token_usage: Option<language_model::TokenUsage>,
    step_count: Option<u64>,
    tool_call_count: Option<u64>,
    tool_calls: Option<std::collections::BTreeMap<String, u64>>,
}

const EXIT_OK: i32 = 0;
const EXIT_ERROR: i32 = 1;
const EXIT_TIMEOUT: i32 = 2;
const EXIT_INTERRUPTED: i32 = 3;
const MODEL_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(30);
const MODEL_DISCOVERY_POLL_INTERVAL: Duration = Duration::from_millis(100);

static TERMINATED: AtomicBool = AtomicBool::new(false);

fn main() {
    let args = Args::parse();

    if args.printenv {
        util::shell_env::print_env();
        return;
    }

    env_logger::init();

    ctrlc::set_handler(|| {
        TERMINATED.store(true, Ordering::SeqCst);
    })
    .expect("failed to set signal handler");

    let instruction = read_instruction(&args).unwrap_or_else(|e| {
        eprintln!("Error reading instruction: {e}");
        process::exit(EXIT_ERROR);
    });

    let workdir = args.workdir.canonicalize().unwrap_or_else(|e| {
        eprintln!("Invalid --workdir {:?}: {e}", args.workdir);
        process::exit(EXIT_ERROR);
    });

    let output_dir = args.output_dir.clone();
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        eprintln!("Error creating output dir {}: {e}", output_dir.display());
        process::exit(EXIT_ERROR);
    }
    let output_dir = output_dir.canonicalize().unwrap_or_else(|e| {
        eprintln!("Invalid --output-dir {:?}: {e}", output_dir);
        process::exit(EXIT_ERROR);
    });

    let http_client = Arc::new(reqwest_client::ReqwestClient::new());
    let app = gpui_platform::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = headless::init(cx);
        cx.set_staff(!args.no_staff);

        // Eval hook: enable additional feature-flag-gated tools (e.g. the LSP
        // navigation tools behind `lsp-tool` / `rename-tool`) so experiments can
        // measure the agent with tools that aren't yet GA. Comma-separated flag
        // names; unset in production.
        if let Ok(raw_flags) = std::env::var("ZED_EVAL_ENABLE_FLAGS") {
            let flags: Vec<String> = raw_flags
                .split(',')
                .map(|flag| flag.trim().to_string())
                .filter(|flag| !flag.is_empty())
                .collect();
            if !flags.is_empty() {
                cx.update_flags(!args.no_staff, flags);
            }
        }

        let openai_compatible_providers_json = openai_compatible_providers_override();
        let anthropic_available_models_json = anthropic_available_models_override();

        let model_name = args.model.clone();
        let timeout = args.timeout;
        let thinking_override = args.thinking;
        let reasoning_effort = args.reasoning_effort.clone();

        cx.spawn(async move |cx| {
            // Each settings change below is applied in its own `cx.update` call (rather than
            // inline in the synchronous body of `app.run`) so that GPUI flushes the resulting
            // `NotifyGlobalObservers` effect before we move on. Without this, the
            // openai_compatible/anthropic provider registration (driven by an
            // `observe_global::<SettingsStore>` callback in language_models::init) wouldn't
            // have run yet by the time we collect `auth_tasks`, so a newly-added provider's
            // `authenticate()` would never get called and it would be permanently stuck
            // unauthenticated.
            if let Some(providers_json) = &openai_compatible_providers_json {
                let result = cx.update(|cx| apply_openai_compatible_providers(providers_json, cx));
                if let Err(e) = result {
                    eprintln!("Error applying {OPENAI_COMPATIBLE_PROVIDERS_ENV}: {e:#}");
                    process::exit(EXIT_ERROR);
                }
            }

            if let Some(models_json) = &anthropic_available_models_json {
                let result = cx.update(|cx| apply_anthropic_available_models(models_json, cx));
                if let Err(e) = result {
                    eprintln!("Error applying {ANTHROPIC_AVAILABLE_MODELS_ENV}: {e:#}");
                    process::exit(EXIT_ERROR);
                }
            }

            let auth_tasks = cx.update(|cx| {
                LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                    registry
                        .providers()
                        .iter()
                        .map(|p| p.authenticate(cx))
                        .collect::<Vec<_>>()
                })
            });
            futures::future::join_all(auth_tasks).await;

            let start = Instant::now();

            let (outcome, stats) = run_agent(
                &app_state,
                &workdir,
                &instruction,
                &model_name,
                timeout,
                thinking_override,
                reasoning_effort.as_deref(),
                Some(&output_dir),
                openai_compatible_providers_json.as_deref(),
                anthropic_available_models_json.as_deref(),
                cx,
            )
            .await;

            let duration = start.elapsed();

            let (status, error, exit_code) = match &outcome {
                Ok(AgentOutcome::Completed) => ("completed".to_string(), None, EXIT_OK),
                Ok(AgentOutcome::Timeout { seconds }) => {
                    eprintln!("Timeout: agent exceeded {seconds}s time limit");
                    ("timeout".to_string(), None, EXIT_TIMEOUT)
                }
                Ok(AgentOutcome::Interrupted) => {
                    eprintln!("Interrupted: received SIGTERM, saved partial output");
                    ("interrupted".to_string(), None, EXIT_INTERRUPTED)
                }
                Err(e) => {
                    eprintln!("Error: {e:#}");
                    ("error".to_string(), Some(format!("{e:#}")), EXIT_ERROR)
                }
            };

            let token_usage = stats.token_usage;
            let result = EvalResult {
                status,
                error,
                duration_secs: duration.as_secs_f64(),
                timeout_secs: timeout,
                model: model_name.clone(),
                input_tokens: token_usage.as_ref().map(|u| u.input_tokens),
                output_tokens: token_usage.as_ref().map(|u| u.output_tokens),
                cache_creation_input_tokens: token_usage
                    .as_ref()
                    .filter(|u| u.cache_creation_input_tokens > 0)
                    .map(|u| u.cache_creation_input_tokens),
                cache_read_input_tokens: token_usage
                    .as_ref()
                    .filter(|u| u.cache_read_input_tokens > 0)
                    .map(|u| u.cache_read_input_tokens),
                step_count: stats.step_count,
                tool_call_count: stats.tool_call_count,
                tool_calls: stats.tool_calls,
            };

            match serde_json::to_string_pretty(&result) {
                Ok(json) => {
                    if let Err(e) = std::fs::write(output_dir.join("result.json"), &json) {
                        eprintln!("Error writing result.json: {e:#}");
                    }
                    eprintln!("[eval-cli] result: {json}");
                }
                Err(e) => eprintln!("Error serializing result: {e:#}"),
            }

            cx.update(|cx| cx.quit());
            process::exit(exit_code);
        })
        .detach();
    });
}

/// Name of the env var carrying a JSON object to merge into
/// `language_models.openai_compatible` user settings before model discovery, in
/// the same shape as Zed's `openai_compatible` settings key (provider id ->
/// `{ "api_url": ..., "available_models": [...] }`). Lets zed-eval route the
/// agent itself through an OpenAI-compatible endpoint (e.g. Baseten) that isn't
/// one of Zed's built-in providers, without hardcoding it into eval-cli.
const OPENAI_COMPATIBLE_PROVIDERS_ENV: &str = "ZED_OPENAI_COMPATIBLE_PROVIDERS";

fn openai_compatible_providers_override() -> Option<String> {
    let raw = std::env::var(OPENAI_COMPATIBLE_PROVIDERS_ENV).ok()?;
    if raw.trim().is_empty() {
        return None;
    }
    Some(raw)
}

fn apply_openai_compatible_providers(providers_json: &str, cx: &mut gpui::App) -> Result<()> {
    let settings = format!(r#"{{"language_models": {{"openai_compatible": {providers_json}}}}}"#);
    SettingsStore::update_global(cx, |store, cx| {
        store.set_user_settings(&settings, cx).result()
    })
    .context("applying openai_compatible provider settings")?;
    Ok(())
}

/// Name of the env var carrying a JSON array to merge into
/// `language_models.anthropic.available_models` user settings before model
/// discovery, in the same shape as Zed's `anthropic.available_models` settings
/// key (a list of `{ "name": ..., "max_tokens": ..., ... }` entries). Lets
/// zed-eval run models that exist on the Anthropic API for the configured
/// key (e.g. early-access-program models) but aren't returned by the live
/// `/v1/models` listing, without hardcoding them into eval-cli.
const ANTHROPIC_AVAILABLE_MODELS_ENV: &str = "ZED_ANTHROPIC_AVAILABLE_MODELS";

fn anthropic_available_models_override() -> Option<String> {
    let raw = std::env::var(ANTHROPIC_AVAILABLE_MODELS_ENV).ok()?;
    if raw.trim().is_empty() {
        return None;
    }
    Some(raw)
}

fn apply_anthropic_available_models(models_json: &str, cx: &mut gpui::App) -> Result<()> {
    let settings =
        format!(r#"{{"language_models": {{"anthropic": {{"available_models": {models_json}}}}}}}"#);
    SettingsStore::update_global(cx, |store, cx| {
        store.set_user_settings(&settings, cx).result()
    })
    .context("applying anthropic available_models settings")?;
    Ok(())
}

fn read_instruction(args: &Args) -> Result<String> {
    let mut text = if let Some(text) = &args.instruction {
        text.clone()
    } else {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("reading instruction from stdin")?;
        buf
    };
    anyhow::ensure!(!text.trim().is_empty(), "instruction is empty");

    if let Some(path) = &args.instruction_suffix_file {
        let suffix = read_instruction_suffix_file(path)?;
        text.push_str("\n\n");
        text.push_str(&suffix);
    }
    Ok(text)
}

fn read_instruction_suffix_file(path: &PathBuf) -> Result<String> {
    let suffix = std::fs::read_to_string(path)
        .with_context(|| format!("reading instruction suffix file {}", path.display()))?;
    let suffix = suffix.trim().to_string();
    anyhow::ensure!(!suffix.is_empty(), "instruction suffix file is empty");
    Ok(suffix)
}

async fn wait_for_model(selected: &SelectedModel, cx: &mut AsyncApp) -> Result<()> {
    let started_at = Instant::now();

    loop {
        let found = cx.update(|cx| find_available_model(selected, cx).is_some());
        if found {
            return Ok(());
        }

        cx.update(|cx| ensure_provider_authenticated(selected, cx))?;

        let selected_provider_has_models = cx.update(|cx| {
            LanguageModelRegistry::global(cx)
                .read(cx)
                .available_models(cx)
                .any(|model| model.provider_id() == selected.provider)
        });
        let should_wait_for_discovery =
            selected.provider == ANTHROPIC_PROVIDER_ID || !selected_provider_has_models;

        if !should_wait_for_discovery || started_at.elapsed() >= MODEL_DISCOVERY_TIMEOUT {
            return Err(cx.update(|cx| model_not_found_error(&selected_model_name(selected), cx)));
        }

        cx.background_executor()
            .timer(MODEL_DISCOVERY_POLL_INTERVAL)
            .await;
    }
}

fn ensure_provider_authenticated(selected: &SelectedModel, cx: &gpui::App) -> Result<()> {
    let registry = LanguageModelRegistry::global(cx);
    let provider = registry
        .read(cx)
        .provider(&selected.provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected.provider.0))?;

    anyhow::ensure!(
        provider.is_authenticated(cx),
        "Provider {} is not authenticated",
        selected.provider.0
    );

    Ok(())
}

fn find_available_model(
    selected: &SelectedModel,
    cx: &gpui::App,
) -> Option<Arc<dyn LanguageModel>> {
    let registry = LanguageModelRegistry::global(cx);
    let models = registry.read(cx).available_models(cx).collect::<Vec<_>>();

    if let Some(model) = models
        .iter()
        .find(|model| model.provider_id() == selected.provider && model.id() == selected.model)
    {
        return Some(model.clone());
    }

    models
        .into_iter()
        .filter(|model| {
            model.provider_id() == selected.provider
                && model_id_matches_selected(&model.provider_id(), &model.id(), &selected.model)
        })
        .max_by(|left, right| left.id().0.to_string().cmp(&right.id().0.to_string()))
}

fn model_id_matches_selected(
    provider_id: &LanguageModelProviderId,
    available: &LanguageModelId,
    selected: &LanguageModelId,
) -> bool {
    if available == selected {
        return true;
    }

    if provider_id != &ANTHROPIC_PROVIDER_ID {
        return false;
    }

    anthropic_model_ids_match(available.0.as_ref(), selected.0.as_ref())
}

fn anthropic_model_ids_match(available: &str, selected: &str) -> bool {
    let available = anthropic_model_alias_base(available);
    let selected = anthropic_model_alias_base(selected);

    available == selected || anthropic_dated_model_id_matches_base(available, selected)
}

fn anthropic_model_alias_base(mut model_id: &str) -> &str {
    if let Some(stripped) = model_id.strip_suffix("-latest") {
        model_id = stripped;
    }
    if let Some(stripped) = model_id.strip_suffix("-thinking") {
        model_id = stripped;
    }
    if let Some(stripped) = model_id.strip_suffix("-1m-context") {
        model_id = stripped;
    }
    model_id
}

fn anthropic_dated_model_id_matches_base(available: &str, selected: &str) -> bool {
    let Some(suffix) = available.strip_prefix(selected) else {
        return false;
    };
    let Some(date) = suffix.strip_prefix('-') else {
        return false;
    };

    date.len() == 8 && date.chars().all(|character| character.is_ascii_digit())
}

fn selected_model_name(selected: &SelectedModel) -> String {
    format!("{}/{}", selected.provider.0, selected.model.0)
}

fn model_not_found_error(model_name: &str, cx: &gpui::App) -> anyhow::Error {
    let available = LanguageModelRegistry::global(cx)
        .read(cx)
        .available_models(cx)
        .map(|model| format!("{}/{}", model.provider_id().0, model.id().0))
        .collect::<Vec<_>>();
    let available = if available.is_empty() {
        "(none)".to_string()
    } else {
        available.join(", ")
    };

    anyhow::anyhow!("Model {model_name} not found. Available: {available}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anthropic_latest_alias_matches_listed_base_model() {
        assert!(model_id_matches_selected(
            &ANTHROPIC_PROVIDER_ID,
            &LanguageModelId("claude-sonnet-4-6".into()),
            &LanguageModelId("claude-sonnet-4-6-latest".into()),
        ));
    }

    #[test]
    fn anthropic_thinking_alias_matches_listed_base_model() {
        assert!(model_id_matches_selected(
            &ANTHROPIC_PROVIDER_ID,
            &LanguageModelId("claude-sonnet-4-6".into()),
            &LanguageModelId("claude-sonnet-4-6-1m-context-thinking-latest".into()),
        ));
    }

    #[test]
    fn anthropic_latest_alias_matches_listed_dated_model() {
        assert!(model_id_matches_selected(
            &ANTHROPIC_PROVIDER_ID,
            &LanguageModelId("claude-sonnet-4-6-20260518".into()),
            &LanguageModelId("claude-sonnet-4-6-latest".into()),
        ));
    }

    #[test]
    fn non_anthropic_models_require_exact_ids() {
        assert!(!model_id_matches_selected(
            &LanguageModelProviderId("other".into()),
            &LanguageModelId("claude-sonnet-4-6".into()),
            &LanguageModelId("claude-sonnet-4-6-latest".into()),
        ));
    }
}

async fn run_agent(
    app_state: &Arc<AgentCliAppState>,
    workdir: &std::path::Path,
    instruction: &str,
    model_name: &str,
    timeout: Option<u64>,
    thinking_override: Option<bool>,
    reasoning_effort: Option<&str>,
    output_dir: Option<&std::path::Path>,
    openai_compatible_providers_json: Option<&str>,
    anthropic_available_models_json: Option<&str>,
    cx: &mut AsyncApp,
) -> (Result<AgentOutcome>, RunStats) {
    let selected = match SelectedModel::from_str(model_name).map_err(|e| anyhow::anyhow!("{e}")) {
        Ok(selected) => selected,
        Err(e) => return (Err(e), RunStats::default()),
    };

    if let Err(e) = wait_for_model(&selected, cx).await {
        return (Err(e), RunStats::default());
    }

    let setup_result: Result<()> = cx.update(|cx| {
        let registry = LanguageModelRegistry::global(cx);
        let model = find_available_model(&selected, cx)
            .ok_or_else(|| model_not_found_error(model_name, cx))?;
        let provider = registry
            .read(cx)
            .provider(&model.provider_id())
            .context("Provider not found")?;

        let supports_thinking = model.supports_thinking();
        let model_id = model.id().0.to_string();

        registry.update(cx, |registry, cx| {
            registry.set_default_model(
                Some(language_model::ConfiguredModel { provider, model }),
                cx,
            );
        });

        let enable_thinking = thinking_override.unwrap_or(supports_thinking);
        let effort = if enable_thinking {
            match reasoning_effort {
                Some(level) => format!("\"{level}\""),
                None => "\"high\"".to_string(),
            }
        } else {
            "null".to_string()
        };
        let provider_id = selected.provider.0.to_string();
        // set_user_settings replaces the whole user settings buffer, so the
        // openai_compatible/anthropic blocks applied earlier (before model
        // discovery) have to be folded back in here, or they would be dropped
        // by this call.
        let mut language_models_fields = Vec::new();
        if let Some(providers_json) = openai_compatible_providers_json {
            language_models_fields.push(format!(r#""openai_compatible": {providers_json}"#));
        }
        if let Some(models_json) = anthropic_available_models_json {
            language_models_fields.push(format!(
                r#""anthropic": {{"available_models": {models_json}}}"#
            ));
        }
        let language_models_settings = format!("{{{}}}", language_models_fields.join(","));
        // Disable specific tools (e.g. `fetch`/`search_web` on air-gapped
        // benchmarks) the canonical way: via the agent profile. The model only
        // sees a built-in tool when the active profile enables it (see
        // Thread::enabled_tools), so we define a dedicated "eval" profile that
        // mirrors the built-in "write" profile minus the disabled tools, and make
        // it the default profile. A fresh profile key is NOT deep-merged against
        // the defaults, so it can't inherit "write"'s tools — the full set is
        // listed explicitly here. Keep WRITE_TOOLS in sync with the "write"
        // profile in assets/settings/default.json.
        let profile_field = {
            let raw = std::env::var("ZED_EVAL_DISABLE_TOOLS").unwrap_or_default();
            let disabled = raw
                .split(',')
                .map(|name| name.trim().to_string())
                .filter(|name| !name.is_empty())
                .collect::<std::collections::HashSet<String>>();
            if disabled.is_empty() {
                String::new()
            } else {
                const WRITE_TOOLS: &[&str] = &[
                    "copy_path",
                    "create_directory",
                    "create_thread",
                    "delete_path",
                    "diagnostics",
                    "apply_code_action",
                    "edit_file",
                    "write_file",
                    "fetch",
                    "find_path",
                    "find_references",
                    "get_code_actions",
                    "go_to_definition",
                    "list_agents_and_models",
                    "list_directory",
                    "move_path",
                    "rename_symbol",
                    "read_file",
                    "grep",
                    "skill",
                    "spawn_agent",
                    "terminal",
                    "search_web",
                ];
                let tools = WRITE_TOOLS
                    .iter()
                    .filter(|tool| !disabled.contains(**tool))
                    .map(|tool| format!(r#""{tool}": true"#))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#","default_profile": "eval", "profiles": {{"eval": {{"name": "Eval", "enable_all_context_servers": true, "tools": {{{tools}}}}}}}"#
                )
            }
        };
        SettingsStore::update_global(cx, |store, cx| {
            let settings = format!(
                r#"{{
                    "language_models": {language_models_settings},
                    "agent": {{
                        "tool_permissions": {{"default": "allow"}},
                        "default_model": {{
                            "provider": "{provider_id}",
                            "model": "{model_id}",
                            "enable_thinking": {enable_thinking},
                            "effort": {effort}
                        }}{profile_field}
                    }},
                    "autosave": "off",
                    "format_on_save": "off"
                }}"
                "#
            );
            store.set_user_settings(&settings, cx).result()
        })
        .context("updating agent settings")?;

        anyhow::Ok(())
    });

    if let Err(e) = setup_result {
        return (Err(e), RunStats::default());
    }

    let project = cx.update(|cx| {
        Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags {
                init_worktree_trust: false,
                ..Default::default()
            },
            cx,
        )
    });

    let worktree = project.update(cx, |project, cx| project.create_worktree(workdir, true, cx));
    let worktree = match worktree.await {
        Ok(w) => w,
        Err(e) => return (Err(e).context("creating worktree"), RunStats::default()),
    };

    let scan_result = worktree.update(cx, |tree, _cx| {
        tree.as_local()
            .context("expected local worktree")
            .map(|local| local.scan_complete())
    });
    match scan_result {
        Ok(future) => future.await,
        Err(e) => return (Err(e), RunStats::default()),
    };

    let output_worktree = match output_dir {
        Some(output_dir) if !output_dir.starts_with(workdir) => {
            let output_worktree = project.update(cx, |project, cx| {
                project.create_worktree(output_dir, true, cx)
            });
            match output_worktree.await {
                Ok(worktree) => Some(worktree),
                Err(e) => {
                    return (
                        Err(e).context("creating output worktree"),
                        RunStats::default(),
                    );
                }
            }
        }
        _ => None,
    };

    if let Some(output_worktree) = output_worktree {
        let scan_result = output_worktree.update(cx, |tree, _cx| {
            tree.as_local()
                .context("expected local output worktree")
                .map(|local| local.scan_complete())
        });
        match scan_result {
            Ok(future) => future.await,
            Err(e) => return (Err(e), RunStats::default()),
        };
    }

    let agent = cx.update(|cx| {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        NativeAgent::new(thread_store, Templates::new(), app_state.fs.clone(), cx)
    });

    let connection = Rc::new(NativeAgentConnection(agent.clone()));
    let acp_thread = match cx
        .update(|cx| {
            connection
                .clone()
                .new_session(project, PathList::new(&[workdir]), cx)
        })
        .await
    {
        Ok(t) => t,
        Err(e) => return (Err(e).context("creating ACP session"), RunStats::default()),
    };

    let _subscription = cx.subscribe(&acp_thread, |acp_thread, event, cx| {
        log_acp_thread_event(&acp_thread, event, cx);
    });

    let message = vec![acp::ContentBlock::Text(acp::TextContent::new(
        instruction.to_string(),
    ))];

    let send_future = acp_thread.update(cx, |acp_thread: &mut acp_thread::AcpThread, cx| {
        acp_thread.send(message, cx)
    });

    let timeout_future = if let Some(timeout_secs) = timeout {
        futures::future::Either::Left(
            cx.background_executor()
                .timer(Duration::from_secs(timeout_secs)),
        )
    } else {
        futures::future::Either::Right(futures::future::pending::<()>())
    };

    let sigterm_future = {
        let executor = cx.background_executor().clone();
        async move {
            while !TERMINATED.load(Ordering::Relaxed) {
                executor.timer(Duration::from_millis(100)).await;
            }
        }
    };

    let outcome = select_biased! {
        result = send_future.fuse() => match result {
            Ok(Some(response)) => {
                eprintln!("[eval-cli] stopped: {:?}", response.stop_reason);
                if response.stop_reason == acp::StopReason::MaxTokens {
                    Err(anyhow::anyhow!("Model hit maximum token limit"))
                } else {
                    Ok(AgentOutcome::Completed)
                }
            }
            Ok(None) => {
                eprintln!("[eval-cli] completed (no response)");
                Ok(AgentOutcome::Completed)
            }
            Err(e) => Err(e).context("agent run failed"),
        },
        _ = sigterm_future.fuse() => {
            eprintln!("[eval-cli] received SIGTERM, cancelling...");
            acp_thread.update(cx, |t: &mut acp_thread::AcpThread, cx| t.cancel(cx)).await;
            Ok(AgentOutcome::Interrupted)
        },
        _ = timeout_future.fuse() => {
            acp_thread.update(cx, |t: &mut acp_thread::AcpThread, cx| t.cancel(cx)).await;
            Ok(AgentOutcome::Timeout { seconds: timeout.unwrap_or(0) })
        }
    };

    let thread = cx.update(|cx| {
        let session_id = acp_thread.read(cx).session_id().clone();
        connection.thread(&session_id, cx)
    });

    let mut step_count = None;
    let mut tool_call_count = None;
    let mut tool_calls = None;
    let cumulative_usage = if let Some(thread) = &thread {
        let db_thread = thread.read_with(cx, |thread, cx| thread.to_db(cx));
        let db_thread = db_thread.await;
        let mut counts = std::collections::BTreeMap::<String, u64>::new();
        let mut agent_turn_count = 0;
        for message in &db_thread.messages {
            let Some(agent_message) = message.as_agent_message() else {
                continue;
            };
            agent_turn_count += 1;
            for request_message in agent_message.to_request() {
                for content in request_message.content {
                    if let language_model::MessageContent::ToolUse(tool_use) = content {
                        *counts.entry(tool_use.name.to_string()).or_default() += 1;
                    }
                }
            }
        }
        step_count = Some(agent_turn_count);
        tool_call_count = Some(counts.values().sum());
        tool_calls = Some(counts);
        let usage = db_thread.cumulative_token_usage;
        if usage.input_tokens > 0 || usage.output_tokens > 0 {
            Some(usage)
        } else {
            None
        }
    } else {
        None
    };

    let acp_usage = cx.update(|cx| {
        acp_thread
            .read(cx)
            .token_usage()
            .map(|usage| language_model::TokenUsage {
                input_tokens: usage.input_tokens,
                output_tokens: usage.output_tokens,
                ..Default::default()
            })
    });

    let final_usage = cumulative_usage.or(acp_usage);

    if let (Some(thread), Some(dir)) = (&thread, output_dir) {
        let markdown = thread.read_with(cx, |thread, _cx| thread.to_markdown());
        if let Err(e) = std::fs::write(dir.join("thread.md"), markdown) {
            eprintln!("Error writing thread.md: {e:#}");
        }

        let db_thread = thread.read_with(cx, |thread, cx| thread.to_db(cx));
        let db_thread = db_thread.await;
        match serde_json::to_string_pretty(&db_thread) {
            Ok(json) => {
                if let Err(e) = std::fs::write(dir.join("thread.json"), json) {
                    eprintln!("Error writing thread.json: {e:#}");
                }
            }
            Err(e) => eprintln!("Error serializing thread.json: {e:#}"),
        }
    }

    (
        outcome,
        RunStats {
            token_usage: final_usage,
            step_count,
            tool_call_count,
            tool_calls,
        },
    )
}

fn log_acp_thread_event(
    acp_thread: &Entity<acp_thread::AcpThread>,
    event: &acp_thread::AcpThreadEvent,
    cx: &mut gpui::App,
) {
    match event {
        acp_thread::AcpThreadEvent::NewEntry => {
            let entries = acp_thread.read(cx).entries();
            if let Some(acp_thread::AgentThreadEntry::AssistantMessage(message)) = entries.last() {
                for chunk in &message.chunks {
                    if let acp_thread::AssistantMessageChunk::Message { block } = chunk {
                        if let acp_thread::ContentBlock::Markdown { markdown } = block {
                            let text = markdown.read(cx).source().to_string();
                            if !text.is_empty() {
                                eprint!("{text}");
                            }
                        }
                    }
                }
            }
        }
        acp_thread::AcpThreadEvent::EntryUpdated(index) => {
            let entries = acp_thread.read(cx).entries();
            if let Some(acp_thread::AgentThreadEntry::ToolCall(tool_call)) = entries.get(*index) {
                if let Some(name) = &tool_call.tool_name {
                    match &tool_call.status {
                        acp_thread::ToolCallStatus::Completed => {
                            eprintln!("[tool] {name} ✓");
                        }
                        acp_thread::ToolCallStatus::Failed => {
                            eprintln!("[tool] {name} ✗");
                        }
                        acp_thread::ToolCallStatus::Rejected => {
                            eprintln!("[tool] {name} rejected");
                        }
                        acp_thread::ToolCallStatus::Canceled => {
                            eprintln!("[tool] {name} canceled");
                        }
                        _ => {}
                    }
                }
            }
        }
        acp_thread::AcpThreadEvent::Stopped(reason) => {
            eprintln!("\n[eval-cli] stopped: {reason:?}");
        }
        acp_thread::AcpThreadEvent::Error => {
            eprintln!("[eval-cli] error event");
        }
        acp_thread::AcpThreadEvent::Retry(status) => {
            eprintln!("[eval-cli] retry: {status:?}");
        }
        acp_thread::AcpThreadEvent::SubagentSpawned(session_id) => {
            eprintln!("[eval-cli] subagent spawned: {session_id}");
        }
        _ => {}
    }
}
