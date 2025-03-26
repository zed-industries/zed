mod eval;
mod headless_assistant;
mod judge;

use clap::Parser;
use eval::{Eval, EvalOutput};
use futures::future;
use gpui::{Application, AsyncApp};
use headless_assistant::{authenticate_model_provider, find_model, HeadlessAppState};
use itertools::Itertools;
use judge::Judge;
use language_model::{LanguageModel, LanguageModelRegistry};
use regex::Regex;
use reqwest_client::ReqwestClient;
use std::{cmp, path::PathBuf, sync::Arc};

#[derive(Parser, Debug)]
#[command(
    name = "assistant_eval",
    disable_version_flag = true,
    before_help = "Tool eval runner"
)]
struct Args {
    /// Regexes to match the names of evals to run.
    eval_name_regexes: Vec<String>,
    /// Runs all evals in `evaluation_data`, causes the regex to be ignored.
    #[arg(long)]
    all: bool,
    /// Name of the model (default: "claude-3-7-sonnet-latest")
    #[arg(long, default_value = "claude-3-7-sonnet-latest")]
    model_name: String,
    /// Name of the editor model (default: value of `--model_name`).
    #[arg(long)]
    editor_model_name: Option<String>,
    /// Name of the judge model (default: value of `--model_name`).
    #[arg(long)]
    judge_model_name: Option<String>,
    /// Number of evaluations to run concurrently (default: 10)
    #[arg(short, long, default_value = "10")]
    concurrency: usize,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    let crate_dir = PathBuf::from("../zed-agent-bench");
    let evaluation_data_dir = crate_dir.join("evaluation_data").canonicalize().unwrap();

    let repos_dir = crate_dir.join("repos");
    if !repos_dir.exists() {
        std::fs::create_dir_all(&repos_dir).unwrap();
    }
    let repos_dir = repos_dir.canonicalize().unwrap();

    let all_evals = std::fs::read_dir(&evaluation_data_dir)
        .unwrap()
        .map(|path| path.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let evals_to_run = if args.all {
        all_evals
    } else {
        args.eval_name_regexes
            .into_iter()
            .map(|regex_string| Regex::new(&regex_string).unwrap())
            .flat_map(|regex| {
                all_evals
                    .iter()
                    .filter(|eval_name| regex.is_match(eval_name))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    if evals_to_run.is_empty() {
        panic!("Names of evals to run must be provided or `--all` specified");
    }

    println!("Will run the following evals: {evals_to_run:?}");
    println!("Running up to {} evals concurrently", args.concurrency);

    let editor_model_name = if let Some(model_name) = args.editor_model_name {
        model_name
    } else {
        args.model_name.clone()
    };

    let judge_model_name = if let Some(model_name) = args.judge_model_name {
        model_name
    } else {
        args.model_name.clone()
    };

    app.run(move |cx| {
        let app_state = headless_assistant::init(cx);

        let model = find_model(&args.model_name, cx).unwrap();
        let editor_model = find_model(&editor_model_name, cx).unwrap();
        let judge_model = find_model(&judge_model_name, cx).unwrap();

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_active_model(Some(model.clone()), cx);
            registry.set_editor_model(Some(editor_model.clone()), cx);
        });

        let model_provider_id = model.provider_id();
        let editor_model_provider_id = editor_model.provider_id();
        let judge_model_provider_id = judge_model.provider_id();

        cx.spawn(async move |cx| {
            // Authenticate all model providers first
            cx.update(|cx| authenticate_model_provider(model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();
            cx.update(|cx| authenticate_model_provider(editor_model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();
            cx.update(|cx| authenticate_model_provider(judge_model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();

            let eval_load_futures = evals_to_run
                .into_iter()
                .map(|eval_name| {
                    let eval_path = evaluation_data_dir.join(&eval_name);
                    let load_future = Eval::load(eval_name.clone(), eval_path, &repos_dir);
                    async move {
                        match load_future.await {
                            Ok(eval) => Some(eval),
                            Err(err) => {
                                // TODO: Persist errors / surface errors at the end.
                                println!("Error loading {eval_name}: {err}");
                                None
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();

            let loaded_evals = future::join_all(eval_load_futures)
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            // The evals need to be loaded and grouped by URL before concurrently running, since
            // evals that use the same remote URL will use the same working directory.
            let mut evals_grouped_by_url: Vec<Vec<Eval>> = loaded_evals
                .into_iter()
                .map(|eval| (eval.eval_setup.url.clone(), eval))
                .into_group_map()
                .into_values()
                .collect::<Vec<_>>();

            // Sort groups in descending order, so that bigger groups start first.
            evals_grouped_by_url.sort_by_key(|evals| cmp::Reverse(evals.len()));

            let result_futures = evals_grouped_by_url
                .into_iter()
                .map(|evals| {
                    let model = model.clone();
                    let judge_model = judge_model.clone();
                    let app_state = app_state.clone();
                    let cx = cx.clone();

                    async move {
                        let mut results = Vec::new();
                        for eval in evals {
                            let name = eval.name.clone();
                            println!("Starting eval named {}", name);
                            let result = run_eval(
                                eval,
                                model.clone(),
                                judge_model.clone(),
                                app_state.clone(),
                                cx.clone(),
                            )
                            .await;
                            results.push((name, result));
                        }
                        results
                    }
                })
                .collect::<Vec<_>>();

            let results = future::join_all(result_futures)
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            // Process results in order of completion
            for (eval_name, result) in results {
                match result {
                    Ok((eval_output, judge_output)) => {
                        println!("Generated diff for {eval_name}:\n");
                        println!("{}\n", eval_output.diff);
                        println!("Last message for {eval_name}:\n");
                        println!("{}\n", eval_output.last_message);
                        println!("Elapsed time: {:?}", eval_output.elapsed_time);
                        println!(
                            "Assistant response count: {}",
                            eval_output.assistant_response_count
                        );
                        println!("Tool use counts: {:?}", eval_output.tool_use_counts);
                        println!("Judge output for {eval_name}: {judge_output}");
                    }
                    Err(err) => {
                        // TODO: Persist errors / surface errors at the end.
                        println!("Error running {eval_name}: {err}");
                    }
                }
            }

            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });

    println!("Done running evals");
}

async fn run_eval(
    eval: Eval,
    model: Arc<dyn LanguageModel>,
    judge_model: Arc<dyn LanguageModel>,
    app_state: Arc<HeadlessAppState>,
    cx: AsyncApp,
) -> anyhow::Result<(EvalOutput, String)> {
    let path = eval.path.clone();
    let judge = Judge::load(&path, judge_model).await?;
    let eval_output = cx.update(|cx| eval.run(app_state, model, cx))?.await?;
    let judge_output = cx.update(|cx| judge.run(&eval_output, cx))?.await?;
    eval_output.save_to_directory(&path, judge_output.to_string())?;
    Ok((eval_output, judge_output))
}
