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
mod pull_examples;
mod reorder_patch;
mod retrieve_context;
mod score;
mod split_commit;
mod split_dataset;
mod synthesize;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use collections::HashSet;
use edit_prediction::EditPredictionStore;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use gpui::{AppContext as _, Application, BackgroundExecutor};
use zeta_prompt::ZetaVersion;

use reqwest_client::ReqwestClient;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{path::PathBuf, sync::Arc};

use crate::distill::run_distill;
use crate::example::{Example, group_examples_by_repo, read_example_files};
use crate::format_prompt::run_format_prompt;
use crate::load_project::run_load_project;
use crate::paths::{FAILED_EXAMPLES_DIR, RUN_DIR};
use crate::predict::run_prediction;
use crate::progress::Progress;
use crate::retrieve_context::run_context_retrieval;
use crate::score::run_scoring;
use crate::split_commit::SplitCommitArgs;
use crate::split_dataset::SplitArgs;
use crate::synthesize::{SynthesizeConfig, run_synthesize};

#[derive(Parser, Debug)]
#[command(name = "ep")]
struct EpArgs {
    #[arg(long, default_value_t = false)]
    printenv: bool,
    #[clap(long, default_value_t = 10, global = true)]
    max_parallelism: usize,
    #[clap(long, global = true)]
    limit: Option<usize>,
    /// Filter examples by name
    #[clap(long, global = true)]
    name: Option<String>,
    /// Filter examples by repository
    #[clap(long, global = true)]
    repo: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
    #[clap(global = true, help = INPUTS_HELP)]
    inputs: Vec<PathBuf>,
    #[arg(long, short, global = true)]
    output: Option<PathBuf>,
    #[arg(long, short, global = true)]
    in_place: bool,
    #[arg(long, short, global = true)]
    failfast: bool,
    /// How to handle failed examples in output: keep them or skip them.
    /// Failed examples are always logged to the run's failed directory.
    #[arg(long, global = true, default_value = "keep")]
    failed: FailedHandling,
}

/// Controls whether failed examples are included in the main output.
/// Failed examples are always logged to the run's failed/ directory regardless of this setting.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum FailedHandling {
    /// Include failed examples in the main output (default)
    #[default]
    Keep,
    /// Exclude failed examples from the main output
    Skip,
}

const INPUTS_HELP: &str = r#"
Inputs can be file paths or special specifiers:

  path
      Path to an example(s) file (.md, .json, or .jsonl)

  captured-after:{timestamp}
      Fetch captured examples from Snowflake after the given RFC3339 timestamp.

      You can specify this multiple times and mix it with file inputs.

      Required environment variables to connect to Snowflake:
          EP_SNOWFLAKE_API_KEY
          EP_SNOWFLAKE_BASE_URL

      Optional:
          EP_SNOWFLAKE_ROLE

Examples:

  # Predict from a file
  ep predict examples.jsonl

  # Predict from captured examples after a timestamp
  ep predict captured-after:2025-01-01T00:00:00Z

  # Mix file inputs and captured-after in the same invocation
  ep predict examples.jsonl captured-after:2025-01-01T00:00:00Z
"#;

#[derive(Subcommand, Debug, Clone)]
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
    /// Split a JSONL dataset into multiple files (stratified by repository_url if present)
    Split(SplitArgs),
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
                    .provider
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
                write!(f, "synthesize --repos {}", args.repos.join(" "))
            }
            Command::Clean => write!(f, "clean"),
            Command::SplitCommit(_) => write!(f, "split-commit"),
            Command::Split(_) => write!(f, "split"),
        }
    }
}

#[derive(Debug, Args, Clone)]
struct FormatPromptArgs {
    #[clap(long, short)]
    provider: PredictionProvider,
    #[clap(
        long,
        short,
        help = "(only for --provider zeta2) A substring of a zeta_prompt::ZetaVersion variant to use",
        value_parser = ZetaVersion::parse,
        default_value_t = ZetaVersion::default(),
    )]
    version: ZetaVersion,
}

#[derive(Debug, Args, Clone)]
struct PredictArgs {
    #[clap(long, short)]
    provider: PredictionProvider,
    #[clap(long, default_value_t = 1)]
    repetitions: usize,
    #[clap(
        long,
        short,
        help = "(only for --provider zeta2) A substring of a zeta_prompt::ZetaVersion variant to use",
        value_parser = ZetaVersion::parse,
    )]
    version: ZetaVersion,
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

