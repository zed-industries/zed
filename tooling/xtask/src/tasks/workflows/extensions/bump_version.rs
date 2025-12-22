use gh_workflow::{
    Event, Expression, Input, Job, PullRequest, PullRequestType, Push, Run, Step, UsesJob,
    Workflow, WorkflowDispatch,
};
use indexmap::IndexMap;
use indoc::indoc;

use crate::tasks::workflows::{
    runners,
    steps::{NamedJob, named},
    vars::{self, JobOutput, StepOutput, one_workflow_per_non_main_branch_and_token},
};

pub(crate) fn bump_version() -> Workflow {
    let (determine_bump_type, bump_type) = determine_bump_type();
    let bump_type = bump_type.as_job_output(&determine_bump_type);

    let call_bump_version = call_bump_version(&determine_bump_type, bump_type);

    named::workflow()
        .on(Event::default()
            .push(
                Push::default()
                    .add_branch("main")
                    .add_ignored_path(".github/**"),
            )
            .pull_request(PullRequest::default().add_type(PullRequestType::Labeled))
            .workflow_dispatch(WorkflowDispatch::default()))
        .concurrency(one_workflow_per_non_main_branch_and_token("labels"))
        .add_job(determine_bump_type.name, determine_bump_type.job)
        .add_job(call_bump_version.name, call_bump_version.job)
}

pub(crate) fn call_bump_version(
    depending_job: &NamedJob,
    bump_type: JobOutput,
) -> NamedJob<UsesJob> {
    let job = Job::default()
        .cond(Expression::new(format!(
            "github.event.action != 'labeled' || {} != 'patch'",
            bump_type.expr()
        )))
        .uses(
            "zed-industries",
            "zed",
            ".github/workflows/extension_bump.yml",
            "main",
        )
        .add_need(depending_job.name.clone())
        .with(
            Input::default()
                .add("bump-type", bump_type.to_string())
                .add("force-bump", true),
        )
        .secrets(IndexMap::from([
            ("app-id".to_owned(), vars::ZED_ZIPPY_APP_ID.to_owned()),
            (
                "app-secret".to_owned(),
                vars::ZED_ZIPPY_APP_PRIVATE_KEY.to_owned(),
            ),
        ]));

    named::job(job)
}

fn determine_bump_type() -> (NamedJob, StepOutput) {
    let (get_bump_type, output) = get_bump_type();
    let job = Job::default()
        .runs_on(runners::LINUX_DEFAULT)
        .add_step(get_bump_type)
        .outputs([(output.name.to_owned(), output.to_string())]);
    (named::job(job), output)
}

fn get_bump_type() -> (Step<Run>, StepOutput) {
    let step = named::bash(
        indoc! {r#"
            if [ "$HAS_MAJOR_LABEL" = "true" ]; then
                bump_type="major"
            elif [ "$HAS_MINOR_LABEL" = "true" ]; then
                bump_type="minor"
            else
                bump_type="patch"
            fi
            echo "bump_type=$bump_type" >> $GITHUB_OUTPUT
        "#},
    )
    .add_env(("HAS_MAJOR_LABEL",
        indoc!{
            "${{ (github.event.action == 'labeled' && github.event.label.name == 'major') ||
            (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'major')) }}"
        }))
    .add_env(("HAS_MINOR_LABEL",
        indoc!{
            "${{ (github.event.action == 'labeled' && github.event.label.name == 'minor') ||
            (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'minor')) }}"
        }))
    .id("get-bump-type");

    let step_output = StepOutput::new(&step, "bump_type");

    (step, step_output)
}
