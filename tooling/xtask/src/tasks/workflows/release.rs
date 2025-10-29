use gh_workflow::{Event, Push, Step, Use, Workflow};

use crate::tasks::workflows::{
    run_bundling, run_tests, runners,
    steps::{NamedJob, dependant_job, named},
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

    let bundle_mac_x86_64 = bundle_mac_x86_64(&[&macos_tests, &check_scripts]);
    let bundle_mac_arm64 = bundle_mac_arm64(&[&macos_tests, &check_scripts]);
    let bundle_linux_x86_64 = bundle_linux_x86_64(&[&linux_tests, &check_scripts]);
    let bundle_linux_arm64 = bundle_linux_arm64(&[&linux_tests, &check_scripts]);
    let bundle_windows_x86_64 = bundle_windows_x86_64(&[&windows_tests, &check_scripts]);
    let bundle_windows_arm64 = bundle_windows_arm64(&[&windows_tests, &check_scripts]);

    let upload_release_assets = upload_release_assets(&[
        &bundle_linux_arm64,
        &bundle_linux_x86_64,
        &bundle_mac_arm64,
        &bundle_mac_x86_64,
        &bundle_windows_arm64,
        &bundle_windows_x86_64,
        &create_draft_release,
    ]);

    named::workflow()
        .on(Event::default().push(
            Push::default()
                .add_branch("gh-workflow-release")
                .tags(vec!["v00.00.00-test".to_string()]),
        ))
        .add_job(macos_tests.name, macos_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(check_scripts.name, check_scripts.job)
        .add_job(create_draft_release.name, create_draft_release.job)
        .add_job(bundle_linux_arm64.name, bundle_linux_arm64.job)
        .add_job(bundle_linux_x86_64.name, bundle_linux_x86_64.job)
        .add_job(bundle_mac_arm64.name, bundle_mac_arm64.job)
        .add_job(bundle_mac_x86_64.name, bundle_mac_x86_64.job)
        .add_job(bundle_windows_arm64.name, bundle_windows_arm64.job)
        .add_job(bundle_windows_x86_64.name, bundle_windows_x86_64.job)
        .add_job(upload_release_assets.name, upload_release_assets.job)
}

fn upload_release_assets(deps: &[&NamedJob]) -> NamedJob {
    fn download_workflow_artifacts() -> Step<Use> {
        named::uses(
            "actions",
            "download-artifact",
            "018cc2cf5baa6db3ef3c5f8a56943fffe632ef53 ", // v6.0.0
        )
        .with(("path", "release-artifacts"))
    }

    fn upload_release_artifacts() -> Step<Use> {
        // todo! combine with create_draft_release somehow
        named::uses(
            "softprops",
            "action-gh-release",
            "de2c0eb89ae2a093876385947365aca7b0e5f844", // v1
        )
        .add_with(("draft", true))
        .add_with(("prerelease", "${{ env.RELEASE_CHANNEL == 'preview' }}"))
        .add_with(("files", "release-artifacts/*")) // todo! const
    }

    named::job(
        dependant_job(deps)
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(download_workflow_artifacts())
            .add_step(upload_release_artifacts()),
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
    named::job(run_bundling::bundle_linux(runners::Arch::X86_64, deps))
}
fn bundle_linux_arm64(deps: &[&NamedJob]) -> NamedJob {
    named::job(run_bundling::bundle_linux(runners::Arch::ARM64, deps))
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
