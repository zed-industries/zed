use crate::tasks::workflows::{
    nix_build::build_nix,
    run_bundling::{bundle_mac, bundle_windows},
    run_tests::run_platform_tests,
    runners::{Arch, Platform},
    steps::NamedJob,
    vars::{mac_bundle_envs, windows_bundle_envs},
};

use super::{runners, steps, steps::named, vars};
use gh_workflow::*;
use indexmap::IndexMap;

/// Generates the release_nightly.yml workflow
pub fn release_nightly() -> Workflow {
    let env: IndexMap<_, _> = [
        ("CARGO_TERM_COLOR", "always"),
        ("CARGO_INCREMENTAL", "0"),
        ("RUST_BACKTRACE", "1"),
        ("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED),
        ("ZED_MINIDUMP_ENDPOINT", vars::ZED_SENTRY_MINIDUMP_ENDPOINT),
        (
            "DIGITALOCEAN_SPACES_ACCESS_KEY",
            vars::DIGITALOCEAN_SPACES_ACCESS_KEY,
        ),
        (
            "DIGITALOCEAN_SPACES_SECRET_KEY",
            vars::DIGITALOCEAN_SPACES_SECRET_KEY,
        ),
    ]
    .into_iter()
    .map(|(key, value)| (key.into(), value.into()))
    .collect();

    let style = check_style();
    let tests = run_platform_tests(Platform::Mac);
    let windows_tests = run_platform_tests(Platform::Windows);
    let bundle_mac_x86 = bundle_mac_nightly(Arch::X86_64, &[&style, &tests]);
    let bundle_mac_arm = bundle_mac_nightly(Arch::ARM64, &[&style, &tests]);
    let linux_x86 = bundle_linux_nightly(Arch::X86_64, &[&style, &tests]);
    let linux_arm = bundle_linux_nightly(Arch::ARM64, &[&style, &tests]);
    let windows_x86 = bundle_windows_nightly(Arch::X86_64, &[&style, &windows_tests]);
    let windows_arm = bundle_windows_nightly(Arch::ARM64, &[&style, &windows_tests]);

    let nix_linux_x86 = build_nix(
        Platform::Linux,
        Arch::X86_64,
        "default",
        None,
        &[&style, &tests],
    );
    let nix_mac_arm = build_nix(
        Platform::Mac,
        Arch::ARM64,
        "default",
        None,
        &[&style, &tests],
    );
    let update_nightly_tag = update_nightly_tag_job(&[
        &bundle_mac_x86,
        &bundle_mac_arm,
        &linux_x86,
        &linux_arm,
        &windows_x86,
        &windows_arm,
    ]);

    named::workflow()
        .on(Event::default()
            // Fire every day at 7:00am UTC (Roughly before EU workday and after US workday)
            .schedule([Schedule::new("0 7 * * *")])
            .push(Push::default().add_tag("nightly")))
        .envs(env)
        .add_job(style.name, style.job)
        .add_job(tests.name, tests.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(bundle_mac_x86.name, bundle_mac_x86.job)
        .add_job(bundle_mac_arm.name, bundle_mac_arm.job)
        .add_job(linux_x86.name, linux_x86.job)
        .add_job(linux_arm.name, linux_arm.job)
        .add_job(windows_x86.name, windows_x86.job)
        .add_job(windows_arm.name, windows_arm.job)
        .add_job(nix_linux_x86.name, nix_linux_x86.job)
        .add_job(nix_mac_arm.name, nix_mac_arm.job)
        .add_job(update_nightly_tag.name, update_nightly_tag.job)
}

fn check_style() -> NamedJob {
    let job = release_job(&[])
        .runs_on(runners::MAC_DEFAULT)
        .add_step(
            steps::checkout_repo()
                .add_with(("clean", false))
                .add_with(("fetch-depth", 0)),
        )
        .add_step(steps::cargo_fmt())
        .add_step(steps::script("./script/clippy"));

    named::job(job)
}

fn release_job(deps: &[&NamedJob]) -> Job {
    let job = Job::default()
        .cond(Expression::new(
            "github.repository_owner == 'zed-industries'",
        ))
        .timeout_minutes(60u32);
    if deps.len() > 0 {
        job.needs(deps.iter().map(|j| j.name.clone()).collect::<Vec<_>>())
    } else {
        job
    }
}

fn bundle_mac_nightly(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    let platform = Platform::Mac;
    NamedJob {
        name: format!("bundle_mac_nightly_{arch}"),
        job: release_job(deps)
            .runs_on(runners::MAC_DEFAULT)
            .envs(mac_bundle_envs())
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_node())
            .add_step(steps::setup_sentry())
            .add_step(steps::clear_target_dir_if_large(platform))
            .add_step(set_release_channel_to_nightly(platform))
            .add_step(bundle_mac(arch))
            .add_step(upload_zed_nightly(platform, arch)),
    }
}

fn bundle_linux_nightly(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    let platform = Platform::Linux;
    let mut job = steps::release_job(deps)
        .runs_on(arch.linux_bundler())
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_sentry())
        .add_step(steps::script("./script/linux"));

    // todo(ci) can we do this on arm too?
    if arch == Arch::X86_64 {
        job = job.add_step(steps::script("./script/install-mold"));
    }
    job = job
        .add_step(steps::clear_target_dir_if_large(platform))
        .add_step(set_release_channel_to_nightly(platform))
        .add_step(steps::script("./script/bundle-linux"))
        .add_step(upload_zed_nightly(platform, arch));
    NamedJob {
        name: format!("bundle_linux_nightly_{arch}"),
        job,
    }
}

