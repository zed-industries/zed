use gh_workflow::{
    Event, Expression, Input, Job, Level, Permissions, Push, Strategy, UsesJob, Workflow,
};
use indoc::formatdoc;
use serde_json::json;

use crate::tasks::workflows::{
    extensions::WithAppSecrets,
    run_tests::DETECT_CHANGED_EXTENSIONS_SCRIPT,
    runners,
    steps::{self, CommonJobConditions, NamedJob, named},
    vars::{StepOutput, one_workflow_per_non_main_branch},
};

/// Generates a workflow that triggers on push to main, detects changed extensions
/// in the `extensions/` directory, and invokes the `extension_bump` reusable workflow
/// for each changed extension via a matrix strategy.
pub(crate) fn extension_auto_bump() -> Workflow {
    let detect = detect_changed_extensions();
    let bump = bump_extension_versions(&detect);

    named::workflow()
        .add_event(
            Event::default().push(Push::default().add_branch("main").add_path("extensions/**")),
        )
        .concurrency(one_workflow_per_non_main_branch())
        .add_job(detect.name, detect.job)
        .add_job(bump.name, bump.job)
}

fn detect_changed_extensions() -> NamedJob {
    let script = formatdoc!(
        r#"
        COMPARE_REV=\"$(git rev-parse HEAD~1)\
        CHANGED_FILES=\"$(git diff --name-only \"$COMPARE_REV\" \"$GITHUB_SHA\")\"
        {DETECT_CHANGED_EXTENSIONS_SCRIPT}
        "#
    );

    let step = named::bash(script).id("detect");

    let output = StepOutput::new(&step, "changed_extensions");

    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(5u32)
        .add_step(steps::checkout_repo().with_custom_fetch_depth(2))
        .add_step(step)
        .outputs([("changed_extensions".to_owned(), output.to_string())]);

    named::job(job)
}

fn bump_extension_versions(detect_job: &NamedJob) -> NamedJob<UsesJob> {
    let job = Job::default()
        .needs(vec![detect_job.name.clone()])
        .cond(Expression::new(format!(
            "needs.{}.outputs.changed_extensions != '[]'",
            detect_job.name
        )))
        .permissions(
            Permissions::default()
                .contents(Level::Write)
                .issues(Level::Write)
                .pull_requests(Level::Write)
                .actions(Level::Write),
        )
        .strategy(Strategy::default().fail_fast(false).matrix(json!({
            "extension": format!(
                "${{{{ fromJson(needs.{}.outputs.changed_extensions) }}}}",
                detect_job.name
            )
        })))
        .uses(
            "zed-industries",
            "zed",
            ".github/workflows/extension_bump.yml",
            "main",
        )
        .with(
            Input::default()
                .add("working-directory", "${{ matrix.extension }}")
                .add("force-bump", false),
        )
        .with_app_secrets();

    named::job(job)
}
