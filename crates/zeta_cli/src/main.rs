mod evaluate;
mod example;
mod headless;
mod metrics;
mod paths;
mod predict;
mod source_location;
mod syntax_retrieval_stats;
mod util;

use crate::{
    evaluate::run_evaluate,
    example::{ExampleFormat, NamedExample},
    headless::ZetaCliAppState,
    predict::run_predict,
    source_location::SourceLocation,
    syntax_retrieval_stats::retrieval_stats,
    util::{open_buffer, open_buffer_with_language_server},
};
use ::util::paths::PathStyle;
use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand, ValueEnum};
use cloud_llm_client::predict_edits_v3;
use edit_prediction_context::EditPredictionExcerptOptions;
use gpui::{Application, AsyncApp, Entity, prelude::*};
use language::{Bias, Buffer, BufferSnapshot, Point};
use metrics::delta_chr_f;
use project::{Project, Worktree, lsp_store::OpenLspBufferHandle};
use reqwest_client::ReqwestClient;
use std::io::{self};
use std::time::Duration;
use std::{collections::HashSet, path::PathBuf, str::FromStr, sync::Arc};
use zeta::ContextMode;
use zeta::udiff::DiffLine;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct ZetaCliArgs {
    #[arg(long, default_value_t = false)]
    printenv: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Context(ContextArgs),
    ContextStats(ContextStatsArgs),
    Predict(PredictArguments),
    Eval(EvaluateArguments),
    ConvertExample {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = ExampleFormat::Md)]
        output_format: ExampleFormat,
    },
    Score {
        golden_patch: PathBuf,
        actual_patch: PathBuf,
    },
    Clean,
}

#[derive(Debug, Args)]
struct ContextStatsArgs {
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    extension: Option<String>,
    #[arg(long)]
    limit: Option<usize>,
    #[arg(long)]
    skip: Option<usize>,
    #[clap(flatten)]
    zeta2_args: Zeta2Args,
}

#[derive(Debug, Args)]
struct ContextArgs {
    #[arg(long)]
    provider: ContextProvider,
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    cursor: SourceLocation,
    #[arg(long)]
    use_language_server: bool,
    #[arg(long)]
    edit_history: Option<FileOrStdin>,
    #[clap(flatten)]
    zeta2_args: Zeta2Args,
}

#[derive(clap::ValueEnum, Default, Debug, Clone, Copy)]
enum ContextProvider {
    Zeta1,
    #[default]
    Zeta2,
}

#[derive(Clone, Debug, Args)]
struct Zeta2Args {
    #[arg(long, default_value_t = 8192)]
    max_prompt_bytes: usize,
    #[arg(long, default_value_t = 2048)]
    max_excerpt_bytes: usize,
    #[arg(long, default_value_t = 1024)]
    min_excerpt_bytes: usize,
    #[arg(long, default_value_t = 0.66)]
    target_before_cursor_over_total_bytes: f32,
    #[arg(long, default_value_t = 1024)]
    max_diagnostic_bytes: usize,
    #[arg(long, value_enum, default_value_t = PromptFormat::default())]
    prompt_format: PromptFormat,
    #[arg(long, value_enum, default_value_t = Default::default())]
    output_format: OutputFormat,
    #[arg(long, default_value_t = 42)]
    file_indexing_parallelism: usize,
    #[arg(long, default_value_t = false)]
    disable_imports_gathering: bool,
    #[arg(long, default_value_t = u8::MAX)]
    max_retrieved_definitions: u8,
}

#[derive(Debug, Args)]
pub struct PredictArguments {
    #[clap(long, short, value_enum, default_value_t = PredictionsOutputFormat::Md)]
    format: PredictionsOutputFormat,
    example_path: PathBuf,
    #[clap(flatten)]
    options: PredictionOptions,
}