#[derive(Debug, Args, Clone)]
struct SynthesizeArgs {
    /// Repository URLs (git@github.com:owner/repo or https://...)
    #[clap(long, required = true, num_args = 1..)]
    repos: Vec<String>,

    /// Number of examples to generate per repository
    #[clap(long, default_value_t = 5)]
    count: usize,

    /// Maximum commits to scan per repository before giving up
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

async fn load_examples(
    http_client: Arc<dyn http_client::HttpClient>,
    args: &EpArgs,
    output_path: Option<&PathBuf>,
    background_executor: BackgroundExecutor,
) -> anyhow::Result<Vec<Example>> {
    let mut captured_after_timestamps = Vec::new();
    let mut file_inputs = Vec::new();

    for input in &args.inputs {
        let input_string = input.to_string_lossy();
        if let Some(timestamp) = pull_examples::parse_captured_after_input(input_string.as_ref()) {
            captured_after_timestamps.push(timestamp.to_string());
        } else {
            file_inputs.push(input.clone());
        }
    }

    let mut examples = read_example_files(&file_inputs);

    Progress::global().set_total_examples(examples.len());

    let remaining_limit_for_snowflake =
        args.limit.map(|limit| limit.saturating_sub(examples.len()));

    if let Some(0) = remaining_limit_for_snowflake {
        log::info!(
            "skipping captured-after inputs because --limit is already satisfied by example files"
        );
    } else if !captured_after_timestamps.is_empty() {
        captured_after_timestamps.sort();

        let max_rows_per_timestamp = remaining_limit_for_snowflake.unwrap_or(5000);

        let mut captured_examples = pull_examples::fetch_captured_examples_after(
            http_client,
            &captured_after_timestamps,
            max_rows_per_timestamp,
            background_executor,
        )
        .await?;
        examples.append(&mut captured_examples);
    }

    crate::example::sort_examples_by_repo_and_rev(&mut examples);

    if let Some(name_filter) = &args.name {
        examples.retain(|example| example.spec.name.contains(name_filter));
    }
    if let Some(repo_filter) = &args.repo {
        examples.retain(|example| example.spec.repository_url.contains(repo_filter));
    }

    if let Some(limit) = args.limit {
        if examples.len() > limit {
            examples.truncate(limit);
        }
    }

    if let Some(path) = output_path {
        resume_from_output(path, &mut examples);
    }

    Progress::global().set_total_examples(examples.len());

    Ok(examples)
}

fn spec_hash(spec: &edit_prediction::example_spec::ExampleSpec) -> u64 {
    let mut hasher = collections::FxHasher::default();
    spec.hash(&mut hasher);
    hasher.finish()
}

fn resume_from_output(path: &PathBuf, examples: &mut Vec<Example>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let input_hashes: HashSet<u64> = examples.iter().map(|e| spec_hash(&e.spec)).collect();

    let reader = BufReader::new(file);
    let mut kept_lines = Vec::new();
    let mut kept_hashes = HashSet::default();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if let Ok(output_example) = serde_json::from_str::<Example>(&line) {
            let hash = spec_hash(&output_example.spec);
            if input_hashes.contains(&hash) && !kept_hashes.contains(&hash) {
                kept_hashes.insert(hash);
                kept_lines.push(line);
            }
        }
    }

    let total = examples.len();
    let already_processed = kept_hashes.len();

    eprintln!(
        "Resuming: {}/{} examples already processed",
        already_processed, total
    );

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open output file for rewriting");
    let mut writer = BufWriter::new(file);
    for line in &kept_lines {
        writeln!(writer, "{}", line).expect("Failed to write to output file");
    }
    writer.flush().expect("Failed to flush output file");

    examples.retain(|e| !kept_hashes.contains(&spec_hash(&e.spec)));
}

