use anyhow::Result;
use collections::{HashMap, HashSet};
use editor::{
    context_header_renderer, diagnostic_block_renderer, diagnostic_header_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties},
    BuildSettings, Editor, ExcerptId, ExcerptProperties, MultiBuffer,
};
use gpui::{
    action, elements::*, keymap::Binding, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, Task, View, ViewContext, ViewHandle,
};
use language::{Bias, Buffer, Diagnostic, DiagnosticEntry, Point};
use postage::watch;
use project::{Project, ProjectPath, WorktreeId};
use std::{cmp::Ordering, ops::Range, path::Path, sync::Arc};
use util::TryFutureExt;
use workspace::Workspace;

action!(Toggle);
action!(ClearInvalid);

const CONTEXT_LINE_COUNT: u32 = 1;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("alt-shift-D", Toggle, None),
        Binding::new(
            "alt-shift-C",
            ClearInvalid,
            Some("ProjectDiagnosticsEditor"),
        ),
    ]);
    cx.add_action(ProjectDiagnosticsEditor::toggle);
    cx.add_action(ProjectDiagnosticsEditor::clear_invalid);
}

type Event = editor::Event;

struct ProjectDiagnostics {
    project: ModelHandle<Project>,
}

struct ProjectDiagnosticsEditor {
    project: ModelHandle<Project>,
    editor: ViewHandle<Editor>,
    excerpts: ModelHandle<MultiBuffer>,
    path_states: Vec<(Arc<Path>, Vec<DiagnosticGroupState>)>,
    paths_to_update: HashMap<WorktreeId, HashSet<ProjectPath>>,
    build_settings: BuildSettings,
}

struct DiagnosticGroupState {
    primary_diagnostic: DiagnosticEntry<language::Anchor>,
    excerpts: Vec<ExcerptId>,
    blocks: HashMap<BlockId, DiagnosticBlock>,
    block_count: usize,
    is_valid: bool,
}

enum DiagnosticBlock {
    Header(Diagnostic),
    Inline(Diagnostic),
    Context,
}

impl ProjectDiagnostics {
    fn new(project: ModelHandle<Project>) -> Self {
        Self { project }
    }
}

impl Entity for ProjectDiagnostics {
    type Event = ();
}

impl Entity for ProjectDiagnosticsEditor {
    type Event = Event;
}

impl View for ProjectDiagnosticsEditor {
    fn ui_name() -> &'static str {
        "ProjectDiagnosticsEditor"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.editor.id()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.editor);
    }
}

