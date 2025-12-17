use std::{
    cmp::Ordering,
    ops::Range,
    sync::{Arc, atomic::AtomicBool},
};

use gpui::{App, AppContext, AsyncApp, Context, Entity, WeakEntity};
use log::debug;
use project::Project;
use project::search::SearchResult;
use search::SearchOptions;
use text::Point;
use ui::IconName;
use util::paths::PathStyle;

use crate::PickerHandle;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::QuickMatch;
use crate::types::{MatchKey, QuickMatchPatch};
use crate::types::{MatchAction, QuickMatchKind};

pub type SearchUiContext<'a> = Context<'a, PickerHandle>;

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
pub struct SourceSpecCore {
    pub supported_options: SearchOptions,
    pub min_query_len: usize,
    pub sort_policy: SortPolicy,
}

#[derive(Clone, Debug)]
pub struct SourceSpecUi {
    pub title: Arc<str>,
    pub icon: IconName,
    pub placeholder: Arc<str>,
    pub list_presentation: ListPresentation,
    pub use_diff_preview: bool,
}

#[derive(Clone, Debug)]
pub struct SourceSpec {
    pub id: SourceId,
    pub core: SourceSpecCore,
    pub ui: SourceSpecUi,
}

#[derive(Clone)]
pub struct SearchContext {
    project: Entity<Project>,
    query: Arc<str>,
    search_options: SearchOptions,
    path_style: PathStyle,
    language_registry: Arc<language::LanguageRegistry>,
    background_executor: gpui::BackgroundExecutor,
    cancel_flag: Arc<AtomicBool>,
}

impl SearchContext {
    pub fn new(
        project: Entity<Project>,
        query: Arc<str>,
        search_options: SearchOptions,
        path_style: PathStyle,
        language_registry: Arc<language::LanguageRegistry>,
        cancel_flag: Arc<AtomicBool>,
        background_executor: gpui::BackgroundExecutor,
    ) -> Self {
        Self {
            project,
            query,
            search_options,
            path_style,
            language_registry,
            background_executor,
            cancel_flag,
        }
    }

    pub fn cancel_flag(&self) -> Arc<AtomicBool> {
        self.cancel_flag.clone()
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn query(&self) -> &Arc<str> {
        &self.query
    }

    pub fn search_options(&self) -> SearchOptions {
        self.search_options
    }

    pub fn path_style(&self) -> PathStyle {
        self.path_style
    }

    pub fn language_registry(&self) -> &Arc<language::LanguageRegistry> {
        &self.language_registry
    }

    pub fn background_executor(&self) -> &gpui::BackgroundExecutor {
        &self.background_executor
    }
}

#[derive(Clone)]
pub struct SearchSink {
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    cancel_flag: Arc<AtomicBool>,
}

impl SearchSink {
    pub fn new(
        picker: WeakEntity<PickerHandle>,
        generation: usize,
        cancel_flag: Arc<AtomicBool>,
    ) -> Self {
        Self {
            picker,
            generation,
            cancel_flag,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_flag.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn record_error(&self, message: String, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }
        crate::record_error(self.picker.clone(), self.generation, message, app);
    }

    pub fn finish_stream(&self, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }
        crate::finish_stream(self.picker.clone(), self.generation, app);
    }

    pub fn flush_batch(&self, batch: &mut Vec<QuickMatch>, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }
        crate::flush_batch(self.picker.clone(), self.generation, batch, app);
    }

    pub fn apply_patches_by_key(&self, patches: Vec<(MatchKey, QuickMatchPatch)>, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }
        crate::apply_patches_by_key(self.picker.clone(), self.generation, patches, app);
    }

    pub fn set_query_notice(&self, notice: Option<String>, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }

        let Some(picker_entity) = self.picker.upgrade() else {
            return;
        };
        if let Err(err) = app.update_entity(&picker_entity, |picker, cx| {
            if picker.delegate.search_engine.generation() != self.generation {
                return;
            }
            picker.delegate.query_notice = notice.clone();
            cx.notify();
        }) {
            debug!("quick_search: failed to set query notice: {:?}", err);
        }
    }

    pub fn set_inflight_results(&self, rx: async_channel::Receiver<SearchResult>, app: &mut AsyncApp) {
        if self.is_cancelled() {
            return;
        }

        let Some(picker_entity) = self.picker.upgrade() else {
            return;
        };
        if let Err(err) = app.update_entity(&picker_entity, |picker, _cx| {
            if picker.delegate.search_engine.generation() != self.generation {
                return;
            }
            picker.delegate.search_engine.set_inflight_results(rx.clone());
        }) {
            debug!("quick_search: failed to store inflight results: {:?}", err);
        }
    }
}

