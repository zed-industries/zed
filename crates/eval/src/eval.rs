mod ids;
mod instance;
mod thread;
mod threads;
mod tool_metrics;

use instance::{
    JudgeOutput, REPOS_DIR, RunOutput, ThreadInstance, WORKTREES_DIR, repo_path_for_url, run_git,
};
pub(crate) use tool_metrics::*;

use ::fs::RealFs;
use anyhow::{Result, anyhow};
use clap::Parser;
use client::{Client, ProxySettings, UserStore};
use collections::HashSet;
use extension::ExtensionHostProxy;
use futures::{StreamExt, future};
use gpui::http_client::{Uri, read_proxy_from_env};
use gpui::{App, AppContext, Application, AsyncApp, Entity, SemanticVersion, UpdateGlobal};
use gpui_tokio::Tokio;
use language::LanguageRegistry;
use language_model::{ConfiguredModel, LanguageModel, LanguageModelRegistry};
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use project::Project;
use project::project_settings::ProjectSettings;
use prompt_store::PromptBuilder;
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use settings::{Settings, SettingsStore};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::usize;
use util::ResultExt as _;

pub const RUNS_DIR: &str = "./crates/eval/runs";

#[derive(Parser, Debug)]
#[command(name = "eval", disable_version_flag = true)]
struct Args {
    /// Runs all examples and threads that contain these substrings. If unspecified, all examples and threads are run.
    #[arg(value_name = "EXAMPLE_SUBSTRING")]
    filter: Vec<String>,
    /// Model to use (default: "claude-3-7-sonnet-latest")
    #[arg(long, default_value = "claude-3-7-sonnet-latest")]
    model: String,
    #[arg(long, value_delimiter = ',')]
    languages: Option<Vec<String>>,
    /// How many times to run each example. Note that this is currently not very efficient as N
    /// worktrees will be created for the examples.
    #[arg(long, default_value = "1")]
    repetitions: u32,
    /// How many times to run the judge on each example run.
    #[arg(long, default_value = "3")]
    judge_repetitions: u32,
    /// Maximum number of examples to run concurrently.
    #[arg(long, default_value = "10")]
    concurrency: usize,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let languages: HashSet<String> = args
        .languages
        .unwrap_or_else(|| vec!["rs".to_string()])
        .into_iter()
        .collect();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());
    let all_threads = threads::all();

    app.run(move |cx| {
        let app_state = init(cx);

        let system_id = ids::get_or_create_id(&ids::eval_system_id_path()).ok();
        let installation_id = ids::get_or_create_id(&ids::eval_installation_id_path()).ok();
        let session_id = uuid::Uuid::new_v4().to_string();

        app_state
            .client
            .telemetry()
            .start(system_id, installation_id, session_id, cx);

        let mut cumulative_tool_metrics = ToolMetrics::default();

        let model_registry = LanguageModelRegistry::read_global(cx);
        let model = find_model("claude-3-7-sonnet-latest", model_registry, cx).unwrap();
        let model_provider_id = model.provider_id();
        let model_provider = model_registry.provider(&model_provider_id).unwrap();

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_default_model(
                Some(ConfiguredModel {
                    provider: model_provider.clone(),
                    model: model.clone(),
                }),
                cx,
            );
        });

        let authenticate_task = model_provider.authenticate(cx);

        cx.spawn(async move |cx| {
            authenticate_task.await.unwrap();

            std::fs::create_dir_all(REPOS_DIR)?;
            std::fs::create_dir_all(WORKTREES_DIR)?;

            let run_id = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

            let run_dir = Path::new(RUNS_DIR).join(&run_id);
            std::fs::create_dir_all(&run_dir)?;

            let mut included = Vec::new();

            const COLORS: [&str; 12] = [
                "\x1b[31m", // Red
                "\x1b[32m", // Green
                "\x1b[33m", // Yellow
                "\x1b[34m", // Blue
                "\x1b[35m", // Magenta
                "\x1b[36m", // Cyan
                "\x1b[91m", // Bright Red
                "\x1b[92m", // Bright Green
                "\x1b[93m", // Bright Yellow
                "\x1b[94m", // Bright Blue
                "\x1b[95m", // Bright Magenta
                "\x1b[96m", // Bright Cyan
            ];

            let mut max_name_width = 0;
            let mut skipped = Vec::new();

            for thread in all_threads {
                let meta = thread.meta();
                if !args.filter.iter().any(|sub| meta.name.contains(sub)) {
                    skipped.push(meta.name);
                    continue;
                }

                if meta.language_server.map_or(false, |language| {
                    !languages.contains(&language.file_extension)
                }) {
                    skipped.push(meta.name);
                    continue;
                }

                // TODO: This creates a worktree per repetition. Ideally these examples should
                // either be run sequentially on the same worktree, or reuse worktrees when there
                // are more examples to run than the concurrency limit.
                for repetition_number in 0..args.repetitions {
                    let thread_instance =
                        ThreadInstance::new(thread.clone(), &run_dir, repetition_number);

                    let name_len = meta.name.len();
                    if name_len > max_name_width {
                        max_name_width = meta.name.len();
                    }

                    included.push(thread_instance);
                }
            }

            println!("Skipped threads: {}\n", skipped.join(", "));

            if included.is_empty() {
                eprintln!("Filter matched no examples");
                return cx.update(|cx| cx.quit());
            }

            let mut repo_urls = HashSet::default();
            let mut clone_tasks = Vec::new();

            for (i, thread_instance) in included.iter_mut().enumerate() {
                let color = COLORS[i % COLORS.len()].to_string();
                thread_instance.set_log_prefix_style(&color, max_name_width);

                println!(
                    "{}Logging to: {}",
                    thread_instance.log_prefix,
                    thread_instance.run_directory.display()
                );

                let repo_url = thread_instance.repo_url();
                if repo_urls.insert(repo_url.clone()) {
                    let repo_path = repo_path_for_url(&repo_url);

                    if !repo_path.join(".git").is_dir() {
                        println!(
                            "{:<width$} < {}",
                            "↓ Cloning",
                            repo_url,
                            width = max_name_width
                        );

                        let git_task = cx.spawn(async move |_cx| {
                            std::fs::create_dir_all(&repo_path)?;
                            run_git(&repo_path, &["init"]).await?;
                            run_git(&repo_path, &["remote", "add", "origin", &repo_url]).await
                        });

                        clone_tasks.push(git_task);
                    } else {
                        println!(
                            "{:<width$}  < {}",
                            "✔︎ Already cloned",
                            repo_url,
                            width = max_name_width
                        );

                        let actual_origin =
                            run_git(&repo_path, &["remote", "get-url", "origin"]).await?;
                        if actual_origin != repo_url {
                            return Err(anyhow!(
                                "remote origin {} does not match expected origin {}",
                                actual_origin,
                                repo_url,
                            ));
                        }
                    }
                }
            }

            future::join_all(clone_tasks).await;

            for thread_instance in included.iter_mut() {
                thread_instance.setup().await?;
            }

            let judge_repetitions = args.judge_repetitions;
            let concurrency = args.concurrency;

            let tasks = included.into_iter().map(|thread| {
                let app_state = app_state.clone();
                let model = model.clone();
                let run_id = run_id.clone();
                cx.spawn(async move |cx| {
                    let result = async {
                        let run_output = cx
                            .update(|cx| thread.run(model.clone(), app_state.clone(), cx))?
                            .await?;
                        let judge_tasks = (0..judge_repetitions).map(|round| {
                            run_judge_repetition(
                                run_id.clone(),
                                thread.clone(),
                                model.clone(),
                                &run_output,
                                round,
                                cx,
                            )
                        });
                        let judge_outputs = future::join_all(judge_tasks).await;
                        anyhow::Ok((run_output, judge_outputs))
                    }
                    .await;
                    (thread, result)
                })
            });

            let results = futures::stream::iter(tasks)
                .buffer_unordered(concurrency)
                .collect::<Vec<_>>()
                .await;

            println!("\n\n");
            print_header("EVAL RESULTS");

            let mut diff_scores = Vec::new();
            let mut thread_scores = Vec::new();
            let mut error_count = 0;

            for (instance, result) in results {
                print_header(&instance.name);

                match result {
                    Err(err) => {
                        println!("💥 {}{:?}", instance.log_prefix, err);
                        error_count += 1;
                    }
                    Ok((run_output, judge_results)) => {
                        cumulative_tool_metrics.merge(&run_output.tool_metrics);

                        let any_judge_results = judge_results.iter().any(|result| {
                            result
                                .iter()
                                .any(|output| output.diff.is_some() || output.thread.is_some())
                        });

                        if any_judge_results {
                            println!("┌───────┬──────┬────────┐");
                            println!("│ Judge │ Diff │ Thread │");
                            println!("├───────┼──────┼────────┤");

                            for (i, judge_result) in judge_results.iter().enumerate() {
                                match judge_result {
                                    Ok(judge_output) => {
                                        let diff_display =
                                            if let Some(diff) = judge_output.diff.as_ref() {
                                                diff_scores.push(diff.score);
                                                format!("{}", diff.score)
                                            } else {
                                                "N/A".to_string()
                                            };

                                        let thread_display =
                                            if let Some(instance) = &judge_output.thread {
                                                let thread_score = instance.score;
                                                thread_scores.push(thread_score);
                                                format!("{}", thread_score)
                                            } else {
                                                "N/A".to_string()
                                            };

                                        println!(
                                            "|{:^7}│{:^6}│{:^8}│",
                                            i + 1,
                                            diff_display,
                                            thread_display
                                        );
                                    }
                                    Err(err) => {
                                        println!(
                                            "|{:^7}│{:^6}│{:^8}│{:?}",
                                            i + 1,
                                            "N/A",
                                            "N/A",
                                            err
                                        );
                                    }
                                }
                            }

                            println!("└───────┴──────┴────────┘");
                        }

                        println!("{}", run_output.tool_metrics);

                        if !run_output.assertions.success.is_empty()
                            || !run_output.assertions.failure.is_empty()
                        {
                            println!("");
                            println!("┌────────────────────────────────────────────┬───────────┐");
                            println!("│ Assertion                                  │ Result    │");
                            println!("├────────────────────────────────────────────┼───────────┤");

                            // Print successful assertions
                            for assertion in &run_output.assertions.success {
                                println!(
                                    "│ {:<42} │ {}  │",
                                    truncate_assertion(assertion),
                                    "\x1b[32m✔︎ Passed\x1b[0m"
                                );
                            }

                            // Print failed assertions
                            for assertion in &run_output.assertions.failure {
                                println!(
                                    "│ {:<42} │ {}  │",
                                    truncate_assertion(assertion),
                                    "\x1b[31m✗ Failed\x1b[0m"
                                );
                            }

                            println!("└────────────────────────────────────────────┴───────────┘");
                        }
                    }
                }
                println!(
                    "{}    > {}",
                    " ".repeat(max_name_width),
                    instance.run_directory.display()
                );
            }

            let diff_score_count = diff_scores.len();
            let average_diff_score = diff_scores
                .into_iter()
                .map(|score| score as f32)
                .sum::<f32>()
                / (diff_score_count as f32);

            if error_count > 0 {
                println!("\n{error_count} examples failed to run!");
            }

            if diff_score_count > 0 {
                println!("\nAverage code diff score: {average_diff_score}");
            }

            let thread_score_count = thread_scores.len();

            // We might have gotten no thread scores if we weren't asked to judge the thread.
            if thread_score_count > 0 {
                let average_thread_score = thread_scores
                    .into_iter()
                    .map(|score| score as f32)
                    .sum::<f32>()
                    / (thread_score_count as f32);

                if diff_score_count > 0 {
                    println!("\nAverage thread score: {average_thread_score}");
                }
            }

            print_header("CUMULATIVE TOOL METRICS");
            println!("{}", cumulative_tool_metrics);

            std::thread::sleep(std::time::Duration::from_secs(2));

            app_state.client.telemetry().flush_events();

            cx.update(|cx| cx.quit())
        })
        .detach_and_log_err(cx);
    });
}

