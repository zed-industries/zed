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
use polars::prelude::*;
use project::{Project, ProjectEntryId, ProjectPath, Worktree};
use serde::{Deserialize, Serialize};
use std::fs;
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
    let target_cli_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/zeta_cli");
    fs::create_dir_all(&target_cli_dir).unwrap();
    let target_cli_dir = target_cli_dir.canonicalize().unwrap();

    let lsp_cache_dir = target_cli_dir.join("cache");
    fs::create_dir_all(&lsp_cache_dir).unwrap();

    let lsp_definitions_path = lsp_cache_dir.join(format!(
        "{}-{:x}.jsonl",
        worktree_path.file_stem().unwrap_or_default().display(),
        files_hash
    ));

    let mut lsp_definitions = HashMap::default();
    let mut lsp_files = 0;

    if fs::exists(&lsp_definitions_path)? {
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

    let (output_tx, output_rx) = mpsc::unbounded::<ReferenceRetrievalResult>();

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

                    let result = ReferenceRetrievalResult {
                        cursor_path: path.clone(),
                        identifier: reference.identifier,
                        cursor_point: query_point,
                        lsp_definitions,
                        retrieved_definitions: retrieve_result.definitions,
                        excerpt_range: retrieve_result.excerpt_range,
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

    let df_task = cx.background_spawn(build_dataframe(output_rx));

    futures::future::try_join_all(tasks).await?;
    let mut df = df_task.await?;

    let run_id = format!(
        "{}-{}",
        worktree_path.file_stem().unwrap_or_default().display(),
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let run_dir = target_cli_dir.join(run_id);
    fs::create_dir(&run_dir).unwrap();

    let parquet_path = run_dir.join("stats.parquet");
    let mut parquet_file = fs::File::create(&parquet_path)?;

    ParquetWriter::new(&mut parquet_file)
        .finish(&mut df)
        .unwrap();

    let stats = SummaryStats::from_dataframe(df)?;

    let stats_path = run_dir.join("stats.txt");
    fs::write(&stats_path, format!("{}", stats))?;

    println!("{}", stats);
    println!("\nWrote:");
    println!("- {}", relativize_path(&parquet_path).display());
    println!("- {}", relativize_path(&stats_path).display());
    println!("- {}", relativize_path(&lsp_definitions_path).display());

    Ok("".to_string())
}

async fn build_dataframe(
    mut output_rx: mpsc::UnboundedReceiver<ReferenceRetrievalResult>,
) -> Result<DataFrame> {
    use soa_rs::{Soa, Soars};

    #[derive(Default, Soars)]
    struct Row {
        ref_id: u32,
        cursor_path: String,
        cursor_row: u32,
        cursor_column: u32,
        cursor_identifier: String,
        gold_in_excerpt: bool,
        gold_path: String,
        gold_row: u32,
        gold_column: u32,
        gold_is_external: bool,
        candidate_count: u32,
        candidate_path: Option<String>,
        candidate_row: Option<u32>,
        candidate_column: Option<u32>,
        candidate_is_gold: Option<bool>,
        candidate_rank: Option<u32>,
        candidate_is_same_file: Option<bool>,
        candidate_is_referenced_nearby: Option<bool>,
        candidate_is_referenced_in_breadcrumb: Option<bool>,
        candidate_reference_count: Option<u32>,
        candidate_same_file_declaration_count: Option<u32>,
        candidate_declaration_count: Option<u32>,
        candidate_reference_line_distance: Option<u32>,
        candidate_declaration_line_distance: Option<u32>,
        candidate_excerpt_vs_item_jaccard: Option<f32>,
        candidate_excerpt_vs_signature_jaccard: Option<f32>,
        candidate_adjacent_vs_item_jaccard: Option<f32>,
        candidate_adjacent_vs_signature_jaccard: Option<f32>,
        candidate_excerpt_vs_item_weighted_overlap: Option<f32>,
        candidate_excerpt_vs_signature_weighted_overlap: Option<f32>,
        candidate_adjacent_vs_item_weighted_overlap: Option<f32>,
        candidate_adjacent_vs_signature_weighted_overlap: Option<f32>,
        candidate_path_import_match_count: Option<u32>,
        candidate_wildcard_path_import_match_count: Option<u32>,
        candidate_import_similarity: Option<f32>,
        candidate_max_import_similarity: Option<f32>,
        candidate_normalized_import_similarity: Option<f32>,
        candidate_wildcard_import_similarity: Option<f32>,
        candidate_normalized_wildcard_import_similarity: Option<f32>,
        candidate_included_by_others: Option<u32>,
        candidate_includes_others: Option<u32>,
    }
    let mut rows = Soa::<Row>::new();
    let mut next_ref_id = 0;

    while let Some(result) = output_rx.next().await {
        let mut gold_is_external = false;
        let mut gold_in_excerpt = false;
        let cursor_path = result.cursor_path.as_unix_str();
        let cursor_row = result.cursor_point.row + 1;
        let cursor_column = result.cursor_point.column + 1;
        let cursor_identifier = result.identifier.name.to_string();
        let ref_id = next_ref_id;
        next_ref_id += 1;

        for lsp_definition in result.lsp_definitions {
            let SourceRange {
                path: gold_path,
                point_range: gold_point_range,
                offset_range: gold_offset_range,
            } = lsp_definition;
            let lsp_point_range =
                SerializablePoint::into_language_point_range(gold_point_range.clone());

            gold_is_external = gold_is_external
                || gold_path.is_absolute()
                || gold_path
                    .components()
                    .any(|component| component.as_os_str() == "node_modules");

            gold_in_excerpt = gold_in_excerpt
                || result.excerpt_range.as_ref().is_some_and(|excerpt_range| {
                    excerpt_range.contains_inclusive(&gold_offset_range)
                });

            let gold_row = gold_point_range.start.row;
            let gold_column = gold_point_range.start.column;
            let candidate_count = result.retrieved_definitions.len() as u32;

            for (candidate_rank, retrieved_definition) in
                result.retrieved_definitions.iter().enumerate()
            {
                let candidate_is_gold = gold_path.as_path()
                    == retrieved_definition.path.as_std_path()
                    && retrieved_definition
                        .range
                        .contains_inclusive(&lsp_point_range);

                let candidate_row = retrieved_definition.range.start.row + 1;
                let candidate_column = retrieved_definition.range.start.column + 1;

                let DeclarationScoreComponents {
                    is_same_file,
                    is_referenced_nearby,
                    is_referenced_in_breadcrumb,
                    reference_count,
                    same_file_declaration_count,
                    declaration_count,
                    reference_line_distance,
                    declaration_line_distance,
                    excerpt_vs_item_jaccard,
                    excerpt_vs_signature_jaccard,
                    adjacent_vs_item_jaccard,
                    adjacent_vs_signature_jaccard,
                    excerpt_vs_item_weighted_overlap,
                    excerpt_vs_signature_weighted_overlap,
                    adjacent_vs_item_weighted_overlap,
                    adjacent_vs_signature_weighted_overlap,
                    path_import_match_count,
                    wildcard_path_import_match_count,
                    import_similarity,
                    max_import_similarity,
                    normalized_import_similarity,
                    wildcard_import_similarity,
                    normalized_wildcard_import_similarity,
                    included_by_others,
                    includes_others,
                } = retrieved_definition.components;

                rows.push(Row {
                    ref_id,
                    cursor_path: cursor_path.to_string(),
                    cursor_row,
                    cursor_column,
                    cursor_identifier: cursor_identifier.clone(),
                    gold_in_excerpt,
                    gold_path: gold_path.to_string_lossy().to_string(),
                    gold_row,
                    gold_column,
                    gold_is_external,
                    candidate_count,
                    candidate_path: Some(retrieved_definition.path.as_unix_str().to_string()),
                    candidate_row: Some(candidate_row),
                    candidate_column: Some(candidate_column),
                    candidate_is_gold: Some(candidate_is_gold),
                    candidate_rank: Some(candidate_rank as u32),
                    candidate_is_same_file: Some(is_same_file),
                    candidate_is_referenced_nearby: Some(is_referenced_nearby),
                    candidate_is_referenced_in_breadcrumb: Some(is_referenced_in_breadcrumb),
                    candidate_reference_count: Some(reference_count as u32),
                    candidate_same_file_declaration_count: Some(same_file_declaration_count as u32),
                    candidate_declaration_count: Some(declaration_count as u32),
                    candidate_reference_line_distance: Some(reference_line_distance),
                    candidate_declaration_line_distance: Some(declaration_line_distance),
                    candidate_excerpt_vs_item_jaccard: Some(excerpt_vs_item_jaccard),
                    candidate_excerpt_vs_signature_jaccard: Some(excerpt_vs_signature_jaccard),
                    candidate_adjacent_vs_item_jaccard: Some(adjacent_vs_item_jaccard),
                    candidate_adjacent_vs_signature_jaccard: Some(adjacent_vs_signature_jaccard),
                    candidate_excerpt_vs_item_weighted_overlap: Some(
                        excerpt_vs_item_weighted_overlap,
                    ),
                    candidate_excerpt_vs_signature_weighted_overlap: Some(
                        excerpt_vs_signature_weighted_overlap,
                    ),
                    candidate_adjacent_vs_item_weighted_overlap: Some(
                        adjacent_vs_item_weighted_overlap,
                    ),
                    candidate_adjacent_vs_signature_weighted_overlap: Some(
                        adjacent_vs_signature_weighted_overlap,
                    ),
                    candidate_path_import_match_count: Some(path_import_match_count as u32),
                    candidate_wildcard_path_import_match_count: Some(
                        wildcard_path_import_match_count as u32,
                    ),
                    candidate_import_similarity: Some(import_similarity),
                    candidate_max_import_similarity: Some(max_import_similarity),
                    candidate_normalized_import_similarity: Some(normalized_import_similarity),
                    candidate_wildcard_import_similarity: Some(wildcard_import_similarity),
                    candidate_normalized_wildcard_import_similarity: Some(
                        normalized_wildcard_import_similarity,
                    ),
                    candidate_included_by_others: Some(included_by_others as u32),
                    candidate_includes_others: Some(includes_others as u32),
                });
            }

            if result.retrieved_definitions.is_empty() {
                rows.push(Row {
                    ref_id,
                    cursor_path: cursor_path.to_string(),
                    cursor_row,
                    cursor_column,
                    cursor_identifier: cursor_identifier.clone(),
                    gold_in_excerpt,
                    gold_path: gold_path.to_string_lossy().to_string(),
                    gold_row,
                    gold_column,
                    gold_is_external,
                    candidate_count,
                    ..Default::default()
                });
            }
        }
    }
    let slices = rows.slices();

    let RowSlices {
        ref_id,
        cursor_path,
        cursor_row,
        cursor_column,
        cursor_identifier,
        gold_in_excerpt,
        gold_path,
        gold_row,
        gold_column,
        gold_is_external,
        candidate_path,
        candidate_row,
        candidate_column,
        candidate_is_gold,
        candidate_rank,
        candidate_count,
        candidate_is_same_file,
        candidate_is_referenced_nearby,
        candidate_is_referenced_in_breadcrumb,
        candidate_reference_count,
        candidate_same_file_declaration_count,
        candidate_declaration_count,
        candidate_reference_line_distance,
        candidate_declaration_line_distance,
        candidate_excerpt_vs_item_jaccard,
        candidate_excerpt_vs_signature_jaccard,
        candidate_adjacent_vs_item_jaccard,
        candidate_adjacent_vs_signature_jaccard,
        candidate_excerpt_vs_item_weighted_overlap,
        candidate_excerpt_vs_signature_weighted_overlap,
        candidate_adjacent_vs_item_weighted_overlap,
        candidate_adjacent_vs_signature_weighted_overlap,
        candidate_path_import_match_count,
        candidate_wildcard_path_import_match_count,
        candidate_import_similarity,
        candidate_max_import_similarity,
        candidate_normalized_import_similarity,
        candidate_wildcard_import_similarity,
        candidate_normalized_wildcard_import_similarity,
        candidate_included_by_others,
        candidate_includes_others,
    } = slices;

    let df = DataFrame::new(vec![
        Series::new(PlSmallStr::from_str("ref_id"), ref_id).into(),
        Series::new(PlSmallStr::from_str("cursor_path"), cursor_path).into(),
        Series::new(PlSmallStr::from_str("cursor_row"), cursor_row).into(),
        Series::new(PlSmallStr::from_str("cursor_column"), cursor_column).into(),
        Series::new(PlSmallStr::from_str("cursor_identifier"), cursor_identifier).into(),
        Series::new(PlSmallStr::from_str("gold_in_excerpt"), gold_in_excerpt).into(),
        Series::new(PlSmallStr::from_str("gold_path"), gold_path).into(),
        Series::new(PlSmallStr::from_str("gold_row"), gold_row).into(),
        Series::new(PlSmallStr::from_str("gold_column"), gold_column).into(),
        Series::new(PlSmallStr::from_str("gold_is_external"), gold_is_external).into(),
        Series::new(PlSmallStr::from_str("candidate_count"), candidate_count).into(),
        Series::new(PlSmallStr::from_str("candidate_path"), candidate_path).into(),
        Series::new(PlSmallStr::from_str("candidate_row"), candidate_row).into(),
        Series::new(PlSmallStr::from_str("candidate_column"), candidate_column).into(),
        Series::new(PlSmallStr::from_str("candidate_is_gold"), candidate_is_gold).into(),
        Series::new(PlSmallStr::from_str("candidate_rank"), candidate_rank).into(),
        Series::new(
            PlSmallStr::from_str("candidate_is_same_file"),
            candidate_is_same_file,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_is_referenced_nearby"),
            candidate_is_referenced_nearby,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_is_referenced_in_breadcrumb"),
            candidate_is_referenced_in_breadcrumb,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_reference_count"),
            candidate_reference_count,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_same_file_declaration_count"),
            candidate_same_file_declaration_count,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_declaration_count"),
            candidate_declaration_count,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_reference_line_distance"),
            candidate_reference_line_distance,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_declaration_line_distance"),
            candidate_declaration_line_distance,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_excerpt_vs_item_jaccard"),
            candidate_excerpt_vs_item_jaccard,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_excerpt_vs_signature_jaccard"),
            candidate_excerpt_vs_signature_jaccard,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_adjacent_vs_item_jaccard"),
            candidate_adjacent_vs_item_jaccard,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_adjacent_vs_signature_jaccard"),
            candidate_adjacent_vs_signature_jaccard,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_excerpt_vs_item_weighted_overlap"),
            candidate_excerpt_vs_item_weighted_overlap,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_excerpt_vs_signature_weighted_overlap"),
            candidate_excerpt_vs_signature_weighted_overlap,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_adjacent_vs_item_weighted_overlap"),
            candidate_adjacent_vs_item_weighted_overlap,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_adjacent_vs_signature_weighted_overlap"),
            candidate_adjacent_vs_signature_weighted_overlap,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_path_import_match_count"),
            candidate_path_import_match_count,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_wildcard_path_import_match_count"),
            candidate_wildcard_path_import_match_count,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_import_similarity"),
            candidate_import_similarity,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_max_import_similarity"),
            candidate_max_import_similarity,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_normalized_import_similarity"),
            candidate_normalized_import_similarity,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_wildcard_import_similarity"),
            candidate_wildcard_import_similarity,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_normalized_wildcard_import_similarity"),
            candidate_normalized_wildcard_import_similarity,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_included_by_others"),
            candidate_included_by_others,
        )
        .into(),
        Series::new(
            PlSmallStr::from_str("candidate_includes_others"),
            candidate_includes_others,
        )
        .into(),
    ])?;

    Ok(df)
}

fn relativize_path(path: &Path) -> &Path {
    path.strip_prefix(std::env::current_dir().unwrap())
        .unwrap_or(path)
}

struct SummaryStats {
    references_count: u32,
    retrieved_count: u32,
    top_match_count: u32,
    non_top_match_count: u32,
    ranking_involved_top_match_count: u32,
    missing_none_retrieved: u32,
    missing_wrong_retrieval: u32,
    missing_external: u32,
    in_excerpt_count: u32,
}

impl SummaryStats {
    fn from_dataframe(df: DataFrame) -> Result<Self> {
        // TODO: use lazy more
        let unique_refs =
            df.unique::<(), ()>(Some(&["ref_id".into()]), UniqueKeepStrategy::Any, None)?;
        let references_count = unique_refs.height() as u32;

        let gold_mask = df.column("candidate_is_gold")?.bool()?;
        let gold_df = df.filter(&gold_mask)?;
        let retrieved_count = gold_df.height() as u32;

        let top_match_mask = gold_df.column("candidate_rank")?.u32()?.equal(0);
        let top_match_df = gold_df.filter(&top_match_mask)?;
        let top_match_count = top_match_df.height() as u32;

        let ranking_involved_top_match_count = top_match_df
            .column("candidate_count")?
            .u32()?
            .gt(1)
            .sum()
            .unwrap_or_default();

        let non_top_match_count = (!top_match_mask).sum().unwrap_or(0);

        let not_retrieved_df = df
            .lazy()
            .group_by(&[col("ref_id"), col("candidate_count")])
            .agg(&[
                col("candidate_is_gold")
                    .fill_null(false)
                    .sum()
                    .alias("gold_count"),
                col("gold_in_excerpt").sum().alias("gold_in_excerpt_count"),
                col("gold_is_external")
                    .sum()
                    .alias("gold_is_external_count"),
            ])
            .filter(col("gold_count").eq(lit(0)))
            .collect()?;

        let in_excerpt_mask = not_retrieved_df
            .column("gold_in_excerpt_count")?
            .u32()?
            .gt(0);
        let in_excerpt_count = in_excerpt_mask.sum().unwrap_or(0);

        let missing_df = not_retrieved_df.filter(&!in_excerpt_mask)?;

        let missing_none_retrieved_mask = missing_df.column("candidate_count")?.u32()?.equal(0);
        let missing_none_retrieved = missing_none_retrieved_mask.sum().unwrap_or(0);
        let external_mask = missing_df.column("gold_is_external_count")?.u32()?.gt(0);
        let missing_external = (missing_none_retrieved_mask & external_mask)
            .sum()
            .unwrap_or(0);

        let missing_wrong_retrieval = missing_df
            .column("candidate_count")?
            .u32()?
            .gt(0)
            .sum()
            .unwrap_or(0);

        Ok(SummaryStats {
            references_count,
            retrieved_count,
            top_match_count,
            non_top_match_count,
            ranking_involved_top_match_count,
            missing_none_retrieved,
            missing_wrong_retrieval,
            missing_external,
            in_excerpt_count,
        })
    }

    fn count_and_percentage(part: u32, total: u32) -> String {
        format!("{} ({:.2}%)", part, (part as f64 / total as f64) * 100.0)
    }
}

impl std::fmt::Display for SummaryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let included = self.in_excerpt_count + self.retrieved_count;
        let missing = self.references_count - included;
        writeln!(f)?;
        writeln!(f, "╮ references: {}", self.references_count)?;
        writeln!(
            f,
            "├─╮ included: {}",
            Self::count_and_percentage(included, self.references_count),
        )?;
        writeln!(
            f,
            "│ ├─╮ retrieved: {}",
            Self::count_and_percentage(self.retrieved_count, self.references_count)
        )?;
        writeln!(
            f,
            "│ │ ├─╮ top match : {}",
            Self::count_and_percentage(self.top_match_count, self.retrieved_count)
        )?;
        writeln!(
            f,
            "│ │ │ ╰─╴ involving ranking: {}",
            Self::count_and_percentage(self.ranking_involved_top_match_count, self.top_match_count)
        )?;
        writeln!(
            f,
            "│ │ ╰─╴ non-top match: {}",
            Self::count_and_percentage(self.non_top_match_count, self.retrieved_count)
        )?;
        writeln!(
            f,
            "│ ╰─╴ in excerpt: {}",
            Self::count_and_percentage(self.in_excerpt_count, included)
        )?;
        writeln!(
            f,
            "╰─╮ missing: {}",
            Self::count_and_percentage(missing, self.references_count)
        )?;
        writeln!(
            f,
            "  ├─╮ none retrieved: {}",
            Self::count_and_percentage(self.missing_none_retrieved, missing)
        )?;
        writeln!(
            f,
            "  │ ╰─╴ external (expected): {}",
            Self::count_and_percentage(self.missing_external, missing)
        )?;
        writeln!(
            f,
            "  ╰─╴ wrong retrieval: {}",
            Self::count_and_percentage(self.missing_wrong_retrieval, missing)
        )?;
        Ok(())
    }
}

#[derive(Debug)]
struct ReferenceRetrievalResult {
    cursor_path: Arc<RelPath>,
    cursor_point: Point,
    identifier: Identifier,
    excerpt_range: Option<Range<usize>>,
    lsp_definitions: Vec<SourceRange>,
    retrieved_definitions: Vec<RetrievedDefinition>,
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
