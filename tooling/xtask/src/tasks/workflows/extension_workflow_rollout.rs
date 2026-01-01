use gh_workflow::{
    Event, Expression, Job, Level, Run, Step, Strategy, Use, Workflow, WorkflowDispatch,
};
use indoc::indoc;
use serde_json::json;

use crate::tasks::workflows::{
    extension_bump::{RepositoryTarget, generate_token},
    runners,
    steps::{self, NamedJob, named},
    vars::{self, StepOutput},
};

const EXCLUDED_REPOS: &[&str] = &["workflows", "material-icon-theme"];

pub(crate) fn extension_workflow_rollout() -> Workflow {
    let fetch_repos = fetch_extension_repos();
    let rollout_workflows = rollout_workflows_to_extension(&fetch_repos);

    named::workflow()
        .on(Event::default().workflow_dispatch(WorkflowDispatch::default()))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_job(fetch_repos.name, fetch_repos.job)
        .add_job(rollout_workflows.name, rollout_workflows.job)
}

fn fetch_extension_repos() -> NamedJob {
    fn get_repositories() -> (Step<Use>, StepOutput) {
        let exclusion_filter = EXCLUDED_REPOS
            .iter()
            .map(|repo| format!("repo.name !== '{}'", repo))
            .collect::<Vec<_>>()
            .join(" && ");

        let step = named::uses("actions", "github-script", "v7")
            .id("list-repos")
            .add_with((
                "script",
                format!(
                    indoc! {r#"
                        const repos = await github.paginate(github.rest.repos.listForOrg, {{
                            org: 'zed-extensions',
                            type: 'public',
                            per_page: 100,
                        }});

                        const filteredRepos = repos
                            .filter(repo => !repo.archived)
                            .filter(repo => {})
                            .map(repo => repo.name);

                        console.log(`Found ${{filteredRepos.length}} extension repos`);
                        return filteredRepos;
                    "#},
                    exclusion_filter
                ),
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
    }

    fn checkout_extension_repo(token: &StepOutput) -> Step<Use> {
        steps::checkout_repo_with_token(token)
            .add_with(("repository", "zed-extensions/${{ matrix.repo }}"))
            .add_with(("path", "extension"))
    }

    fn copy_workflow_files() -> Step<Run> {
        named::bash(indoc! {r#"
            mkdir -p extension/.github/workflows
            cp zed/extensions/workflows/shared/*.yml extension/.github/workflows/
        "#})
    }

    fn get_short_sha() -> (Step<Run>, StepOutput) {
        let step = named::bash(indoc! {r#"
            echo "sha_short=$(git rev-parse --short HEAD)" >> "$GITHUB_OUTPUT"
        "#})
        .id("short-sha")
        .working_directory("zed");

        let step_output = StepOutput::new(&step, "sha_short");

        (step, step_output)
    }

    fn create_pull_request(token: &StepOutput, short_sha: StepOutput) -> Step<Use> {
        let title = format!("Update CI workflows to `zed@{}`", short_sha);

        named::uses("peter-evans", "create-pull-request", "v7")
            .add_with(("path", "extension"))
            .add_with(("title", title.clone()))
            .add_with((
                "body",
                indoc! {r#"
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
        named::bash(indoc! {r#"
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
        .add_step(copy_workflow_files())
        .add_step(calculate_short_sha)
        .add_step(create_pull_request(&token, short_sha))
        .add_step(enable_auto_merge(&token));

    named::job(job)
}