/// Subset of `workspace::AppState` needed by `HeadlessAssistant`, with additional fields.
pub struct AgentAppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Entity<UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub node_runtime: NodeRuntime,

    // Additional fields not present in `workspace::AppState`.
    pub prompt_builder: Arc<PromptBuilder>,
}

pub fn init(cx: &mut App) -> Arc<AgentAppState> {
    release_channel::init(SemanticVersion::default(), cx);
    gpui_tokio::init(cx);

    let mut settings_store = SettingsStore::new(cx);
    settings_store
        .set_default_settings(settings::default_settings().as_ref(), cx)
        .unwrap();
    cx.set_global(settings_store);
    client::init_settings(cx);

    // Set User-Agent so we can download language servers from GitHub
    let user_agent = format!(
        "Zed/{} ({}; {})",
        AppVersion::global(cx),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();
    let proxy_url = proxy_str
        .as_ref()
        .and_then(|input| input.parse::<Uri>().ok())
        .or_else(read_proxy_from_env);
    let http = {
        let _guard = Tokio::handle(cx).enter();

        ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
            .expect("could not start HTTP client")
    };
    cx.set_http_client(Arc::new(http));

    Project::init_settings(cx);

    let client = Client::production(cx);
    cx.set_http_client(client.http_client().clone());

    let git_binary_path = None;
    let fs = Arc::new(RealFs::new(
        git_binary_path,
        cx.background_executor().clone(),
    ));

    let mut languages = LanguageRegistry::new(cx.background_executor().clone());
    languages.set_language_server_download_dir(paths::languages_dir().clone());
    let languages = Arc::new(languages);

    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));

    extension::init(cx);

    let (tx, rx) = async_watch::channel(None);
    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = &ProjectSettings::get_global(cx).node;
        let options = NodeBinaryOptions {
            allow_path_lookup: !settings.ignore_system_version.unwrap_or_default(),
            allow_binary_download: true,
            use_paths: settings.path.as_ref().map(|node_path| {
                let node_path = PathBuf::from(shellexpand::tilde(node_path).as_ref());
                let npm_path = settings
                    .npm_path
                    .as_ref()
                    .map(|path| PathBuf::from(shellexpand::tilde(&path).as_ref()));
                (
                    node_path.clone(),
                    npm_path.unwrap_or_else(|| {
                        let base_path = PathBuf::new();
                        node_path.parent().unwrap_or(&base_path).join("npm")
                    }),
                )
            }),
        };
        tx.send(Some(options)).log_err();
    })
    .detach();
    let node_runtime = NodeRuntime::new(client.http_client().clone(), rx);

    let extension_host_proxy = ExtensionHostProxy::global(cx);

    language::init(cx);
    language_extension::init(extension_host_proxy.clone(), languages.clone());
    language_model::init(client.clone(), cx);
    language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);
    languages::init(languages.clone(), node_runtime.clone(), cx);
    assistant_tools::init(client.http_client().clone(), cx);
    context_server::init(cx);
    prompt_store::init(cx);
    let stdout_is_a_pty = false;
    let prompt_builder = PromptBuilder::load(fs.clone(), stdout_is_a_pty, cx);
    agent::init(fs.clone(), client.clone(), prompt_builder.clone(), cx);

    SettingsStore::update_global(cx, |store, cx| {
        store.set_user_settings(include_str!("../runner_settings.json"), cx)
    })
    .unwrap();

    Arc::new(AgentAppState {
        languages,
        client,
        user_store,
        fs,
        node_runtime,
        prompt_builder,
    })
}

