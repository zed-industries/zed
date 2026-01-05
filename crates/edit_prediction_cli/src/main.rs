mod anthropic_client;
mod distill;
mod example;
mod format_prompt;
mod git;
mod headless;
mod load_project;
mod metrics;
mod paths;
mod predict;
mod progress;
mod reorder_patch;
mod retrieve_context;
mod score;
mod split_commit;
mod synthesize;

use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use edit_prediction::EditPredictionStore;
use gpui::Application;
use reqwest_client::ReqwestClient;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{path::PathBuf, sync::Arc};

use crate::distill::run_distill;
use crate::example::{group_examples_by_repo, read_examples, write_examples};
use crate::format_prompt::run_format_prompt;
use crate::load_project::run_load_project;
use crate::paths::FAILED_EXAMPLES_DIR;
use crate::predict::run_prediction;
use crate::progress::Progress;
use crate::retrieve_context::run_context_retrieval;
use crate::score::run_scoring;
use crate::split_commit::SplitCommitArgs;
use crate::synthesize::{SynthesizeConfig, run_synthesize};

#[derive(Parser, Debug)]
#[command(name = "ep")]
struct EpArgs {
    #[arg(long, default_value_t = false)]
    printenv: bool,
    #[clap(long, default_value_t = 10, global = true)]
    max_parallelism: usize,
    #[command(subcommand)]
    command: Option<Command>,
    #[clap(global = true)]
    inputs: Vec<PathBuf>,
    #[arg(long, short, global = true)]
    output: Option<PathBuf>,
    #[arg(long, short, global = true)]
    in_place: bool,
    #[arg(long, short, global = true)]
    failfast: bool,
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
    /// Prepares a distillation dataset by copying expected outputs to
    /// predicted outputs and removing actual outputs and prompts.
    Distill,
    /// Print aggregated scores
    Eval(PredictArgs),
    /// Generate eval examples by analyzing git commits from a repository
    Synthesize(SynthesizeArgs),
    /// Remove git repositories and worktrees
    Clean,
    /// Generate an evaluation example by splitting a chronologically-ordered commit
    SplitCommit(SplitCommitArgs),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::ParseExample => write!(f, "parse-example"),
            Command::LoadProject => write!(f, "load-project"),
            Command::Context => write!(f, "context"),
            Command::FormatPrompt(format_prompt_args) => write!(
                f,
                "format-prompt --prompt-format={}",
                format_prompt_args
                    .prompt_format
                    .to_possible_value()
                    .unwrap()
                    .get_name()
            ),
            Command::Predict(predict_args) => {
                write!(
                    f,
                    "predict --provider={:?}",
                    predict_args
                        .provider
                        .to_possible_value()
                        .unwrap()
                        .get_name()
                )
            }
            Command::Score(predict_args) => {
                write!(
                    f,
                    "score --provider={:?}",
                    predict_args
                        .provider
                        .to_possible_value()
                        .unwrap()
                        .get_name()
                )
            }
            Command::Distill => write!(f, "distill"),
            Command::Eval(predict_args) => write!(
                f,
                "eval --provider={:?}",
                predict_args
                    .provider
                    .to_possible_value()
                    .unwrap()
                    .get_name()
            ),
            Command::Synthesize(args) => {
                write!(f, "synthesize --repo={}", args.repo)
            }
            Command::Clean => write!(f, "clean"),
            Command::SplitCommit(_) => write!(f, "split-commit"),
        }
    }
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

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, Serialize, Deserialize)]
enum PredictionProvider {
    Sweep,
    Mercury,
    Zeta1,
    Zeta2,
    Teacher,
    TeacherNonBatching,
}

#[derive(Debug, Args)]
struct SynthesizeArgs {
    /// Repository URL (git@github.com:owner/repo or https://...)
    #[clap(long)]
    repo: String,

    /// Number of examples to generate
    #[clap(long, default_value_t = 5)]
    count: usize,

    /// Maximum commits to scan before giving up
    #[clap(long, default_value_t = 100)]
    max_commits: usize,

