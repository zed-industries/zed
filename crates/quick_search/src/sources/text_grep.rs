use std::{
    ops::Range,
    path,
    path::Path,
    sync::{Arc, OnceLock, atomic::AtomicBool},
};

use file_icons::FileIcons;
use gpui::{App, AppContext as _, AsyncApp, Context, WeakEntity};
use language::Buffer;
use search::SearchOptions;
use text::{Anchor as TextAnchor, Point, ToPoint};
use ui::IconName;

use crate::QuickSearchDelegate;
use crate::PickerHandle;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::{GroupHeader, GroupInfo, QuickMatch, QuickMatchKind};
use log::debug;
use project::ProjectPath;
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
                            if let Some(matches) = build_matches_for_buffer(
                                &mut app,
                                &buffer,
                                ranges,
                                &path_style,
                                &source_id,
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
        _query: &str,
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

fn elide_path(segments: &[Arc<str>]) -> Arc<str> {
    const MAX_SEGMENTS: usize = 5;
    if segments.is_empty() {
        return Arc::<str>::from("");
    }
    if segments.len() <= MAX_SEGMENTS {
        return Arc::<str>::from(segments.join("/"));
    }

    let head = &segments[0];
    let tail_count = MAX_SEGMENTS.saturating_sub(1);
    let tail_start = segments.len().saturating_sub(tail_count);
    let mut parts = Vec::with_capacity(2 + tail_count);
    parts.push(head.clone());
    parts.push(Arc::<str>::from("…"));
    parts.extend_from_slice(&segments[tail_start..]);
    Arc::<str>::from(parts.join("/"))
}

fn clip_snippet(text: &str) -> String {
    for (count, (idx, _)) in text.char_indices().enumerate() {
        if count == crate::MAX_SNIPPET_CHARS {
            let clipped = &text[..idx];
            return format!("{clipped}.");
        }
    }
    text.to_string()
}

fn build_matches_for_buffer(
    app: &mut AsyncApp,
    buffer: &gpui::Entity<Buffer>,
    ranges: Vec<Range<TextAnchor>>,
    path_style: &PathStyle,
    source_id: &Arc<str>,
) -> Option<Vec<QuickMatch>> {
    let snapshot = match app.read_entity(buffer, |b, _| b.snapshot()) {
        Ok(s) => s,
        Err(_) => return None,
    };

    let (project_path, path_label): (Option<ProjectPath>, Arc<str>) = app
        .read_entity(buffer, |b, cx| {
            let Some(file) = b.file() else {
                return (None, Arc::<str>::from("<untitled>"));
            };
            let project_path = ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path().clone(),
            };
            let path_label: Arc<str> =
                Arc::<str>::from(file.path().display(*path_style).to_string());
            (Some(project_path), path_label)
        })
        .unwrap_or((None, Arc::<str>::from("<untitled>")));

    let file_name: Arc<str> = path_label
        .rsplit_once(path::MAIN_SEPARATOR)
        .map(|(_, name)| Arc::<str>::from(name))
        .or_else(|| path_label.rsplit_once('/').map(|(_, name)| Arc::<str>::from(name)))
        .unwrap_or_else(|| path_label.clone());

    let path_segments = crate::split_path_segments(&path_label);
    let buffer_id = snapshot.text.remote_id();
    let display_path: Arc<str> = elide_path(&path_segments);

    let group: Option<Arc<GroupInfo>> = project_path.as_ref().map(|project_path| {
        let title: Arc<str> = project_path
            .path
            .file_name()
            .map(|name| Arc::<str>::from(name.to_string()))
            .unwrap_or_else(|| Arc::<str>::from(project_path.path.as_unix_str().to_string()));
        let subtitle: Option<Arc<str>> = project_path.path.parent().and_then(|path| {
            let s = path.as_unix_str().to_string();
            (!s.is_empty()).then(|| Arc::<str>::from(s))
        });
        let icon_path = app
            .update({
                let file_name = file_name.clone();
                move |cx| FileIcons::get_icon(Path::new(file_name.as_ref()), cx)
            })
            .ok()
            .flatten();

        Arc::new(GroupInfo {
            key: crate::types::compute_group_key_for_project_path(source_id, project_path),
            header: GroupHeader {
                icon_name: IconName::File,
                icon_path,
                title,
                subtitle,
            },
        })
    });

    let mut per_line: std::collections::HashMap<u32, Vec<(u32, Range<TextAnchor>)>> =
        std::collections::HashMap::new();
    let mut line_order: Vec<u32> = Vec::new();
    for range in ranges {
        let start = range.start.to_point(&snapshot.text);
        let row = start.row;
        if !per_line.contains_key(&row) {
            line_order.push(row);
        }
        per_line.entry(row).or_default().push((start.column, range));
    }

    let mut line_cache: std::collections::HashMap<u32, Arc<str>> = std::collections::HashMap::new();
    let mut matches = Vec::with_capacity(line_order.len());
    for row in line_order {
        let mut items = match per_line.remove(&row) {
            Some(v) => v,
            None => continue,
        };
        items.sort_by_key(|(col, _)| *col);

        let mut ranges_for_line = Vec::with_capacity(items.len());
        for (_, r) in &items {
            ranges_for_line.push(r.clone());
        }

        let (first_col, first_range) = &items[0];
        let start_point = first_range.start.to_point(&snapshot.text);
        let end_point = first_range.end.to_point(&snapshot.text);
        let location_label: Option<Arc<str>> =
            Some(format!(":{}:{}", row + 1, first_col + 1).into());

        let snippet: Arc<str> = line_cache
            .entry(row)
            .or_insert_with(|| {
                let max_row = snapshot.text.max_point().row;
                let row = row.min(max_row);
                let line_start = Point::new(row, 0);
                let line_end = Point::new(row, snapshot.text.line_len(row));
                let mut line_text = String::new();
                for part in snapshot.text.text_for_range(line_start..line_end) {
                    line_text.push_str(part);
                }
                let clipped = clip_snippet(line_text.trim_end());
                Arc::<str>::from(clipped)
            })
            .clone();

        matches.push(QuickMatch {
            id: 0,
            key: crate::types::MatchKey(0),
            source_id: source_id.clone(),
            group: group.clone(),
            path_label: path_label.clone(),
            display_path: display_path.clone(),
            display_path_positions: None,
            path_segments: path_segments.clone(),
            file_name: file_name.clone(),
            file_name_positions: None,
            location_label,
            snippet: Some(snippet.clone()),
            first_line_snippet: Some(snippet),
            blame: None,
            kind: crate::types::QuickMatchKind::Buffer {
                buffer: buffer.clone(),
                project_path: project_path.clone(),
                ranges: ranges_for_line,
                buffer_id,
                position: Some((row, start_point.column)),
                position_end: Some((end_point.row, end_point.column)),
            },
        });
    }

    Some(matches)
}

