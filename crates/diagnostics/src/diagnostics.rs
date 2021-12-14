use collections::HashMap;
use editor::{Editor, ExcerptProperties, MultiBuffer};
use gpui::{
    action, elements::*, keymap::Binding, AppContext, Entity, ModelContext, ModelHandle,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
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
    excerpts: ModelHandle<MultiBuffer>,
    project: ModelHandle<Project>,
}

struct ProjectDiagnosticsEditor {
    editor: ViewHandle<Editor>,
}

impl ProjectDiagnostics {
    fn new(project: ModelHandle<Project>, cx: &mut ModelContext<Self>) -> Self {
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

                    for diagnostic in snapshot.all_diagnostics::<Point>() {
                        this.update(&mut cx, |this, cx| {
                            this.excerpts.update(cx, |excerpts, cx| {
                                excerpts.push_excerpt(
                                    ExcerptProperties {
                                        buffer: &buffer,
                                        range: diagnostic.range,
                                        header_height: 1,
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

        Self {
            excerpts: cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id(cx))),
            project,
        }
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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.editor.id()).boxed()
    }
}

impl ProjectDiagnosticsEditor {
    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        dbg!("HEY!!!!");
        let diagnostics =
            cx.add_model(|cx| ProjectDiagnostics::new(workspace.project().clone(), cx));
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
        let excerpts = handle.read(cx).excerpts.clone();
        let editor = cx.add_view(|cx| {
            Editor::for_buffer(
                excerpts.clone(),
                editor::settings_builder(excerpts.downgrade(), settings),
                cx,
            )
        });
        ProjectDiagnosticsEditor { editor }
    }

    fn project_path(&self) -> Option<project::ProjectPath> {
        None
    }
}

impl workspace::ItemView for ProjectDiagnosticsEditor {
    fn title(&self, _: &AppContext) -> String {
        "Project Diagnostics".to_string()
    }

    fn project_path(&self, cx: &AppContext) -> Option<project::ProjectPath> {
        None
    }

    fn save(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> anyhow::Result<gpui::Task<anyhow::Result<()>>> {
        todo!()
    }

    fn save_as(
        &mut self,
        worktree: ModelHandle<project::Worktree>,
        path: &std::path::Path,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        todo!()
    }
}
