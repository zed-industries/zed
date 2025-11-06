use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{NamedJob, dependant_job, named},
    vars::{self, StepOutput},
};

pub fn after_release() -> Workflow {
    let refresh_zed_dev = rebuild_releases_page();
    let post_to_discord = post_to_discord(&[&refresh_zed_dev]);
    let publish_winget = publish_winget();

    named::workflow()
        .on(Event::default().release(Release::default().types(vec![ReleaseType::Published])))
        .add_job(refresh_zed_dev.name, refresh_zed_dev.job)
        .add_job(post_to_discord.name, post_to_discord.job)
        .add_job(publish_winget.name, publish_winget.job)
}

fn rebuild_releases_page() -> NamedJob {
    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .cond(Expression::new(
                "github.repository_owner == 'zed-industries'",
            ))
            .add_step(named::bash(
                "curl https://zed.dev/api/revalidate-releases -H \"Authorization: Bearer ${RELEASE_NOTES_API_TOKEN}\"",
            ).add_env(("RELEASE_NOTES_API_TOKEN", vars::RELEASE_NOTES_API_TOKEN))),
    )
}

fn post_to_discord(deps: &[&NamedJob]) -> NamedJob {
    fn get_release_url() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [ "${{ github.event.release.prerelease }}" == "true" ]; then
                URL="https://zed.dev/releases/preview"
            else
                URL="https://zed.dev/releases/stable"
            fi

            echo "URL=$URL" >> "$GITHUB_OUTPUT"
        "#})
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
            indoc::indoc! {r#"
                📣 Zed [${{ github.event.release.tag_name }}](<${{ steps.get-release-url.outputs.URL }}>) was just released!

                ${{ github.event.release.body }}
            "#},
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
        .cond(Expression::new(
            "github.repository_owner == 'zed-industries'",
        ))
        .add_step(get_release_url())
        .add_step(get_content())
        .add_step(discord_webhook_action());
    named::job(job)
}

fn publish_winget() -> NamedJob {
    fn set_package_name() -> (Step<Run>, StepOutput) {
        let step = named::bash(indoc::indoc! {r#"
            if [ "${{ github.event.release.prerelease }}" == "true" ]; then
                PACKAGE_NAME=ZedIndustries.Zed.Preview
            else
                PACKAGE_NAME=ZedIndustries.Zed
            fi

            echo "PACKAGE_NAME=$PACKAGE_NAME" >> "$GITHUB_OUTPUT"
        "#})
        .id("set-package-name");

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
        .add_with(("max-versions-to-keep", 5))
        .add_with(("token", vars::WINGET_TOKEN))
    }

    let (set_package_name, package_name) = set_package_name();

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(set_package_name)
            .add_step(winget_releaser(&package_name)),
    )
}
