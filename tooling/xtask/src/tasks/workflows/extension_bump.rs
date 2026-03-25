use gh_workflow::{ctx::Context, *};
use indoc::{formatdoc, indoc};

use crate::tasks::workflows::{
    extension_tests::{self},
    runners,
    steps::{
        self, BASH_SHELL, CommonJobConditions, DEFAULT_REPOSITORY_OWNER_GUARD, NamedJob,
        RepositoryTarget, cache_rust_dependencies_namespace, checkout_repo, dependant_job,
        generate_token, named,
    },
    vars::{
        JobOutput, StepOutput, WorkflowInput, WorkflowSecret,
        one_workflow_per_non_main_branch_and_token,
    },
};

const VERSION_CHECK: &str =
    r#"sed -n 's/^version = \"\(.*\)\"/\1/p' < extension.toml | tr -d '[:space:]'"#;

// This is used by various extensions repos in the zed-extensions org to bump extension versions.
pub(crate) fn extension_bump() -> Workflow {
    let bump_type = WorkflowInput::string("bump-type", Some("patch".to_owned()));
    // TODO: Ideally, this would have a default of `false`, but this is currently not
    // supported in gh-workflows
    let force_bump = WorkflowInput::bool("force-bump", None);
    let working_directory = WorkflowInput::string("working-directory", Some(".".to_owned()));

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
    let (create_label, tag) = create_version_label(
        &dependencies,
        &version_changed,
        &current_version,
        &app_id,
        &app_secret,
    );
    let tag = tag.as_job_output(&create_label);
    let trigger_release = trigger_release(
        &[&check_version_changed, &create_label],
        tag,
        &app_id,
        &app_secret,
    );

    named::workflow()
        .add_event(
            Event::default().workflow_call(
                WorkflowCall::default()
                    .add_input(bump_type.name, bump_type.call_input())
                    .add_input(force_bump.name, force_bump.call_input())
                    .add_input(working_directory.name, working_directory.call_input())
                    .secrets([
                        (app_id.name.to_owned(), app_id.secret_configuration()),
                        (
                            app_secret.name.to_owned(),
                            app_secret.secret_configuration(),
                        ),
                    ]),
            ),
        )
        .concurrency(one_workflow_per_non_main_branch_and_token("extension-bump"))
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

fn extension_job_defaults() -> Defaults {
    Defaults::default().run(
        RunDefaults::default()
            .shell(BASH_SHELL)
            .working_directory("${{ inputs.working-directory }}"),
    )
}

fn check_version_changed() -> (NamedJob, StepOutput, StepOutput) {
    let (compare_versions, version_changed, current_version) = compare_versions();

    let job = Job::default()
        .defaults(extension_job_defaults())
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
        .add_step(steps::checkout_repo().with_full_history())
        .add_step(compare_versions);

    (named::job(job), version_changed, current_version)
}

fn create_version_label(
    dependencies: &[&NamedJob],
    version_changed_output: &JobOutput,
    current_version: &JobOutput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> (NamedJob, StepOutput) {
    let (generate_token, generated_token) =
        generate_token(&app_id.to_string(), &app_secret.to_string()).into();
    let (determine_tag_step, tag) = determine_tag(current_version);
    let job = steps::dependant_job(dependencies)
        .defaults(extension_job_defaults())
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} && github.event_name == 'push' && \
            github.ref == 'refs/heads/main' && {version_changed} == 'true'",
            version_changed = version_changed_output.expr(),
        )))
        .outputs([(tag.name.to_owned(), tag.to_string())])
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(determine_tag_step)
        .add_step(create_version_tag(&tag, generated_token));

    (named::job(job), tag)
}

fn create_version_tag(tag: &StepOutput, generated_token: StepOutput) -> Step<Use> {
    named::uses("actions", "github-script", "v7").with(
        Input::default()
            .add(
                "script",
                formatdoc! {r#"
                    github.rest.git.createRef({{
                        owner: context.repo.owner,
                        repo: context.repo.repo,
                        ref: 'refs/tags/{tag}',
                        sha: context.sha
                    }})"#
                },
            )
            .add("github-token", generated_token.to_string()),
    )
}

