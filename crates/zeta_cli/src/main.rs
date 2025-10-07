mod headless;

use anyhow::{Context as _, Result, anyhow};
use clap::{Args, Parser, Subcommand};
use cloud_llm_client::predict_edits_v3::{self, DeclarationScoreComponents};
use edit_prediction_context::{
    Declaration, DeclarationStyle, EditPredictionContext, EditPredictionContextOptions,
    EditPredictionExcerptOptions, EditPredictionScoreOptions, Identifier, Reference,
    ReferenceRegion, SyntaxIndex, SyntaxIndexState, references_in_range,
};
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, Application, AsyncApp};
use gpui::{Entity, Task};
use language::{Bias, BufferSnapshot, LanguageServerId, Point};
use language::{Buffer, OffsetRangeExt};
use language::{LanguageId, ParseStatus};
use language_model::LlmApiToken;
use ordered_float::OrderedFloat;
use project::{Project, ProjectPath, Worktree};
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
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
        #[clap(flatten)]
        zeta2_args: Zeta2Args,
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
    #[arg(long, default_value_t = false)]
    disable_imports_gathering: bool,
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct SourceLocation {
    path: Arc<RelPath>,
    point: Point,
}

impl Serialize for SourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.path.display(PathStyle::Posix),
            self.point.row + 1,
            self.point.column + 1
        )
    }
}

impl FromStr for SourceLocation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid source location. Expected 'file.rs:line:column', got '{}'",
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

        Ok(SourceLocation { path, point })
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
                    zeta.set_options((&zeta2_args).into());
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

impl Into<zeta2::ZetaOptions> for &Zeta2Args {
    fn into(self) -> zeta2::ZetaOptions {
        zeta2::ZetaOptions {
            context: EditPredictionContextOptions {
                use_imports: !self.disable_imports_gathering,
                excerpt: EditPredictionExcerptOptions {
                    max_bytes: self.max_excerpt_bytes,
                    min_bytes: self.min_excerpt_bytes,
                    target_before_cursor_over_total_bytes: self
                        .target_before_cursor_over_total_bytes,
                },
                score: EditPredictionScoreOptions {
                    omit_excerpt_overlaps: true,
                },
            },
            max_diagnostic_bytes: self.max_diagnostic_bytes,
            max_prompt_bytes: self.max_prompt_bytes,
            prompt_format: self.prompt_format.clone().into(),
            file_indexing_parallelism: self.file_indexing_parallelism,
        }
    }
}

