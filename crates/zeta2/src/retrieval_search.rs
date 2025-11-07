use std::ops::Range;

use anyhow::Result;
use cloud_zeta2_prompt::retrieval_prompt::SearchToolQuery;
use collections::HashMap;
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedSender},
};
use gpui::{AppContext, AsyncApp, Entity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt, ToOffset, ToPoint};
use project::{
    Project, WorktreeSettings,
    debugger::session::StackFrame,
    search::{SearchQuery, SearchResult},
};
use util::{
    ResultExt as _,
    paths::{PathMatcher, PathStyle},
};
use workspace::item::Settings as _;

pub async fn run_retrieval_searches(
    project: Entity<Project>,
    queries: Vec<SearchToolQuery>,
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

    for query in queries {
        let exclude_matcher = exclude_matcher.clone();
        let results_tx = results_tx.clone();
        let project = project.clone();
        cx.spawn(async move |cx| {
            run_query(
                query,
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

                let mut index = 1;
                while index < ranges.len() {
                    if ranges[index - 1]
                        .end
                        .cmp(&ranges[index].start, snapshot)
                        .is_gt()
                    {
                        let removed = ranges.remove(index);
                        ranges[index - 1].end = removed.end;
                    } else {
                        index += 1;
                    }
                }
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
    input_query: SearchToolQuery,
    results_tx: UnboundedSender<(Entity<Buffer>, BufferSnapshot, Vec<(Range<Anchor>, usize)>)>,
    path_style: PathStyle,
    exclude_matcher: PathMatcher,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let include_matcher = PathMatcher::new(vec![input_query.glob], path_style)?;

    let make_search = |regex: &str| -> Result<SearchQuery> {
        SearchQuery::regex(
            regex,
            false,
            true,
            false,
            true,
            include_matcher.clone(),
            exclude_matcher.clone(),
            true,
            None,
        )
    };

    let mut syntax_mode_regexes_iter = input_query.syntax_node.iter();

    if let Some(top_search_regex) = syntax_mode_regexes_iter.next() {
        let top_search_query = make_search(top_search_regex)?;

        let top_search_results_rx =
            project.update(cx, |project, cx| project.search(top_search_query, cx))?;
        futures::pin_mut!(top_search_results_rx);

        let mut matched_node: Option<(Entity<Buffer>, BufferSnapshot, Range<usize>)> = None;

        while let Some(SearchResult::Buffer { buffer, ranges }) = top_search_results_rx.next().await
        {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            // todo! handle the rest?
            if let Some(node_range) = ranges
                .first()
                .and_then(|range| expand_to_parent_range(range, &snapshot))
            {
                matched_node = Some((buffer, snapshot, node_range));
                break;
            };
        }

        let Some(mut matched_node) = matched_node else {
            return anyhow::Ok(());
        };

        for syntax_node_regex in syntax_mode_regexes_iter {
            let search_query = make_search(syntax_node_regex)?;

            let (_, snapshot, parent_range) = &matched_node;
            let results = search_query
                .search(&snapshot, Some(parent_range.clone()))
                .await;

            // todo! handle the rest?
            if let Some(node_range) = results
                .first()
                .and_then(|range| expand_to_parent_range(range, snapshot))
            {
                matched_node.2 = node_range;
                break;
            };
        }

        if let Some(content_regex) = input_query.content {
            let search_query = make_search(&content_regex)?;

            let (buffer, snapshot, parent_range) = matched_node;
            let results = search_query
                .search(&snapshot, Some(parent_range.clone()))
                .await;

            // todo! expand excerpts
            let ranges = results
                .into_iter()
                .map(|range| {
                    let size = range.len();
                    (
                        snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end),
                        size,
                    )
                })
                .collect();

            let send_result = results_tx.unbounded_send((buffer.clone(), snapshot.clone(), ranges));

            if let Err(err) = send_result
                && !err.is_disconnected()
            {
                log::error!("{err}");
            }
        }
    } else if let Some(content_regex) = &input_query.content {
        let search_query = make_search(&content_regex)?;

        let results_rx = project.update(cx, |project, cx| project.search(search_query, cx))?;
        futures::pin_mut!(results_rx);

        while let Some(SearchResult::Buffer { buffer, ranges }) = results_rx.next().await {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            // todo! expand excerpts
            let ranges = ranges
                .into_iter()
                .map(|range| {
                    let size = range.to_offset(&snapshot).len();
                    (range, size)
                })
                .collect();

            let send_result = results_tx.unbounded_send((buffer.clone(), snapshot.clone(), ranges));

            if let Err(err) = send_result
                && !err.is_disconnected()
            {
                log::error!("{err}");
            }
        }
    }

    anyhow::Ok(())
}

fn expand_to_parent_range<T: ToPoint + ToOffset>(
    range: &Range<T>,
    snapshot: &BufferSnapshot,
) -> Option<Range<usize>> {
    let mut line_range = range.to_point(&snapshot);
    line_range.start.column = 0;
    line_range.end.column = snapshot.line_len(line_range.start.row);
    // todo! skip result if matched line isn't the first node line?

    let node = snapshot.syntax_ancestor(line_range)?;
    Some(node.byte_range())
}
