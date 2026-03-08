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
//!   - `result.json`  — structured result with status, timing, and token usage
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
use agent_client_protocol as acp;
use anyhow::{Context, Result};
use clap::Parser;
use feature_flags::FeatureFlagAppExt as _;

use futures::{FutureExt, select_biased};
use gpui::{AppContext as _, AsyncApp, Entity, UpdateGlobal};
use language_model::{LanguageModelRegistry, SelectedModel};
use project::Project;
use settings::SettingsStore;

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

    /// Instruction/prompt text. If omitted, read from --instruction-file or stdin.
    #[arg(long)]
    instruction: Option<String>,

    /// Language model to use, in `provider/model` format.
    #[arg(long, default_value = "anthropic/claude-sonnet-4-6-latest")]
    model: String,

    /// Maximum wall-clock time in seconds for the agent run.
    #[arg(long)]
    timeout: Option<u64>,

    /// Directory for output artifacts (result.json, thread.md, thread.json).
    #[arg(long, default_value = "/logs/agent")]
    output_dir: PathBuf,
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
}

const EXIT_OK: i32 = 0;
const EXIT_ERROR: i32 = 1;
const EXIT_TIMEOUT: i32 = 2;
const EXIT_INTERRUPTED: i32 = 3;

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

    let http_client = Arc::new(reqwest_client::ReqwestClient::new());
    let app = gpui_platform::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = headless::init(cx);
        cx.set_staff(true);

        let auth_tasks = LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry
                .providers()
                .iter()
                .map(|p| p.authenticate(cx))
                .collect::<Vec<_>>()
        });

        let model_name = args.model.clone();
        let timeout = args.timeout;

        cx.spawn(async move |cx| {
            futures::future::join_all(auth_tasks).await;

            let start = Instant::now();

            let (outcome, token_usage) = run_agent(
                &app_state,
                &workdir,
                &instruction,
                &model_name,
                timeout,
                Some(&output_dir),
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

fn read_instruction(args: &Args) -> Result<String> {
    let text = if let Some(text) = &args.instruction {
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
    Ok(text)
}

async fn run_agent(
    app_state: &Arc<AgentCliAppState>,
    workdir: &std::path::Path,
    instruction: &str,
    model_name: &str,
    timeout: Option<u64>,
    output_dir: Option<&std::path::Path>,
    cx: &mut AsyncApp,
) -> (Result<AgentOutcome>, Option<language_model::TokenUsage>) {
    let setup_result: Result<()> = cx.update(|cx| {
        let selected = SelectedModel::from_str(model_name).map_err(|e| anyhow::anyhow!("{e}"))?;
        let registry = LanguageModelRegistry::global(cx);
        let model = registry
            .read(cx)
            .available_models(cx)
            .find(|m| m.id() == selected.model && m.provider_id() == selected.provider)
            .ok_or_else(|| {
                let available = registry
                    .read(cx)
                    .available_models(cx)
                    .map(|m| format!("{}/{}", m.provider_id().0, m.id().0))
                    .collect::<Vec<_>>()
                    .join(", ");
                anyhow::anyhow!("Model {model_name} not found. Available: {available}")
            })?;

        let supports_thinking = model.supports_thinking();

        registry.update(cx, |registry, cx| {
            registry.set_default_model(
                Some(language_model::ConfiguredModel {
                    provider: registry
                        .provider(&model.provider_id())
                        .context("Provider not found")?,
                    model,
                }),
                cx,
            );
            anyhow::Ok(())
        })?;

        let (enable_thinking, effort) = if supports_thinking {
            (true, "\"high\"")
        } else {
            (false, "null")
        };
        let provider_id = selected.provider.0.to_string();
        let model_id = selected.model.0.to_string();
        SettingsStore::update_global(cx, |store, cx| {
            let settings = format!(
                r#"{{
                    "agent": {{
                        "tool_permissions": {{"default": "allow"}},
                        "default_model": {{
                            "provider": "{provider_id}",
                            "model": "{model_id}",
                            "enable_thinking": {enable_thinking},
                            "effort": {effort}
                        }}
                    }},
                    "autosave": "off",
                    "format_on_save": "off"
                }}"
                "#
            );
            store.set_user_settings(&settings, cx).ok();
        });

        anyhow::Ok(())
    });

    if let Err(e) = setup_result {
        return (Err(e), None);
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
        Err(e) => return (Err(e).context("creating worktree"), None),
    };

    let scan_result = worktree.update(cx, |tree, _cx| {
        tree.as_local()
            .context("expected local worktree")
            .map(|local| local.scan_complete())
    });
    match scan_result {
        Ok(future) => future.await,
        Err(e) => return (Err(e), None),
    };

    let thread_store = cx.new(|cx| ThreadStore::new(cx));
    let agent = match NativeAgent::new(
        project.clone(),
        thread_store,
        Templates::new(),
        None,
        app_state.fs.clone(),
        cx,
    )
    .await
    {
        Ok(a) => a,
        Err(e) => return (Err(e).context("creating agent"), None),
    };

    let connection = Rc::new(NativeAgentConnection(agent.clone()));
    let acp_thread = match cx
        .update(|cx| connection.clone().new_session(project, workdir, cx))
        .await
    {
        Ok(t) => t,
        Err(e) => return (Err(e).context("creating ACP session"), None),
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

    let cumulative_usage = if let Some(thread) = &thread {
        let db_thread = thread.read_with(cx, |thread, cx| thread.to_db(cx));
        let db_thread = db_thread.await;
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

    (outcome, final_usage)
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