pub async fn retrieval_stats(
    worktree: PathBuf,
    app_state: Arc<ZetaCliAppState>,
    only_extension: Option<String>,
    file_limit: Option<usize>,
    skip_files: Option<usize>,
    options: zeta2::ZetaOptions,
    cx: &mut AsyncApp,
) -> Result<String> {
    let options = Arc::new(options);
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

    // wait for worktree scan so that wait_for_initial_file_indexing waits for the whole worktree.
    worktree
        .read_with(cx, |worktree, _cx| {
            worktree.as_local().unwrap().scan_complete()
        })?
        .await;

    let index = cx.new(|cx| SyntaxIndex::new(&project, options.file_indexing_parallelism, cx))?;
    index
        .read_with(cx, |index, cx| index.wait_for_initial_file_indexing(cx))?
        .await?;
    let indexed_files = index
        .read_with(cx, |index, cx| index.indexed_file_paths(cx))?
        .await;
    let mut filtered_files = indexed_files
        .into_iter()
        .filter(|project_path| {
            let file_extension = project_path.path.extension();
            if let Some(only_extension) = only_extension.as_ref() {
                file_extension.is_some_and(|extension| extension == only_extension)
            } else {
                file_extension
                    .is_some_and(|extension| !["md", "json", "sh", "diff"].contains(&extension))
            }
        })
        .collect::<Vec<_>>();
    filtered_files.sort_by(|a, b| a.path.cmp(&b.path));

    let index_state = index.read_with(cx, |index, _cx| index.state().clone())?;
    cx.update(|_| {
        drop(index);
    })?;
    let index_state = Arc::new(
        Arc::into_inner(index_state)
            .context("Index state had more than 1 reference")?
            .into_inner(),
    );

    let mut hasher = collections::FxHasher::default();
    worktree.read_with(cx, |worktree, _cx| {
        for file in &filtered_files {
            let content = std::fs::read(worktree.absolutize(&file.path))?;
            content.hash(&mut hasher);
        }
        anyhow::Ok(())
    })??;
    let files_hash = hasher.finish();
    let lsp_definitions_path = std::env::current_dir()?.join(format!(
        "target/zeta2-lsp-definitions-{:x}.json",
        files_hash
    ));

    let lsp_definitions: Arc<_> = if std::fs::exists(&lsp_definitions_path)? {
        log::info!(
            "Using cached LSP definitions from {}",
            lsp_definitions_path.display()
        );
        serde_json::from_reader(File::open(&lsp_definitions_path)?)?
    } else {
        log::warn!(
            "No LSP definitions found populating {}",
            lsp_definitions_path.display()
        );
        let lsp_definitions =
            gather_lsp_definitions(&filtered_files, &worktree, &project, cx).await?;
        serde_json::to_writer_pretty(File::create(&lsp_definitions_path)?, &lsp_definitions)?;
        lsp_definitions
    }
    .into();

    let filtered_files = filtered_files
        .into_iter()
        .skip(skip_files.unwrap_or(0))
        .take(file_limit.unwrap_or(usize::MAX))
        .collect::<Vec<_>>();
    let filtered_files_len = filtered_files.len();

    let (output_tx, mut output_rx) = mpsc::unbounded::<RetrievalStatsResult>();
    let mut output = std::fs::File::create("target/zeta-retrieval-stats.txt")?;

    let tasks = filtered_files
        .into_iter()
        .enumerate()
        .map(|(file_index, project_path)| {
            let index_state = index_state.clone();
            let lsp_definitions = lsp_definitions.clone();
            let options = options.clone();
            let project = project.clone();
            let worktree = worktree.clone();
            let output_tx = output_tx.clone();
            cx.spawn(async move |cx| {
                let buffer = open_buffer(&project, &worktree, &project_path.path, cx).await?;

                let Some((snapshot, path, parent_abs_path)) =
                    buffer.read_with(cx, |buffer, cx| {
                        project::File::from_dyn(buffer.file()).and_then(|f| {
                            let mut path = f.worktree.read(cx).absolutize(&f.path);
                            if path.pop() {
                                Some((buffer.snapshot(), f.path.clone(), path))
                            } else {
                                None
                            }
                        })
                    })?
                else {
                    anyhow::bail!("Buffer had no path")
                };
                let full_range = 0..snapshot.len();
                let references = references_in_range(
                    full_range,
                    &snapshot.text(),
                    ReferenceRegion::Nearby,
                    &snapshot,
                );

                println!(
                    "{:02}/{:02} references: {}",
                    file_index + 1,
                    filtered_files_len,
                    references.len(),
                );

                for reference in references {
                    let query_point = snapshot.offset_to_point(reference.range.start);
                    let source_location = SourceLocation {
                        path: path.clone(),
                        point: query_point,
                    };
                    let lsp_definitions = lsp_definitions
                        .definitions
                        .get(&source_location)
                        .cloned()
                        .unwrap_or_else(|| {
                            log::warn!(
                                "No definitions found for source location: {:?}",
                                source_location
                            );
                            Vec::new()
                        });

                    let retrieved_definitions = retrieve_definitions(
                        &reference,
                        query_point,
                        &snapshot,
                        Some(parent_abs_path.as_path()),
                        &index_state,
                        &project,
                        &worktree,
                        &options.context,
                        cx,
                    )
                    .await?;

                    // TODO: LSP returns things like locals, this filters out some of those, but potentially
                    // hides some retrieval issues.
                    if retrieved_definitions.is_empty() {
                        continue;
                    }

                    let mut best_match = None;
                    let mut has_external_definition = false;
                    for (index, retrieved_definition) in retrieved_definitions.iter().enumerate() {
                        for lsp_definition in &lsp_definitions {
                            let SourceRange {
                                path,
                                point_range,
                                offset_range: _,
                            } = lsp_definition;
                            let lsp_point_range =
                                SerializablePoint::into_language_point_range(point_range.clone());
                            has_external_definition = has_external_definition
                                || path.is_absolute()
                                || path
                                    .components()
                                    .any(|component| component.as_os_str() == "node_modules");
                            let is_match = path.as_path()
                                == retrieved_definition.path.as_std_path()
                                && retrieved_definition
                                    .range
                                    .contains_inclusive(&lsp_point_range);
                            if is_match {
                                if best_match.is_none() {
                                    best_match = Some(index);
                                }
                            }
                        }
                    }

                    let outcome = if let Some(best_match) = best_match {
                        RetrievalOutcome::Match { best_match }
                    } else if has_external_definition {
                        RetrievalOutcome::NoMatchDueToExternalLspDefinitions
                    } else {
                        RetrievalOutcome::NoMatch
                    };

                    let result = RetrievalStatsResult {
                        outcome,
                        path: path.clone(),
                        identifier: reference.identifier,
                        point: query_point,
                        lsp_definitions,
                        retrieved_definitions,
                    };

                    output_tx.unbounded_send(result).ok();
                }

                Ok(())
            })
        })
        .collect::<Vec<_>>();

    drop(output_tx);

    let results_task = cx.background_spawn(async move {
        let mut results = Vec::new();
        while let Some(result) = output_rx.next().await {
            output
                .write_all(format!("{:#?}\n", result).as_bytes())
                .log_err();
            results.push(result)
        }
        results
    });

    futures::future::try_join_all(tasks).await?;
    let results = results_task.await;

    let mut references_count = 0;

    let mut match_count = 0;
    let mut both_absent_count = 0;
    let mut top_match_count = 0;
    let mut non_top_match_count = 0;
    let mut ranking_involved_top_match_count = 0;
    let mut ranking_involved_non_top_match_count = 0;

    let mut no_match_count = 0;
    let mut no_match_none_retrieved = 0;
    let mut no_match_wrong_retrieval = 0;

    let mut expected_no_match_count = 0;

    for result in results {
        references_count += 1;
        match &result.outcome {
            RetrievalOutcome::Match { best_match } => {
                match_count += 1;
                let multiple = result.retrieved_definitions.len() > 1;
                if *best_match == 0 {
                    top_match_count += 1;
                    if multiple {
                        ranking_involved_top_match_count += 1;
                    }
                } else {
                    non_top_match_count += 1;
                    if multiple {
                        ranking_involved_non_top_match_count += 1;
                    }
                }
            }
            RetrievalOutcome::NoMatch => {
                if result.lsp_definitions.is_empty() {
                    match_count += 1;
                    both_absent_count += 1;
                } else {
                    no_match_count += 1;
                    if result.retrieved_definitions.is_empty() {
                        no_match_none_retrieved += 1;
                    } else {
                        no_match_wrong_retrieval += 1;
                    }
                }
            }
            RetrievalOutcome::NoMatchDueToExternalLspDefinitions => {
                expected_no_match_count += 1;
            }
        }
    }

    fn count_and_percentage(part: usize, total: usize) -> String {
        format!("{} ({:.2}%)", part, (part as f64 / total as f64) * 100.0)
    }

    println!("");
    println!("╮ references: {}", references_count);
    println!(
        "├─╮ match: {}",
        count_and_percentage(match_count, references_count)
    );
    println!(
        "│ ├─╴ both absent: {}",
        count_and_percentage(both_absent_count, match_count)
    );
    println!(
        "│ ├─╮ top match: {}",
        count_and_percentage(top_match_count, match_count)
    );
    println!(
        "│ │ ╰ involving ranking: {}",
        count_and_percentage(ranking_involved_top_match_count, top_match_count)
    );
    println!(
        "│ ╰─╮ non-top match: {}",
        count_and_percentage(non_top_match_count, match_count)
    );
    println!(
        "│   ╰ involving ranking: {}",
        count_and_percentage(ranking_involved_non_top_match_count, non_top_match_count)
    );
    println!(
        "├─╮ no match: {}",
        count_and_percentage(no_match_count, references_count)
    );
    println!(
        "│ ├─ none retrieved: {}",
        count_and_percentage(no_match_none_retrieved, no_match_count)
    );
    println!(
        "│ ╰─ wrong retrieval: {}",
        count_and_percentage(no_match_wrong_retrieval, no_match_count)
    );
    println!(
        "╰─╴ expected no match: {}",
        count_and_percentage(expected_no_match_count, references_count)
    );

    println!("");
    println!("LSP definition cache at {}", lsp_definitions_path.display());

    Ok("".to_string())
}

