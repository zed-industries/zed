mod anthropic_client;
mod distill;
mod example;
mod filter_languages;
mod format_prompt;
mod git;
mod headless;
mod load_project;
mod metrics;
mod openai_client;
mod parse_output;
mod paths;
mod predict;
mod progress;
mod prompt_assets;
mod pull_examples;
mod qa;
mod reorder_patch;
mod repair;
mod retrieve_context;
mod reversal_tracking;
mod score;
mod split_commit;
mod split_dataset;
mod synthesize;
mod truncate_expected_patch;
mod word_diff;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use collections::HashSet;
use edit_prediction::EditPredictionStore;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use gpui::{AppContext as _, Application, BackgroundExecutor, Task};
use zeta_prompt::ZetaVersion;

use reqwest_client::ReqwestClient;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::distill::run_distill;
use crate::example::{Example, group_examples_by_repo, read_example_files};
use crate::filter_languages::{FilterLanguagesArgs, run_filter_languages};
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
use crate::truncate_expected_patch::TruncatePatchArgs;

#[derive(Parser, Debug)]
#[command(name = "ep")]
struct EpArgs {
    #[arg(long, default_value_t = false)]
    printenv: bool,
    #[clap(long, default_value_t = 10, global = true)]
    max_parallelism: usize,
    /// The limit for the number of examples to process
    /// Default is unlimited for processing local datasets, 5000 when pulling from snowflake
    #[clap(long, global = true)]
    limit: Option<usize>,
    #[clap(long, global = true)]
    offset: Option<usize>,
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
    /// Output as markdown files instead of JSONL. When set, -o specifies a directory
    /// where one .md file per example will be written (named after each example).
    #[arg(long, short, global = true)]
    markdown: bool,
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
    /// Skip writing files
    SkipNoFiles,
}

const INPUTS_HELP: &str = r#"
Inputs can be file paths or special specifiers:

  path
      Path to an example(s) file (.md, .json, or .jsonl)

  captured-after:{timestamp}
      Fetch captured examples from Snowflake after the given RFC3339 timestamp.
      These are examples captured via the "Capture Edit Prediction Example" action.

  rejected-after:{timestamp}
      Fetch rejected edit predictions from Snowflake after the given RFC3339 timestamp.
      These are predictions that were shown to users but rejected (useful for DPO training).

  rated-after:{timestamp}
      Fetch user-rated edit predictions from Snowflake after the given RFC3339 timestamp.
      These are predictions that users explicitly rated as positive or negative via the
      rate completions modal. Only zeta2 predictions are included.
      - Positive ratings: output becomes expected_patches
      - Negative ratings: output becomes rejected_patch

  rated-positive-after:{timestamp}
      Same as rated-after, but only fetches positively rated predictions.

  rated-negative-after:{timestamp}
      Same as rated-after, but only fetches negatively rated predictions.

      Required environment variables to connect to Snowflake:
          EP_SNOWFLAKE_API_KEY
          EP_SNOWFLAKE_BASE_URL

      Optional:
          EP_SNOWFLAKE_ROLE

Examples:

  # Read examples from a file
  ep read examples.jsonl -o output.jsonl

  # Read captured examples after a timestamp
  ep read captured-after:2025-01-01T00:00:00Z -o captured.jsonl

  # Read rejected predictions for DPO training
  ep read rejected-after:2025-01-01T00:00:00Z -o rejected.jsonl

  # Read user-rated predictions
  ep read rated-after:2025-01-01T00:00:00Z -o rated.jsonl

  # Read only positively rated predictions
  ep read rated-positive-after:2025-01-01T00:00:00Z -o positive.jsonl

  # Read only negatively rated predictions
  ep read rated-negative-after:2025-01-01T00:00:00Z -o negative.jsonl

  # Mix multiple input sources
  ep predict examples.jsonl captured-after:2025-01-01T00:00:00Z
