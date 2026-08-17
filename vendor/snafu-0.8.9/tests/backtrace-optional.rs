use snafu::{prelude::*, Backtrace, ErrorCompat};

#[derive(Debug, Snafu)]
enum Error {
    BacktraceAlways { backtrace: Backtrace },
    BacktraceSometimes { backtrace: Option<Backtrace> },
}

#[test]
fn bare_backtrace_is_always_present() {
    let always = BacktraceAlwaysSnafu.build();
    assert!(ErrorCompat::backtrace(&always).is_some());
}

#[test]
fn optional_backtrace_is_not_present_without_environment_variable() {
    use std::env;

    // The check requires RUST_BACKTRACE to be unset. Back up the
    // current value of the environment variable and restore it
    // afterwards. If we add more tests to this file that rely on
    // environment variables, we should introduce a mutex as
    // environment variables are a global resource.
    const BACKTRACE_ENV_NAME: &str = "RUST_BACKTRACE";
    let previous_backtrace_env_value = env::var_os(BACKTRACE_ENV_NAME);
    env::remove_var(BACKTRACE_ENV_NAME);

    let sometimes = BacktraceSometimesSnafu.build();
    assert!(ErrorCompat::backtrace(&sometimes).is_none());

    if let Some(v) = previous_backtrace_env_value {
        env::set_var(BACKTRACE_ENV_NAME, v);
    }
}