async fn retrieve_definitions(
    reference: &Reference,
    query_point: Point,
    snapshot: &BufferSnapshot,
    parent_abs_path: Option<&Path>,
    index: &SyntaxIndexState,
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    options: &EditPredictionContextOptions,
    cx: &mut AsyncApp,
) -> Result<Vec<RetrievedDefinition>> {
    let mut single_reference_map = HashMap::default();
    single_reference_map.insert(reference.identifier.clone(), vec![reference.clone()]);
    let edit_prediction_context = EditPredictionContext::gather_context_with_references_fn(
        query_point,
        &snapshot,
        parent_abs_path.as_deref(),
        &options,
        Some(&index),
        |_, _, _| single_reference_map,
    );

    let Some(edit_prediction_context) = edit_prediction_context else {
        return Ok(Vec::new());
    };

    let mut retrieved_definitions = Vec::new();
    for scored_declaration in edit_prediction_context.declarations {
        match &scored_declaration.declaration {
            Declaration::File {
                project_entry_id,
                declaration,
                ..
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
                    worktree_id: worktree.read_with(cx, |worktree, _cx| worktree.id())?,
                    path: path.clone(),
                };
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                    .await?;
                let rope = buffer.read_with(cx, |buffer, _cx| buffer.as_rope().clone())?;
                retrieved_definitions.push(RetrievedDefinition {
                    path,
                    range: rope.offset_to_point(declaration.item_range.start)
                        ..rope.offset_to_point(declaration.item_range.end),
                    score: scored_declaration.score(DeclarationStyle::Declaration),
                    retrieval_score: scored_declaration.retrieval_score(),
                    components: scored_declaration.components,
                });
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
                retrieved_definitions.push(RetrievedDefinition {
                    path,
                    range: rope.offset_to_point(declaration.item_range.start)
                        ..rope.offset_to_point(declaration.item_range.end),
                    score: scored_declaration.score(DeclarationStyle::Declaration),
                    retrieval_score: scored_declaration.retrieval_score(),
                    components: scored_declaration.components,
                });
            }
        }
    }
    retrieved_definitions.sort_by_key(|definition| Reverse(OrderedFloat(definition.score)));

    Ok(retrieved_definitions)
}

