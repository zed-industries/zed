mod headless;

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};
use cloud_llm_client::predict_edits_v3;
use edit_prediction_context::{
    Declaration, EditPredictionContext, EditPredictionExcerptOptions, Identifier, ReferenceRegion,
    SyntaxIndex, references_in_range,
};
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, Application, AsyncApp};
use gpui::{Entity, Task};
use language::{Bias, LanguageServerId};
use language::{Buffer, OffsetRangeExt};
use language::{LanguageId, Point};
use language_model::LlmApiToken;
use ordered_float::OrderedFloat;
use project::{Project, ProjectPath, Worktree};
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use util::{RangeExt, ResultExt as _};
use zeta::{PerformPredictEditsParams, Zeta};

use crate::headless::ZetaCliAppState;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct ZetaCliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Context(ContextArgs),
    Zeta2Context {
        #[clap(flatten)]
        zeta2_args: Zeta2Args,
        #[clap(flatten)]
        context_args: ContextArgs,
    },
    Predict {
        #[arg(long)]
        predict_edits_body: Option<FileOrStdin>,
        #[clap(flatten)]
        context_args: Option<ContextArgs>,
    },
    RetrievalStats {
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long, default_value_t = 42)]
        file_indexing_parallelism: usize,
    },
}

#[derive(Debug, Args)]
#[group(requires = "worktree")]
struct ContextArgs {
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    cursor: CursorPosition,
    #[arg(long)]
    use_language_server: bool,
    #[arg(long)]
    events: Option<FileOrStdin>,
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

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum PromptFormat {
    #[default]
    MarkedExcerpt,
    LabeledSections,
    OnlySnippets,
}

impl Into<predict_edits_v3::PromptFormat> for PromptFormat {
    fn into(self) -> predict_edits_v3::PromptFormat {
        match self {
            Self::MarkedExcerpt => predict_edits_v3::PromptFormat::MarkedExcerpt,
            Self::LabeledSections => predict_edits_v3::PromptFormat::LabeledSections,
            Self::OnlySnippets => predict_edits_v3::PromptFormat::OnlySnippets,
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

#[derive(Debug, Clone)]
struct CursorPosition {
    path: Arc<RelPath>,
    point: Point,
}

impl FromStr for CursorPosition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid cursor format. Expected 'file.rs:line:column', got '{}'",
                s
            ));
        }

        let path = RelPath::new(Path::new(&parts[0]), PathStyle::local())?.into_arc();
        let line: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid line number: '{}'", parts[1]))?;
        let column: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid column number: '{}'", parts[2]))?;

        // Convert from 1-based to 0-based indexing
        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(CursorPosition { path, point })
    }
}

enum GetContextOutput {
    Zeta1(zeta::GatherContextOutput),
    Zeta2(String),
}

async fn get_context(
    zeta2_args: Option<Zeta2Args>,
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<GetContextOutput> {
    let ContextArgs {
        worktree: worktree_path,
        cursor,
        use_language_server,
        events,
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
    let (_lsp_open_handle, buffer) = if use_language_server {
        let (lsp_open_handle, _, buffer) = open_buffer_with_language_server(
            &project,
            &worktree,
            &cursor.path,
            &mut ready_languages,
            cx,
        )
        .await?;
        (Some(lsp_open_handle), buffer)
    } else {
        let buffer = open_buffer(&project, &worktree, &cursor.path, cx).await?;
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

    let events = match events {
        Some(events) => events.read_to_string().await?,
        None => String::new(),
    };

    if let Some(zeta2_args) = zeta2_args {
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
                    zeta.set_options(zeta2::ZetaOptions {
                        excerpt: EditPredictionExcerptOptions {
                            max_bytes: zeta2_args.max_excerpt_bytes,
                            min_bytes: zeta2_args.min_excerpt_bytes,
                            target_before_cursor_over_total_bytes: zeta2_args
                                .target_before_cursor_over_total_bytes,
                        },
                        max_diagnostic_bytes: zeta2_args.max_diagnostic_bytes,
                        max_prompt_bytes: zeta2_args.max_prompt_bytes,
                        prompt_format: zeta2_args.prompt_format.into(),
                        file_indexing_parallelism: zeta2_args.file_indexing_parallelism,
                    });
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

                    let planned_prompt = cloud_zeta2_prompt::PlannedPrompt::populate(&request)?;
                    let (prompt_string, section_labels) = planned_prompt.to_prompt_string()?;

                    match zeta2_args.output_format {
                        OutputFormat::Prompt => anyhow::Ok(prompt_string),
                        OutputFormat::Request => {
                            anyhow::Ok(serde_json::to_string_pretty(&request)?)
                        }
                        OutputFormat::Full => anyhow::Ok(serde_json::to_string_pretty(&json!({
                            "request": request,
                            "prompt": prompt_string,
                            "section_labels": section_labels,
                        }))?),
                    }
                })
            })?
            .await?;
        Ok(GetContextOutput::Zeta2(output))
    } else {
        let prompt_for_events = move || (events, 0);
        Ok(GetContextOutput::Zeta1(
            cx.update(|cx| {
                zeta::gather_context(
                    full_path_str,
                    &snapshot,
                    clipped_cursor,
                    prompt_for_events,
                    cx,
                )
            })?
            .await?,
        ))
    }
}

