use crate::tasks::workflows::{
    nix_build::build_nix,
    release::{
        ReleaseBundleJobs, create_sentry_release, download_workflow_artifacts, notify_on_failure,
        prep_release_artifacts,
    },
    run_bundling::{bundle_linux, bundle_mac, bundle_windows},
    run_tests::run_platform_tests,
    runners::{Arch, Platform, ReleaseChannel},
    steps::{CommonJobConditions, FluentBuilder, NamedJob},
};

use super::{runners, steps, steps::named, vars};
use gh_workflow::*;

/// Generates the release_nightly.yml workflow
pub fn release_nightly() -> Workflow {
    let style = check_style();
    // run only on windows as that's our fastest platform right now.
    let tests = run_platform_tests(Platform::Windows);
    let nightly = Some(ReleaseChannel::Nightly);

    let bundle = ReleaseBundleJobs {
        linux_aarch64: bundle_linux(Arch::AARCH64, nightly, &[&style, &tests]),
        linux_x86_64: bundle_linux(Arch::X86_64, nightly, &[&style, &tests]),
        mac_aarch64: bundle_mac(Arch::AARCH64, nightly, &[&style, &tests]),
        mac_x86_64: bundle_mac(Arch::X86_64, nightly, &[&style, &tests]),
        windows_aarch64: bundle_windows(Arch::AARCH64, nightly, &[&style, &tests]),
        windows_x86_64: bundle_windows(Arch::X86_64, nightly, &[&style, &tests]),
    };

    let nix_linux_x86 = build_nix(
        Platform::Linux,
        Arch::X86_64,
        "default",
        None,
        &[&style, &tests],
    );
    let nix_mac_arm = build_nix(
        Platform::Mac,
        Arch::AARCH64,
        "default",
        None,
        &[&style, &tests],
    );
    let update_nightly_tag = update_nightly_tag_job(&bundle);
    let notify_on_failure = notify_on_failure(&bundle.jobs());

    named::workflow()
        .on(Event::default()
            // Fire every day at 7:00am UTC (Roughly before EU workday and after US workday)
            .schedule([Schedule::new("0 7 * * *")])
            .push(Push::default().add_tag("nightly")))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_job(style.name, style.job)
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

fn check_style() -> NamedJob {
    let job = release_job(&[])
        .runs_on(runners::MAC_DEFAULT)
        .add_step(
            steps::checkout_repo()
                .add_with(("clean", false))
                .add_with(("fetch-depth", 0)),
        )
        .add_step(steps::cargo_fmt())
        .add_step(steps::script("./script/clippy"));

    named::job(job)
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
    fn update_nightly_tag() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [ "$(git rev-parse nightly)" = "$(git rev-parse HEAD)" ]; then
              echo "Nightly tag already points to current commit. Skipping tagging."
              exit 0
            fi
            git config user.name github-actions
            git config user.email github-actions@github.com
            git tag -f nightly
            git push origin nightly --force
        "#})
    }

    NamedJob {
        name: "update_nightly_tag".to_owned(),
        job: steps::release_job(&bundle.jobs())
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(steps::checkout_repo().add_with(("fetch-depth", 0)))
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
            .add_step(update_nightly_tag())
            .add_step(create_sentry_release()),
    }
}
