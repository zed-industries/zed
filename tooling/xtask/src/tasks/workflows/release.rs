use gh_workflow::{Event, Expression, Push, Run, Step, Use, Workflow};

use crate::tasks::workflows::{
    run_bundling::{bundle_linux, bundle_mac, bundle_windows},
    run_tests,
    runners::{self, Arch, Platform},
    steps::{self, FluentBuilder, NamedJob, dependant_job, named, release_job},
    vars::{self, assets},
};

pub(crate) fn release() -> Workflow {
    let macos_tests = run_tests::run_platform_tests(Platform::Mac);
    let linux_tests = run_tests::run_platform_tests(Platform::Linux);
    let windows_tests = run_tests::run_platform_tests(Platform::Windows);
    let macos_clippy = run_tests::clippy(Platform::Mac);
    let linux_clippy = run_tests::clippy(Platform::Linux);
    let windows_clippy = run_tests::clippy(Platform::Windows);
    let check_scripts = run_tests::check_scripts();

    let create_draft_release = create_draft_release();

    let bundle = ReleaseBundleJobs {
        linux_aarch64: bundle_linux(
            Arch::AARCH64,
            None,
            &[&linux_tests, &linux_clippy, &check_scripts],
        ),
        linux_x86_64: bundle_linux(
            Arch::X86_64,
            None,
            &[&linux_tests, &linux_clippy, &check_scripts],
        ),
        mac_aarch64: bundle_mac(
            Arch::AARCH64,
            None,
            &[&macos_tests, &macos_clippy, &check_scripts],
        ),
        mac_x86_64: bundle_mac(
            Arch::X86_64,
            None,
            &[&macos_tests, &macos_clippy, &check_scripts],
        ),
        windows_aarch64: bundle_windows(
            Arch::AARCH64,
            None,
            &[&windows_tests, &windows_clippy, &check_scripts],
        ),
        windows_x86_64: bundle_windows(
            Arch::X86_64,
            None,
            &[&windows_tests, &windows_clippy, &check_scripts],
        ),
    };

    let upload_release_assets = upload_release_assets(&[&create_draft_release], &bundle);

    let auto_release_preview = auto_release_preview(&[&upload_release_assets]);
    let notify_on_failure = notify_on_failure(&[&upload_release_assets, &auto_release_preview]);

    named::workflow()
        .on(Event::default().push(Push::default().tags(vec!["v*".to_string()])))
        .concurrency(vars::one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_job(macos_tests.name, macos_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(macos_clippy.name, macos_clippy.job)
        .add_job(linux_clippy.name, linux_clippy.job)
        .add_job(windows_clippy.name, windows_clippy.job)
        .add_job(check_scripts.name, check_scripts.job)
        .add_job(create_draft_release.name, create_draft_release.job)
        .map(|mut workflow| {
            for job in bundle.into_jobs() {
                workflow = workflow.add_job(job.name, job.job);
            }
            workflow
        })
        .add_job(upload_release_assets.name, upload_release_assets.job)
        .add_job(auto_release_preview.name, auto_release_preview.job)
        .add_job(notify_on_failure.name, notify_on_failure.job)
}

pub(crate) struct ReleaseBundleJobs {
    pub linux_aarch64: NamedJob,
    pub linux_x86_64: NamedJob,
    pub mac_aarch64: NamedJob,
    pub mac_x86_64: NamedJob,
    pub windows_aarch64: NamedJob,
    pub windows_x86_64: NamedJob,
}

impl ReleaseBundleJobs {
    pub fn jobs(&self) -> Vec<&NamedJob> {
        vec![
            &self.linux_aarch64,
            &self.linux_x86_64,
            &self.mac_aarch64,
            &self.mac_x86_64,
            &self.windows_aarch64,
            &self.windows_x86_64,
        ]
    }

    pub fn into_jobs(self) -> Vec<NamedJob> {
        vec![
            self.linux_aarch64,
            self.linux_x86_64,
            self.mac_aarch64,
            self.mac_x86_64,
            self.windows_aarch64,
            self.windows_x86_64,
        ]
    }
}

pub(crate) fn create_sentry_release() -> Step<Use> {
    named::uses(
        "getsentry",
        "action-release",
        "526942b68292201ac6bbb99b9a0747d4abee354c", // v3
    )
    .add_env(("SENTRY_ORG", "zed-dev"))
    .add_env(("SENTRY_PROJECT", "zed"))
    .add_env(("SENTRY_AUTH_TOKEN", vars::SENTRY_AUTH_TOKEN))
    .add_with(("environment", "production"))
}

fn auto_release_preview(deps: &[&NamedJob; 1]) -> NamedJob {
    let (authenticate, token) = steps::authenticate_as_zippy();

    named::job(
        dependant_job(deps)
            .runs_on(runners::LINUX_SMALL)
            .cond(Expression::new(indoc::indoc!(
                r#"startsWith(github.ref, 'refs/tags/v') && endsWith(github.ref, '-pre') && !endsWith(github.ref, '.0-pre')"#
            )))
            .add_step(authenticate)
            .add_step(
                steps::script(
                    r#"gh release edit "$GITHUB_REF_NAME" --repo=zed-industries/zed --draft=false"#,
                )
                .add_env(("GITHUB_TOKEN", &token)),
            )
    )
}

pub(crate) fn download_workflow_artifacts() -> Step<Use> {
    named::uses(
        "actions",
        "download-artifact",
        "018cc2cf5baa6db3ef3c5f8a56943fffe632ef53", // v6.0.0
    )
    .add_with(("path", "./artifacts/"))
}

pub(crate) fn prep_release_artifacts() -> Step<Run> {
    let mut script_lines = vec!["mkdir -p release-artifacts/\n".to_string()];
    for asset in assets::all() {
        let mv_command = format!("mv ./artifacts/{asset}/{asset} release-artifacts/{asset}");
        script_lines.push(mv_command)
    }

    named::bash(&script_lines.join("\n"))
}

fn upload_release_assets(deps: &[&NamedJob], bundle: &ReleaseBundleJobs) -> NamedJob {
    let mut deps = deps.to_vec();
    deps.extend(bundle.jobs());

    named::job(
        dependant_job(&deps)
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(download_workflow_artifacts())
            .add_step(steps::script("ls -lR ./artifacts"))
            .add_step(prep_release_artifacts())
            .add_step(
                steps::script("gh release upload \"$GITHUB_REF_NAME\" --repo=zed-industries/zed release-artifacts/*")
                    .add_env(("GITHUB_TOKEN", vars::GITHUB_TOKEN)),
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
            .add_env(("GITHUB_TOKEN", vars::GITHUB_TOKEN))
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

pub(crate) fn notify_on_failure(deps: &[&NamedJob]) -> NamedJob {
    fn notify_slack() -> Step<Run> {
        named::bash(
            "curl -X POST -H 'Content-type: application/json'\\\n --data '{\"text\":\"${{ github.workflow }} failed:  ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}\"}' \"$SLACK_WEBHOOK\""
        ).add_env(("SLACK_WEBHOOK", vars::SLACK_WEBHOOK_WORKFLOW_FAILURES))
    }

    let job = dependant_job(deps)
        .runs_on(runners::LINUX_SMALL)
        .cond(Expression::new("failure()"))
        .add_step(notify_slack());
    named::job(job)
}
