use gh_workflow::*;
use indexmap::IndexMap;

use super::{runners, steps, vars};

/// Generates the danger.yml workflow
pub fn danger() -> Workflow {
    Workflow::default()
        .name("Danger")
        .on(
            Event::default().pull_request(PullRequest::default().add_branch("main").types([
                PullRequestType::Opened,
                PullRequestType::Synchronize,
                PullRequestType::Reopened,
                PullRequestType::Edited,
            ])),
        )
        .add_job(
            "danger",
            Job::default()
                .cond(Expression::new(
                    "github.repository_owner == 'zed-industries'",
                ))
                .runs_on(runners::LINUX_CHEAP)
                .add_step(steps::checkout_repo())
                .add_step(steps::setup_pnpm())
                .add_step(steps::danger::setup_node())
                .add_step(steps::danger::install_deps())
                .add_step(steps::danger::run()),
        )
}

/// Generates the nix.yml workflow
pub fn nix() -> Workflow {
    let env: IndexMap<_, _> = [
        ("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED),
        ("ZED_MINIDUMP_ENDPOINT", vars::ZED_MINIDUMP_ENDPOINT),
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

    Workflow::default()
        .name("Nix build")
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
                .env(env.clone())
                .add_step(steps::checkout_repo().add_with(("clean", "false")))
                .add_step(steps::nix::install_nix())
                .add_step(steps::nix::cachix_action(&input_cachix_filter))
                .add_step(steps::nix::build(&input_flake_output)),
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
                .env(env)
                .add_step(steps::checkout_repo().add_with(("clean", "false")))
                .add_step(steps::nix::set_path())
                .add_step(steps::nix::cachix_action(&input_cachix_filter))
                .add_step(steps::nix::build(&input_flake_output))
                .add_step(steps::nix::limit_store()),
        )
}