pub async fn retrieval_stats(
    worktree: PathBuf,
    file_indexing_parallelism: usize,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<String> {
    let worktree_path = worktree.canonicalize()?;

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
    let worktree_id = worktree.read_with(cx, |worktree, _cx| worktree.id())?;

    // wait for worktree scan so that wait_for_initial_file_indexing waits for the whole worktree.
    worktree
        .read_with(cx, |worktree, _cx| {
            worktree.as_local().unwrap().scan_complete()
        })?
        .await;

    let index = cx.new(|cx| SyntaxIndex::new(&project, file_indexing_parallelism, cx))?;
    index
        .read_with(cx, |index, cx| index.wait_for_initial_file_indexing(cx))?
        .await?;
    let files = index
        .read_with(cx, |index, cx| index.indexed_file_paths(cx))?
        .await
        .into_iter()
        .filter(|project_path| {
            project_path
                .path
                .extension()
                .is_some_and(|extension| !["md", "json", "sh", "diff"].contains(&extension))
        })
        .collect::<Vec<_>>();

    let lsp_store = project.read_with(cx, |project, _cx| project.lsp_store())?;
    cx.subscribe(&lsp_store, {
        move |_, event, _| {
            if let project::LspStoreEvent::LanguageServerUpdate {
                message:
                    client::proto::update_language_server::Variant::WorkProgress(
                        client::proto::LspWorkProgress {
                            message: Some(message),
                            ..
                        },
                    ),
                ..
            } = event
            {
                println!("⟲ {message}")
            }
        }
    })?
    .detach();

    let mut lsp_open_handles = Vec::new();
    let mut output = std::fs::File::create("retrieval-stats.txt")?;
    let mut results = Vec::new();
    let mut ready_languages = HashSet::default();
    for (file_index, project_path) in files.iter().enumerate() {
        let processing_file_message = format!(
            "Processing file {} of {}: {}",
            file_index + 1,
            files.len(),
            project_path.path.display(PathStyle::Posix)
        );
        println!("{}", processing_file_message);
        write!(output, "{processing_file_message}\n\n").ok();

        let Some((lsp_open_handle, language_server_id, buffer)) = open_buffer_with_language_server(
            &project,
            &worktree,
            &project_path.path,
            &mut ready_languages,
            cx,
        )
        .await
        .log_err() else {
            continue;
        };
        lsp_open_handles.push(lsp_open_handle);

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
        let full_range = 0..snapshot.len();
        let references = references_in_range(
            full_range,
            &snapshot.text(),
            ReferenceRegion::Nearby,
            &snapshot,
        );

        loop {
            let is_ready = lsp_store
                .read_with(cx, |lsp_store, _cx| {
                    lsp_store
                        .language_server_statuses
                        .get(&language_server_id)
                        .is_some_and(|status| status.pending_work.is_empty())
                })
                .unwrap();
            if is_ready {
                break;
            }
            cx.background_executor()
                .timer(Duration::from_millis(10))
                .await;
        }

        let index = index.read_with(cx, |index, _cx| index.state().clone())?;
        let index = index.lock().await;
        for reference in references {
            let query_point = snapshot.offset_to_point(reference.range.start);
            let mut single_reference_map = HashMap::default();
            single_reference_map.insert(reference.identifier.clone(), vec![reference.clone()]);
            let edit_prediction_context = EditPredictionContext::gather_context_with_references_fn(
                query_point,
                &snapshot,
                &zeta2::DEFAULT_EXCERPT_OPTIONS,
                Some(&index),
                |_, _, _| single_reference_map,
            );

            let Some(edit_prediction_context) = edit_prediction_context else {
                let result = RetrievalStatsResult {
                    identifier: reference.identifier,
                    point: query_point,
                    outcome: RetrievalStatsOutcome::NoExcerpt,
                };
                write!(output, "{:?}\n\n", result)?;
                results.push(result);
                continue;
            };

            let mut retrieved_definitions = Vec::new();
            for scored_declaration in edit_prediction_context.declarations {
                match &scored_declaration.declaration {
                    Declaration::File {
                        project_entry_id,
                        declaration,
                    } => {
                        let Some(path) = worktree.read_with(cx, |worktree, _cx| {
                            worktree
                                .entry_for_id(*project_entry_id)
                                .map(|entry| entry.path.clone())
                        })?
                        else {
                            log::error!("bug: file project entry not found");
                            continue;
                        };
                        let project_path = ProjectPath {
                            worktree_id,
                            path: path.clone(),
                        };
                        let buffer = project
                            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                            .await?;
                        let rope = buffer.read_with(cx, |buffer, _cx| buffer.as_rope().clone())?;
                        retrieved_definitions.push((
                            path,
                            rope.offset_to_point(declaration.item_range.start)
                                ..rope.offset_to_point(declaration.item_range.end),
                            scored_declaration.scores.declaration,
                            scored_declaration.scores.retrieval,
                        ));
                    }
                    Declaration::Buffer {
                        project_entry_id,
                        rope,
                        declaration,
                        ..
                    } => {
                        let Some(path) = worktree.read_with(cx, |worktree, _cx| {
                            worktree
                                .entry_for_id(*project_entry_id)
                                .map(|entry| entry.path.clone())
                        })?
                        else {
                            // This case happens when dependency buffers have been opened by
                            // go-to-definition, resulting in single-file worktrees.
                            continue;
                        };
                        retrieved_definitions.push((
                            path,
                            rope.offset_to_point(declaration.item_range.start)
                                ..rope.offset_to_point(declaration.item_range.end),
                            scored_declaration.scores.declaration,
                            scored_declaration.scores.retrieval,
                        ));
                    }
                }
            }
            retrieved_definitions
                .sort_by_key(|(_, _, _, retrieval_score)| Reverse(OrderedFloat(*retrieval_score)));

            // TODO: Consider still checking language server in this case, or having a mode for
            // this. For now assuming that the purpose of this is to refine the ranking rather than
            // refining whether the definition is present at all.
            if retrieved_definitions.is_empty() {
                continue;
            }

            // TODO: Rename declaration to definition in edit_prediction_context?
            let lsp_result = project
                .update(cx, |project, cx| {
                    project.definitions(&buffer, reference.range.start, cx)
                })?
                .await;
            match lsp_result {
                Ok(lsp_definitions) => {
                    let lsp_definitions = lsp_definitions
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|definition| {
                            definition
                                .target
                                .buffer
                                .read_with(cx, |buffer, _cx| {
                                    let path = buffer.file()?.path();
                                    // filter out definitions from single-file worktrees
                                    if path.is_empty() {
                                        None
                                    } else {
                                        Some((
                                            path.clone(),
                                            definition.target.range.to_point(&buffer),
                                        ))
                                    }
                                })
                                .ok()?
                        })
                        .collect::<Vec<_>>();

                    let result = RetrievalStatsResult {
                        identifier: reference.identifier,
                        point: query_point,
                        outcome: RetrievalStatsOutcome::Success {
                            matches: lsp_definitions
                                .iter()
                                .map(|(path, range)| {
                                    retrieved_definitions.iter().position(
                                        |(retrieved_path, retrieved_range, _, _)| {
                                            path == retrieved_path
                                                && retrieved_range.contains_inclusive(&range)
                                        },
                                    )
                                })
                                .collect(),
                            lsp_definitions,
                            retrieved_definitions,
                        },
                    };
                    write!(output, "{:?}\n\n", result)?;
                    results.push(result);
                }
                Err(err) => {
                    let result = RetrievalStatsResult {
                        identifier: reference.identifier,
                        point: query_point,
                        outcome: RetrievalStatsOutcome::LanguageServerError {
                            message: err.to_string(),
                        },
                    };
                    write!(output, "{:?}\n\n", result)?;
                    results.push(result);
                }
            }
        }
    }

    let mut no_excerpt_count = 0;
    let mut error_count = 0;
    let mut definitions_count = 0;
    let mut top_match_count = 0;
    let mut non_top_match_count = 0;
    let mut ranking_involved_count = 0;
    let mut ranking_involved_top_match_count = 0;
    let mut ranking_involved_non_top_match_count = 0;
    for result in &results {
        match &result.outcome {
            RetrievalStatsOutcome::NoExcerpt => no_excerpt_count += 1,
            RetrievalStatsOutcome::LanguageServerError { .. } => error_count += 1,
            RetrievalStatsOutcome::Success {
                matches,
                retrieved_definitions,
                ..
            } => {
                definitions_count += 1;
                let top_matches = matches.contains(&Some(0));
                if top_matches {
                    top_match_count += 1;
                }
                let non_top_matches = !top_matches && matches.iter().any(|index| *index != Some(0));
                if non_top_matches {
                    non_top_match_count += 1;
                }
                if retrieved_definitions.len() > 1 {
                    ranking_involved_count += 1;
                    if top_matches {
                        ranking_involved_top_match_count += 1;
                    }
                    if non_top_matches {
                        ranking_involved_non_top_match_count += 1;
                    }
                }
            }
        }
    }

    println!("\nStats:\n");
    println!("No Excerpt: {}", no_excerpt_count);
    println!("Language Server Error: {}", error_count);
    println!("Definitions: {}", definitions_count);
    println!("Top Match: {}", top_match_count);
    println!("Non-Top Match: {}", non_top_match_count);
    println!("Ranking Involved: {}", ranking_involved_count);
    println!(
        "Ranking Involved Top Match: {}",
        ranking_involved_top_match_count
    );
    println!(
        "Ranking Involved Non-Top Match: {}",
        ranking_involved_non_top_match_count
    );

    Ok("".to_string())
}

