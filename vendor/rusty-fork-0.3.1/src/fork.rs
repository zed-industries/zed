//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::env;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Seek};
use std::panic;
use std::process;

use fnv;
use tempfile;

use crate::cmdline;
use crate::error::*;
use crate::child_wrapper::ChildWrapper;

const OCCURS_ENV: &str = "RUSTY_FORK_OCCURS";
const OCCURS_TERM_LENGTH: usize = 17; /* ':' plus 16 hexits */

/// Simulate a process fork.
///
/// The function documentation here only lists information unique to calling it
/// directly; please see the crate documentation for more details on how the
/// forking process works.
///
/// Since this is not a true process fork, the calling code must be structured
/// to ensure that the child process, upon starting from the same entry point,
/// also reaches this same `fork()` call. Recursive forks are supported; the
/// child branch is taken from all child processes of the fork even if it is
/// not directly the child of a particular branch. However, encountering the
/// same fork point more than once in a single execution sequence of a child
/// process is not (e.g., putting this call in a recursive function) and
/// results in unspecified behaviour.
///
/// The child's output is buffered into an anonymous temporary file. Before
/// this call returns, this output is copied to the parent's standard output
/// (passing through the redirect mechanism Rust test uses).
///
/// `test_name` must exactly match the full path of the test function being
/// run.
///
/// `fork_id` is a unique identifier identifying this particular fork location.
/// This *must* be stable across processes of the same executable; pointers are
/// not suitable stable, and string constants may not be suitably unique. The
/// [`rusty_fork_id!()`](macro.rusty_fork_id.html) macro is the recommended way
/// to supply this parameter.
///
/// If this is the parent process, `in_parent` is invoked, and the return value
/// becomes the return value from this function. The callback is passed a
/// handle to the file which receives the child's output. If is the callee's
/// responsibility to wait for the child to exit. If this is the child process,
/// `in_child` is invoked, and when the callback returns, the child process
/// exits.
///
/// If `in_parent` returns or panics before the child process has terminated,
/// the child process is killed.
///
/// If `in_child` panics, the child process exits with a failure code
/// immediately rather than let the panic propagate out of the `fork()` call.
///
/// `process_modifier` is invoked on the `std::process::Command` immediately
/// before spawning the new process. The callee may modify the process
/// parameters if desired, but should not do anything that would modify or
/// remove any environment variables beginning with `RUSTY_FORK_`.
///
/// ## Panics
///
/// Panics if the environment indicates that there are already at least 16
/// levels of fork nesting.
///
/// Panics if `std::env::current_exe()` fails determine the path to the current
/// executable.
///
/// Panics if any argument to the current process is not valid UTF-8.
pub fn fork<ID, MODIFIER, PARENT, CHILD, R>(
    test_name: &str,
    fork_id: ID,
    process_modifier: MODIFIER,
    in_parent: PARENT,
    in_child: CHILD) -> Result<R>
where
    ID : Hash,
    MODIFIER : FnOnce (&mut process::Command),
    PARENT : FnOnce (&mut ChildWrapper, &mut fs::File) -> R,
    CHILD : FnOnce ()
{
    let fork_id = id_str(fork_id);

    // Erase the generics so we don't instantiate the actual implementation for
    // every single test
    let mut return_value = None;
    let mut process_modifier = Some(process_modifier);
    let mut in_parent = Some(in_parent);
    let mut in_child = Some(in_child);

    fork_impl(test_name, fork_id,
              &mut |cmd| process_modifier.take().unwrap()(cmd),
              &mut |child, file| return_value = Some(
                  in_parent.take().unwrap()(child, file)),
              &mut || in_child.take().unwrap()())
        .map(|_| return_value.unwrap())
}