fn bundle_windows_nightly(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    let platform = Platform::Windows;
    NamedJob {
        name: format!("bundle_windows_nightly_{arch}"),
        job: steps::release_job(deps)
            .runs_on(runners::WINDOWS_DEFAULT)
            .envs(windows_bundle_envs())
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_sentry())
            .add_step(set_release_channel_to_nightly(platform))
            .add_step(bundle_windows(arch))
            .add_step(upload_zed_nightly(platform, arch)),
    }
}

fn update_nightly_tag_job(deps: &[&NamedJob]) -> NamedJob {
    fn update_nightly_tag() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [ "$(git rev-parse nightly)" = "$(git rev-parse HEAD)" ]; then
              echo "Nightly tag already points to current commit. Skipping tagging."
              exit 0
            fi
            git config user.name github-actions
            git config user.email github-actions@github.com
            git tag -f nightly
            git push origin nightly --force
        "#})
    }

    fn create_sentry_release() -> Step<Use> {
        named::uses(
            "getsentry",
            "action-release",
            "526942b68292201ac6bbb99b9a0747d4abee354c", // v3
        )
        .add_env(("SENTRY_ORG", "zed-dev"))
        .add_env(("SENTRY_PROJECT", "zed"))
        .add_env(("SENTRY_AUTH_TOKEN", vars::SENTRY_AUTH_TOKEN))
        .add_with(("environment", "production"))
    }

    NamedJob {
        name: "update_nightly_tag".to_owned(),
        job: steps::release_job(deps)
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo().add_with(("fetch-depth", 0)))
            .add_step(update_nightly_tag())
            .add_step(create_sentry_release()),
    }
}

fn set_release_channel_to_nightly(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Linux | Platform::Mac => named::bash(indoc::indoc! {r#"
            set -eu
            version=$(git rev-parse --short HEAD)
            echo "Publishing version: ${version} on release channel nightly"
            echo "nightly" > crates/zed/RELEASE_CHANNEL
        "#}),
        Platform::Windows => named::pwsh(indoc::indoc! {r#"
            $ErrorActionPreference = "Stop"
            $version = git rev-parse --short HEAD
            Write-Host "Publishing version: $version on release channel nightly"
            "nightly" | Set-Content -Path "crates/zed/RELEASE_CHANNEL"
        "#})
        .working_directory("${{ env.ZED_WORKSPACE }}"),
    }
}

fn upload_zed_nightly(platform: Platform, arch: Arch) -> Step<Run> {
    match platform {
        Platform::Linux => named::bash(&format!("script/upload-nightly linux-targz {arch}")),
        Platform::Mac => named::bash(&format!("script/upload-nightly macos {arch}")),
        Platform::Windows => {
            let cmd = match arch {
                Arch::X86_64 => "script/upload-nightly.ps1 -Architecture x86_64",
                Arch::ARM64 => "script/upload-nightly.ps1 -Architecture aarch64",
            };
            named::pwsh(cmd).working_directory("${{ env.ZED_WORKSPACE }}")
        }
    }
}
