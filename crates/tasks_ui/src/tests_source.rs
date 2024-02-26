use gpui::{AppContext, Model, WeakModel};
use project::Project;
use task::Source;
use ui::Context;

/// Returns runnables for tests at current cursor, module and file.
pub struct TestSource {
    project: WeakModel<Project>,
}

impl TestSource {
    pub fn new(project: WeakModel<Project>, cx: &mut AppContext) -> Model<Box<dyn Source>> {
        cx.new_model(|_| Box::new(Self { project }) as _)
    }
}
impl Source for TestSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        _path: Option<&std::path::Path>,
        _cx: &mut gpui::ModelContext<Box<dyn Source>>,
    ) -> Vec<std::sync::Arc<dyn task::Task>> {
        dbg!(_path);
        vec![]
    }
}
