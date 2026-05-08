pub mod blame;

use super::*;

impl Editor {
    pub fn diff_hunks_in_ranges<'a>(
        &'a self,
        ranges: &'a [Range<Anchor>],
        buffer: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = MultiBufferDiffHunk> {
        ranges.iter().flat_map(move |range| {
            let end_excerpt = buffer.excerpt_containing(range.end..range.end);
            let range = range.to_point(buffer);
            let mut peek_end = range.end;
            if range.end.row < buffer.max_row().0 {
                peek_end = Point::new(range.end.row + 1, 0);
            }
            buffer
                .diff_hunks_in_range(range.start..peek_end)
                .filter(move |hunk| {
                    if let Some((_, excerpt_range)) = &end_excerpt
                        && let Some(end_anchor) =
                            buffer.anchor_in_excerpt(excerpt_range.context.end)
                        && let Some(hunk_end_anchor) =
                            buffer.anchor_in_excerpt(hunk.excerpt_range.context.end)
                        && hunk_end_anchor.cmp(&end_anchor, buffer).is_gt()
                    {
                        false
                    } else {
                        true
                    }
                })
        })
    }

    pub fn set_render_diff_hunk_controls(
        &mut self,
        render_diff_hunk_controls: RenderDiffHunkControlsFn,
        cx: &mut Context<Self>,
    ) {
        self.render_diff_hunk_controls = render_diff_hunk_controls;
        cx.notify();
    }

