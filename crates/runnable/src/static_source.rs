//! A source of runnables, based on a static configuration, deserialized from the runnables config file, and related infrastructure for tracking changes to the file.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::HashMap;
use futures::StreamExt;
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::{Runnable, Source, StaticRunnable};
use futures::channel::mpsc::UnboundedReceiver;

/// The source of runnables defined in a runnables config file.
pub struct StaticSource {
    runnables: Vec<StaticRunnable>,
    _definitions: Model<TrackedFile<DefinitionProvider>>,
    _subscription: Subscription,
}

/// Static runnable definition from the runnables config file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct Definition {
    /// Human readable name of the runnable to display in the UI.
    pub label: String,
    /// Executable command to spawn.
    pub command: String,
    /// Arguments to the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Env overrides for the command, will be appended to the terminal's environment from the settings.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Current working directory to spawn the command into, defaults to current project root.
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Whether to use a new terminal tab or reuse the existing one to spawn the process.
    #[serde(default)]
    pub use_new_terminal: bool,
    /// Whether to allow multiple instances of the same runnable to be run, or rather wait for the existing ones to finish.
    #[serde(default)]
    pub allow_concurrent_runs: bool,
}

/// A group of Runnables defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DefinitionProvider {
    version: String,
    runnables: Vec<Definition>,
}

impl DefinitionProvider {
    /// Generates JSON schema of Runnables JSON definition format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}
/// A Wrapper around deserializable T that keeps track of it's contents
/// via a provided channel. Once T value changes, the observers of [`TrackedFile`] are
/// notified.
struct TrackedFile<T> {
    parsed_contents: T,
}

impl<T: for<'a> Deserialize<'a> + PartialEq + 'static> TrackedFile<T> {
    fn new(
        parsed_contents: T,
        mut tracker: UnboundedReceiver<String>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(move |cx| {
            cx.spawn(|tracked_file, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    let Some(new_contents) = serde_json_lenient::from_str(&new_contents).log_err()
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
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            Self { parsed_contents }
        })
    }

    fn get(&self) -> &T {
        &self.parsed_contents
    }
}

impl StaticSource {
    /// Initializes the static source, reacting on runnables config changes.
    pub fn new(
        runnables_file_tracker: UnboundedReceiver<String>,
        cx: &mut AppContext,
    ) -> Model<Box<dyn Source>> {
        let definitions =
            TrackedFile::new(DefinitionProvider::default(), runnables_file_tracker, cx);
        cx.new_model(|cx| {
            let _subscription = cx.observe(
                &definitions,
                |source: &mut Box<(dyn Source + 'static)>, new_definitions, cx| {
                    if let Some(static_source) = source.as_any().downcast_mut::<Self>() {
                        static_source.runnables = new_definitions
                            .read(cx)
                            .get()
                            .runnables
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(id, definition)| StaticRunnable::new(id, definition))
                            .collect();
                        cx.notify();
                    }
                },
            );
            Box::new(Self {
                runnables: Vec::new(),
                _definitions: definitions,
                _subscription,
            })
        })
    }
}

impl Source for StaticSource {
    fn runnables_for_path(
        &mut self,
        _: Option<&Path>,
        _: &mut ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn Runnable>> {
        self.runnables
            .clone()
            .into_iter()
            .map(|runnable| Arc::new(runnable) as Arc<dyn Runnable>)
            .collect()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
