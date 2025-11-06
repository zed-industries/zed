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
        'outer: while let Some((buffer, snapshot, excerpts)) = results_rx.next().await {
            snapshots.insert(buffer.entity_id(), snapshot);
            let existing = results.entry(buffer).or_default();
            existing.reserve(excerpts.len());

            for (range, size) in excerpts {
                // Blunt trimming of the results until we have a proper algorithmic filtering step
                if (total_bytes + size) > MAX_RESULTS_LEN {
                    log::trace!("Combined results reached limit of {MAX_RESULTS_LEN}B");
                    break 'outer;
                }
                total_bytes += size;
                existing.push(range);
            }
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
    results_tx: UnboundedSender<(Entity<Buffer>, BufferSnapshot, Vec<(Range<Anchor>, usize)>)>,
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

    while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
        if results_tx.is_closed() {
            break;
        }

        if ranges.is_empty() {
            continue;
        }

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
        let results_tx = results_tx.clone();

        cx.background_spawn(async move {
            let mut excerpts = Vec::with_capacity(ranges.len());

            for range in ranges {
                let offset_range = range.to_offset(&snapshot);
                let query_point = (offset_range.start + offset_range.len() / 2).to_point(&snapshot);

                let excerpt = EditPredictionExcerpt::select_from_buffer(
                    query_point,
                    &snapshot,
                    &EditPredictionExcerptOptions {
                        max_bytes: MAX_EXCERPT_LEN,
                        min_bytes: MIN_EXCERPT_LEN,
                        target_before_cursor_over_total_bytes: 0.5,
                    },
                    None,
                );

                if let Some(excerpt) = excerpt
                    && !excerpt.line_range.is_empty()
                {
                    excerpts.push((
                        snapshot.anchor_after(excerpt.range.start)
                            ..snapshot.anchor_before(excerpt.range.end),
                        excerpt.range.len(),
                    ));
                }
            }

            let send_result = results_tx.unbounded_send((buffer, snapshot, excerpts));

            if let Err(err) = send_result
                && !err.is_disconnected()
            {
                log::error!("{err}");
            }
        })
        .detach();
    }

    anyhow::Ok(())
}