fn fork_impl(test_name: &str, fork_id: String,
             process_modifier: &mut dyn FnMut (&mut process::Command),
             in_parent: &mut dyn FnMut (&mut ChildWrapper, &mut fs::File),
             in_child: &mut dyn FnMut ()) -> Result<()> {
    let mut occurs = env::var(OCCURS_ENV).unwrap_or_else(|_| String::new());
    if occurs.contains(&fork_id) {
        match panic::catch_unwind(panic::AssertUnwindSafe(in_child)) {
            Ok(_) => process::exit(0),
            // Assume that the default panic handler already printed something
            //
            // We don't use process::abort() since it produces core dumps on
            // some systems and isn't something more special than a normal
            // panic.
            Err(_) => process::exit(70 /* EX_SOFTWARE */),
        }
    } else {
        // Prevent misconfiguration creating a fork bomb
        if occurs.len() > 16 * OCCURS_TERM_LENGTH {
            panic!("rusty-fork: Not forking due to >=16 levels of recursion");
        }

        let file = tempfile::tempfile()?;

        struct KillOnDrop(ChildWrapper, fs::File);
        impl Drop for KillOnDrop {
            fn drop(&mut self) {
                // Kill the child if it hasn't exited yet
                let _ = self.0.kill();

                // Copy the child's output to our own
                // Awkwardly, `print!()` and `println!()` are our only gateway
                // to putting things in the captured output. Generally test
                // output really is text, so work on that assumption and read
                // line-by-line, converting lossily into UTF-8 so we can
                // println!() it.
                let _ = self.1.seek(io::SeekFrom::Start(0));

                let mut buf = Vec::new();
                let mut br = io::BufReader::new(&mut self.1);
                loop {
                    // We can't use read_line() or lines() since they break if
                    // there's any non-UTF-8 output at all. \n occurs at the
                    // end of the line endings on all major platforms, so we
                    // can just use that as a delimiter.
                    if br.read_until(b'\n', &mut buf).is_err() {
                        break;
                    }
                    if buf.is_empty() {
                        break;
                    }

                    // not println!() because we already have a line ending
                    // from above.
                    print!("{}", String::from_utf8_lossy(&buf));
                    buf.clear();
                }
            }
        }

        occurs.push_str(&fork_id);
        let mut command =
            process::Command::new(
                env::current_exe()
                    .expect("current_exe() failed, cannot fork"));
        command
            .args(cmdline::strip_cmdline(env::args())?)
            .args(cmdline::RUN_TEST_ARGS)
            .arg(test_name)
            .env(OCCURS_ENV, &occurs)
            .stdin(process::Stdio::null())
            .stdout(file.try_clone()?)
            .stderr(file.try_clone()?);
        process_modifier(&mut command);

        let mut child = command.spawn().map(ChildWrapper::new)
            .map(|p| KillOnDrop(p, file))?;

        let ret = in_parent(&mut child.0, &mut child.1);

        Ok(ret)
    }
}

fn id_str<ID : Hash>(id: ID) -> String {
    let mut hasher = fnv::FnvHasher::default();
    id.hash(&mut hasher);

    return format!(":{:016X}", hasher.finish());
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use std::thread;

    use super::*;

    fn sleep(ms: u64) {
        thread::sleep(::std::time::Duration::from_millis(ms));
    }

    fn capturing_output(cmd: &mut process::Command) {
        // Only actually capture stdout since we can't use
        // wait_with_output() since it for some reason consumes the `Child`.
        cmd.stdout(process::Stdio::piped())
            .stderr(process::Stdio::inherit());
    }

    fn inherit_output(cmd: &mut process::Command) {
        cmd.stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit());
    }

    fn wait_for_child_output(child: &mut ChildWrapper,
                             _file: &mut fs::File) -> String {
        let mut output = String::new();
        child.inner_mut().stdout.as_mut().unwrap()
            .read_to_string(&mut output).unwrap();
        assert!(child.wait().unwrap().success());
        output
    }

    fn wait_for_child(child: &mut ChildWrapper,
                      _file: &mut fs::File) {
        assert!(child.wait().unwrap().success());
    }

    #[test]
    fn fork_basically_works() {
        let status =
            fork("fork::test::fork_basically_works", rusty_fork_id!(),
                 |_| (),
                 |child, _| child.wait().unwrap(),
                 || println!("hello from child")).unwrap();
        assert!(status.success());
    }

    #[test]
    fn child_output_captured_and_repeated() {
        let output = fork(
            "fork::test::child_output_captured_and_repeated",
            rusty_fork_id!(),
            capturing_output, wait_for_child_output,
            || fork(
                "fork::test::child_output_captured_and_repeated",
                rusty_fork_id!(),
                |_| (), wait_for_child,
                || println!("hello from child")).unwrap())
            .unwrap();
        assert!(output.contains("hello from child"));
    }

    #[test]
    fn child_killed_if_parent_exits_first() {
        let output = fork(
            "fork::test::child_killed_if_parent_exits_first",
            rusty_fork_id!(),
            capturing_output, wait_for_child_output,
            || fork(
                "fork::test::child_killed_if_parent_exits_first",
                rusty_fork_id!(),
                inherit_output, |_, _| (),
                || {
                    sleep(1_000);
                    println!("hello from child");
                }).unwrap()).unwrap();

        sleep(2_000);
        assert!(!output.contains("hello from child"),
                "Had unexpected output:\n{}", output);
    }

    #[test]
    fn child_killed_if_parent_panics_first() {
        let output = fork(
            "fork::test::child_killed_if_parent_panics_first",
            rusty_fork_id!(),
            capturing_output, wait_for_child_output,
            || {
                assert!(
                    panic::catch_unwind(panic::AssertUnwindSafe(|| fork(
                        "fork::test::child_killed_if_parent_panics_first",
                        rusty_fork_id!(),
                        inherit_output,
                        |_, _| panic!("testing a panic, nothing to see here"),
                        || {
                            sleep(1_000);
                            println!("hello from child");
                        }).unwrap())).is_err());
            }).unwrap();

        sleep(2_000);
        assert!(!output.contains("hello from child"),
                "Had unexpected output:\n{}", output);
    }

    #[test]
    fn child_aborted_if_panics() {
        let status = fork(
            "fork::test::child_aborted_if_panics",
            rusty_fork_id!(),
            |_| (),
            |child, _| child.wait().unwrap(),
            || panic!("testing a panic, nothing to see here")).unwrap();
        assert_eq!(70, status.code().unwrap());
    }
}