#[derive(Clone, Debug, Args)]
pub struct PredictionOptions {
    #[clap(flatten)]
    zeta2: Zeta2Args,
    #[clap(long)]
    provider: PredictionProvider,
    #[clap(long, value_enum, default_value_t = CacheMode::default())]
    cache: CacheMode,
}

#[derive(Debug, ValueEnum, Default, Clone, Copy, PartialEq)]
pub enum CacheMode {
    /// Use cached LLM requests and responses, except when multiple repetitions are requested
    #[default]
    Auto,
    /// Use cached LLM requests and responses, based on the hash of the prompt and the endpoint.
    #[value(alias = "request")]
    Requests,
    /// Ignore existing cache entries for both LLM and search.
    Skip,
    /// Use cached LLM responses AND search results for full determinism. Fails if they haven't been cached yet.
    /// Useful for reproducing results and fixing bugs outside of search queries
    Force,
}

impl CacheMode {
    fn use_cached_llm_responses(&self) -> bool {
        self.assert_not_auto();
        matches!(self, CacheMode::Requests | CacheMode::Force)
    }

    fn use_cached_search_results(&self) -> bool {
        self.assert_not_auto();
        matches!(self, CacheMode::Force)
    }

    fn assert_not_auto(&self) {
        assert_ne!(
            *self,
            CacheMode::Auto,
            "Cache mode should not be auto at this point!"
        );
    }
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum PredictionsOutputFormat {
    Json,
    Md,
    Diff,
}

#[derive(Debug, Args)]
pub struct EvaluateArguments {
    example_paths: Vec<PathBuf>,
    #[clap(flatten)]
    options: PredictionOptions,
    #[clap(short, long, default_value_t = 1, alias = "repeat")]
    repetitions: u16,
    #[arg(long)]
    skip_prediction: bool,
}

#[derive(clap::ValueEnum, Default, Debug, Clone, Copy, PartialEq)]
enum PredictionProvider {
    Zeta1,
    #[default]
    Zeta2,
    Sweep,
}

fn zeta2_args_to_options(args: &Zeta2Args) -> zeta::ZetaOptions {
    zeta::ZetaOptions {
        context: ContextMode::Lsp(EditPredictionExcerptOptions {
            max_bytes: args.max_excerpt_bytes,
            min_bytes: args.min_excerpt_bytes,
            target_before_cursor_over_total_bytes: args.target_before_cursor_over_total_bytes,
        }),
        max_diagnostic_bytes: args.max_diagnostic_bytes,
        max_prompt_bytes: args.max_prompt_bytes,
        prompt_format: args.prompt_format.into(),
        file_indexing_parallelism: args.file_indexing_parallelism,
        buffer_change_grouping_interval: Duration::ZERO,
    }
}

#[derive(clap::ValueEnum, Default, Debug, Clone, Copy)]
enum PromptFormat {
    MarkedExcerpt,
    LabeledSections,
    OnlySnippets,
    #[default]
    NumberedLines,
    OldTextNewText,
    Minimal,
    MinimalQwen,
    SeedCoder1120,
}

impl Into<predict_edits_v3::PromptFormat> for PromptFormat {
    fn into(self) -> predict_edits_v3::PromptFormat {
        match self {
            Self::MarkedExcerpt => predict_edits_v3::PromptFormat::MarkedExcerpt,
            Self::LabeledSections => predict_edits_v3::PromptFormat::LabeledSections,
            Self::OnlySnippets => predict_edits_v3::PromptFormat::OnlySnippets,
            Self::NumberedLines => predict_edits_v3::PromptFormat::NumLinesUniDiff,
            Self::OldTextNewText => predict_edits_v3::PromptFormat::OldTextNewText,
            Self::Minimal => predict_edits_v3::PromptFormat::Minimal,
            Self::MinimalQwen => predict_edits_v3::PromptFormat::MinimalQwen,
            Self::SeedCoder1120 => predict_edits_v3::PromptFormat::SeedCoder1120,
        }
    }
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum OutputFormat {
    #[default]
    Prompt,
    Request,
    Full,
}

#[derive(Debug, Clone)]
enum FileOrStdin {
    File(PathBuf),
    Stdin,
}

impl FileOrStdin {
    async fn read_to_string(&self) -> Result<String, std::io::Error> {
        match self {
            FileOrStdin::File(path) => smol::fs::read_to_string(path).await,
            FileOrStdin::Stdin => smol::unblock(|| std::io::read_to_string(std::io::stdin())).await,
        }
    }
}

impl FromStr for FileOrStdin {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdin),
            _ => Ok(Self::File(PathBuf::from_str(s)?)),
        }
    }
}

