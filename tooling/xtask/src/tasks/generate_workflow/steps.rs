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
            .env(("GITHUB_TOKEN", "not_a_real_token"))
            // All requests are instead proxied through an instance of
            // https://github.com/maxdeviant/danger-proxy that allows Danger to securely
            // authenticate with GitHub while still being able to run on PRs from forks.
            .env((
                "DANGER_GITHUB_API_BASE_URL",
                "https://danger-proxy.fly.dev/github",
            ))
    }
}

// (janky) helpers to generate steps with a name that coresponds
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