impl ProjectDiagnosticsEditor {
    fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<workspace::Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&project, |this, _, event, cx| match event {
            project::Event::DiskBasedDiagnosticsUpdated { worktree_id } => {
                if let Some(paths) = this.paths_to_update.remove(&worktree_id) {
                    this.update_excerpts(paths, cx);
                }
            }
            project::Event::DiagnosticsUpdated(path) => {
                this.paths_to_update
                    .entry(path.worktree_id)
                    .or_default()
                    .insert(path.clone());
            }
            _ => {}
        })
        .detach();

        let excerpts = cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id()));
        let build_settings = editor::settings_builder(excerpts.downgrade(), settings.clone());
        let editor =
            cx.add_view(|cx| Editor::for_buffer(excerpts.clone(), build_settings.clone(), cx));
        cx.subscribe(&editor, |_, _, event, cx| cx.emit(*event))
            .detach();

        let paths_to_update = project
            .read(cx)
            .diagnostic_summaries(cx)
            .map(|e| e.0)
            .collect();
        let this = Self {
            project,
            excerpts,
            editor,
            build_settings,
            path_states: Default::default(),
            paths_to_update: Default::default(),
        };
        this.update_excerpts(paths_to_update, cx);
        this
    }

    #[cfg(test)]
    fn text(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let diagnostics = cx.add_model(|_| ProjectDiagnostics::new(workspace.project().clone()));
        workspace.add_item(diagnostics, cx);
    }

    fn clear_invalid(&mut self, _: &ClearInvalid, cx: &mut ViewContext<Self>) {
        let mut blocks_to_delete = HashSet::default();
        let mut excerpts_to_delete = Vec::new();
        let mut path_ixs_to_delete = Vec::new();
        for (ix, (_, groups)) in self.path_states.iter_mut().enumerate() {
            groups.retain(|group| {
                if group.is_valid {
                    true
                } else {
                    blocks_to_delete.extend(group.blocks.keys().copied());
                    excerpts_to_delete.extend(group.excerpts.iter().cloned());
                    false
                }
            });

            if groups.is_empty() {
                path_ixs_to_delete.push(ix);
            }
        }

        for ix in path_ixs_to_delete.into_iter().rev() {
            self.path_states.remove(ix);
        }

        self.excerpts.update(cx, |excerpts, cx| {
            excerpts_to_delete.sort_unstable();
            excerpts.remove_excerpts(&excerpts_to_delete, cx)
        });
        self.editor
            .update(cx, |editor, cx| editor.remove_blocks(blocks_to_delete, cx));
    }

    fn update_excerpts(&self, paths: HashSet<ProjectPath>, cx: &mut ViewContext<Self>) {
        let project = self.project.clone();
        cx.spawn(|this, mut cx| {
            async move {
                for path in paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(path, cx))
                        .await?;
                    this.update(&mut cx, |view, cx| view.populate_excerpts(buffer, cx))
                }
                Result::<_, anyhow::Error>::Ok(())
            }
            .log_err()
        })
        .detach();
    }

    fn populate_excerpts(&mut self, buffer: ModelHandle<Buffer>, cx: &mut ViewContext<Self>) {
        let snapshot;
        let path;
        {
            let buffer = buffer.read(cx);
            snapshot = buffer.snapshot();
            if let Some(file) = buffer.file() {
                path = file.path().clone();
            } else {
                return;
            }
        }

        let path_ix = match self
            .path_states
            .binary_search_by_key(&path.as_ref(), |e| e.0.as_ref())
        {
            Ok(ix) => ix,
            Err(ix) => {
                self.path_states
                    .insert(ix, (path.clone(), Default::default()));
                ix
            }
        };

        let mut prev_excerpt_id = if path_ix > 0 {
            let prev_path_last_group = &self.path_states[path_ix - 1].1.last().unwrap();
            prev_path_last_group.excerpts.last().unwrap().clone()
        } else {
            ExcerptId::min()
        };

        let groups = &mut self.path_states[path_ix].1;
        let mut groups_to_add = Vec::new();
        let mut group_ixs_to_remove = Vec::new();
        let mut blocks_to_add = Vec::new();
        let mut blocks_to_restyle = HashMap::default();
        let mut blocks_to_remove = HashSet::default();
        let selected_excerpts = self
            .editor
            .read(cx)
            .local_anchor_selections()
            .iter()
            .flat_map(|s| [s.start.excerpt_id().clone(), s.end.excerpt_id().clone()])
            .collect::<HashSet<_>>();
        let mut diagnostic_blocks = Vec::new();
        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, excerpts_cx| {
            let mut old_groups = groups.iter_mut().enumerate().peekable();
            let mut new_groups = snapshot
                .diagnostic_groups()
                .into_iter()
                .filter(|group| group.entries[group.primary_ix].diagnostic.is_disk_based)
                .peekable();

            loop {
                let mut to_insert = None;
                let mut to_invalidate = None;
                let mut to_validate = None;
                match (old_groups.peek(), new_groups.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => to_insert = new_groups.next(),
                    (Some(_), None) => to_invalidate = old_groups.next(),
                    (Some((_, old_group)), Some(new_group)) => {
                        let old_primary = &old_group.primary_diagnostic;
                        let new_primary = &new_group.entries[new_group.primary_ix];
                        match compare_diagnostics(old_primary, new_primary, &snapshot) {
                            Ordering::Less => to_invalidate = old_groups.next(),
                            Ordering::Equal => {
                                to_validate = old_groups.next();
                                new_groups.next();
                            }
                            Ordering::Greater => to_insert = new_groups.next(),
                        }
                    }
                }

                if let Some(group) = to_insert {
                    let mut group_state = DiagnosticGroupState {
                        primary_diagnostic: group.entries[group.primary_ix].clone(),
                        excerpts: Default::default(),
                        blocks: Default::default(),
                        block_count: 0,
                        is_valid: true,
                    };
                    let mut pending_range: Option<(Range<Point>, usize)> = None;
                    let mut is_first_excerpt_for_group = true;
                    for (ix, entry) in group.entries.iter().map(Some).chain([None]).enumerate() {
                        let resolved_entry = entry.map(|e| e.resolve::<Point>(&snapshot));
                        if let Some((range, start_ix)) = &mut pending_range {
                            if let Some(entry) = resolved_entry.as_ref() {
                                if entry.range.start.row
                                    <= range.end.row + 1 + CONTEXT_LINE_COUNT * 2
                                {
                                    range.end = range.end.max(entry.range.end);
                                    continue;
                                }
                            }

                            let excerpt_start =
                                Point::new(range.start.row.saturating_sub(CONTEXT_LINE_COUNT), 0);
                            let excerpt_end = snapshot.clip_point(
                                Point::new(range.end.row + CONTEXT_LINE_COUNT, u32::MAX),
                                Bias::Left,
                            );
                            let excerpt_id = excerpts.insert_excerpt_after(
                                &prev_excerpt_id,
                                ExcerptProperties {
                                    buffer: &buffer,
                                    range: excerpt_start..excerpt_end,
                                },
                                excerpts_cx,
                            );

                            prev_excerpt_id = excerpt_id.clone();
                            group_state.excerpts.push(excerpt_id.clone());
                            let header_position = (excerpt_id.clone(), language::Anchor::min());

                            if is_first_excerpt_for_group {
                                is_first_excerpt_for_group = false;
                                let primary = &group.entries[group.primary_ix].diagnostic;
                                group_state.block_count += 1;
                                diagnostic_blocks.push(DiagnosticBlock::Header(primary.clone()));
                                blocks_to_add.push(BlockProperties {
                                    position: header_position,
                                    height: 2,
                                    render: diagnostic_header_renderer(
                                        buffer.clone(),
                                        primary.clone(),
                                        true,
                                        self.build_settings.clone(),
                                    ),
                                    disposition: BlockDisposition::Above,
                                });
                            } else {
                                group_state.block_count += 1;
                                diagnostic_blocks.push(DiagnosticBlock::Context);
                                blocks_to_add.push(BlockProperties {
                                    position: header_position,
                                    height: 1,
                                    render: context_header_renderer(self.build_settings.clone()),
                                    disposition: BlockDisposition::Above,
                                });
                            }

                            for entry in &group.entries[*start_ix..ix] {
                                if !entry.diagnostic.is_primary {
                                    group_state.block_count += 1;
                                    diagnostic_blocks
                                        .push(DiagnosticBlock::Inline(entry.diagnostic.clone()));
                                    blocks_to_add.push(BlockProperties {
                                        position: (excerpt_id.clone(), entry.range.start.clone()),
                                        height: entry.diagnostic.message.matches('\n').count()
                                            as u8
                                            + 1,
                                        render: diagnostic_block_renderer(
                                            entry.diagnostic.clone(),
                                            true,
                                            self.build_settings.clone(),
                                        ),
                                        disposition: BlockDisposition::Below,
                                    });
                                }
                            }

                            pending_range.take();
                        }

                        if let Some(entry) = resolved_entry {
                            pending_range = Some((entry.range.clone(), ix));
                        }
                    }

                    groups_to_add.push(group_state);
                } else if let Some((group_ix, group_state)) = to_invalidate {
                    if group_state
                        .excerpts
                        .iter()
                        .any(|excerpt_id| selected_excerpts.contains(excerpt_id))
                    {
                        for (block_id, block) in &group_state.blocks {
                            match block {
                                DiagnosticBlock::Header(diagnostic) => {
                                    blocks_to_restyle.insert(
                                        *block_id,
                                        diagnostic_header_renderer(
                                            buffer.clone(),
                                            diagnostic.clone(),
                                            false,
                                            self.build_settings.clone(),
                                        ),
                                    );
                                }
                                DiagnosticBlock::Inline(diagnostic) => {
                                    blocks_to_restyle.insert(
                                        *block_id,
                                        diagnostic_block_renderer(
                                            diagnostic.clone(),
                                            false,
                                            self.build_settings.clone(),
                                        ),
                                    );
                                }
                                DiagnosticBlock::Context => {}
                            }
                        }

                        group_state.is_valid = false;
                        prev_excerpt_id = group_state.excerpts.last().unwrap().clone();
                    } else {
                        excerpts.remove_excerpts(group_state.excerpts.iter(), excerpts_cx);
                        group_ixs_to_remove.push(group_ix);
                        blocks_to_remove.extend(group_state.blocks.keys().copied());
                    }
                } else if let Some((_, group_state)) = to_validate {
                    for (block_id, block) in &group_state.blocks {
                        match block {
                            DiagnosticBlock::Header(diagnostic) => {
                                blocks_to_restyle.insert(
                                    *block_id,
                                    diagnostic_header_renderer(
                                        buffer.clone(),
                                        diagnostic.clone(),
                                        true,
                                        self.build_settings.clone(),
                                    ),
                                );
                            }
                            DiagnosticBlock::Inline(diagnostic) => {
                                blocks_to_restyle.insert(
                                    *block_id,
                                    diagnostic_block_renderer(
                                        diagnostic.clone(),
                                        true,
                                        self.build_settings.clone(),
                                    ),
                                );
                            }
                            DiagnosticBlock::Context => {}
                        }
                    }
                    group_state.is_valid = true;
                    prev_excerpt_id = group_state.excerpts.last().unwrap().clone();
                } else {
                    unreachable!();
                }
            }

            excerpts.snapshot(excerpts_cx)
        });

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, cx);
            editor.replace_blocks(blocks_to_restyle, cx);
            let mut block_ids = editor
                .insert_blocks(
                    blocks_to_add.into_iter().map(|block| {
                        let (excerpt_id, text_anchor) = block.position;
                        BlockProperties {
                            position: excerpts_snapshot.anchor_in_excerpt(excerpt_id, text_anchor),
                            height: block.height,
                            render: block.render,
                            disposition: block.disposition,
                        }
                    }),
                    cx,
                )
                .into_iter()
                .zip(diagnostic_blocks);

            for group_state in &mut groups_to_add {
                group_state.blocks = block_ids.by_ref().take(group_state.block_count).collect();
            }
        });

        for ix in group_ixs_to_remove.into_iter().rev() {
            groups.remove(ix);
        }
        groups.extend(groups_to_add);
        groups.sort_unstable_by(|a, b| {
            let range_a = &a.primary_diagnostic.range;
            let range_b = &b.primary_diagnostic.range;
            range_a
                .start
                .cmp(&range_b.start, &snapshot)
                .unwrap()
                .then_with(|| range_a.end.cmp(&range_b.end, &snapshot).unwrap())
        });

        if groups.is_empty() {
            self.path_states.remove(path_ix);
        }

        cx.notify();
    }
}

