use gh_workflow::{Event, Expression, Push, Run, Step, Use, Workflow, ctx::Context};
use indoc::formatdoc;

use crate::tasks::workflows::{
    run_bundling::{bundle_linux, bundle_mac, bundle_windows},
    run_tests,
    runners::{self, Arch, Platform},
    steps::{self, FluentBuilder, NamedJob, dependant_job, named, release_job},
    vars::{self, StepOutput, assets},
};

const CURRENT_ACTION_RUN_URL: &str =
    "${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}";

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
    let validate_release_assets = validate_release_assets(&[&upload_release_assets]);

    let auto_release_preview = auto_release_preview(&[&validate_release_assets]);

    let test_jobs = [
        &macos_tests,
        &linux_tests,
        &windows_tests,
        &macos_clippy,
        &linux_clippy,
        &windows_clippy,
        &check_scripts,
    ];
    let push_slack_notification = push_release_update_notification(
        &create_draft_release,
        &upload_release_assets,
        &validate_release_assets,
        &auto_release_preview,
        &test_jobs,
        &bundle,
    );

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
        .add_job(validate_release_assets.name, validate_release_assets.job)
        .add_job(auto_release_preview.name, auto_release_preview.job)
        .add_job(push_slack_notification.name, push_slack_notification.job)
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

fn validate_release_assets(deps: &[&NamedJob]) -> NamedJob {
    let expected_assets: Vec<String> = assets::all().iter().map(|a| format!("\"{a}\"")).collect();
    let expected_assets_json = format!("[{}]", expected_assets.join(", "));

    let validation_script = formatdoc! {r#"
        EXPECTED_ASSETS='{expected_assets_json}'
        TAG="$GITHUB_REF_NAME"

        ACTUAL_ASSETS=$(gh release view "$TAG" --repo=zed-industries/zed --json assets -q '[.assets[].name]')

        MISSING_ASSETS=$(echo "$EXPECTED_ASSETS" | jq -r --argjson actual "$ACTUAL_ASSETS" '. - $actual | .[]')

        if [ -n "$MISSING_ASSETS" ]; then
            echo "Error: The following assets are missing from the release:"
            echo "$MISSING_ASSETS"
            exit 1
        fi

        echo "All expected assets are present in the release."
        "#,
    };

    named::job(
        dependant_job(deps).runs_on(runners::LINUX_SMALL).add_step(
            named::bash(&validation_script).add_env(("GITHUB_TOKEN", vars::GITHUB_TOKEN)),
        ),
    )
}

