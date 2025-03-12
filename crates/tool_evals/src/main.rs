mod eval;
mod headless_assistant;
mod judge;

use clap::Parser;
use eval::Eval;
use gpui::Application;
use judge::Judge;
use language_model::{LanguageModelProviderId, ANTHROPIC_PROVIDER_ID};
use reqwest_client::ReqwestClient;
use std::{path::PathBuf, sync::Arc};

// todo! no hardcoded system prompt
const SYSTEM_PROMPT: &str = include_str!("system_prompt.md");

#[derive(Parser, Debug)]
#[command(
    name = "tool_evals",
    disable_version_flag = true,
    before_help = "Tool eval runner"
)]
struct Args {
    evals_to_run: Vec<String>,
    /// Runs all evals in `evaluation_data`.
    #[arg(long)]
    all: bool,
    /// Name of the model provider (default: "anthropic")
    #[arg(long, default_value = ANTHROPIC_PROVIDER_ID)]
    provider_id: String,
    /// Name of the model (default: "claude-3-7-sonnet-latest")
    #[arg(long, default_value = "claude-3-7-sonnet-latest")]
    model_name: String,
    /// Name of the editor model provider (default: value of `--provider_id`).
    #[arg(long)]
    editor_model_provider_id: Option<String>,
    /// Name of the editor model (default: value of `--model_name`).
    #[arg(long)]
    editor_model_name: Option<String>,
    /// Name of the judge model provider (default: value of `--provider_id`).
    #[arg(long)]
    judge_model_provider_id: Option<String>,
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

    let evals_to_run = if args.all {
        std::fs::read_dir(&evaluation_data_dir)
            .unwrap()
            .map(|path| path.unwrap().file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>()
    } else {
        args.evals_to_run
    };

    if evals_to_run.is_empty() {
        panic!("Names of evals to run must be provided or `--all` specified");
    }

    let provider_id = LanguageModelProviderId(args.provider_id.into());
    let editor_model_provider_id = if let Some(provider_id) = args.editor_model_provider_id {
        LanguageModelProviderId(provider_id.into())
    } else {
        provider_id.clone()
    };

    let editor_model_name = if let Some(model_name) = args.editor_model_name {
        model_name
    } else {
        args.model_name.clone()
    };

    let judge_model_provider_id = if let Some(provider_id) = args.judge_model_provider_id {
        LanguageModelProviderId(provider_id.into())
    } else {
        provider_id.clone()
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
                let eval_path = evaluation_data_dir.join(&eval_name).canonicalize().unwrap();
                let repo_path = repos_dir.canonicalize().unwrap().join(&eval_name);
                let eval = Eval::load(
                    &eval_path,
                    &repo_path,
                    Some(SYSTEM_PROMPT.to_string()),
                    provider_id.clone(),
                    args.model_name.clone(),
                    editor_model_provider_id.clone(),
                    editor_model_name.clone(),
                )
                .unwrap();

                let judge = Judge::load(
                    &eval_path,
                    judge_model_provider_id.clone(),
                    judge_model_name.clone(),
                )
                .unwrap();

                let task = cx.update(|cx| eval.run(app_state.clone(), cx)).unwrap();
                match task.await {
                    Ok(result) => {
                        println!("Result: {:?}", result);
                        // judge.run(&result).unwrap();
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }

            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });

    println!("Done running evals");
}
