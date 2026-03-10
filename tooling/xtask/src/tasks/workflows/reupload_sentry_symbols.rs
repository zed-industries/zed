use gh_workflow::*;

use crate::tasks::workflows::{
    runners::{self, Arch, Platform},
    steps::{self, CommonJobConditions, NamedJob, named},
    vars::{WorkflowInput, bundle_envs},
};

pub fn reupload_sentry_symbols() -> Workflow {
    let tag = WorkflowInput::string("tag", None)
        .description("Git tag to rebuild and upload symbols for (e.g. v0.200.0-pre)");

    let jobs = [
        upload_job(&tag, Platform::Linux, Arch::X86_64),
        upload_job(&tag, Platform::Linux, Arch::AARCH64),
        upload_job(&tag, Platform::Mac, Arch::X86_64),
        upload_job(&tag, Platform::Mac, Arch::AARCH64),
        upload_job(&tag, Platform::Windows, Arch::X86_64),
        upload_job(&tag, Platform::Windows, Arch::AARCH64),
    ];

    let mut workflow = named::workflow()
        .on(Event::default()
            .workflow_dispatch(WorkflowDispatch::default().add_input(tag.name, tag.input())))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"));

    for job in jobs {
        workflow = workflow.add_job(job.name, job.job);
    }

    workflow
}

fn upload_job(tag: &WorkflowInput, platform: Platform, arch: Arch) -> NamedJob {
    fn upload_symbols_unix() -> Step<Run> {
        named::bash("./script/upload-sentry-symbols --verify")
    }

    fn upload_symbols_windows() -> Step<Run> {
        named::pwsh("./script/upload-sentry-symbols.ps1 -Verify")
    }

    let runner = match platform {
        Platform::Linux => arch.linux_bundler(),
        Platform::Mac => runners::MAC_DEFAULT,
        Platform::Windows => runners::WINDOWS_DEFAULT,
    };

    let mut job = Job::default()
        .runs_on(runner)
        .timeout_minutes(60u32)
        .with_repository_owner_guard()
        .envs(bundle_envs(platform))
        .add_step(steps::checkout_repo().with_ref(tag))
        .add_step(steps::setup_sentry());

    if platform == Platform::Linux {
        job = steps::install_linux_dependencies(job);
    }

    job = match platform {
        Platform::Windows => job.add_step(upload_symbols_windows()),
        _ => job.add_step(upload_symbols_unix()),
    };

    NamedJob {
        name: format!("upload_{platform}_{arch}"),
        job,
    }
}
