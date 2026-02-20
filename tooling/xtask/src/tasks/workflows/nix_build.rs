use crate::tasks::workflows::{
    runners::{Arch, Platform},
    steps::{CommonJobConditions, NamedJob},
};

use super::{runners, steps, steps::named, vars};
use gh_workflow::*;

pub(crate) fn build_nix(
    platform: Platform,
    arch: Arch,
    flake_output: &str,
    cachix_filter: Option<&str>,
    deps: &[&NamedJob],
) -> NamedJob {
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

    // After install-nix, register ~/nix-cache as a local binary cache
    // substituter so nix pulls from it on demand during builds (no bulk
    // import). Also restart the daemon so it picks up the new config.
    pub fn configure_local_nix_cache() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            mkdir -p ~/nix-cache
            echo "extra-substituters = file://$HOME/nix-cache?priority=10" | sudo tee -a /etc/nix/nix.conf
            echo "require-sigs = false" | sudo tee -a /etc/nix/nix.conf
            sudo launchctl kickstart -k system/org.nixos.nix-daemon
        "#})
    }

    // Incrementally copy only new store paths from the build result's
    // closure into the local binary cache for the next run.
    pub fn export_to_local_nix_cache() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [ -L result ]; then
              echo "Copying build closure to local binary cache..."
              nix copy --to "file://$HOME/nix-cache" ./result || echo "Warning: nix copy to local cache failed"
            else
              echo "No build result found, skipping cache export."
            fi
        "#})
        .if_condition(Expression::new("always()"))
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

    // On Linux, `cache: nix` uses bind-mounts so the /nix store is available
    // before install-nix-action runs — no extra steps needed.
    //
    // On macOS, `/nix` lives on a read-only root filesystem and the nscloud
    // cache action cannot mount or symlink there. Instead we cache a
    // user-writable directory (~/nix-cache) as a local binary cache and
    // register it as a nix substituter. Nix then pulls paths from it on
    // demand during builds (zero-copy at startup), and after building we
    // incrementally copy new paths into the cache for the next run.
    job = match platform {
        Platform::Linux => job
            .add_step(steps::cache_nix_dependencies_namespace())
            .add_step(install_nix())
            .add_step(cachix_action(cachix_filter))
            .add_step(build(&flake_output)),
        Platform::Mac => job
            .add_step(steps::cache_nix_store_macos())
            .add_step(install_nix())
            .add_step(configure_local_nix_cache())
            .add_step(cachix_action(cachix_filter))
            .add_step(build(&flake_output))
            .add_step(export_to_local_nix_cache()),
        Platform::Windows => unimplemented!(),
    };

    NamedJob {
        name: format!("build_nix_{platform}_{arch}"),
        job,
    }
}