fn main() {
    let args = EpArgs::parse();

    if args.printenv {
        ::util::shell_env::print_env();
        return;
    }

    let output = args.output_path();
    let command = match &args.command {
        Some(cmd) => cmd.clone(),
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
                repo_urls: synth_args.repos.clone(),
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
            if let Err(error) =
                split_commit::run_split_commit(split_commit_args, &args.inputs, output.as_ref())
            {
                eprintln!("{error:#}");
                std::process::exit(1);
            }
            return;
        }
        Command::Split(split_args) => {
            if let Err(error) = split_dataset::run_split(split_args, &args.inputs) {
                eprintln!("{error:#}");
                std::process::exit(1);
            }
            return;
        }
        _ => {}
    }

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = Arc::new(headless::init(cx));
        EditPredictionStore::global(&app_state.client, &app_state.user_store, cx);

        cx.spawn(async move |cx| {
            let result = async {
                let mut examples = load_examples(
                    app_state.client.http_client(),
                    &args,
                    output.as_ref(),
                    cx.background_executor().clone(),
                )
                .await?;

                match &command {
                    Command::Predict(args) | Command::Score(args) | Command::Eval(args) => {
                        predict::sync_batches(&args.provider).await?;
                    }
                    _ => (),
                }

                let failfast_on_single_example = examples.len() == 1;

                let output_sender: Option<mpsc::UnboundedSender<String>> =
                    if args.output.is_some() || !matches!(command, Command::Eval(_)) {
                        output.as_ref().map(|path| {
                            let file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(path)
                                .expect("Failed to open output file");
                            let mut writer = BufWriter::new(file);
                            let (sender, mut receiver) = mpsc::unbounded::<String>();
                            cx.background_spawn(async move {
                                while let Some(line) = receiver.next().await {
                                    writeln!(writer, "{}", line).expect("Failed to write example");
                                    writer.flush().expect("Failed to flush output");
                                }
                            })
                            .detach();
                            sender
                        })
                    } else {
                        None
                    };

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
                                            args,
                                            app_state.clone(),
                                            cx.clone(),
                                        )
                                        .await?;
                                    }
                                    Command::Predict(args) => {
                                        run_prediction(
                                            example,
                                            args,
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
                                    | Command::SplitCommit(_)
                                    | Command::Split(_) => {
                                        unreachable!()
                                    }
                                }
                                anyhow::Ok(())
                            }
                            .await;

                            let failed = if let Err(error) = result {
                                handle_error(
                                    error,
                                    &args,
                                    &command,
                                    &app_state,
                                    failfast_on_single_example,
                                    example,
                                )
                                .await;
                                true
                            } else {
                                false
                            };

                            let should_write = !failed || args.failed == FailedHandling::Keep;
                            if should_write {
                                if let Some(ref mut sender) = output_sender.clone() {
                                    let line = serde_json::to_string(example).unwrap();
                                    sender
                                        .send(line)
                                        .await
                                        .expect("Failed to send to output writer");
                                } else if args.output.is_none()
                                    && !matches!(command, Command::Eval(_))
                                {
                                    let line = serde_json::to_string(example).unwrap();
                                    println!("{}", line);
                                }
                            }
                        }
                    });
                    futures::future::join_all(futures).await;
                }

                Progress::global().finalize();

                match &command {
                    Command::Predict(args) | Command::Score(args) | Command::Eval(args) => {
                        predict::sync_batches(&args.provider).await?;
                    }
                    _ => (),
                }

                match &command {
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

async fn handle_error(
    error: anyhow::Error,
    args: &EpArgs,
    command: &Command,
    app_state: &Arc<headless::EpAppState>,
    failfast_on_single_example: bool,
    example: &Example,
) {
    Progress::global().increment_failed();
    let example_name = example.spec.filename();
    let failed_example_path = FAILED_EXAMPLES_DIR.join(format!("{}.json", example_name));
    app_state
        .fs
        .write(
            &failed_example_path,
            &serde_json::to_vec_pretty(&example).unwrap(),
        )
        .await
        .unwrap();
    let err_path = FAILED_EXAMPLES_DIR.join(format!("{}_err.txt", example_name));
    app_state
        .fs
        .write(&err_path, format!("{error:?}").as_bytes())
        .await
        .unwrap();

    let failed_jsonl_path = RUN_DIR.join("failed.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&failed_jsonl_path)
        .expect("Failed to open failed.jsonl");
    writeln!(file, "{}", serde_json::to_string(example).unwrap())
        .expect("Failed to write to failed.jsonl");

    let cursor_path = example
        .repo_name()
        .unwrap()
        .worktree_path()
        .join(&example.spec.cursor_path);

    let msg = format!(
        indoc::indoc! {"
            While processing \"{}\":

            \x1b[31m{:?}\x1b[0m

            Example:        \x1b[36m{}\x1b[0m
            Error file:     \x1b[36m{}\x1b[0m
            Cursor file:    \x1b[36m{}\x1b[0m
            Re-run:         cargo run -p edit_prediction_cli -- {} \x1b[36m{}\x1b[0m
        "},
        example.spec.name,
        error,
        failed_example_path.display(),
        err_path.display(),
        cursor_path.display(),
        command,
        failed_example_path.display(),
    );
    if args.failfast || failfast_on_single_example {
        Progress::global().finalize();
        panic!("{}", msg);
    } else {
        log::error!("{}", msg);
    }
}
