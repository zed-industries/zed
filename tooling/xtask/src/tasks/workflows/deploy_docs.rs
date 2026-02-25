use gh_workflow::{Event, Expression, Job, Push, Run, Step, Use, Workflow, WorkflowDispatch};

use crate::tasks::workflows::{
    runners,
    steps::{self, FluentBuilder as _, NamedJob, named, release_job},
    vars::{self, StepOutput, WorkflowInput},
};

pub(crate) enum DocsChannel {
    Nightly,
    Preview,
    Stable,
}

impl DocsChannel {
    pub(crate) fn site_url(&self) -> &'static str {
        match self {
            Self::Nightly => "/docs/nightly/",
            Self::Preview => "/docs/preview/",
            Self::Stable => "/docs/",
        }
    }

    pub(crate) fn project_name(&self) -> &'static str {
        match self {
            Self::Nightly => "docs-nightly",
            Self::Preview => "docs-preview",
            Self::Stable => "docs",
        }
    }

    pub(crate) fn channel_name(&self) -> &'static str {
        match self {
            Self::Nightly => "nightly",
            Self::Preview => "preview",
            Self::Stable => "stable",
        }
    }
}

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

fn pages_deploy_step(project_name: &StepOutput) -> Step<Use> {
    named::uses(
        "cloudflare",
        "wrangler-action",
        "da0e0dfe58b7a431659754fdf3f186c529afbe65",
    ) // v3
    .add_with(("apiToken", vars::CLOUDFLARE_API_TOKEN))
    .add_with(("accountId", vars::CLOUDFLARE_ACCOUNT_ID))
    .add_with((
        "command",
        format!(
            "pages deploy target/deploy --project-name=${{{{ {} }}}}",
            project_name.expr()
        ),
    ))
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

fn resolve_channel_step(
    channel_input: &WorkflowInput,
) -> (Step<Run>, StepOutput, StepOutput, StepOutput) {
    let step = named::bash(format!(
        indoc::indoc! {r#"
            if [ "${{{{ github.event_name }}}}" = "workflow_dispatch" ]; then
                CHANNEL="${{{{ {dispatch_channel} }}}}"
            else
                case "${{{{ github.ref }}}}" in
                    "refs/heads/main")
                        CHANNEL="nightly"
                        ;;
                    "refs/heads/preview")
                        CHANNEL="preview"
                        ;;
                    "refs/heads/stable")
                        CHANNEL="stable"
                        ;;
                    *)
                        echo "::error::Unsupported ref for docs deploy: ${{{{ github.ref }}}}"
                        exit 1
                        ;;
                esac
            fi

            case "$CHANNEL" in
                "nightly")
                    SITE_URL="{nightly_site_url}"
                    PROJECT_NAME="{nightly_project_name}"
                    ;;
                "preview")
                    SITE_URL="{preview_site_url}"
                    PROJECT_NAME="{preview_project_name}"
                    ;;
                "stable")
                    SITE_URL="{stable_site_url}"
                    PROJECT_NAME="{stable_project_name}"
                    ;;
                *)
                    echo "::error::Invalid docs channel '$CHANNEL'. Expected one of: nightly, preview, stable."
                    exit 1
                    ;;
            esac

            echo "channel=$CHANNEL" >> "$GITHUB_OUTPUT"
            echo "site_url=$SITE_URL" >> "$GITHUB_OUTPUT"
            echo "project_name=$PROJECT_NAME" >> "$GITHUB_OUTPUT"
        "#},
        dispatch_channel = channel_input.expr(),
        nightly_site_url = DocsChannel::Nightly.site_url(),
        preview_site_url = DocsChannel::Preview.site_url(),
        stable_site_url = DocsChannel::Stable.site_url(),
        nightly_project_name = DocsChannel::Nightly.project_name(),
        preview_project_name = DocsChannel::Preview.project_name(),
        stable_project_name = DocsChannel::Stable.project_name(),
    ))
    .id("resolve-channel");

    let channel = StepOutput::new(&step, "channel");
    let site_url = StepOutput::new(&step, "site_url");
    let project_name = StepOutput::new(&step, "project_name");
    (step, channel, site_url, project_name)
}

