use crate::tasks::workflows::{
    nix_build::build_nix,
    release::{
        ReleaseBundleJobs, create_sentry_release, download_workflow_artifacts, notify_on_failure,
        prep_release_artifacts,
    },
    run_bundling::{bundle_linux, bundle_mac, bundle_windows},
    run_tests::run_platform_tests_no_filter,
    runners::{Arch, Platform, ReleaseChannel},
    steps::{
        CommonJobConditions, DEFAULT_REPOSITORY_OWNER_GUARD, FluentBuilder, GitRef, NamedJob,
        RefSha, RepositoryTarget, TokenPermissions,
    },
};

use super::{runners, steps, steps::named, vars};
use gh_workflow::*;

/// Generates the release_nightly.yml workflow
pub fn release_nightly() -> Workflow {
    let (check_tag, skip) = check_nightly_tag();
    let mut tests = run_platform_tests_no_filter(Platform::Linux);
    tests.job = tests
        .job
        .needs([check_tag.name.clone()])
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} && {} != 'true'",
            skip.expr()
        )));

    const NIGHTLY: Option<ReleaseChannel> = Some(ReleaseChannel::Nightly);

    let bundle = ReleaseBundleJobs {
        linux_aarch64: bundle_linux(Arch::AARCH64, NIGHTLY, &[&tests]),
        linux_x86_64: bundle_linux(Arch::X86_64, NIGHTLY, &[&tests]),
        mac_aarch64: bundle_mac(Arch::AARCH64, NIGHTLY, &[&tests]),
        mac_x86_64: bundle_mac(Arch::X86_64, NIGHTLY, &[&tests]),
        windows_aarch64: bundle_windows(Arch::AARCH64, NIGHTLY, &[&tests]),
        windows_x86_64: bundle_windows(Arch::X86_64, NIGHTLY, &[&tests]),
    };

    let nix_linux_x86 = build_nix(Platform::Linux, Arch::X86_64, "default", None, &[&tests]);
    let nix_mac_arm = build_nix(Platform::Mac, Arch::AARCH64, "default", None, &[&tests]);
    let update_nightly_tag = update_nightly_tag_job(&bundle);
    let notify_on_failure = notify_on_failure(&bundle.jobs());

    named::workflow()
        .on(Event::default()
            // Fire 6 times a day
            .schedule([Schedule::new("0 */4 * * *")])
            .workflow_dispatch(WorkflowDispatch::default()))
        .concurrency(
            Concurrency::default()
                .group("release-nightly")
                .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_job(check_tag.name, check_tag.job)
        .add_job(tests.name, tests.job)
        .map(|mut workflow| {
            for job in bundle.into_jobs() {
                workflow = workflow.add_job(job.name, job.job);
            }
            workflow
        })
        .add_job(nix_linux_x86.name, nix_linux_x86.job)
        .add_job(nix_mac_arm.name, nix_mac_arm.job)
        .add_job(update_nightly_tag.name, update_nightly_tag.job)
        .add_job(notify_on_failure.name, notify_on_failure.job)
}

fn check_nightly_tag() -> (NamedJob, vars::JobOutput) {
    let step = named::bash(indoc::indoc! {r#"
        NIGHTLY_SHA=$(git rev-parse "nightly" 2>/dev/null || echo "")
        if [ "$NIGHTLY_SHA" = "$GITHUB_SHA" ]; then
            echo "Nightly tag already points to current commit. Skipping."
            echo "skip=true" >> "$GITHUB_OUTPUT"
        else
            echo "skip=false" >> "$GITHUB_OUTPUT"
        fi
    "#})
    .id("check");

    let skip_output = vars::StepOutput::new(&step, "skip");

    let job = release_job(&[])
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(5u32)
        .outputs([("skip".to_owned(), skip_output.to_string())])
        .add_step(steps::checkout_repo().with_fetch_tags())
        .add_step(step);

    let job = named::job(job);
    let skip = skip_output.as_job_output(&job);
    (job, skip)
}

fn release_job(deps: &[&NamedJob]) -> Job {
    let job = Job::default()
        .with_repository_owner_guard()
        .timeout_minutes(60u32);
    if deps.len() > 0 {
        job.needs(deps.iter().map(|j| j.name.clone()).collect::<Vec<_>>())
    } else {
        job
    }
}

fn update_nightly_tag_job(bundle: &ReleaseBundleJobs) -> NamedJob {
    let (authenticate, token) = steps::authenticate_as_zippy()
        .for_repository(RepositoryTarget::current())
        .with_permissions([(TokenPermissions::Contents, Level::Write)])
        .into();

    NamedJob {
        name: "update_nightly_tag".to_owned(),
        job: steps::release_job(&bundle.jobs())
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(authenticate)
            .add_step(steps::checkout_repo().with_fetch_tags())
            .add_step(download_workflow_artifacts())
            .add_step(steps::script("ls -lR ./artifacts"))
            .add_step(prep_release_artifacts())
            .add_step(
                steps::script("./script/upload-nightly")
                    .add_env((
                        "DIGITALOCEAN_SPACES_ACCESS_KEY",
                        vars::DIGITALOCEAN_SPACES_ACCESS_KEY,
                    ))
                    .add_env((
                        "DIGITALOCEAN_SPACES_SECRET_KEY",
                        vars::DIGITALOCEAN_SPACES_SECRET_KEY,
                    )),
            )
            .add_step(steps::update_ref(
                GitRef::tag("nightly"),
                RefSha::Context,
                &token,
                true,
            ))
            .add_step(create_sentry_release()),
    }
}
