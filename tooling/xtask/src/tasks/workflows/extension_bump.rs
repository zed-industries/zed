use gh_workflow::{ctx::Context, *};
use indoc::{formatdoc, indoc};

use crate::tasks::workflows::{
    extension_tests::{self},
    runners,
    steps::{
        self, CommonJobConditions, DEFAULT_REPOSITORY_OWNER_GUARD, FluentBuilder, NamedJob,
        checkout_repo, dependant_job, named,
    },
    vars::{
        JobOutput, StepOutput, WorkflowInput, WorkflowSecret, one_workflow_per_non_main_branch,
    },
};

const VERSION_CHECK: &str = r#"sed -n 's/version = \"\(.*\)\"/\1/p' < extension.toml"#;

// This is used by various extensions repos in the zed-extensions org to bump extension versions.
pub(crate) fn extension_bump() -> Workflow {
    let bump_type = WorkflowInput::string("bump-type", Some("patch".to_owned()));
    // TODO: Ideally, this would have a default of `false`, but this is currently not
    // supported in gh-workflows
    let force_bump = WorkflowInput::bool("force-bump", None);

    let (app_id, app_secret) = extension_workflow_secrets();
    let (check_version_changed, version_changed, current_version) = check_version_changed();

    let version_changed = version_changed.as_job_output(&check_version_changed);
    let current_version = current_version.as_job_output(&check_version_changed);

    let dependencies = [&check_version_changed];
    let bump_version = bump_extension_version(
        &dependencies,
        &current_version,
        &bump_type,
        &version_changed,
        &force_bump,
        &app_id,
        &app_secret,
    );
    let create_label = create_version_label(
        &dependencies,
        &version_changed,
        &current_version,
        &app_id,
        &app_secret,
    );
    let trigger_release = trigger_release(
        &[&check_version_changed, &create_label],
        current_version,
        &app_id,
        &app_secret,
    );

    named::workflow()
        .add_event(
            Event::default().workflow_call(
                WorkflowCall::default()
                    .add_input(bump_type.name, bump_type.call_input())
                    .add_input(force_bump.name, force_bump.call_input())
                    .secrets([
                        (app_id.name.to_owned(), app_id.secret_configuration()),
                        (
                            app_secret.name.to_owned(),
                            app_secret.secret_configuration(),
                        ),
                    ]),
            ),
        )
        .concurrency(one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env((
            "ZED_EXTENSION_CLI_SHA",
            extension_tests::ZED_EXTENSION_CLI_SHA,
        ))
        .add_job(check_version_changed.name, check_version_changed.job)
        .add_job(bump_version.name, bump_version.job)
        .add_job(create_label.name, create_label.job)
        .add_job(trigger_release.name, trigger_release.job)
}

fn check_version_changed() -> (NamedJob, StepOutput, StepOutput) {
    let (compare_versions, version_changed, current_version) = compare_versions();

    let job = Job::default()
        .with_repository_owner_guard()
        .outputs([
            (version_changed.name.to_owned(), version_changed.to_string()),
            (
                current_version.name.to_string(),
                current_version.to_string(),
            ),
        ])
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(steps::checkout_repo().add_with(("fetch-depth", 0)))
        .add_step(compare_versions);

    (named::job(job), version_changed, current_version)
}

fn create_version_label(
    dependencies: &[&NamedJob],
    version_changed_output: &JobOutput,
    current_version: &JobOutput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> NamedJob {
    let (generate_token, generated_token) =
        generate_token(&app_id.to_string(), &app_secret.to_string(), None);
    let job = steps::dependant_job(dependencies)
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} && github.event_name == 'push' && \
            github.ref == 'refs/heads/main' && {version_changed} == 'true'",
            version_changed = version_changed_output.expr(),
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(create_version_tag(current_version, generated_token));

    named::job(job)
}

fn create_version_tag(current_version: &JobOutput, generated_token: StepOutput) -> Step<Use> {
    named::uses("actions", "github-script", "v7").with(
        Input::default()
            .add(
                "script",
                formatdoc! {r#"
                    github.rest.git.createRef({{
                        owner: context.repo.owner,
                        repo: context.repo.repo,
                        ref: 'refs/tags/v{current_version}',
                        sha: context.sha
                    }})"#
                },
            )
            .add("github-token", generated_token.to_string()),
    )
}

/// Compares the current and previous commit and checks whether versions changed inbetween.
pub(crate) fn compare_versions() -> (Step<Run>, StepOutput, StepOutput) {
    let check_needs_bump = named::bash(formatdoc! {
    r#"
        CURRENT_VERSION="$({VERSION_CHECK})"
        PR_PARENT_SHA="${{{{ github.event.pull_request.head.sha }}}}"

        if [[ -n "$PR_PARENT_SHA" ]]; then
            git checkout "$PR_PARENT_SHA"
        elif BRANCH_PARENT_SHA="$(git merge-base origin/main origin/zed-zippy-autobump)"; then
            git checkout "$BRANCH_PARENT_SHA"
        else
            git checkout "$(git log -1 --format=%H)"~1
        fi

        PARENT_COMMIT_VERSION="$({VERSION_CHECK})"

        [[ "$CURRENT_VERSION" == "$PARENT_COMMIT_VERSION" ]] && \
            echo "version_changed=false" >> "$GITHUB_OUTPUT" || \
            echo "version_changed=true" >> "$GITHUB_OUTPUT"

        echo "current_version=${{CURRENT_VERSION}}" >> "$GITHUB_OUTPUT"
        "#
    })
    .id("compare-versions-check");

    let version_changed = StepOutput::new(&check_needs_bump, "version_changed");
    let current_version = StepOutput::new(&check_needs_bump, "current_version");

    (check_needs_bump, version_changed, current_version)
}

fn bump_extension_version(
    dependencies: &[&NamedJob],
    current_version: &JobOutput,
    bump_type: &WorkflowInput,
    version_changed_output: &JobOutput,
    force_bump_output: &WorkflowInput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> NamedJob {
    let (generate_token, generated_token) =
        generate_token(&app_id.to_string(), &app_secret.to_string(), None);
    let (bump_version, new_version) = bump_version(current_version, bump_type);

    let job = steps::dependant_job(dependencies)
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} &&\n({force_bump} == 'true' || {version_changed} == 'false')",
            force_bump = force_bump_output.expr(),
            version_changed = version_changed_output.expr(),
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(3u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(install_bump_2_version())
        .add_step(bump_version)
        .add_step(create_pull_request(new_version, generated_token));

    named::job(job)
}

pub(crate) fn generate_token(
    app_id_source: &str,
    app_secret_source: &str,
    repository_target: Option<RepositoryTarget>,
) -> (Step<Use>, StepOutput) {
    let step = named::uses("actions", "create-github-app-token", "v2")
        .id("generate-token")
        .add_with(
            Input::default()
                .add("app-id", app_id_source)
                .add("private-key", app_secret_source)
                .when_some(
                    repository_target,
                    |input,
                     RepositoryTarget {
                         owner,
                         repositories,
                         permissions,
                     }| {
                        input
                            .when_some(owner, |input, owner| input.add("owner", owner))
                            .when_some(repositories, |input, repositories| {
                                input.add("repositories", repositories)
                            })
                            .when_some(permissions, |input, permissions| {
                                permissions
                                    .into_iter()
                                    .fold(input, |input, (permission, level)| {
                                        input.add(
                                            permission,
                                            serde_json::to_value(&level).unwrap_or_default(),
                                        )
                                    })
                            })
                    },
                ),
        );

    let generated_token = StepOutput::new(&step, "token");

    (step, generated_token)
}

fn install_bump_2_version() -> Step<Run> {
    named::run(
        runners::Platform::Linux,
        "pip install bump2version --break-system-packages",
    )
}

fn bump_version(current_version: &JobOutput, bump_type: &WorkflowInput) -> (Step<Run>, StepOutput) {
    let step = named::bash(formatdoc! {r#"
        OLD_VERSION="{current_version}"

        BUMP_FILES=("extension.toml")
        if [[ -f "Cargo.toml" ]]; then
            BUMP_FILES+=("Cargo.toml")
        fi

        bump2version \
            --search "version = \"{{current_version}}"\" \
            --replace "version = \"{{new_version}}"\" \
            --current-version "$OLD_VERSION" \
            --no-configured-files {bump_type} "${{BUMP_FILES[@]}}"

        if [[ -f "Cargo.toml" ]]; then
            cargo update --workspace
        fi

        NEW_VERSION="$({VERSION_CHECK})"

        echo "new_version=${{NEW_VERSION}}" >> "$GITHUB_OUTPUT"
        "#
    })
    .id("bump-version");

    let new_version = StepOutput::new(&step, "new_version");
    (step, new_version)
}

fn create_pull_request(new_version: StepOutput, generated_token: StepOutput) -> Step<Use> {
    let formatted_version = format!("v{new_version}");

    named::uses("peter-evans", "create-pull-request", "v7").with(
        Input::default()
            .add("title", format!("Bump version to {new_version}"))
            .add(
                "body",
                format!("This PR bumps the version of this extension to {formatted_version}",),
            )
            .add(
                "commit-message",
                format!("Bump version to {formatted_version}"),
            )
            .add("branch", "zed-zippy-autobump")
            .add(
                "committer",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            )
            .add("base", "main")
            .add("delete-branch", true)
            .add("token", generated_token.to_string())
            .add("sign-commits", true)
            .add("assignees", Context::github().actor().to_string()),
    )
}

fn trigger_release(
    dependencies: &[&NamedJob],
    version: JobOutput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> NamedJob {
    let extension_registry = RepositoryTarget::new("zed-industries", &["extensions"]);
    let (generate_token, generated_token) = generate_token(
        &app_id.to_string(),
        &app_secret.to_string(),
        Some(extension_registry),
    );
    let (get_extension_id, extension_id) = get_extension_id();

    let job = dependant_job(dependencies)
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_SMALL)
        .add_step(generate_token)
        .add_step(checkout_repo())
        .add_step(get_extension_id)
        .add_step(release_action(extension_id, version, generated_token));

    named::job(job)
}

fn get_extension_id() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc! {
    r#"
        EXTENSION_ID="$(sed -n 's/id = \"\(.*\)\"/\1/p' < extension.toml)"

        echo "extension_id=${EXTENSION_ID}" >> "$GITHUB_OUTPUT"
    "#})
    .id("get-extension-id");

    let extension_id = StepOutput::new(&step, "extension_id");

    (step, extension_id)
}

fn release_action(
    extension_id: StepOutput,
    version: JobOutput,
    generated_token: StepOutput,
) -> Step<Use> {
    named::uses("huacnlee", "zed-extension-action", "v2")
        .add_with(("extension-name", extension_id.to_string()))
        .add_with(("push-to", "zed-industries/extensions"))
        .add_with(("tag", format!("v{version}")))
        .add_env(("COMMITTER_TOKEN", generated_token.to_string()))
}

fn extension_workflow_secrets() -> (WorkflowSecret, WorkflowSecret) {
    let app_id = WorkflowSecret::new("app-id", "The app ID used to create the PR");
    let app_secret =
        WorkflowSecret::new("app-secret", "The app secret for the corresponding app ID");

    (app_id, app_secret)
}

pub(crate) struct RepositoryTarget {
    owner: Option<String>,
    repositories: Option<String>,
    permissions: Option<Vec<(String, Level)>>,
}

impl RepositoryTarget {
    pub fn new<T: ToString>(owner: T, repositories: &[&str]) -> Self {
        Self {
            owner: Some(owner.to_string()),
            repositories: Some(repositories.join("\n")),
            permissions: None,
        }
    }

    pub fn current() -> Self {
        Self {
            owner: None,
            repositories: None,
            permissions: None,
        }
    }

    pub fn permissions(self, permissions: impl Into<Vec<(String, Level)>>) -> Self {
        Self {
            permissions: Some(permissions.into()),
            ..self
        }
    }
}
