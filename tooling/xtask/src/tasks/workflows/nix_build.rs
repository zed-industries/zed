use crate::tasks::workflows::{
    runners::{Arch, Platform},
    steps::{CommonJobConditions, NamedJob},
};

use super::{runners, steps, steps::named, vars};
use gh_workflow::*;
use indoc::indoc;

pub(crate) fn build_nix(
    platform: Platform,
    arch: Arch,
    flake_output: &str,
    cachix_filter: Option<&str>,
    deps: &[&NamedJob],
) -> NamedJob {
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

    pub fn cachix_action(cachix_filter: Option<&str>) -> Step<Use> {
        let mut step = named::uses(
            "cachix",
            "cachix-action",
            "0fc020193b5a1fa3ac4575aa3a7d3aa6a35435ad", // v16
        )
        .add_with(("name", "zed"))
        .add_with(("authToken", vars::CACHIX_AUTH_TOKEN))
        .add_with(("cachixArgs", "-v"));
        if let Some(cachix_filter) = cachix_filter {
            step = step.add_with(("pushFilter", cachix_filter));
        }
        step
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

    let runner = match platform {
        Platform::Windows => unimplemented!(),
        Platform::Linux => runners::LINUX_X86_BUNDLER,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    let mut job = Job::default()
        .timeout_minutes(60u32)
        .continue_on_error(true)
        .with_repository_owner_guard()
        .runs_on(runner)
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_env(("ZED_MINIDUMP_ENDPOINT", vars::ZED_SENTRY_MINIDUMP_ENDPOINT))
        .add_env((
            "ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON",
            vars::ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON,
        ))
        .add_env(("GIT_LFS_SKIP_SMUDGE", "1")) // breaks the livekit rust sdk examples which we don't actually depend on
        .add_step(steps::checkout_repo());

    if deps.len() > 0 {
        job = job.needs(deps.iter().map(|d| d.name.clone()).collect::<Vec<String>>());
    }

    job = if platform == Platform::Linux {
        job.add_step(install_nix())
            .add_step(cachix_action(cachix_filter))
            .add_step(build(&flake_output))
    } else {
        job.add_step(set_path())
            .add_step(cachix_action(cachix_filter))
            .add_step(build(&flake_output))
            .add_step(limit_store())
    };

    NamedJob {
        name: format!("build_nix_{platform}_{arch}"),
        job,
    }
}
