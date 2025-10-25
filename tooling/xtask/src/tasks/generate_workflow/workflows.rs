use gh_workflow::*;

use super::{runners, steps};

/// Generates the danger.yml workflow
pub fn danger() -> Workflow {
    Workflow::default()
        .name("Danger")
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
                .add_step(steps::danger::setup_node())
                .add_step(steps::danger::install_deps())
                .add_step(steps::danger::run()),
        )
}
