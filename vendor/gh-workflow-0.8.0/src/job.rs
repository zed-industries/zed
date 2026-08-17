//!
//! Job-related structures and implementations for GitHub workflow jobs.

use std::any::Any;
use std::time::Duration;

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::concurrency::Concurrency;
use crate::step::{Step, StepType, StepValue};
use crate::{
    private, Artifacts, Container, Defaults, Env, Expression, Input, Permissions, RetryStrategy,
    Strategy,
};

/// Represents the environment in which a job runs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct RunsOn(Value);

impl<T> From<T> for RunsOn
where
    T: Into<Value>,
{
    /// Converts a value into a `RunsOn` instance.
    fn from(value: T) -> Self {
        RunsOn(value.into())
    }
}

/// Represents input parameters for a step.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Secrets(
    #[serde(skip_serializing_if = "IndexMap::is_empty")] pub IndexMap<String, String>,
);

impl From<IndexMap<String, String>> for Secrets {
    /// Converts an `IndexMap` into an `Input`.
    fn from(value: IndexMap<String, String>) -> Self {
        Secrets(value)
    }
}

impl Merge for Secrets {
    /// Merges another `Input` into this one.
    fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl Secrets {
    /// Adds a new input parameter to the `Input`.
    pub fn add<S: ToString, V: ToString>(mut self, key: S, value: V) -> Self {
        self.0.insert(key.to_string(), value.to_string());
        self
    }

    /// Checks if the `Input` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub trait JobType: Any + Default + private::Sealed {
    fn to_value(j: Job<Self>) -> JobValue;
}

#[derive(Debug, Clone, Default, Setters, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<RunJob>", field = "config")
)]
pub struct RunJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    pub envs: Option<Env>,
}

#[derive(Debug, Clone, Default, Setters, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<UsesJob>", field = "config")
)]
pub struct UsesJob {
    #[setters(skip)]
    pub uses: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
}

impl private::Sealed for RunJob {}
impl private::Sealed for UsesJob {}

impl JobType for RunJob {
    fn to_value(j: Job<Self>) -> JobValue {
        let RunJob { steps, envs } = j.config;
        JobValue { steps, envs, ..j.value }
    }
}

impl JobType for UsesJob {
    fn to_value(j: Job<Self>) -> JobValue {
        let UsesJob { uses, with, secrets } = j.config;
        JobValue { uses: Some(uses), with, secrets, ..j.value }
    }
}

/// Represents a job in the workflow.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job<J: JobType = RunJob> {
    #[serde(flatten)]
    config: J,
    #[serde(flatten)]
    value: JobValue,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<T>", generics = "<T: JobType>", field = "value")
)]
pub struct JobValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub cond: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    #[setters(skip)]
    pub envs: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub steps: Option<Vec<StepValue>>,
    #[setters(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub with: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Job {
    /// Creates a new `Job` that uses a reusable workflow.
    pub fn uses<Owner: ToString, Repo: ToString, Path: ToString, Version: ToString>(
        self,
        owner: Owner,
        repo: Repo,
        path: Path,
        version: Version,
    ) -> Job<UsesJob> {
        Job {
            config: UsesJob {
                uses: format!(
                    "{}/{}/{}@{}",
                    owner.to_string(),
                    repo.to_string(),
                    path.to_string(),
                    version.to_string()
                ),
                ..Default::default()
            },
            value: self.value,
        }
    }

    /// Creates a new `Job` that uses a local reusable workflow.
    pub fn uses_local<Path: ToString>(self, path: Path) -> Job<UsesJob> {
        Job {
            config: UsesJob {
                uses: {
                    let path = path.to_string();
                    if path.starts_with("./") {
                        path
                    } else {
                        format!("./{path}")
                    }
                },
                ..Default::default()
            },
            value: self.value,
        }
    }
}

impl<J: JobType> Job<J> {
    /// Creates a new `Job` with the specified name and default settings.
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            value: JobValue {
                name: Some(name.to_string()),
                runs_on: Some(RunsOn(Value::from("ubuntu-latest"))),
                ..Default::default()
            },

            ..Default::default()
        }
    }

    /// Sets the timeout for the job.
    pub fn timeout(self, duration: Duration) -> Self {
        Self {
            value: self.value.timeout_minutes(duration.as_secs() as u32 / 60),
            ..self
        }
    }

    pub fn add_need<T: ToString>(mut self, job_id: T) -> Self {
        if let Some(needs) = self.value.needs.as_mut() {
            needs.push(job_id.to_string());
        } else {
            self.value.needs = Some(vec![job_id.to_string()]);
        }
        self
    }
}

impl Job<RunJob> {
    /// Adds a step to the job.
    pub fn add_step<S: Into<Step<T>>, T: StepType>(mut self, step: S) -> Self {
        let mut steps = self.config.steps.take().unwrap_or_default();
        let step: Step<T> = step.into();
        let step: StepValue = T::to_value(step);
        steps.push(step);
        self.config.steps = Some(steps);
        self
    }

    /// Adds an environment variable to the job.
    pub fn add_env<E: Into<Env>>(mut self, new_env: E) -> Self {
        let mut env = self.config.envs.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.config.envs = Some(env);
        self
    }

    /// Adds a service container to the job.
    pub fn add_service<S: ToString>(mut self, name: S, container: Container) -> Self {
        let mut services = self.value.services.take().unwrap_or_default();
        services.insert(name.to_string(), container);
        self.value.services = Some(services);
        self
    }
}
