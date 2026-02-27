use gh_workflow::{Event, Expression, Job, Push, Run, Step, Use, Workflow};

use crate::tasks::workflows::{
    runners::{self, Platform},
    steps::{self, FluentBuilder as _, NamedJob, named, release_job},
    vars,
};

pub(crate) fn lychee_link_check(dir: &str) -> Step<Use> {
    named::uses(
        "lycheeverse",
        "lychee-action",
        "82202e5e9c2f4ef1a55a3d02563e1cb6041e5332",
    ) // v2.4.1
    .add_with(("args", format!("--no-progress --exclude '^http' '{dir}'")))
    .add_with(("fail", true))
    .add_with(("jobSummary", false))
}

pub(crate) fn install_mdbook() -> Step<Use> {
    named::uses(
        "peaceiris",
        "actions-mdbook",
        "ee69d230fe19748b7abf22df32acaa93833fad08", // v2
    )
    .with(("mdbook-version", "0.4.37"))
}

pub(crate) fn build_docs_book() -> Step<Run> {
    named::bash(indoc::indoc! {r#"
        mkdir -p target/deploy
        mdbook build ./docs --dest-dir=../target/deploy/docs/
    "#})
}

fn deploy_docs_to_pages() -> Step<Use> {
    named::uses(
        "cloudflare",
        "wrangler-action",
        "da0e0dfe58b7a431659754fdf3f186c529afbe65",
    ) // v3
    .add_with(("apiToken", vars::CLOUDFLARE_API_TOKEN))
    .add_with(("accountId", vars::CLOUDFLARE_ACCOUNT_ID))
    .add_with(("command", "pages deploy target/deploy --project-name=docs"))
}

fn deploy_install_script() -> Step<Use> {
    named::uses(
        "cloudflare",
        "wrangler-action",
        "da0e0dfe58b7a431659754fdf3f186c529afbe65",
    ) // v3
    .add_with(("apiToken", vars::CLOUDFLARE_API_TOKEN))
    .add_with(("accountId", vars::CLOUDFLARE_ACCOUNT_ID))
    .add_with((
        "command",
        "r2 object put -f script/install.sh zed-open-source-website-assets/install.sh",
    ))
}

fn deploy_docs_worker() -> Step<Use> {
    named::uses(
        "cloudflare",
        "wrangler-action",
        "da0e0dfe58b7a431659754fdf3f186c529afbe65",
    ) // v3
    .add_with(("apiToken", vars::CLOUDFLARE_API_TOKEN))
    .add_with(("accountId", vars::CLOUDFLARE_ACCOUNT_ID))
    .add_with(("command", "deploy .cloudflare/docs-proxy/src/worker.js"))
}

fn upload_wrangler_logs() -> Step<Use> {
    named::uses(
        "actions",
        "upload-artifact",
        "ea165f8d65b6e75b540449e92b4886f43607fa02",
    ) // v4
    .if_condition(Expression::new("always()"))
    .add_with(("name", "wrangler_logs"))
    .add_with(("path", "/home/runner/.config/.wrangler/logs/"))
}

fn docs_build_steps(job: Job) -> Job {
    job.add_env(("DOCS_AMPLITUDE_API_KEY", vars::DOCS_AMPLITUDE_API_KEY))
        .runs_on(runners::LINUX_XL)
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_cargo_config(Platform::Linux))
        .add_step(steps::cache_rust_dependencies_namespace())
        .map(steps::install_linux_dependencies)
        .add_step(steps::script("./script/generate-action-metadata"))
        .add_step(lychee_link_check("./docs/src/**/*"))
        .add_step(install_mdbook())
        .add_step(build_docs_book())
        .add_step(lychee_link_check("target/deploy/docs"))
}

pub(crate) fn check_docs() -> NamedJob {
    NamedJob {
        name: "check_docs".to_owned(),
        job: docs_build_steps(release_job(&[])),
    }
}

pub(crate) fn deploy_docs_job() -> NamedJob {
    named::job(
        docs_build_steps(
            release_job(&[])
                .name("Build and Deploy Docs")
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                )),
        )
        .add_step(deploy_docs_to_pages())
        .add_step(deploy_install_script())
        .add_step(deploy_docs_worker())
        .add_step(upload_wrangler_logs()),
    )
}

pub(crate) fn deploy_docs() -> Workflow {
    let deploy_docs = deploy_docs_job();

    named::workflow()
        .add_event(
            Event::default().push(
                Push::default()
                    .add_branch("main")
                    // todo! remove
                    .add_branch("staged-docs-releases"),
            ),
        )
        .add_job(deploy_docs.name, deploy_docs.job)
}
