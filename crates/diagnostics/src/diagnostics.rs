use std::sync::Arc;

use collections::HashMap;
use editor::{diagnostic_style, Editor, ExcerptProperties, MultiBuffer};
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
        let editor = cx.add_view(|cx| {
            Editor::for_buffer(
                excerpts.clone(),
                editor::settings_builder(excerpts.downgrade(), settings.clone()),
                cx,
            )
        });

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

                    let mut grouped_diagnostics = HashMap::default();
                    for entry in snapshot.all_diagnostics() {
                        let mut group = grouped_diagnostics
                            .entry(entry.diagnostic.group_id)
                            .or_insert((Point::zero(), Vec::new()));
                        if entry.diagnostic.is_primary {
                            group.0 = entry.range.start;
                        }
                        group.1.push(entry);
                    }
                    let mut sorted_diagnostic_groups =
                        grouped_diagnostics.into_values().collect::<Vec<_>>();
                    sorted_diagnostic_groups.sort_by_key(|group| group.0);

                    for entry in snapshot.all_diagnostics::<Point>() {
                        this.update(&mut cx, |this, cx| {
                            this.excerpts.update(cx, |excerpts, cx| {
                                excerpts.push_excerpt(
                                    ExcerptProperties {
                                        buffer: &buffer,
                                        range: entry.range,
                                        header_height: entry
                                            .diagnostic
                                            .message
                                            .matches('\n')
                                            .count()
                                            as u8
                                            + 1,
                                        render_header: Some(Arc::new({
                                            let message = entry.diagnostic.message.clone();
                                            let settings = settings.clone();

                                            move |_| {
                                                let editor_style = &settings.borrow().theme.editor;
                                                let mut text_style = editor_style.text.clone();
                                                text_style.color = diagnostic_style(
                                                    entry.diagnostic.severity,
                                                    true,
                                                    &editor_style,
                                                )
                                                .text;

                                                Text::new(message.clone(), text_style).boxed()
                                            }
                                        })),
                                    },
                                    cx,
                                );
                                cx.notify();
                            });
                        })
                    }
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
