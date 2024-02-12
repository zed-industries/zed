use std::sync::Arc;

use futures::StreamExt;
use gpui::{AppContext, Context, Model, Subscription};
use serde::Deserialize;
use util::ResultExt;

use crate::{
    static_runnable_file::RunnableProvider, RunState, Runnable, RunnableToken, Source, StaticRunner,
};
use futures::channel::mpsc::UnboundedReceiver;

pub struct StaticSource {
    // This is gonna come into play later once we tackle handling multiple instances of a single runnable (spawning multiple runnables from a single static runnable definition).
    #[allow(unused)]
    definitions: Model<TrackedFile<RunnableProvider>>,
    runnables: Vec<RunnableToken>,
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
    ) -> Model<Box<dyn Source>> {
        cx.new_model(|cx| {
            let _subscription = cx.observe(&definitions, |this, new_definitions, cx| {
                let tasks = new_definitions.read(cx).get().tasks.clone();
                let runnables = tasks
                    .into_iter()
                    .map(|task| {
                        let runner = StaticRunner::new(task.clone());
                        let display_name = runner.name();
                        let source = cx.weak_model();
                        let state = cx.new_model(|_| RunState::NotScheduled(Arc::new(runner)));
                        crate::RunnableToken {
                            metadata: Arc::new(crate::RunnableMetadata {
                                source,
                                display_name,
                            }),
                            state,
                        }
                    })
                    .collect();
                let this: Option<&mut Self> = this.as_any().downcast_mut();

                if let Some(this) = this {
                    this.runnables = runnables;
                    cx.notify();
                }
            });
            Box::new(Self {
                definitions, // TODO kb use Option instead?
                runnables: vec![],
                _subscription,
            })
        })
    }
}

impl Source for StaticSource {
    fn runnables_for_path<'a>(
        &'a self,
        _: &std::path::Path,
        _cx: &'a AppContext,
    ) -> anyhow::Result<Box<dyn Iterator<Item = crate::RunnableToken> + 'a>> {
        Ok(Box::new(self.runnables.iter().cloned()))
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
