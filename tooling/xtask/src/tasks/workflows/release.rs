use gh_workflow::{Concurrency, Event, Job, Push, Run, Step, Use, Workflow};

use crate::tasks::workflows::{
    run_bundling, run_tests, runners,
    steps::{self, NamedJob, dependant_job, named},
};

// ideal release flow:
//  phase 1:
//   - tests on linux/windows/mac
//   - (maybe) check fmt and spelling / licenses? [or maybe not, we already test them everywhere else...]
//   - create draft release with the draft release notes
//  phase 2:
//   - build linux/windows/mac
//  phase 3:
//   - auto-release preview (for patch releases on the preview branch)

pub(crate) fn release() -> Workflow {
    let macos_tests = run_tests::run_platform_tests(runners::Platform::Mac);
    let linux_tests = run_tests::run_platform_tests(runners::Platform::Linux);
    let windows_tests = run_tests::run_platform_tests(runners::Platform::Windows);
    let check_scripts = run_tests::check_scripts();

    let create_draft_release = create_draft_release();

    let bundle = ReleaseBundleJobs {
        linux_arm64: bundle_linux_arm64(&[&linux_tests, &check_scripts]),
        linux_x86_64: bundle_linux_x86_64(&[&linux_tests, &check_scripts]),
        mac_arm64: bundle_mac_arm64(&[&macos_tests, &check_scripts]),
        mac_x86_64: bundle_mac_x86_64(&[&macos_tests, &check_scripts]),
        windows_arm64: bundle_windows_arm64(&[&windows_tests, &check_scripts]),
        windows_x86_64: bundle_windows_x86_64(&[&windows_tests, &check_scripts]),
    };

    let upload_release_assets = upload_release_assets(&[&create_draft_release], &bundle);

    named::workflow()
        .on(Event::default().push(Push::default().tags(vec!["v00.00.00-test".to_string()])))
        .concurrency(
            // todo! what should actual concurrency be? We don't want two workflows trying to create the same release
            Concurrency::default()
                .group("${{ github.workflow }}")
                .cancel_in_progress(true),
        )
        // todo! re-enable tests
        .add_job(macos_tests.name, use_fake_job_instead(macos_tests.job))
        .add_job(linux_tests.name, use_fake_job_instead(linux_tests.job))
        .add_job(windows_tests.name, use_fake_job_instead(windows_tests.job))
        .add_job(check_scripts.name, use_fake_job_instead(check_scripts.job))
        .add_job(create_draft_release.name, create_draft_release.job)
        .add_job(bundle.linux_arm64.name, bundle.linux_arm64.job)
        .add_job(bundle.linux_x86_64.name, bundle.linux_x86_64.job)
        .add_job(bundle.mac_arm64.name, bundle.mac_arm64.job)
        .add_job(bundle.mac_x86_64.name, bundle.mac_x86_64.job)
        .add_job(bundle.windows_arm64.name, bundle.windows_arm64.job)
        .add_job(bundle.windows_x86_64.name, bundle.windows_x86_64.job)
        .add_job(upload_release_assets.name, upload_release_assets.job)
    // todo! auto-release preview
}

fn use_fake_job_instead(_: Job) -> Job {
    Job::default()
        .add_step(steps::checkout_repo())
        .runs_on(runners::LINUX_SMALL)
}

struct ReleaseBundleJobs {
    linux_arm64: NamedJob,
    linux_x86_64: NamedJob,
    mac_arm64: NamedJob,
    mac_x86_64: NamedJob,
    windows_arm64: NamedJob,
    windows_x86_64: NamedJob,
}

