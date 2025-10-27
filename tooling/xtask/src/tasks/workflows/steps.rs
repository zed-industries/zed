use gh_workflow::*;

pub fn checkout_repo() -> Step<Use> {
    named::uses(
        "actions",
        "checkout",
        "11bd71901bbe5b1630ceea73d27597364c9af683", // v4
    )
}

pub fn setup_pnpm() -> Step<Use> {
    named::uses(
        "pnpm",
        "action-setup",
        "fe02b34f77f8bc703788d5817da081398fad5dd2", // v4.0.0
    )
    .add_with(("version", "9"))
}

pub mod danger {
    use super::*;

    pub fn setup_node() -> Step<Use> {
        named::uses(
            "actions",
            "setup-node",
            "49933ea5288caeca8642d1e84afbd3f7d6820020", // v4
        )
        .add_with(("node-version", "20"))
        .add_with(("cache", "pnpm"))
        .add_with(("cache-dependency-path", "script/danger/pnpm-lock.yaml"))
    }

    pub fn install_deps() -> Step<Run> {
        named::run("pnpm install --dir script/danger")
    }

    pub fn run() -> Step<Run> {
        named::run("pnpm run --dir script/danger danger ci")
            // This GitHub token is not used, but the value needs to be here to prevent
            // Danger from throwing an error.
            .add_env(("GITHUB_TOKEN", "not_a_real_token"))
            // All requests are instead proxied through an instance of
            // https://github.com/maxdeviant/danger-proxy that allows Danger to securely
            // authenticate with GitHub while still being able to run on PRs from forks.
            .add_env((
                "DANGER_GITHUB_API_BASE_URL",
                "https://danger-proxy.fly.dev/github",
            ))
    }
}

pub mod nix {
    use indoc::indoc;

    use crate::tasks::workflows::vars;

    use super::*;

    // on our macs we manually install nix. for some reason the cachix action is running
    // under a non-login /bin/bash shell which doesn't source the proper script to add the
    // nix profile to PATH, so we manually add them here
    pub fn set_path() -> Step<Run> {
        named::run(indoc! {r#"
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
        named::run(&format!(
            "nix build .#{} -L --accept-flake-config",
            flake_output
        ))
    }

    pub fn limit_store() -> Step<Run> {
        named::run(indoc! {r#"
            if [ "$(du -sm /nix/store | cut -f1)" -gt 50000 ]; then
                nix-collect-garbage -d || true
            fi"#
        })
    }
}

// (janky) helpers to generate steps with a name that corresponds
// to the name of the calling function.
mod named {
    use gh_workflow::*;

    pub(super) fn uses(owner: &str, repo: &str, ref_: &str) -> Step<Use> {
        Step::new(function_name(1)).uses(owner, repo, ref_)
    }

    pub(super) fn run(script: &str) -> Step<Run> {
        Step::new(function_name(1))
            .run(script)
            .shell("bash -euxo pipefail {0}")
    }

    fn function_name(i: usize) -> String {
        let mut name = "<unknown>".to_string();
        let mut count = 0;
        backtrace::trace(|frame| {
            if count < i + 3 {
                count += 1;
                return true;
            }
            backtrace::resolve_frame(frame, |cb| {
                if let Some(s) = cb.name() {
                    name = s.to_string()
                }
            });
            false
        });
        name.split("::")
            .skip_while(|s| s != &"steps")
            .collect::<Vec<_>>()
            .join("::")
    }
}
