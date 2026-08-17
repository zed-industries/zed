#![cfg(any(target_os = "linux", target_os = "android"))]

use {
    common::*,
    minidump::Minidump,
    minidump_writer::{minidump_writer::MinidumpWriterConfig, FailSpotName},
    serde_json::json,
};

mod common;

#[test]
fn soft_error_stream() {
    let mut child = start_child_and_wait_for_threads(1);
    let pid = child.id() as i32;

    let mut tmpfile = tempfile::Builder::new()
        .prefix("soft_error_stream")
        .tempfile()
        .unwrap();

    let mut fail_client = FailSpotName::testing_client();
    fail_client.set_enabled(FailSpotName::StopProcess, true);

    // Write a minidump
    MinidumpWriterConfig::new(pid, pid)
        .write(&mut tmpfile)
        .expect("cound not write minidump");
    child.kill().expect("Failed to kill process");
    child.wait().expect("Failed to wait on killed process");

    // Ensure the minidump has a MozSoftErrors present
    let dump = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");
    read_minidump_soft_errors_or_panic(&dump);
}

#[test]
fn soft_error_stream_content() {
    let expected_errors = vec![
        json!({"InitErrors": [
            {"StopProcessFailed": {"Stop": "EPERM"}},
            {"FillMissingAuxvInfoErrors": ["InvalidFormat"]},
            {"EnumerateThreadsErrors": [
                {"ReadThreadNameFailed": "\
                    Custom {\n    \
                        kind: Other,\n    \
                        error: \"testing requested failure reading thread name\",\n\
                    }"
                }
            ]},
            {"SuspendThreadsErrors": [{"PtraceAttachError": [1234, "EPERM"]}]}
        ]}),
        json!({"WriteSystemInfoErrors": [
            {"WriteCpuInformationFailed": {"IOError": "\
                Custom {\n    \
                    kind: Other,\n    \
                    error: \"test requested cpuinfo file failure\",\n\
                }"
            }}
        ]}),
    ];

    let mut child = start_child_and_wait_for_threads(1);
    let pid = child.id() as i32;

    let mut tmpfile = tempfile::Builder::new()
        .prefix("soft_error_stream_content")
        .tempfile()
        .unwrap();

    let mut fail_client = FailSpotName::testing_client();
    for name in [
        FailSpotName::StopProcess,
        FailSpotName::FillMissingAuxvInfo,
        FailSpotName::ThreadName,
        FailSpotName::SuspendThreads,
        FailSpotName::CpuInfoFileOpen,
    ] {
        fail_client.set_enabled(name, true);
    }

    // Write a minidump
    MinidumpWriterConfig::new(pid, pid)
        .write(&mut tmpfile)
        .expect("cound not write minidump");
    child.kill().expect("Failed to kill process");
    child.wait().expect("Failed to wait on killed process");

    // Ensure the MozSoftErrors stream contains the expected errors
    let dump = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");
    assert_soft_errors_in_minidump(&dump, &expected_errors);
}
