mod eval;
mod headless_assistant;

use clap::Parser;
use eval::Eval;
use gpui::Application;
use language_model::{LanguageModelProviderId, ANTHROPIC_PROVIDER_ID};
use reqwest_client::ReqwestClient;
use std::{path::PathBuf, sync::Arc};

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
}

fn main() {
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

    app.run(move |cx| {
        let app_state = headless_assistant::init(cx);

        cx.spawn(move |cx| async move {
            for eval_name in evals_to_run {
                let eval_path = evaluation_data_dir.join(&eval_name);
                let repo_path = repos_dir.join(&eval_name);
                let eval = Eval::load(
                    &eval_path,
                    &repo_path,
                    provider_id.clone(),
                    args.model_name.clone(),
                )
                .unwrap();

                let task = cx.update(|cx| eval.run(app_state.clone(), cx)).unwrap();
                match task.await {
                    Ok(response) => println!("Response: {:?}", response),
                    Err(err) => println!("Error: {}", err),
                }
            }
        })
        .detach();
    });

    println!("Test succeeded!");
}
