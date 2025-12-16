use std::{
    ops::Range,
    sync::{Arc, OnceLock, atomic::AtomicBool},
};

use gpui::{App, AppContext as _, Context, WeakEntity};
use search::SearchOptions;
use text::{Anchor as TextAnchor, ToPoint};
use ui::IconName;

use crate::QuickSearchDelegate;
use crate::PickerHandle;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::{QuickMatch, QuickMatchKind};
use log::debug;
use project::search::{SearchQuery, SearchResult};
use smol::future::yield_now;
use util::paths::{PathMatcher, PathStyle};

use super::{ConfirmOutcome, ListPresentation, PreviewPanelUi, QuickSearchSource, SortPolicy, SourceId, SourceSpec};

pub static TEXT_GREP_SOURCE: TextGrepSource = TextGrepSource;

pub struct TextGrepSource;

impl TextGrepSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("grep")),
            title: Arc::from("Text"),
            icon: IconName::MagnifyingGlass,
            placeholder: Arc::from("Live grep..."),
            supported_options: SearchOptions::REGEX
                | SearchOptions::CASE_SENSITIVE
                | SearchOptions::WHOLE_WORD
                | SearchOptions::INCLUDE_IGNORED,
            min_query_len: crate::MIN_QUERY_LEN,
            list_presentation: ListPresentation::Grouped,
            use_diff_preview: false,
            sort_policy: SortPolicy::StreamOrder,
        })
    }
}

