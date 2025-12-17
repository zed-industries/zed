use std::{
    sync::{Arc, OnceLock},
};

use search::SearchOptions;
use ui::IconName;

use crate::types::{QuickMatchBuilder, QuickMatchKind};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use util::rel_path::RelPath;

use crate::core::{
    ListPresentation, QuickSearchSource, SearchContext, SearchSink, SearchUiContext, SortPolicy,
    MatchBatcher, SourceId, SourceSpec, SourceSpecCore, SourceSpecUi,
};

pub struct FilesSource;

impl FilesSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("files")),
            core: SourceSpecCore {
                supported_options: SearchOptions::INCLUDE_IGNORED,
                min_query_len: 1,
                sort_policy: SortPolicy::StreamOrder,
            },
            ui: SourceSpecUi {
                title: Arc::from("Files"),
                icon: IconName::File,
                placeholder: Arc::from("Find files..."),
                list_presentation: ListPresentation::Flat,
                use_diff_preview: false,
            },
        })
    }
}

impl QuickSearchSource for FilesSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn start_search(
        &self,
        ctx: SearchContext,
        sink: SearchSink,
        cx: &mut SearchUiContext<'_>,
    ) {
        let include_ignored = ctx.search_options().contains(SearchOptions::INCLUDE_IGNORED);
        let path_style = ctx.path_style();
        let worktrees = ctx
            .project()
            .read(cx)
            .worktree_store()
            .read(cx)
            .visible_worktrees_and_single_files(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;

        let mut set_id_to_worktree_id = std::collections::HashMap::<usize, WorktreeId>::new();
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                let snapshot = worktree.snapshot();
                set_id_to_worktree_id.insert(snapshot.id().to_usize(), worktree.id());
                PathMatchCandidateSet {
                    snapshot,
                    include_ignored,
                    include_root_name,
                    candidates: project::Candidates::Files,
                }
            })
            .collect::<Vec<_>>();

        let executor = ctx.background_executor().clone();
        let source_id = self.spec().id.0.clone();
        let query = ctx.query().clone();
        let cancellation = ctx.cancellation().clone();
        let cancel_flag = cancellation.flag();
        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancellation.is_cancelled() {
                    return;
                }

                let relative_to: Option<Arc<RelPath>> = None;
                let path_matches = fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_ref(),
                    &relative_to,
                    false,
                    2_000,
                    &cancel_flag,
                    executor,
                )
                .await;

                if cancellation.is_cancelled() {
                    return;
                }

                let mut batcher = MatchBatcher::new();
                for pm in path_matches {
                    let Some(worktree_id) = set_id_to_worktree_id.get(&pm.worktree_id).copied()
                    else {
                        continue;
                    };

                    let project_path = ProjectPath {
                        worktree_id,
                        path: pm.path.clone(),
                    };

                    let full_path = pm.path_prefix.join(&pm.path);
                    let file_name_str = full_path.file_name().unwrap_or("");
                    let file_name_start = full_path
                        .as_unix_str()
                        .len()
                        .saturating_sub(file_name_str.len());
                    let mut dir_positions = pm.positions.clone();
                    let file_name_positions = dir_positions
                        .iter()
                        .filter_map(|pos| pos.checked_sub(file_name_start))
                        .collect::<Vec<_>>();

                    let display_path_string = full_path
                        .display(path_style)
                        .trim_end_matches(file_name_str)
                        .to_string();
                    dir_positions.retain(|idx| *idx < display_path_string.len());

                    let mut path_label_string = display_path_string.clone();
                    path_label_string.push_str(file_name_str);
                    let path_label: Arc<str> = Arc::from(path_label_string);
                    let display_path: Arc<str> = Arc::from(display_path_string);

                    let file_name: Arc<str> = if file_name_str.is_empty() {
                        path_label.clone()
                    } else {
                        Arc::from(file_name_str.to_string())
                    };

                    batcher.push(
                        QuickMatchBuilder::new(
                            source_id.clone(),
                            QuickMatchKind::ProjectPath { project_path },
                        )
                        .path_label(path_label)
                        .display_path(display_path)
                        .display_path_positions(Some(Arc::<[usize]>::from(dir_positions)))
                        .path_segments_from_label()
                        .file_name(file_name)
                        .file_name_positions(Some(Arc::<[usize]>::from(file_name_positions)))
                        .build(),
                        &sink,
                        &mut app,
                    );
                }

                if !cancellation.is_cancelled() {
                    batcher.finish(&sink, &mut app);
                }
            }
        })
        .detach();
    }
}