    /// Ignore state file and reprocess all commits
    #[clap(long)]
    fresh: bool,
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
        Command::Synthesize(synth_args) => {
            let Some(output_dir) = args.output else {
                panic!("output dir is required");
            };
            let config = SynthesizeConfig {
                repo_url: synth_args.repo.clone(),
                count: synth_args.count,
                max_commits: synth_args.max_commits,
                output_dir,
                fresh: synth_args.fresh,
            };
            smol::block_on(async {
                if let Err(e) = run_synthesize(config).await {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                }
            });
            return;
        }
        Command::SplitCommit(split_commit_args) => {
            if let Err(error) = split_commit::run_split_commit(split_commit_args) {
                eprintln!("{error:#}");
                std::process::exit(1);
            }
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
            let result = async {
                if let Command::Predict(args) = &command {
                    predict::sync_batches(&args.provider).await?;
                }

                let total_examples = examples.len();
                Progress::global().set_total_examples(total_examples);

                let mut grouped_examples = group_examples_by_repo(&mut examples);
                let example_batches = grouped_examples.chunks_mut(args.max_parallelism);

                for example_batch in example_batches {
                    let futures = example_batch.into_iter().map(|repo_examples| async {
                        for example in repo_examples.iter_mut() {
                            let result = async {
                                match &command {
                                    Command::ParseExample => {}
                                    Command::LoadProject => {
                                        run_load_project(example, app_state.clone(), cx.clone())
                                            .await?;
                                    }
                                    Command::Context => {
                                        run_context_retrieval(
                                            example,
                                            app_state.clone(),
                                            cx.clone(),
                                        )
                                        .await?;
                                    }
                                    Command::FormatPrompt(args) => {
                                        run_format_prompt(
                                            example,
                                            args.prompt_format,
                                            app_state.clone(),
                                            cx.clone(),
                                        )
                                        .await?;
                                    }
                                    Command::Predict(args) => {
                                        run_prediction(
                                            example,
                                            Some(args.provider),
                                            args.repetitions,
                                            app_state.clone(),
                                            cx.clone(),
                                        )
                                        .await?;
                                    }
                                    Command::Distill => {
                                        run_distill(example).await?;
                                    }
                                    Command::Score(args) | Command::Eval(args) => {
                                        run_scoring(example, &args, app_state.clone(), cx.clone())
                                            .await?;
                                    }
                                    Command::Clean
                                    | Command::Synthesize(_)
                                    | Command::SplitCommit(_) => {
                                        unreachable!()
                                    }
                                }
                                anyhow::Ok(())
                            }
                            .await;

                            if let Err(e) = result {
                                Progress::global().increment_failed();
                                let failed_example_path =
                                    FAILED_EXAMPLES_DIR.join(format!("{}.json", example.spec.name));
                                app_state
                                    .fs
                                    .write(
                                        &failed_example_path,
                                        &serde_json::to_vec_pretty(&example).unwrap(),
                                    )
                                    .await
                                    .unwrap();
                                let err_path = FAILED_EXAMPLES_DIR
                                    .join(format!("{}_err.txt", example.spec.name));
                                app_state
                                    .fs
                                    .write(&err_path, e.to_string().as_bytes())
                                    .await
                                    .unwrap();

                                let msg = format!(
                                    indoc::indoc! {"
                                        While processing {}:

                                        {:?}

                                        Written to: \x1b[36m{}\x1b[0m

                                        Explore this example data with:
                                            fx \x1b[36m{}\x1b[0m

                                        Re-run this example with:
                                            cargo run -p edit_prediction_cli -- {} \x1b[36m{}\x1b[0m
                                    "},
                                    example.spec.name,
                                    e,
                                    err_path.display(),
                                    failed_example_path.display(),
                                    command,
                                    failed_example_path.display(),
                                );
                                if args.failfast || total_examples == 1 {
                                    Progress::global().finalize();
                                    panic!("{}", msg);
                                } else {
                                    log::error!("{}", msg);
                                }
                            }
                        }
                    });
                    futures::future::join_all(futures).await;
                }
                Progress::global().finalize();

                if args.output.is_some() || !matches!(command, Command::Eval(_)) {
                    write_examples(&examples, output.as_ref());
                }

                match &command {
                    Command::Predict(args) => predict::sync_batches(&args.provider).await?,
                    Command::Eval(_) => score::print_report(&examples),
                    _ => (),
                };

                anyhow::Ok(())
            }
            .await;

            if let Err(e) = result {
                panic!("Fatal error: {:?}", e);
            }

            let _ = cx.update(|cx| cx.quit());
        })
        .detach();
    });
}
