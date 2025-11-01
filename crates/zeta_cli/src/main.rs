mod example;
mod headless;
mod source_location;
mod syntax_retrieval_stats;
mod util;

use crate::example::{ExampleFormat, NamedExample};
use crate::syntax_retrieval_stats::retrieval_stats;
use ::serde::Serialize;
use ::util::paths::PathStyle;
use anyhow::{Context as _, Result, anyhow};
use clap::{Args, Parser, Subcommand};
use cloud_llm_client::predict_edits_v3::{self, Excerpt};
use cloud_zeta2_prompt::{CURSOR_MARKER, write_codeblock};
use edit_prediction_context::{
    EditPredictionContextOptions, EditPredictionExcerpt, EditPredictionExcerptOptions,
    EditPredictionScoreOptions, Line,
};
use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{Application, AsyncApp, Entity, prelude::*};
use language::{Bias, Buffer, BufferSnapshot, OffsetRangeExt, Point};
use language_model::LanguageModelRegistry;
use project::{Project, Worktree};
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::io;
use std::{collections::HashSet, path::PathBuf, process::exit, str::FromStr, sync::Arc};
use zeta2::{ContextMode, LlmContextOptions, SearchToolQuery};

use crate::headless::ZetaCliAppState;
use crate::source_location::SourceLocation;
use crate::util::{open_buffer, open_buffer_with_language_server};

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct ZetaCliArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Zeta1 {
        #[command(subcommand)]
        command: Zeta1Command,
    },
    Zeta2 {
        #[clap(flatten)]
        args: Zeta2Args,
        #[command(subcommand)]
        command: Zeta2Command,
    },
    ConvertExample {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = ExampleFormat::Md)]
        output_format: ExampleFormat,
    },
}

#[derive(Subcommand, Debug)]
enum Zeta1Command {
    Context {
        #[clap(flatten)]
        context_args: ContextArgs,
    },
}

#[derive(Subcommand, Debug)]
enum Zeta2Command {
    Syntax {
        #[clap(flatten)]
        syntax_args: Zeta2SyntaxArgs,
        #[command(subcommand)]
        command: Zeta2SyntaxCommand,
    },
    Llm {
        #[command(subcommand)]
        command: Zeta2LlmCommand,
    },
}

#[derive(Subcommand, Debug)]
enum Zeta2SyntaxCommand {
    Context {
        #[clap(flatten)]
        context_args: ContextArgs,
    },
    Stats {
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        extension: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long)]
        skip: Option<usize>,
    },
}

#[derive(Subcommand, Debug)]
enum Zeta2LlmCommand {
    Context {
        #[clap(flatten)]
        context_args: ContextArgs,
    },
}

#[derive(Debug, Args)]
#[group(requires = "worktree")]
struct ContextArgs {
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    cursor: SourceLocation,
    #[arg(long)]
    use_language_server: bool,
    #[arg(long)]
    edit_history: Option<FileOrStdin>,
}

#[derive(Debug, Args)]
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
}

#[derive(Debug, Args)]
struct Zeta2SyntaxArgs {
    #[arg(long, default_value_t = false)]
    disable_imports_gathering: bool,
    #[arg(long, default_value_t = u8::MAX)]
    max_retrieved_definitions: u8,
}

