// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Warnings

fn run_process(input: &str) -> String {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = std::process::Command::new("target/debug/examples/parse")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stderr).unwrap()
}

#[test]
fn style_01() {
    assert_eq!(
        run_process("> {}"),
        "WARN: Selector parsing failed cause unexpected combinator.\n"
    );
}

#[test]
fn style_02() {
    assert_eq!(
        run_process("@import 'subs.css';"),
        "WARN: The @import rule is not supported. Skipped.\n"
    );
}
