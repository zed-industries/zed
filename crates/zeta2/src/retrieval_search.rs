use anyhow::Result;
use cloud_zeta2_prompt::retrieval_prompt::SearchToolQuery;
use collections::HashMap;
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedSender},
};
use gpui::{AppContext, AsyncApp, Entity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt, Point, ToOffset, ToPoint};
use project::{
    Project, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use smol::channel;
use std::ops::Range;
use util::{
    ResultExt as _,
    paths::{PathMatcher, PathStyle},
};
use workspace::item::Settings as _;

#[cfg(feature = "eval-support")]
type CachedSearchResults = std::collections::BTreeMap<std::path::PathBuf, Vec<Range<usize>>>;

pub async fn run_retrieval_searches(
    queries: Vec<SearchToolQuery>,
    project: Entity<Project>,
    #[cfg(feature = "eval-support")] eval_cache: Option<std::sync::Arc<dyn crate::EvalCache>>,
    cx: &mut AsyncApp,
) -> Result<HashMap<Entity<Buffer>, Vec<Range<Anchor>>>> {
    #[cfg(feature = "eval-support")]
    let cache = if let Some(eval_cache) = eval_cache {
        use crate::EvalCacheEntryKind;
        use anyhow::Context;
        use collections::FxHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = FxHasher::default();
        project.read_with(cx, |project, cx| {
            let mut worktrees = project.worktrees(cx);
            let Some(worktree) = worktrees.next() else {
                panic!("Expected a single worktree in eval project. Found none.");
            };
            assert!(
                worktrees.next().is_none(),
                "Expected a single worktree in eval project. Found more than one."
            );
            worktree.read(cx).abs_path().hash(&mut hasher);
        })?;

        queries.hash(&mut hasher);
        let key = (EvalCacheEntryKind::Search, hasher.finish());

        if let Some(cached_results) = eval_cache.read(key) {
            let file_results = serde_json::from_str::<CachedSearchResults>(&cached_results)
                .context("Failed to deserialize cached search results")?;
            let mut results = HashMap::default();

            for (path, ranges) in file_results {
                let buffer = project
                    .update(cx, |project, cx| {
                        let project_path = project.find_project_path(path, cx).unwrap();
                        project.open_buffer(project_path, cx)
                    })?
                    .await?;
                let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
                let mut ranges: Vec<_> = ranges
                    .into_iter()
                    .map(|range| {
                        snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end)
                    })
                    .collect();
                merge_anchor_ranges(&mut ranges, &snapshot);
                results.insert(buffer, ranges);
            }

            return Ok(results);
        }

        Some((eval_cache, serde_json::to_string_pretty(&queries)?, key))
    } else {
        None
    };

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

    #[cfg(feature = "eval-support")]
    let cache = cache.clone();
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

        #[cfg(feature = "eval-support")]
        if let Some((cache, queries, key)) = cache {
            let cached_results: CachedSearchResults = results
                .iter()
                .filter_map(|(buffer, ranges)| {
                    let snapshot = snapshots.get(&buffer.entity_id())?;
                    let path = snapshot.file().map(|f| f.path());
                    let mut ranges = ranges
                        .iter()
                        .map(|range| range.to_offset(&snapshot))
                        .collect::<Vec<_>>();
                    ranges.sort_unstable_by_key(|range| (range.start, range.end));

                    Some((path?.as_std_path().to_path_buf(), ranges))
                })
                .collect();
            cache.write(
                key,
                &queries,
                &serde_json::to_string_pretty(&cached_results)?,
            );
        }

        for (buffer, ranges) in results.iter_mut() {
            if let Some(snapshot) = snapshots.get(&buffer.entity_id()) {
                merge_anchor_ranges(ranges, snapshot);
            }
        }

        Ok(results)
    })
    .await
}

pub(crate) fn merge_anchor_ranges(ranges: &mut Vec<Range<Anchor>>, snapshot: &BufferSnapshot) {
    ranges.sort_unstable_by(|a, b| {
        a.start
            .cmp(&b.start, snapshot)
            .then(b.end.cmp(&a.end, snapshot))
    });

    let mut index = 1;
    while index < ranges.len() {
        if ranges[index - 1]
            .end
            .cmp(&ranges[index].start, snapshot)
            .is_ge()
        {
            let removed = ranges.remove(index);
            if removed.end.cmp(&ranges[index - 1].end, snapshot).is_gt() {
                ranges[index - 1].end = removed.end;
            }
        } else {
            index += 1;
        }
    }
}

