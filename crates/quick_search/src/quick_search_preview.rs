use super::QuickSearch;
use anyhow::{Context as AnyhowContext, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{
    Addon, Anchor as MultiBufferAnchor, Editor, EditorMode, MultiBuffer, SelectionEffects,
    SizingBehavior, scroll::Autoscroll,
};
use gpui::Entity;
use gpui::{AppContext, Context, IntoElement, Subscription, WeakEntity, Window};
use language::{
    Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, ReplicaId, Rope, TextBuffer,
};
use log::debug;
use multi_buffer::{ExcerptRange, PathKey};
use project::Project;
use project::WorktreeId;
use std::{
    any::Any,
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use text::{Anchor as TextAnchor, BufferId, Point, ToPoint};
use ui::LabelCommon;
use util::ResultExt;
use util::paths::PathStyle;
use util::rel_path::RelPath;

use crate::GenerationGuard;
use project::ProjectPath;
use project::debounced_delay::DebouncedDelay;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct PreviewKey(pub u64);

#[derive(Clone)]
pub(crate) enum PreviewRequest {
    Empty,
    Buffer {
        key: PreviewKey,
        buffer: Entity<Buffer>,
        strong_ranges: Vec<Range<TextAnchor>>,
        weak_ranges: Vec<Range<TextAnchor>>,
        use_diff_preview: bool,
    },
    ProjectPath {
        key: PreviewKey,
        project_path: ProjectPath,
        use_diff_preview: bool,
    },
    GitCommit {
        key: PreviewKey,
        repo_workdir: Arc<std::path::Path>,
        sha: Arc<str>,
        query: Arc<str>,
    },
}

struct PreviewManager {
    generation_guard: GenerationGuard,
    debounce: DebouncedDelay<QuickSearch>,
}

impl PreviewManager {
    fn new() -> Self {
        Self {
            generation_guard: GenerationGuard::new(),
            debounce: DebouncedDelay::new(),
        }
    }

    fn generation(&self) -> usize {
        self.generation_guard.generation()
    }

    fn begin_request(&mut self) -> (usize, Arc<AtomicBool>) {
        self.generation_guard.begin_request()
    }

    fn debounce_mut(&mut self) -> &mut DebouncedDelay<QuickSearch> {
        &mut self.debounce
    }
}

pub struct PreviewState {
    project: Entity<Project>,
    text_preview_multi: Entity<MultiBuffer>,
    text_preview_editor: Entity<Editor>,
    diff_preview_multi: Entity<MultiBuffer>,
    diff_preview_editor: Entity<Editor>,
    use_diff_preview: bool,
    manager: PreviewManager,
    pub current_preview: Option<PreviewKey>,
    pub current_preview_anchors: Option<Vec<Range<TextAnchor>>>,
    pub current_weak_preview_anchors: Option<Vec<Range<TextAnchor>>>,
    pub needs_preview_scroll: bool,
    pub error_message: Option<String>,
    _text_scroll_subscription: Subscription,
    _diff_scroll_subscription: Subscription,
}

enum QuickSearchPreviewStrongHighlights {}
enum QuickSearchPreviewWeakHighlights {}

impl PreviewState {
    pub fn new(
        project: Entity<project::Project>,
        initial_buffer: Entity<language::Buffer>,
        window: &mut Window,
        cx: &mut Context<QuickSearch>,
    ) -> Self {
        let project_for_text = project.clone();
        let (text_preview_multi, text_preview_editor) =
            build_preview_editor(initial_buffer.clone(), project_for_text, false, window, cx);
        let project_for_diff = project.clone();
        let (diff_preview_multi, diff_preview_editor) =
            build_preview_editor(initial_buffer, project_for_diff, true, window, cx);

        let text_preview_editor_handle = text_preview_editor.clone();
        let text_sub = cx.subscribe_in(
            &text_preview_editor_handle,
            window,
            |this: &mut QuickSearch, _editor, event, _window, _cx| {
                if let editor::EditorEvent::ScrollPositionChanged {
                    autoscroll: false, ..
                } = event
                {
                    this.preview.needs_preview_scroll = false;
                }
            },
        );

        let diff_preview_editor_handle = diff_preview_editor.clone();
        let diff_sub = cx.subscribe_in(
            &diff_preview_editor_handle,
            window,
            |this: &mut QuickSearch, _editor, event, _window, _cx| {
                if let editor::EditorEvent::ScrollPositionChanged {
                    autoscroll: false, ..
                } = event
                {
                    this.preview.needs_preview_scroll = false;
                }
            },
        );

        Self {
            project,
            text_preview_multi,
            text_preview_editor,
            diff_preview_multi,
            diff_preview_editor,
            use_diff_preview: false,
            manager: PreviewManager::new(),
            current_preview: None,
            current_preview_anchors: None,
            current_weak_preview_anchors: None,
            needs_preview_scroll: false,
            error_message: None,
            _text_scroll_subscription: text_sub,
            _diff_scroll_subscription: diff_sub,
        }
    }

    fn active_preview_multi(&self) -> &Entity<MultiBuffer> {
        if self.use_diff_preview {
            &self.diff_preview_multi
        } else {
            &self.text_preview_multi
        }
    }

    fn active_preview_editor(&self) -> &Entity<Editor> {
        if self.use_diff_preview {
            &self.diff_preview_editor
        } else {
            &self.text_preview_editor
        }
    }

    pub fn preview_editor(&self) -> Entity<Editor> {
        self.active_preview_editor().clone()
    }

    pub(super) fn project(&self) -> Entity<Project> {
        self.project.clone()
    }

    pub fn set_error(&mut self, message: impl Into<String>) {
        self.error_message = Some(message.into());
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    fn replace_preview(&mut self, buffer: Entity<language::Buffer>, cx: &mut Context<QuickSearch>) {
        let buffer_id = buffer.read(cx).remote_id();
        self.active_preview_editor().update(cx, |editor, cx| {
            editor.disable_header_for_buffer(buffer_id, cx);
        });
        self.active_preview_multi().update(cx, |multi, cx| {
            multi.clear(cx);
            multi.push_excerpts(
                buffer,
                [ExcerptRange::new(text::Anchor::min_max_range_for_buffer(
                    buffer_id,
                ))],
                cx,
            );
        });
        self.needs_preview_scroll = false;
    }

    pub fn request_preview(
        &mut self,
        request: PreviewRequest,
        owner: &WeakEntity<QuickSearch>,
        window: &mut Window,
        cx: &mut Context<QuickSearch>,
    ) {
        self.maybe_update_preview(request, owner, window, cx);
    }

    pub fn maybe_update_preview(
        &mut self,
        request: PreviewRequest,
        owner: &WeakEntity<QuickSearch>,
        window: &mut Window,
        cx: &mut Context<QuickSearch>,
    ) {
        let (preview_key, strong_anchors, weak_anchors, use_diff_preview) = match &request {
            PreviewRequest::Empty => (None, None, None, false),
            PreviewRequest::Buffer {
                key,
                strong_ranges,
                weak_ranges,
                use_diff_preview,
                ..
            } => (
                Some(key.clone()),
                Some(strong_ranges.clone()),
                Some(weak_ranges.clone()),
                *use_diff_preview,
            ),
            PreviewRequest::ProjectPath {
                key,
                use_diff_preview,
                ..
            } => (Some(key.clone()), None, None, *use_diff_preview),
            PreviewRequest::GitCommit { key, .. } => (Some(key.clone()), None, None, true),
        };

        if matches!(request, PreviewRequest::Empty) {
            self.manager.begin_request();
            self.current_preview = None;
            self.current_preview_anchors = None;
            self.current_weak_preview_anchors = None;
            self.needs_preview_scroll = false;
            self.use_diff_preview = false;
            self.apply_preview_highlights(cx);
            return;
        }

        let Some(preview_key) = preview_key else {
            return;
        };

        let same_preview = self.current_preview.as_ref() == Some(&preview_key);
        let same_anchors = self.current_preview_anchors.as_ref() == strong_anchors.as_ref();
        let same_weak = self.current_weak_preview_anchors.as_ref() == weak_anchors.as_ref();
        let same_presentation = self.use_diff_preview == use_diff_preview;

        if same_preview && same_anchors && same_weak && same_presentation {
            if self.needs_preview_scroll {
                self.apply_preview_selection(window, cx);
            }
            return;
        }

        if same_preview && same_anchors && !same_weak && same_presentation {
            self.current_weak_preview_anchors = weak_anchors;
            self.apply_preview_highlights(cx);
            return;
        }

        self.current_preview_anchors = strong_anchors;
        self.current_weak_preview_anchors = weak_anchors;
        self.needs_preview_scroll = false;

        let (preview_generation, cancel_flag) = self.manager.begin_request();

        let quick_search = owner.clone();
        let project_for_task = self.project.clone();
        let request_for_task = request.clone();
        let preview_key_for_task = preview_key.clone();
        self.current_preview = Some(preview_key);
        self.use_diff_preview = use_diff_preview;

        if let PreviewRequest::Buffer { buffer, .. } = &request {
            let same_buffer = self
                .active_preview_multi()
                .read(cx)
                .as_singleton()
                .map(|b| b == *buffer)
                .unwrap_or(false);

            if !same_buffer {
                self.replace_preview(buffer.clone(), cx);
            }
            self.needs_preview_scroll = true;
            self.apply_preview_highlights(cx);
            self.apply_preview_selection(window, cx);
            return;
        }

        let window_handle = window.window_handle();
        self.manager
            .debounce_mut()
            .fire_new(Duration::from_millis(24), cx, move |_, cx| {
                cx.spawn(move |_, app: &mut gpui::AsyncApp| {
                    let mut app = app.clone();
                    async move {
                    if cancel_flag.load(Ordering::SeqCst) {
                        return;
                    }

                    if let PreviewRequest::GitCommit { repo_workdir, sha, .. } = &request_for_task
                    {
                        let query_for_commit = match &request_for_task {
                            PreviewRequest::GitCommit { query, .. } => query.clone(),
                            _ => Arc::<str>::from(""),
                        };

                        let placeholder = format!("Loading commit {sha}…\n");
                        if let Some(qs) = quick_search.upgrade() {
                            if let Err(err) = app.update_entity(&qs, |qs, cx| {
                                if qs.preview.current_preview.as_ref() != Some(&preview_key_for_task)
                                    || qs.preview.manager.generation() != preview_generation
                                {
                                    return;
                                }
                                let buffer = cx.new(|cx| language::Buffer::local(&placeholder, cx));
                                qs.preview.replace_preview(buffer, cx);
                                qs.preview.needs_preview_scroll = false;
                                qs.preview.apply_preview_highlights(cx);
                                cx.notify();
                            }) {
                                debug!("quick_search: failed to set git preview placeholder: {:?}", err);
                            }
                        }

                        let repository = app
                            .read_entity(&project_for_task, |project, cx| {
                                project
                                    .git_store()
                                    .read(cx)
                                    .repositories()
                                    .values()
                                    .find(|repo| {
                                        repo.read(cx).work_directory_abs_path.as_ref()
                                            == repo_workdir.as_ref()
                                    })
                                    .cloned()
                            })
                            .ok()
                            .flatten();

                        let Some(repository) = repository else {
                            let text = format!(
                                "Failed to load commit:\nNo repository found for {}\n",
                                repo_workdir.display()
                            );
                            if let Some(qs) = quick_search.upgrade() {
                                if let Err(err) = app.update_entity(&qs, |qs, cx| {
                                    if qs.preview.current_preview.as_ref() != Some(&preview_key_for_task)
                                        || qs.preview.manager.generation() != preview_generation
                                    {
                                        return;
                                    }
                                    let buffer =
                                        cx.new(|cx| language::Buffer::local(text.clone(), cx));
                                    qs.preview.replace_preview(buffer, cx);
                                    qs.preview.needs_preview_scroll = false;
                                    qs.preview.apply_preview_highlights(cx);
                                    cx.notify();
                                }) {
                                    debug!("quick_search: failed to show repo missing preview: {:?}", err);
                                }
                            }
                            return;
                        };

                        let Ok(language_registry) =
                            app.read_entity(&project_for_task, |project, _| project.languages().clone())
                        else {
                            return;
                        };

                        let first_worktree_id = app
                            .read_entity(&project_for_task, |project, cx| {
                                project
                                    .worktrees(cx)
                                    .next()
                                    .map(|worktree| worktree.read(cx).id())
                            })
                            .ok()
                            .flatten();

                        let commit_diff_rx = app
                            .update_entity(&repository, |repo, _| repo.load_commit_diff(sha.to_string()))
                            .ok();
                        let Some(commit_diff_rx) = commit_diff_rx else {
                            return;
                        };

                        let commit_diff = match commit_diff_rx.await {
                            Ok(Ok(d)) => d,
                            Ok(Err(err)) => {
                                let text = format!("Failed to load commit diff:\n{err:?}\n");
                                if let Some(qs) = quick_search.upgrade() {
                                    if let Err(err) = app.update_entity(&qs, |qs, cx| {
                                        if qs.preview.current_preview.as_ref() != Some(&preview_key_for_task)
                                            || qs.preview.manager.generation() != preview_generation
                                        {
                                            return;
                                        }
                                        let buffer =
                                            cx.new(|cx| language::Buffer::local(text.clone(), cx));
                                        qs.preview.replace_preview(buffer, cx);
                                        qs.preview.needs_preview_scroll = false;
                                        qs.preview.apply_preview_highlights(cx);
                                        cx.notify();
                                    }) {
                                        debug!("quick_search: failed to show commit diff error: {:?}", err);
                                    }
                                }
                                return;
                            }
                            Err(err) => {
                                let text = format!("Failed to load commit diff:\n{err:?}\n");
                                if let Some(qs) = quick_search.upgrade() {
                                    if let Err(err) = app.update_entity(&qs, |qs, cx| {
                                        if qs.preview.current_preview.as_ref() != Some(&preview_key_for_task)
                                            || qs.preview.manager.generation() != preview_generation
                                        {
                                            return;
                                        }
                                        let buffer =
                                            cx.new(|cx| language::Buffer::local(text.clone(), cx));
                                        qs.preview.replace_preview(buffer, cx);
                                        qs.preview.needs_preview_scroll = false;
                                        qs.preview.apply_preview_highlights(cx);
                                        cx.notify();
                                    }) {
                                        debug!("quick_search: failed to show commit diff error: {:?}", err);
                                    }
                                }
                                return;
                            }
                        };

                        if cancel_flag.load(Ordering::SeqCst) {
                            return;
                        }

                        let mut built: Vec<(Entity<Buffer>, Entity<BufferDiff>)> = Vec::new();
                        for file in commit_diff.files {
                            if cancel_flag.load(Ordering::SeqCst) {
                                return;
                            }

                            let is_deleted = file.new_text.is_none();
                            let new_text = file.new_text.unwrap_or_default();
                            let old_text = file.old_text;

                            let worktree_id = match app.update_entity(&repository, |repo, cx| {
                                repo.repo_path_to_project_path(&file.path, cx)
                                    .map(|p| p.worktree_id)
                                    .or(first_worktree_id)
                            }) {
                                Ok(Some(id)) => id,
                                _ => continue,
                            };

                            let display_name: Arc<str> = Arc::from(
                                file.path
                                    .display(PathStyle::Posix)
                                    .to_string()
                                    .into_boxed_str(),
                            );

                            let file = Arc::new(GitBlob {
                                path: file.path.clone(),
                                worktree_id,
                                is_deleted,
                                display_name,
                            }) as Arc<dyn File>;

                            let buffer = match build_commit_file_buffer(
                                new_text,
                                file,
                                &language_registry,
                                &mut app,
                            )
                            .await
                            {
                                Ok(buffer) => buffer,
                                Err(err) => {
                                    debug!("quick_search: failed to build commit file buffer: {err:?}");
                                    continue;
                                }
                            };

                            let buffer_diff = match build_commit_file_diff(
                                old_text,
                                &buffer,
                                &language_registry,
                                &mut app,
                            )
                            .await
                            {
                                Ok(diff) => diff,
                                Err(err) => {
                                    debug!("quick_search: failed to build commit file diff: {err:?}");
                                    continue;
                                }
                            };

                            built.push((buffer, buffer_diff));
                        }

                        if cancel_flag.load(Ordering::SeqCst) {
                            return;
                        }

                        if let Some(qs) = quick_search.upgrade() {
                            let preview_id = preview_key_for_task.clone();
                            let mut should_apply_selection = false;
                            let update_result = app.update_entity(&qs, |qs, cx| {
                                if qs.preview.current_preview.as_ref() != Some(&preview_key_for_task)
                                    || qs.preview.manager.generation() != preview_generation
                                {
                                    return;
                                }

                                fn find_query_anchor_in_buffer(
                                    snapshot: &language::BufferSnapshot,
                                    query: &str,
                                ) -> Option<TextAnchor> {
                                    let query = query.trim();
                                    if query.is_empty() {
                                        return None;
                                    }

                                    let query_lower = query.to_ascii_lowercase();
                                    for row in 0..=snapshot.text.max_point().row {
                                        let line: String = snapshot
                                            .text
                                            .text_for_range(
                                                Point::new(row, 0)
                                                    ..Point::new(row, snapshot.text.line_len(row)),
                                            )
                                            .collect();
                                        let hay_lower = line.to_ascii_lowercase();
                                        let Some(byte_ix) = hay_lower.find(&query_lower) else {
                                            continue;
                                        };
                                        let col = line[..byte_ix].chars().count() as u32;
                                        return Some(
                                            snapshot.text.anchor_after(Point::new(row, col)),
                                        );
                                    }
                                    None
                                }

                                let mut focus_range: Option<Range<TextAnchor>> = None;
                                if !query_for_commit.trim().is_empty() {
                                    for (buffer, _buffer_diff) in &built {
                                        let snapshot = buffer.read(cx).snapshot();
                                        if let Some(anchor) =
                                            find_query_anchor_in_buffer(&snapshot, &query_for_commit)
                                        {
                                            focus_range = Some(anchor..anchor);
                                            break;
                                        }
                                    }
                                }

                                if focus_range.is_none() {
                                    for (buffer, buffer_diff) in &built {
                                        let snapshot = buffer.read(cx).snapshot();
                                        let first_hunk = buffer_diff
                                            .read(cx)
                                            .hunks(&snapshot.text, cx)
                                            .next();
                                        if let Some(hunk) = first_hunk {
                                            let anchor = hunk.buffer_range.start;
                                            focus_range = Some(anchor..anchor);
                                            break;
                                        }
                                    }
                                }

                                qs.preview.use_diff_preview = true;
                                qs.preview.diff_preview_multi.update(cx, |multibuffer, cx| {
                                    multibuffer.clear(cx);
                                    for (buffer, buffer_diff) in &built {
                                        let snapshot = buffer.read(cx).snapshot();
                                        let Some(path) =
                                            snapshot.file().map(|file| file.path().clone())
                                        else {
                                            continue;
                                        };

                                        let excerpt_ranges =
                                            vec![language::Point::zero()..snapshot.max_point()];

                                        let (_preview_anchors, _new_excerpts) =
                                            multibuffer.set_excerpts_for_path(
                                            PathKey::with_sort_prefix(FILE_NAMESPACE_SORT_PREFIX, path),
                                            buffer.clone(),
                                            excerpt_ranges,
                                            0,
                                            cx,
                                        );
                                        multibuffer.add_diff(buffer_diff.clone(), cx);
                                    }
                                });

                                qs.preview.current_preview_anchors =
                                    focus_range.map(|range| vec![range]);
                                qs.preview.needs_preview_scroll = true;
                                qs.preview.apply_preview_highlights(cx);
                                should_apply_selection = true;
                                cx.notify();
                            });
                            if let Err(err) = update_result {
                                debug!("quick_search: failed to apply commit preview: {:?}", err);
                            } else if should_apply_selection {
                                let quick_search = quick_search.clone();
                                if let Err(err) = app.update_window(window_handle, move |_, window, cx| {
                                    let Some(qs) = quick_search.upgrade() else {
                                        return;
                                    };
                                    qs.update(cx, |qs, cx| {
                                        if qs.preview.current_preview.as_ref() != Some(&preview_id)
                                            || qs.preview.manager.generation() != preview_generation
                                        {
                                            return;
                                        }
                                        qs.preview.apply_preview_selection(window, cx);
                                    });
                                }) {
                                    debug!("quick_search: window update failed: {:?}", err);
                                }
                            }
                        }
                        return;
                    }

                    let buffer_for_preview = match &request_for_task {
                        PreviewRequest::Buffer { buffer, .. } => buffer.clone(),
                        PreviewRequest::ProjectPath { project_path, .. } => {
                                let open_task = match app.update_entity(
                                    &project_for_task,
                                    |project, cx| project.open_buffer(project_path.clone(), cx),
                                ) {
                                    Ok(task) => task,
                                    Err(err) => {
                                        debug!("quick_search: failed to start open_buffer: {:?}", err);
                                        if let Some(qs) = quick_search.upgrade() {
                                            if let Err(update_err) = app.update_entity(&qs, |qs, _cx| {
                                                qs.preview.set_error(format!("Failed to open file: {err}"));
                                            }) {
                                                debug!(
                                                    "quick_search: failed to record preview error: {:?}",
                                                    update_err
                                                );
                                            }
                                        }
                                        return;
                                    }
                                };
                                let buffer = match open_task.await {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        debug!("quick_search: failed to open buffer: {:?}", err);
                                        if let Some(qs) = quick_search.upgrade() {
                                            if let Err(update_err) = app.update_entity(&qs, |qs, _cx| {
                                                qs.preview.set_error(format!("Failed to open file: {err}"));
                                            }) {
                                                debug!(
                                                    "quick_search: failed to record preview error: {:?}",
                                                    update_err
                                                );
                                            }
                                        }
                                        return;
                                    }
                                };
                                buffer
                            }
                        _ => return,
                    };

                    if let Some(qs) = quick_search.upgrade() {
                        let preview_id = preview_key_for_task.clone();
                        let mut should_apply_selection = false;
                        let update_result = app.update_entity(&qs, |qs, cx| {
                            if qs.preview.current_preview.as_ref() != Some(&preview_id)
                                || qs.preview.manager.generation() != preview_generation
                            {
                                return;
                            }
                            let buffer_changed = qs
                                .preview
                                .active_preview_multi()
                                .read(cx)
                                .as_singleton()
                                .map(|b| b != buffer_for_preview)
                                .unwrap_or(true);
                            if buffer_changed {
                                qs.preview.replace_preview(buffer_for_preview.clone(), cx);
                            }
                            qs.preview.clear_error();
                            qs.preview.needs_preview_scroll = true;
                            qs.preview.apply_preview_highlights(cx);
                            should_apply_selection = true;
                            cx.notify();
                        });
                        if let Err(err) = update_result {
                            debug!(
                                "quick_search: quick search dropped before preview applied: {:?}",
                                err
                            );
                        } else if should_apply_selection {
                            let quick_search = quick_search.clone();
                            let preview_id = preview_key_for_task.clone();
                            if let Err(err) = app.update_window(window_handle, move |_, window, cx| {
                                let Some(qs) = quick_search.upgrade() else {
                                    return;
                                };
                                qs.update(cx, |qs, cx| {
                                    if qs.preview.current_preview.as_ref() != Some(&preview_id)
                                        || qs.preview.manager.generation() != preview_generation
                                    {
                                        return;
                                    }
                                    qs.preview.apply_preview_selection(window, cx);
                                });
                            }) {
                                debug!("quick_search: window update failed: {:?}", err);
                            }
                        }
                    }
                }
            })
            });
    }

    fn apply_preview_highlights(&mut self, cx: &mut Context<QuickSearch>) {
        let strong = self.current_preview_anchors.clone().unwrap_or_default();
        let weak = self
            .current_weak_preview_anchors
            .clone()
            .unwrap_or_default();
        let use_diff_preview = self.use_diff_preview;

        self.active_preview_editor().update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().read(cx);
            let snapshot = multi_buffer.snapshot(cx);
            let excerpt_buffers: std::collections::HashMap<
                multi_buffer::ExcerptId,
                &language::BufferSnapshot,
            > = snapshot
                .excerpts()
                .map(|(excerpt_id, buffer, _)| (excerpt_id, buffer))
                .collect();

            let mut excerpt_ids_by_buffer =
                std::collections::HashMap::<BufferId, multi_buffer::ExcerptId>::new();
            let mut fallback_excerpt_id: Option<multi_buffer::ExcerptId> = None;
            if use_diff_preview {
                for (excerpt_id, buffer, _range) in snapshot.excerpts() {
                    fallback_excerpt_id.get_or_insert(excerpt_id);
                    excerpt_ids_by_buffer
                        .entry(buffer.remote_id())
                        .or_insert(excerpt_id);
                }
            } else {
                fallback_excerpt_id = snapshot.excerpts().next().map(|(id, _, _)| id);
            }

            let Some(fallback_excerpt_id) = fallback_excerpt_id else {
                editor.highlight_background::<QuickSearchPreviewWeakHighlights>(
                    &[],
                    |_, theme| theme.colors().search_match_background.opacity(0.35),
                    cx,
                );
                editor.highlight_background::<QuickSearchPreviewStrongHighlights>(
                    &[],
                    |_, theme| theme.colors().search_active_match_background,
                    cx,
                );
                return;
            };

            let convert_range = |range: &Range<TextAnchor>| {
                let excerpt_id = if use_diff_preview {
                    range
                        .start
                        .buffer_id
                        .and_then(|buffer_id| excerpt_ids_by_buffer.get(&buffer_id).copied())
                        .unwrap_or(fallback_excerpt_id)
                } else {
                    fallback_excerpt_id
                };
                let converted = MultiBufferAnchor::range_in_buffer(excerpt_id, range.clone());
                is_safe_anchor_range(&converted, &excerpt_buffers).then_some(converted)
            };

            let weak_ranges: Vec<_> = weak.iter().filter_map(convert_range).collect();
            let strong_ranges: Vec<_> = strong.iter().filter_map(convert_range).collect();

            editor.highlight_background::<QuickSearchPreviewWeakHighlights>(
                &weak_ranges,
                |_, theme| theme.colors().search_match_background.opacity(0.35),
                cx,
            );
            editor.highlight_background::<QuickSearchPreviewStrongHighlights>(
                &strong_ranges,
                |_, theme| theme.colors().search_active_match_background,
                cx,
            );
        });
    }

    pub fn apply_preview_selection(&mut self, window: &mut Window, cx: &mut Context<QuickSearch>) {
        if !self.needs_preview_scroll {
            return;
        }
        if self.use_diff_preview {
            let strong = self.current_preview_anchors.clone().unwrap_or_default();
            self.active_preview_editor().update(cx, |editor, cx| {
                let multi_buffer = editor.buffer().read(cx);
                let snapshot = multi_buffer.snapshot(cx);
                let mut excerpt_ids_by_buffer =
                    std::collections::HashMap::<BufferId, multi_buffer::ExcerptId>::new();
                let mut fallback_excerpt_id: Option<multi_buffer::ExcerptId> = None;
                for (excerpt_id, buffer, _range) in snapshot.excerpts() {
                    fallback_excerpt_id.get_or_insert(excerpt_id);
                    excerpt_ids_by_buffer
                        .entry(buffer.remote_id())
                        .or_insert(excerpt_id);
                }
                let Some(fallback_excerpt_id) = fallback_excerpt_id else {
                    return;
                };

                let excerpt_buffers: std::collections::HashMap<
                    multi_buffer::ExcerptId,
                    &language::BufferSnapshot,
                > = snapshot
                    .excerpts()
                    .map(|(excerpt_id, buffer, _)| (excerpt_id, buffer))
                    .collect();

                let mut anchor_ranges: Vec<Range<MultiBufferAnchor>> = strong
                    .iter()
                    .filter_map(|range| {
                        let excerpt_id = range
                            .start
                            .buffer_id
                            .and_then(|buffer_id| excerpt_ids_by_buffer.get(&buffer_id).copied())
                            .unwrap_or(fallback_excerpt_id);
                        let converted =
                            MultiBufferAnchor::range_in_buffer(excerpt_id, range.clone());
                        is_safe_anchor_range(&converted, &excerpt_buffers).then_some(converted)
                    })
                    .collect();
                if anchor_ranges.is_empty() {
                    anchor_ranges.push(MultiBufferAnchor::min()..MultiBufferAnchor::min());
                }

                let effects = SelectionEffects::scroll(Autoscroll::fit());
                editor.change_selections(effects, window, cx, move |selections| {
                    selections.clear_disjoint();
                    selections.select_anchor_ranges(anchor_ranges);
                });
            });
        } else {
            let strong = self.current_preview_anchors.clone().unwrap_or_default();
            self.active_preview_editor().update(cx, |editor, cx| {
                let multi_buffer = editor.buffer().read(cx);
                let snapshot = multi_buffer.snapshot(cx);
                let Some((excerpt_id, _buffer, _range)) = snapshot.excerpts().next() else {
                    return;
                };

                let excerpt_buffers: std::collections::HashMap<
                    multi_buffer::ExcerptId,
                    &language::BufferSnapshot,
                > = snapshot
                    .excerpts()
                    .map(|(excerpt_id, buffer, _)| (excerpt_id, buffer))
                    .collect();

                let mut anchor_ranges: Vec<Range<MultiBufferAnchor>> = strong
                    .iter()
                    .filter_map(|range| {
                        let converted =
                            MultiBufferAnchor::range_in_buffer(excerpt_id, range.clone());
                        is_safe_anchor_range(&converted, &excerpt_buffers).then_some(converted)
                    })
                    .collect();
                if anchor_ranges.is_empty() {
                    anchor_ranges.push(MultiBufferAnchor::min()..MultiBufferAnchor::min());
                }

                let effects = SelectionEffects::scroll(Autoscroll::fit());
                editor.change_selections(effects, window, cx, move |selections| {
                    selections.clear_disjoint();
                    selections.select_anchor_ranges(anchor_ranges);
                });
            });
        }
        self.needs_preview_scroll = false;
    }
}

