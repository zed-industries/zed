use ::util::rel_path::RelPath;
use ::util::{RangeExt, ResultExt as _};
use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::DeclarationScoreComponents;
use edit_prediction_context::{
    Declaration, DeclarationStyle, EditPredictionContext, Identifier, Imports, Reference,
    ReferenceRegion, SyntaxIndex, SyntaxIndexState, references_in_range,
};
use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::Entity;
use gpui::{AppContext, AsyncApp};
use language::OffsetRangeExt;
use language::{BufferSnapshot, Point};
use ordered_float::OrderedFloat;
use project::{Project, ProjectEntryId, ProjectPath, Worktree};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader, BufWriter, Write as _},
    ops::Range,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    time::Duration,
};
use util::paths::PathStyle;

use crate::headless::ZetaCliAppState;
use crate::source_location::SourceLocation;
use crate::util::{open_buffer, open_buffer_with_language_server};

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

    struct FileSnapshot {
        project_entry_id: ProjectEntryId,
        snapshot: BufferSnapshot,
        hash: u64,
        parent_abs_path: Arc<Path>,
    }

    let files: Vec<FileSnapshot> = futures::future::try_join_all({
        filtered_files
            .iter()
            .map(|file| {
                let buffer_task =
                    open_buffer(project.clone(), worktree.clone(), file.path.clone(), cx);
                cx.spawn(async move |cx| {
                    let buffer = buffer_task.await?;
                    let (project_entry_id, parent_abs_path, snapshot) =
                        buffer.read_with(cx, |buffer, cx| {
                            let file = project::File::from_dyn(buffer.file()).unwrap();
                            let project_entry_id = file.project_entry_id().unwrap();
                            let mut parent_abs_path = file.worktree.read(cx).absolutize(&file.path);
                            if !parent_abs_path.pop() {
                                panic!("Invalid worktree path");
                            }

                            (project_entry_id, parent_abs_path, buffer.snapshot())
                        })?;

                    anyhow::Ok(
                        cx.background_spawn(async move {
                            let mut hasher = collections::FxHasher::default();
                            snapshot.text().hash(&mut hasher);
                            FileSnapshot {
                                project_entry_id,
                                snapshot,
                                hash: hasher.finish(),
                                parent_abs_path: parent_abs_path.into(),
                            }
                        })
                        .await,
                    )
                })
            })
            .collect::<Vec<_>>()
    })
    .await?;

    let mut file_snapshots = HashMap::default();
    let mut hasher = collections::FxHasher::default();
    for FileSnapshot {
        project_entry_id,
        snapshot,
        hash,
        ..
    } in &files
    {
        file_snapshots.insert(*project_entry_id, snapshot.clone());
        hash.hash(&mut hasher);
    }
    let files_hash = hasher.finish();
    let file_snapshots = Arc::new(file_snapshots);

    let lsp_definitions_path = std::env::current_dir()?.join(format!(
        "target/zeta2-lsp-definitions-{:x}.jsonl",
        files_hash
    ));

    let mut lsp_definitions = HashMap::default();
    let mut lsp_files = 0;

    if std::fs::exists(&lsp_definitions_path)? {
        log::info!(
            "Using cached LSP definitions from {}",
            lsp_definitions_path.display()
        );

        let file = File::options()
            .read(true)
            .write(true)
            .open(&lsp_definitions_path)?;
        let lines = BufReader::new(&file).lines();
        let mut valid_len: usize = 0;

        for (line, expected_file) in lines.zip(files.iter()) {
            let line = line?;
            let FileLspDefinitions { path, references } = match serde_json::from_str(&line) {
                Ok(ok) => ok,
                Err(_) => {
                    log::error!("Found invalid cache line. Truncating to #{lsp_files}.",);
                    file.set_len(valid_len as u64)?;
                    break;
                }
            };
            let expected_path = expected_file.snapshot.file().unwrap().path().as_unix_str();
            if expected_path != path.as_ref() {
                log::error!(
                    "Expected file #{} to be {expected_path}, but found {path}. Truncating to #{lsp_files}.",
                    lsp_files + 1
                );
                file.set_len(valid_len as u64)?;
                break;
            }
            for (point, ranges) in references {
                let Ok(path) = RelPath::new(Path::new(path.as_ref()), PathStyle::Posix) else {
                    log::warn!("Invalid path: {}", path);
                    continue;
                };
                lsp_definitions.insert(
                    SourceLocation {
                        path: path.into_arc(),
                        point: point.into(),
                    },
                    ranges,
                );
            }
            lsp_files += 1;
            valid_len += line.len() + 1
        }
    }

    if lsp_files < files.len() {
        if lsp_files == 0 {
            log::warn!(
                "No LSP definitions found, populating {}",
                lsp_definitions_path.display()
            );
        } else {
            log::warn!("{} files missing from LSP cache", files.len() - lsp_files);
        }

        gather_lsp_definitions(
            &lsp_definitions_path,
            lsp_files,
            &filtered_files,
            &worktree,
            &project,
            &mut lsp_definitions,
            cx,
        )
        .await?;
    }
    let files_len = files.len().min(file_limit.unwrap_or(usize::MAX));
    let done_count = Arc::new(AtomicUsize::new(0));

    let (output_tx, mut output_rx) = mpsc::unbounded::<RetrievalStatsResult>();
    let mut output = std::fs::File::create("target/zeta-retrieval-stats.txt")?;

    let tasks = files
        .into_iter()
        .skip(skip_files.unwrap_or(0))
        .take(file_limit.unwrap_or(usize::MAX))
        .map(|project_file| {
            let index_state = index_state.clone();
            let lsp_definitions = lsp_definitions.clone();
            let options = options.clone();
            let output_tx = output_tx.clone();
            let done_count = done_count.clone();
            let file_snapshots = file_snapshots.clone();
            cx.background_spawn(async move {
                let snapshot = project_file.snapshot;

                let full_range = 0..snapshot.len();
                let references = references_in_range(
                    full_range,
                    &snapshot.text(),
                    ReferenceRegion::Nearby,
                    &snapshot,
                );

                println!("references: {}", references.len(),);

                let imports = if options.context.use_imports {
                    Imports::gather(&snapshot, Some(&project_file.parent_abs_path))
                } else {
                    Imports::default()
                };

                let path = snapshot.file().unwrap().path();

                for reference in references {
                    let query_point = snapshot.offset_to_point(reference.range.start);
                    let source_location = SourceLocation {
                        path: path.clone(),
                        point: query_point,
                    };
                    let lsp_definitions = lsp_definitions
                        .get(&source_location)
                        .cloned()
                        .unwrap_or_else(|| {
                            log::warn!(
                                "No definitions found for source location: {:?}",
                                source_location
                            );
                            Vec::new()
                        });

                    let retrieve_result = retrieve_definitions(
                        &reference,
                        &imports,
                        query_point,
                        &snapshot,
                        &index_state,
                        &file_snapshots,
                        &options,
                    )
                    .await?;

                    // TODO: LSP returns things like locals, this filters out some of those, but potentially
                    // hides some retrieval issues.
                    if retrieve_result.definitions.is_empty() {
                        continue;
                    }

                    let mut best_match = None;
                    let mut has_external_definition = false;
                    let mut in_excerpt = false;
                    for (index, retrieved_definition) in
                        retrieve_result.definitions.iter().enumerate()
                    {
                        for lsp_definition in &lsp_definitions {
                            let SourceRange {
                                path,
                                point_range,
                                offset_range,
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
                            in_excerpt = in_excerpt
                                || retrieve_result.excerpt_range.as_ref().is_some_and(
                                    |excerpt_range| excerpt_range.contains_inclusive(&offset_range),
                                );
                        }
                    }

                    let outcome = if let Some(best_match) = best_match {
                        RetrievalOutcome::Match { best_match }
                    } else if has_external_definition {
                        RetrievalOutcome::NoMatchDueToExternalLspDefinitions
                    } else if in_excerpt {
                        RetrievalOutcome::ProbablyLocal
                    } else {
                        RetrievalOutcome::NoMatch
                    };

                    let result = RetrievalStatsResult {
                        outcome,
                        path: path.clone(),
                        identifier: reference.identifier,
                        point: query_point,
                        lsp_definitions,
                        retrieved_definitions: retrieve_result.definitions,
                    };

                    output_tx.unbounded_send(result).ok();
                }

                println!(
                    "{:02}/{:02} done",
                    done_count.fetch_add(1, atomic::Ordering::Relaxed) + 1,
                    files_len,
                );

                anyhow::Ok(())
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
    println!("Tasks completed");
    let results = results_task.await;
    println!("Results received");

    let mut references_count = 0;

    let mut included_count = 0;
    let mut both_absent_count = 0;

    let mut retrieved_count = 0;
    let mut top_match_count = 0;
    let mut non_top_match_count = 0;
    let mut ranking_involved_top_match_count = 0;

    let mut no_match_count = 0;
    let mut no_match_none_retrieved = 0;
    let mut no_match_wrong_retrieval = 0;

    let mut expected_no_match_count = 0;
    let mut in_excerpt_count = 0;
    let mut external_definition_count = 0;

    for result in results {
        references_count += 1;
        match &result.outcome {
            RetrievalOutcome::Match { best_match } => {
                included_count += 1;
                retrieved_count += 1;
                let multiple = result.retrieved_definitions.len() > 1;
                if *best_match == 0 {
                    top_match_count += 1;
                    if multiple {
                        ranking_involved_top_match_count += 1;
                    }
                } else {
                    non_top_match_count += 1;
                }
            }
            RetrievalOutcome::NoMatch => {
                if result.lsp_definitions.is_empty() {
                    included_count += 1;
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
                external_definition_count += 1;
            }
            RetrievalOutcome::ProbablyLocal => {
                included_count += 1;
                in_excerpt_count += 1;
            }
        }
    }

    fn count_and_percentage(part: usize, total: usize) -> String {
        format!("{} ({:.2}%)", part, (part as f64 / total as f64) * 100.0)
    }

    println!("");
    println!("╮ references: {}", references_count);
    println!(
        "├─╮ included: {}",
        count_and_percentage(included_count, references_count),
    );
    println!(
        "│ ├─╮ retrieved: {}",
        count_and_percentage(retrieved_count, references_count)
    );
    println!(
        "│ │ ├─╮ top match : {}",
        count_and_percentage(top_match_count, retrieved_count)
    );
    println!(
        "│ │ │ ╰─╴ involving ranking: {}",
        count_and_percentage(ranking_involved_top_match_count, top_match_count)
    );
    println!(
        "│ │ ╰─╴ non-top match: {}",
        count_and_percentage(non_top_match_count, retrieved_count)
    );
    println!(
        "│ ├─╴ both absent: {}",
        count_and_percentage(both_absent_count, included_count)
    );
    println!(
        "│ ╰─╴ in excerpt: {}",
        count_and_percentage(in_excerpt_count, included_count)
    );
    println!(
        "├─╮ no match: {}",
        count_and_percentage(no_match_count, references_count)
    );
    println!(
        "│ ├─╴ none retrieved: {}",
        count_and_percentage(no_match_none_retrieved, no_match_count)
    );
    println!(
        "│ ╰─╴ wrong retrieval: {}",
        count_and_percentage(no_match_wrong_retrieval, no_match_count)
    );
    println!(
        "╰─╮ expected no match: {}",
        count_and_percentage(expected_no_match_count, references_count)
    );
    println!(
        "  ╰─╴ external definition: {}",
        count_and_percentage(external_definition_count, expected_no_match_count)
    );

    println!("");
    println!("LSP definition cache at {}", lsp_definitions_path.display());

    Ok("".to_string())
}

struct RetrieveResult {
    definitions: Vec<RetrievedDefinition>,
    excerpt_range: Option<Range<usize>>,
}

async fn retrieve_definitions(
    reference: &Reference,
    imports: &Imports,
    query_point: Point,
    snapshot: &BufferSnapshot,
    index: &Arc<SyntaxIndexState>,
    file_snapshots: &Arc<HashMap<ProjectEntryId, BufferSnapshot>>,
    options: &Arc<zeta2::ZetaOptions>,
) -> Result<RetrieveResult> {
    let mut single_reference_map = HashMap::default();
    single_reference_map.insert(reference.identifier.clone(), vec![reference.clone()]);
    let edit_prediction_context = EditPredictionContext::gather_context_with_references_fn(
        query_point,
        snapshot,
        imports,
        &options.context,
        Some(&index),
        |_, _, _| single_reference_map,
    );

    let Some(edit_prediction_context) = edit_prediction_context else {
        return Ok(RetrieveResult {
            definitions: Vec::new(),
            excerpt_range: None,
        });
    };

    let mut retrieved_definitions = Vec::new();
    for scored_declaration in edit_prediction_context.declarations {
        match &scored_declaration.declaration {
            Declaration::File {
                project_entry_id,
                declaration,
                ..
            } => {
                let Some(snapshot) = file_snapshots.get(&project_entry_id) else {
                    log::error!("bug: file project entry not found");
                    continue;
                };
                let path = snapshot.file().unwrap().path().clone();
                retrieved_definitions.push(RetrievedDefinition {
                    path,
                    range: snapshot.offset_to_point(declaration.item_range.start)
                        ..snapshot.offset_to_point(declaration.item_range.end),
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
                let Some(snapshot) = file_snapshots.get(&project_entry_id) else {
                    // This case happens when dependency buffers have been opened by
                    // go-to-definition, resulting in single-file worktrees.
                    continue;
                };
                let path = snapshot.file().unwrap().path().clone();
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

    Ok(RetrieveResult {
        definitions: retrieved_definitions,
        excerpt_range: Some(edit_prediction_context.excerpt.range),
    })
}

async fn gather_lsp_definitions(
    lsp_definitions_path: &Path,
    start_index: usize,
    files: &[ProjectPath],
    worktree: &Entity<Worktree>,
    project: &Entity<Project>,
    definitions: &mut HashMap<SourceLocation, Vec<SourceRange>>,
    cx: &mut AsyncApp,
) -> Result<()> {
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

    let (cache_line_tx, mut cache_line_rx) = mpsc::unbounded::<FileLspDefinitions>();

    let cache_file = File::options()
        .append(true)
        .create(true)
        .open(lsp_definitions_path)
        .unwrap();

    let cache_task = cx.background_spawn(async move {
        let mut writer = BufWriter::new(cache_file);
        while let Some(line) = cache_line_rx.next().await {
            serde_json::to_writer(&mut writer, &line).unwrap();
            writer.write_all(&[b'\n']).unwrap();
        }
        writer.flush().unwrap();
    });

    let mut error_count = 0;
    let mut lsp_open_handles = Vec::new();
    let mut ready_languages = HashSet::default();
    for (file_index, project_path) in files[start_index..].iter().enumerate() {
        println!(
            "Processing file {} of {}: {}",
            start_index + file_index + 1,
            files.len(),
            project_path.path.display(PathStyle::Posix)
        );

        let Some((lsp_open_handle, language_server_id, buffer)) = open_buffer_with_language_server(
            project.clone(),
            worktree.clone(),
            project_path.path.clone(),
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

        let mut cache_line_references = Vec::with_capacity(references.len());

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

                    let point = snapshot.offset_to_point(reference.range.start);

                    cache_line_references.push((point.into(), targets.clone()));
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

        cache_line_tx
            .unbounded_send(FileLspDefinitions {
                path: project_path.path.as_unix_str().into(),
                references: cache_line_references,
            })
            .log_err();
    }

    drop(cache_line_tx);

    if error_count > 0 {
        log::error!("Encountered {} language server errors", error_count);
    }

    cache_task.await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct FileLspDefinitions {
    path: Arc<str>,
    references: Vec<(SerializablePoint, Vec<SourceRange>)>,
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
    ProbablyLocal,
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
