use std::ops::Range;

use anyhow::Result;
use collections::HashMap;
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedSender},
};
use gpui::{AppContext, AsyncApp, Entity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt, ToPoint as _};
use project::{
    Project, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use util::{
    ResultExt as _,
    paths::{PathMatcher, PathStyle},
};
use workspace::item::Settings as _;

pub async fn run_retrieval_searches(
    project: Entity<Project>,
    regex_by_glob: HashMap<String, String>,
    cx: &mut AsyncApp,
) -> Result<HashMap<Entity<Buffer>, Vec<Range<Anchor>>>> {
    let (exclude_matcher, path_style) = project.update(cx, |project, cx| {
        let global_settings = WorktreeSettings::get_global(cx);
        let exclude_patterns = global_settings
            .file_scan_exclusions
            .sources()
            .iter()
            .chain(global_settings.private_files.sources().iter());
        let path_style = project.path_style(cx);
        anyhow::Ok((PathMatcher::new(exclude_patterns, path_style)?, path_style))
    })??;

    let (results_tx, mut results_rx) = mpsc::unbounded();

    for (glob, regex) in regex_by_glob {
        let exclude_matcher = exclude_matcher.clone();
        let results_tx = results_tx.clone();
        let project = project.clone();
        cx.spawn(async move |cx| {
            run_query(
                &glob,
                &regex,
                results_tx.clone(),
                path_style,
                exclude_matcher,
                &project,
                cx,
            )
            .await
            .log_err();
        })
        .detach()
    }
    drop(results_tx);

    cx.background_spawn(async move {
        let mut results: HashMap<Entity<Buffer>, Vec<Range<Anchor>>> = HashMap::default();
        let mut snapshots = HashMap::default();

        let mut total_bytes = 0;
        while let Some((buffer, snapshot, ranges, size)) = results_rx.next().await {
            if total_bytes + size > MAX_RESULTS_LEN {
                break;
            }
            total_bytes += size;
            snapshots.insert(buffer.entity_id(), snapshot);
            results.entry(buffer).or_default().extend(ranges);
        }

        for (buffer, ranges) in results.iter_mut() {
            if let Some(snapshot) = snapshots.get(&buffer.entity_id()) {
                ranges.sort_unstable_by(|a, b| {
                    a.start
                        .cmp(&b.start, snapshot)
                        .then(b.end.cmp(&b.end, snapshot))
                });
            }
        }

        Ok(results)
    })
    .await
}

const MIN_EXCERPT_LEN: usize = 16;
const MAX_EXCERPT_LEN: usize = 768;
const MAX_RESULTS_LEN: usize = MAX_EXCERPT_LEN * 5;

async fn run_query(
    glob: &str,
    regex: &str,
    results_tx: UnboundedSender<(Entity<Buffer>, BufferSnapshot, Vec<Range<Anchor>>, usize)>,
    path_style: PathStyle,
    exclude_matcher: PathMatcher,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let include_matcher = PathMatcher::new(vec![glob], path_style)?;

    let query = SearchQuery::regex(
        regex,
        false,
        true,
        false,
        true,
        include_matcher,
        exclude_matcher,
        true,
        None,
    )?;

    let results = project.update(cx, |project, cx| project.search(query, cx))?;
    futures::pin_mut!(results);

    let mut total_search_bytes = 0;

    while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
        if ranges.is_empty() {
            continue;
        }

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
        let results_tx = results_tx.clone();

        cx.background_spawn(async move {
            let mut anchor_ranges = Vec::with_capacity(ranges.len());
            let mut total_buffer_bytes = 0;

            for range in ranges {
                let offset_range = range.to_offset(&snapshot);
                let query_point = (offset_range.start + offset_range.len() / 2).to_point(&snapshot);

                if total_search_bytes + MIN_EXCERPT_LEN >= MAX_RESULTS_LEN {
                    break;
                }

                let excerpt = EditPredictionExcerpt::select_from_buffer(
                    query_point,
                    &snapshot,
                    &EditPredictionExcerptOptions {
                        max_bytes: MAX_EXCERPT_LEN.min(MAX_RESULTS_LEN - total_search_bytes),
                        min_bytes: MIN_EXCERPT_LEN,
                        target_before_cursor_over_total_bytes: 0.5,
                    },
                    None,
                );

                if let Some(excerpt) = excerpt {
                    total_search_bytes += excerpt.range.len();
                    total_buffer_bytes += excerpt.range.len();
                    if !excerpt.line_range.is_empty() {
                        anchor_ranges.push(
                            snapshot.anchor_after(excerpt.range.start)
                                ..snapshot.anchor_before(excerpt.range.end),
                        );
                    }
                }
            }

            results_tx
                .unbounded_send((buffer, snapshot, anchor_ranges, total_buffer_bytes))
                .log_err();
        })
        .detach();
    }

    anyhow::Ok(())
}
