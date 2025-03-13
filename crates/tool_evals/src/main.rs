mod eval;
mod headless_assistant;
mod judge;

use clap::Parser;
use eval::{Eval, EvalOutput};
use gpui::{Application, AsyncApp};
use headless_assistant::HeadlessAppState;
use judge::Judge;
use regex::Regex;
use reqwest_client::ReqwestClient;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

// todo! no hardcoded system prompt
const SYSTEM_PROMPT: &str = include_str!("system_prompt.md");

#[derive(Parser, Debug)]
#[command(
    name = "tool_evals",
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
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    let crate_dir = PathBuf::from("../zed-agent-bench");
    let evaluation_data_dir = crate_dir.join("evaluation_data");
    let repos_dir = crate_dir.join("repos");

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

        cx.spawn(move |cx| async move {
            for eval_name in evals_to_run {
                println!("Running eval named {eval_name}");
                let result = run_eval(
                    &eval_name,
                    &evaluation_data_dir,
                    &repos_dir,
                    args.model_name.clone(),
                    editor_model_name.clone(),
                    judge_model_name.clone(),
                    app_state.clone(),
                    cx.clone(),
                )
                .await;
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
    eval_name: &str,
    evaluation_data_dir: &Path,
    repos_dir: &Path,
    model_name: String,
    editor_model_name: String,
    judge_model_name: String,
    app_state: Arc<HeadlessAppState>,
    cx: AsyncApp,
) -> anyhow::Result<(EvalOutput, String)> {
    let eval_path = evaluation_data_dir.join(eval_name).canonicalize()?;
    let repo_path = repos_dir.canonicalize()?.join(eval_name);
    let eval = Eval::load(
        &eval_path,
        &repo_path,
        Some(SYSTEM_PROMPT.to_string()),
        model_name.clone(),
        editor_model_name.clone(),
    )?;

    let judge = Judge::load(&eval_path, judge_model_name.clone())?;

    let eval_output = cx.update(|cx| eval.run(app_state, cx))?.await?;
    let judge_output = cx.update(|cx| judge.run(&eval_output, cx))?.await?;
    Ok((eval_output, judge_output))
}
