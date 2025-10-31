use crate::tasks::workflows::{
    steps::{FluentBuilder, NamedJob, dependant_job, named},
    vars::{mac_bundle_envs, windows_bundle_envs},
};

use super::{runners, steps, vars};
use gh_workflow::*;
use indexmap::IndexMap;

pub fn run_bundling() -> Workflow {
    named::workflow()
        .on(Event::default().pull_request(
            PullRequest::default().types([PullRequestType::Labeled, PullRequestType::Synchronize]),
        ))
        .concurrency(
            Concurrency::new(Expression::new(
                "${{ github.workflow }}-${{ github.head_ref || github.ref }}",
            ))
            .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", "0"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_env(("ZED_MINIDUMP_ENDPOINT", vars::ZED_SENTRY_MINIDUMP_ENDPOINT))
        .add_job(
            "bundle_mac_x86_64",
            bundle_mac_job(runners::Arch::X86_64, &[]),
        )
        .add_job(
            "bundle_mac_arm64",
            bundle_mac_job(runners::Arch::ARM64, &[]),
        )
        .add_job(
            "bundle_linux_x86_64",
            bundle_linux_job(runners::Arch::X86_64, &[]),
        )
        .add_job(
            "bundle_linux_arm64",
            bundle_linux_job(runners::Arch::ARM64, &[]),
        )
        .add_job(
            "bundle_windows_x86_64",
            bundle_windows_job(runners::Arch::X86_64, &[]),
        )
        .add_job(
            "bundle_windows_arm64",
            bundle_windows_job(runners::Arch::ARM64, &[]),
        )
}

fn bundle_job(deps: &[&NamedJob]) -> Job {
    dependant_job(deps)
        .when(deps.len() == 0, |job|
                job.cond(Expression::new(
                "(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') ||
                 (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))",
            )))
        .timeout_minutes(60u32)
}

pub(crate) fn bundle_mac_job(arch: runners::Arch, deps: &[&NamedJob]) -> Job {
    use vars::GITHUB_SHA;
    let artifact_name = format!("Zed_{GITHUB_SHA}-{arch}.dmg");
    let remote_server_artifact_name = format!("zed-remote-server-{GITHUB_SHA}-macos-{arch}.gz");
    bundle_job(deps)
        .runs_on(runners::MAC_DEFAULT)
        .envs(mac_bundle_envs())
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_node())
        .add_step(steps::setup_sentry())
        .add_step(steps::clear_target_dir_if_large(runners::Platform::Mac))
        .add_step(bundle_mac(arch))
        .add_step(steps::upload_artifact(
            &artifact_name,
            &format!("target/{arch}-apple-darwin/release/Zed.dmg"),
        ))
        .add_step(steps::upload_artifact(
            &remote_server_artifact_name,
            &format!("target/zed-remote-server-macos-{arch}.gz"),
        ))
        .outputs(
            [
                ("zed".to_string(), artifact_name),
                ("remote-server".to_string(), remote_server_artifact_name),
            ]
            .into_iter()
            .collect::<IndexMap<_, _>>(),
        )
}

pub fn bundle_mac(arch: runners::Arch) -> Step<Run> {
    named::bash(&format!("./script/bundle-mac {arch}-apple-darwin"))
}

pub(crate) fn bundle_linux_job(arch: runners::Arch, deps: &[&NamedJob]) -> Job {
    let artifact_name = format!("zed-{}-{}.tar.gz", vars::GITHUB_SHA, arch.triple());
    let remote_server_artifact_name = format!(
        "zed-remote-server-{}-{}.tar.gz",
        vars::GITHUB_SHA,
        arch.triple()
    );
    bundle_job(deps)
        .runs_on(arch.linux_bundler())
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_sentry())
        .map(steps::install_linux_dependencies)
        .add_step(steps::script("./script/bundle-linux"))
        .add_step(steps::upload_artifact(
            &artifact_name,
            "target/release/zed-*.tar.gz",
        ))
        .add_step(steps::upload_artifact(
            &remote_server_artifact_name,
            "target/zed-remote-server-*.gz",
        ))
        .outputs(
            [
                ("zed".to_string(), artifact_name),
                ("remote-server".to_string(), remote_server_artifact_name),
            ]
            .into_iter()
            .collect::<IndexMap<_, _>>(),
        )
}

pub(crate) fn bundle_windows_job(arch: runners::Arch, deps: &[&NamedJob]) -> Job {
    use vars::GITHUB_SHA;
    let artifact_name = format!("Zed_{GITHUB_SHA}-{arch}.exe");
    bundle_job(deps)
        .runs_on(runners::WINDOWS_DEFAULT)
        .envs(windows_bundle_envs())
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_sentry())
        .add_step(bundle_windows(arch))
        .add_step(steps::upload_artifact(
            &artifact_name,
            "${{ env.SETUP_PATH }}",
        ))
        .outputs(
            [("zed".to_string(), artifact_name)]
                .into_iter()
                .collect::<IndexMap<_, _>>(),
        )
}

pub fn bundle_windows(arch: runners::Arch) -> Step<Run> {
    let step = match arch {
        runners::Arch::X86_64 => named::pwsh("script/bundle-windows.ps1 -Architecture x86_64"),
        runners::Arch::ARM64 => named::pwsh("script/bundle-windows.ps1 -Architecture aarch64"),
    };
    step.working_directory("${{ env.ZED_WORKSPACE }}")
}