"#;

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Read examples from files or fetch from Snowflake, output as .jsonl
    Read,
    /// Create git worktrees for each example and load file contents
    LoadProject,
    /// Retrieve context for input examples.
    Context,
    /// Generate a prompt string for a specific model
    FormatPrompt(FormatPromptArgs),
    /// Runs edit prediction
    Predict(PredictArgs),
    /// Parse model outputs (actual_output) into unified diffs (actual_patch).
    /// Requires format-prompt to have been run first. Uses provider from prompt.
    ParseOutput,
    /// Computes a score based on actual and expected patches
    Score(PredictArgs),
    /// Prepares a distillation dataset by copying expected outputs to
    /// predicted outputs and removing actual outputs and prompts.
    Distill,
    /// Print aggregated scores
    Eval(EvalArgs),
    /// Generate eval examples by analyzing git commits from a repository
    Synthesize(SynthesizeArgs),
    /// Remove git repositories and worktrees
    Clean,
    /// Generate an evaluation example by splitting a chronologically-ordered commit
    SplitCommit(SplitCommitArgs),
    /// Truncate expected patch by the given criteria
    TruncatePatch(TruncatePatchArgs),
    /// Split a JSONL dataset into multiple files (stratified by repository_url if present)
    Split(SplitArgs),
    /// Filter a JSONL dataset by programming language (based on cursor_path extension)
    FilterLanguages(FilterLanguagesArgs),
    /// Import Anthropic batch results by batch IDs (useful for recovering after database loss)
    ImportBatch(ImportBatchArgs),
    /// Assess the quality of predictions using LLM-as-a-judge
    Qa(qa::QaArgs),
    /// Repair predictions that received poor QA scores by generating improved predictions
    Repair(repair::RepairArgs),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Read => write!(f, "read"),
            Command::LoadProject => write!(f, "load-project"),
            Command::Context => write!(f, "context"),
            Command::FormatPrompt(args) => {
                write!(f, "format-prompt --provider={}", args.provider)
            }
            Command::Predict(args) => match &args.provider {
                Some(provider) => write!(f, "predict --provider={}", provider),
                None => write!(f, "predict"),
            },
            Command::ParseOutput => write!(f, "parse-output"),
            Command::Score(args) => match &args.provider {
                Some(provider) => write!(f, "score --provider={}", provider),
                None => write!(f, "score"),
            },
            Command::Distill => write!(f, "distill"),
            Command::Eval(args) => match &args.predict.provider {
                Some(provider) => write!(f, "eval --provider={}", provider),
                None => write!(f, "eval"),
            },
            Command::Synthesize(args) => {
                write!(f, "synthesize --repos {}", args.repos.join(" "))
            }
            Command::Clean => write!(f, "clean"),
            Command::SplitCommit(_) => write!(f, "split-commit"),
            Command::TruncatePatch(_) => write!(f, "truncate-patch"),
            Command::Split(_) => write!(f, "split"),
            Command::FilterLanguages(_) => write!(f, "filter-languages"),
            Command::ImportBatch(args) => {
                write!(f, "import-batch --batch-ids {}", args.batch_ids.join(" "))
            }
            Command::Qa(_) => {
                write!(f, "qa")
            }
            Command::Repair(_) => {
                write!(f, "repair")
            }
        }
    }
}

#[derive(Debug, Args, Clone)]
struct FormatPromptArgs {
    #[clap(long, short('p'), default_value_t = PredictionProvider::default())]
    provider: PredictionProvider,
}

#[derive(Debug, Args, Clone)]
struct PredictArgs {
    #[clap(long, short('p'))]
    provider: Option<PredictionProvider>,
    #[clap(long, default_value_t = 1)]
    repetitions: usize,
    /// Only use cached responses, don't queue new requests for batching
    #[clap(long)]
    cache_only: bool,
}

#[derive(Debug, Args, Clone)]
struct EvalArgs {
    #[clap(flatten)]
    predict: PredictArgs,
    /// Path to write summary scores as JSON
    #[clap(long)]
    summary_json: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeacherBackend {
    Sonnet45,
    Gpt52,
}

impl std::fmt::Display for TeacherBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeacherBackend::Sonnet45 => write!(f, "sonnet45"),
            TeacherBackend::Gpt52 => write!(f, "gpt52"),
        }
    }
}

impl std::str::FromStr for TeacherBackend {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sonnet45" | "sonnet" | "claude" => Ok(TeacherBackend::Sonnet45),
            "gpt52" | "gpt" | "openai" => Ok(TeacherBackend::Gpt52),
            "v0114180editableregion" => Ok(TeacherBackend::Sonnet45),
            _ => anyhow::bail!("unknown teacher backend `{s}`. Valid options: sonnet45, gpt52"),
        }
    }
}

