//! A type-safe implementation of workflow context: <https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs>

use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use gh_workflow_macros::Context;

use crate::Expression;

#[derive(Clone)]
pub struct Context<A> {
    marker: PhantomData<A>,
    step: Step,
}

#[derive(Default, Clone)]
enum Step {
    #[default]
    Root,
    Select {
        name: Rc<String>,
        object: Box<Step>,
    },
    Eq {
        left: Box<Step>,
        right: Box<Step>,
    },
    And {
        left: Box<Step>,
        right: Box<Step>,
    },
    Or {
        left: Box<Step>,
        right: Box<Step>,
    },
    Literal(String),
    Concat {
        left: Box<Step>,
        right: Box<Step>,
    },
}

impl<A> Context<A> {
    fn new() -> Self {
        Context { marker: PhantomData, step: Step::Root }
    }

    fn select<B>(&self, path: impl Into<String>) -> Context<B> {
        Context {
            marker: PhantomData,
            step: Step::Select {
                name: Rc::new(path.into()),
                object: Box::new(self.step.clone()),
            },
        }
    }

    pub fn eq(&self, other: Context<A>) -> Context<bool> {
        Context {
            marker: Default::default(),
            step: Step::Eq {
                left: Box::new(self.step.clone()),
                right: Box::new(other.step.clone()),
            },
        }
    }

    pub fn and(&self, other: Context<A>) -> Context<bool> {
        Context {
            marker: Default::default(),
            step: Step::And {
                left: Box::new(self.step.clone()),
                right: Box::new(other.step.clone()),
            },
        }
    }

    pub fn or(&self, other: Context<A>) -> Context<bool> {
        Context {
            marker: Default::default(),
            step: Step::Or {
                left: Box::new(self.step.clone()),
                right: Box::new(other.step.clone()),
            },
        }
    }
}

impl Context<String> {
    pub fn concat(&self, other: Context<String>) -> Context<String> {
        Context {
            marker: Default::default(),
            step: Step::Concat {
                left: Box::new(self.step.clone()),
                right: Box::new(other.step),
            },
        }
    }
}

#[allow(unused)]
#[derive(Context)]
pub struct Github {
    /// The name of the action currently running, or the id of a step.
    action: String,
    /// The path where an action is located. This property is only supported in
    /// composite actions.
    action_path: String,
    /// For a step executing an action, this is the ref of the action being
    /// executed.
    action_ref: String,
    /// For a step executing an action, this is the owner and repository name of
    /// the action.
    action_repository: String,
    /// For a composite action, the current result of the composite action.
    action_status: String,
    /// The username of the user that triggered the initial workflow run.
    actor: String,
    /// The account ID of the person or app that triggered the initial workflow
    /// run.
    actor_id: String,
    /// The URL of the GitHub REST API.
    api_url: String,
    /// The base_ref or target branch of the pull request in a workflow run.
    base_ref: String,
    /// Path on the runner to the file that sets environment variables from
    /// workflow commands.
    env: String,
    /// The full event webhook payload.
    event: serde_json::Value,
    /// The name of the event that triggered the workflow run.
    event_name: String,
    /// The path to the file on the runner that contains the full event webhook
    /// payload.
    event_path: String,
    /// The URL of the GitHub GraphQL API.
    graphql_url: String,
    /// The head_ref or source branch of the pull request in a workflow run.
    head_ref: String,
    /// The job id of the current job.
    job: String,
    /// The path of the repository.
    path: String,
    /// The short ref name of the branch or tag that triggered the workflow run.
    ref_name: String,
    /// true if branch protections are configured for the ref that triggered the
    /// workflow run.
    ref_protected: bool,
    /// The type of ref that triggered the workflow run. Valid values are branch
    /// or tag.
    ref_type: String,
    /// The owner and repository name.
    repository: String,
    /// The ID of the repository.
    repository_id: String,
    /// The repository owner's username.
    repository_owner: String,
    /// The repository owner's account ID.
    repository_owner_id: String,
    /// The Git URL to the repository.
    repository_url: String,
    /// The number of days that workflow run logs and artifacts are kept.
    retention_days: String,
    /// A unique number for each workflow run within a repository.
    run_id: String,
    /// A unique number for each run of a particular workflow in a repository.
    run_number: String,
    /// A unique number for each attempt of a particular workflow run in a
    /// repository.
    run_attempt: String,
    /// The source of a secret used in a workflow.
    secret_source: String,
    /// The URL of the GitHub server.
    server_url: String,
    /// The commit SHA that triggered the workflow.
    sha: String,
    /// A token to authenticate on behalf of the GitHub App installed on your
    /// repository.
    token: String,
    /// The username of the user that initiated the workflow run.
    triggering_actor: String,
    /// The name of the workflow.
    workflow: String,
    /// The ref path to the workflow.
    workflow_ref: String,
    /// The commit SHA for the workflow file.
    workflow_sha: String,
    /// The default working directory on the runner for steps.
    workspace: String,
}

