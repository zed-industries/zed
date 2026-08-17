//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs, unsafe_code)]

//! Rusty-fork provides a way to "fork" unit tests into separate processes.
//!
//! There are a number of reasons to want to run some tests in isolated
//! processes:
//!
//! - When tests share a process, if any test causes the process to abort,
//! segfault, overflow the stack, etc., the entire test runner process dies. If
//! the test is in a subprocess, only the subprocess dies and the test runner
//! simply fails the test.
//!
//! - Isolating a test to a subprocess makes it possible to add a timeout to
//! the test and forcibly terminate it and produce a normal test failure.
//!
//! - Tests which need to interact with some inherently global property, such
//! as the current working directory, can do so without interfering with other
//! tests.
//!
//! This crate itself provides two things:
//!
//! - The [`rusty_fork_test!`](macro.rusty_fork_test.html) macro, which is a
//! simple way to wrap standard Rust tests to be run in subprocesses with
//! optional timeouts.
//!
//! - The [`fork`](fn.fork.html) function which can be used as a building block
//! to make other types of process isolation strategies.
//!
//! ## Quick Start
//!
//! If you just want to run normal Rust tests in isolated processes, getting
//! started is pretty quick.
//!
//! In `Cargo.toml`, add
//!
//! ```toml
//! [dev-dependencies]
//! rusty-fork = "0.3.0"
//! ```
//!
//! Then, you can simply wrap any test(s) to be isolated with the
//! [`rusty_fork_test!`](macro.rusty_fork_test.html) macro.
//!
//! ```rust
//! use rusty_fork::rusty_fork_test;
//!
//! rusty_fork_test! {
//! # /* NOREADME
//!     #[test]
//! # NOREADME */
//!     fn my_test() {
//!         assert_eq!(2, 1 + 1);
//!     }
//!
//!     // more tests...
//! }
//! # // NOREADME
//! # fn main() { my_test(); } // NOREADME
//! ```
//!
//! For more advanced usage, have a look at the [`fork`](fn.fork.html)
//! function.
//!
//! ## How rusty-fork works
//!
//! Unix-style process forking isn't really viable within the standard Rust
//! test environment for a number of reasons.
//!
//! - While true process forking can be done on Windows, it's neither fast nor
//! reliable.
//!
//! - The Rust test environment is multi-threaded, so attempting to do anything
//! non-trivial after a process fork would result in undefined behaviour.
//!
//! Rusty-fork instead works by _spawning_ a fresh instance of the current
//! process, after adjusting the command-line to ensure that only the desired
//! test is entered. Some additional coordination establishes the parent/child
//! branches and (not quite seamlessly) integrates the child's output with the
//! test output capture system.
//!
//! Coordination between the processes is performed via environment variables,
//! since there is otherwise no way to pass parameters to a test.
//!
//! Since it needs to spawn new copies of the test runner executable,
//! rusty-fork does need to know about the meaning of every flag passed by the
//! user. If any unknown flags are encountered, forking will fail. Please do
//! not hesitate to file
//! [issues](https://github.com/AltSysrq/rusty-fork/issues) if rusty-fork fails
//! to recognise any valid flags passed to the test runner.
//!
//! It is possible to inform rusty-fork of new flags without patching by
//! setting environment variables. For example, if a new `--frob-widgets` flag
//! were added to the test runner, you could set `RUSTY_FORK_FLAG_FROB_WIDGETS`
//! to one of the following:
//!
//! - `pass` — Pass the flag (just the flag) to the child process
//! - `pass-arg` — Pass the flag and its following argument to the child process
//! - `drop` — Don't pass the flag to the child process
//! - `drop-arg` — Don't pass the flag to the child process, and ignore whatever
//!   argument follows.
//!
//! In general, arguments that affect which tests are run should be dropped,
//! and others should be passed.
//!
//! <!-- ENDREADME -->

#[macro_use] extern crate quick_error;

#[macro_use] mod sugar;
#[macro_use] pub mod fork_test;
mod error;
mod cmdline;
mod fork;
mod child_wrapper;

pub use crate::sugar::RustyForkId;
pub use crate::error::{Error, Result};
pub use crate::fork::fork;
pub use crate::child_wrapper::{ChildWrapper, ExitStatusWrapper};