async fn gather_lsp_definitions(
    files: &[ProjectPath],
    worktree: &Entity<Worktree>,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<LspResults> {
    let worktree_id = worktree.read_with(cx, |worktree, _cx| worktree.id())?;

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

    let mut definitions = HashMap::default();
    let mut error_count = 0;
    let mut lsp_open_handles = Vec::new();
    let mut ready_languages = HashSet::default();
    for (file_index, project_path) in files.iter().enumerate() {
        println!(
            "Processing file {} of {}: {}",
            file_index + 1,
            files.len(),
            project_path.path.display(PathStyle::Posix)
        );

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

        for reference in references {
            // TODO: Rename declaration to definition in edit_prediction_context?
            let lsp_result = project
                .update(cx, |project, cx| {
                    project.definitions(&buffer, reference.range.start, cx)
                })?
                .await;

            match lsp_result {
                Ok(lsp_definitions) => {
                    let mut targets = Vec::new();
                    for target in lsp_definitions.unwrap_or_default() {
                        let buffer = target.target.buffer;
                        let anchor_range = target.target.range;
                        buffer.read_with(cx, |buffer, cx| {
                            let Some(file) = project::File::from_dyn(buffer.file()) else {
                                return;
                            };
                            let file_worktree = file.worktree.read(cx);
                            let file_worktree_id = file_worktree.id();
                            // Relative paths for worktree files, absolute for all others
                            let path = if worktree_id != file_worktree_id {
                                file.worktree.read(cx).absolutize(&file.path)
                            } else {
                                file.path.as_std_path().to_path_buf()
                            };
                            let offset_range = anchor_range.to_offset(&buffer);
                            let point_range = SerializablePoint::from_language_point_range(
                                offset_range.to_point(&buffer),
                            );
                            targets.push(SourceRange {
                                path,
                                offset_range,
                                point_range,
                            });
                        })?;
                    }

                    let offset = reference.range.start;
                    let point = snapshot.offset_to_point(offset).into();

                    definitions.insert(
                        SourceLocation {
                            path: project_path.path.clone(),
                            point,
                        },
                        targets,
                    );
                }
                Err(err) => {
                    log::error!("Language server error: {err}");
                    error_count += 1;
                }
            }
        }
    }

    log::error!("Encountered {} language server errors", error_count);

    Ok(LspResults { definitions })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct LspResults {
    definitions: HashMap<SourceLocation, Vec<SourceRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceRange {
    path: PathBuf,
    point_range: Range<SerializablePoint>,
    offset_range: Range<usize>,
}

/// Serializes to 1-based row and column indices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePoint {
    pub row: u32,
    pub column: u32,
}

impl SerializablePoint {
    pub fn into_language_point_range(range: Range<Self>) -> Range<Point> {
        range.start.into()..range.end.into()
    }

    pub fn from_language_point_range(range: Range<Point>) -> Range<Self> {
        range.start.into()..range.end.into()
    }
}

impl From<Point> for SerializablePoint {
    fn from(point: Point) -> Self {
        SerializablePoint {
            row: point.row + 1,
            column: point.column + 1,
        }
    }
}

impl From<SerializablePoint> for Point {
    fn from(serializable: SerializablePoint) -> Self {
        Point {
            row: serializable.row.saturating_sub(1),
            column: serializable.column.saturating_sub(1),
        }
    }
}

#[derive(Debug)]
struct RetrievalStatsResult {
    outcome: RetrievalOutcome,
    #[allow(dead_code)]
    path: Arc<RelPath>,
    #[allow(dead_code)]
    identifier: Identifier,
    #[allow(dead_code)]
    point: Point,
    #[allow(dead_code)]
    lsp_definitions: Vec<SourceRange>,
    retrieved_definitions: Vec<RetrievedDefinition>,
}

#[derive(Debug)]
enum RetrievalOutcome {
    Match {
        /// Lowest index within retrieved_definitions that matches an LSP definition.
        best_match: usize,
    },
    NoMatch,
    NoMatchDueToExternalLspDefinitions,
}

#[derive(Debug)]
struct RetrievedDefinition {
    path: Arc<RelPath>,
    range: Range<Point>,
    score: f32,
    #[allow(dead_code)]
    retrieval_score: f32,
    #[allow(dead_code)]
    components: DeclarationScoreComponents,
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

    let buffer = project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
        .await?;

    let mut parse_status = buffer.read_with(cx, |buffer, _cx| buffer.parse_status())?;
    while *parse_status.borrow() != ParseStatus::Idle {
        parse_status.changed().await?;
    }

    Ok(buffer)
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
                    zeta2_args,
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
                        (&zeta2_args).into(),
                        cx,
                    )
                    .await
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
