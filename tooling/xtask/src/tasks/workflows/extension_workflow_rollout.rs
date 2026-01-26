use gh_workflow::{
    Event, Expression, Job, Level, Run, Step, Strategy, Use, Workflow, WorkflowDispatch,
};
use indoc::formatdoc;
use indoc::indoc;
use serde_json::json;

use crate::tasks::workflows::{
    extension_bump::{RepositoryTarget, generate_token},
    runners,
    steps::{self, NamedJob, named},
    vars::{self, StepOutput},
};

const ROLLOUT_TAG_NAME: &str = "extension-workflows";

pub(crate) fn extension_workflow_rollout() -> Workflow {
    let fetch_repos = fetch_extension_repos();
    let rollout_workflows = rollout_workflows_to_extension(&fetch_repos);
    let create_tag = create_rollout_tag(&rollout_workflows);

    named::workflow()
        .on(Event::default().workflow_dispatch(WorkflowDispatch::default()))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_job(fetch_repos.name, fetch_repos.job)
        .add_job(rollout_workflows.name, rollout_workflows.job)
        .add_job(create_tag.name, create_tag.job)
}

fn fetch_extension_repos() -> NamedJob {
    fn get_repositories() -> (Step<Use>, StepOutput) {
        let step = named::uses("actions", "github-script", "v7")
            .id("list-repos")
            .add_with((
                "script",
                indoc::indoc! {r#"
                    const repos = await github.paginate(github.rest.repos.listForOrg, {
                        org: 'zed-extensions',
                        type: 'public',
                        per_page: 100,
                    });

                    const filteredRepos = repos
                        .filter(repo => !repo.archived)
                        .map(repo => repo.name);

                    console.log(`Found ${filteredRepos.length} extension repos`);
                    return filteredRepos;
                "#},
            ))
            .add_with(("result-encoding", "json"));

        let filtered_repos = StepOutput::new(&step, "result");

        (step, filtered_repos)
    }

    let (get_org_repositories, list_repos_output) = get_repositories();

    let job = Job::default()
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(5u32)
        .outputs([("repos".to_owned(), list_repos_output.to_string())])
        .add_step(get_org_repositories);

    named::job(job)
}

fn rollout_workflows_to_extension(fetch_repos_job: &NamedJob) -> NamedJob {
    fn checkout_zed_repo() -> Step<Use> {
        steps::checkout_repo()
            .name("checkout_zed_repo")
            .add_with(("path", "zed"))
            .add_with(("fetch-depth", "0"))
    }

    fn checkout_extension_repo(token: &StepOutput) -> Step<Use> {
        steps::checkout_repo_with_token(token)
            .add_with(("repository", "zed-extensions/${{ matrix.repo }}"))
            .add_with(("path", "extension"))
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
        .id("prev-tag")
        .working_directory("zed");

        let step_output = StepOutput::new(&step, "prev_commit");

        (step, step_output)
    }

    fn get_removed_files(prev_commit: &StepOutput) -> (Step<Run>, StepOutput) {
        let step = named::bash(formatdoc! {r#"
            PREV_COMMIT="{prev_commit}"

            if [ "${{{{ matrix.repo }}}}" = "workflows" ]; then
                WORKFLOW_DIR="extensions/workflows"
            else
                WORKFLOW_DIR="extensions/workflows/shared"
            fi

            echo "Calculating changes from $PREV_COMMIT to HEAD for $WORKFLOW_DIR"

            # Get deleted files (status D) and renamed files (status R - old name needs removal)
            # Using -M to detect renames, then extracting files that are gone from their original location
            REMOVED_FILES=$(git diff --name-status -M "$PREV_COMMIT" HEAD -- "$WORKFLOW_DIR" | \
                awk '/^D/ {{ print $2 }} /^R/ {{ print $2 }}' | \
                xargs -I{{}} basename {{}} 2>/dev/null | \
                tr '\n' ' ' || echo "")

            REMOVED_FILES=$(echo "$REMOVED_FILES" | xargs)

            echo "Files to remove: $REMOVED_FILES"
            echo "removed_files=$REMOVED_FILES" >> "$GITHUB_OUTPUT"
        "#})
        .id("calc-changes")
        .working_directory("zed");

        let removed_files = StepOutput::new(&step, "removed_files");

        (step, removed_files)
    }

    fn sync_workflow_files(removed_files: &StepOutput) -> Step<Run> {
        named::bash(formatdoc! {r#"
            REMOVED_FILES="{removed_files}"

            mkdir -p extension/.github/workflows
            cd extension/.github/workflows

            if [ -n "$REMOVED_FILES" ]; then
                for file in $REMOVED_FILES; do
                    if [ -f "$file" ]; then
                        rm -f "$file"
                    fi
                done
            fi

            cd - > /dev/null

            if [ "${{{{ matrix.repo }}}}" = "workflows" ]; then
                cp zed/extensions/workflows/*.yml extension/.github/workflows/
            else
                cp zed/extensions/workflows/shared/*.yml extension/.github/workflows/
            fi
        "#})
    }

    fn get_short_sha() -> (Step<Run>, StepOutput) {
        let step = named::bash(indoc::indoc! {r#"
            echo "sha_short=$(git rev-parse --short HEAD)" >> "$GITHUB_OUTPUT"
        "#})
        .id("short-sha")
        .working_directory("zed");

        let step_output = StepOutput::new(&step, "sha_short");

        (step, step_output)
    }

    fn create_pull_request(token: &StepOutput, short_sha: &StepOutput) -> Step<Use> {
        let title = format!("Update CI workflows to `zed@{}`", short_sha);

        named::uses("peter-evans", "create-pull-request", "v7")
            .add_with(("path", "extension"))
            .add_with(("title", title.clone()))
            .add_with((
                "body",
                indoc::indoc! {r#"
                    This PR updates the CI workflow files from the main Zed repository
                    based on the commit zed-industries/zed@${{ github.sha }}
                "#},
            ))
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
        named::bash(indoc::indoc! {r#"
            PR_NUMBER="${{ steps.create-pr.outputs.pull-request-number }}"
            if [ -n "$PR_NUMBER" ]; then
                cd extension
                gh pr merge "$PR_NUMBER" --auto --squash
            fi
        "#})
        .add_env(("GH_TOKEN", token.to_string()))
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
    let (get_prev_tag, prev_commit) = get_previous_tag_commit();
    let (calc_changes, removed_files) = get_removed_files(&prev_commit);
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
                .max_parallel(5u32)
                .matrix(json!({
                    "repo": format!("${{{{ fromJson(needs.{}.outputs.repos) }}}}", fetch_repos_job.name)
                })),
        )
        .add_step(authenticate)
        .add_step(checkout_zed_repo())
        .add_step(checkout_extension_repo(&token))
        .add_step(get_prev_tag)
        .add_step(calc_changes)
        .add_step(sync_workflow_files(&removed_files))
        .add_step(calculate_short_sha)
        .add_step(create_pull_request(&token, &short_sha))
        .add_step(enable_auto_merge(&token));

    named::job(job)
}

fn create_rollout_tag(rollout_job: &NamedJob) -> NamedJob {
    fn checkout_zed_repo(token: &StepOutput) -> Step<Use> {
        steps::checkout_repo_with_token(token).add_with(("fetch-depth", "0"))
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
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(authenticate)
        .add_step(checkout_zed_repo(&token))
        .add_step(configure_git())
        .add_step(update_rollout_tag());

    named::job(job)
}
