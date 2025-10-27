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

pub fn bundle_mac() -> Workflow {
    Workflow::default()
        .name("Bundle macOS")
        .on(Event::default().pull_request(
            PullRequest::default().types([PullRequestType::Labeled, PullRequestType::Synchronize]),
        ))
        .concurrency(
            Concurrency::new(Expression::new(
                "${{ github.workflow }}-${{ github.head_ref || github.ref }}",
            ))
            .cancel_in_progress(true),
        )
        .add_job(
            "bundle-mac",
            Job::default()
                .cond(Expression::new(
                    "(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') || (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))",
                ))
                .runs_on(runners::MAC_DEFAULT)
                .timeout_minutes(120u32)
                .add_env(("MACOS_CERTIFICATE", vars::MACOS_CERTIFICATE))
                .add_env(("MACOS_CERTIFICATE_PASSWORD", vars::MACOS_CERTIFICATE_PASSWORD))
                .add_env(("APPLE_NOTARIZATION_KEY", vars::APPLE_NOTARIZATION_KEY))
                .add_env(("APPLE_NOTARIZATION_KEY_ID", vars::APPLE_NOTARIZATION_KEY_ID))
                .add_env(("APPLE_NOTARIZATION_ISSUER_ID", vars::APPLE_NOTARIZATION_ISSUER_ID))
                .add_step(
                    Step::new("Install Node")
                        .uses("actions", "setup-node", "49933ea5288caeca8642d1e84afbd3f7d6820020")
                        .add_with(("node-version", "18"))
                )
                .add_step(
                    Step::new("Setup Sentry CLI")
                        .uses("matbour", "setup-sentry-cli", "3e938c54b3018bdd019973689ef984e033b0454b")
                        .add_with(("token", vars::SENTRY_AUTH_TOKEN))
                )
                .add_step(
                    steps::checkout_repo()
                        .add_with(("fetch-depth", "25"))
                        .add_with(("clean", "false"))
                )
                .add_step(Step::new("Limit target directory size").run("script/clear-target-dir-if-larger-than 100"))
                .add_step(Step::new("Create macOS app bundle").run("script/bundle-mac"))
                .add_step(
                    Step::new("Rename binaries")
                        .run("mv target/aarch64-apple-darwin/release/Zed.dmg target/aarch64-apple-darwin/release/Zed-aarch64.dmg\nmv target/x86_64-apple-darwin/release/Zed.dmg target/x86_64-apple-darwin/release/Zed-x86_64.dmg")
                )
                .add_step(
                    Step::new("Upload app bundle (aarch64)")
                        .uses("actions", "upload-artifact", "ea165f8d65b6e75b540449e92b4886f43607fa02")
                        .add_with(("name", "Zed_${{ github.event.pull_request.head.sha || github.sha }}-aarch64.dmg"))
                        .add_with(("path", "target/aarch64-apple-darwin/release/Zed-aarch64.dmg"))
                )
                .add_step(
                    Step::new("Upload app bundle (x86_64)")
                        .uses("actions", "upload-artifact", "ea165f8d65b6e75b540449e92b4886f43607fa02")
                        .add_with(("name", "Zed_${{ github.event.pull_request.head.sha || github.sha }}-x86_64.dmg"))
                        .add_with(("path", "target/x86_64-apple-darwin/release/Zed-x86_64.dmg"))
                ),
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

    // todo!() instead of having these as optional YAML inputs,
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