fn upload_release_assets(deps: &[&NamedJob], bundle_jobs: &ReleaseBundleJobs) -> NamedJob {
    fn download_workflow_artifacts() -> Step<Use> {
        named::uses(
            "actions",
            "download-artifact",
            "018cc2cf5baa6db3ef3c5f8a56943fffe632ef53", // v6.0.0
        )
        .add_with(("path", "./artifacts/"))
    }

    // todo! consider splitting this up per release
    // upload_release_artifacts_job(platform, arm64_job, x86_64_job) -> NamedJob;
    // pro - when doing releases, once assets appear, you know it's all of them
    // con - no testing Windows assets while waiting for Linux to finish
    fn prep_release_artifacts(bundle: &ReleaseBundleJobs) -> Step<Run> {
        // todo! is it worth it to try and make the
        // "zed" and "remote-server" output names here type safe/codified?
        let assets = [
            (&bundle.mac_x86_64.name, "zed", "Zed-x86_64.dmg"),
            (&bundle.mac_arm64.name, "zed", "Zed-aarch64.dmg"),
            (&bundle.windows_x86_64.name, "zed", "Zed-x86_64.exe"),
            (&bundle.windows_arm64.name, "zed", "Zed-aarch64.exe"),
            (&bundle.linux_arm64.name, "zed", "zed-linux-aarch64.tar.gz"),
            (&bundle.linux_x86_64.name, "zed", "zed-linux-x86_64.tar.gz"),
            (
                &bundle.linux_x86_64.name,
                "remote-server",
                "zed-remote-server-linux-x86_64.gz",
            ),
            (
                &bundle.linux_arm64.name,
                "remote-server",
                "zed-remote-server-linux-aarch64.gz",
            ),
            (
                &bundle.mac_x86_64.name,
                "remote-server",
                "zed-remote-server-macos-x86_64.gz",
            ),
            (
                &bundle.mac_arm64.name,
                "remote-server",
                "zed-remote-server-macos-aarch64.gz",
            ),
        ];

        let mut script_lines = vec!["mkdir -p release-artifacts/\n".to_string()];
        for (job_name, artifact_kind, release_artifact_name) in assets {
            let artifact_path =
                ["${{ needs.", job_name, ".outputs.", artifact_kind, " }}"].join("");
            let mv_command = format!(
                "mv ./artifacts/{artifact_path}/* release-artifacts/{release_artifact_name}"
            );
            script_lines.push(mv_command)
        }

        named::bash(&script_lines.join("\n"))
    }

    let mut deps = deps.to_vec();
    deps.extend([
        &bundle_jobs.linux_arm64,
        &bundle_jobs.linux_x86_64,
        &bundle_jobs.mac_arm64,
        &bundle_jobs.mac_x86_64,
        &bundle_jobs.windows_arm64,
        &bundle_jobs.windows_x86_64,
    ]);

    named::job(
        dependant_job(&deps)
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(download_workflow_artifacts())
            .add_step(steps::script("ls -lR ./artifacts"))
            .add_step(prep_release_artifacts(bundle_jobs))
            .add_step(
                steps::script("gh release upload ${{ github.ref }} release-artifacts/*")
                    .add_env(("GH_TOKEN", "${{ secrets.GITHUB_TOKEN }}")),
            ),
    )
}

fn create_draft_release() -> NamedJob {
    named::job(
        dependant_job(&[]).runs_on(runners::LINUX_SMALL).add_step(
            named::uses(
                "softprops",
                "action-gh-release",
                "de2c0eb89ae2a093876385947365aca7b0e5f844", // v1
            )
            .add_with(("draft", true))
            .add_with(("prerelease", "${{ env.RELEASE_CHANNEL == 'preview' }}")),
        ),
    )
}

fn bundle_mac_x86_64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_mac_job(runners::Arch::X86_64, deps))
}
fn bundle_mac_arm64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_mac_job(runners::Arch::ARM64, deps))
}
fn bundle_linux_x86_64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_linux_job(runners::Arch::X86_64, deps))
}
fn bundle_linux_arm64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_linux_job(runners::Arch::ARM64, deps))
}
fn bundle_windows_x86_64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_windows_job(
        runners::Arch::X86_64,
        deps,
    ))
}
fn bundle_windows_arm64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_windows_job(runners::Arch::ARM64, deps))
}