pub fn find_model(
    model_name: &str,
    model_registry: &LanguageModelRegistry,
    cx: &App,
) -> anyhow::Result<Arc<dyn LanguageModel>> {
    let model = model_registry
        .available_models(cx)
        .find(|model| model.id().0 == model_name);

    let Some(model) = model else {
        return Err(anyhow!(
            "No language model named {} was available. Available models: {}",
            model_name,
            model_registry
                .available_models(cx)
                .map(|model| model.id().0.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    };

    Ok(model)
}

pub async fn get_current_commit_id(repo_path: &Path) -> Option<String> {
    (run_git(repo_path, &["rev-parse", "HEAD"]).await).ok()
}

fn truncate_assertion(assertion: &str) -> String {
    const MAX_WIDTH: usize = 42;
    if assertion.len() <= MAX_WIDTH {
        assertion.to_string()
    } else {
        format!("{}...", &assertion[..MAX_WIDTH - 3])
    }
}

pub fn get_current_commit_id_sync(repo_path: &Path) -> String {
    futures::executor::block_on(async {
        get_current_commit_id(repo_path).await.unwrap_or_default()
    })
}

async fn run_judge_repetition(
    run_id: String,
    instance: ThreadInstance,
    model: Arc<dyn LanguageModel>,
    run_output: &RunOutput,
    round: u32,
    cx: &AsyncApp,
) -> Result<JudgeOutput> {
    let judge_result = instance.judge(model.clone(), &run_output, round, cx).await;

    if let Ok(judge_output) = &judge_result {
        let path = std::path::Path::new(".");
        let commit_id = get_current_commit_id(path).await.unwrap_or_default();

        telemetry::event!(
            "Agent Eval Completed",
            cohort_id = run_id,
            example_name = instance.name.clone(),
            round = round,
            diff_score = judge_output.diff.clone().map(|diff| diff.score),
            diff_analysis = judge_output.diff.clone().map(|diff| diff.analysis),
            thread_score = judge_output.thread.clone().map(|thread| thread.score),
            thread_analysis = judge_output.thread.clone().map(|thread| thread.analysis),
            tool_metrics = run_output.tool_metrics,
            response_count = run_output.response_count,
            token_usage = run_output.token_usage,
            model = model.telemetry_id(),
            model_provider = model.provider_id().to_string(),
            repository_url = instance.repo_url(),
            repository_revision = instance.revision(),
            diagnostics_before = run_output.diagnostics_before,
            diagnostics_after = run_output.diagnostics_after,
            commit_id = commit_id
        );
    }

    judge_result
}

fn print_header(header: &str) {
    println!("\n========================================");
    println!("{:^40}", header);
    println!("========================================\n");
}