const MAX_EXCERPT_LEN: usize = 768;
const MAX_RESULTS_LEN: usize = MAX_EXCERPT_LEN * 5;

struct SearchJob {
    buffer: Entity<Buffer>,
    snapshot: BufferSnapshot,
    ranges: Vec<Range<usize>>,
    query_ix: usize,
    jobs_tx: channel::Sender<SearchJob>,
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

    if let Some(outer_syntax_regex) = input_query.syntax_node.first() {
        let outer_syntax_query = make_search(outer_syntax_regex)?;
        let nested_syntax_queries = input_query
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

        let outer_search_results_rx =
            project.update(cx, |project, cx| project.search(outer_syntax_query, cx))?;

        let outer_search_task = cx.spawn(async move |cx| {
            futures::pin_mut!(outer_search_results_rx);
            while let Some(SearchResult::Buffer { buffer, ranges }) =
                outer_search_results_rx.next().await
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
                        jobs_tx: jobs_tx.clone(),
                    })
                    .await?;
            }
            anyhow::Ok(())
        });

        let n_workers = cx.background_executor().num_cpus();
        let search_job_task = cx.background_executor().scoped(|scope| {
            for _ in 0..n_workers {
                scope.spawn(async {
                    while let Ok(job) = jobs_rx.recv().await {
                        process_nested_search_job(
                            &results_tx,
                            &nested_syntax_queries,
                            &content_query,
                            job,
                        )
                        .await;
                    }
                });
            }
        });

        search_job_task.await;
        outer_search_task.await?;
    } else if let Some(content_regex) = &input_query.content {
        let search_query = make_search(&content_regex)?;

        let results_rx = project.update(cx, |project, cx| project.search(search_query, cx))?;
        futures::pin_mut!(results_rx);

        while let Some(SearchResult::Buffer { buffer, ranges }) = results_rx.next().await {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            let ranges = ranges
                .into_iter()
                .map(|range| {
                    let range = range.to_offset(&snapshot);
                    let range = expand_to_entire_lines(range, &snapshot);
                    let size = range.len();
                    let range =
                        snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end);
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

async fn process_nested_search_job(
    results_tx: &UnboundedSender<(Entity<Buffer>, BufferSnapshot, Vec<(Range<Anchor>, usize)>)>,
    queries: &Vec<SearchQuery>,
    content_query: &Option<SearchQuery>,
    job: SearchJob,
) {
    if let Some(search_query) = queries.get(job.query_ix) {
        let mut subranges = Vec::new();
        for range in job.ranges {
            let start = range.start;
            let search_results = search_query.search(&job.snapshot, Some(range)).await;
            for subrange in search_results {
                let subrange = start + subrange.start..start + subrange.end;
                subranges.extend(expand_to_parent_range(&subrange, &job.snapshot));
            }
        }
        job.jobs_tx
            .send(SearchJob {
                buffer: job.buffer,
                snapshot: job.snapshot,
                ranges: subranges,
                query_ix: job.query_ix + 1,
                jobs_tx: job.jobs_tx.clone(),
            })
            .await
            .ok();
    } else {
        let ranges = if let Some(content_query) = content_query {
            let mut subranges = Vec::new();
            for range in job.ranges {
                let start = range.start;
                let search_results = content_query.search(&job.snapshot, Some(range)).await;
                for subrange in search_results {
                    let subrange = start + subrange.start..start + subrange.end;
                    subranges.push(subrange);
                }
            }
            subranges
        } else {
            job.ranges
        };

        let matches = ranges
            .into_iter()
            .map(|range| {
                let snapshot = &job.snapshot;
                let range = expand_to_entire_lines(range, snapshot);
                let size = range.len();
                let range = snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end);
                (range, size)
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

fn expand_to_entire_lines(range: Range<usize>, snapshot: &BufferSnapshot) -> Range<usize> {
    let mut point_range = range.to_point(snapshot);
    point_range.start.column = 0;
    if point_range.end.column > 0 {
        point_range.end = snapshot.max_point().min(point_range.end + Point::new(1, 0));
    }
    point_range.to_offset(snapshot)
}

fn expand_to_parent_range<T: ToPoint + ToOffset>(
    range: &Range<T>,
    snapshot: &BufferSnapshot,
) -> Option<Range<usize>> {
    let mut line_range = range.to_point(&snapshot);
    line_range.start.column = snapshot.indent_size_for_line(line_range.start.row).len;
    line_range.end.column = snapshot.line_len(line_range.end.row);
    // TODO skip result if matched line isn't the first node line?

    let node = snapshot.syntax_ancestor(line_range)?;
    Some(node.byte_range())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assemble_excerpts::assemble_excerpts;
    use cloud_zeta2_prompt::write_codeblock;
    use edit_prediction_context::Line;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use util::path;

    #[gpui::test]
    async fn test_retrieval(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "user.rs": indoc!{"
                    pub struct Organization {
                        owner: Arc<User>,
                    }

                    pub struct User {
                        first_name: String,
                        last_name: String,
                    }

                    impl Organization {
                        pub fn owner(&self) -> Arc<User> {
                            self.owner.clone()
                        }
                    }

                    impl User {
                        pub fn new(first_name: String, last_name: String) -> Self {
                            Self {
                                first_name,
                                last_name
                            }
                        }

                        pub fn first_name(&self) -> String {
                            self.first_name.clone()
                        }

                        pub fn last_name(&self) -> String {
                            self.last_name.clone()
                        }
                    }
                "},
                "main.rs": indoc!{r#"
                    fn main() {
                        let user = User::new(FIRST_NAME.clone(), "doe".into());
                        println!("user {:?}", user);
                    }
                "#},
            }),
        )
        .await;

        let project = Project::test(fs, vec![Path::new(path!("/root"))], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(rust_lang().into())
        });

        assert_results(
            &project,
            SearchToolQuery {
                glob: "user.rs".into(),
                syntax_node: vec!["impl\\s+User".into(), "pub\\s+fn\\s+first_name".into()],
                content: None,
            },
            indoc! {r#"
                `````root/user.rs
                …
                impl User {
                …
                    pub fn first_name(&self) -> String {
                        self.first_name.clone()
                    }
                …
                `````
            "#},
            cx,
        )
        .await;

        assert_results(
            &project,
            SearchToolQuery {
                glob: "user.rs".into(),
                syntax_node: vec!["impl\\s+User".into()],
                content: Some("\\.clone".into()),
            },
            indoc! {r#"
                `````root/user.rs
                …
                impl User {
                …
                    pub fn first_name(&self) -> String {
                        self.first_name.clone()
                …
                    pub fn last_name(&self) -> String {
                        self.last_name.clone()
                …
                `````
            "#},
            cx,
        )
        .await;

        assert_results(
            &project,
            SearchToolQuery {
                glob: "*.rs".into(),
                syntax_node: vec![],
                content: Some("\\.clone".into()),
            },
            indoc! {r#"
                `````root/main.rs
                fn main() {
                    let user = User::new(FIRST_NAME.clone(), "doe".into());
                …
                `````

                `````root/user.rs
                …
                impl Organization {
                    pub fn owner(&self) -> Arc<User> {
                        self.owner.clone()
                …
                impl User {
                …
                    pub fn first_name(&self) -> String {
                        self.first_name.clone()
                …
                    pub fn last_name(&self) -> String {
                        self.last_name.clone()
                …
                `````
            "#},
            cx,
        )
        .await;
    }

    async fn assert_results(
        project: &Entity<Project>,
        query: SearchToolQuery,
        expected_output: &str,
        cx: &mut TestAppContext,
    ) {
        let results = run_retrieval_searches(
            vec![query],
            project.clone(),
            #[cfg(feature = "eval-support")]
            None,
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let mut results = results.into_iter().collect::<Vec<_>>();
        results.sort_by_key(|results| {
            results
                .0
                .read_with(cx, |buffer, _| buffer.file().unwrap().path().clone())
        });

        let mut output = String::new();
        for (buffer, ranges) in results {
            buffer.read_with(cx, |buffer, cx| {
                let excerpts = ranges.into_iter().map(|range| {
                    let point_range = range.to_point(buffer);
                    if point_range.end.column > 0 {
                        Line(point_range.start.row)..Line(point_range.end.row + 1)
                    } else {
                        Line(point_range.start.row)..Line(point_range.end.row)
                    }
                });

                write_codeblock(
                    &buffer.file().unwrap().full_path(cx),
                    assemble_excerpts(&buffer.snapshot(), excerpts).iter(),
                    &[],
                    Line(buffer.max_point().row),
                    false,
                    &mut output,
                );
            });
        }
        output.pop();

        assert_eq!(output, expected_output);
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(move |cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            zlog::init_test();
        });
    }
}
