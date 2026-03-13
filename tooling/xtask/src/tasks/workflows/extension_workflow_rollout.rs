use gh_workflow::{
    Event, Expression, Job, Level, Run, Step, Strategy, Use, Workflow, WorkflowDispatch,
};
use indoc::formatdoc;
use indoc::indoc;
use serde_json::json;

use crate::tasks::workflows::steps::CheckoutStep;
use crate::tasks::workflows::steps::cache_rust_dependencies_namespace;
use crate::tasks::workflows::vars::JobOutput;
use crate::tasks::workflows::{
    extension_bump::{RepositoryTarget, generate_token},
    runners,
    steps::{self, DEFAULT_REPOSITORY_OWNER_GUARD, NamedJob, named},
    vars::{self, StepOutput, WorkflowInput},
};

const ROLLOUT_TAG_NAME: &str = "extension-workflows";
const WORKFLOW_ARTIFACT_NAME: &str = "extension-workflow-files";

pub(crate) fn extension_workflow_rollout() -> Workflow {
    let filter_repos_input = WorkflowInput::string("filter-repos", Some(String::new()))
        .description(
            "Comma-separated list of repository names to rollout to. Leave empty for all repos.",
        );
    let extra_context_input = WorkflowInput::string("change-description", Some(String::new()))
        .description("Description for the changes to be expected with this rollout");

    let (fetch_repos, removed_ci, removed_shared) = fetch_extension_repos(&filter_repos_input);
    let rollout_workflows = rollout_workflows_to_extension(
        &fetch_repos,
        removed_ci,
        removed_shared,
        &extra_context_input,
    );
    let create_tag = create_rollout_tag(&rollout_workflows, &filter_repos_input);

    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(filter_repos_input.name, filter_repos_input.input())
                .add_input(extra_context_input.name, extra_context_input.input()),
        ))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_job(fetch_repos.name, fetch_repos.job)
        .add_job(rollout_workflows.name, rollout_workflows.job)
        .add_job(create_tag.name, create_tag.job)
}