impl Context<Github> {
    pub fn ref_(&self) -> Context<String> {
        self.select("ref")
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::Root => write!(f, ""),
            Step::Select { name, object } => {
                if matches!(**object, Step::Root) {
                    write!(f, "{name}")
                } else {
                    write!(f, "{object}.{name}")
                }
            }
            Step::Eq { left, right } => {
                write!(f, "{left} == {right}")
            }
            Step::And { left, right } => {
                write!(f, "{left} && {right}")
            }
            Step::Or { left, right } => {
                write!(f, "{left} || {right}")
            }
            Step::Literal(value) => {
                write!(f, "'{value}'")
            }
            Step::Concat { left, right } => {
                write!(f, "{left}{right}")
            }
        }
    }
}

impl<A> fmt::Display for Context<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ {} }}}}", self.step.to_string().replace('"', ""))
    }
}

impl<A> From<Context<A>> for Expression {
    fn from(value: Context<A>) -> Self {
        Expression::new(value.to_string())
    }
}

impl<T: Into<String>> From<T> for Context<String> {
    fn from(value: T) -> Self {
        Context {
            marker: Default::default(),
            step: Step::Literal(value.into()),
        }
    }
}

#[allow(unused)]
#[derive(Context)]
/// The job context contains information about the currently running job.
pub struct Job {
    /// A unique number for each container in a job. This property is only
    /// available if the job uses a container.
    container: Container,

    /// The services configured for a job. This property is only available if
    /// the job uses service containers.
    services: Services,

    /// The status of the current job.
    status: JobStatus,
}

/// The status of a job execution
#[derive(Clone)]
pub enum JobStatus {
    /// The job completed successfully
    Success,
    /// The job failed
    Failure,
    /// The job was cancelled
    Cancelled,
}

#[derive(Context)]
#[allow(unused)]
/// Container information for a job. This is only available if the job runs in a
/// container.
pub struct Container {
    /// The ID of the container
    id: String,
    /// The container network
    network: String,
}

#[derive(Context)]

/// Services configured for a job. This is only available if the job uses
/// service containers.
pub struct Services {}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_expr() {
        let github = Context::github(); // Expr<Github>

        assert_eq!(github.to_string(), "${{ github }}");

        let action = github.action(); // Expr<String>
        assert_eq!(action.to_string(), "${{ github.action }}");

        let action_path = github.action_path(); // Expr<String>
        assert_eq!(action_path.to_string(), "${{ github.action_path }}");
    }

    #[test]
    fn test_expr_eq() {
        let github = Context::github();
        let action = github.action();
        let action_path = github.action_path();

        let expr = action.eq(action_path);

        assert_eq!(
            expr.to_string(),
            "${{ github.action == github.action_path }}"
        );
    }

    #[test]
    fn test_expr_and() {
        let push = Context::github().event_name().eq("push".into());
        let main = Context::github().ref_().eq("ref/heads/main".into());
        let expr = push.and(main);

        assert_eq!(
            expr.to_string(),
            "${{ github.event_name == 'push' && github.ref == 'ref/heads/main' }}"
        )
    }

    #[test]
    fn test_expr_or() {
        let github = Context::github();
        let action = github.action();
        let action_path = github.action_path();
        let action_ref = github.action_ref();

        let expr = action.eq(action_path).or(action.eq(action_ref));

        assert_eq!(
            expr.to_string(),
            "${{ github.action == github.action_path || github.action == github.action_ref }}"
        );
    }
}
