use std::{
    collections::{BTreeSet, HashMap, HashSet},
    sync::Arc,
};

use futures::StreamExt;
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use serde::Deserialize;
use util::ResultExt;

use crate::{
    static_runnable_file::{Definition, RunnableProvider},
    RunState, Runnable, RunnableToken, Source, StaticRunner,
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
                let runnables = new_definitions.read(cx).get().runnables.clone();
                let runnables = runnables
                    .into_iter()
                    .map(|runnable| Self::token_from_definition(runnable, cx))
                    .collect();
                let this: Option<&mut Self> = this.as_any().downcast_mut();

                if let Some(this) = this {
                    this.runnables = runnables;
                    cx.notify();
                }
            });
            Box::new(Self {
                definitions,
                runnables: Vec::new(),
                _subscription,
            })
        })
    }
    fn token_from_definition(
        runnable: Definition,
        cx: &mut ModelContext<Box<dyn Source>>,
    ) -> crate::RunnableToken {
        let runner = StaticRunner::new(runnable.clone());
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
    }
}

impl Source for StaticSource {
    fn runnables_for_path(
        &mut self,
        _: &std::path::Path,
        cx: &mut ModelContext<Box<dyn Source>>,
    ) -> anyhow::Result<Vec<RunnableToken>> {
        let mut known_definitions: HashMap<String, _> = self
            .definitions
            .read(cx)
            .parsed_contents
            .runnables
            .iter()
            .cloned()
            .map(|meta| (meta.label.clone(), meta))
            .collect();

        // Refill runnables.
        for runnable in &self.runnables {
            if !runnable.was_scheduled(cx) {
                known_definitions.remove(runnable.metadata.display_name());
            }
        }
        for (_, meta) in known_definitions {
            self.runnables.push(Self::token_from_definition(meta, cx));
        }
        Ok(self.runnables.clone())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
