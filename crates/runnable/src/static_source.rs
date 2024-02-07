use std::sync::Arc;

use futures::StreamExt;
use gpui::{AppContext, Context, Model, Subscription};
use serde::Deserialize;
use util::ResultExt;

use crate::{
    next_source_id, static_runnable_file::RunnableProvider, RunState, Runnable, RunnablePebble,
    Source, SourceId, StaticRunner,
};
use futures::channel::mpsc::UnboundedReceiver;

pub struct StaticSource {
    id: SourceId,
    // This is gonna come into play later once we tackle handling multiple instances of a single runnable (spawning multiple runnables from a single static runnable definition).
    #[allow(unused)]
    definitions: Model<TrackedFile<RunnableProvider>>,
    runnables: Vec<RunnablePebble>,
    _subscription: Subscription,
}

/// A Wrapper around deserializable T that keeps track of it's contents
/// via a provided channel. Once T value changes, the observers of TrackedFile are
/// notified.
pub struct TrackedFile<T> {
    parsed_contents: T,
}

impl<T: for<'a> Deserialize<'a> + PartialEq + 'static> TrackedFile<T> {
    pub fn new(
        initial_contents: T,
        mut tracker: UnboundedReceiver<String>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(move |cx| {
            cx.spawn(|this, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    let Some(new_contents) = serde_json::from_str(&new_contents).log_err() else {
                        continue;
                    };
                    this.update(&mut cx, |this: &mut TrackedFile<T>, cx| {
                        if this.parsed_contents != new_contents {
                            this.parsed_contents = new_contents;
                            cx.notify();
                        };
                    })?;
                }
                Result::<_, anyhow::Error>::Ok(())
            })
            .detach_and_log_err(cx);
            Self {
                parsed_contents: initial_contents,
            }
        })
    }

    fn get(&self) -> &T {
        &self.parsed_contents
    }
}

impl StaticSource {
    pub fn new(
        definitions: Model<TrackedFile<RunnableProvider>>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| {
            let _subscription = cx.observe(&definitions, |this: &mut Self, new_definitions, cx| {
                let tasks = new_definitions.read(cx).get().tasks.clone();
                let runnables = tasks
                    .into_iter()
                    .map(|task| {
                        let source_id = this.id;
                        let runner = StaticRunner::new(task.clone());
                        let display_name = runner.name();
                        let runnable_id = runner.id();
                        let state = cx.new_model(|_| RunState::NotScheduled(Arc::new(runner)));
                        crate::RunnablePebble {
                            metadata: Arc::new(crate::RunnableLens {
                                source_id,
                                runnable_id,
                                display_name,
                            }),
                            state,
                        }
                    })
                    .collect();
                this.runnables = runnables;
                cx.notify();
            });
            Self {
                id: next_source_id(),
                definitions, // TODO kb use Option instead?
                runnables: vec![],
                _subscription,
            }
        })
    }
}

impl Source for Model<StaticSource> {
    fn id(&self, cx: &AppContext) -> crate::SourceId {
        self.read(cx).id
    }

    fn runnables_for_path<'a>(
        &'a self,
        _: &std::path::Path,
        cx: &'a AppContext,
    ) -> anyhow::Result<Box<dyn Iterator<Item = crate::RunnablePebble> + 'a>> {
        Ok(Box::new(self.read(cx).runnables.iter().cloned()))
    }
}