pub trait QuickSearchSource {
    fn spec(&self) -> &'static SourceSpec;

    fn cmp_matches(&self, _a: &QuickMatch, _b: &QuickMatch) -> Ordering {
        Ordering::Equal
    }

    fn start_search(
        &self,
        ctx: SearchContext,
        sink: SearchSink,
        cx: &mut SearchUiContext<'_>,
    );
}

#[derive(Clone)]
pub struct SourceRegistry {
    sources: Arc<[Arc<dyn QuickSearchSource>]>,
}

pub struct SourceRegistryBuilder {
    sources: Vec<Arc<dyn QuickSearchSource>>,
}

impl SourceRegistryBuilder {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    pub fn with_source<T: QuickSearchSource + 'static>(mut self, source: T) -> Self {
        self.sources.push(Arc::new(source));
        self
    }

    pub fn build(self) -> SourceRegistry {
        SourceRegistry {
            sources: Arc::from(self.sources),
        }
    }
}

pub struct MatchBatcher {
    batch: Vec<QuickMatch>,
}

impl MatchBatcher {
    pub fn new() -> Self {
        Self {
            batch: Vec::with_capacity(crate::RESULTS_BATCH_SIZE),
        }
    }

    pub fn push(&mut self, match_item: QuickMatch, sink: &SearchSink, app: &mut AsyncApp) {
        self.batch.push(match_item);
        if self.batch.len() >= crate::RESULTS_BATCH_SIZE {
            sink.flush_batch(&mut self.batch, app);
        }
    }

    pub fn flush(&mut self, sink: &SearchSink, app: &mut AsyncApp) {
        sink.flush_batch(&mut self.batch, app);
    }

    pub fn finish(mut self, sink: &SearchSink, app: &mut AsyncApp) {
        self.flush(sink, app);
        sink.finish_stream(app);
    }
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
    pub fn builder() -> SourceRegistryBuilder {
        SourceRegistryBuilder::new()
    }

    pub fn default_builtin() -> Self {
        Self::builder()
            .with_source(crate::sources::files::FilesSource)
            .with_source(crate::sources::text_grep::TextGrepSource)
            .with_source(crate::sources::commits::CommitsSource)
            .build()
    }

    pub fn available_sources(&self) -> &[Arc<dyn QuickSearchSource>] {
        &self.sources
    }

