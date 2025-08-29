mod assertions;
mod example;
mod examples;
mod explorer;
mod ids;
mod instance;
mod tool_metrics;

use assertions::{AssertionsReport, display_error_row};
use instance::{ExampleInstance, JudgeOutput, RunOutput, run_git};
use language_extension::LspAccess;
pub(crate) use tool_metrics::*;

use ::fs::RealFs;
use clap::Parser;
use client::{Client, ProxySettings, UserStore};
use collections::{HashMap, HashSet};
use extension::ExtensionHostProxy;
use futures::future;
use gpui::http_client::read_proxy_from_env;
use gpui::{App, AppContext, Application, AsyncApp, Entity, UpdateGlobal};
use gpui_tokio::Tokio;
use language::LanguageRegistry;
use language_model::{ConfiguredModel, LanguageModel, LanguageModelRegistry, SelectedModel};
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use project::Project;
use project::project_settings::ProjectSettings;
use prompt_store::PromptBuilder;
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use settings::{Settings, SettingsStore};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use util::ResultExt as _;

static CARGO_MANIFEST_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));

#[derive(Parser, Debug)]
#[command(name = "eval", disable_version_flag = true)]
struct Args {
    /// Runs all examples and threads that contain these substrings. If unspecified, all examples and threads are run.
    #[arg(value_name = "EXAMPLE_SUBSTRING")]
    filter: Vec<String>,
    /// provider/model to use for agent
    #[arg(long, default_value = "anthropic/claude-3-7-sonnet-latest")]
    model: String,
    /// provider/model to use for judges
    #[arg(long, default_value = "anthropic/claude-3-7-sonnet-latest")]
    judge_model: String,
    #[arg(long, value_delimiter = ',', default_value = "rs,ts,py")]
    languages: Vec<String>,
    /// How many times to run each example.
    #[arg(long, default_value = "8")]
    repetitions: usize,
    /// Maximum number of examples to run concurrently.
    #[arg(long, default_value = "4")]
    concurrency: usize,
}

