use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use settings::SettingsStore;

use crate::{
    next_source_id, static_runnable_file::RunnableProvider, RunState, Runnable, Source, SourceId,
    StaticRunner,
};

pub struct StaticSource {
    id: SourceId,
    definitions: Model<RunnableProvider>,
    _settings_changed_subscription: Subscription,
}

impl StaticSource {
    pub fn new(contents: Model<RunnableProvider>, cx: &mut AppContext) -> Self {
        let definitions = cx.new_model(|cx| RunnableProvider::default());

        let _settings_changed_subscription = definitions.update(cx, |_, cx| {
            cx.observe_global::<SettingsStore>(|runnables, cx| {
                on_settings_changed(runnables, cx);
            })
        });
        Self {
            id: next_source_id(),
            definitions, // TODO kb use Option instead?
            _settings_changed_subscription,
        }
    }
}

fn on_settings_changed(runnables: &mut RunnableProvider, cx: &mut ModelContext<RunnableProvider>) {
    // TODO kb change contents of the static runnable provider, if needed, blah.
    // self.definitions.update(cx, |this, cx| {
    //     update(this);
    // });
    // let new_settings = RunnableSettings::get_blobal(cx);
    // if self.previous_runnables_config != new_settings { reinit_static_source() }
    todo!()
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
