use std::{
    cmp::Ordering,
    ops::Range,
    sync::{Arc, atomic::AtomicBool},
};

use gpui::{App, Context, Entity, WeakEntity};
use project::Project;
use search::SearchOptions;
use text::{Anchor as TextAnchor, Point};
use ui::IconName;

use crate::QuickSearchDelegate;
use crate::PickerHandle;
use crate::preview::PreviewRequest;
use crate::types::QuickMatch;

#[path = "sources/files.rs"]
mod files;
#[path = "sources/text_grep.rs"]
mod text_grep;
#[path = "sources/commits.rs"]
mod commits;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SourceId(pub Arc<str>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ListPresentation {
    Flat,
    Grouped,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum SortPolicy {
    StreamOrder,
    FinalSort,
}

#[derive(Clone, Debug)]
pub struct SourceSpec {
    pub id: SourceId,
    pub title: Arc<str>,
    pub icon: IconName,
    pub placeholder: Arc<str>,
    pub supported_options: SearchOptions,
    pub min_query_len: usize,
    pub list_presentation: ListPresentation,
    pub use_diff_preview: bool,
    pub sort_policy: SortPolicy,
}

pub trait QuickSearchSource {
    fn spec(&self) -> &'static SourceSpec;

    fn cmp_matches(&self, _a: &QuickMatch, _b: &QuickMatch) -> Ordering {
        Ordering::Equal
    }

    fn start_search(
        &self,
        delegate: &mut QuickSearchDelegate,
        query: String,
        generation: usize,
        cancel_flag: Arc<AtomicBool>,
        picker: WeakEntity<PickerHandle>,
        cx: &mut Context<PickerHandle>,
    );

    fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        weak_ranges: Vec<Range<TextAnchor>>,
        use_diff_preview: bool,
        query: &str,
    ) -> PreviewRequest;

    fn weak_preview_ranges(
        &self,
        _delegate: &QuickSearchDelegate,
        _selected: &QuickMatch,
        _query: &str,
    ) -> Vec<Range<TextAnchor>> {
        Vec::new()
    }

    fn confirm_outcome_for_match(&self, selected: &QuickMatch, cx: &App) -> ConfirmOutcome;

    fn preview_panel_ui_for_match(
        &self,
        selected: &QuickMatch,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> PreviewPanelUi;
}

#[derive(Clone)]
pub struct SourceRegistry {
    sources: Arc<[&'static dyn QuickSearchSource]>,
}

#[derive(Clone)]
pub enum ConfirmOutcome {
    OpenProjectPath {
        project_path: project::ProjectPath,
        point_range: Option<Range<Point>>,
    },
    OpenGitCommit {
        repo_workdir: Arc<std::path::Path>,
        sha: Arc<str>,
    },
    Dismiss,
}

#[derive(Clone)]
pub struct GitCommitPreviewMeta {
    pub sha: Arc<str>,
    pub subject: Arc<str>,
    pub author: Arc<str>,
    pub commit_timestamp: i64,
    pub repo_label: Arc<str>,
    pub remote: Option<::git::GitRemote>,
    pub github_url: Option<Arc<str>>,
}

#[derive(Clone)]
pub enum PreviewPanelUi {
    GitCommit {
        meta: GitCommitPreviewMeta,
    },
    Standard {
        path_text: Arc<str>,
        highlights: Vec<usize>,
    },
}

impl SourceRegistry {
    pub fn default_builtin() -> Self {
        Self {
            sources: Arc::from([
                &files::FILES_SOURCE as &dyn QuickSearchSource,
                &text_grep::TEXT_GREP_SOURCE as &dyn QuickSearchSource,
                &commits::COMMITS_SOURCE as &dyn QuickSearchSource,
            ]),
        }
    }

    pub fn available_sources(&self) -> &[&'static dyn QuickSearchSource] {
        &self.sources
    }

    pub fn spec_for_id(&self, id: &SourceId) -> Option<&'static SourceSpec> {
        self.sources
            .iter()
            .find(|source| source.spec().id == *id)
            .map(|source| source.spec())
    }

    pub fn source_for_id(&self, id: &SourceId) -> Option<&'static dyn QuickSearchSource> {
        self.sources
            .iter()
            .copied()
            .find(|source| source.spec().id == *id)
    }

    pub fn source_for_match(&self, selected: &QuickMatch) -> Option<&'static dyn QuickSearchSource> {
        let id = SourceId(selected.source_id.clone());
        self.source_for_id(&id)
    }

    pub fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        weak_ranges: Vec<Range<TextAnchor>>,
        use_diff_preview: bool,
        query: &str,
    ) -> PreviewRequest {
        self.source_for_match(selected)
            .map(|s| s.preview_request_for_match(selected, weak_ranges, use_diff_preview, query))
            .unwrap_or(PreviewRequest::Empty)
    }

    pub fn weak_preview_ranges_for_match(
        &self,
        delegate: &QuickSearchDelegate,
        selected: &QuickMatch,
        query: &str,
    ) -> Vec<Range<TextAnchor>> {
        self.source_for_match(selected)
            .map(|s| s.weak_preview_ranges(delegate, selected, query))
            .unwrap_or_default()
    }

    pub fn confirm_outcome_for_match(&self, selected: &QuickMatch, cx: &App) -> ConfirmOutcome {
        self.source_for_match(selected)
            .map(|s| s.confirm_outcome_for_match(selected, cx))
            .unwrap_or(ConfirmOutcome::Dismiss)
    }

    pub fn preview_panel_ui_for_match(
        &self,
        selected: &QuickMatch,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> PreviewPanelUi {
        self.source_for_match(selected)
            .map(|s| s.preview_panel_ui_for_match(selected, project, cx))
            .unwrap_or(PreviewPanelUi::Standard {
                path_text: selected.display_path.clone(),
                highlights: selected
                    .display_path_positions
                    .as_deref()
                    .map(|p| p.to_vec())
                    .unwrap_or_default(),
            })
    }
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::default_builtin()
    }
}

pub fn default_source_id() -> SourceId {
    SourceId(Arc::from("grep"))
}