    pub fn working_directory(&self, cx: &App) -> Option<PathBuf> {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local())
                && let Some(dir) = file.abs_path(cx).parent()
            {
                return Some(dir.to_owned());
            }
        }

        None
    }

    pub fn target_file_abs_path(&self, cx: &mut Context<Self>) -> Option<PathBuf> {
        self.active_buffer(cx).and_then(|buffer| {
            let buffer = buffer.read(cx);
            if let Some(project_path) = buffer.project_path(cx) {
                let project = self.project()?.read(cx);
                project.absolute_path(&project_path, cx)
            } else {
                buffer
                    .file()
                    .and_then(|file| file.as_local().map(|file| file.abs_path(cx)))
            }
        })
    }

    /// Returns the project path for the editor's buffer, if any buffer is
    /// opened in the editor.
    pub fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        if let Some(buffer) = self.buffer.read(cx).as_singleton() {
            buffer.read(cx).project_path(cx)
        } else {
            None
        }
    }

    pub fn git_blame_inline_enabled(&self) -> bool {
        self.git_blame_inline_enabled
    }

    pub fn selection_menu_enabled(&self, cx: &App) -> bool {
        self.show_selection_menu
            .unwrap_or_else(|| EditorSettings::get_global(cx).toolbar.selections_menu)
    }

    pub fn toggle_selection_menu(
        &mut self,
        _: &ToggleSelectionMenu,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_selection_menu = self
            .show_selection_menu
            .map(|show_selections_menu| !show_selections_menu)
            .or_else(|| Some(!EditorSettings::get_global(cx).toolbar.selections_menu));

        cx.notify();
    }

    pub fn blame(&self) -> Option<&Entity<GitBlame>> {
        self.blame.as_ref()
    }

    pub fn show_git_blame_gutter(&self) -> bool {
        self.show_git_blame_gutter
    }

    pub fn expand_selected_diff_hunks(&mut self, cx: &mut Context<Self>) {
        let ranges: Vec<_> = self
            .selections
            .disjoint_anchors()
            .iter()
            .map(|s| s.range())
            .collect();
        self.buffer
            .update(cx, |buffer, cx| buffer.expand_diff_hunks(ranges, cx))
    }

    pub fn copy_file_name_without_extension(
        &mut self,
        _: &CopyFileNameWithoutExtension,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(file_stem) = self.active_buffer(cx).and_then(|buffer| {
            let file = buffer.read(cx).file()?;
            file.path().file_stem()
        }) {
            cx.write_to_clipboard(ClipboardItem::new_string(file_stem.to_string()));
        }
    }

    pub fn copy_file_name(&mut self, _: &CopyFileName, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(file_name) = self.active_buffer(cx).and_then(|buffer| {
            let file = buffer.read(cx).file()?;
            Some(file.file_name(cx))
        }) {
            cx.write_to_clipboard(ClipboardItem::new_string(file_name.to_string()));
        }
    }

    pub fn toggle_git_blame(
        &mut self,
        _: &::git::Blame,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_git_blame_gutter = !self.show_git_blame_gutter;

        if self.show_git_blame_gutter && !self.has_blame_entries(cx) {
            self.start_git_blame(true, window, cx);
        }

        cx.notify();
    }

    pub fn toggle_git_blame_inline(
        &mut self,
        _: &ToggleGitBlameInline,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_git_blame_inline_internal(true, window, cx);
        cx.notify();
    }

    pub(super) fn toggle_staged_selected_diff_hunks(
        &mut self,
        _: &::git::ToggleStaged,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let ranges: Vec<_> = self
            .selections
            .disjoint_anchors()
            .iter()
            .map(|s| s.range())
            .collect();
        let stage = self.has_stageable_diff_hunks_in_ranges(&ranges, &snapshot);
        self.stage_or_unstage_diff_hunks(stage, ranges, cx);
    }

    pub(super) fn stage_and_next(
        &mut self,
        _: &::git::StageAndNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_stage_or_unstage_and_next(true, window, cx);
    }

    pub(super) fn unstage_and_next(
        &mut self,
        _: &::git::UnstageAndNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_stage_or_unstage_and_next(false, window, cx);
    }

    pub(super) fn stage_or_unstage_diff_hunks(
        &mut self,
        stage: bool,
        ranges: Vec<Range<Anchor>>,
        cx: &mut Context<Self>,
    ) {
        if self.delegate_stage_and_restore {
            let snapshot = self.buffer.read(cx).snapshot(cx);
            let hunks: Vec<_> = self.diff_hunks_in_ranges(&ranges, &snapshot).collect();
            if !hunks.is_empty() {
                cx.emit(EditorEvent::StageOrUnstageRequested { stage, hunks });
            }
            return;
        }
        let task = self.save_buffers_for_ranges_if_needed(&ranges, cx);
        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |this, cx| {
                let snapshot = this.buffer.read(cx).snapshot(cx);
                let chunk_by = this
                    .diff_hunks_in_ranges(&ranges, &snapshot)
                    .chunk_by(|hunk| hunk.buffer_id);
                for (buffer_id, hunks) in &chunk_by {
                    this.do_stage_or_unstage(stage, buffer_id, hunks, cx);
                }
            })
        })
        .detach_and_log_err(cx);
    }

    pub(super) fn do_stage_or_unstage(
        &self,
        stage: bool,
        buffer_id: BufferId,
        hunks: impl Iterator<Item = MultiBufferDiffHunk>,
        cx: &mut App,
    ) -> Option<()> {
        let project = self.project()?;
        let buffer = project.read(cx).buffer_for_id(buffer_id, cx)?;
        let diff = self.buffer.read(cx).diff_for(buffer_id)?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let file_exists = buffer_snapshot
            .file()
            .is_some_and(|file| file.disk_state().exists());
        diff.update(cx, |diff, cx| {
            diff.stage_or_unstage_hunks(
                stage,
                &hunks
                    .map(|hunk| buffer_diff::DiffHunk {
                        buffer_range: hunk.buffer_range,
                        // We don't need to pass in word diffs here because they're only used for rendering and
                        // this function changes internal state
                        base_word_diffs: Vec::default(),
                        buffer_word_diffs: Vec::default(),
                        diff_base_byte_range: hunk.diff_base_byte_range.start.0
                            ..hunk.diff_base_byte_range.end.0,
                        secondary_status: hunk.status.secondary,
                        range: Point::zero()..Point::zero(), // unused
                    })
                    .collect::<Vec<_>>(),
                &buffer_snapshot,
                file_exists,
                cx,
            )
        });
        None
    }

    pub(super) fn clear_expanded_diff_hunks(&mut self, cx: &mut Context<Self>) -> bool {
        self.buffer.update(cx, |buffer, cx| {
            let ranges = vec![Anchor::Min..Anchor::Max];
            if !buffer.all_diff_hunks_expanded()
                && buffer.has_expanded_diff_hunks_in_ranges(&ranges, cx)
            {
                buffer.collapse_diff_hunks(ranges, cx);
                true
            } else {
                false
            }
        })
    }

    pub(super) fn has_any_expanded_diff_hunks(&self, cx: &App) -> bool {
        if self.buffer.read(cx).all_diff_hunks_expanded() {
            return true;
        }
        let ranges = vec![Anchor::Min..Anchor::Max];
        self.buffer
            .read(cx)
            .has_expanded_diff_hunks_in_ranges(&ranges, cx)
    }

    pub(super) fn toggle_diff_hunks_in_ranges(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        cx: &mut Context<Editor>,
    ) {
        self.buffer.update(cx, |buffer, cx| {
            let expand = !buffer.has_expanded_diff_hunks_in_ranges(&ranges, cx);
            buffer.expand_or_collapse_diff_hunks(ranges, expand, cx);
        })
    }

    pub(super) fn toggle_single_diff_hunk(&mut self, range: Range<Anchor>, cx: &mut Context<Self>) {
        self.buffer.update(cx, |buffer, cx| {
            buffer.toggle_single_diff_hunk(range, cx);
        })
    }

    pub(super) fn apply_all_diff_hunks(
        &mut self,
        _: &ApplyAllDiffHunks,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        let buffers = self.buffer.read(cx).all_buffers();
        for branch_buffer in buffers {
            branch_buffer.update(cx, |branch_buffer, cx| {
                branch_buffer.merge_into_base(Vec::new(), cx);
            });
        }

        if let Some(project) = self.project.clone() {
            self.save(
                SaveOptions {
                    format: true,
                    force_format: false,
                    autosave: false,
                },
                project,
                window,
                cx,
            )
            .detach_and_log_err(cx);
        }
    }

    pub(super) fn apply_selected_diff_hunks(
        &mut self,
        _: &ApplyDiffHunk,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        let snapshot = self.snapshot(window, cx);
        let hunks = snapshot.hunks_for_ranges(
            self.selections
                .all(&snapshot.display_snapshot)
                .into_iter()
                .map(|selection| selection.range()),
        );
        let mut ranges_by_buffer = HashMap::default();
        self.transact(window, cx, |editor, _window, cx| {
            for hunk in hunks {
                if let Some(buffer) = editor.buffer.read(cx).buffer(hunk.buffer_id) {
                    ranges_by_buffer
                        .entry(buffer.clone())
                        .or_insert_with(Vec::new)
                        .push(hunk.buffer_range.to_offset(buffer.read(cx)));
                }
            }

            for (buffer, ranges) in ranges_by_buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.merge_into_base(ranges, cx);
                });
            }
        });

        if let Some(project) = self.project.clone() {
            self.save(
                SaveOptions {
                    format: true,
                    force_format: false,
                    autosave: false,
                },
                project,
                window,
                cx,
            )
            .detach_and_log_err(cx);
        }
    }

    pub(super) fn target_file<'a>(&self, cx: &'a App) -> Option<&'a dyn language::LocalFile> {
        self.active_buffer(cx)?
            .read(cx)
            .file()
            .and_then(|f| f.as_local())
    }

    pub(super) fn reveal_in_finder(
        &mut self,
        _: &RevealInFileManager,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.target_file_abs_path(cx) {
            if let Some(project) = self.project() {
                project.update(cx, |project, cx| project.reveal_path(&path, cx));
            } else {
                cx.reveal_path(&path);
            }
        }
    }

    pub(super) fn copy_path(
        &mut self,
        _: &zed_actions::workspace::CopyPath,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.target_file_abs_path(cx)
            && let Some(path) = path.to_str()
        {
            cx.write_to_clipboard(ClipboardItem::new_string(path.to_string()));
        } else {
            cx.propagate();
        }
    }

    pub(super) fn copy_relative_path(
        &mut self,
        _: &zed_actions::workspace::CopyRelativePath,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.active_buffer(cx).and_then(|buffer| {
            let project = self.project()?.read(cx);
            let path = buffer.read(cx).file()?.path();
            let path = path.display(project.path_style(cx));
            Some(path)
        }) {
            cx.write_to_clipboard(ClipboardItem::new_string(path.to_string()));
        } else {
            cx.propagate();
        }
    }

    pub(super) fn go_to_active_debug_line(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        maybe!({
            let breakpoint_store = self.breakpoint_store.as_ref()?;

            let (active_stack_frame, debug_line_pane_id) = {
                let store = breakpoint_store.read(cx);
                let active_stack_frame = store.active_position().cloned();
                let debug_line_pane_id = store.active_debug_line_pane_id();
                (active_stack_frame, debug_line_pane_id)
            };

            let Some(active_stack_frame) = active_stack_frame else {
                self.clear_row_highlights::<ActiveDebugLine>();
                return None;
            };

            if let Some(debug_line_pane_id) = debug_line_pane_id {
                if let Some(workspace) = self
                    .workspace
                    .as_ref()
                    .and_then(|(workspace, _)| workspace.upgrade())
                {
                    let editor_pane_id = workspace
                        .read(cx)
                        .pane_for_item_id(cx.entity_id())
                        .map(|pane| pane.entity_id());

                    if editor_pane_id.is_some_and(|id| id != debug_line_pane_id) {
                        self.clear_row_highlights::<ActiveDebugLine>();
                        return None;
                    }
                }
            }

            let position = active_stack_frame.position;

            let snapshot = self.buffer.read(cx).snapshot(cx);
            let multibuffer_anchor = snapshot.anchor_in_excerpt(position)?;

            self.clear_row_highlights::<ActiveDebugLine>();

            self.go_to_line::<ActiveDebugLine>(
                multibuffer_anchor,
                Some(cx.theme().colors().editor_debugger_active_line_background),
                window,
                cx,
            );

            cx.notify();

            Some(())
        })
        .is_some()
    }

    pub(super) fn open_git_blame_commit(
        &mut self,
        _: &OpenGitBlameCommit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_git_blame_commit_internal(window, cx);
    }

    pub(super) fn start_git_blame(
        &mut self,
        user_triggered: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(project) = self.project() {
            if let Some(buffer) = self.buffer().read(cx).as_singleton()
                && buffer.read(cx).file().is_none()
            {
                return;
            }

            let focused = self.focus_handle(cx).contains_focused(window, cx);

            let project = project.clone();
            let blame = cx
                .new(|cx| GitBlame::new(self.buffer.clone(), project, user_triggered, focused, cx));
            self.blame_subscription =
                Some(cx.observe_in(&blame, window, |_, _, _, cx| cx.notify()));
            self.blame = Some(blame);
        }
    }

    pub(super) fn toggle_git_blame_inline_internal(
        &mut self,
        user_triggered: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.git_blame_inline_enabled {
            self.git_blame_inline_enabled = false;
            self.show_git_blame_inline = false;
            self.show_git_blame_inline_delay_task.take();
        } else {
            self.git_blame_inline_enabled = true;
            self.start_git_blame_inline(user_triggered, window, cx);
        }

        cx.notify();
    }

    pub(super) fn start_git_blame_inline(
        &mut self,
        user_triggered: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_git_blame(user_triggered, window, cx);

        if ProjectSettings::get_global(cx)
            .git
            .inline_blame_delay()
            .is_some()
        {
            self.start_inline_blame_timer(window, cx);
        } else {
            self.show_git_blame_inline = true
        }
    }

    pub(super) fn render_git_blame_gutter(&self, cx: &App) -> bool {
        !self.mode().is_minimap() && self.show_git_blame_gutter && self.has_blame_entries(cx)
    }

    pub(super) fn render_git_blame_inline(&self, window: &Window, cx: &App) -> bool {
        self.show_git_blame_inline
            && (self.focus_handle.is_focused(window) || self.inline_blame_popover.is_some())
            && !self.newest_selection_head_on_empty_line(cx)
            && self.has_blame_entries(cx)
    }

    fn has_stageable_diff_hunks_in_ranges(
        &self,
        ranges: &[Range<Anchor>],
        snapshot: &MultiBufferSnapshot,
    ) -> bool {
        let mut hunks = self.diff_hunks_in_ranges(ranges, snapshot);
        hunks.any(|hunk| hunk.status().has_secondary_hunk())
    }

    fn save_buffers_for_ranges_if_needed(
        &mut self,
        ranges: &[Range<Anchor>],
        cx: &mut Context<Editor>,
    ) -> Task<Result<()>> {
        let multibuffer = self.buffer.read(cx);
        let snapshot = multibuffer.read(cx);
        let buffer_ids: HashSet<_> = ranges
            .iter()
            .flat_map(|range| snapshot.buffer_ids_for_range(range.clone()))
            .collect();
        drop(snapshot);

        let mut buffers = HashSet::default();
        for buffer_id in buffer_ids {
            if let Some(buffer_entity) = multibuffer.buffer(buffer_id) {
                let buffer = buffer_entity.read(cx);
                if buffer.file().is_some_and(|file| file.disk_state().exists()) && buffer.is_dirty()
                {
                    buffers.insert(buffer_entity);
                }
            }
        }

        if let Some(project) = &self.project {
            project.update(cx, |project, cx| project.save_buffers(buffers, cx))
        } else {
            Task::ready(Ok(()))
        }
    }

    fn do_stage_or_unstage_and_next(
        &mut self,
        stage: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ranges = self.selections.disjoint_anchor_ranges().collect::<Vec<_>>();

        if ranges.iter().any(|range| range.start != range.end) {
            self.stage_or_unstage_diff_hunks(stage, ranges, cx);
            return;
        }

        self.stage_or_unstage_diff_hunks(stage, ranges, cx);

        let all_diff_hunks_expanded = self.buffer().read(cx).all_diff_hunks_expanded();
        let wrap_around = !all_diff_hunks_expanded;
        let snapshot = self.snapshot(window, cx);
        let position = self
            .selections
            .newest::<Point>(&snapshot.display_snapshot)
            .head();

        self.go_to_hunk_before_or_after_position(
            &snapshot,
            position,
            Direction::Next,
            wrap_around,
            window,
            cx,
        );
    }

    fn open_git_blame_commit_internal(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let blame = self.blame.as_ref()?;
        let snapshot = self.snapshot(window, cx);
        let cursor = self
            .selections
            .newest::<Point>(&snapshot.display_snapshot)
            .head();
        let (buffer, point) = snapshot.buffer_snapshot().point_to_buffer_point(cursor)?;
        let (_, blame_entry) = blame
            .update(cx, |blame, cx| {
                blame
                    .blame_for_rows(
                        &[RowInfo {
                            buffer_id: Some(buffer.remote_id()),
                            buffer_row: Some(point.row),
                            ..Default::default()
                        }],
                        cx,
                    )
                    .next()
            })
            .flatten()?;
        let renderer = cx.global::<GlobalBlameRenderer>().0.clone();
        let repo = blame.read(cx).repository(cx, buffer.remote_id())?;
        let workspace = self.workspace()?.downgrade();
        renderer.open_blame_commit(blame_entry, repo, workspace, window, cx);
        None
    }

    fn has_blame_entries(&self, cx: &App) -> bool {
        self.blame()
            .is_some_and(|blame| blame.read(cx).has_generated_entries())
    }

    fn newest_selection_head_on_empty_line(&self, cx: &App) -> bool {
        let cursor_anchor = self.selections.newest_anchor().head();

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let buffer_row = MultiBufferRow(cursor_anchor.to_point(&snapshot).row);

        snapshot.line_len(buffer_row) == 0
    }
}
