use anyhow::Result;
use gpui::{AppContext, Model, Task, WeakModel};
use language::Buffer;
use project::Project;
use project_core::Location;
use task::Source;
use ui::Context;

/// Returns runnables for tests at current cursor, module and file.
pub struct TestSource {
    project: WeakModel<Project>,
    all_tasks_for: Option<(WeakModel<Buffer>, Task<Result<Vec<lsp::DocumentSymbol>>>)>,
}

impl TestSource {
    pub fn new(project: WeakModel<Project>, cx: &mut AppContext) -> Model<Box<dyn Source>> {
        cx.new_model(|_| {
            Box::new(Self {
                project,
                all_tasks_for: None,
            }) as _
        })
    }
}
impl Source for TestSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        _path: Option<&Location>,
        cx: &mut gpui::ModelContext<Box<dyn Source>>,
    ) -> Vec<std::sync::Arc<dyn task::Task>> {
        if let Some(path) = _path {
            self.project.update(cx, move |_, cx| {
                let p = path.buffer.clone();
                cx.spawn(|this, mut cx| async move {
                    dbg!("Heyyo");
                    dbg!(
                        this.update(&mut cx, |this, cx| this.document_symbols(&p, cx))
                            .ok()?
                            .await
                    );
                    dbg!("Hey");
                    Some(())
                })
                .detach();
            });
            // self.all_tasks_for = Some((
            //     path.buffer.downgrade(),
            //     self.project.update(cx, |this, cx| {
            //         this.document_symbols(&path.buffer, cx).shared()
            //     }),
            // ));
        }

        vec![]
    }
}
