use std::{
    collections::HashMap,
    ops::Range,
    path,
    path::Path,
    sync::{Arc, OnceLock, atomic::AtomicBool},
};

use file_icons::FileIcons;
use gpui::{App, AppContext, AsyncApp, Context, WeakEntity};
use language::{Buffer, HighlightId, LanguageRegistry};
use search::SearchOptions;
use text::{Anchor as TextAnchor, BufferId, Point, ToOffset, ToPoint};
use ui::IconName;

use crate::PickerHandle;
use crate::QuickSearchDelegate;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::{GroupHeader, GroupInfo, QuickMatch, QuickMatchKind};
use log::debug;
use project::ProjectPath;
use project::search::{SearchQuery, SearchResult};
use smol::future::yield_now;
use util::paths::{PathMatcher, PathStyle};

use super::{
    ConfirmOutcome, ListPresentation, PreviewPanelUi, QuickSearchSource, SortPolicy, SourceId,
    SourceSpec,
};

pub static TEXT_GREP_SOURCE: TextGrepSource = TextGrepSource;

pub struct TextGrepSource;

#[derive(Clone)]
struct SyntaxEnrichItem {
    key: crate::types::MatchKey,
    row: u32,
    snippet_len: usize,
}

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
        let language_registry = delegate.project.read(cx).languages().clone();

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
                        crate::record_error(picker.clone(), generation, err.to_string(), &mut app);
                        return;
                    }
                    Err(err) => {
                        crate::record_error(picker.clone(), generation, err.to_string(), &mut app);
                        return;
                    }
                };

                let receiver = match app
                    .update_entity(&project, |project, cx| project.search(search_query, cx))
                {
                    Ok(receiver) => receiver,
                    Err(err) => {
                        crate::record_error(picker.clone(), generation, err.to_string(), &mut app);
                        return;
                    }
                };

                let receiver_for_drop = receiver.clone();
                if let Some(picker) = picker.upgrade() {
                    if let Err(err) = app.update_entity(&picker, |picker, _cx| {
                        picker
                            .delegate
                            .search_engine
                            .set_inflight_results(receiver_for_drop);
                    }) {
                        debug!("quick_search: failed to store inflight results: {:?}", err);
                    }
                }

                let mut batch: Vec<QuickMatch> = Vec::with_capacity(crate::RESULTS_BATCH_SIZE);
                let mut syntax_workers: HashMap<BufferId, async_channel::Sender<SyntaxEnrichItem>> =
                    HashMap::new();
                loop {
                    let result = match receiver.recv().await {
                        Ok(r) => r,
                        Err(_) => break,
                    };
                    if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            if let Some(out) = build_matches_for_buffer(
                                &mut app,
                                &buffer,
                                ranges,
                                &path_style,
                                &source_id,
                            ) {
                                if !out.pending_syntax.is_empty() {
                                    ensure_syntax_worker(
                                        &mut app,
                                        &mut syntax_workers,
                                        out.buffer_id,
                                        buffer.clone(),
                                        picker.clone(),
                                        generation,
                                        cancel_flag.clone(),
                                        language_registry.clone(),
                                    );
                                    if let Some(sender) = syntax_workers.get(&out.buffer_id) {
                                        for item in out.pending_syntax {
                                            if let Err(err) = sender.try_send(item) {
                                                debug!(
                                                    "quick_search: failed to queue syntax enrich item: {:?}",
                                                    err
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }

                                batch.extend(out.matches);
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

                drop(syntax_workers);
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

fn clip_snippet(text: &str) -> (String, usize) {
    if text.len() <= crate::MAX_SNIPPET_BYTES {
        return (text.to_string(), text.len());
    }

    let suffix = "…";
    let max_content_bytes = crate::MAX_SNIPPET_BYTES.saturating_sub(suffix.len());
    let mut end = max_content_bytes.min(text.len());
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }

    let mut out = String::with_capacity(end + suffix.len());
    out.push_str(&text[..end]);
    out.push_str(suffix);
    (out, end)
}

fn coalesce_syntax_runs(runs: &mut Vec<(Range<usize>, HighlightId)>) {
    if runs.len() <= 1 {
        return;
    }
    runs.sort_by_key(|(range, _)| (range.start, range.end));
    let mut out: Vec<(Range<usize>, HighlightId)> = Vec::with_capacity(runs.len());
    for (range, id) in runs.drain(..) {
        if let Some((last_range, last_id)) = out.last_mut() {
            if *last_id == id && last_range.end == range.start {
                last_range.end = range.end;
                continue;
            }
        }
        out.push((range, id));
    }
    *runs = out;
}

struct BuildMatchesOutput {
    matches: Vec<QuickMatch>,
    pending_syntax: Vec<SyntaxEnrichItem>,
    buffer_id: BufferId,
}

fn ensure_syntax_worker(
    app: &mut AsyncApp,
    workers: &mut HashMap<BufferId, async_channel::Sender<SyntaxEnrichItem>>,
    buffer_id: BufferId,
    buffer: gpui::Entity<Buffer>,
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    cancel_flag: Arc<AtomicBool>,
    language_registry: Arc<LanguageRegistry>,
) {
    if workers.contains_key(&buffer_id) {
        return;
    }

    let (sender, receiver) = async_channel::unbounded();
    workers.insert(buffer_id, sender);

    app.spawn(async move |app| {
        let mut language_attempted = false;
        let mut queued: Vec<SyntaxEnrichItem> = Vec::new();

        loop {
            let first = match receiver.recv().await {
                Ok(item) => item,
                Err(_) => break,
            };
            queued.push(first);
            while let Ok(item) = receiver.try_recv() {
                queued.push(item);
            }

            if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let snapshot = match app.read_entity(&buffer, |b, _| b.snapshot()) {
                Ok(s) => s,
                Err(_) => break,
            };

            if snapshot.language().is_none() && !language_attempted {
                language_attempted = true;
                let file = app
                    .read_entity(&buffer, |b, _| b.file().cloned())
                    .ok()
                    .flatten();
                if let Some(file) = file {
                    let available = app
                        .update({
                            let language_registry = language_registry.clone();
                            let file = file.clone();
                            move |cx| language_registry.language_for_file(&file, None, cx)
                        })
                        .ok()
                        .flatten();
                    if let Some(available) = available {
                        let language_receiver = language_registry.load_language(&available);
                        if let Ok(Ok(language)) = language_receiver.await {
                            if let Err(err) = app.update_entity(&buffer, |b, cx| {
                                b.set_language_registry(language_registry.clone());
                                b.set_language_async(Some(language.clone()), cx);
                            }) {
                                debug!(
                                    "quick_search: failed to set language for syntax enrich worker: {:?}",
                                    err
                                );
                            }
                        }
                    }
                }
            }

            let parsing_idle = app.read_entity(&buffer, |b, _| b.parsing_idle());
            if let Ok(idle) = parsing_idle {
                idle.await;
            }

            while let Ok(item) = receiver.try_recv() {
                queued.push(item);
            }

            if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let snapshot = match app.read_entity(&buffer, |b, _| b.snapshot()) {
                Ok(s) => s,
                Err(_) => break,
            };
            if snapshot.language().is_none() {
                queued.clear();
                continue;
            }

            let mut patches: Vec<(crate::types::MatchKey, crate::types::QuickMatchPatch)> =
                Vec::new();

            for item in queued.drain(..) {
                let snippet_len = item.snippet_len;
                if snippet_len == 0 {
                    continue;
                }

                let max_row = snapshot.text.max_point().row;
                let row = item.row.min(max_row);
                let line_start = Point::new(row, 0);
                let line_end = Point::new(row, snapshot.text.line_len(row));
                let line_start_offset = snapshot.text.point_to_offset(line_start);
                let line_end_offset = snapshot.text.point_to_offset(line_end);

                let line_text: String = snapshot
                    .text_for_range(line_start_offset..line_end_offset)
                    .collect();
                let line_trimmed_end = line_text.trim_end();
                let trim_start = line_trimmed_end.len() - line_trimmed_end.trim_start().len();
                let snippet_end_abs = (trim_start + snippet_len).min(line_trimmed_end.len());
                if trim_start >= snippet_end_abs {
                    continue;
                }

                let mut highlight_ids: Vec<(Range<usize>, HighlightId)> = Vec::new();
                let mut current_offset = 0usize;
                for chunk in snapshot.chunks(line_start_offset..line_end_offset, true) {
                    let chunk_len = chunk.text.len();

                    if let Some(highlight_id) = chunk.syntax_highlight_id {
                        let abs_start = current_offset;
                        let abs_end = current_offset + chunk_len;
                        let rel_start = abs_start.saturating_sub(trim_start);
                        let rel_end = abs_end.saturating_sub(trim_start);
                        if rel_end > 0 && rel_start < snippet_len {
                            let clamped_start = rel_start.min(snippet_len);
                            let clamped_end = rel_end.min(snippet_len);
                            if clamped_start < clamped_end {
                                highlight_ids.push((clamped_start..clamped_end, highlight_id));
                            }
                        }
                    }

                    current_offset += chunk_len;
                }

                if highlight_ids.is_empty() {
                    continue;
                }
                coalesce_syntax_runs(&mut highlight_ids);

                patches.push((
                    item.key,
                    crate::types::QuickMatchPatch {
                        snippet_syntax_highlights: crate::types::PatchValue::SetTo(Arc::from(
                            highlight_ids.into_boxed_slice(),
                        )),
                        ..Default::default()
                    },
                ));
            }

            if !patches.is_empty() {
                crate::apply_patches_by_key(picker.clone(), generation, patches, app);
            }
        }
    })
    .detach();
}

fn build_matches_for_buffer(
    app: &mut AsyncApp,
    buffer: &gpui::Entity<Buffer>,
    ranges: Vec<Range<TextAnchor>>,
    path_style: &PathStyle,
    source_id: &Arc<str>,
) -> Option<BuildMatchesOutput> {
    let snapshot = match app.read_entity(buffer, |b, _| b.snapshot()) {
        Ok(s) => s,
        Err(_) => return None,
    };
    let buffer_id = snapshot.text.remote_id();

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
        .or_else(|| {
            path_label
                .rsplit_once('/')
                .map(|(_, name)| Arc::<str>::from(name))
        })
        .unwrap_or_else(|| path_label.clone());

    let path_segments = crate::split_path_segments(&path_label);
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

    let mut matches = Vec::with_capacity(line_order.len());
    let mut pending_syntax: Vec<SyntaxEnrichItem> = Vec::new();
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
        let location_label: Option<Arc<str>> =
            Some(format!(":{}:{}", row + 1, first_col + 1).into());

        let max_row = snapshot.text.max_point().row;
        let row = row.min(max_row);
        let line_start = Point::new(row, 0);
        let line_end = Point::new(row, snapshot.text.line_len(row));
        let line_start_offset = snapshot.text.point_to_offset(line_start);
        let line_end_offset = snapshot.text.point_to_offset(line_end);
        let line_text: String = snapshot
            .text_for_range(line_start_offset..line_end_offset)
            .collect();
        let line_trimmed_end = line_text.trim_end();
        let trim_start = line_trimmed_end.len() - line_trimmed_end.trim_start().len();
        let line_trimmed = &line_trimmed_end[trim_start..];
        let (snippet_string, snippet_content_len) = clip_snippet(line_trimmed);
        let snippet: Arc<str> = Arc::<str>::from(snippet_string);

        let mut snippet_match_positions: Vec<Range<usize>> = Vec::new();
        for r in &ranges_for_line {
            let match_start_offset = r.start.to_offset(&snapshot.text);
            let match_end_offset = r.end.to_offset(&snapshot.text);

            let start_in_line = match_start_offset.saturating_sub(line_start_offset);
            let end_in_line = match_end_offset.saturating_sub(line_start_offset);

            let start_in_preview = start_in_line.saturating_sub(trim_start);
            let end_in_preview = end_in_line.saturating_sub(trim_start);

            if start_in_preview >= snippet_content_len || end_in_preview == 0 {
                continue;
            }

            let clamped_start = start_in_preview.min(snippet_content_len);
            let clamped_end = end_in_preview.min(snippet_content_len);
            if clamped_start >= clamped_end {
                continue;
            }

            let snippet_str = snippet.as_ref();
            let mut safe_start = clamped_start.min(snippet_str.len());
            while safe_start > 0 && !snippet_str.is_char_boundary(safe_start) {
                safe_start -= 1;
            }
            let mut safe_end = clamped_end.min(snippet_str.len());
            while safe_end < snippet_str.len() && !snippet_str.is_char_boundary(safe_end) {
                safe_end += 1;
            }

            if safe_start < safe_end {
                snippet_match_positions.push(safe_start..safe_end);
            }
        }
        snippet_match_positions.sort_by_key(|r| (r.start, r.end));
        snippet_match_positions.dedup();

        let mut snippet_syntax_highlights: Vec<(Range<usize>, HighlightId)> = Vec::new();
        if snippet_content_len > 0 && snapshot.language().is_some() {
            let mut rel_offset = 0usize;
            let mut chunks = snapshot.chunks(line_start_offset..line_end_offset, true);
            for chunk in chunks.by_ref() {
                let chunk_len = chunk.text.len();
                let chunk_start = rel_offset;
                let chunk_end = rel_offset + chunk_len;
                rel_offset = chunk_end;

                let chunk_start = chunk_start.min(line_trimmed_end.len());
                let chunk_end = chunk_end.min(line_trimmed_end.len());
                let start_abs = chunk_start.max(trim_start);
                let end_abs = chunk_end.min(trim_start + snippet_content_len);
                if start_abs >= end_abs {
                    continue;
                }

                if let Some(id) = chunk.syntax_highlight_id {
                    let start_rel = start_abs - trim_start;
                    let end_rel = end_abs - trim_start;
                    if start_rel < end_rel {
                        snippet_syntax_highlights.push((start_rel..end_rel, id));
                    }
                }
            }
            coalesce_syntax_runs(&mut snippet_syntax_highlights);
        }

        let mut match_item = QuickMatch {
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
            snippet_match_positions: (!snippet_match_positions.is_empty())
                .then(|| Arc::<[Range<usize>]>::from(snippet_match_positions)),
            snippet_syntax_highlights: (!snippet_syntax_highlights.is_empty())
                .then(|| Arc::<[(Range<usize>, HighlightId)]>::from(snippet_syntax_highlights)),
            blame: None,
            kind: crate::types::QuickMatchKind::Buffer {
                buffer: buffer.clone(),
                project_path: project_path.clone(),
                ranges: ranges_for_line,
                buffer_id,
                position: Some((row, start_point.column)),
            },
        };
        match_item.key = crate::types::compute_match_key(&match_item);
        if match_item.snippet_syntax_highlights.is_none() && snippet_content_len > 0 {
            pending_syntax.push(SyntaxEnrichItem {
                key: match_item.key,
                row,
                snippet_len: snippet_content_len,
            });
        }
        matches.push(match_item);
    }

    Some(BuildMatchesOutput {
        matches,
        pending_syntax,
        buffer_id,
    })
}
