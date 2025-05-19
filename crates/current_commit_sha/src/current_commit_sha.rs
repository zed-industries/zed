use std::process::Command;

pub fn generate_current_commit_sha(prefix: &'static str, repo_root: &str, print_warning: bool) {
    println!("cargo:rerun-if-changed={repo_root}/.git/logs/HEAD");
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            let git_sha = String::from_utf8_lossy(&output.stdout);
            let git_sha = git_sha.trim();

            println!("cargo:rustc-env={prefix}_COMMIT_SHA={git_sha}");

            if print_warning {
                if let Ok(build_profile) = std::env::var("PROFILE") {
                    if build_profile == "release" {
                        // This is currently the best way to make `cargo build ...`'s build script
                        // to print something to stdout without extra verbosity.
                        println!(
                            "cargo:warning=Info: using '{git_sha}' hash for {prefix}_COMMIT_SHA env var"
                        );
                    }
                }
            }
        }
    }
}

pub fn current_commit_sha(prefix: &'static str) -> Option<String> {
    std::env::var(format!("{prefix}_COMMIT_SHA")).ok()
}