struct LoadedContext {
    full_path_str: String,
    snapshot: BufferSnapshot,
    clipped_cursor: Point,
    worktree: Entity<Worktree>,
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    lsp_open_handle: Option<OpenLspBufferHandle>,
}

async fn load_context(
    args: &ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<LoadedContext> {
    let ContextArgs {
        worktree: worktree_path,
        cursor,
        use_language_server,
        ..
    } = args;

    let worktree_path = worktree_path.canonicalize()?;

    let project = cx.update(|cx| {
        Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            cx,
        )
    })?;

    let worktree = project
        .update(cx, |project, cx| {
            project.create_worktree(&worktree_path, true, cx)
        })?
        .await?;

    let mut ready_languages = HashSet::default();
    let (lsp_open_handle, buffer) = if *use_language_server {
        let (lsp_open_handle, _, buffer) = open_buffer_with_language_server(
            project.clone(),
            worktree.clone(),
            cursor.path.clone(),
            &mut ready_languages,
            cx,
        )
        .await?;
        (Some(lsp_open_handle), buffer)
    } else {
        let buffer =
            open_buffer(project.clone(), worktree.clone(), cursor.path.clone(), cx).await?;
        (None, buffer)
    };

    let full_path_str = worktree
        .read_with(cx, |worktree, _| worktree.root_name().join(&cursor.path))?
        .display(PathStyle::local())
        .to_string();

    let snapshot = cx.update(|cx| buffer.read(cx).snapshot())?;
    let clipped_cursor = snapshot.clip_point(cursor.point, Bias::Left);
    if clipped_cursor != cursor.point {
        let max_row = snapshot.max_point().row;
        if cursor.point.row < max_row {
            return Err(anyhow!(
                "Cursor position {:?} is out of bounds (line length is {})",
                cursor.point,
                snapshot.line_len(cursor.point.row)
            ));
        } else {
            return Err(anyhow!(
                "Cursor position {:?} is out of bounds (max row is {})",
                cursor.point,
                max_row
            ));
        }
    }

    Ok(LoadedContext {
        full_path_str,
        snapshot,
        clipped_cursor,
        worktree,
        project,
        buffer,
        lsp_open_handle,
    })
}

