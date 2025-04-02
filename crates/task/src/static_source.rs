//! A source of tasks, based on a static configuration, deserialized from the tasks config file, and related infrastructure for tracking changes to the file.

use std::sync::Arc;

use futures::{StreamExt, channel::mpsc::UnboundedSender};
use gpui::{App, AppContext};
use parking_lot::RwLock;
use serde::Deserialize;
use util::ResultExt;

use crate::TaskTemplates;
use futures::channel::mpsc::UnboundedReceiver;

/// The source of tasks defined in a tasks config file.
pub struct StaticSource {
    tasks: TrackedFile<TaskTemplates>,
}

/// A Wrapper around deserializable T that keeps track of its contents
/// via a provided channel.
pub struct TrackedFile<T> {
    parsed_contents: Arc<RwLock<T>>,
}

impl<T: PartialEq + 'static + Sync> TrackedFile<T> {
    /// Initializes new [`TrackedFile`] with a type that's deserializable.
    pub fn new(
        mut tracker: UnboundedReceiver<String>,
        notification_outlet: UnboundedSender<()>,
        cx: &App,
    ) -> Self
    where
        T: for<'a> Deserialize<'a> + Default + Send,
    {
        let parsed_contents: Arc<RwLock<T>> = Arc::default();
        cx.background_spawn({
            let parsed_contents = parsed_contents.clone();
            async move {
                while let Some(new_contents) = tracker.next().await {
                    if Arc::strong_count(&parsed_contents) == 1 {
                        // We're no longer being observed. Stop polling.
                        break;
                    }
                    if !new_contents.trim().is_empty() {
                        let Some(new_contents) =
                            serde_json_lenient::from_str::<T>(&new_contents).log_err()
                        else {
                            continue;
                        };
                        let mut contents = parsed_contents.write();
                        if *contents != new_contents {
                            *contents = new_contents;
                            if notification_outlet.unbounded_send(()).is_err() {
                                // Whoever cared about contents is not around anymore.
                                break;
                            }
                        }
                    }
                }
                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);
        Self { parsed_contents }
    }

    /// Initializes new [`TrackedFile`] with a type that's convertible from another deserializable type.
    pub fn new_convertible<U: for<'a> Deserialize<'a> + TryInto<T, Error = anyhow::Error>>(
        mut tracker: UnboundedReceiver<String>,
        notification_outlet: UnboundedSender<()>,
        cx: &App,
    ) -> Self
    where
        T: Default + Send,
    {
        let parsed_contents: Arc<RwLock<T>> = Arc::default();
        cx.background_spawn({
            let parsed_contents = parsed_contents.clone();
            async move {
                while let Some(new_contents) = tracker.next().await {
                    if Arc::strong_count(&parsed_contents) == 1 {
                        // We're no longer being observed. Stop polling.
                        break;
                    }

                    if !new_contents.trim().is_empty() {
                        let Some(new_contents) =
                            serde_json_lenient::from_str::<U>(&new_contents).log_err()
                        else {
                            continue;
                        };
                        let Some(new_contents) = new_contents.try_into().log_err() else {
                            continue;
                        };
                        let mut contents = parsed_contents.write();
                        if *contents != new_contents {
                            *contents = new_contents;
                            if notification_outlet.unbounded_send(()).is_err() {
                                // Whoever cared about contents is not around anymore.
                                break;
                            }
                        }
                    }
                }
                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);
        Self {
            parsed_contents: Default::default(),
        }
    }
}

impl StaticSource {
    /// Initializes the static source, reacting on tasks config changes.
    pub fn new(tasks: TrackedFile<TaskTemplates>) -> Self {
        Self { tasks }
    }
    /// Returns current list of tasks
    pub fn tasks_to_schedule(&self) -> TaskTemplates {
        self.tasks.parsed_contents.read().clone()
    }
}
