use gh_workflow::*;

use crate::tasks::workflows::{
    release::{self, notify_on_failure},
    runners,
    steps::{CommonJobConditions, NamedJob, checkout_repo, dependant_job, named},
    vars::{self, StepOutput, WorkflowInput},
};

const TAG_NAME: &str = "${{ github.event.release.tag_name || inputs.tag_name }}";
const IS_PRERELEASE: &str = "${{ github.event.release.prerelease || inputs.prerelease }}";
const RELEASE_BODY: &str = "${{ github.event.release.body || inputs.body }}";

pub fn after_release() -> Workflow {
    let tag_name = WorkflowInput::string("tag_name", None);
    let prerelease = WorkflowInput::bool("prerelease", None);
    let body = WorkflowInput::string("body", Some(String::new()));

    let refresh_zed_dev = rebuild_releases_page();
    let post_to_discord = post_to_discord(&[&refresh_zed_dev]);
    let publish_winget = publish_winget();
    let create_sentry_release = create_sentry_release();
    let notify_on_failure = notify_on_failure(&[
        &refresh_zed_dev,
        &post_to_discord,
        &publish_winget,
        &create_sentry_release,
    ]);

    named::workflow()
        .on(Event::default()
            .release(Release::default().types(vec![ReleaseType::Published]))
            .workflow_dispatch(
                WorkflowDispatch::default()
                    .add_input(tag_name.name, tag_name.input())
                    .add_input(prerelease.name, prerelease.input())
                    .add_input(body.name, body.input()),
            ))
        .add_job(refresh_zed_dev.name, refresh_zed_dev.job)
        .add_job(post_to_discord.name, post_to_discord.job)
        .add_job(publish_winget.name, publish_winget.job)
        .add_job(create_sentry_release.name, create_sentry_release.job)
        .add_job(notify_on_failure.name, notify_on_failure.job)
}

fn rebuild_releases_page() -> NamedJob {
    fn refresh_cloud_releases() -> Step<Run> {
        named::bash(format!(
            "curl -fX POST https://cloud.zed.dev/releases/refresh?expect_tag={TAG_NAME}"
        ))
    }

    fn redeploy_zed_dev() -> Step<Run> {
        named::bash("./script/redeploy-vercel").add_env(("VERCEL_TOKEN", vars::VERCEL_TOKEN))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .with_repository_owner_guard()
            .add_step(refresh_cloud_releases())
            .add_step(checkout_repo())
            .add_step(redeploy_zed_dev()),
    )
}

fn post_to_discord(deps: &[&NamedJob]) -> NamedJob {
    fn get_release_url() -> Step<Run> {
        named::bash(format!(
            r#"if [ "{IS_PRERELEASE}" == "true" ]; then
    URL="https://zed.dev/releases/preview"
else
    URL="https://zed.dev/releases/stable"
fi

echo "URL=$URL" >> "$GITHUB_OUTPUT"
"#
        ))
        .id("get-release-url")
    }

    fn get_content() -> Step<Use> {
        named::uses(
            "2428392",
            "gh-truncate-string-action",
            "b3ff790d21cf42af3ca7579146eedb93c8fb0757", // v1.4.1
        )
        .id("get-content")
        .add_with((
            "stringToTruncate",
            format!(
                "📣 Zed [{TAG_NAME}](<${{{{ steps.get-release-url.outputs.URL }}}}>)  was just released!\n\n{RELEASE_BODY}\n"
            ),
        ))
        .add_with(("maxLength", 2000))
        .add_with(("truncationSymbol", "..."))
    }

    fn discord_webhook_action() -> Step<Use> {
        named::uses(
            "tsickert",
            "discord-webhook",
            "c840d45a03a323fbc3f7507ac7769dbd91bfb164", // v5.3.0
        )
        .add_with(("webhook-url", vars::DISCORD_WEBHOOK_RELEASE_NOTES))
        .add_with(("content", "${{ steps.get-content.outputs.string }}"))
    }
    let job = dependant_job(deps)
        .runs_on(runners::LINUX_SMALL)
        .with_repository_owner_guard()
        .add_step(get_release_url())
        .add_step(get_content())
        .add_step(discord_webhook_action());
    named::job(job)
}

fn publish_winget() -> NamedJob {
    fn sync_winget_pkgs_fork() -> Step<Run> {
        named::pwsh(indoc::indoc! {r#"
            $headers = @{
                "Authorization" = "Bearer $env:WINGET_TOKEN"
                "Accept" = "application/vnd.github+json"
                "X-GitHub-Api-Version" = "2022-11-28"
            }
            $body = @{ branch = "master" } | ConvertTo-Json
            $uri = "https://api.github.com/repos/${{ github.repository_owner }}/winget-pkgs/merge-upstream"
            try {
                Invoke-RestMethod -Uri $uri -Method Post -Headers $headers -Body $body -ContentType "application/json"
                Write-Host "Successfully synced winget-pkgs fork"
            } catch {
                Write-Host "Fork sync response: $_"
                Write-Host "Continuing anyway - fork may already be up to date"
            }
        "#})
        .add_env(("WINGET_TOKEN", vars::WINGET_TOKEN))
    }

    fn set_package_name() -> (Step<Run>, StepOutput) {
        let script = format!(
            r#"if ("{IS_PRERELEASE}" -eq "true") {{
    $PACKAGE_NAME = "ZedIndustries.Zed.Preview"
}} else {{
    $PACKAGE_NAME = "ZedIndustries.Zed"
}}

echo "PACKAGE_NAME=$PACKAGE_NAME" >> $env:GITHUB_OUTPUT
"#
        );
        let step = named::pwsh(&script).id("set-package-name");

        let output = StepOutput::new(&step, "PACKAGE_NAME");
        (step, output)
    }

    fn winget_releaser(package_name: &StepOutput) -> Step<Use> {
        named::uses(
            "vedantmgoyal9",
            "winget-releaser",
            "19e706d4c9121098010096f9c495a70a7518b30f", // v2
        )
        .add_with(("identifier", package_name.to_string()))
        .add_with(("release-tag", TAG_NAME))
        .add_with(("max-versions-to-keep", 5))
        .add_with(("token", vars::WINGET_TOKEN))
    }

    let (set_package_name, package_name) = set_package_name();

    named::job(
        Job::default()
            .runs_on(runners::WINDOWS_DEFAULT)
            .add_step(sync_winget_pkgs_fork())
            .add_step(set_package_name)
            .add_step(winget_releaser(&package_name)),
    )
}

fn create_sentry_release() -> NamedJob {
    let job = Job::default()
        .runs_on(runners::LINUX_SMALL)
        .with_repository_owner_guard()
        .add_step(checkout_repo())
        .add_step(release::create_sentry_release());
    named::job(job)
}
