use gpui::{AppContext, Context};

use crate::{
    next_source_id, static_runnable_file::Runnables, RunState, Runnable, RunnablePebble, Source,
    SourceId, StaticRunner,
};

struct StaticSource {
    id: SourceId,
    definitions: Runnables,
}

impl StaticSource {
    fn new() -> Self {
        Self {
            id: next_source_id(),
            definitions: Runnables::default(), // TODO kb use Option instead?
        }
    }

    /// Replace current definitions with the newly parsed runnables from the file(s)
    pub fn update_definitions(&mut self, runnables: Runnables) {
        self.definitions = runnables
    }
}

impl Source for StaticSource {
    fn id(&self) -> crate::SourceId {
        self.id
    }

    fn runnables_for_path(
        &self,
        _: &std::path::Path,
        cx: &mut AppContext,
    ) -> anyhow::Result<Box<dyn Iterator<Item = crate::RunnablePebble>>> {
        Ok(Box::new(self.definitions.tasks.iter().cloned().map(
            |def| {
                let runner = StaticRunner::new(def);
                let source_id = self.id;
                let display_name = runner.name();
                let runnable_id = runner.id();
                let state = cx.new_model(|_| RunState::NotScheduled(Box::new(runner)));
                crate::RunnablePebble {
                    metadata: crate::RunnableLens {
                        source_id,
                        runnable_id,
                        display_name,
                    },
                    state,
                }
            },
        )))
    }
}