impl workspace::Item for ProjectDiagnostics {
    type View = ProjectDiagnosticsEditor;

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<workspace::Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        let project = handle.read(cx).project.clone();
        ProjectDiagnosticsEditor::new(project, settings, cx)
    }

    fn project_path(&self) -> Option<project::ProjectPath> {
        None
    }
}

impl workspace::ItemView for ProjectDiagnosticsEditor {
    fn title(&self, _: &AppContext) -> String {
        "Project Diagnostics".to_string()
    }

    fn project_path(&self, _: &AppContext) -> Option<project::ProjectPath> {
        None
    }

    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>> {
        self.excerpts.update(cx, |excerpts, cx| excerpts.save(cx))
    }

    fn save_as(
        &mut self,
        _: ModelHandle<project::Worktree>,
        _: &std::path::Path,
        _: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).read(cx).has_conflict()
    }

    fn should_update_tab_on_event(event: &Event) -> bool {
        matches!(
            event,
            Event::Saved | Event::Dirtied | Event::FileHandleChanged
        )
    }

    fn can_save(&self, _: &AppContext) -> bool {
        true
    }

    fn can_save_as(&self, _: &AppContext) -> bool {
        false
    }
}

fn compare_diagnostics<L: language::ToOffset, R: language::ToOffset>(
    lhs: &DiagnosticEntry<L>,
    rhs: &DiagnosticEntry<R>,
    snapshot: &language::BufferSnapshot,
) -> Ordering {
    lhs.range
        .start
        .to_offset(&snapshot)
        .cmp(&rhs.range.start.to_offset(snapshot))
        .then_with(|| {
            lhs.range
                .end
                .to_offset(&snapshot)
                .cmp(&rhs.range.end.to_offset(snapshot))
        })
        .then_with(|| lhs.diagnostic.message.cmp(&rhs.diagnostic.message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{http::ServerResponse, test::FakeHttpClient, Client, UserStore};
    use gpui::TestAppContext;
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, LanguageRegistry, PointUtf16};
    use project::{worktree, FakeFs};
    use serde_json::json;
    use std::sync::Arc;
    use unindent::Unindent as _;
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_diagnostics(mut cx: TestAppContext) {
        let settings = cx.update(WorkspaceParams::test).settings;
        let http_client = FakeHttpClient::new(|_| async move { Ok(ServerResponse::new(404)) });
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let fs = Arc::new(FakeFs::new());

        let project = cx.update(|cx| {
            Project::local(
                client.clone(),
                user_store,
                Arc::new(LanguageRegistry::new()),
                fs.clone(),
                cx,
            )
        });

        fs.insert_tree(
            "/test",
            json!({
                "a.rs": "
                    const a: i32 = 'a';
                ".unindent(),

                "main.rs": "
                    fn main() {
                        let x = vec![];
                        let y = vec![];
                        a(x);
                        b(y);
                        // comment 1
                        // comment 2
                        c(y);
                        d(x);
                    }
                "
                .unindent(),
            }),
        )
        .await;

        let worktree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree("/test", cx)
            })
            .await
            .unwrap();

        worktree.update(&mut cx, |worktree, cx| {
            worktree
                .update_diagnostic_entries(
                    Arc::from("/test/main.rs".as_ref()),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: PointUtf16::new(1, 8)..PointUtf16::new(1, 9),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(2, 8)..PointUtf16::new(2, 9),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(3, 6)..PointUtf16::new(3, 7),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(4, 6)..PointUtf16::new(4, 7),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(7, 6)..PointUtf16::new(7, 7),
                            diagnostic: Diagnostic {
                                message: "use of moved value".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(7, 6)..PointUtf16::new(7, 7),
                            diagnostic: Diagnostic {
                                message: "value used here after move".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(8, 6)..PointUtf16::new(8, 7),
                            diagnostic: Diagnostic {
                                message: "use of moved value".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(8, 6)..PointUtf16::new(8, 7),
                            diagnostic: Diagnostic {
                                message: "value used here after move".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
        });

        let view = cx.add_view(Default::default(), |cx| {
            ProjectDiagnosticsEditor::new(project.clone(), settings, cx)
        });

        view.condition(&mut cx, |view, cx| view.text(cx).contains("fn main()"))
            .await;

        view.update(&mut cx, |view, cx| {
            let editor = view.editor.update(cx, |editor, cx| editor.snapshot(cx));

            assert_eq!(
                editor.text(),
                concat!(
                    //
                    // main.rs, diagnostic group 1
                    //
                    "\n", // primary message
                    "\n", // filename
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "\n", // supporting diagnostic
                    "    a(x);\n",
                    "    b(y);\n",
                    "\n", // supporting diagnostic
                    "    // comment 1\n",
                    "    // comment 2\n",
                    "    c(y);\n",
                    "\n", // supporting diagnostic
                    "    d(x);\n",
                    //
                    // main.rs, diagnostic group 2
                    //
                    "\n", // primary message
                    "\n", // filename
                    "fn main() {\n",
                    "    let x = vec![];\n",
                    "\n", // supporting diagnostic
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // supporting diagnostic
                    "    b(y);\n",
                    "\n", // context ellipsis
                    "    c(y);\n",
                    "    d(x);\n",
                    "\n", // supporting diagnostic
                    "}"
                )
            );
        });

        worktree.update(&mut cx, |worktree, cx| {
            worktree
                .update_diagnostic_entries(
                    Arc::from("/test/a.rs".as_ref()),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: PointUtf16::new(0, 15)..PointUtf16::new(0, 15),
                            diagnostic: Diagnostic {
                                message: "mismatched types".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(0, 15)..PointUtf16::new(0, 15),
                            diagnostic: Diagnostic {
                                message: "expected `usize`, found `char`".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
            cx.emit(worktree::Event::DiskBasedDiagnosticsUpdated);
        });

        view.condition(&mut cx, |view, cx| view.text(cx).contains("const a"))
            .await;

        view.update(&mut cx, |view, cx| {
            let editor = view.editor.update(cx, |editor, cx| editor.snapshot(cx));

            assert_eq!(
                editor.text(),
                concat!(
                    //
                    // a.rs
                    //
                    "\n", // primary message
                    "\n", // filename
                    "const a: i32 = 'a';\n",
                    "\n", // supporting diagnostic
                    "\n", // context line
                    //
                    // main.rs, diagnostic group 1
                    //
                    "\n", // primary message
                    "\n", // filename
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "\n", // supporting diagnostic
                    "    a(x);\n",
                    "    b(y);\n",
                    "\n", // supporting diagnostic
                    "    // comment 1\n",
                    "    // comment 2\n",
                    "    c(y);\n",
                    "\n", // supporting diagnostic
                    "    d(x);\n",
                    //
                    // main.rs, diagnostic group 2
                    //
                    "\n", // primary message
                    "\n", // filename
                    "fn main() {\n",
                    "    let x = vec![];\n",
                    "\n", // supporting diagnostic
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // supporting diagnostic
                    "    b(y);\n",
                    "\n", // context ellipsis
                    "    c(y);\n",
                    "    d(x);\n",
                    "\n", // supporting diagnostic
                    "}"
                )
            );
        });
    }
}
