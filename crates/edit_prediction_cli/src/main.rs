mod anthropic_client;
mod example;
mod format_prompt;
mod headless;
mod load_project;
mod metrics;
mod paths;
mod predict;
mod retrieve_context;
mod score;

use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use edit_prediction::EditPredictionStore;
use gpui::Application;
use reqwest_client::ReqwestClient;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

use crate::example::{read_examples, write_examples};
use crate::format_prompt::run_format_prompt;
use crate::load_project::run_load_project;
use crate::predict::run_prediction;
use crate::retrieve_context::run_context_retrieval;
use crate::score::run_scoring;

#[derive(Parser, Debug)]
#[command(name = "ep")]
struct EpArgs {
    #[arg(long, default_value_t = false)]
    printenv: bool,
    #[clap(long, default_value_t = 10)]
    max_parallelism: usize,
    #[command(subcommand)]
    command: Option<Command>,
    #[clap(global = true)]
    inputs: Vec<PathBuf>,
    #[arg(long, short, global = true)]
    output: Option<PathBuf>,
    #[arg(long, short, global = true)]
    in_place: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse markdown examples and output a combined .jsonl file
    ParseExample,
    /// Create git worktrees for each example and load file contents
    LoadProject,
    /// Retrieve context for input examples.
    Context,
    /// Generate a prompt string for a specific model
    FormatPrompt(FormatPromptArgs),
    /// Runs edit prediction
    Predict(PredictArgs),
    /// Computes a score based on actual and expected patches
    Score(PredictArgs),
    /// Print aggregated scores
    Eval(PredictArgs),
    /// Remove git repositories and worktrees
    Clean,
}

#[derive(Debug, Args)]
struct FormatPromptArgs {
    #[clap(long)]
    prompt_format: PromptFormat,
}

#[derive(Clone, Copy, Debug, ValueEnum, Serialize, Deserialize)]
enum PromptFormat {
    Teacher,
    Zeta2,
}

#[derive(Debug, Args)]
struct PredictArgs {
    #[clap(long)]
    provider: PredictionProvider,
    #[clap(long, default_value_t = 1)]
    repetitions: usize,
}

#[derive(Clone, Copy, Debug, ValueEnum, Serialize, Deserialize)]
enum PredictionProvider {
    Sweep,
    Mercury,
    Zeta1,
    Zeta2,
    Teacher,
}

impl EpArgs {
    fn output_path(&self) -> Option<PathBuf> {
        if self.in_place {
            if self.inputs.len() == 1 {
                self.inputs.first().cloned()
            } else {
                panic!("--in-place requires exactly one input file")
            }
        } else {
            self.output.clone()
        }
    }
}

fn main() {
    zlog::init();
    zlog::init_output_stderr();
    let args = EpArgs::parse();

    if args.printenv {
        ::util::shell_env::print_env();
        return;
    }

    let output = args.output_path();
    let command = match args.command {
        Some(cmd) => cmd,
        None => {
            EpArgs::command().print_help().unwrap();
            return;
        }
    };

    match &command {
        Command::Clean => {
            std::fs::remove_dir_all(&*paths::DATA_DIR).unwrap();
            return;
        }
        _ => {}
    }

    let mut examples = read_examples(&args.inputs);
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = Arc::new(headless::init(cx));
        EditPredictionStore::global(&app_state.client, &app_state.user_store, cx);

        cx.spawn(async move |cx| {
            match &command {
                Command::Predict(args) => predict::sync_batches(&args.provider).await,
                _ => (),
            };

            let chunks = examples.chunks_mut(args.max_parallelism);
            let total_chunks = chunks.len();
            for (batch_ix, data) in chunks.enumerate() {
                let mut futures = Vec::new();
                eprintln!("Processing batch: {}/{}", batch_ix + 1, total_chunks);

                for example in data.iter_mut() {
                    let cx = cx.clone();
                    let app_state = app_state.clone();
                    futures.push(async {
                        match &command {
                            Command::ParseExample => {}
                            Command::LoadProject => {
                                run_load_project(example, app_state.clone(), cx).await;
                            }
                            Command::Context => {
                                run_context_retrieval(example, app_state, cx).await;
                            }
                            Command::FormatPrompt(args) => {
                                run_format_prompt(example, args.prompt_format, app_state, cx).await;
                            }
                            Command::Predict(args) => {
                                run_prediction(
                                    example,
                                    Some(args.provider),
                                    args.repetitions,
                                    app_state.clone(),
                                    cx,
                                )
                                .await;
                            }
                            Command::Score(args) | Command::Eval(args) => {
                                run_scoring(example, &args, app_state, cx).await;
                            }
                            Command::Clean => {
                                unreachable!()
                            }
                        }
                    });
                }
                futures::future::join_all(futures).await;
            }

            if args.output.is_some() || !matches!(command, Command::Eval(_)) {
                write_examples(&examples, output.as_ref());
            }

            match &command {
                Command::Predict(args) => predict::sync_batches(&args.provider).await,
                Command::Eval(_) => score::print_report(&examples),
                _ => (),
            };

            let _ = cx.update(|cx| cx.quit());
        })
        .detach();
    });
}
