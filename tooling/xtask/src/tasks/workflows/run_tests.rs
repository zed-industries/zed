use gh_workflow::Workflow;

use super::{
    runners::{self, Platform},
    steps::{self, NamedJob, named, release_job},
};

pub(crate) fn run_tests() -> Workflow {
    let check_style = check_style();
    let windows_tests = run_platform_tests(Platform::Windows);
    let linux_tests = run_platform_tests(Platform::Linux);
    let mac_tests = run_platform_tests(Platform::Mac);

    named::workflow()
        .add_job(check_style.name, check_style.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(mac_tests.name, mac_tests.job)
}

pub(crate) fn run_platform_tests(platform: Platform) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    NamedJob {
        name: format!("run_tests_{platform}"),
        job: release_job(&[])
            .runs_on(runner)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(platform))
            .add_step(steps::setup_node())
            .add_step(steps::cargo_install_nextest(platform))
            .add_step(steps::clear_target_dir_if_large(platform))
            .add_step(steps::cargo_nextest(platform))
            .add_step(steps::cleanup_cargo_config(platform)),
    }
}

pub(crate) fn check_style() -> NamedJob {
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
