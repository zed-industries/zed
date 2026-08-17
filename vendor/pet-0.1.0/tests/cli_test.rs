// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! CLI integration tests for the `pet find` and `pet resolve` commands.
//!
//! These tests spawn the pet binary via `std::process::Command` and validate
//! its JSON output. All tests are gated behind the `ci` feature flag since
//! they require a real Python installation on PATH.

use serde_json::Value;
use std::process::Command;

/// Helper to run `pet find --json` with optional extra args and return parsed JSON.
fn run_find_json(extra_args: &[&str]) -> (Value, std::process::Output) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pet"));
    cmd.arg("find").arg("--json");
    for arg in extra_args {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("failed to run pet find");
    assert!(
        output.status.success(),
        "pet find failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value =
        serde_json::from_slice(&output.stdout).expect("pet find stdout is not valid JSON");
    (json, output)
}

/// Test 1: `find --json` produces valid output with `managers` and `environments` arrays.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn find_json_output_is_valid() {
    let (json, _) = run_find_json(&[]);

    assert!(
        json["managers"].is_array(),
        "expected 'managers' array in output"
    );
    assert!(
        json["environments"].is_array(),
        "expected 'environments' array in output"
    );

    // Each environment should have at minimum a kind.
    // Executable may be null for environments without Python installed (e.g. Conda
    // envs created without specifying python as a dependency).
    let environments = json["environments"].as_array().unwrap();
    assert!(
        !environments.is_empty(),
        "expected at least one environment to be discovered"
    );
    let mut has_executable = false;
    for env in environments {
        assert!(env["kind"].is_string(), "environment missing 'kind': {env}");
        if env["executable"].is_string() {
            has_executable = true;
        }
    }
    assert!(
        has_executable,
        "expected at least one environment with an executable"
    );
}

/// Test 2: `resolve --json` returns a resolved environment with a version.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn resolve_json_output_has_version() {
    // First, find an environment to resolve
    let (found, _) = run_find_json(&[]);
    let environments = found["environments"]
        .as_array()
        .expect("expected environments array");
    assert!(
        !environments.is_empty(),
        "need at least one environment to test resolve"
    );

    // Pick an environment that has an executable path (skip broken entries)
    let exe = environments
        .iter()
        .find_map(|e| e["executable"].as_str())
        .expect("no environment with an executable found");

    // Now resolve it
    let output = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["resolve", exe, "--json"])
        .output()
        .expect("failed to run pet resolve");

    assert!(
        output.status.success(),
        "pet resolve failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let resolved: Value =
        serde_json::from_slice(&output.stdout).expect("pet resolve stdout is not valid JSON");

    // resolved should not be null
    assert!(!resolved.is_null(), "resolve returned null for {exe}");

    // Should have a version
    assert!(
        resolved["version"].is_string(),
        "resolved environment missing 'version' for {exe}: {resolved}"
    );

    // Executable should match what we passed in (or be a symlink equivalent)
    assert!(
        resolved["executable"].is_string(),
        "resolved environment missing 'executable'"
    );
}

/// Convert a PascalCase kind from JSON to the kebab-case format expected by clap's `--kind` flag.
fn to_cli_kind(json_kind: &str) -> String {
    let mut result = String::new();
    for (i, ch) in json_kind.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('-');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

/// Test 3: `find --kind <kind> --json` filters environments by kind.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn find_kind_filter_works() {
    // First find all environments to pick a kind that exists
    let (all, _) = run_find_json(&[]);
    let environments = all["environments"]
        .as_array()
        .expect("expected environments array");
    assert!(
        !environments.is_empty(),
        "need at least one environment to test kind filtering"
    );

    // Pick the kind of the first environment and convert to CLI format
    let json_kind = environments[0]["kind"]
        .as_str()
        .expect("expected kind string");
    let cli_kind = to_cli_kind(json_kind);

    // Now filter by that kind
    let (filtered, _) = run_find_json(&["--kind", &cli_kind]);
    let filtered_envs = filtered["environments"]
        .as_array()
        .expect("expected environments array");

    assert!(
        !filtered_envs.is_empty(),
        "expected at least one environment of kind '{json_kind}'"
    );

    // All returned environments must match the requested kind
    for env in filtered_envs {
        assert_eq!(
            env["kind"].as_str().unwrap(),
            json_kind,
            "environment kind mismatch: expected '{json_kind}', got {:?}",
            env["kind"]
        );
    }

    // Filtered count should be <= total count
    assert!(
        filtered_envs.len() <= environments.len(),
        "filtered count ({}) should not exceed total count ({})",
        filtered_envs.len(),
        environments.len()
    );
}

