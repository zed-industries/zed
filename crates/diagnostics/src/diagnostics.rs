use anyhow::Result;
use collections::{hash_map, HashMap, HashSet};
use editor::{
    context_header_renderer, diagnostic_block_renderer, diagnostic_header_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties},
    BuildSettings, Editor, ExcerptId, ExcerptProperties, MultiBuffer,
};
use gpui::{
    action, elements::*, keymap::Binding, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, Task, View, ViewContext, ViewHandle,
};
use language::{Bias, Buffer, Point};
use postage::watch;
use project::Project;
use std::{ops::Range, path::Path, sync::Arc};
use util::TryFutureExt;
use workspace::Workspace;

action!(Toggle);

const CONTEXT_LINE_COUNT: u32 = 1;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([Binding::new("alt-shift-D", Toggle, None)]);
    cx.add_action(ProjectDiagnosticsEditor::toggle);
}

type Event = editor::Event;

struct ProjectDiagnostics {
    project: ModelHandle<Project>,
}

struct ProjectDiagnosticsEditor {
    editor: ViewHandle<Editor>,
    excerpts: ModelHandle<MultiBuffer>,
    path_states: Vec<(Arc<Path>, PathState)>,
    build_settings: BuildSettings,
}

#[derive(Default)]
struct PathState {
    last_excerpt: ExcerptId,
    diagnostic_group_states: HashMap<usize, DiagnosticGroupState>,
}

#[derive(Default)]
struct DiagnosticGroupState {
    excerpts: Vec<ExcerptId>,
    blocks: Vec<BlockId>,
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
        let project_paths = project
            .read(cx)
            .diagnostic_summaries(cx)
            .map(|e| e.0)
            .collect::<Vec<_>>();