impl QuickSearchSource for TextGrepSource {
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
        let project = delegate.project.clone();
        let search_options = delegate.search_engine.search_options;
        let source_id = self.spec().id.0.clone();

        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }

                let path_style =
                    match app.update_entity(&project, |project, cx| project.path_style(cx)) {
                        Ok(style) => style,
                        Err(err) => {
                            debug!("quick_search: failed to get path style: {:?}", err);
                            PathStyle::local()
                        }
                    };
                let search_query = match app.update_entity(&project, |_project, _| {
                    let include = PathMatcher::default();
                    let exclude = PathMatcher::default();
                    if search_options.contains(SearchOptions::REGEX) {
                        SearchQuery::regex(
                            &query,
                            search_options.contains(SearchOptions::WHOLE_WORD),
                            search_options.contains(SearchOptions::CASE_SENSITIVE),
                            search_options.contains(SearchOptions::INCLUDE_IGNORED),
                            false,
                            include,
                            exclude,
                            false,
                            None,
                        )
                    } else {
                        SearchQuery::text(
                            &query,
                            search_options.contains(SearchOptions::WHOLE_WORD),
                            search_options.contains(SearchOptions::CASE_SENSITIVE),
                            search_options.contains(SearchOptions::INCLUDE_IGNORED),
                            include,
                            exclude,
                            false,
                            None,
                        )
                    }
                }) {
                    Ok(Ok(query)) => query,
                    Ok(Err(err)) => {
                        crate::record_error(
                            picker.clone(),
                            generation,
                            err.to_string(),
                            &mut app,
                        );
                        return;
                    }
                    Err(err) => {
                        crate::record_error(
                            picker.clone(),
                            generation,
                            err.to_string(),
                            &mut app,
                        );
                        return;
                    }
                };

                let rx = match app.update_entity(&project, |project, cx| project.search(search_query, cx))
                {
                    Ok(rx) => rx,
                    Err(err) => {
                        crate::record_error(
                            picker.clone(),
                            generation,
                            err.to_string(),
                            &mut app,
                        );
                        return;
                    }
                };

                let rx_for_drop = rx.clone();
                if let Some(picker) = picker.upgrade() {
                    if let Err(err) = app.update_entity(&picker, |picker, _cx| {
                        picker.delegate.search_engine.set_inflight_results(rx_for_drop);
                    }) {
                        debug!("quick_search: failed to store inflight results: {:?}", err);
                    }
                }

                let mut batch: Vec<QuickMatch> = Vec::with_capacity(crate::RESULTS_BATCH_SIZE);
                loop {
                    let result = match rx.recv().await {
                        Ok(r) => r,
                        Err(_) => break,
                    };
                    if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            if let Some(matches) = crate::build_matches_fg(
                                &mut app,
                                &buffer,
                                ranges,
                                &path_style,
                                source_id.clone(),
                            ) {
                                batch.extend(matches);
                                if batch.len() >= crate::RESULTS_BATCH_SIZE {
                                    crate::flush_batch(
                                        picker.clone(),
                                        generation,
                                        &mut batch,
                                        &mut app,
                                    );
                                    if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                                        break;
                                    }
                                }
                            }
                        }
                        SearchResult::LimitReached => {
                            if !batch.is_empty() {
                                crate::flush_batch(
                                    picker.clone(),
                                    generation,
                                    &mut batch,
                                    &mut app,
                                );
                            }
                            if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                                break;
                            }
                            break;
                        }
                    }

                    yield_now().await;
                }

                if !batch.is_empty() && !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    crate::flush_batch(picker.clone(), generation, &mut batch, &mut app);
                }
                if !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    crate::finish_stream(picker, generation, &mut app);
                }
            }
        })
        .detach();
    }

    fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        weak_ranges: Vec<Range<TextAnchor>>,
        use_diff_preview: bool,
    ) -> PreviewRequest {
        let key = PreviewKey(selected.id);
        match &selected.kind {
            QuickMatchKind::Buffer { buffer, ranges, .. } => PreviewRequest::Buffer {
                key,
                buffer: buffer.clone(),
                strong_ranges: ranges.clone(),
                weak_ranges,
                use_diff_preview,
            },
            QuickMatchKind::ProjectPath { project_path } => PreviewRequest::ProjectPath {
                key,
                project_path: project_path.clone(),
                use_diff_preview,
            },
            _ => PreviewRequest::Empty,
        }
    }

    fn weak_preview_ranges(
        &self,
        delegate: &QuickSearchDelegate,
        selected: &QuickMatch,
        query: &str,
    ) -> Vec<Range<TextAnchor>> {
        if query.len() < 3 {
            return Vec::new();
        }
        let Some(selected_buffer_id) = selected.buffer_id() else {
            return Vec::new();
        };

        const WINDOW: usize = 120;
        const MAX_RANGES: usize = 600;

        let selected_ix = delegate.selected_match_index().unwrap_or(0);
        let match_count = delegate.match_list.match_count();
        if match_count == 0 {
            return Vec::new();
        }

        let start = selected_ix.saturating_sub(WINDOW);
        let end = (selected_ix + WINDOW).min(match_count.saturating_sub(1));

        let mut weak = Vec::new();
        for ix in start..=end {
            let Some(m) = delegate.match_list.item(ix) else {
                continue;
            };
            if m.id == selected.id {
                continue;
            }
            if m.buffer_id() != Some(selected_buffer_id) {
                continue;
            }
            let Some(ranges) = m.ranges() else {
                continue;
            };
            for r in ranges {
                weak.push(r.clone());
                if weak.len() >= MAX_RANGES {
                    return weak;
                }
            }
        }

        weak
    }

    fn confirm_outcome_for_match(&self, selected: &QuickMatch, cx: &App) -> ConfirmOutcome {
        match &selected.kind {
            QuickMatchKind::ProjectPath { project_path } => ConfirmOutcome::OpenProjectPath {
                project_path: project_path.clone(),
                point_range: None,
            },
            QuickMatchKind::Buffer { buffer, ranges, .. } => {
                let project_path = buffer.read(cx).file().map(|file| project::ProjectPath {
                    worktree_id: file.worktree_id(cx),
                    path: file.path().clone(),
                });
                let point_range = ranges.first().map(|range| {
                    let snapshot = buffer.read(cx).snapshot();
                    let start = range.start.to_point(&snapshot.text);
                    let end = range.end.to_point(&snapshot.text);
                    start..end
                });
                match project_path {
                    Some(project_path) => ConfirmOutcome::OpenProjectPath {
                        project_path,
                        point_range,
                    },
                    None => ConfirmOutcome::Dismiss,
                }
            }
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