impl TeacherBackend {
    pub fn model_name(&self) -> &'static str {
        match self {
            TeacherBackend::Sonnet45 => "claude-sonnet-4-5",
            TeacherBackend::Gpt52 => "gpt-5.2",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PredictionProvider {
    Sweep,
    Mercury,
    Zeta1,
    Zeta2(ZetaVersion),
    Teacher(TeacherBackend),
    TeacherNonBatching(TeacherBackend),
    Repair,
}

impl Default for PredictionProvider {
    fn default() -> Self {
        PredictionProvider::Zeta2(ZetaVersion::default())
    }
}

impl std::fmt::Display for PredictionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PredictionProvider::Sweep => write!(f, "sweep"),
            PredictionProvider::Mercury => write!(f, "mercury"),
            PredictionProvider::Zeta1 => write!(f, "zeta1"),
            PredictionProvider::Zeta2(version) => write!(f, "zeta2:{version}"),
            PredictionProvider::Teacher(backend) => write!(f, "teacher:{backend}"),
            PredictionProvider::TeacherNonBatching(backend) => {
                write!(f, "teacher-non-batching:{backend}")
            }
            PredictionProvider::Repair => write!(f, "repair"),
        }
    }
}

impl std::str::FromStr for PredictionProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (provider, arg) = s.split_once(':').map_or((s, None), |(p, a)| (p, Some(a)));

        let provider_lower = provider.to_lowercase();
        match provider_lower.as_str() {
            "sweep" => Ok(PredictionProvider::Sweep),
            "mercury" => Ok(PredictionProvider::Mercury),
            "zeta1" => Ok(PredictionProvider::Zeta1),
            "zeta2" => {
                let version = arg.map(ZetaVersion::parse).transpose()?.unwrap_or_default();
                Ok(PredictionProvider::Zeta2(version))
            }
            "teacher" => {
                let backend = arg
                    .map(|a| a.parse())
                    .transpose()?
                    .unwrap_or(TeacherBackend::Sonnet45);
                Ok(PredictionProvider::Teacher(backend))
            }
            "teacher-non-batching" | "teacher_non_batching" | "teachernonbatching" => {
                let backend = arg
                    .map(|a| a.parse())
                    .transpose()?
                    .unwrap_or(TeacherBackend::Sonnet45);
                Ok(PredictionProvider::TeacherNonBatching(backend))
            }
            "repair" => Ok(PredictionProvider::Repair),
            _ => {
                anyhow::bail!(
                    "unknown provider `{provider}`. Valid options: sweep, mercury, zeta1, zeta2, zeta2:<version>, teacher, teacher:<backend>, teacher-non-batching, repair\n\
                 For zeta2, you can optionally specify a version like `zeta2:ordered` or `zeta2:V0113_Ordered`.\n\
                 For teacher, you can specify a backend like `teacher:sonnet45` or `teacher:gpt52`.\n\
                 Available zeta versions:\n{}",
                    ZetaVersion::options_as_string()
                )
            }
        }
    }
}

impl Serialize for PredictionProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PredictionProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
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

