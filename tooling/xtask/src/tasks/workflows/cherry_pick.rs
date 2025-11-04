use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named},
    vars::Input,
};

pub fn cherry_pick() -> Workflow {
    let commit = Input::string("commit", None);
    let base = Input::string("base", None);
    let pr_no = Input::string("pr_no", None);
    let cherry_pick = run_cherry_pick(&commit, &base, &pr_no);
    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(commit.name, commit.input())
                .add_input(base.name, base.input())
                .add_input(pr_no.name, pr_no.input()),
        ))
        .add_job(cherry_pick.name, cherry_pick.job)
}

fn run_cherry_pick(commit: &Input, base: &Input, pr_no: &Input) -> NamedJob {
    fn new_branch(base: &str, commit: &str) -> Step<Run> {
        // These are the commit SHAs, so always ascii and >8 chars.
        let commit = &commit[0..8];
        named::bash(&format!("git checkout -b cherry-pick-{base}-{commit}"))
    }

    fn cherry_remote(commit: &str) -> Step<Run> {
        named::bash(&format!("git fetch {commit} && git cherry-pick {commit}"))
    }

    fn make_pr(base: &str, pr_no: &str) -> Step<Run> {
        named::bash(&format!(
            "COMMIT=$(git log --pretty=format:%s -n 1) gh pr new --head --base {base} --title \"Cherry-pick \\\"$COMMIT\\\" to {base}\" --body \"PR #{pr_no}\" && gh pr merge --auto"
        ))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(steps::git_checkout(&base.var()))
            .add_step(new_branch(&base.var(), &commit.var()))
            .add_step(cherry_remote(&commit.var()))
            .add_step(
                make_pr(&base.var(), &pr_no.var())
                    .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")),
            ),
    )
}