#[derive(Debug)]
struct RetrievalStatsResult {
    #[allow(dead_code)]
    identifier: Identifier,
    #[allow(dead_code)]
    point: Point,
    outcome: RetrievalStatsOutcome,
}

#[derive(Debug)]
enum RetrievalStatsOutcome {
    NoExcerpt,
    LanguageServerError {
        #[allow(dead_code)]
        message: String,
    },
    Success {
        matches: Vec<Option<usize>>,
        #[allow(dead_code)]
        lsp_definitions: Vec<(Arc<RelPath>, Range<Point>)>,
        retrieved_definitions: Vec<(Arc<RelPath>, Range<Point>, f32, f32)>,
    },
}

pub async fn open_buffer(
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    path: &RelPath,
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
        worktree_id: worktree.id(),
        path: path.into(),
    })?;

    project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
        .await
}

pub async fn open_buffer_with_language_server(
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    path: &RelPath,
    ready_languages: &mut HashSet<LanguageId>,
    cx: &mut AsyncApp,
) -> Result<(Entity<Entity<Buffer>>, LanguageServerId, Entity<Buffer>)> {
    let buffer = open_buffer(project, worktree, path, cx).await?;

    let (lsp_open_handle, path_style) = project.update(cx, |project, cx| {
        (
            project.register_buffer_with_language_servers(&buffer, cx),
            project.path_style(cx),
        )
    })?;

    let Some(language_id) = buffer.read_with(cx, |buffer, _cx| {
        buffer.language().map(|language| language.id())
    })?
    else {
        return Err(anyhow!("No language for {}", path.display(path_style)));
    };

    let log_prefix = path.display(path_style);
    if !ready_languages.contains(&language_id) {
        wait_for_lang_server(&project, &buffer, log_prefix.into_owned(), cx).await?;
        ready_languages.insert(language_id);
    }

    let lsp_store = project.read_with(cx, |project, _cx| project.lsp_store())?;

    // hacky wait for buffer to be registered with the language server
    for _ in 0..100 {
        let Some(language_server_id) = lsp_store.update(cx, |lsp_store, cx| {
            buffer.update(cx, |buffer, cx| {
                lsp_store
                    .language_servers_for_local_buffer(&buffer, cx)
                    .next()
                    .map(|(_, language_server)| language_server.server_id())
            })
        })?
        else {
            cx.background_executor()
                .timer(Duration::from_millis(10))
                .await;
            continue;
        };

        return Ok((lsp_open_handle, language_server_id, buffer));
    }

    return Err(anyhow!("No language server found for buffer"));
}

