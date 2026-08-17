//!
//! The serde representation of Github Actions Workflow.

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};

use crate::concurrency::Concurrency;
use crate::defaults::Defaults;
// Import the moved types
use crate::env::Env;
use crate::error::Result;
use crate::generate::Generate;
use crate::job::Job;
use crate::permissions::Permissions;
use crate::secret::Secret;
use crate::{Event, JobType, JobValue};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Jobs(pub(crate) IndexMap<String, JobValue>);
impl Jobs {
    pub fn add<J: Into<Job<T>>, T: JobType>(mut self, key: String, value: J) -> Self {
        let job: Job<T> = value.into();
        let job: JobValue = T::to_value(job);
        self.0.insert(key, job);
        self
    }

    /// Gets a reference to a job by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the job to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(&Job)` if the job exists, `None` otherwise.
    pub fn get(&self, key: &str) -> Option<&JobValue> {
        self.0.get(key)
    }
}

/// Represents the configuration for a GitHub workflow.
///
/// A workflow is a configurable automated process made up of one or more jobs.
/// This struct defines the properties that can be set in a workflow YAML file
/// for GitHub Actions, including the name, environment variables, permissions,
/// jobs, concurrency settings, and more.
#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Workflow {
    /// The name of the workflow. GitHub displays the names of your workflows
    /// under your repository's "Actions" tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Environment variables that can be used in the workflow.
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    pub envs: Option<Env>,

    /// The name for workflow runs generated from the workflow.
    /// GitHub displays the workflow run name in the list of workflow runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,

    /// The event that triggers the workflow. This can include events like
    /// `push`, `pull_request`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<Event>,

    /// Permissions granted to the `GITHUB_TOKEN` for the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    /// The jobs that are defined in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Jobs>,

    /// Concurrency settings for the workflow, allowing control over
    /// how jobs are executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,

    /// Default settings for jobs in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,

    /// Secrets that can be used in the workflow, such as tokens or passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,

    /// The maximum number of minutes a job can run before it is canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

/// Represents an action that can be triggered by an event in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    /// A list of branches that trigger the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    /// A list of branches that are ignored for the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    /// Creates a new `Workflow` with the specified name.
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: Some(name.to_string()), ..Default::default() }
    }

    /// Converts the `Workflow` to a YAML string representation.
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Adds a job to the workflow with the specified ID and job configuration.
    pub fn add_job<I: ToString, J: Into<Job<T>>, T: JobType>(mut self, id: I, job: J) -> Self {
        let key = id.to_string();
        let jobs = self.jobs.take().unwrap_or_default().add(key, job.into());

        self.jobs = Some(jobs);
        self
    }

    /// Parses a YAML string into a `Workflow`.
    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    /// Generates the workflow using the `Generate` struct.
    pub fn generate(self) -> Result<()> {
        Generate::new(self).generate()
    }

    /// Adds an event to the workflow.
    pub fn add_event<T: Into<Event>>(mut self, that: T) -> Self {
        if let Some(mut this) = self.on.take() {
            this.merge(that.into());
            self.on = Some(this);
        } else {
            self.on = Some(that.into());
        }
        self
    }

    /// Adds an environment variable to the workflow.
    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.envs.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.envs = Some(env);
        self
    }
}