#[derive(Debug, Args, Clone)]
struct ImportBatchArgs {
    /// Batch IDs to import (e.g., msgbatch_xxx for Anthropic, batch_xxx for OpenAI)
    #[clap(long, required = true, num_args = 1..)]
    batch_ids: Vec<String>,
    /// Which provider's batches to import (anthropic or openai)
    #[clap(long, default_value = "anthropic")]
    provider: BatchProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum BatchProvider {
    Anthropic,
    Openai,
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
    let mut rejected_after_timestamps = Vec::new();
    let mut requested_after_timestamps = Vec::new();
    let mut rated_after_inputs: Vec<(String, Option<telemetry_events::EditPredictionRating>)> =
        Vec::new();
    let mut file_inputs = Vec::new();

    for input in &args.inputs {
        let input_string = input.to_string_lossy();
        if let Some(timestamp) = pull_examples::parse_captured_after_input(input_string.as_ref()) {
            captured_after_timestamps.push(timestamp.to_string());
        } else if let Some(timestamp) =
            pull_examples::parse_rejected_after_input(input_string.as_ref())
        {
            rejected_after_timestamps.push(timestamp.to_string());
        } else if let Some(timestamp) =
            pull_examples::parse_requested_after_input(input_string.as_ref())
        {
            requested_after_timestamps.push(timestamp.to_string());
        } else if let Some((timestamp, rating_filter)) =
            pull_examples::parse_rated_after_input(input_string.as_ref())
        {
            rated_after_inputs.push((timestamp.to_string(), rating_filter));
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
            "skipping Snowflake inputs because --limit is already satisfied by example files"
        );
    } else {
        let max_rows_per_timestamp = remaining_limit_for_snowflake.unwrap_or(5000);

        if !captured_after_timestamps.is_empty() {
            captured_after_timestamps.sort();

            let mut captured_examples = pull_examples::fetch_captured_examples_after(
                http_client.clone(),
                &captured_after_timestamps,
                max_rows_per_timestamp,
                background_executor.clone(),
            )
            .await?;
            examples.append(&mut captured_examples);
        }

        if !rejected_after_timestamps.is_empty() {
            rejected_after_timestamps.sort();

            let mut rejected_examples = pull_examples::fetch_rejected_examples_after(
                http_client.clone(),
                &rejected_after_timestamps,
                max_rows_per_timestamp,
                background_executor.clone(),
            )
            .await?;
            examples.append(&mut rejected_examples);
        }

        if !requested_after_timestamps.is_empty() {
            requested_after_timestamps.sort();

            let mut requested_examples = pull_examples::fetch_requested_examples_after(
                http_client.clone(),
                &requested_after_timestamps,
                max_rows_per_timestamp,
                background_executor.clone(),
            )
            .await?;
            examples.append(&mut requested_examples);
        }

        if !rated_after_inputs.is_empty() {
            rated_after_inputs.sort();

            let mut rated_examples = pull_examples::fetch_rated_examples_after(
                http_client,
                &rated_after_inputs,
                max_rows_per_timestamp,
                background_executor,
            )
            .await?;
            examples.append(&mut rated_examples);
        }
    }

    crate::example::sort_examples_by_repo_and_rev(&mut examples);

    if let Some(name_filter) = &args.name {
        examples.retain(|example| example.spec.name.contains(name_filter));
    }
    if let Some(repo_filter) = &args.repo {
        examples.retain(|example| example.spec.repository_url.contains(repo_filter));
    }

    // Skip resume logic for --in-place since input and output are the same file,
    // which would incorrectly treat all input examples as already processed.
    if !args.in_place {
        if let Some(path) = output_path
            && let Some(command) = &args.command
        {
            resume_from_output(path, &mut examples, command);
        }
    }

    if let Some(offset) = args.offset {
        examples.splice(0..offset, []);
    }

    if let Some(limit) = args.limit {
        examples.truncate(limit);
    }

    let progress = Progress::global();
    progress.set_total_examples(examples.len());
    progress.set_max_example_name_len(examples.iter().map(|e| &e.spec.name));

    Ok(examples)
}

fn spec_hash(spec: &edit_prediction::example_spec::ExampleSpec) -> u64 {
    let mut hasher = collections::FxHasher::default();
    spec.hash(&mut hasher);
    hasher.finish()
}

fn resume_from_output(path: &PathBuf, examples: &mut Vec<Example>, command: &Command) {
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
                let is_complete = match command {
                    Command::Qa(_) => output_example
                        .qa
                        .first()
                        .and_then(|q| q.as_ref())
                        .and_then(|q| q.confidence)
                        .is_some(),
                    Command::Repair(_) => output_example.predictions.iter().any(|p| {
                        p.provider == PredictionProvider::Repair && p.actual_patch.is_some()
                    }),
                    _ => true,
                };
                if is_complete {
                    kept_hashes.insert(hash);
                    kept_lines.push(line);
                }
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

    if args.markdown && output.is_none() {
        eprintln!("--markdown requires -o to specify the output directory");
        std::process::exit(1);
    }

    let command = match &args.command {
        Some(cmd) => cmd.clone(),
        None => {
            EpArgs::command().print_help().unwrap();
            return;
        }
    };

    match &command {
        Command::ImportBatch(import_args) => {
            smol::block_on(async {
                match import_args.provider {
                    BatchProvider::Anthropic => {
                        let client = anthropic_client::AnthropicClient::batch(&paths::LLM_CACHE_DB)
                            .expect("Failed to create Anthropic client");
                        if let Err(e) = client.import_batches(&import_args.batch_ids).await {
                            eprintln!("Error importing Anthropic batches: {:?}", e);
                            std::process::exit(1);
                        }
                    }
                    BatchProvider::Openai => {
                        let client = openai_client::OpenAiClient::batch(&paths::LLM_CACHE_DB)
                            .expect("Failed to create OpenAI client");
                        if let Err(e) = client.import_batches(&import_args.batch_ids).await {
                            eprintln!("Error importing OpenAI batches: {:?}", e);
                            std::process::exit(1);
                        }
                    }
                }
                println!(
                    "Successfully imported {} batch(es)",
                    import_args.batch_ids.len()
                );
            });
            return;
        }
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
            if let Err(error) = split_commit::run_split_commit(
                split_commit_args,
                &args.inputs,
                output.as_ref(),
                args.failed,
            ) {
                eprintln!("{error:#}");
                std::process::exit(1);
            }
            return;
        }
        Command::TruncatePatch(truncate_args) => {
            if let Err(error) =
                truncate_expected_patch::run_truncate_expected_patch(truncate_args, &args.inputs)
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
        Command::FilterLanguages(filter_args) => {
            if let Err(error) =
                run_filter_languages(filter_args, &args.inputs, args.output.as_ref())
            {
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
                let examples = load_examples(
                    app_state.client.http_client(),
                    &args,
                    output.as_ref(),
                    cx.background_executor().clone(),
                )
                .await?;

                match &command {
                    Command::Predict(args) | Command::Score(args) => {
                        predict::sync_batches(args.provider.as_ref()).await?;
                    }
                    Command::Eval(args) => {
                        predict::sync_batches(args.predict.provider.as_ref()).await?;
                    }
                    Command::Qa(args) => {
                        qa::sync_batches(args).await?;
                    }
                    Command::Repair(args) => {
                        repair::sync_batches(args).await?;
                    }
                    _ => (),
                }

                let failfast_on_single_example = examples.len() == 1;

                // For --markdown mode, create the output directory if it doesn't exist
                if args.markdown {
                    let dir = output.as_ref().expect("--markdown requires -o");
                    if !dir.exists() {
                        std::fs::create_dir_all(dir)
                            .expect("Failed to create markdown output directory");
                    }
                }

                // Set up JSONL output writer (not used in markdown mode)
                let mut output_sender: Option<mpsc::UnboundedSender<String>> = None;
                let mut in_place_temp_path: Option<PathBuf> = None;
                if !args.markdown
                    && let Some(output_path) = output.as_ref()
                {
                    let write_path = if args.in_place {
                        let temp = output_path.with_extension("jsonl.tmp");
                        in_place_temp_path = Some(temp.clone());
                        temp
                    } else {
                        output_path.clone()
                    };

                    let file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(args.in_place)
                        .append(!args.in_place)
                        .open(&write_path)
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
                    output_sender = Some(sender);
                }

                let grouped_examples = Mutex::new(group_examples_by_repo(examples));
                let finished_examples = Mutex::new(Vec::new());

                let mut tasks = Vec::new();
                for _ in 0..args.max_parallelism {
                    tasks.push(async {
                        loop {
                            let Some(mut repo_examples) =
                                grouped_examples.lock().unwrap().pop_front()
                            else {
                                break;
                            };
                            for example in &mut repo_examples {
                                let example_progress =
                                    Progress::global().start_group(&example.spec.name);

                                let result = async {
                                    match &command {
                                        Command::Read => {}
                                        Command::LoadProject => {
                                            run_load_project(
                                                example,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::Context => {
                                            run_context_retrieval(
                                                example,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::FormatPrompt(args) => {
                                            run_format_prompt(
                                                example,
                                                args,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::Predict(args) => {
                                            run_prediction(
                                                example,
                                                args,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::ParseOutput => {
                                            parse_output::run_parse_output(example)?;
                                        }
                                        Command::Distill => {
                                            run_distill(example).await?;
                                        }
                                        Command::Score(args) => {
                                            run_scoring(
                                                example,
                                                args,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::Eval(args) => {
                                            run_scoring(
                                                example,
                                                &args.predict,
                                                app_state.clone(),
                                                &example_progress,
                                                cx.clone(),
                                            )
                                            .await?;
                                        }
                                        Command::Qa(args) => {
                                            qa::run_qa(example, args, &example_progress).await?;
                                        }
                                        Command::Repair(args) => {
                                            repair::run_repair(example, args, &example_progress)
                                                .await?;
                                        }
                                        Command::Clean
                                        | Command::Synthesize(_)
                                        | Command::SplitCommit(_)
                                        | Command::Split(_)
                                        | Command::TruncatePatch(_)
                                        | Command::FilterLanguages(_)
                                        | Command::ImportBatch(_) => {
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
                                        &example,
                                    )
                                    .await;
                                    true
                                } else {
                                    false
                                };

                                let should_write = !failed || args.failed == FailedHandling::Keep;
                                if should_write {
                                    if args.markdown {
                                        let markdown_dir =
                                            output.as_ref().expect("--markdown requires -o");
                                        let filename = format!("{}.md", example.spec.filename());
                                        let path = markdown_dir.join(&filename);
                                        let markdown = example.spec.to_markdown();
                                        std::fs::write(&path, &markdown)
                                            .expect("Failed to write markdown file");
                                    } else if let Some(ref mut sender) = output_sender.clone() {
                                        let line = serde_json::to_string(&example).unwrap();
                                        sender
                                            .send(line)
                                            .await
                                            .expect("Failed to send to output writer");
                                    } else if args.output.is_none()
                                        && !matches!(command, Command::Eval(_))
                                    {
                                        let line = serde_json::to_string(&example).unwrap();
                                        println!("{}", line);
                                    }
                                }
                            }

                            let repo_url = &repo_examples.first().unwrap().spec.repository_url;
                            let project = repo_examples
                                .iter()
                                .find_map(|e| e.state.as_ref().map(|s| s.project.clone()))
                                .or_else(|| app_state.project_cache.get(repo_url));

                            if let Some(project) = project {
                                let mut cx = cx.clone();

                                let shutdown_task: Task<()> =
                                    project.update(&mut cx, |project, cx| {
                                        let lsp_store = project.lsp_store();
                                        lsp_store.update(cx, |lsp_store, cx| {
                                            lsp_store.shutdown_all_language_servers(cx)
                                        })
                                    });

                                shutdown_task.await;

                                if let Some(ep_store) =
                                    cx.update(|cx| EditPredictionStore::try_global(cx))
                                {
                                    ep_store.update(&mut cx, |store, _| {
                                        store.remove_project(&project);
                                    });
                                }
                            }

                            app_state.project_cache.remove(repo_url);
                            for example in &mut repo_examples {
                                example.state.take();
                            }
                            finished_examples
                                .lock()
                                .unwrap()
                                .extend_from_slice(&repo_examples);
                        }
                    });
                }
                futures::future::join_all(tasks).await;

                Progress::global().finalize();

                match &command {
                    Command::Predict(args) | Command::Score(args) => {
                        predict::sync_batches(args.provider.as_ref()).await?;
                    }
                    Command::Eval(args) => {
                        predict::sync_batches(args.predict.provider.as_ref()).await?;
                    }
                    Command::Qa(args) => {
                        qa::sync_batches(args).await?;
                    }
                    Command::Repair(args) => {
                        repair::sync_batches(args).await?;
                    }
                    _ => (),
                }

                match &command {
                    Command::Eval(args) => {
                        let examples = finished_examples.lock().unwrap();
                        score::print_report(&examples);
                        if let Some(summary_path) = &args.summary_json {
                            score::write_summary_json(&examples, summary_path)?;
                        }
                    }
                    _ => (),
                };

                // For --in-place, atomically rename temp file to original
                if let Some(temp_path) = &in_place_temp_path {
                    let final_path = output.as_ref().expect("in_place_temp_path requires output");
                    std::fs::rename(temp_path, final_path)
                        .expect("Failed to rename temp file to final output");
                }

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

    let msg;
    if !matches!(args.failed, FailedHandling::SkipNoFiles) {
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

        let cursor_path = match example.repo_name() {
            Ok(repo_name) => repo_name.worktree_path().join(&example.spec.cursor_path),
            Err(_) => example.spec.cursor_path.as_ref().to_path_buf(),
        };
        msg = format!(
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
    } else {
        msg = format!(
            indoc::indoc! {"
            While processing \"{}\":

                \x1b[31m{:?}\x1b[0m
            "},
            example.spec.name, error
        );
    }

    if args.failfast || failfast_on_single_example {
        Progress::global().finalize();
        panic!("{}", msg);
    } else {
        log::error!("{}", msg);
    }
}
