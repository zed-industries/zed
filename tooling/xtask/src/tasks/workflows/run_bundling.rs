use crate::tasks::workflows::steps::named;

use super::{runners, steps, vars};
use gh_workflow::*;

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
        .add_job("bundle_mac", bundle_mac())
        .add_job("bundle_linux_x86_64", bundle_linux(runners::Arch::X86_64))
        .add_job("bundle_linux_arm64", bundle_linux(runners::Arch::AARCH64))
        .add_job(
            "bundle_windows_x86_64",
            bundle_windows_job(runners::Arch::X86_64),
        )
        .add_job(
            "bundle_windows_arm64",
            bundle_windows_job(runners::Arch::AARCH64),
        )
}

fn bundle_job() -> Job {
    Job::default()
        .cond(Expression::new(
                "(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') ||
                 (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))",
            ))
        .timeout_minutes(60u32)
}

fn bundle_mac() -> Job {
    bundle_job()
        .runs_on(runners::MAC_DEFAULT)
        .add_env(("MACOS_CERTIFICATE", vars::MACOS_CERTIFICATE))
        .add_env((
            "MACOS_CERTIFICATE_PASSWORD",
            vars::MACOS_CERTIFICATE_PASSWORD,
        ))
        .add_env(("APPLE_NOTARIZATION_KEY", vars::APPLE_NOTARIZATION_KEY))
        .add_env(("APPLE_NOTARIZATION_KEY_ID", vars::APPLE_NOTARIZATION_KEY_ID))
        .add_env((
            "APPLE_NOTARIZATION_ISSUER_ID",
            vars::APPLE_NOTARIZATION_ISSUER_ID,
        ))
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_node())
        .add_step(steps::setup_sentry())
        .add_step(steps::clear_target_dir_if_large())
        .add_step(steps::script("./script/bundle-mac"))
        .add_step(steps::upload_artifact(
            "Zed_${{ github.event.pull_request.head.sha || github.sha }}-aarch64.dmg",
            "target/aarch64-apple-darwin/release/Zed.dmg",
        ))
        .add_step(steps::upload_artifact(
            "Zed_${{ github.event.pull_request.head.sha || github.sha }}-x86_64.dmg",
            "target/x86_64-apple-darwin/release/Zed.dmg",
        ))
}

fn bundle_linux(arch: runners::Arch) -> Job {
    let artifact_name = format!("zed-{}-{}.tar.gz", vars::GITHUB_SHA, arch.triple());
    let remote_server_artifact_name = format!(
        "zed-remote-server-{}-{}.tar.gz",
        vars::GITHUB_SHA,
        arch.triple()
    );
    let mut job = bundle_job()
        .runs_on(arch.linux_bundler())
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_sentry())
        .add_step(steps::script("./script/linux"));
    // todo(ci) can we do this on arm too?
    if arch == runners::Arch::X86_64 {
        job = job.add_step(steps::script("./script/install-mold"));
    }
    job.add_step(steps::script("./script/bundle-linux"))
        .add_step(steps::upload_artifact(
            &artifact_name,
            "target/release/zed-*.tar.gz",
        ))
        .add_step(steps::upload_artifact(
            &remote_server_artifact_name,
            "target/release/zed-remote-server-*.tar.gz",
        ))
}

fn bundle_windows_job(arch: runners::Arch) -> Job {
    use vars::GITHUB_SHA;
    bundle_job()
        .runs_on(runners::WINDOWS_DEFAULT)
        .add_env(("AZURE_TENANT_ID", vars::AZURE_SIGNING_TENANT_ID))
        .add_env(("AZURE_CLIENT_ID", vars::AZURE_SIGNING_CLIENT_ID))
        .add_env(("AZURE_CLIENT_SECRET", vars::AZURE_SIGNING_CLIENT_SECRET))
        .add_env(("ACCOUNT_NAME", vars::AZURE_SIGNING_ACCOUNT_NAME))
        .add_env(("CERT_PROFILE_NAME", vars::AZURE_SIGNING_CERT_PROFILE_NAME))
        .add_env(("ENDPOINT", vars::AZURE_SIGNING_ENDPOINT))
        .add_env(("FILE_DIGEST", "SHA256"))
        .add_env(("TIMESTAMP_DIGEST", "SHA256"))
        .add_env(("TIMESTAMP_SERVER", "http://timestamp.acs.microsoft.com"))
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_sentry())
        .add_step(bundle_windows(arch))
        .add_step(steps::upload_artifact(
            &format!("Zed_{GITHUB_SHA}-{arch}.exe"),
            "${{ env.SETUP_PATH }}",
        ))
}

fn bundle_windows(arch: runners::Arch) -> Step<Run> {
    let step = match arch {
        runners::Arch::X86_64 => named::pwsh("script/bundle-windows.ps1 -Architecture x86_64"),
        runners::Arch::AARCH64 => named::pwsh("script/bundle-windows.ps1 -Architecture aarch64"),
    };
    step.working_directory("${{ env.ZED_WORKSPACE }}")
}