fn is_safe_anchor_range(
    range: &Range<MultiBufferAnchor>,
    excerpt_buffers: &std::collections::HashMap<multi_buffer::ExcerptId, &language::BufferSnapshot>,
) -> bool {
    is_safe_anchor(&range.start, excerpt_buffers) && is_safe_anchor(&range.end, excerpt_buffers)
}

fn is_safe_anchor(
    anchor: &MultiBufferAnchor,
    excerpt_buffers: &std::collections::HashMap<multi_buffer::ExcerptId, &language::BufferSnapshot>,
) -> bool {
    if anchor.is_min() || anchor.is_max() {
        return true;
    }

    let Some(buffer) = excerpt_buffers.get(&anchor.excerpt_id) else {
        return false;
    };
    if !buffer.can_resolve(&anchor.text_anchor) {
        return false;
    }

    let point = anchor.text_anchor.to_point(buffer);
    let max_point = buffer.text.max_point();
    if point.row > max_point.row {
        return false;
    }
    let max_col = buffer.text.line_len(point.row);
    point.column <= max_col
}

fn build_preview_editor(
    buffer: Entity<language::Buffer>,
    project: Entity<Project>,
    include_commit_addon: bool,
    window: &mut Window,
    cx: &mut Context<QuickSearch>,
) -> (Entity<MultiBuffer>, Entity<Editor>) {
    let buffer_id = buffer.read(cx).remote_id();
    let preview_multi = cx.new(|cx| {
        let mut multi = if include_commit_addon {
            MultiBuffer::new(Capability::ReadOnly)
        } else {
            MultiBuffer::without_headers(Capability::ReadOnly)
        };
        multi.push_excerpts(
            buffer,
            [ExcerptRange::new(TextAnchor::min_max_range_for_buffer(
                buffer_id,
            ))],
            cx,
        );
        multi
    });
    let preview_editor = cx.new(|cx| {
        let mut editor = Editor::new(
            EditorMode::Full {
                scale_ui_elements_with_buffer_font_size: true,
                show_active_line_background: true,

                sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
            },
            preview_multi.clone(),
            Some(project.clone()),
            window,
            cx,
        );
        editor.set_read_only(true);
        editor.set_searchable(false);
        editor.set_in_project_search(true);
        if include_commit_addon {
            editor.set_expand_all_diff_hunks(cx);
            editor.register_addon(CommitPreviewAddon {
                multibuffer: preview_multi.downgrade(),
            });
        }
        editor.set_show_line_numbers(true, cx);
        editor.set_show_wrap_guides(false, cx);
        editor.set_show_runnables(false, cx);
        editor.set_show_breakpoints(false, cx);
        editor.set_show_horizontal_scrollbar(false, cx);
        editor.set_show_gutter(true, cx);
        editor.set_show_scrollbars(true, cx);
        editor.disable_expand_excerpt_buttons(cx);
        if !include_commit_addon {
            editor.disable_header_for_buffer(buffer_id, cx);
        }
        editor
    });
    (preview_multi, preview_editor)
}

