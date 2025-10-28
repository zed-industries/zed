use super::{runners, steps, steps::named, vars};
use gh_workflow::*;
use indexmap::IndexMap;
use indoc::indoc;

/// Generates the nix.yml workflow
pub fn nix_build() -> Workflow {
    let env: IndexMap<_, _> = [
        ("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED),
        ("ZED_MINIDUMP_ENDPOINT", vars::ZED_SENTRY_MINIDUMP_ENDPOINT),
        (
            "ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON",
            vars::ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON,
        ),
        ("GIT_LFS_SKIP_SMUDGE", "1"), // breaks the livekit rust sdk examples which we don't actually depend on
    ]
    .into_iter()
    .map(|(key, value)| (key.into(), value.into()))
    .collect();

    // todo(ci) instead of having these as optional YAML inputs,
    // should we just generate two copies of the job (one for release-nightly
    // and one for CI?)
    let (input_flake_output, flake_output) = vars::input(
        "flake-output",
        WorkflowCallInput {
            input_type: "string".into(),
            default: Some("default".into()),
            ..Default::default()
        },
    );
    let (input_cachix_filter, cachix_filter) = vars::input(
        "cachix-filter",
        WorkflowCallInput {
            input_type: "string".into(),
            ..Default::default()
        },
    );

    named::workflow()
        .on(Event::default().workflow_call(
            WorkflowCall::default()
                .add_input(flake_output.0, flake_output.1)
                .add_input(cachix_filter.0, cachix_filter.1),
        ))
        .add_job(
            "nix-build-linux-x86",
            Job::default()
                .timeout_minutes(60u32)
                .continue_on_error(true)
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                ))
                .runs_on(runners::LINUX_DEFAULT)
                .envs(env.clone())
                .add_step(steps::checkout_repo().add_with(("clean", "false")))
                .add_step(install_nix())
                .add_step(cachix_action(&input_cachix_filter))
                .add_step(build(&input_flake_output)),
        )
        .add_job(
            "nix-build-mac-arm",
            Job::default()
                .timeout_minutes(60u32)
                .continue_on_error(true)
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                ))
                .runs_on(runners::MAC_DEFAULT)
                .envs(env)
                .add_step(steps::checkout_repo().add_with(("clean", "false")))
                .add_step(set_path())
                .add_step(cachix_action(&input_cachix_filter))
                .add_step(build(&input_flake_output))
                .add_step(limit_store()),
        )
}
// on our macs we manually install nix. for some reason the cachix action is running
// under a non-login /bin/bash shell which doesn't source the proper script to add the
// nix profile to PATH, so we manually add them here
pub fn set_path() -> Step<Run> {
    named::bash(indoc! {r#"
            echo "/nix/var/nix/profiles/default/bin" >> "$GITHUB_PATH"
            echo "/Users/administrator/.nix-profile/bin" >> "$GITHUB_PATH"
        "#})
}

pub fn install_nix() -> Step<Use> {
    named::uses(
        "cachix",
        "install-nix-action",
        "02a151ada4993995686f9ed4f1be7cfbb229e56f", // v31
    )
    .add_with(("github_access_token", vars::GITHUB_TOKEN))
}

pub fn cachix_action(cachix_filter: &str) -> Step<Use> {
    named::uses(
        "cachix",
        "cachix-action",
        "0fc020193b5a1fa3ac4575aa3a7d3aa6a35435ad", // v16
    )
    .add_with(("name", "zed"))
    .add_with(("authToken", vars::CACHIX_AUTH_TOKEN))
    .add_with(("pushFilter", cachix_filter))
    .add_with(("cachixArgs", "-v"))
}

pub fn build(flake_output: &str) -> Step<Run> {
    named::bash(&format!(
        "nix build .#{} -L --accept-flake-config",
        flake_output
    ))
}

pub fn limit_store() -> Step<Run> {
    named::bash(indoc! {r#"
            if [ "$(du -sm /nix/store | cut -f1)" -gt 50000 ]; then
                nix-collect-garbage -d || true
            fi"#
    })
}
