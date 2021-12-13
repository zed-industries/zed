use editor::{Editor, MultiBuffer};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, View, ViewContext, ViewHandle};
use postage::watch;
use project::Project;

struct ProjectDiagnostics {
    editor: ViewHandle<Editor>,
    project: ModelHandle<Project>,
}

impl ProjectDiagnostics {
    fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<workspace::Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut buffer = cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id(cx)));
        for diagnostic_summary in project.read(cx).diagnostic_summaries(cx) {
            //
        }

        Self {
            editor: cx.add_view(|cx| {
                Editor::for_buffer(
                    buffer.clone(),
                    editor::settings_builder(buffer.downgrade(), settings),
                    cx,
                )
            }),
            project,
        }
    }
}

impl Entity for ProjectDiagnostics {
    type Event = ();
}

impl View for ProjectDiagnostics {
    fn ui_name() -> &'static str {
        "ProjectDiagnostics"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.editor.id()).boxed()
    }
}