fn fetch_extension_repos(filter_repos_input: &WorkflowInput) -> (NamedJob, JobOutput, JobOutput) {
    fn get_repositories(filter_repos_input: &WorkflowInput) -> (Step<Use>, StepOutput) {
        let step = named::uses("actions", "github-script", "v7")
            .id("list-repos")
            .add_with((
                "script",
                formatdoc! {r#"
                    const repos = await github.paginate(github.rest.repos.listForOrg, {{
                        org: 'zed-extensions',
                        type: 'public',
                        per_page: 100,
                    }});

                    let filteredRepos = repos
                        .filter(repo => !repo.archived)
                        .map(repo => repo.name);

                    const filterInput = `{filter_repos_input}`.trim();
                    if (filterInput.length > 0) {{
                        const allowedNames = filterInput.split(',').map(s => s.trim()).filter(s => s.length > 0);
                        filteredRepos = filteredRepos.filter(name => allowedNames.includes(name));
                        console.log(`Filter applied. Matched ${{filteredRepos.length}} repos from ${{allowedNames.length}} requested.`);
                    }}

                    console.log(`Found ${{filteredRepos.length}} extension repos`);
                    return filteredRepos;
                "#},
            ))
            .add_with(("result-encoding", "json"));

        let filtered_repos = StepOutput::new(&step, "result");

        (step, filtered_repos)
    }

    fn checkout_zed_repo() -> CheckoutStep {
        steps::checkout_repo()
            .with_full_history()
            .with_custom_name("checkout_zed_repo")
    }

    fn get_previous_tag_commit() -> (Step<Run>, StepOutput) {
        let step = named::bash(formatdoc! {r#"
            PREV_COMMIT=$(git rev-parse "{ROLLOUT_TAG_NAME}^{{commit}}" 2>/dev/null || echo "")
            if [ -z "$PREV_COMMIT" ]; then
                echo "::error::No previous rollout tag '{ROLLOUT_TAG_NAME}' found. Cannot determine file changes."
                exit 1
            fi
            echo "Found previous rollout at commit: $PREV_COMMIT"
            echo "prev_commit=$PREV_COMMIT" >> "$GITHUB_OUTPUT"
        "#})
        .id("prev-tag");

        let step_output = StepOutput::new(&step, "prev_commit");

        (step, step_output)
    }

    fn get_removed_files(prev_commit: &StepOutput) -> (Step<Run>, StepOutput, StepOutput) {
        let step = named::bash(indoc! {r#"
            for workflow_type in "ci" "shared"; do
                if [ "$workflow_type" = "ci" ]; then
                    WORKFLOW_DIR="extensions/workflows"
                else
                    WORKFLOW_DIR="extensions/workflows/shared"
                fi

                REMOVED=$(git diff --name-status -M "$PREV_COMMIT" HEAD -- "$WORKFLOW_DIR" | \
                    awk '/^D/ { print $2 } /^R/ { print $2 }' | \
                    xargs -I{} basename {} 2>/dev/null | \
                    tr '\n' ' ' || echo "")
                REMOVED=$(echo "$REMOVED" | xargs)

                echo "Removed files for $workflow_type: $REMOVED"
                echo "removed_${workflow_type}=$REMOVED" >> "$GITHUB_OUTPUT"
            done
        "#})
        .id("calc-changes")
        .add_env(("PREV_COMMIT", prev_commit.to_string()));

        // These are created in the for-loop above and thus do exist
        let removed_ci = StepOutput::new_unchecked(&step, "removed_ci");
        let removed_shared = StepOutput::new_unchecked(&step, "removed_shared");

        (step, removed_ci, removed_shared)
    }

    fn generate_workflow_files() -> Step<Run> {
        named::bash(indoc! {r#"
            cargo xtask workflows "$COMMIT_SHA"
        "#})
        .add_env(("COMMIT_SHA", "${{ github.sha }}"))
    }

    fn upload_workflow_files() -> Step<Use> {
        named::uses(
            "actions",
            "upload-artifact",
            "330a01c490aca151604b8cf639adc76d48f6c5d4", // v5
        )
        .add_with(("name", WORKFLOW_ARTIFACT_NAME))
        .add_with(("path", "extensions/workflows/**/*.yml"))
        .add_with(("if-no-files-found", "error"))
    }

    let (get_org_repositories, list_repos_output) = get_repositories(filter_repos_input);
    let (get_prev_tag, prev_commit) = get_previous_tag_commit();
    let (calc_changes, removed_ci, removed_shared) = get_removed_files(&prev_commit);

    let job = Job::default()
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} && github.ref == 'refs/heads/main'"
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(10u32)
        .outputs([
            ("repos".to_owned(), list_repos_output.to_string()),
            ("prev_commit".to_owned(), prev_commit.to_string()),
            ("removed_ci".to_owned(), removed_ci.to_string()),
            ("removed_shared".to_owned(), removed_shared.to_string()),
        ])
        .add_step(checkout_zed_repo())
        .add_step(get_prev_tag)
        .add_step(calc_changes)
        .add_step(get_org_repositories)
        .add_step(cache_rust_dependencies_namespace())
        .add_step(generate_workflow_files())
        .add_step(upload_workflow_files());

    let job = named::job(job);
    let (removed_ci, removed_shared) = (
        removed_ci.as_job_output(&job),
        removed_shared.as_job_output(&job),
    );

    (job, removed_ci, removed_shared)
}

fn rollout_workflows_to_extension(
    fetch_repos_job: &NamedJob,
    removed_ci: JobOutput,
    removed_shared: JobOutput,
    extra_context_input: &WorkflowInput,
) -> NamedJob {
    fn checkout_extension_repo(token: &StepOutput) -> CheckoutStep {
        steps::checkout_repo()
            .with_custom_name("checkout_extension_repo")
            .with_token(token)
            .with_repository("zed-extensions/${{ matrix.repo }}")
            .with_path("extension")
    }

    fn download_workflow_files() -> Step<Use> {
        named::uses(
            "actions",
            "download-artifact",
            "018cc2cf5baa6db3ef3c5f8a56943fffe632ef53", // v6.0.0
        )
        .add_with(("name", WORKFLOW_ARTIFACT_NAME))
        .add_with(("path", "workflow-files"))
    }

    fn sync_workflow_files(removed_ci: JobOutput, removed_shared: JobOutput) -> Step<Run> {
        named::bash(indoc! {r#"
            mkdir -p extension/.github/workflows

            if [ "$MATRIX_REPO" = "workflows" ]; then
                REMOVED_FILES="$REMOVED_CI"
            else
                REMOVED_FILES="$REMOVED_SHARED"
            fi

            cd extension/.github/workflows

            if [ -n "$REMOVED_FILES" ]; then
                for file in $REMOVED_FILES; do
                    if [ -f "$file" ]; then
                        rm -f "$file"
                    fi
                done
            fi

            cd - > /dev/null

            if [ "$MATRIX_REPO" = "workflows" ]; then
                cp workflow-files/*.yml extension/.github/workflows/
            else
                cp workflow-files/shared/*.yml extension/.github/workflows/
            fi
        "#})
        .add_env(("REMOVED_CI", removed_ci))
        .add_env(("REMOVED_SHARED", removed_shared))
        .add_env(("MATRIX_REPO", "${{ matrix.repo }}"))
    }

    fn get_short_sha() -> (Step<Run>, StepOutput) {
        let step = named::bash(indoc! {r#"
            echo "sha_short=$(echo "$GITHUB_SHA" | cut -c1-7)" >> "$GITHUB_OUTPUT"
        "#})
        .id("short-sha");

        let step_output = StepOutput::new(&step, "sha_short");

        (step, step_output)
    }

    fn create_pull_request(
        token: &StepOutput,
        short_sha: &StepOutput,
        context_input: &WorkflowInput,
    ) -> Step<Use> {
        let title = format!("Update CI workflows to `{short_sha}`");

        let body = formatdoc! {r#"
            This PR updates the CI workflow files from the main Zed repository
            based on the commit zed-industries/zed@${{{{ github.sha }}}}

            {context_input}
        "#,
        };

        named::uses("peter-evans", "create-pull-request", "v7")
            .add_with(("path", "extension"))
            .add_with(("title", title.clone()))
            .add_with(("body", body))
            .add_with(("commit-message", title))
            .add_with(("branch", "update-workflows"))
            .add_with((
                "committer",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            ))
            .add_with((
                "author",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            ))
            .add_with(("base", "main"))
            .add_with(("delete-branch", true))
            .add_with(("token", token.to_string()))
            .add_with(("sign-commits", true))
            .id("create-pr")
    }

    fn enable_auto_merge(token: &StepOutput) -> Step<gh_workflow::Run> {
        named::bash(indoc! {r#"
            if [ -n "$PR_NUMBER" ]; then
                gh pr merge "$PR_NUMBER" --auto --squash
            fi
        "#})
        .working_directory("extension")
        .add_env(("GH_TOKEN", token.to_string()))
        .add_env((
            "PR_NUMBER",
            "${{ steps.create-pr.outputs.pull-request-number }}",
        ))
    }

    let (authenticate, token) = generate_token(
        vars::ZED_ZIPPY_APP_ID,
        vars::ZED_ZIPPY_APP_PRIVATE_KEY,
        Some(
            RepositoryTarget::new("zed-extensions", &["${{ matrix.repo }}"]).permissions([
                ("permission-pull-requests".to_owned(), Level::Write),
                ("permission-contents".to_owned(), Level::Write),
                ("permission-workflows".to_owned(), Level::Write),
            ]),
        ),
    );
    let (calculate_short_sha, short_sha) = get_short_sha();

    let job = Job::default()
        .needs([fetch_repos_job.name.clone()])
        .cond(Expression::new(format!(
            "needs.{}.outputs.repos != '[]'",
            fetch_repos_job.name
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(10u32)
        .strategy(
            Strategy::default()
                .fail_fast(false)
                .max_parallel(10u32)
                .matrix(json!({
                    "repo": format!("${{{{ fromJson(needs.{}.outputs.repos) }}}}", fetch_repos_job.name)
                })),
        )
        .add_step(authenticate)
        .add_step(checkout_extension_repo(&token))
        .add_step(download_workflow_files())
        .add_step(sync_workflow_files(removed_ci, removed_shared))
        .add_step(calculate_short_sha)
        .add_step(create_pull_request(&token, &short_sha, extra_context_input))
        .add_step(enable_auto_merge(&token));

    named::job(job)
}

fn create_rollout_tag(rollout_job: &NamedJob, filter_repos_input: &WorkflowInput) -> NamedJob {
    fn checkout_zed_repo(token: &StepOutput) -> CheckoutStep {
        steps::checkout_repo().with_full_history().with_token(token)
    }

    fn update_rollout_tag() -> Step<Run> {
        named::bash(formatdoc! {r#"
            if git rev-parse "{ROLLOUT_TAG_NAME}" >/dev/null 2>&1; then
                git tag -d "{ROLLOUT_TAG_NAME}"
                git push origin ":refs/tags/{ROLLOUT_TAG_NAME}" || true
            fi

            echo "Creating new tag '{ROLLOUT_TAG_NAME}' at $(git rev-parse --short HEAD)"
            git tag "{ROLLOUT_TAG_NAME}"
            git push origin "{ROLLOUT_TAG_NAME}"
        "#})
    }

    fn configure_git() -> Step<Run> {
        named::bash(indoc! {r#"
            git config user.name "zed-zippy[bot]"
            git config user.email "234243425+zed-zippy[bot]@users.noreply.github.com"
        "#})
    }

    let (authenticate, token) = generate_token(
        vars::ZED_ZIPPY_APP_ID,
        vars::ZED_ZIPPY_APP_PRIVATE_KEY,
        Some(
            RepositoryTarget::current()
                .permissions([("permission-contents".to_owned(), Level::Write)]),
        ),
    );

    let job = Job::default()
        .needs([rollout_job.name.clone()])
        .cond(Expression::new(format!(
            "{filter_repos} == ''",
            filter_repos = filter_repos_input.expr(),
        )))
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(authenticate)
        .add_step(checkout_zed_repo(&token))
        .add_step(configure_git())
        .add_step(update_rollout_tag());

    named::job(job)
}