fn syntax_args_to_options(
    zeta2_args: &Zeta2Args,
    syntax_args: &Zeta2SyntaxArgs,
    omit_excerpt_overlaps: bool,
) -> zeta2::ZetaOptions {
    zeta2::ZetaOptions {
        context: ContextMode::Syntax(EditPredictionContextOptions {
            max_retrieved_declarations: syntax_args.max_retrieved_definitions,
            use_imports: !syntax_args.disable_imports_gathering,
            excerpt: EditPredictionExcerptOptions {
                max_bytes: zeta2_args.max_excerpt_bytes,
                min_bytes: zeta2_args.min_excerpt_bytes,
                target_before_cursor_over_total_bytes: zeta2_args
                    .target_before_cursor_over_total_bytes,
            },
            score: EditPredictionScoreOptions {
                omit_excerpt_overlaps,
            },
        }),
        max_diagnostic_bytes: zeta2_args.max_diagnostic_bytes,
        max_prompt_bytes: zeta2_args.max_prompt_bytes,
        prompt_format: zeta2_args.prompt_format.clone().into(),
        file_indexing_parallelism: zeta2_args.file_indexing_parallelism,
    }
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum PromptFormat {
    MarkedExcerpt,
    LabeledSections,
    OnlySnippets,
    #[default]
    NumberedLines,
}

impl Into<predict_edits_v3::PromptFormat> for PromptFormat {
    fn into(self) -> predict_edits_v3::PromptFormat {
        match self {
            Self::MarkedExcerpt => predict_edits_v3::PromptFormat::MarkedExcerpt,
            Self::LabeledSections => predict_edits_v3::PromptFormat::LabeledSections,
            Self::OnlySnippets => predict_edits_v3::PromptFormat::OnlySnippets,
            Self::NumberedLines => predict_edits_v3::PromptFormat::NumLinesUniDiff,
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
    let (_lsp_open_handle, buffer) = if *use_language_server {
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
    })
}

async fn zeta2_syntax_context(
    zeta2_args: Zeta2Args,
    syntax_args: Zeta2SyntaxArgs,
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<String> {
    let LoadedContext {
        worktree,
        project,
        buffer,
        clipped_cursor,
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
                zeta2::Zeta::new(app_state.client.clone(), app_state.user_store.clone(), cx)
            });
            let indexing_done_task = zeta.update(cx, |zeta, cx| {
                zeta.set_options(syntax_args_to_options(&zeta2_args, &syntax_args, true));
                zeta.register_buffer(&buffer, &project, cx);
                zeta.wait_for_initial_indexing(&project, cx)
            });
            cx.spawn(async move |cx| {
                indexing_done_task.await?;
                let request = zeta
                    .update(cx, |zeta, cx| {
                        let cursor = buffer.read(cx).snapshot().anchor_before(clipped_cursor);
                        zeta.cloud_request_for_zeta_cli(&project, &buffer, cursor, cx)
                    })?
                    .await?;

                let (prompt_string, section_labels) = cloud_zeta2_prompt::build_prompt(&request)?;

                match zeta2_args.output_format {
                    OutputFormat::Prompt => anyhow::Ok(prompt_string),
                    OutputFormat::Request => anyhow::Ok(serde_json::to_string_pretty(&request)?),
                    OutputFormat::Full => anyhow::Ok(serde_json::to_string_pretty(&json!({
                        "request": request,
                        "prompt": prompt_string,
                        "section_labels": section_labels,
                    }))?),
                }
            })
        })?
        .await?;

    Ok(output)
}

