//! Parses output of [libtest](https://github.com/rust-lang/rust/blob/master/library/test/src/formatters/json.rs).
//!
//! Since this module parses output in an unstable format, all structs in this module may change at any time, and are exempt from semver guarantees.
use serde::{Deserialize, Serialize};

/// Suite related event
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "event")]
#[serde(rename_all = "lowercase")]
/// Suite event
pub enum SuiteEvent {
    /// emitted on the start of a test run, and the start of the doctests
    Started {
        /// number of tests in this suite
        test_count: usize,
    },
    /// the suite has finished
    Ok {
        /// the number of tests that passed
        passed: usize,
        /// the number of tests that failed
        failed: usize,
        /// number of tests that were ignored
        ignored: usize,
        /// number of benchmarks run
        measured: usize,
        /// i think this is based on what you specify in the cargo test argument
        filtered_out: usize,
        /// how long the suite took to run
        exec_time: f32,
    },
    /// the suite has at least one failing test
    Failed {
        /// the number of tests that passed
        passed: usize,
        /// the number of tests that failed
        failed: usize,
        /// number of tests that were ignored
        ignored: usize,
        /// i think its something to do with benchmarks?
        measured: usize,
        /// i think this is based on what you specify in the cargo test argument
        filtered_out: usize,
        /// how long the suite took to run
        exec_time: f32,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "event")]
#[serde(rename_all = "lowercase")]
/// Test event
pub enum TestEvent {
    /// a new test starts
    Started {
        /// the name of this test
        name: String,
    },
    /// the test has finished
    Ok {
        /// which one
        name: String,
        /// in how long
        exec_time: f32,
        /// what did it say?
        stdout: Option<String>,
    },
    /// the test has failed
    Failed {
        /// which one
        name: String,
        /// in how long
        exec_time: f32,
        /// why?
        stdout: Option<String>,
        /// it timed out?
        reason: Option<String>,
        /// what message
        message: Option<String>,
    },
    /// the test has been ignored
    Ignored {
        /// which one
        name: String,
    },
    /// the test has timed out
    Timeout {
        /// which one
        name: String,
    },
}

impl TestEvent {
    /// Get the name of this test
    pub fn name(&self) -> &str {
        let (Self::Started { name }
        | Self::Ok { name, .. }
        | Self::Ignored { name }
        | Self::Failed { name, .. }
        | Self::Timeout { name }) = self;
        name
    }

    /// Get the stdout of this test, if available.
    pub fn stdout(&self) -> Option<&str> {
        match self {
            Self::Ok { stdout, .. } | Self::Failed { stdout, .. } => stdout.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the output of `cargo test -- -Zunstable-options --report-time --show-output --format json`.
///
/// requires --report-time
///
/// # Stability
///
/// As this struct is for interfacing with the unstable libtest json output, this struct may change at any time, without semver guarantees.
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TestMessage {
    /// suite related message
    Suite(SuiteEvent),
    /// test related message
    Test(TestEvent),
    /// bench related message
    Bench {
        /// name of benchmark
        name: String,
        /// distribution
        median: f32,
        /// deviation
        deviation: f32,
        /// thruput in MiB per second
        mib_per_second: Option<f32>,
    },
}

#[test]
fn deser() {
    macro_rules! run {
        ($($input:literal parses to $output:expr),+) => {
            $(assert_eq!(dbg!(serde_json::from_str::<TestMessage>($input)).unwrap(), $output);)+
        };
    }
    run![
        r#"{ "type": "suite", "event": "started", "test_count": 2 }"# parses to TestMessage::Suite(SuiteEvent::Started { test_count: 2 }),
        r#"{ "type": "test", "event": "started", "name": "fail" }"# parses to TestMessage::Test(TestEvent::Started { name: "fail".into() }),
        r#"{ "type": "test", "name": "fail", "event": "ok", "exec_time": 0.000003428, "stdout": "hello world" }"# parses to TestMessage::Test(TestEvent::Ok { name: "fail".into(), exec_time: 0.000003428, stdout: Some("hello world".into()) }),
        r#"{ "type": "test", "event": "started", "name": "nope" }"# parses to TestMessage::Test(TestEvent::Started { name: "nope".into() }),
        r#"{ "type": "test", "name": "nope", "event": "ignored" }"# parses to TestMessage::Test(TestEvent::Ignored { name: "nope".into() }),
        r#"{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "ignored": 1, "measured": 0, "filtered_out": 0, "exec_time": 0.000684028 }"# parses to TestMessage::Suite(SuiteEvent::Ok { passed: 1, failed: 0, ignored: 1, measured: 0, filtered_out: 0, exec_time: 0.000684028 })
    ];

    run![
        r#"{ "type": "suite", "event": "started", "test_count": 2 }"# parses to TestMessage::Suite(SuiteEvent::Started { test_count: 2 }),
        r#"{ "type": "test", "event": "started", "name": "fail" }"# parses to TestMessage::Test(TestEvent::Started { name: "fail".into() }),
        r#"{ "type": "test", "event": "started", "name": "benc" }"# parses to TestMessage::Test(TestEvent::Started { name: "benc".into() }),
        r#"{ "type": "bench", "name": "benc", "median": 0, "deviation": 0 }"# parses to TestMessage::Bench { name: "benc".into(), median: 0., deviation: 0., mib_per_second: None },
        r#"{ "type": "test", "name": "fail", "event": "failed", "exec_time": 0.000081092, "stdout": "thread 'fail' panicked" }"# parses to TestMessage::Test(TestEvent::Failed { name: "fail".into(), exec_time: 0.000081092, stdout: Some("thread 'fail' panicked".into()), reason: None, message: None} ),
        r#"{ "type": "suite", "event": "failed", "passed": 0, "failed": 1, "ignored": 0, "measured": 1, "filtered_out": 0, "exec_time": 0.000731068 }"# parses to TestMessage::Suite(SuiteEvent::Failed { passed: 0, failed: 1, ignored: 0, measured: 1, filtered_out: 0, exec_time: 0.000731068 })
    ];
}