fn main() {
    dotenvy::from_filename(CARGO_MANIFEST_DIR.join(".env")).ok();

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
        .unwrap()
        .canonicalize()
        .unwrap();
    let eval_crate_dir = root_dir.join("crates").join("eval");
    let repos_dir = eval_crate_dir.join("repos");
    let worktrees_dir = eval_crate_dir.join("worktrees");
    let examples_dir = eval_crate_dir.join("src").join("examples");
    let run_dir = eval_crate_dir
        .join("runs")
        .join(format!("{}", run_timestamp));
    std::fs::create_dir_all(&run_dir).unwrap();
    std::fs::create_dir_all(&repos_dir).unwrap();
    std::fs::create_dir_all(&worktrees_dir).unwrap();
    std::fs::create_dir_all(&examples_dir).unwrap();
    std::fs::create_dir_all(&paths::config_dir()).unwrap();

    let zed_commit_sha = commit_sha_for_path(&root_dir);
    let zed_branch_name = git_branch_for_path(&root_dir);
    let args = Args::parse();
    let languages: HashSet<String> = args.languages.into_iter().collect();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);
    let all_threads = examples::all(&examples_dir);

    app.run(move |cx| {
        let app_state = init(cx);

        let telemetry = app_state.client.telemetry();
        telemetry.start(system_id, installation_id, session_id, cx);

        let enable_telemetry = env::var("ZED_EVAL_TELEMETRY").is_ok_and(|value| value == "1")
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

        let agent_model = load_model(&args.model, cx).unwrap();
        let judge_model = load_model(&args.judge_model, cx).unwrap();

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_default_model(Some(agent_model.clone()), cx);
        });

        let auth1 = agent_model.provider.authenticate(cx);
        let auth2 = judge_model.provider.authenticate(cx);

        cx.spawn(async move |cx| {
            auth1.await?;
            auth2.await?;

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

            let mut skipped = Vec::new();

            for thread in all_threads {
                let meta = thread.meta();
                if !args.filter.is_empty() && !args.filter.iter().any(|sub| meta.name.contains(sub))
                {
                    skipped.push(meta.name);
                    continue;
                }

                if let Some(language) = meta.language_server
                    && !languages.contains(&language.file_extension) {
                        panic!(
                            "Eval for {:?} could not be run because no language server was found for extension {:?}",
                            meta.name,
                            language.file_extension
                        );
                    }

                // TODO: This creates a worktree per repetition. Ideally these examples should
                // either be run sequentially on the same worktree, or reuse worktrees when there
                // are more examples to run than the concurrency limit.
                for repetition_number in 0..args.repetitions {
                    let example_instance = ExampleInstance::new(
                        thread.clone(),
                        &repos_dir,
                        &run_dir,
                        &worktrees_dir,
                        repetition_number,
                    );

                    examples.push(example_instance);
                }
            }

            if !skipped.is_empty() {
                println!("Skipped threads: {}", skipped.join(", "));
            }

            if examples.is_empty() {
                eprintln!("Filter matched no examples");
                return cx.update(|cx| cx.quit());
            }

            let mut repo_urls = HashSet::default();
            let mut clone_tasks = Vec::new();

            let max_name_width = examples
                .iter()
                .map(|e| e.worktree_name().len())
                .max()
                .unwrap_or(0);

            for (i, example_instance) in examples.iter_mut().enumerate() {
                let color = COLORS[i % COLORS.len()].to_string();
                example_instance.set_log_prefix_style(&color, max_name_width);

                println!(
                    "{}Logging to: {}",
                    example_instance.log_prefix,
                    example_instance.run_directory.display()
                );

                let repo_url = example_instance.repo_url();
                if repo_urls.insert(repo_url.clone()) {
                    let repo_path = example_instance.repo_path.clone();

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
                        anyhow::ensure!(
                            actual_origin == repo_url,
                            "remote origin {actual_origin} does not match expected origin {repo_url}"
                        );
                    }
                }
            }

            future::join_all(clone_tasks).await;

            for example_instance in examples.iter_mut() {
                example_instance.fetch().await?;
            }

            let examples = Rc::new(RefCell::new(VecDeque::from(examples)));
            let results_by_example_name = Rc::new(RefCell::new(HashMap::default()));

            future::join_all((0..args.concurrency).map(|_| {
                let app_state = app_state.clone();
                let model = agent_model.model.clone();
                let judge_model = judge_model.model.clone();
                let zed_commit_sha = zed_commit_sha.clone();
                let zed_branch_name = zed_branch_name.clone();
                let run_id = run_id.clone();
                let examples = examples.clone();
                let results = results_by_example_name.clone();
                cx.spawn(async move |cx| {
                    loop {
                        let Some(mut example) = examples.borrow_mut().pop_front() else {
                            break;
                        };
                        let result = async {
                            example.setup().await?;
                            let run_output = cx
                                .update(|cx| example.run(model.clone(), app_state.clone(), cx))?
                                .await?;
                            let judge_output = judge_example(
                                example.clone(),
                                judge_model.clone(),
                                &zed_commit_sha,
                                &zed_branch_name,
                                &run_id,
                                &run_output,
                                enable_telemetry,
                                cx,
                            )
                            .await;
                            anyhow::Ok((run_output, judge_output))
                        }
                        .await;
                        results
                            .borrow_mut()
                            .entry(example.name.clone())
                            .or_insert(Vec::new())
                            .push((example.clone(), result));
                    }
                })
            }))
            .await;

            print_report(
                &mut results_by_example_name.borrow_mut(),
                &mut cumulative_tool_metrics,
                &run_dir,
            )?;

            app_state.client.telemetry().flush_events().await;

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
    let app_version = AppVersion::load(env!("ZED_PKG_VERSION"));
    release_channel::init(app_version, cx);
    gpui_tokio::init(cx);

    let mut settings_store = SettingsStore::new(cx);
    settings_store
        .set_default_settings(settings::default_settings().as_ref(), cx)
        .unwrap();
    cx.set_global(settings_store);
    client::init_settings(cx);

    // Set User-Agent so we can download language servers from GitHub
    let user_agent = format!(
        "Zed Agent Eval/{} ({}; {})",
        app_version,
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();
    let proxy_url = proxy_str
        .as_ref()
        .and_then(|input| input.parse().ok())
        .or_else(read_proxy_from_env);
    let http = {
        let _guard = Tokio::handle(cx).enter();

        ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
            .expect("could not start HTTP client")
    };
    cx.set_http_client(Arc::new(http));

    Project::init_settings(cx);

    let client = Client::production(cx);
    cx.set_http_client(client.http_client());

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

    let (mut tx, rx) = watch::channel(None);
    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = &ProjectSettings::get_global(cx).node;
        let options = NodeBinaryOptions {
            allow_path_lookup: !settings.ignore_system_version,
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
    let node_runtime = NodeRuntime::new(client.http_client(), None, rx);

    let extension_host_proxy = ExtensionHostProxy::global(cx);

    language::init(cx);
    debug_adapter_extension::init(extension_host_proxy.clone(), cx);
    language_extension::init(LspAccess::Noop, extension_host_proxy, languages.clone());
    language_model::init(client.clone(), cx);
    language_models::init(user_store.clone(), client.clone(), cx);
    languages::init(languages.clone(), node_runtime.clone(), cx);
    prompt_store::init(cx);
    terminal_view::init(cx);
    let stdout_is_a_pty = false;
    let prompt_builder = PromptBuilder::load(fs.clone(), stdout_is_a_pty, cx);
    agent_ui::init(
        fs.clone(),
        client.clone(),
        prompt_builder.clone(),
        languages.clone(),
        true,
        cx,
    );
    assistant_tools::init(client.http_client(), cx);

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
    let selected = SelectedModel::from_str(model_name).map_err(|e| anyhow::anyhow!(e))?;
    model_registry
        .available_models(cx)
        .find(|model| model.id() == selected.model && model.provider_id() == selected.provider)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No language model with ID {}/{} was available. Available models: {}",
                selected.model.0,
                selected.provider.0,
                model_registry
                    .available_models(cx)
                    .map(|model| format!("{}/{}", model.provider_id().0, model.id().0))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

pub fn load_model(model_name: &str, cx: &mut App) -> anyhow::Result<ConfiguredModel> {
    let model = {
        let model_registry = LanguageModelRegistry::read_global(cx);
        find_model(model_name, model_registry, cx)?
    };

    let provider = {
        let model_registry = LanguageModelRegistry::read_global(cx);
        model_registry
            .provider(&model.provider_id())
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", model.provider_id()))?
    };

    Ok(ConfiguredModel {
        provider: provider.clone(),
        model: model.clone(),
    })
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

async fn judge_example(
    example: ExampleInstance,
    model: Arc<dyn LanguageModel>,
    zed_commit_sha: &str,
    zed_branch_name: &str,
    run_id: &str,
    run_output: &RunOutput,
    enable_telemetry: bool,
    cx: &AsyncApp,
) -> JudgeOutput {
    let judge_output = example.judge(model.clone(), run_output, cx).await;

    if enable_telemetry {
        telemetry::event!(
            "Agent Example Evaluated",
            zed_commit_sha = zed_commit_sha,
            zed_branch_name = zed_branch_name,
            run_id = run_id,
            example_name = example.name.clone(),
            example_repetition = example.repetition,
            diff_evaluation = judge_output.diff.clone(),
            thread_evaluation = judge_output.thread,
            tool_metrics = run_output.tool_metrics,
            response_count = run_output.response_count,
            token_usage = run_output.token_usage,
            model = model.telemetry_id(),
            model_provider = model.provider_id().to_string(),
            repository_url = example.repo_url(),
            repository_revision = example.revision(),
            diagnostic_summary_before = run_output.diagnostic_summary_before,
            diagnostic_summary_after = run_output.diagnostic_summary_after,
            diagnostics_before = run_output.diagnostics_before,
            diagnostics_after = run_output.diagnostics_after,
        );
    }

    judge_output
}

const HEADER_WIDTH: usize = 65;

fn print_h1(header: &str) {
    println!("\n\n{:=^HEADER_WIDTH$}", "");
    println!("{:^HEADER_WIDTH$}", header);
    println!("{:=^HEADER_WIDTH$}\n", "");
}

fn print_h2(header: &str) {
    println!("\n{:-^HEADER_WIDTH$}", "");
    println!("{:^HEADER_WIDTH$}", header);
    println!("{:-^HEADER_WIDTH$}\n", "");
}

fn print_report(
    results_by_example_name: &mut HashMap<
        String,
        Vec<(ExampleInstance, anyhow::Result<(RunOutput, JudgeOutput)>)>,
    >,
    cumulative_tool_metrics: &mut ToolMetrics,
    run_dir: &Path,
) -> anyhow::Result<()> {
    print_h1("EVAL RESULTS");

    let mut diff_scores = Vec::new();
    let mut thread_scores = Vec::new();
    let mut programmatic_scores = Vec::new();
    let mut error_count = 0;

    for (example_name, results) in results_by_example_name.iter_mut() {
        print_h2(example_name);

        results.sort_unstable_by_key(|(example, _)| example.repetition);
        let mut example_cumulative_tool_metrics = ToolMetrics::default();

        let mut table_rows = String::new();

        for (example, result) in results.iter() {
            match result {
                Err(err) => {
                    display_error_row(&mut table_rows, example.repetition, err.to_string())?;
                    error_count += 1;
                    programmatic_scores.push(0.0);
                    diff_scores.push(0.0);
                    thread_scores.push(0.0);
                }
                Ok((run_output, judge_output)) => {
                    cumulative_tool_metrics.merge(&run_output.tool_metrics);
                    example_cumulative_tool_metrics.merge(&run_output.tool_metrics);

                    if run_output.programmatic_assertions.total_count() > 0 {
                        for assertion in &run_output.programmatic_assertions.ran {
                            assertions::display_table_row(
                                &mut table_rows,
                                example.repetition,
                                assertion,
                            )?;
                        }

                        programmatic_scores
                            .push(run_output.programmatic_assertions.passed_percentage())
                    }

                    if !judge_output.diff.is_empty() {
                        diff_scores.push(judge_output.diff.passed_percentage());

                        for assertion in &judge_output.diff.ran {
                            assertions::display_table_row(
                                &mut table_rows,
                                example.repetition,
                                assertion,
                            )?;
                        }
                    }

                    if !judge_output.thread.is_empty() {
                        thread_scores.push(judge_output.thread.passed_percentage());

                        for assertion in &judge_output.thread.ran {
                            assertions::display_table_row(
                                &mut table_rows,
                                example.repetition,
                                assertion,
                            )?;
                        }
                    }
                }
            }
        }

        let mut all_asserts = Vec::new();

        if !table_rows.is_empty() {
            assertions::print_table_header();
            print!("{}", table_rows);

            assertions::print_table_divider();

            for (example, result) in results.iter() {
                if let Ok((run_output, judge_output)) = result {
                    let asserts = [
                        run_output.programmatic_assertions.clone(),
                        judge_output.diff.clone(),
                        judge_output.thread.clone(),
                    ];
                    all_asserts.extend_from_slice(&asserts);
                    assertions::print_table_round_summary(
                        &example.repetition.to_string(),
                        asserts.iter(),
                    )
                } else if let Err(err) = result {
                    let assert = AssertionsReport::error(err.to_string());
                    all_asserts.push(assert.clone());
                    assertions::print_table_round_summary(
                        &example.repetition.to_string(),
                        [assert].iter(),
                    )
                }
            }

            assertions::print_table_divider();

            assertions::print_table_round_summary("avg", all_asserts.iter());

            assertions::print_table_footer();
        }

        if !example_cumulative_tool_metrics.is_empty() {
            println!("{}", &example_cumulative_tool_metrics);
        }
    }

    if results_by_example_name.len() > 1 {
        print_h1("AGGREGATE");

        if error_count > 0 {
            println!("\n{error_count} examples failed to run!");
        }

        let programmatic_score_count = programmatic_scores.len();
        if programmatic_score_count > 0 {
            let average_programmatic_score = (programmatic_scores.into_iter().sum::<f32>()
                / (programmatic_score_count as f32))
                .floor();
            println!("Average programmatic score: {average_programmatic_score}%");
        }

        let diff_score_count = diff_scores.len();
        if diff_score_count > 0 {
            let average_diff_score =
                (diff_scores.into_iter().sum::<f32>() / (diff_score_count as f32)).floor();
            println!("Average diff score: {average_diff_score}%");
        }

        let thread_score_count = thread_scores.len();

        if thread_score_count > 0 {
            let average_thread_score =
                (thread_scores.into_iter().sum::<f32>() / (thread_score_count as f32)).floor();
            println!("Average thread score: {average_thread_score}%");
        }

        println!();

        print_h2("CUMULATIVE TOOL METRICS");
        println!("{}", cumulative_tool_metrics);
    }

    let explorer_output_path = run_dir.join("overview.html");
    let mut json_paths: Vec<PathBuf> = results_by_example_name
        .values()
        .flat_map(|results| {
            results.iter().map(|(example, _)| {
                let absolute_path = run_dir.join(example.run_directory.join("last.messages.json"));
                let cwd = std::env::current_dir().expect("Can't get current dir");
                pathdiff::diff_paths(&absolute_path, cwd).unwrap_or_else(|| absolute_path.clone())
            })
        })
        .collect::<Vec<_>>();
    json_paths.sort();
    if let Err(err) = explorer::generate_explorer_html(&json_paths, &explorer_output_path) {
        eprintln!("Failed to generate explorer HTML: {}", err);
    }

    Ok(())
}
