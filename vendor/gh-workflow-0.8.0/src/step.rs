//!
//! Step-related structures and implementations for GitHub workflow steps.

use std::time::Duration;

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::toolchain::{Abi, Arch, Component, System, Target, Toolchain, Vendor, Version};
use crate::{private, Artifacts, Env, Expression, RetryStrategy};

/// Represents a step in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Step<A> {
    /// The value of the step.
    pub value: StepValue,
    #[serde(skip)]
    pub marker: A,
}

impl From<Step<Run>> for StepValue {
    /// Converts a `Step<Run>` into a `StepValue`.
    fn from(step: Step<Run>) -> Self {
        step.value
    }
}

impl From<Step<Use>> for StepValue {
    /// Converts a `Step<Use>` into a `StepValue`.
    fn from(step: Step<Use>) -> Self {
        step.value
    }
}

/// Represents a step that uses an action.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Use;

/// Represents a step that runs a command.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Run;

/// A trait to convert `Step<Run>` and `Step<Use>` to `StepValue`.
pub trait StepType: Sized + private::Sealed {
    /// Converts a step to its value representation.
    fn to_value(s: Step<Self>) -> StepValue;
}

impl private::Sealed for Run {}
impl private::Sealed for Use {}

impl StepType for Run {
    /// Converts a `Step<Run>` to `StepValue`.
    fn to_value(s: Step<Self>) -> StepValue {
        s.into()
    }
}

impl StepType for Use {
    /// Converts a `Step<Use>` to `StepValue`.
    fn to_value(s: Step<Self>) -> StepValue {
        s.into()
    }
}

/// Represents input parameters for a step.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Input(#[serde(skip_serializing_if = "IndexMap::is_empty")] pub IndexMap<String, Value>);

impl From<IndexMap<String, Value>> for Input {
    /// Converts an `IndexMap` into an `Input`.
    fn from(value: IndexMap<String, Value>) -> Self {
        Input(value)
    }
}

impl Merge for Input {
    /// Merges another `Input` into this one.
    fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl Input {
    /// Adds a new input parameter to the `Input`.
    pub fn add<S: ToString, V: Into<Value>>(mut self, key: S, value: V) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }

    /// Checks if the `Input` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Represents a step value in the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Merge)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Step<Run>", field = "value"),
    generate_delegates(ty = "Step<Use>", field = "value")
)]
pub struct StepValue {
    /// The ID of the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The condition under which the step runs.
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,

    /// The action to use in the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub uses: Option<String>,

    /// Input parameters for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Input>,

    /// The command to run in the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub run: Option<String>,

    /// Shell to run with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// Environment variables for the step.
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    pub envs: Option<Env>,

    /// The timeout for the step in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,

    /// Whether to continue on error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,

    /// The working directory for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,

    /// The retry strategy for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,

    /// Artifacts produced by the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl StepValue {
    /// Creates a new `StepValue` that runs the provided shell command.
    pub fn run<T: ToString>(cmd: T) -> Self {
        StepValue { run: Some(cmd.to_string()), ..Default::default() }
    }

    /// Creates a new `StepValue` that uses an action.
    pub fn uses<Owner: ToString, Repo: ToString, Version: ToString>(
        owner: Owner,
        repo: Repo,
        version: Version,
    ) -> Self {
        StepValue {
            uses: Some(format!(
                "{}/{}@{}",
                owner.to_string(),
                repo.to_string(),
                version.to_string()
            )),
            ..Default::default()
        }
    }
}

/// Represents a step in the workflow.
impl<T> Step<T> {
    /// Adds an environment variable to the step.
    pub fn add_env<R: Into<Env>>(mut self, new_env: R) -> Self {
        let mut env = self.value.envs.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.value.envs = Some(env);
        self
    }

    /// Sets the timeout for the job.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.value = self.value.timeout_minutes(duration.as_secs() as u32 / 60);
        self
    }
}

impl Step<()> {
    pub fn new(name: impl ToString) -> Self {
        Step {
            value: StepValue::default().name(name.to_string()),
            marker: Default::default(),
        }
    }

    pub fn uses<Owner: ToString, Repo: ToString, Version: ToString>(
        mut self,
        owner: Owner,
        repo: Repo,
        version: Version,
    ) -> Step<Use> {
        self.value.merge(StepValue::uses(owner, repo, version));
        Step { value: self.value, marker: Default::default() }
    }

    pub fn run(mut self, cmd: impl ToString) -> Step<Run> {
        self.value.merge(StepValue::run(cmd));
        Step { value: self.value, marker: Default::default() }
    }
}

/// Represents a step that uses an action.
impl Step<Use> {
    /// Creates a step pointing to the default GitHub's Checkout Action.
    pub fn checkout() -> Step<Use> {
        Step::new("Checkout Code").uses("actions", "checkout", "v5")
    }

    /// Adds a new input to the step.
    pub fn add_with<I: Into<Input>>(mut self, new_with: I) -> Self {
        let mut with = self.value.with.take().unwrap_or_default();
        with.merge(new_with.into());
        if with.0.is_empty() {
            self.value.with = None;
        } else {
            self.value.with = Some(with);
        }

        self
    }
}

/// Represents a key-value pair for inputs.
impl<S1: ToString, S2: Into<Value>> From<(S1, S2)> for Input {
    /// Converts a tuple into an `Input`.
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), value.1.into());
        Input(index_map)
    }
}

impl Step<Toolchain> {
    pub fn toolchain() -> Step<Toolchain> {
        Step { value: Default::default(), marker: Toolchain::default() }
    }

    pub fn add_version(mut self, version: Version) -> Self {
        self.marker.version.push(version);
        self
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.marker.components.push(component);
        self
    }

    pub fn add_stable(mut self) -> Self {
        self.marker.version.push(Version::Stable);
        self
    }

    pub fn add_nightly(mut self) -> Self {
        self.marker.version.push(Version::Nightly);
        self
    }

    pub fn add_clippy(mut self) -> Self {
        self.marker.components.push(Component::Clippy);
        self
    }

    pub fn add_fmt(mut self) -> Self {
        self.marker.components.push(Component::Rustfmt);
        self
    }

    pub fn target(mut self, arch: Arch, vendor: Vendor, system: System, abi: Option<Abi>) -> Self {
        self.marker.target = Some(Target { arch, vendor, system, abi });
        self
    }
}

impl StepType for Toolchain {
    fn to_value(s: Step<Self>) -> StepValue {
        let step: Step<Use> = s.marker.into();
        StepValue::from(step)
    }
}