async fn zeta2_llm_context(
    zeta2_args: Zeta2Args,
    context_args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<String> {
    let LoadedContext {
        buffer,
        clipped_cursor,
        snapshot: cursor_snapshot,
        project,
        ..
    } = load_context(&context_args, app_state, cx).await?;

    let cursor_position = cursor_snapshot.anchor_after(clipped_cursor);

    cx.update(|cx| {
        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry
                .provider(&zeta2::related_excerpts::MODEL_PROVIDER_ID)
                .unwrap()
                .authenticate(cx)
        })
    })?
    .await?;

    let edit_history_unified_diff = match context_args.edit_history {
        Some(events) => events.read_to_string().await?,
        None => String::new(),
    };

    let (debug_tx, mut debug_rx) = mpsc::unbounded();

    let excerpt_options = EditPredictionExcerptOptions {
        max_bytes: zeta2_args.max_excerpt_bytes,
        min_bytes: zeta2_args.min_excerpt_bytes,
        target_before_cursor_over_total_bytes: zeta2_args.target_before_cursor_over_total_bytes,
    };

    let related_excerpts = cx
        .update(|cx| {
            zeta2::related_excerpts::find_related_excerpts(
                buffer,
                cursor_position,
                &project,
                edit_history_unified_diff,
                &LlmContextOptions {
                    excerpt: excerpt_options.clone(),
                },
                Some(debug_tx),
                cx,
            )
        })?
        .await?;

    let cursor_excerpt = EditPredictionExcerpt::select_from_buffer(
        clipped_cursor,
        &cursor_snapshot,
        &excerpt_options,
        None,
    )
    .context("line didn't fit")?;

    #[derive(Serialize)]
    struct Output {
        excerpts: Vec<OutputExcerpt>,
        formatted_excerpts: String,
        meta: OutputMeta,
    }

    #[derive(Default, Serialize)]
    struct OutputMeta {
        search_prompt: String,
        search_queries: Vec<SearchToolQuery>,
    }

    #[derive(Serialize)]
    struct OutputExcerpt {
        path: PathBuf,
        #[serde(flatten)]
        excerpt: Excerpt,
    }

    let mut meta = OutputMeta::default();

    while let Some(debug_info) = debug_rx.next().await {
        match debug_info {
            zeta2::ZetaDebugInfo::ContextRetrievalStarted(info) => {
                meta.search_prompt = info.search_prompt;
            }
            zeta2::ZetaDebugInfo::SearchQueriesGenerated(info) => {
                meta.search_queries = info.queries
            }
            _ => {}
        }
    }

    cx.update(|cx| {
        let mut excerpts = Vec::new();
        let mut formatted_excerpts = String::new();

        let cursor_insertions = [(
            predict_edits_v3::Point {
                line: Line(clipped_cursor.row),
                column: clipped_cursor.column,
            },
            CURSOR_MARKER,
        )];

        let mut cursor_excerpt_added = false;

        for (buffer, ranges) in related_excerpts {
            let excerpt_snapshot = buffer.read(cx).snapshot();

            let mut line_ranges = ranges
                .into_iter()
                .map(|range| {
                    let point_range = range.to_point(&excerpt_snapshot);
                    Line(point_range.start.row)..Line(point_range.end.row)
                })
                .collect::<Vec<_>>();

            let Some(file) = excerpt_snapshot.file() else {
                continue;
            };
            let path = file.full_path(cx);

            let is_cursor_file = path == cursor_snapshot.file().unwrap().full_path(cx);
            if is_cursor_file {
                let insertion_ix = line_ranges
                    .binary_search_by(|probe| {
                        probe
                            .start
                            .cmp(&cursor_excerpt.line_range.start)
                            .then(cursor_excerpt.line_range.end.cmp(&probe.end))
                    })
                    .unwrap_or_else(|ix| ix);
                line_ranges.insert(insertion_ix, cursor_excerpt.line_range.clone());
                cursor_excerpt_added = true;
            }

            let merged_excerpts =
                zeta2::merge_excerpts::merge_excerpts(&excerpt_snapshot, line_ranges)
                    .into_iter()
                    .map(|excerpt| OutputExcerpt {
                        path: path.clone(),
                        excerpt,
                    });

            let excerpt_start_ix = excerpts.len();
            excerpts.extend(merged_excerpts);

            write_codeblock(
                &path,
                excerpts[excerpt_start_ix..].iter().map(|e| &e.excerpt),
                if is_cursor_file {
                    &cursor_insertions
                } else {
                    &[]
                },
                Line(excerpt_snapshot.max_point().row),
                true,
                &mut formatted_excerpts,
            );
        }

        if !cursor_excerpt_added {
            write_codeblock(
                &cursor_snapshot.file().unwrap().full_path(cx),
                &[Excerpt {
                    start_line: cursor_excerpt.line_range.start,
                    text: cursor_excerpt.text(&cursor_snapshot).body.into(),
                }],
                &cursor_insertions,
                Line(cursor_snapshot.max_point().row),
                true,
                &mut formatted_excerpts,
            );
        }

        let output = Output {
            excerpts,
            formatted_excerpts,
            meta,
        };

        Ok(serde_json::to_string_pretty(&output)?)
    })
    .unwrap()
}

async fn zeta1_context(
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<zeta::GatherContextOutput> {
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
        zeta::gather_context(
            full_path_str,
            &snapshot,
            clipped_cursor,
            prompt_for_events,
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
            let result = match args.command {
                Command::Zeta1 {
                    command: Zeta1Command::Context { context_args },
                } => {
                    let context = zeta1_context(context_args, &app_state, cx).await.unwrap();
                    serde_json::to_string_pretty(&context.body).map_err(|err| anyhow::anyhow!(err))
                }
                Command::Zeta2 { args, command } => match command {
                    Zeta2Command::Syntax {
                        syntax_args,
                        command,
                    } => match command {
                        Zeta2SyntaxCommand::Context { context_args } => {
                            zeta2_syntax_context(args, syntax_args, context_args, &app_state, cx)
                                .await
                        }
                        Zeta2SyntaxCommand::Stats {
                            worktree,
                            extension,
                            limit,
                            skip,
                        } => {
                            retrieval_stats(
                                worktree,
                                app_state,
                                extension,
                                limit,
                                skip,
                                syntax_args_to_options(&args, &syntax_args, false),
                                cx,
                            )
                            .await
                        }
                    },
                    Zeta2Command::Llm { command } => match command {
                        Zeta2LlmCommand::Context { context_args } => {
                            zeta2_llm_context(args, context_args, &app_state, cx).await
                        }
                    },
                },
                Command::ConvertExample {
                    path,
                    output_format,
                } => {
                    let example = NamedExample::load(path).unwrap();
                    example.write(output_format, io::stdout()).unwrap();
                    let _ = cx.update(|cx| cx.quit());
                    return;
                }
            };

            match result {
                Ok(output) => {
                    println!("{}", output);
                    let _ = cx.update(|cx| cx.quit());
                }
                Err(e) => {
                    eprintln!("Failed: {:?}", e);
                    exit(1);
                }
            }
        })
        .detach();
    });
}
