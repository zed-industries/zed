//-
// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use std::fs;
use std::io::{self, BufRead, Read, Seek, Write};
use std::path::Path;
use std::string::String;
use std::vec::Vec;

use crate::test_runner::{Seed, TestCaseError, TestCaseResult};

const SENTINEL: &'static str = "proptest-forkfile";

/// A "replay" of a `TestRunner` invocation.
///
/// The replay mechanism is used to support forking. When a child process
/// exits, the parent can read the replay to reproduce the state the child had;
/// similarly, if a child crashes, a new one can be started and given a replay
/// which steps it one complication past the input that caused the crash.
///
/// The replay system is tightly coupled to the `TestRunner` itself. It does
/// not carry enough information to be used in different builds of the same
/// application, or even two different runs of the test process since changes
/// to the persistence file will perturb the replay.
///
/// `Replay` has a special string format for being stored in files. It starts
/// with a line just containing the text in `SENTINEL`, then 16 lines
/// containing the values of `seed`, then an unterminated line consisting of
/// `+`, `-`, and `!` characters to indicate test case passes/failures/rejects,
/// `.` to indicate termination of the test run, or ` ` as a dummy "I'm alive"
/// signal. This format makes it easy for the child process to blindly append
/// to the file without having to worry about the possibility of appends being
/// non-atomic.
#[derive(Clone, Debug)]
pub(crate) struct Replay {
    /// The seed of the RNG used to start running the test cases.
    pub(crate) seed: Seed,
    /// A log of whether certain test cases passed or failed. The runner will
    /// assume the same results occur without actually running the test cases.
    pub(crate) steps: Vec<TestCaseResult>,
}

impl Replay {
    /// If `other` is longer than `self`, add the extra elements to `self`.
    pub fn merge(&mut self, other: &Replay) {
        if other.steps.len() > self.steps.len() {
            let sl = self.steps.len();
            self.steps.extend_from_slice(&other.steps[sl..]);
        }
    }
}

/// Result of loading a replay file.
#[derive(Clone, Debug)]
pub(crate) enum ReplayFileStatus {
    /// The file is valid and represents a currently-in-progress test.
    InProgress(Replay),
    /// The file is valid, but indicates that all testing has completed.
    Terminated(Replay),
    /// The file is not parsable.
    Corrupt,
}

/// Open the file in the usual read+append+create mode.
pub(crate) fn open_file(path: impl AsRef<Path>) -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .truncate(false)
        .open(path)
}

fn step_to_char(step: &TestCaseResult) -> char {
    match *step {
        Ok(_) => '+',
        Err(TestCaseError::Reject(_)) => '!',
        Err(TestCaseError::Fail(_)) => '-',
    }
}

/// Append the given step to the given output.
pub(crate) fn append(
    mut file: impl Write,
    step: &TestCaseResult,
) -> io::Result<()> {
    write!(file, "{}", step_to_char(step))
}

/// Append a no-op step to the given output.
pub(crate) fn ping(mut file: impl Write) -> io::Result<()> {
    write!(file, " ")
}

/// Append a termination mark to the given output.
pub(crate) fn terminate(mut file: impl Write) -> io::Result<()> {
    write!(file, ".")
}

impl Replay {
    /// Write the full state of this `Replay` to the given output.
    pub fn init_file(&self, mut file: impl Write) -> io::Result<()> {
        writeln!(file, "{}", SENTINEL)?;
        writeln!(file, "{}", self.seed.to_persistence())?;

        let mut step_data = Vec::<u8>::new();
        for step in &self.steps {
            step_data.push(step_to_char(step) as u8);
        }

        file.write_all(&step_data)?;

        Ok(())
    }

    /// Mark the replay as complete in the file.
    pub fn complete(mut file: impl Write) -> io::Result<()> {
        write!(file, ".")
    }

    /// Parse a `Replay` out of the given file.
    ///
    /// The reader is implicitly seeked to the beginning before reading.
    pub fn parse_from(
        mut file: impl Read + Seek,
    ) -> io::Result<ReplayFileStatus> {
        file.seek(io::SeekFrom::Start(0))?;

        let mut reader = io::BufReader::new(&mut file);
        let mut line = String::new();

        // Ensure it starts with the sentinel. We do this since we rely on a
        // named temporary file which could be in a location where another
        // actor could replace it with, eg, a symlink to a location they don't
        // control but we do. By rejecting a read from a file missing the
        // sentinel, and not doing any writes if we can't read the file, we
        // won't risk overwriting another file since the prospective attacker
        // would need to be able to change the file to start with the sentinel
        // themselves.
        //
        // There are still some possible symlink attacks that can work by
        // tricking us into reading, but those are non-destructive things like
        // interfering with a FIFO or Unix socket.
        reader.read_line(&mut line)?;
        if SENTINEL != line.trim() {
            return Ok(ReplayFileStatus::Corrupt);
        }

        line.clear();
        reader.read_line(&mut line)?;
        let seed = match Seed::from_persistence(&line) {
            Some(seed) => seed,
            None => return Ok(ReplayFileStatus::Corrupt),
        };

        line.clear();
        reader.read_line(&mut line)?;

        let mut steps = Vec::new();
        for ch in line.chars() {
            match ch {
                '+' => steps.push(Ok(())),
                '-' => steps
                    .push(Err(TestCaseError::fail("failed in other process"))),
                '!' => steps.push(Err(TestCaseError::reject(
                    "rejected in other process",
                ))),
                '.' => {
                    return Ok(ReplayFileStatus::Terminated(Replay {
                        seed,
                        steps,
                    }))
                }
                ' ' => (),
                _ => return Ok(ReplayFileStatus::Corrupt),
            }
        }

        Ok(ReplayFileStatus::InProgress(Replay { seed, steps }))
    }
}