    pub fn spec_for_id(&self, id: &SourceId) -> Option<&'static SourceSpec> {
        self.sources
            .iter()
            .find(|source| source.spec().id == *id)
            .map(|source| source.spec())
    }

    pub fn source_for_id(&self, id: &SourceId) -> Option<&dyn QuickSearchSource> {
        self.sources
            .iter()
            .find(|source| source.spec().id == *id)
            .map(|source| source.as_ref())
    }

    pub fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        weak_ranges: Vec<Range<Point>>,
        use_diff_preview: bool,
        query: &str,
        project: &Entity<Project>,
        cx: &App,
    ) -> PreviewRequest {
        let key = PreviewKey(selected.id);
        match &selected.kind {
            QuickMatchKind::Buffer {
                buffer_id,
                ranges,
                ..
            } => {
                let buffer = project.read(cx).buffer_for_id(*buffer_id, cx);
                let Some(buffer) = buffer else {
                    return match &selected.action {
                        MatchAction::OpenProjectPath { project_path, .. } => {
                            PreviewRequest::ProjectPath {
                                key,
                                project_path: project_path.clone(),
                                use_diff_preview,
                            }
                        }
                        _ => PreviewRequest::Empty,
                    };
                };

                let snapshot = buffer.read(cx).snapshot();
                let strong_ranges = ranges
                    .iter()
                    .map(|range| crate::types::point_range_to_anchor_range(range.clone(), &snapshot.text))
                    .collect::<Vec<_>>();
                let weak_ranges = weak_ranges
                    .into_iter()
                    .map(|range| crate::types::point_range_to_anchor_range(range, &snapshot.text))
                    .collect::<Vec<_>>();

                PreviewRequest::Buffer {
                    key,
                    buffer,
                    strong_ranges,
                    weak_ranges,
                    use_diff_preview,
                }
            }
            QuickMatchKind::ProjectPath { project_path } => PreviewRequest::ProjectPath {
                key,
                project_path: project_path.clone(),
                use_diff_preview,
            },
            QuickMatchKind::GitCommit {
                repo_workdir, sha, ..
            } => PreviewRequest::GitCommit {
                key,
                repo_workdir: repo_workdir.clone(),
                sha: sha.clone(),
                query: Arc::<str>::from(query.to_string()),
            },
        }
    }

    pub fn confirm_outcome_for_match(&self, selected: &QuickMatch, _cx: &App) -> ConfirmOutcome {
        match &selected.action {
            MatchAction::OpenGitCommit { repo_workdir, sha } => ConfirmOutcome::OpenGitCommit {
                repo_workdir: repo_workdir.clone(),
                sha: sha.clone(),
            },
            MatchAction::OpenProjectPath {
                project_path,
                point_range,
            } => {
                let mut point_range = point_range.clone();
                if point_range.is_none() {
                    if let QuickMatchKind::Buffer { ranges, .. } = &selected.kind {
                        point_range = ranges.first().cloned();
                    }
                }
                ConfirmOutcome::OpenProjectPath {
                    project_path: project_path.clone(),
                    point_range,
                }
            }
            MatchAction::Dismiss => ConfirmOutcome::Dismiss,
        }
    }

    pub fn preview_panel_ui_for_match(
        &self,
        selected: &QuickMatch,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> PreviewPanelUi {
        match &selected.kind {
            QuickMatchKind::GitCommit {
                repo_workdir,
                sha,
                commit_timestamp,
                ..
            } => PreviewPanelUi::GitCommit {
                meta: {
                    let remote = resolve_git_remote_for_workdir(repo_workdir, project, cx);
                    let github_url = remote.as_ref().map(|remote| {
                        Arc::<str>::from(format!(
                            "{}/{}/{}/commit/{}",
                            remote.host.base_url(),
                            remote.owner,
                            remote.repo,
                            sha,
                        ))
                    });
                    GitCommitPreviewMeta {
                        sha: sha.clone(),
                        subject: selected
                            .snippet
                            .clone()
                            .unwrap_or_else(|| Arc::<str>::from("")),
                        author: selected
                            .location_label
                            .clone()
                            .unwrap_or_else(|| Arc::<str>::from("")),
                        commit_timestamp: *commit_timestamp,
                        repo_label: selected.path_label.clone(),
                        remote,
                        github_url,
                    }
                },
            },
            _ => PreviewPanelUi::Standard {
                path_text: selected.display_path.clone(),
                highlights: selected
                    .display_path_positions
                    .as_deref()
                    .map(|positions| positions.to_vec())
                    .unwrap_or_default(),
            },
        }
    }
}

fn resolve_git_remote_for_workdir(
    repo_workdir: &Arc<std::path::Path>,
    project: &Entity<Project>,
    cx: &mut App,
) -> Option<::git::GitRemote> {
    let git_store = project.read(cx).git_store().read(cx);
    let repo = git_store
        .repositories()
        .values()
        .find(|repo| repo.read(cx).work_directory_abs_path.as_ref() == repo_workdir.as_ref())?;

    let snapshot = repo.read(cx).snapshot();
    let remote_url = snapshot
        .remote_upstream_url
        .as_ref()
        .or(snapshot.remote_origin_url.as_ref())?;

    let provider_registry = ::git::GitHostingProviderRegistry::default_global(cx);
    let (host, parsed) = ::git::parse_git_remote_url(provider_registry, remote_url)?;
    Some(::git::GitRemote {
        host,
        owner: parsed.owner.into(),
        repo: parsed.repo.into(),
    })
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::default_builtin()
    }
}

pub fn default_source_id() -> SourceId {
    SourceId(Arc::from("grep"))
}
