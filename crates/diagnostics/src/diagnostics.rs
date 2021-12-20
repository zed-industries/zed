use anyhow::Result;
use editor::{
    context_header_renderer, diagnostic_block_renderer, diagnostic_header_renderer,
    display_map::{BlockDisposition, BlockProperties},
    BuildSettings, Editor, ExcerptProperties, MultiBuffer,
};
use gpui::{
    action, elements::*, keymap::Binding, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, Task, View, ViewContext, ViewHandle,
};
use language::{Bias, Buffer, Point};
use postage::watch;
use project::Project;
use std::ops::Range;
use workspace::Workspace;

action!(Toggle);

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
    build_settings: BuildSettings,
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
        replica_id: u16,
        settings: watch::Receiver<workspace::Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let excerpts = cx.add_model(|_| MultiBuffer::new(replica_id));
        let build_settings = editor::settings_builder(excerpts.downgrade(), settings.clone());
        let editor =
            cx.add_view(|cx| Editor::for_buffer(excerpts.clone(), build_settings.clone(), cx));
        cx.subscribe(&editor, |_, _, event, cx| cx.emit(*event))
            .detach();
        Self {
            excerpts,
            editor,
            build_settings,
        }
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let diagnostics = cx.add_model(|_| ProjectDiagnostics::new(workspace.project().clone()));
        workspace.add_item(diagnostics, cx);
    }

    fn populate_excerpts(&mut self, buffer: ModelHandle<Buffer>, cx: &mut ViewContext<Self>) {
        let mut blocks = Vec::new();
        let snapshot = buffer.read(cx).snapshot();

        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, excerpts_cx| {
            for group in snapshot.diagnostic_groups::<Point>() {
                let mut pending_range: Option<(Range<Point>, usize)> = None;
                let mut is_first_excerpt = true;
                for (ix, entry) in group.entries.iter().map(Some).chain([None]).enumerate() {
                    if let Some((range, start_ix)) = &mut pending_range {
                        if let Some(entry) = entry {
                            if entry.range.start.row <= range.end.row + 1 {
                                range.end = range.end.max(entry.range.end);
                                continue;
                            }
                        }

                        let excerpt_start = Point::new(range.start.row.saturating_sub(1), 0);
                        let excerpt_end = snapshot
                            .clip_point(Point::new(range.end.row + 1, u32::MAX), Bias::Left);

                        let mut excerpt = ExcerptProperties {
                            buffer: &buffer,
                            range: excerpt_start..excerpt_end,
                            header_height: 0,
                            render_header: None,
                        };

                        if is_first_excerpt {
                            let primary = &group.entries[group.primary_ix].diagnostic;
                            let mut header = primary.clone();
                            header.message =
                                primary.message.split('\n').next().unwrap().to_string();
                            excerpt.header_height = 2;
                            excerpt.render_header = Some(diagnostic_header_renderer(
                                buffer.clone(),
                                header,
                                self.build_settings.clone(),
                            ));
                        } else {
                            excerpt.header_height = 1;
                            excerpt.render_header =
                                Some(context_header_renderer(self.build_settings.clone()));
                        }

                        is_first_excerpt = false;
                        let excerpt_id = excerpts.push_excerpt(excerpt, excerpts_cx);
                        for entry in &group.entries[*start_ix..ix] {
                            let mut diagnostic = entry.diagnostic.clone();
                            if diagnostic.is_primary {
                                let mut lines = entry.diagnostic.message.split('\n');
                                lines.next();
                                diagnostic.message = lines.collect();
                            }

                            if !diagnostic.message.is_empty() {
                                let buffer_anchor = snapshot.anchor_before(entry.range.start);
                                blocks.push(BlockProperties {
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
            }

            excerpts.snapshot(excerpts_cx)
        });

        self.editor.update(cx, |editor, cx| {
            editor.insert_blocks(
                blocks.into_iter().map(|block| {
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
        });
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
        let project_paths = project
            .read(cx)
            .diagnostic_summaries(cx)
            .map(|e| e.0)
            .collect::<Vec<_>>();

        cx.spawn(|view, mut cx| {
            let project = project.clone();
            async move {
                for project_path in project_paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))
                        .await?;
                    view.update(&mut cx, |view, cx| view.populate_excerpts(buffer, cx))
                }
                Result::<_, anyhow::Error>::Ok(())
            }
        })
        .detach();

        ProjectDiagnosticsEditor::new(project.read(cx).replica_id(), settings, cx)
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
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, PointUtf16};
    use unindent::Unindent as _;
    use workspace::WorkspaceParams;

    #[gpui::test]
    fn test_diagnostics(cx: &mut MutableAppContext) {
        let settings = WorkspaceParams::test(cx).settings;
        let view = cx.add_view(Default::default(), |cx| {
            ProjectDiagnosticsEditor::new(0, settings, cx)
        });

        let text = "
        fn main() {
            let x = vec![];
            let y = vec![];
            a(x);
            b(y);
            // comment 1
            // comment 2
            // comment 3
            // comment 4
            d(y);
            e(x);
        }
        "
        .unindent();

        let buffer = cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, text, cx);
            buffer
                .update_diagnostics(
                    None,
                    vec![
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
                            range: PointUtf16::new(8, 6)..PointUtf16::new(8, 7),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
            buffer
        });

        view.update(cx, |view, cx| {
            view.populate_excerpts(buffer, cx);
            assert_eq!(
                view.excerpts.read(cx).read(cx).text(),
                concat!(
                    "\n", // primary diagnostic message
                    "\n", // filename
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // context ellipsis
                    "    a(x);\n",
                    "    b(y);\n",
                    "    // comment 1\n",
                    "\n", // context ellipsis
                    "    // comment 3\n",
                    "    // comment 4\n",
                    "    d(y);"
                )
            );
        });
    }
}
