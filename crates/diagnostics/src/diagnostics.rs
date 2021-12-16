use std::{cmp, sync::Arc};

use editor::{
    diagnostic_block_renderer, diagnostic_style,
    display_map::{BlockDisposition, BlockProperties},
    Editor, ExcerptProperties, MultiBuffer,
};
use gpui::{
    action, elements::*, keymap::Binding, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, View, ViewContext, ViewHandle,
};
use language::Point;
use postage::watch;
use project::Project;
use workspace::Workspace;

action!(Toggle);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([Binding::new("alt-shift-D", Toggle, None)]);
    cx.add_action(ProjectDiagnosticsEditor::toggle);
}

struct ProjectDiagnostics {
    project: ModelHandle<Project>,
}

struct ProjectDiagnosticsEditor {
    editor: ViewHandle<Editor>,
    excerpts: ModelHandle<MultiBuffer>,
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
    type Event = ();
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
    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let diagnostics = cx.add_model(|_| ProjectDiagnostics::new(workspace.project().clone()));
        workspace.add_item(diagnostics, cx);
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
        let excerpts = cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id(cx)));
        let build_settings = editor::settings_builder(excerpts.downgrade(), settings.clone());
        let editor =
            cx.add_view(|cx| Editor::for_buffer(excerpts.clone(), build_settings.clone(), cx));

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
                    let snapshot = buffer.read_with(&cx, |b, _| b.snapshot());

                    this.update(&mut cx, |this, cx| {
                        let mut blocks = Vec::new();
                        let excerpts_snapshot =
                            this.excerpts.update(cx, |excerpts, excerpts_cx| {
                                for group in snapshot.diagnostic_groups::<Point>() {
                                    let excerpt_start = cmp::min(
                                        group.primary.range.start.row,
                                        group
                                            .supporting
                                            .first()
                                            .map_or(u32::MAX, |entry| entry.range.start.row),
                                    );
                                    let excerpt_end = cmp::max(
                                        group.primary.range.end.row,
                                        group
                                            .supporting
                                            .last()
                                            .map_or(0, |entry| entry.range.end.row),
                                    );

                                    let primary_diagnostic = group.primary.diagnostic;
                                    let excerpt_id = excerpts.push_excerpt(
                                        ExcerptProperties {
                                            buffer: &buffer,
                                            range: Point::new(excerpt_start, 0)
                                                ..Point::new(
                                                    excerpt_end,
                                                    snapshot.line_len(excerpt_end),
                                                ),
                                            header_height: primary_diagnostic
                                                .message
                                                .matches('\n')
                                                .count()
                                                as u8
                                                + 1,
                                            render_header: Some(Arc::new({
                                                let settings = settings.clone();

                                                move |_| {
                                                    let editor_style =
                                                        &settings.borrow().theme.editor;
                                                    let mut text_style = editor_style.text.clone();
                                                    text_style.color = diagnostic_style(
                                                        primary_diagnostic.severity,
                                                        true,
                                                        &editor_style,
                                                    )
                                                    .text;

                                                    Text::new(
                                                        primary_diagnostic.message.clone(),
                                                        text_style,
                                                    )
                                                    .boxed()
                                                }
                                            })),
                                        },
                                        excerpts_cx,
                                    );

                                    for entry in group.supporting {
                                        let buffer_anchor =
                                            snapshot.anchor_before(entry.range.start);
                                        blocks.push(BlockProperties {
                                            position: (excerpt_id.clone(), buffer_anchor),
                                            height: entry.diagnostic.message.matches('\n').count()
                                                as u8
                                                + 1,
                                            render: diagnostic_block_renderer(
                                                entry.diagnostic,
                                                true,
                                                build_settings.clone(),
                                            ),
                                            disposition: BlockDisposition::Below,
                                        });
                                    }
                                }

                                excerpts.snapshot(excerpts_cx)
                            });

                        this.editor.update(cx, |editor, cx| {
                            editor.insert_blocks(
                                blocks.into_iter().map(|block| {
                                    let (excerpt_id, text_anchor) = block.position;
                                    BlockProperties {
                                        position: excerpts_snapshot
                                            .anchor_in_excerpt(excerpt_id, text_anchor),
                                        height: block.height,
                                        render: block.render,
                                        disposition: block.disposition,
                                    }
                                }),
                                cx,
                            );
                        });
                    })
                }
                Result::Ok::<_, anyhow::Error>(())
            }
        })
        .detach();

        ProjectDiagnosticsEditor { editor, excerpts }
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

    fn save(
        &mut self,
        _: &mut ViewContext<Self>,
    ) -> anyhow::Result<gpui::Task<anyhow::Result<()>>> {
        todo!()
    }

    fn save_as(
        &mut self,
        _: ModelHandle<project::Worktree>,
        _: &std::path::Path,
        _: &mut ViewContext<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        todo!()
    }
}