fn determine_tag(current_version: &JobOutput) -> (Step<Run>, StepOutput) {
    let step = named::bash(formatdoc! {r#"
        EXTENSION_ID="$(sed -n 's/^id = "\(.*\)"/\1/p' < extension.toml | head -1 | tr -d '[:space:]')"

        if [[ "$WORKING_DIR" == "." || -z "$WORKING_DIR" ]]; then
            TAG="v${{CURRENT_VERSION}}"
        else
            TAG="${{EXTENSION_ID}}-v${{CURRENT_VERSION}}"
        fi

        echo "tag=${{TAG}}" >> "$GITHUB_OUTPUT"
    "#})
    .id("determine-tag")
    .add_env(("CURRENT_VERSION", current_version.to_string()))
    .add_env(("WORKING_DIR", "${{ inputs.working-directory }}"));

    let tag = StepOutput::new(&step, "tag");
    (step, tag)
}

/// Compares the current and previous commit and checks whether versions changed inbetween.
pub(crate) fn compare_versions() -> (Step<Run>, StepOutput, StepOutput) {
    let check_needs_bump = named::bash(formatdoc! {
    r#"
        CURRENT_VERSION="$({VERSION_CHECK})"

        if [[ "$GITHUB_EVENT_NAME" == "pull_request" ]]; then
            PR_FORK_POINT="$(git merge-base origin/main HEAD)"
            git checkout "$PR_FORK_POINT"
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
        generate_token(&app_id.to_string(), &app_secret.to_string()).into();
    let (bump_version, _new_version, title, body, branch_name) =
        bump_version(current_version, bump_type);

    let job = steps::dependant_job(dependencies)
        .defaults(extension_job_defaults())
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} &&\n({force_bump} == true || {version_changed} == 'false')",
            force_bump = force_bump_output.expr(),
            version_changed = version_changed_output.expr(),
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(5u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(cache_rust_dependencies_namespace())
        .add_step(install_bump_2_version())
        .add_step(bump_version)
        .add_step(create_pull_request(
            title,
            body,
            generated_token,
            branch_name,
        ));

    named::job(job)
}

fn install_bump_2_version() -> Step<Run> {
    named::run(
        runners::Platform::Linux,
        "pip install bump2version --break-system-packages",
    )
}

fn bump_version(
    current_version: &JobOutput,
    bump_type: &WorkflowInput,
) -> (Step<Run>, StepOutput, StepOutput, StepOutput, StepOutput) {
    let step = named::bash(formatdoc! {r#"
        BUMP_FILES=("extension.toml")
        if [[ -f "Cargo.toml" ]]; then
            BUMP_FILES+=("Cargo.toml")
        fi

        bump2version \
            --search "version = \"{{current_version}}"\" \
            --replace "version = \"{{new_version}}"\" \
            --current-version "$OLD_VERSION" \
            --no-configured-files "$BUMP_TYPE" "${{BUMP_FILES[@]}}"

        if [[ -f "Cargo.toml" ]]; then
            cargo +stable update --workspace
        fi

        NEW_VERSION="$({VERSION_CHECK})"
        EXTENSION_ID="$(sed -n 's/^id = "\(.*\)"/\1/p' < extension.toml | head -1 | tr -d '[:space:]')"
        EXTENSION_NAME="$(sed -n 's/^name = "\(.*\)"/\1/p' < extension.toml | head -1 | tr -d '[:space:]')"

        if [[ "$WORKING_DIR" == "." || -z "$WORKING_DIR" ]]; then
            {{
                echo "title=Bump version to ${{NEW_VERSION}}";
                echo "body=This PR bumps the version of this extension to v${{NEW_VERSION}}";
                echo "branch_name=zed-zippy-autobump";
            }} >> "$GITHUB_OUTPUT"
        else
            {{
                echo "title=${{EXTENSION_ID}}: Bump to v${{NEW_VERSION}}";
                echo "body<<EOF";
                echo "This PR bumps the version of the ${{EXTENSION_NAME}} extension to v${{NEW_VERSION}}.";
                echo "";
                echo "Release Notes:";
                echo "";
                echo "- N/A";
                echo "EOF";
                echo "branch_name=zed-zippy-${{EXTENSION_ID}}-autobump";
            }} >> "$GITHUB_OUTPUT"
        fi

        echo "new_version=${{NEW_VERSION}}" >> "$GITHUB_OUTPUT"
        "#
    })
    .id("bump-version")
    .add_env(("OLD_VERSION", current_version.to_string()))
    .add_env(("BUMP_TYPE", bump_type.to_string()))
    .add_env(("WORKING_DIR", "${{ inputs.working-directory }}"));

    let new_version = StepOutput::new(&step, "new_version");
    let title = StepOutput::new(&step, "title");
    let body = StepOutput::new(&step, "body");
    let branch_name = StepOutput::new(&step, "branch_name");
    (step, new_version, title, body, branch_name)
}

fn create_pull_request(
    title: StepOutput,
    body: StepOutput,
    generated_token: StepOutput,
    branch_name: StepOutput,
) -> Step<Use> {
    named::uses(
        "peter-evans",
        "create-pull-request",
        "98357b18bf14b5342f975ff684046ec3b2a07725",
    )
    .with(
        Input::default()
            .add("title", title.to_string())
            .add("body", body.to_string())
            .add("commit-message", title.to_string())
            .add("branch", branch_name.to_string())
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
    tag: JobOutput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> NamedJob {
    let extension_registry = RepositoryTarget::new("zed-industries", &["extensions"]);
    let (generate_token, generated_token) =
        generate_token(&app_id.to_string(), &app_secret.to_string())
            .for_repository(extension_registry);
    let (get_extension_id, extension_id) = get_extension_id();
    let (release_action, pull_request_number) = release_action(extension_id, tag, &generated_token);

    let job = dependant_job(dependencies)
        .defaults(extension_job_defaults())
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_SMALL)
        .add_step(generate_token)
        .add_step(checkout_repo())
        .add_step(get_extension_id)
        .add_step(release_action)
        .add_step(enable_automerge_if_staff(
            pull_request_number,
            generated_token,
        ));

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
    tag: JobOutput,
    generated_token: &StepOutput,
) -> (Step<Use>, StepOutput) {
    let step = named::uses(
        "huacnlee",
        "zed-extension-action",
        "82920ff0876879f65ffbcfa3403589114a8919c6",
    )
    .id("extension-update")
    .add_with(("extension-name", extension_id.to_string()))
    .add_with(("push-to", "zed-industries/extensions"))
    .add_with(("tag", tag.to_string()))
    .add_env(("COMMITTER_TOKEN", generated_token.to_string()));

    let pull_request_number = StepOutput::new(&step, "pull-request-number");

    (step, pull_request_number)
}

fn enable_automerge_if_staff(
    pull_request_number: StepOutput,
    generated_token: StepOutput,
) -> Step<Use> {
    named::uses("actions", "github-script", "v7")
        .add_with(("github-token", generated_token.to_string()))
        .add_with((
            "script",
            indoc! {r#"
                const prNumber = process.env.PR_NUMBER;
                if (!prNumber) {
                    console.log('No pull request number set, skipping automerge.');
                    return;
                }

                const author = process.env.GITHUB_ACTOR;
                let isStaff = false;
                try {
                    const response = await github.rest.teams.getMembershipForUserInOrg({
                        org: 'zed-industries',
                        team_slug: 'staff',
                        username: author
                    });
                    isStaff = response.data.state === 'active';
                } catch (error) {
                    if (error.status !== 404) {
                        throw error;
                    }
                }

                if (!isStaff) {
                    console.log(`Actor ${author} is not a staff member, skipping automerge.`);
                    return;
                }

                // Assign staff member responsible for the bump
                const pullNumber = parseInt(prNumber);

                await github.rest.issues.addAssignees({
                    owner: 'zed-industries',
                    repo: 'extensions',
                    issue_number: pullNumber,
                    assignees: [author]
                });
                console.log(`Assigned ${author} to PR #${prNumber} in zed-industries/extensions`);

                // Get the GraphQL node ID
                const { data: pr } = await github.rest.pulls.get({
                    owner: 'zed-industries',
                    repo: 'extensions',
                    pull_number: pullNumber
                });

                await github.graphql(`
                    mutation($pullRequestId: ID!) {
                        enablePullRequestAutoMerge(input: { pullRequestId: $pullRequestId, mergeMethod: SQUASH }) {
                            pullRequest {
                                autoMergeRequest {
                                    enabledAt
                                }
                            }
                        }
                    }
                `, { pullRequestId: pr.node_id });

                console.log(`Automerge enabled for PR #${prNumber} in zed-industries/extensions`);
            "#},
        ))
        .add_env(("PR_NUMBER", pull_request_number.to_string()))
}

fn extension_workflow_secrets() -> (WorkflowSecret, WorkflowSecret) {
    let app_id = WorkflowSecret::new("app-id", "The app ID used to create the PR");
    let app_secret =
        WorkflowSecret::new("app-secret", "The app secret for the corresponding app ID");

    (app_id, app_secret)
}
