// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref PYTHON_VERSION: Regex = Regex::new("([\\d+\\.?]*).*")
        .expect("error parsing Version regex for Python Version in test");
    static ref PYTHON_FULLVERSION: Regex = Regex::new("(\\d+\\.?\\d+\\.?\\d+).*")
        .expect("error parsing Version regex for Python Version in test");
}

#[allow(dead_code)]
pub fn resolve_test_path(paths: &[&str]) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");

    paths.iter().for_each(|p| root.push(p));

    root
}

#[allow(dead_code)]
pub fn does_version_match(version: &str, expected_version: &str) -> bool {
    let version = get_version(version);
    expected_version.starts_with(&version)
}

fn get_version(value: &str) -> String {
    // Regex to extract just the d.d.d version from the full version string
    let captures = PYTHON_VERSION.captures(value).unwrap();
    let version = captures.get(1).unwrap().as_str().to_string();
    if version.ends_with('.') {
        version[..version.len() - 1].to_string()
    } else {
        version
    }
}

#[allow(dead_code)]
pub fn is_valid_version(value: &str) -> bool {
    PYTHON_FULLVERSION.is_match(value)
}
