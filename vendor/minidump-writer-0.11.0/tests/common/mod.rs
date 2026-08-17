use std::{
    error,
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    result,
};

#[allow(unused)]
type Error = Box<dyn error::Error + std::marker::Send + std::marker::Sync>;
#[allow(unused)]
pub type Result<T> = result::Result<T, Error>;

fn build_command() -> Command {
    // Anything that needs to spawn a child will need to give it permission to
    // ptrace itself, as Yama LSM will normally only allow a parent to debug
    // a child, not the other way around.
    #[cfg(target_os = "linux")]
    {
        let rc = unsafe { libc::prctl(libc::PR_SET_PTRACER, libc::PR_SET_PTRACER_ANY) };
        assert_eq!(rc, 0);
    }

    let mut cmd;
    if let Some(binary) = std::env::var_os("TEST_HELPER") {
        cmd = Command::new(binary);
    } else {
        cmd = Command::new("cargo");
        cmd.args(["run", "-q", "--bin", "test"]);

        // In normal cases where the host and target are the same this won't matter,
        // but tests will fail if you are eg running in a cross container which will
        // likely be x86_64 but may be targetting aarch64 or i686, which will result
        // in tests failing, or at the least not testing what you think
        cmd.args(["--target", current_platform::CURRENT_PLATFORM, "--"]);
    }

    cmd.env("RUST_BACKTRACE", "1");

    cmd
}

#[allow(unused)]
pub fn spawn_child(command: &str, args: &[&str]) {
    let mut cmd = build_command();
    cmd.arg(command).args(args);

    let child = cmd.output().expect("failed to execute child");

    println!("Child output:");
    std::io::stdout().write_all(&child.stdout).unwrap();
    std::io::stdout().write_all(&child.stderr).unwrap();
    assert_eq!(child.status.code().expect("No return value"), 0);
}

fn start_child_and_wait_for_threads_helper(command: &str, num: usize) -> Child {
    let mut cmd = build_command();
    cmd.arg(command).arg(num.to_string());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn cargo");
    wait_for_threads(&mut child, num);
    child
}

#[allow(unused)]
pub fn start_child_and_wait_for_threads(num: usize) -> Child {
    start_child_and_wait_for_threads_helper("spawn_and_wait", num)
}

#[allow(unused)]
pub fn start_child_and_wait_for_named_threads(num: usize) -> Child {
    start_child_and_wait_for_threads_helper("spawn_name_wait", num)
}

#[allow(unused)]
pub fn start_child_and_wait_for_create_files(num: usize) -> Child {
    start_child_and_wait_for_threads_helper("create_files_wait", num)
}

#[allow(unused)]
pub fn wait_for_threads(child: &mut Child, num: usize) {
    let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
    let mut lines = 0;
    while lines < num {
        let mut buf = String::new();
        match f.read_line(&mut buf) {
            Ok(_) => {
                if buf == "1\n" {
                    lines += 1;
                }
            }
            Err(e) => {
                std::panic::panic_any(e);
            }
        }
    }
}

#[allow(unused)]
pub fn start_child_and_return(args: &[&str]) -> Child {
    let mut cmd = build_command();
    cmd.args(args);

    cmd.stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child")
}

#[allow(unused)]
pub fn read_minidump_soft_errors_or_panic<'a, T>(
    dump: &minidump::Minidump<'a, T>,
) -> serde_json::Value
where
    T: std::ops::Deref<Target = [u8]> + 'a,
{
    let contents = std::str::from_utf8(
        dump.get_raw_stream(minidump_common::format::MINIDUMP_STREAM_TYPE::MozSoftErrors.into())
            .expect("missing soft error stream"),
    )
    .expect("expected utf-8 stream");

    serde_json::from_str(contents).expect("expected json")
}

#[allow(unused)]
pub fn assert_soft_errors_in_minidump<'a, 'b, T, I>(
    dump: &minidump::Minidump<'a, T>,
    expected_errors: I,
) where
    T: std::ops::Deref<Target = [u8]> + 'a,
    I: IntoIterator<Item = &'b serde_json::Value>,
{
    let actual_json = read_minidump_soft_errors_or_panic(dump);
    let actual_errors = actual_json.as_array().unwrap();

    // Ensure that every error we expect is in the actual list somewhere
    for expected_error in expected_errors {
        assert!(actual_errors
            .iter()
            .any(|actual_error| actual_error == expected_error),
            "soft error list missing expected error `{expected_error:#?}`\nError_list: {actual_errors:#?}"
        );
    }
}