fn static_channel_resolution_step(
    channel: DocsChannel,
) -> (Step<Run>, StepOutput, StepOutput, StepOutput) {
    let (channel_name, site_url, project_name) = match channel {
        DocsChannel::Nightly => (
            DocsChannel::Nightly.channel_name(),
            DocsChannel::Nightly.site_url(),
            DocsChannel::Nightly.project_name(),
        ),
        DocsChannel::Preview => (
            DocsChannel::Preview.channel_name(),
            DocsChannel::Preview.site_url(),
            DocsChannel::Preview.project_name(),
        ),
        DocsChannel::Stable => (
            DocsChannel::Stable.channel_name(),
            DocsChannel::Stable.site_url(),
            DocsChannel::Stable.project_name(),
        ),
    };

    let step = named::bash(format!(
        indoc::indoc! {r#"
            echo "channel={channel_name}" >> "$GITHUB_OUTPUT"
            echo "site_url={site_url}" >> "$GITHUB_OUTPUT"
            echo "project_name={project_name}" >> "$GITHUB_OUTPUT"
        "#},
        channel_name = channel_name,
        site_url = site_url,
        project_name = project_name,
    ))
    .id("resolve-channel");

    let channel = StepOutput::new(&step, "channel");
    let site_url = StepOutput::new(&step, "site_url");
    let project_name = StepOutput::new(&step, "project_name");
    (step, channel, site_url, project_name)
}

fn docs_build_steps(
    job: Job,
    resolved_channel_step: Step<Run>,
    channel: &StepOutput,
    site_url: &StepOutput,
    project_name: &StepOutput,
    include_deploy_steps: bool,
) -> Job {
    let mut job = job
        .add_env(("DOCS_AMPLITUDE_API_KEY", vars::DOCS_AMPLITUDE_API_KEY))
        .add_step(steps::checkout_repo())
        .add_step(resolved_channel_step)
        .add_env(("MDBOOK_BOOK__SITE_URL", site_url.to_string()))
        .add_env(("DOCS_CHANNEL", channel.to_string()))
        .runs_on(runners::LINUX_XL)
        .add_step(steps::setup_cargo_config(runners::Platform::Linux))
        .add_step(steps::cache_rust_dependencies_namespace())
        .map(steps::install_linux_dependencies)
        .add_step(steps::script("./script/generate-action-metadata"))
        .add_step(lychee_link_check("./docs/src/**/*"))
        .add_step(install_mdbook())
        .add_step(build_docs_book())
        .add_step(lychee_link_check("target/deploy/docs"));

    if include_deploy_steps {
        job = job
            .add_step(pages_deploy_step(project_name))
            .add_step(deploy_install_script())
            .add_step(deploy_docs_worker())
            .add_step(upload_wrangler_logs());
    }

    job
}

pub(crate) fn check_docs() -> NamedJob {
    let (resolve_step, channel, site_url, project_name) =
        static_channel_resolution_step(DocsChannel::Stable);

    NamedJob {
        name: "check_docs".to_owned(),
        job: docs_build_steps(
            release_job(&[]),
            resolve_step,
            &channel,
            &site_url,
            &project_name,
            false,
        ),
    }
}

pub(crate) fn deploy_docs_job(channel_input: &WorkflowInput) -> NamedJob {
    let (resolve_step, channel, site_url, project_name) = resolve_channel_step(channel_input);

    NamedJob {
        name: "deploy_docs".to_owned(),
        job: docs_build_steps(
            release_job(&[])
                .name("Build and Deploy Docs")
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                )),
            resolve_step,
            &channel,
            &site_url,
            &project_name,
            true,
        ),
    }
}

pub(crate) fn deploy_docs() -> Workflow {
    let channel = WorkflowInput::string("channel", Some("nightly".to_string()))
        .description("Docs channel to deploy: nightly, preview, or stable");

    let deploy_docs = deploy_docs_job(&channel);

    named::workflow()
        .on(Event::default()
            .push(
                Push::default()
                    .add_branch("main")
                    .add_branch("preview")
                    .add_branch("stable"),
            )
            .workflow_dispatch(
                WorkflowDispatch::default().add_input(channel.name, channel.input()),
            ))
        .add_job(deploy_docs.name, deploy_docs.job)
}
