mod example;
mod ids;
mod tool_metrics;

pub(crate) use example::*;
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
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::ResultExt as _;

#[derive(Parser, Debug)]
#[command(name = "eval", disable_version_flag = true)]
struct Args {
    /// Runs all examples that contain these substrings. If unspecified, all examples are run.
    #[arg(value_name = "EXAMPLE_SUBSTRING")]
    examples: Vec<String>,
    /// Model to use (default: "claude-3-7-sonnet-latest")
    #[arg(long, default_value = "claude-3-7-sonnet-latest")]
    model: String,
    #[arg(long, value_delimiter = ',', default_value = "rs,ts")]
    languages: Vec<String>,
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

    let system_id = ids::get_or_create_id(&ids::eval_system_id_path()).ok();
    let installation_id = ids::get_or_create_id(&ids::eval_installation_id_path()).ok();
    let session_id = uuid::Uuid::new_v4().to_string();
    let run_timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let run_id = match env::var("GITHUB_RUN_ID") {
        Ok(run_id) => format!("github/{}", run_id),
        Err(_) => format!("local/{}", run_timestamp),
    };

    let root_dir = Path::new(std::env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let eval_crate_dir = root_dir.join("crates/eval");
    let repos_dir = eval_crate_dir.join("repos");
    let worktrees_dir = eval_crate_dir.join("worktrees");
    let examples_dir = eval_crate_dir.join("examples");
    let runs_dir = eval_crate_dir.join("runs");
    let run_dir = runs_dir.join(format!("{}", run_timestamp));
    std::fs::create_dir_all(&run_dir).unwrap();
    std::fs::create_dir_all(&repos_dir).unwrap();
    std::fs::create_dir_all(&worktrees_dir).unwrap();
    std::fs::create_dir_all(&examples_dir).unwrap();
    std::fs::create_dir_all(&paths::config_dir()).unwrap();

    let zed_commit_sha = commit_sha_for_path(root_dir);
    let zed_branch_name = git_branch_for_path(root_dir);
    let args = Args::parse();
    let all_available_examples = list_all_examples(&examples_dir).unwrap();

    let example_paths = all_available_examples
        .iter()
        .filter_map(|example_path| {
            let name = example_path.file_name()?.to_string_lossy();
            if args.examples.is_empty()
                || args
                    .examples
                    .iter()
                    .any(|name_substring| name.contains(name_substring))
            {
                Some(example_path.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    app.run(move |cx| {
        let app_state = init(cx);

        let telemetry = app_state.client.telemetry();
        telemetry.start(system_id, installation_id, session_id, cx);

        let enable_telemetry = env::var("ZED_EVAL_TELEMETRY").map_or(false, |value| value == "1")
            && telemetry.has_checksum_seed();
        if enable_telemetry {
            println!("Telemetry enabled");
            telemetry::event!(
                "Agent Eval Started",
                zed_commit_sha = zed_commit_sha,
                zed_branch_name = zed_branch_name,
                run_id = run_id,
            );
        }

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

            let mut examples = Vec::new();

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

            for example_path in &example_paths {
                let example = Example::load_from_directory(
                    example_path,
                    &run_dir,
                    &worktrees_dir,
                    &repos_dir,
                )?;

                if !example
                    .base
                    .language_extension
                    .as_ref()
                    .map_or(false, |lang| args.languages.contains(lang))
                {
                    skipped.push(example.name);
                    continue;
                }

                // TODO: This creates a worktree per repetition. Ideally these examples should
                // either be run sequentially on the same worktree, or reuse worktrees when there
                // are more examples to run than the concurrency limit.
                for repetition_number in 0..args.repetitions {
                    let mut example = example.clone();
                    example.set_repetition_number(repetition_number);

                    let name_len = example.name.len();
                    if name_len > max_name_width {
                        max_name_width = example.name.len();
                    }

                    examples.push(example);
                }
            }

            println!("Skipped examples: {}\n", skipped.join(", "));

            if examples.is_empty() {
                eprintln!("Filter matched no examples");
                return cx.update(|cx| cx.quit());
            }

            let mut repo_urls = HashSet::default();
            let mut clone_tasks = Vec::new();

            for (i, example) in examples.iter_mut().enumerate() {
                let color = COLORS[i % COLORS.len()].to_string();
                example.set_log_prefix_style(&color, max_name_width);

                println!(
                    "{}Logging to: {}",
                    example.log_prefix,
                    example.example_output_directory().display()
                );

                let repo_url = example.base.url.clone();
                if repo_urls.insert(repo_url.clone()) {
                    let repo_path = example.repo_path.clone();

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

            for example in examples.iter_mut() {
                example.setup().await?;
            }

            let judge_repetitions = args.judge_repetitions;
            let concurrency = args.concurrency;

            let tasks = examples.iter().map(|example| {
                let app_state = app_state.clone();
                let model = model.clone();
                let example = example.clone();
                let zed_commit_sha = zed_commit_sha.clone();
                let zed_branch_name = zed_branch_name.clone();
                let run_id = run_id.clone();
                cx.spawn(async move |cx| {
                    let result = async {
                        let run_output = cx
                            .update(|cx| example.run(model.clone(), app_state.clone(), cx))?
                            .await?;
                        let judge_tasks = (0..judge_repetitions).map(|round| {
                            run_judge_repetition(
                                example.clone(),
                                model.clone(),
                                &zed_commit_sha,
                                &zed_branch_name,
                                &run_id,
                                &run_output,
                                round,
                                enable_telemetry,
                                cx,
                            )
                        });
                        let judge_outputs = future::join_all(judge_tasks).await;
                        anyhow::Ok((run_output, judge_outputs))
                    }
                    .await;
                    (example, result)
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

            for (example, result) in results {
                print_header(&example.name);

                match result {
                    Err(err) => {
                        println!("💥 {}{:?}", example.log_prefix, err);
                        error_count += 1;
                    }
                    Ok((run_output, judge_results)) => {
                        cumulative_tool_metrics.merge(&run_output.tool_metrics);

                        println!("┌───────┬──────┬────────┐");
                        println!("│ Judge │ Diff │ Thread │");
                        println!("├───────┼──────┼────────┤");

                        for (i, judge_result) in judge_results.iter().enumerate() {
                            match judge_result {
                                Ok(judge_output) => {
                                    let diff_score = judge_output.diff.score;
                                    diff_scores.push(diff_score);

                                    let thread_display = if let Some(thread) = &judge_output.thread
                                    {
                                        let thread_score = thread.score;
                                        thread_scores.push(thread_score);
                                        format!("{}", thread_score)
                                    } else {
                                        "N/A".to_string()
                                    };

                                    println!(
                                        "|{:^7}│{:^6}│{:^8}│",
                                        i + 1,
                                        diff_score,
                                        thread_display
                                    );
                                }
                                Err(err) => {
                                    println!("|{:^7}│{:^6}│{:^8}│{:?}", i + 1, "N/A", "N/A", err);
                                }
                            }
                        }

                        println!("└───────┴──────┴────────┘");

                        println!("{}", run_output.tool_metrics);
                    }
                }
                println!(
                    "{}    > {}",
                    " ".repeat(max_name_width),
                    example.example_output_directory().display()
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

            app_state.client.telemetry().flush_events().await;

            cx.update(|cx| cx.quit())
        })
        .detach_and_log_err(cx);
    });
}

fn list_all_examples(examples_dir: &Path) -> Result<Vec<PathBuf>> {
    let path = std::fs::canonicalize(examples_dir).unwrap();
    let entries = std::fs::read_dir(path).unwrap();
    let mut result_paths = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result_paths.push(path);
        }
    }
    Ok(result_paths)
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

pub fn commit_sha_for_path(repo_path: &Path) -> String {
    futures::executor::block_on(run_git(repo_path, &["rev-parse", "HEAD"])).unwrap()
}

pub fn git_branch_for_path(repo_path: &Path) -> String {
    match std::env::var("GITHUB_REF_NAME") {
        Ok(branch) => branch,
        Err(_) => {
            futures::executor::block_on(run_git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"]))
                .unwrap_or_else(|_| "unknown".to_string())
        }
    }
}

async fn run_judge_repetition(
    example: Example,
    model: Arc<dyn LanguageModel>,
    zed_commit_sha: &str,
    zed_branch_name: &str,
    run_id: &str,
    run_output: &RunOutput,
    round: u32,
    enable_telemetry: bool,
    cx: &AsyncApp,
) -> Result<JudgeOutput> {
    let judge_output = example.judge(model.clone(), &run_output, round, cx).await;

    let diff_evaluation;
    let thread_diff_evaluation;
    if let Ok(output) = judge_output.as_ref() {
        diff_evaluation = Some(output.diff.clone());
        thread_diff_evaluation = output.thread.clone();
    } else {
        diff_evaluation = None;
        thread_diff_evaluation = None;
    }

    if enable_telemetry {
        telemetry::event!(
            "Agent Example Evaluated",
            zed_commit_sha = zed_commit_sha,
            zed_branch_name = zed_branch_name,
            run_id = run_id,
            example_name = example.name.clone(),
            round = round,
            diff_evaluation = diff_evaluation,
            thread_evaluation = thread_diff_evaluation,
            tool_metrics = run_output.tool_metrics,
            response_count = run_output.response_count,
            token_usage = run_output.token_usage,
            model = model.telemetry_id(),
            model_provider = model.provider_id().to_string(),
            repository_url = example.base.url.clone(),
            repository_revision = example.base.revision.clone(),
            diagnostics_before = run_output.diagnostics_before,
            diagnostics_after = run_output.diagnostics_after,
        );
    }

    judge_output
}

fn print_header(header: &str) {
    println!("\n========================================");
    println!("{:^40}", header);
    println!("========================================\n");
}