        cx.spawn(|this, mut cx| {
            let project = project.clone();
            async move {
                for project_path in project_paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))
                        .await?;
                    this.update(&mut cx, |view, cx| view.populate_excerpts(buffer, cx))
                }
                Result::<_, anyhow::Error>::Ok(())
            }
        })
        .detach();

        cx.subscribe(&project, |_, project, event, cx| {
            if let project::Event::DiagnosticsUpdated(project_path) = event {
                let project_path = project_path.clone();
                cx.spawn(|this, mut cx| {
                    async move {
                        let buffer = project
                            .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))
                            .await?;
                        this.update(&mut cx, |view, cx| view.populate_excerpts(buffer, cx));
                        Ok(())
                    }
                    .log_err()
                })
                .detach();
            }
        })
        .detach();

        let excerpts = cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id()));
        let build_settings = editor::settings_builder(excerpts.downgrade(), settings.clone());
        let editor =
            cx.add_view(|cx| Editor::for_buffer(excerpts.clone(), build_settings.clone(), cx));
        cx.subscribe(&editor, |_, _, event, cx| cx.emit(*event))
            .detach();
        Self {
            excerpts,
            editor,
            build_settings,
            path_states: Default::default(),
        }
    }

    #[cfg(test)]
    fn text(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let diagnostics = cx.add_model(|_| ProjectDiagnostics::new(workspace.project().clone()));
        workspace.add_item(diagnostics, cx);
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
                self.path_states.insert(
                    ix,
                    (
                        path.clone(),
                        PathState {
                            last_excerpt: ExcerptId::max(),
                            diagnostic_group_states: Default::default(),
                        },
                    ),
                );
                ix
            }
        };
        let mut prev_excerpt_id = if path_ix > 0 {
            self.path_states[path_ix - 1].1.last_excerpt.clone()
        } else {
            ExcerptId::min()
        };
        let path_state = &mut self.path_states[path_ix].1;

        let mut blocks_to_add = Vec::new();
        let mut blocks_to_remove = HashSet::default();
        let mut excerpts_to_remove = Vec::new();
        let mut block_counts_by_group = Vec::new();

        let diagnostic_groups = snapshot.diagnostic_groups::<Point>();
        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, excerpts_cx| {
            for group in &diagnostic_groups {
                let group_id = group.entries[0].diagnostic.group_id;

                let group_state = match path_state.diagnostic_group_states.entry(group_id) {
                    hash_map::Entry::Occupied(e) => {
                        prev_excerpt_id = e.get().excerpts.last().unwrap().clone();
                        block_counts_by_group.push(0);
                        continue;
                    }
                    hash_map::Entry::Vacant(e) => e.insert(DiagnosticGroupState::default()),
                };

                let mut block_count = 0;
                let mut pending_range: Option<(Range<Point>, usize)> = None;
                let mut is_first_excerpt_for_group = true;
                for (ix, entry) in group.entries.iter().map(Some).chain([None]).enumerate() {
                    if let Some((range, start_ix)) = &mut pending_range {
                        if let Some(entry) = entry {
                            if entry.range.start.row <= range.end.row + 1 + CONTEXT_LINE_COUNT * 2 {
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
                            let mut header = primary.clone();
                            header.message =
                                primary.message.split('\n').next().unwrap().to_string();
                            block_count += 1;
                            blocks_to_add.push(BlockProperties {
                                position: header_position,
                                height: 2,
                                render: diagnostic_header_renderer(
                                    buffer.clone(),
                                    header,
                                    self.build_settings.clone(),
                                ),
                                disposition: BlockDisposition::Above,
                            });
                        } else {
                            block_count += 1;
                            blocks_to_add.push(BlockProperties {
                                position: header_position,
                                height: 1,
                                render: context_header_renderer(self.build_settings.clone()),
                                disposition: BlockDisposition::Above,
                            });
                        }

                        for entry in &group.entries[*start_ix..ix] {
                            let mut diagnostic = entry.diagnostic.clone();
                            if diagnostic.is_primary {
                                let mut lines = entry.diagnostic.message.split('\n');
                                lines.next();
                                diagnostic.message = lines.collect();
                            }

                            if !diagnostic.message.is_empty() {
                                let buffer_anchor = snapshot.anchor_before(entry.range.start);
                                block_count += 1;
                                blocks_to_add.push(BlockProperties {
                                    position: (excerpt_id.clone(), buffer_anchor),
                                    height: diagnostic.message.matches('\n').count() as u8 + 1,
                                    render: diagnostic_block_renderer(
                                        diagnostic,
                                        true,
                                        self.build_settings.clone(),
                                    ),
                                    disposition: BlockDisposition::Below,
                                });
                            }
                        }

                        pending_range.take();
                    }

                    if let Some(entry) = entry {
                        pending_range = Some((entry.range.clone(), ix));
                    }
                }

                block_counts_by_group.push(block_count);
            }

            path_state
                .diagnostic_group_states
                .retain(|group_id, group_state| {
                    if diagnostic_groups
                        .iter()
                        .any(|group| group.entries[0].diagnostic.group_id == *group_id)
                    {
                        true
                    } else {
                        excerpts_to_remove.extend(group_state.excerpts.drain(..));
                        blocks_to_remove.extend(group_state.blocks.drain(..));
                        false
                    }
                });

            excerpts_to_remove.sort();
            excerpts.remove_excerpts(excerpts_to_remove.iter(), excerpts_cx);
            excerpts.snapshot(excerpts_cx)
        });

        path_state.last_excerpt = prev_excerpt_id;

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, cx);
            let block_ids = editor.insert_blocks(
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
            );

            let mut block_ids = block_ids.into_iter();
            let mut block_counts_by_group = block_counts_by_group.into_iter();
            for group in &diagnostic_groups {
                let group_id = group.entries[0].diagnostic.group_id;
                let block_count = block_counts_by_group.next().unwrap();
                let group_state = path_state
                    .diagnostic_group_states
                    .get_mut(&group_id)
                    .unwrap();
                group_state
                    .blocks
                    .extend(block_ids.by_ref().take(block_count));
            }
        });
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

#[cfg(test)]
mod tests {
    use super::*;
    use client::{http::ServerResponse, test::FakeHttpClient, Client, UserStore};
    use gpui::TestAppContext;
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, LanguageRegistry, PointUtf16};
    use project::FakeFs;
    use serde_json::json;
    use std::sync::Arc;
    use unindent::Unindent as _;
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_diagnostics(mut cx: TestAppContext) {
        let settings = cx.update(WorkspaceParams::test).settings;
        let http_client = FakeHttpClient::new(|_| async move { Ok(ServerResponse::new(404)) });
        let client = Client::new();
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
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(7, 6)..PointUtf16::new(7, 7),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(8, 6)..PointUtf16::new(8, 7),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
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
                    vec![DiagnosticEntry {
                        range: PointUtf16::new(0, 15)..PointUtf16::new(0, 15),
                        diagnostic: Diagnostic {
                            message: "mismatched types\nexpected `usize`, found `char`".to_string(),
                            severity: DiagnosticSeverity::ERROR,
                            is_primary: true,
                            group_id: 0,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
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