async fn zeta2_context(
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<String> {
    let LoadedContext {
        worktree,
        project,
        buffer,
        clipped_cursor,
        lsp_open_handle: _handle,
        ..
    } = load_context(&args, app_state, cx).await?;

    // wait for worktree scan before starting zeta2 so that wait_for_initial_indexing waits for
    // the whole worktree.
    worktree
        .read_with(cx, |worktree, _cx| {
            worktree.as_local().unwrap().scan_complete()
        })?
        .await;
    let output = cx
        .update(|cx| {
            let zeta = cx.new(|cx| {
                zeta::Zeta::new(app_state.client.clone(), app_state.user_store.clone(), cx)
            });
            let indexing_done_task = zeta.update(cx, |zeta, cx| {
                zeta.set_options(zeta2_args_to_options(&args.zeta2_args));
                zeta.register_buffer(&buffer, &project, cx);
                zeta.wait_for_initial_indexing(&project, cx)
            });
            cx.spawn(async move |cx| {
                indexing_done_task.await?;
                let updates_rx = zeta.update(cx, |zeta, cx| {
                    let cursor = buffer.read(cx).snapshot().anchor_before(clipped_cursor);
                    zeta.set_use_context(true);
                    zeta.refresh_context_if_needed(&project, &buffer, cursor, cx);
                    zeta.project_context_updates(&project).unwrap()
                })?;

                updates_rx.recv().await.ok();

                let context = zeta.update(cx, |zeta, cx| {
                    zeta.context_for_project(&project, cx).to_vec()
                })?;

                anyhow::Ok(serde_json::to_string_pretty(&context).unwrap())
            })
        })?
        .await?;

    Ok(output)
}

async fn zeta1_context(
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<zeta::zeta1::GatherContextOutput> {
    let LoadedContext {
        full_path_str,
        snapshot,
        clipped_cursor,
        ..
    } = load_context(&args, app_state, cx).await?;

    let events = match args.edit_history {
        Some(events) => events.read_to_string().await?,
        None => String::new(),
    };

    let prompt_for_events = move || (events, 0);
    cx.update(|cx| {
        zeta::zeta1::gather_context(
            full_path_str,
            &snapshot,
            clipped_cursor,
            prompt_for_events,
            cloud_llm_client::PredictEditsRequestTrigger::Cli,
            cx,
        )
    })?
    .await
}

fn main() {
    zlog::init();
    zlog::init_output_stderr();
    let args = ZetaCliArgs::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = Arc::new(headless::init(cx));
        cx.spawn(async move |cx| {
            match args.command {
                None => {
                    if args.printenv {
                        ::util::shell_env::print_env();
                    } else {
                        panic!("Expected a command");
                    }
                }
                Some(Command::ContextStats(arguments)) => {
                    let result = retrieval_stats(
                        arguments.worktree,
                        app_state,
                        arguments.extension,
                        arguments.limit,
                        arguments.skip,
                        zeta2_args_to_options(&arguments.zeta2_args),
                        cx,
                    )
                    .await;
                    println!("{}", result.unwrap());
                }
                Some(Command::Context(context_args)) => {
                    let result = match context_args.provider {
                        ContextProvider::Zeta1 => {
                            let context =
                                zeta1_context(context_args, &app_state, cx).await.unwrap();
                            serde_json::to_string_pretty(&context.body).unwrap()
                        }
                        ContextProvider::Zeta2 => {
                            zeta2_context(context_args, &app_state, cx).await.unwrap()
                        }
                    };
                    println!("{}", result);
                }
                Some(Command::Predict(arguments)) => {
                    run_predict(arguments, &app_state, cx).await;
                }
                Some(Command::Eval(arguments)) => {
                    run_evaluate(arguments, &app_state, cx).await;
                }
                Some(Command::ConvertExample {
                    path,
                    output_format,
                }) => {
                    let example = NamedExample::load(path).unwrap();
                    example.write(output_format, io::stdout()).unwrap();
                }
                Some(Command::Score {
                    golden_patch,
                    actual_patch,
                }) => {
                    let golden_content = std::fs::read_to_string(golden_patch).unwrap();
                    let actual_content = std::fs::read_to_string(actual_patch).unwrap();

                    let golden_diff: Vec<DiffLine> = golden_content
                        .lines()
                        .map(|line| DiffLine::parse(line))
                        .collect();

                    let actual_diff: Vec<DiffLine> = actual_content
                        .lines()
                        .map(|line| DiffLine::parse(line))
                        .collect();

                    let score = delta_chr_f(&golden_diff, &actual_diff);
                    println!("{:.2}", score);
                }
                Some(Command::Clean) => {
                    std::fs::remove_dir_all(&*crate::paths::TARGET_ZETA_DIR).unwrap()
                }
            };

            let _ = cx.update(|cx| cx.quit());
        })
        .detach();
    });
}
