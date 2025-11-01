use gh_workflow::{Event, Expression, Push, Run, Step, Use, Workflow};

use crate::tasks::workflows::{
    run_bundling, run_tests, runners,
    steps::{self, NamedJob, dependant_job, named, release_job},
    vars,
};

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

    let auto_release_preview = auto_release_preview(&[&upload_release_assets]);

    named::workflow()
        .on(Event::default().push(Push::default().tags(vec!["v*".to_string()])))
        .concurrency(vars::one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", "0"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_env(("ZED_MINIDUMP_ENDPOINT", vars::ZED_SENTRY_MINIDUMP_ENDPOINT))
        .add_job(macos_tests.name, macos_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(check_scripts.name, check_scripts.job)
        .add_job(create_draft_release.name, create_draft_release.job)
        .add_job(bundle.linux_arm64.name, bundle.linux_arm64.job)
        .add_job(bundle.linux_x86_64.name, bundle.linux_x86_64.job)
        .add_job(bundle.mac_arm64.name, bundle.mac_arm64.job)
        .add_job(bundle.mac_x86_64.name, bundle.mac_x86_64.job)
        .add_job(bundle.windows_arm64.name, bundle.windows_arm64.job)
        .add_job(bundle.windows_x86_64.name, bundle.windows_x86_64.job)
        .add_job(upload_release_assets.name, upload_release_assets.job)
        .add_job(auto_release_preview.name, auto_release_preview.job)
}

struct ReleaseBundleJobs {
    linux_arm64: NamedJob,
    linux_x86_64: NamedJob,
    mac_arm64: NamedJob,
    mac_x86_64: NamedJob,
    windows_arm64: NamedJob,
    windows_x86_64: NamedJob,
}

fn auto_release_preview(deps: &[&NamedJob; 1]) -> NamedJob {
    fn create_sentry_release() -> Step<Use> {
        named::uses(
            "getsentry",
            "action-release",
            "526942b68292201ac6bbb99b9a0747d4abee354c", // v3
        )
        .add_env(("SENTRY_ORG", "zed-dev"))
        .add_env(("SENTRY_PROJECT", "zed"))
        .add_env(("SENTRY_AUTH_TOKEN", "${{ secrets.SENTRY_AUTH_TOKEN }}"))
        .add_with(("environment", "production"))
    }

    named::job(
        dependant_job(deps)
            .runs_on(runners::LINUX_SMALL)
            .cond(Expression::new(indoc::indoc!(
                r#"
                false
                && startsWith(github.ref, 'refs/tags/v')
                && endsWith(github.ref, '-pre') && !endsWith(github.ref, '.0-pre')
            "# // todo(ci-release) enable
            )))
            .add_step(
                steps::script(
                    r#"gh release edit "$GITHUB_REF_NAME" --repo=zed-industries/zed --draft=false"#,
                )
                .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")),
            )
            .add_step(create_sentry_release()),
    )
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

    fn prep_release_artifacts(bundle: &ReleaseBundleJobs) -> Step<Run> {
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
                steps::script("gh release upload \"$GITHUB_REF_NAME\" --repo=zed-industries/zed release-artifacts/*")
                    .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")),
            ),
    )
}

fn create_draft_release() -> NamedJob {
    fn generate_release_notes() -> Step<Run> {
        named::bash(
            r#"node --redirect-warnings=/dev/null ./script/draft-release-notes "$RELEASE_VERSION" "$RELEASE_CHANNEL" > target/release-notes.md"#,
        )
    }

    fn create_release() -> Step<Run> {
        named::bash("script/create-draft-release target/release-notes.md")
            .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"))
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            // We need to fetch more than one commit so that `script/draft-release-notes`
            // is able to diff between the current and previous tag.
            //
            // 25 was chosen arbitrarily.
            .add_step(
                steps::checkout_repo()
                    .add_with(("fetch-depth", 25))
                    .add_with(("clean", false))
                    .add_with(("ref", "${{ github.ref }}")),
            )
            .add_step(steps::script("script/determine-release-channel"))
            .add_step(steps::script("mkdir -p target/"))
            .add_step(generate_release_notes())
            .add_step(create_release()),
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