fn auto_release_preview(deps: &[&NamedJob]) -> NamedJob {
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

pub(crate) fn push_release_update_notification(
    create_draft_release_job: &NamedJob,
    upload_assets_job: &NamedJob,
    validate_assets_job: &NamedJob,
    auto_release_preview: &NamedJob,
    test_jobs: &[&NamedJob],
    bundle_jobs: &ReleaseBundleJobs,
) -> NamedJob {
    let all_job_names = test_jobs
        .into_iter()
        .map(|j| j.name.as_ref())
        .chain(bundle_jobs.jobs().into_iter().map(|j| j.name.as_ref()));

    let notification_script = formatdoc! {r#"
        DRAFT_RESULT="${{{{ needs.{draft_job}.result }}}}"
        UPLOAD_RESULT="${{{{ needs.{upload_job}.result }}}}"
        VALIDATE_RESULT="${{{{ needs.{validate_job}.result }}}}"
        AUTO_RELEASE_RESULT="${{{{ needs.{auto_release_job}.result }}}}"
        TAG="$GITHUB_REF_NAME"
        RUN_URL="{run_url}"

        if [ "$DRAFT_RESULT" == "failure" ]; then
            echo "❌ Draft release creation failed for $TAG: $RUN_URL"
        else
            RELEASE_URL=$(gh release view "$TAG" --repo=zed-industries/zed --json url -q '.url')
            if [ "$UPLOAD_RESULT" == "failure" ]; then
                echo "❌ Release asset upload failed for $TAG: $RELEASE_URL"
            elif [ "$UPLOAD_RESULT" == "cancelled" ] || [ "$UPLOAD_RESULT" == "skipped" ]; then
                FAILED_JOBS=""
                {failure_checks}
                FAILED_JOBS=$(echo "$FAILED_JOBS" | xargs)
                if [ "$UPLOAD_RESULT" == "cancelled" ]; then
                    if [ -n "$FAILED_JOBS" ]; then
                        echo "❌ Release job for $TAG was cancelled, most likely because tests \`$FAILED_JOBS\` failed: $RUN_URL"
                    else
                        echo "❌ Release job for $TAG was cancelled: $RUN_URL"
                    fi
                else
                    if [ -n "$FAILED_JOBS" ]; then
                        echo "❌ Tests \`$FAILED_JOBS\` for $TAG failed: $RUN_URL"
                    else
                        echo "❌ Tests for $TAG failed: $RUN_URL"
                    fi
                fi
            elif [ "$VALIDATE_RESULT" == "failure" ]; then
                echo "❌ Release asset validation failed for $TAG (missing assets): $RUN_URL"
            elif [ "$AUTO_RELEASE_RESULT" == "success" ]; then
                echo "✅ Release $TAG was auto-released successfully: $RELEASE_URL"
            elif [ "$AUTO_RELEASE_RESULT" == "failure" ]; then
                echo "❌ Auto release failed for $TAG: $RUN_URL"
            else
                echo "👀 Release $TAG sitting freshly baked in the oven and waiting to be published: $RELEASE_URL"
            fi
        fi
        "#,
        draft_job = create_draft_release_job.name,
        upload_job = upload_assets_job.name,
        validate_job = validate_assets_job.name,
        auto_release_job = auto_release_preview.name,
        run_url = CURRENT_ACTION_RUN_URL,
        failure_checks = all_job_names
            .into_iter()
            .map(|name: &str| format!(
                "if [ \"${{{{ needs.{name}.result }}}}\" == \"failure\" ];\
                then FAILED_JOBS=\"$FAILED_JOBS {name}\"; fi"
            ))
            .collect::<Vec<_>>()
            .join("\n        "),
    };

    let mut all_deps: Vec<&NamedJob> = vec![
        create_draft_release_job,
        upload_assets_job,
        validate_assets_job,
        auto_release_preview,
    ];
    all_deps.extend(test_jobs.iter().copied());
    all_deps.extend(bundle_jobs.jobs());

    let mut job = dependant_job(&all_deps)
        .runs_on(runners::LINUX_SMALL)
        .cond(Expression::new("always()"));

    for step in notify_slack(MessageType::Evaluated(notification_script)) {
        job = job.add_step(step);
    }
    named::job(job)
}

pub(crate) fn notify_on_failure(deps: &[&NamedJob]) -> NamedJob {
    let failure_message = format!("❌ ${{{{ github.workflow }}}} failed: {CURRENT_ACTION_RUN_URL}");

    let mut job = dependant_job(deps)
        .runs_on(runners::LINUX_SMALL)
        .cond(Expression::new("failure()"));

    for step in notify_slack(MessageType::Static(failure_message)) {
        job = job.add_step(step);
    }
    named::job(job)
}

pub(crate) enum MessageType {
    Static(String),
    Evaluated(String),
}

fn notify_slack(message: MessageType) -> Vec<Step<Run>> {
    match message {
        MessageType::Static(message) => vec![send_slack_message(message)],
        MessageType::Evaluated(expression) => {
            let (generate_step, generated_message) = generate_slack_message(expression);

            vec![
                generate_step,
                send_slack_message(generated_message.to_string()),
            ]
        }
    }
}

fn generate_slack_message(expression: String) -> (Step<Run>, StepOutput) {
    let script = formatdoc! {r#"
        MESSAGE=$({expression})
        echo "message=$MESSAGE" >> "$GITHUB_OUTPUT"
        "#
    };
    let generate_step = named::bash(&script)
        .id("generate-webhook-message")
        .add_env(("GH_TOKEN", Context::github().token()));

    let output = StepOutput::new(&generate_step, "message");

    (generate_step, output)
}

fn send_slack_message(message: String) -> Step<Run> {
    let script = formatdoc! {r#"
        curl -X POST -H 'Content-type: application/json'\
         --data '{{"text":"{message}"}}' "$SLACK_WEBHOOK"
        "#
    };
    named::bash(&script).add_env(("SLACK_WEBHOOK", vars::SLACK_WEBHOOK_WORKFLOW_FAILURES))
}