/// Test 4: `find --workspace --json` scopes to workspace environments only.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn find_workspace_scoping() {
    // Use an empty temp dir as the workspace — should find zero or very few environments
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let temp_path = temp_dir.path().to_str().expect("temp path not valid UTF-8");

    let (json, _) = run_find_json(&["--workspace", temp_path]);

    assert!(
        json["managers"].is_array(),
        "expected 'managers' array in workspace output"
    );
    assert!(
        json["environments"].is_array(),
        "expected 'environments' array in workspace output"
    );

    let scoped_envs = json["environments"].as_array().unwrap();
    for env in scoped_envs {
        // executable may be null for environments without Python installed.
        // No has_executable check: workspace-scoped finds may return zero environments.
        assert!(
            env["kind"].is_string(),
            "workspace environment missing 'kind': {env}"
        );
    }

    // An empty temp dir should yield fewer environments than an unrestricted find
    let (all, _) = run_find_json(&[]);
    let all_envs = all["environments"].as_array().unwrap();
    assert!(
        scoped_envs.len() <= all_envs.len(),
        "workspace-scoped count ({}) should not exceed global count ({})",
        scoped_envs.len(),
        all_envs.len()
    );
}

/// Test 5: CLI flag and env var produce equivalent results for conda executable.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn cli_flag_and_env_var_equivalence() {
    // Use a non-existent path — the point is to verify both delivery mechanisms
    // produce the same Configuration, not that conda actually works.
    let fake_conda = if cfg!(windows) {
        "C:\\nonexistent\\conda.exe"
    } else {
        "/nonexistent/conda"
    };

    // Via CLI flag
    let output_flag = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["find", "--json", "--conda-executable", fake_conda])
        .output()
        .expect("failed to run pet find with --conda-executable");

    // Via env var
    let output_env = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["find", "--json"])
        .env("PET_CONDA_EXECUTABLE", fake_conda)
        .output()
        .expect("failed to run pet find with PET_CONDA_EXECUTABLE");

    assert!(output_flag.status.success());
    assert!(output_env.status.success());

    let json_flag: Value =
        serde_json::from_slice(&output_flag.stdout).expect("flag output is not valid JSON");
    let json_env: Value =
        serde_json::from_slice(&output_env.stdout).expect("env var output is not valid JSON");

    // Both should produce valid output with the same structure
    assert!(json_flag["environments"].is_array());
    assert!(json_env["environments"].is_array());

    // Environment counts should match (same discovery, just different config delivery)
    assert_eq!(
        json_flag["environments"].as_array().unwrap().len(),
        json_env["environments"].as_array().unwrap().len(),
        "CLI flag and env var should produce the same number of environments"
    );
}

/// Test 6: CLI flag takes precedence over env var when both are set.
/// Note: This is a crash-safety test — clap handles flag/env precedence internally,
/// and the effective config isn't exposed in JSON output, so we verify the binary
/// runs successfully when both are provided without conflicting.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn cli_flag_takes_precedence_over_env_var() {
    // Set env var to one value, CLI flag to another.
    // Both are non-existent paths — validates the binary handles both without error.
    let flag_value = if cfg!(windows) {
        "C:\\flag\\conda.exe"
    } else {
        "/flag/conda"
    };
    let env_value = if cfg!(windows) {
        "C:\\envvar\\conda.exe"
    } else {
        "/envvar/conda"
    };

    let output = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["find", "--json", "--conda-executable", flag_value])
        .env("PET_CONDA_EXECUTABLE", env_value)
        .output()
        .expect("failed to run pet find");

    assert!(
        output.status.success(),
        "pet find failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("output is not valid JSON");
    assert!(json["environments"].is_array());
}

/// Test 7: Glob expansion in search paths works for quoted globs.
/// Requires glob expansion support from issue #354.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn find_glob_expansion_in_search_paths() {
    // Create a temp directory structure that matches a glob pattern
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let sub_a = temp_dir.path().join("project_a");
    let sub_b = temp_dir.path().join("project_b");
    std::fs::create_dir_all(&sub_a).unwrap();
    std::fs::create_dir_all(&sub_b).unwrap();

    // Build a glob pattern: <tempdir>/project_*
    let glob_pattern = format!(
        "{}{}project_*",
        temp_dir.path().display(),
        std::path::MAIN_SEPARATOR
    );

    // Run pet find with the glob pattern as a search path — this should not error
    let output = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["find", "--json", &glob_pattern])
        .output()
        .expect("failed to run pet find with glob pattern");

    assert!(
        output.status.success(),
        "pet find with glob pattern failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("output is not valid JSON");
    assert!(
        json["environments"].is_array(),
        "expected valid JSON output with glob search path"
    );
}

/// Test 8: `find --json` with `--environment-directories` via env var.
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn find_environment_directories_via_env_var() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

    let output = Command::new(env!("CARGO_BIN_EXE_pet"))
        .args(["find", "--json"])
        .env(
            "PET_ENVIRONMENT_DIRECTORIES",
            temp_dir.path().to_string_lossy().as_ref(),
        )
        .output()
        .expect("failed to run pet find with PET_ENVIRONMENT_DIRECTORIES");

    assert!(
        output.status.success(),
        "pet find with PET_ENVIRONMENT_DIRECTORIES failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("output is not valid JSON");
    assert!(json["environments"].is_array());
}
