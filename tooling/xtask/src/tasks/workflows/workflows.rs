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

pub fn run_bundling() -> Workflow {
    let condition = Expression::new(
        "(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') || (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))",
    );

    Workflow::default()
        .name("Run Bundling")
        .on(Event::default().pull_request(
            PullRequest::default().types([PullRequestType::Labeled, PullRequestType::Synchronize]),
        ))
        .concurrency(
            Concurrency::new(Expression::new(
                "${{ github.workflow }}-${{ github.head_ref || github.ref }}",
            ))
            .cancel_in_progress(true),
        )
        .add_job("bundle-mac", bundle_mac(condition.clone()))
        .add_job(
            "bundle-linux-x86_64",
            bundle_linux(condition.clone(), runners::Arch::X86_64),
        )
        .add_job(
            "bundle-linux-aarch64",
            bundle_linux(condition.clone(), runners::Arch::AARCH64),
        )
        .add_job("bundle-windows", bundle_windows(condition))
}

fn bundle_mac(condition: Expression) -> Job {
    Job::default()
        .cond(condition)
        .runs_on(runners::MAC_DEFAULT)
        .timeout_minutes(120u32)
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
        .add_step(steps::clean_target_dir())
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

fn bundle_linux(condition: Expression, arch: runners::Arch) -> Job {
    let sha = "${{ github.event.pull_request.head.sha || github.sha }}";
    let artifact_name = format!("zed-{}-{}.tar.gz", sha, arch.triple());
    let remote_server_artifact_name = format!("zed-remote-server-{}-{}.tar.gz", sha, arch.triple());
    let mut job = Job::default()
        .cond(condition)
        .runs_on(arch.linux_runner())
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

fn bundle_windows(condition: Expression) -> Job {
    Job::default()
        .cond(condition)
        .runs_on(runners::WINDOWS_DEFAULT)
        .timeout_minutes(120u32)
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
        .add_step(
            steps::script_windows("script/bundle-windows.ps1")
                .working_directory("${{ env.ZED_WORKSPACE }}"),
        )
        .add_step(steps::upload_artifact(
            "Zed_${{ github.event.pull_request.head.sha || github.sha }}-x86_64.exe",
            "${{ env.SETUP_PATH }}",
        ))
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
