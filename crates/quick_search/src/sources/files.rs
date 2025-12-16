use std::{
    ops::Range,
    sync::{Arc, OnceLock, atomic::AtomicBool},
};

use gpui::{App, Context, WeakEntity};
use search::SearchOptions;
use text::Anchor as TextAnchor;
use ui::IconName;

use crate::QuickSearchDelegate;
use crate::PickerHandle;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::{QuickMatch, QuickMatchKind};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use util::rel_path::RelPath;

use super::{ConfirmOutcome, ListPresentation, PreviewPanelUi, QuickSearchSource, SortPolicy, SourceId, SourceSpec};

pub static FILES_SOURCE: FilesSource = FilesSource;

pub struct FilesSource;

impl FilesSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("files")),
            title: Arc::from("Files"),
            icon: IconName::File,
            placeholder: Arc::from("Find files..."),
            supported_options: SearchOptions::INCLUDE_IGNORED,
            min_query_len: 1,
            list_presentation: ListPresentation::Flat,
            use_diff_preview: false,
            sort_policy: SortPolicy::StreamOrder,
        })
    }
}

impl QuickSearchSource for FilesSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn start_search(
        &self,
        delegate: &mut QuickSearchDelegate,
        query: String,
        generation: usize,
        cancel_flag: Arc<AtomicBool>,
        picker: WeakEntity<PickerHandle>,
        cx: &mut Context<PickerHandle>,
    ) {
        let include_ignored = delegate
            .search_engine
            .search_options
            .contains(SearchOptions::INCLUDE_IGNORED);
        let path_style = delegate.project.read(cx).path_style(cx);
        let worktrees = delegate
            .project
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

        let executor = cx.background_executor().clone();
        let source_id = self.spec().id.0.clone();
        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }

                let relative_to: Option<Arc<RelPath>> = None;
                let path_matches = fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    &query,
                    &relative_to,
                    false,
                    2_000,
                    &cancel_flag,
                    executor,
                )
                .await;

                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }

                let mut batch = Vec::with_capacity(path_matches.len());
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

                    let path_segments = crate::split_path_segments(&path_label);

                    batch.push(QuickMatch {
                        id: 0,
                        key: crate::types::MatchKey(0),
                        source_id: source_id.clone(),
                        path_label,
                        display_path,
                        display_path_positions: Some(Arc::<[usize]>::from(dir_positions)),
                        path_segments,
                        file_name,
                        file_name_positions: Some(Arc::<[usize]>::from(file_name_positions)),
                        location_label: None,
                        snippet: None,
                        first_line_snippet: None,
                        blame: None,
                        kind: QuickMatchKind::ProjectPath { project_path },
                    });
                }

                if !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    crate::flush_batch(picker.clone(), generation, &mut batch, &mut app);
                    crate::finish_stream(picker, generation, &mut app);
                }
            }
        })
        .detach();
    }

    fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        _weak_ranges: Vec<Range<TextAnchor>>,
        use_diff_preview: bool,
    ) -> PreviewRequest {
        let key = PreviewKey(selected.id);
        match &selected.kind {
            QuickMatchKind::ProjectPath { project_path } => PreviewRequest::ProjectPath {
                key,
                project_path: project_path.clone(),
                use_diff_preview,
            },
            _ => PreviewRequest::Empty,
        }
    }

    fn confirm_outcome_for_match(&self, selected: &QuickMatch, _cx: &App) -> ConfirmOutcome {
        match &selected.kind {
            QuickMatchKind::ProjectPath { project_path } => ConfirmOutcome::OpenProjectPath {
                project_path: project_path.clone(),
                point_range: None,
            },
            _ => ConfirmOutcome::Dismiss,
        }
    }

    fn preview_panel_ui_for_match(
        &self,
        selected: &QuickMatch,
        _project: &gpui::Entity<project::Project>,
        _cx: &mut gpui::App,
    ) -> PreviewPanelUi {
        PreviewPanelUi::Standard {
            path_text: selected.display_path.clone(),
            highlights: selected
                .display_path_positions
                .as_deref()
                .map(|positions| positions.to_vec())
                .unwrap_or_default(),
        }
    }
}