pub(super) const FILE_NAMESPACE_SORT_PREFIX: u64 = 1;

pub(super) struct CommitPreviewAddon {
    pub(super) multibuffer: WeakEntity<MultiBuffer>,
}

impl Addon for CommitPreviewAddon {
    fn render_buffer_header_controls(
        &self,
        excerpt: &multi_buffer::ExcerptInfo,
        _window: &Window,
        cx: &gpui::App,
    ) -> Option<gpui::AnyElement> {
        let multibuffer = self.multibuffer.upgrade()?;
        let snapshot = multibuffer.read(cx).snapshot(cx);
        let excerpts = snapshot.excerpts().collect::<Vec<_>>();
        let current_idx = excerpts.iter().position(|(id, _, _)| *id == excerpt.id)?;
        let (_, _, current_range) = &excerpts[current_idx];

        let start_row = current_range.context.start.to_point(&excerpt.buffer).row;

        let prev_end_row = if current_idx > 0 {
            let (_, prev_buffer, prev_range) = &excerpts[current_idx - 1];
            if prev_buffer.remote_id() == excerpt.buffer_id {
                prev_range.context.end.to_point(&excerpt.buffer).row
            } else {
                0
            }
        } else {
            0
        };

        let skipped_lines = start_row.saturating_sub(prev_end_row);
        if skipped_lines > 0 {
            Some(
                ui::Label::new(format!("{skipped_lines} unchanged lines"))
                    .color(ui::Color::Muted)
                    .size(ui::LabelSize::Small)
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub(super) struct GitBlob {
    pub(super) path: git::repository::RepoPath,
    pub(super) worktree_id: WorktreeId,
    pub(super) is_deleted: bool,
    pub(super) display_name: Arc<str>,
}

impl File for GitBlob {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        if self.is_deleted {
            DiskState::Deleted
        } else {
            DiskState::New
        }
    }

    fn path_style(&self, _: &gpui::App) -> PathStyle {
        PathStyle::Posix
    }

    fn path(&self) -> &Arc<RelPath> {
        self.path.as_ref()
    }

    fn full_path(&self, _: &gpui::App) -> std::path::PathBuf {
        self.path.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a gpui::App) -> &'a str {
        self.display_name.as_ref()
    }

    fn worktree_id(&self, _: &gpui::App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _cx: &gpui::App) -> language::proto::File {
        language::proto::File {
            worktree_id: self.worktree_id.to_proto(),
            entry_id: None,
            path: self.path.as_ref().as_unix_str().to_string(),
            mtime: None,
            is_deleted: self.is_deleted,
        }
    }

    fn is_private(&self) -> bool {
        false
    }
}

pub(super) async fn build_commit_file_buffer(
    mut text: String,
    file: Arc<dyn File>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut gpui::AsyncApp,
) -> Result<Entity<Buffer>> {
    let line_ending = LineEnding::detect(&text);
    LineEnding::normalize(&mut text);
    let text = Rope::from(text);

    let language = cx.update(|cx| language_registry.language_for_file(&file, Some(&text), cx))?;
    let language = if let Some(language) = language {
        language_registry
            .load_language(&language)
            .await
            .ok()
            .and_then(|e| e.log_err())
    } else {
        None
    };

    let buffer = cx
        .new(|cx| {
            let buffer = TextBuffer::new_normalized(
                ReplicaId::LOCAL,
                cx.entity_id().as_non_zero_u64().into(),
                line_ending,
                text,
            );
            let mut buffer = Buffer::build(buffer, Some(file), Capability::ReadWrite);
            buffer.set_language_async(language, cx);
            buffer
        })
        .context("creating commit preview buffer")?;

    Ok(buffer)
}

pub(super) async fn build_commit_file_diff(
    mut old_text: Option<String>,
    buffer: &Entity<Buffer>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut gpui::AsyncApp,
) -> Result<Entity<BufferDiff>> {
    if let Some(old_text) = &mut old_text {
        LineEnding::normalize(old_text);
    }

    let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot())?;

    let base_buffer = cx
        .update(|cx| {
            Buffer::build_snapshot(
                old_text.as_deref().unwrap_or("").into(),
                buffer_snapshot.language().cloned(),
                Some(language_registry.clone()),
                cx,
            )
        })?
        .await;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                buffer_snapshot.text.clone(),
                old_text.map(Arc::new),
                base_buffer,
                cx,
            )
        })?
        .await;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer_snapshot.text, cx);
        diff.set_snapshot(diff_snapshot, &buffer_snapshot.text, cx);
        diff
    })
    .context("creating commit preview diff")
}
