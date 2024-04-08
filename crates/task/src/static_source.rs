//! A source of tasks, based on a static configuration, deserialized from the tasks config file, and related infrastructure for tracking changes to the file.

use std::{borrow::Cow, sync::Arc};

use futures::StreamExt;
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use serde::Deserialize;
use util::ResultExt;

use crate::{from_template, Task, TaskId, TaskSource, TaskTemplates};
use futures::channel::mpsc::UnboundedReceiver;

/// The source of tasks defined in a tasks config file.
pub struct StaticSource {
    tasks: Vec<Arc<dyn Task>>,
    _templates: Model<TrackedFile<TaskTemplates>>,
    _subscription: Subscription,
}

/// A Wrapper around deserializable T that keeps track of its contents
/// via a provided channel. Once T value changes, the observers of [`TrackedFile`] are
/// notified.
pub struct TrackedFile<T> {
    parsed_contents: T,
}

impl<T: PartialEq + 'static> TrackedFile<T> {
    /// Initializes new [`TrackedFile`] with a type that's deserializable.
    pub fn new(mut tracker: UnboundedReceiver<String>, cx: &mut AppContext) -> Model<Self>
    where
        T: for<'a> Deserialize<'a> + Default,
    {
        cx.new_model(move |cx| {
            cx.spawn(|tracked_file, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    if !new_contents.trim().is_empty() {
                        // String -> T (ZedTaskFormat)
                        // String -> U (VsCodeFormat) -> Into::into T
                        let Some(new_contents) =
                            serde_json_lenient::from_str(&new_contents).log_err()
                        else {
                            continue;
                        };
                        tracked_file.update(&mut cx, |tracked_file: &mut TrackedFile<T>, cx| {
                            if tracked_file.parsed_contents != new_contents {
                                tracked_file.parsed_contents = new_contents;
                                cx.notify();
                            };
                        })?;
                    }
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            Self {
                parsed_contents: Default::default(),
            }
        })
    }

    /// Initializes new [`TrackedFile`] with a type that's convertible from another deserializable type.
    pub fn new_convertible<U: for<'a> Deserialize<'a> + TryInto<T, Error = anyhow::Error>>(
        mut tracker: UnboundedReceiver<String>,
        cx: &mut AppContext,
    ) -> Model<Self>
    where
        T: Default,
    {
        cx.new_model(move |cx| {
            cx.spawn(|tracked_file, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    if !new_contents.trim().is_empty() {
                        let Some(new_contents) =
                            serde_json_lenient::from_str::<U>(&new_contents).log_err()
                        else {
                            continue;
                        };
                        let Some(new_contents) = new_contents.try_into().log_err() else {
                            continue;
                        };
                        tracked_file.update(&mut cx, |tracked_file: &mut TrackedFile<T>, cx| {
                            if tracked_file.parsed_contents != new_contents {
                                tracked_file.parsed_contents = new_contents;
                                cx.notify();
                            };
                        })?;
                    }
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            Self {
                parsed_contents: Default::default(),
            }
        })
    }

    fn get(&self) -> &T {
        &self.parsed_contents
    }
}

impl StaticSource {
    /// Initializes the static source, reacting on tasks config changes.
    pub fn new(
        id_base: impl Into<Cow<'static, str>>,
        templates: Model<TrackedFile<TaskTemplates>>,
        cx: &mut AppContext,
    ) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|cx| {
            let id_base = id_base.into();
            let _subscription = cx.observe(
                &templates,
                move |source: &mut Box<(dyn TaskSource + 'static)>, new_templates, cx| {
                    if let Some(static_source) = source.as_any().downcast_mut::<Self>() {
                        static_source.tasks = new_templates
                            .read(cx)
                            .get()
                            .0
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, template)| {
                                from_template(
                                    TaskId(format!("static_{id_base}_{i}_{}", template.label)),
                                    template,
                                )
                            })
                            .collect();
                        cx.notify();
                    }
                },
            );
            Box::new(Self {
                tasks: Vec::new(),
                _templates: templates,
                _subscription,
            })
        })
    }
}

impl TaskSource for StaticSource {
    fn tasks_to_schedule(
        &mut self,
        _: &mut ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn Task>> {
        self.tasks
            .iter()
            .map(|task| task.clone() as Arc<dyn Task>)
            .collect()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