// TODO: Dedupe with similar function in crates/eval/src/instance.rs
pub fn wait_for_lang_server(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    log_prefix: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    println!("{}⏵ Waiting for language server", log_prefix);

    let (mut tx, mut rx) = mpsc::channel(1);

    let lsp_store = project
        .read_with(cx, |project, _| project.lsp_store())
        .unwrap();

    let has_lang_server = buffer
        .update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .language_servers_for_local_buffer(buffer, cx)
                    .next()
                    .is_some()
            })
        })
        .unwrap_or(false);

    if has_lang_server {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .unwrap()
            .detach();
    }
    let (mut added_tx, mut added_rx) = mpsc::channel(1);

    let subscriptions = [
        cx.subscribe(&lsp_store, {
            let log_prefix = log_prefix.clone();
            move |_, event, _| {
                if let project::LspStoreEvent::LanguageServerUpdate {
                    message:
                        client::proto::update_language_server::Variant::WorkProgress(
                            client::proto::LspWorkProgress {
                                message: Some(message),
                                ..
                            },
                        ),
                    ..
                } = event
                {
                    println!("{}⟲ {message}", log_prefix)
                }
            }
        }),
        cx.subscribe(project, {
            let buffer = buffer.clone();
            move |project, event, cx| match event {
                project::Event::LanguageServerAdded(_, _, _) => {
                    let buffer = buffer.clone();
                    project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx))
                        .detach();
                    added_tx.try_send(()).ok();
                }
                project::Event::DiskBasedDiagnosticsFinished { .. } => {
                    tx.try_send(()).ok();
                }
                _ => {}
            }
        }),
    ];

    cx.spawn(async move |cx| {
        if !has_lang_server {
            // some buffers never have a language server, so this aborts quickly in that case.
            let timeout = cx.background_executor().timer(Duration::from_secs(5));
            futures::select! {
                _ = added_rx.next() => {},
                _ = timeout.fuse() => {
                    anyhow::bail!("Waiting for language server add timed out after 5 seconds");
                }
            };
        }
        let timeout = cx.background_executor().timer(Duration::from_secs(60 * 5));
        let result = futures::select! {
            _ = rx.next() => {
                println!("{}⚑ Language server idle", log_prefix);
                anyhow::Ok(())
            },
            _ = timeout.fuse() => {
                anyhow::bail!("LSP wait timed out after 5 minutes");
            }
        };
        drop(subscriptions);
        result
    })
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
                Commands::Zeta2Context {
                    zeta2_args,
                    context_args,
                } => match get_context(Some(zeta2_args), context_args, &app_state, cx).await {
                    Ok(GetContextOutput::Zeta1 { .. }) => unreachable!(),
                    Ok(GetContextOutput::Zeta2(output)) => Ok(output),
                    Err(err) => Err(err),
                },
                Commands::Context(context_args) => {
                    match get_context(None, context_args, &app_state, cx).await {
                        Ok(GetContextOutput::Zeta1(output)) => {
                            Ok(serde_json::to_string_pretty(&output.body).unwrap())
                        }
                        Ok(GetContextOutput::Zeta2 { .. }) => unreachable!(),
                        Err(err) => Err(err),
                    }
                }
                Commands::Predict {
                    predict_edits_body,
                    context_args,
                } => {
                    cx.spawn(async move |cx| {
                        let app_version = cx.update(|cx| AppVersion::global(cx))?;
                        app_state.client.sign_in(true, cx).await?;
                        let llm_token = LlmApiToken::default();
                        llm_token.refresh(&app_state.client).await?;

                        let predict_edits_body =
                            if let Some(predict_edits_body) = predict_edits_body {
                                serde_json::from_str(&predict_edits_body.read_to_string().await?)?
                            } else if let Some(context_args) = context_args {
                                match get_context(None, context_args, &app_state, cx).await? {
                                    GetContextOutput::Zeta1(output) => output.body,
                                    GetContextOutput::Zeta2 { .. } => unreachable!(),
                                }
                            } else {
                                return Err(anyhow!(
                                    "Expected either --predict-edits-body-file \
                                    or the required args of the `context` command."
                                ));
                            };

                        let (response, _usage) =
                            Zeta::perform_predict_edits(PerformPredictEditsParams {
                                client: app_state.client.clone(),
                                llm_token,
                                app_version,
                                body: predict_edits_body,
                            })
                            .await?;

                        Ok(response.output_excerpt)
                    })
                    .await
                }
                Commands::RetrievalStats {
                    worktree,
                    file_indexing_parallelism,
                } => retrieval_stats(worktree, file_indexing_parallelism, app_state, cx).await,
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
