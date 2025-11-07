use std::ops::Range;

use anyhow::Result;
use cloud_zeta2_prompt::retrieval_prompt::SearchToolQuery;
use collections::HashMap;
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedSender},
};
use gpui::{AppContext, AsyncApp, Entity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt, ToOffset, ToPoint};
use project::{
    Project, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use smol::channel;
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

struct SearchJob {
    buffer: Entity<Buffer>,
    snapshot: BufferSnapshot,
    ranges: Vec<Range<usize>>,
    query_ix: usize,
}

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

    if let Some(top_search_regex) = input_query.syntax_node.first() {
        let top_search_query = make_search(top_search_regex)?;
        let queries = input_query
            .syntax_node
            .into_iter()
            .skip(1)
            .map(|query| make_search(&query))
            .collect::<Result<Vec<_>>>()?;
        let content_query = input_query
            .content
            .map(|regex| make_search(&regex))
            .transpose()?;

        let (jobs_tx, jobs_rx) = channel::unbounded();

        let top_search_results_rx =
            project.update(cx, |project, cx| project.search(top_search_query, cx))?;

        let top_search_task = cx.spawn({
            let jobs_tx = jobs_tx.clone();
            async move |cx| {
                futures::pin_mut!(top_search_results_rx);
                while let Some(SearchResult::Buffer { buffer, ranges }) =
                    top_search_results_rx.next().await
                {
                    buffer
                        .read_with(cx, |buffer, _| buffer.parsing_idle())?
                        .await;
                    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
                    let expanded_ranges: Vec<_> = ranges
                        .into_iter()
                        .filter_map(|range| expand_to_parent_range(&range, &snapshot))
                        .collect();
                    jobs_tx
                        .send(SearchJob {
                            buffer,
                            snapshot,
                            ranges: expanded_ranges,
                            query_ix: 0,
                        })
                        .await?;
                }
                anyhow::Ok(())
            }
        });

        let n_workers = cx.background_executor().num_cpus();
        let search_job_task = cx.background_executor().scoped(|scope| {
            for _ in 0..n_workers {
                scope.spawn(async {
                    while let Ok(job) = jobs_rx.recv().await {
                        process_search_job(&results_tx, &jobs_tx, &queries, &content_query, job)
                            .await;
                    }
                });
            }
        });

        search_job_task.await;
        top_search_task.await?;
    } else if let Some(content_regex) = &input_query.content {
        let search_query = make_search(&content_regex)?;

        let results_rx = project.update(cx, |project, cx| project.search(search_query, cx))?;
        futures::pin_mut!(results_rx);

        while let Some(SearchResult::Buffer { buffer, ranges }) = results_rx.next().await {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            // todo! expand
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
    } else {
        log::warn!("Context gathering model produced a glob-only search");
    }

    anyhow::Ok(())
}

async fn process_search_job(
    results_tx: &UnboundedSender<(Entity<Buffer>, BufferSnapshot, Vec<(Range<Anchor>, usize)>)>,
    jobs_tx: &channel::Sender<SearchJob>,
    queries: &Vec<SearchQuery>,
    content_query: &Option<SearchQuery>,
    job: SearchJob,
) {
    if let Some(search_query) = queries.get(job.query_ix) {
        let mut subranges = Vec::new();
        for range in job.ranges {
            let search_results = search_query.search(&job.snapshot, Some(range)).await;

            for range in search_results {
                subranges.extend(expand_to_parent_range(&range, &job.snapshot));
            }
        }
        jobs_tx
            .send(SearchJob {
                buffer: job.buffer,
                snapshot: job.snapshot,
                ranges: subranges,
                query_ix: job.query_ix + 1,
            })
            .await
            .ok();
    } else {
        let ranges = if let Some(content_query) = content_query {
            let mut subranges = Vec::new();
            for range in job.ranges {
                let search_results = content_query.search(&job.snapshot, Some(range)).await;
                subranges.extend(search_results);
            }
            subranges
        } else {
            job.ranges
        };

        let matches = ranges
            .into_iter()
            .map(|range| {
                let size = range.len();
                (
                    job.snapshot.anchor_before(range.start)..job.snapshot.anchor_after(range.end),
                    size,
                )
            })
            .collect();

        let send_result = results_tx.unbounded_send((job.buffer, job.snapshot, matches));

        if let Err(err) = send_result
            && !err.is_disconnected()
        {
            log::error!("{err}");
        }
    }
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
