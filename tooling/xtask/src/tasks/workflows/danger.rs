use gh_workflow::*;

use crate::tasks::workflows::steps::named;

use super::{runners, steps};

/// Generates the danger.yml workflow
pub fn danger() -> Workflow {
    named::workflow()
        .on(
            Event::default().pull_request(PullRequest::default().add_branch("main").types([
                PullRequestType::Opened,
                PullRequestType::Synchronize,
                PullRequestType::Reopened,
                PullRequestType::Edited,
            ])),
        )
        .add_job(
            "danger",
            Job::default()
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                ))
                .runs_on(runners::LINUX_CHEAP)
                .add_step(steps::checkout_repo())
                .add_step(steps::setup_pnpm())
                .add_step(
                    steps::setup_node()
                        .add_with(("cache", "pnpm"))
                        .add_with(("cache-dependency-path", "script/danger/pnpm-lock.yaml")),
                )
                .add_step(install_deps())
                .add_step(run()),
        )
}

pub fn install_deps() -> Step<Run> {
    named::bash("pnpm install --dir script/danger")
}

pub fn run() -> Step<Run> {
    named::bash("pnpm run --dir script/danger danger ci")
        // This GitHub token is not used, but the value needs to be here to prevent
        // Danger from throwing an error.
        .add_env(("GITHUB_TOKEN", "not_a_real_token"))
        // All requests are instead proxied through an instance of
        // https://github.com/maxdeviant/danger-proxy that allows Danger to securely
        // authenticate with GitHub while still being able to run on PRs from forks.
        .add_env((
            "DANGER_GITHUB_API_BASE_URL",
            "https://danger-proxy.fly.dev/github",
        ))
}
