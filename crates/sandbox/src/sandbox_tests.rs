//! Integration tests for terminal sandboxing.
//!
//! These tests exercise the real kernel sandbox (Seatbelt on macOS, Landlock on
//! Linux) by spawning child processes and verifying OS enforcement. They do NOT
//! use mocks.
//!
//! These tests use `std::process::Command::output()` rather than `smol::process`
//! because they need `pre_exec` hooks to apply sandboxes before exec.
#![allow(clippy::disallowed_methods)]

use crate::{ResolvedSystemPaths, SandboxConfig, SandboxExecConfig};
use std::collections::HashSet;
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Build a minimal `SandboxConfig` for testing.
///
/// Uses default executable and read-only system paths so `/bin/sh` and
/// commands like `echo`, `cat`, `rm`, `env`, and `curl` are available.
///
/// Crucially, the read-write system paths are restricted to `/dev` and
/// `/private/tmp` only — NOT `/private/var/folders`. This is because the
/// test temp directories live under `/private/var/folders`, and granting
/// blanket access there would make it impossible to test that the sandbox
/// blocks access to sibling directories outside the project.
fn test_sandbox_config(project_dir: PathBuf) -> SandboxConfig {
    let defaults = ResolvedSystemPaths::with_defaults();
    SandboxConfig {
        project_dir,
        system_paths: ResolvedSystemPaths {
            executable: defaults.executable,
            read_only: defaults.read_only,
            read_write: vec![
                PathBuf::from("/dev"),
                #[cfg(target_os = "macos")]
                PathBuf::from("/private/tmp"),
                #[cfg(target_os = "linux")]
                PathBuf::from("/tmp"),
                #[cfg(target_os = "linux")]
                PathBuf::from("/var/tmp"),
            ],
        },
        additional_executable_paths: vec![],
        additional_read_only_paths: vec![],
        additional_read_write_paths: vec![],
        allow_network: true,
        allowed_env_vars: SandboxConfig::default_allowed_env_vars(),
    }
}

/// Exercises the full `sandbox_exec_main` production codepath in a child
/// process.
///
/// Returns `(success, stdout, stderr)`.
fn run_sandboxed_command(
    config: &SandboxConfig,
    extra_parent_env: &[(&str, &str)],
    shell_command: &str,
) -> (bool, String, String) {
    let exec_config = SandboxExecConfig::from_sandbox_config(config);
    let config_json = exec_config.to_json();
    let parsed = SandboxExecConfig::from_json(&config_json)
        .expect("SandboxExecConfig JSON roundtrip failed");
    let mut sandbox_config = parsed.to_sandbox_config();
    sandbox_config.canonicalize_paths();

    let zed_vars = [
        "ZED_TERM",
        "TERM_PROGRAM",
        "TERM",
        "COLORTERM",
        "TERM_PROGRAM_VERSION",
    ];
    let allowed: HashSet<&str> = parsed.allowed_env_vars.iter().map(|s| s.as_str()).collect();

    let mut parent_env: Vec<(String, String)> = std::env::vars().collect();
    for &(key, value) in extra_parent_env {
        parent_env.push((key.to_string(), value.to_string()));
    }
    let filtered_env: Vec<(String, String)> = parent_env
        .into_iter()
        .filter(|(key, _)| allowed.contains(key.as_str()) || zed_vars.contains(&key.as_str()))
        .collect();

    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c").arg(shell_command);
    cmd.current_dir(&sandbox_config.project_dir);
    cmd.env_clear();
    cmd.envs(filtered_env);

    unsafe {
        cmd.pre_exec(move || {
            #[cfg(target_os = "macos")]
            {
                crate::sandbox_macos::apply_sandbox(&sandbox_config)?;
            }
            #[cfg(target_os = "linux")]
            {
                crate::sandbox_linux::apply_sandbox(&sandbox_config)?;
            }
            Ok(())
        });
    }

    let output = cmd
        .output()
        .expect("failed to spawn sandboxed child process");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Run a shell command *without* any sandbox for comparison.
fn run_unsandboxed_command(shell_command: &str) -> (bool, String, String) {
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(shell_command)
        .output()
        .expect("failed to spawn unsandboxed child process");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

// ---------------------------------------------------------------------------
// Unit tests: SandboxExecConfig serialization roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_sandbox_exec_config_roundtrip() {
    let original = SandboxConfig {
        project_dir: PathBuf::from("/tmp/my-project"),
        system_paths: ResolvedSystemPaths {
            executable: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
            read_only: vec![PathBuf::from("/etc")],
            read_write: vec![PathBuf::from("/tmp")],
        },
        additional_executable_paths: vec![PathBuf::from("/opt/tools/bin")],
        additional_read_only_paths: vec![PathBuf::from("/opt/data")],
        additional_read_write_paths: vec![PathBuf::from("/opt/cache")],
        allow_network: false,
        allowed_env_vars: vec!["PATH".into(), "HOME".into()],
    };

    let exec_config = SandboxExecConfig::from_sandbox_config(&original);
    let json = exec_config.to_json();
    let deserialized = SandboxExecConfig::from_json(&json).expect("failed to parse JSON");
    let roundtripped = deserialized.to_sandbox_config();

    assert_eq!(roundtripped.project_dir, original.project_dir);
    assert_eq!(
        roundtripped.system_paths.executable,
        original.system_paths.executable
    );
    assert_eq!(
        roundtripped.system_paths.read_only,
        original.system_paths.read_only
    );
    assert_eq!(
        roundtripped.system_paths.read_write,
        original.system_paths.read_write
    );
    assert_eq!(
        roundtripped.additional_executable_paths,
        original.additional_executable_paths
    );
    assert_eq!(
        roundtripped.additional_read_only_paths,
        original.additional_read_only_paths
    );
    assert_eq!(
        roundtripped.additional_read_write_paths,
        original.additional_read_write_paths
    );
    assert_eq!(roundtripped.allow_network, original.allow_network);
    assert_eq!(roundtripped.allowed_env_vars, original.allowed_env_vars);
}

#[test]
fn test_sandbox_exec_config_from_json_invalid() {
    let result = SandboxExecConfig::from_json("not json");
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Unit tests: SandboxConfig::from_settings
// ---------------------------------------------------------------------------

#[test]
fn test_sandbox_config_from_settings_defaults() {
    let settings = settings_content::SandboxSettingsContent::default();
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/projects/test"));

    assert_eq!(config.project_dir, PathBuf::from("/projects/test"));
    assert_eq!(config.allow_network, true);
    assert_eq!(
        config.allowed_env_vars,
        SandboxConfig::default_allowed_env_vars()
    );
    assert!(config.additional_executable_paths.is_empty());
    assert!(config.additional_read_only_paths.is_empty());
    assert!(config.additional_read_write_paths.is_empty());

    assert!(!config.system_paths.executable.is_empty());
    assert!(!config.system_paths.read_only.is_empty());
    assert!(!config.system_paths.read_write.is_empty());
}

#[test]
fn test_sandbox_config_tilde_expansion() {
    let home = std::env::var("HOME").expect("HOME not set");
    let settings = settings_content::SandboxSettingsContent {
        additional_read_only_paths: Some(vec!["~/documents".into(), "/absolute/path".into()]),
        ..Default::default()
    };
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/tmp/test"));

    assert_eq!(
        config.additional_read_only_paths,
        vec![
            PathBuf::from(format!("{}/documents", home)),
            PathBuf::from("/absolute/path"),
        ]
    );
}

#[test]
fn test_sandbox_config_custom_allowed_env_vars() {
    let settings = settings_content::SandboxSettingsContent {
        allowed_env_vars: Some(vec!["CUSTOM_VAR".into()]),
        ..Default::default()
    };
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/tmp/test"));
    assert_eq!(config.allowed_env_vars, vec!["CUSTOM_VAR".to_string()]);
}

#[test]
fn test_sandbox_config_network_disabled() {
    let settings = settings_content::SandboxSettingsContent {
        allow_network: Some(false),
        ..Default::default()
    };
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/tmp/test"));
    assert!(!config.allow_network);
}

// ---------------------------------------------------------------------------
// Unit tests: SandboxConfig::resolve_if_enabled
// ---------------------------------------------------------------------------

#[test]
fn test_resolve_if_enabled_disabled() {
    let settings = settings_content::SandboxSettingsContent {
        enabled: Some(false),
        ..Default::default()
    };
    assert!(
        SandboxConfig::resolve_if_enabled(
            &settings,
            settings_content::SandboxApplyTo::Terminal,
            PathBuf::from("/tmp/test"),
        )
        .is_none()
    );
}

#[test]
fn test_resolve_if_enabled_terminal_matches_terminal() {
    let settings = settings_content::SandboxSettingsContent {
        enabled: Some(true),
        apply_to: Some(settings_content::SandboxApplyTo::Terminal),
        ..Default::default()
    };
    assert!(
        SandboxConfig::resolve_if_enabled(
            &settings,
            settings_content::SandboxApplyTo::Terminal,
            PathBuf::from("/tmp/test"),
        )
        .is_some()
    );
}

#[test]
fn test_resolve_if_enabled_terminal_does_not_match_tool() {
    let settings = settings_content::SandboxSettingsContent {
        enabled: Some(true),
        apply_to: Some(settings_content::SandboxApplyTo::Terminal),
        ..Default::default()
    };
    assert!(
        SandboxConfig::resolve_if_enabled(
            &settings,
            settings_content::SandboxApplyTo::Tool,
            PathBuf::from("/tmp/test"),
        )
        .is_none()
    );
}

#[test]
fn test_resolve_if_enabled_both_matches_both_targets() {
    let settings = settings_content::SandboxSettingsContent {
        enabled: Some(true),
        apply_to: Some(settings_content::SandboxApplyTo::Both),
        ..Default::default()
    };
    assert!(
        SandboxConfig::resolve_if_enabled(
            &settings,
            settings_content::SandboxApplyTo::Terminal,
            PathBuf::from("/tmp/test"),
        )
        .is_some()
    );
    assert!(
        SandboxConfig::resolve_if_enabled(
            &settings,
            settings_content::SandboxApplyTo::Tool,
            PathBuf::from("/tmp/test"),
        )
        .is_some()
    );
}

// ---------------------------------------------------------------------------
// Unit tests: macOS SBPL profile generation
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod sbpl_tests {
    use super::*;
    use crate::sandbox_macos::{generate_sbpl_profile, sbpl_escape};

    #[test]
    fn test_sbpl_escape_plain_path() {
        let path = Path::new("/usr/bin");
        assert_eq!(sbpl_escape(path), "/usr/bin");
    }

    #[test]
    fn test_sbpl_escape_with_quotes() {
        let path = Path::new("/tmp/has\"quote");
        assert_eq!(sbpl_escape(path), "/tmp/has\\\"quote");
    }

    #[test]
    fn test_sbpl_escape_with_backslash() {
        let path = Path::new("/tmp/has\\backslash");
        assert_eq!(sbpl_escape(path), "/tmp/has\\\\backslash");
    }

    #[test]
    fn test_sbpl_escape_with_both() {
        let path = Path::new("/tmp/a\"b\\c");
        assert_eq!(sbpl_escape(path), "/tmp/a\\\"b\\\\c");
    }

    #[test]
    fn test_sbpl_profile_has_deny_default() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(profile.contains("(deny default)"));
    }

    #[test]
    fn test_sbpl_profile_has_version() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(profile.starts_with("(version 1)\n"));
    }

    #[test]
    fn test_sbpl_profile_includes_project_dir() {
        let config = test_sandbox_config(PathBuf::from("/tmp/my-project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(
            profile.contains("(subpath \"/tmp/my-project\")"),
            "Profile should include project dir as a subpath rule. Profile:\n{profile}"
        );
    }

    #[test]
    fn test_sbpl_profile_includes_system_paths() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(
            profile.contains("(subpath \"/usr/bin\")"),
            "Profile should include /usr/bin. Profile:\n{profile}"
        );
    }

    #[test]
    fn test_sbpl_profile_network_allowed() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(profile.contains("(allow network-outbound)"));
        assert!(profile.contains("(allow network-inbound)"));
    }

    #[test]
    fn test_sbpl_profile_network_denied() {
        let mut config = test_sandbox_config(PathBuf::from("/tmp/project"));
        config.allow_network = false;
        let profile = generate_sbpl_profile(&config, None);
        assert!(!profile.contains("(allow network-outbound)"));
        assert!(!profile.contains("(allow network-inbound)"));
    }

    #[test]
    fn test_sbpl_profile_no_unrestricted_process_exec() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        let lines: Vec<&str> = profile.lines().collect();
        for line in &lines {
            if line.contains("process-exec") {
                assert!(
                    line.contains("subpath") || line.contains("literal"),
                    "process-exec should be scoped to specific paths, found bare rule: {line}"
                );
            }
        }
    }

    #[test]
    fn test_sbpl_profile_no_unrestricted_mach_lookup() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        let lines: Vec<&str> = profile.lines().collect();
        for line in &lines {
            if line.contains("mach-lookup") {
                assert!(
                    line.contains("global-name"),
                    "mach-lookup should be scoped to specific services, found: {line}"
                );
            }
        }
    }

    #[test]
    fn test_sbpl_profile_additional_paths() {
        let mut config = test_sandbox_config(PathBuf::from("/tmp/project"));
        config.additional_executable_paths = vec![PathBuf::from("/opt/tools/bin")];
        config.additional_read_only_paths = vec![PathBuf::from("/opt/data")];
        config.additional_read_write_paths = vec![PathBuf::from("/opt/cache")];

        let profile = generate_sbpl_profile(&config, None);

        assert!(
            profile.contains("(subpath \"/opt/tools/bin\")"),
            "Should include additional executable path"
        );
        assert!(
            profile.contains("(subpath \"/opt/data\")"),
            "Should include additional read-only path"
        );
        assert!(
            profile.contains("(subpath \"/opt/cache\")"),
            "Should include additional read-write path"
        );
    }

    #[test]
    fn test_sbpl_profile_signal_scoped_to_children() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config, None);
        assert!(
            profile.contains("(allow signal (target children))"),
            "Signal should be scoped to children. Profile:\n{profile}"
        );
        let lines: Vec<&str> = profile.lines().collect();
        for line in &lines {
            if line.contains("(allow signal") {
                assert!(
                    line.contains("(target children)"),
                    "Found unscoped signal rule: {line}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Integration tests: filesystem enforcement
// ---------------------------------------------------------------------------

fn canonical_tempdir() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let canonical = dir.path().canonicalize().expect("failed to canonicalize");
    (dir, canonical)
}

fn create_test_directory(base: &Path, name: &str, content: &str) -> (PathBuf, PathBuf) {
    let dir = base.join(name);
    fs::create_dir_all(&dir).expect("failed to create test directory");
    let file = dir.join("test_file.txt");
    fs::write(&file, content).expect("failed to write test file");
    (dir, file)
}

#[test]
fn test_sandbox_blocks_rm_rf() {
    let (_base_guard, base) = canonical_tempdir();

    let (project_dir, _) = create_test_directory(&base, "project", "project content");
    let (target_dir, target_file) = create_test_directory(&base, "target", "do not delete me");

    let config = test_sandbox_config(project_dir);
    let cmd = format!("rm -rf {}", target_dir.display());
    let (success, _stdout, _stderr) = run_sandboxed_command(&config, &[], &cmd);

    assert!(
        target_dir.exists() && target_file.exists(),
        "Sandboxed rm -rf should not be able to delete target directory. \
         success={success}, dir_exists={}, file_exists={}",
        target_dir.exists(),
        target_file.exists(),
    );

    let (success, _, _) = run_unsandboxed_command(&format!("rm -rf {}", target_dir.display()));
    assert!(success, "Unsandboxed rm -rf should succeed");
    assert!(
        !target_dir.exists(),
        "Unsandboxed rm -rf should have deleted the directory"
    );
}

#[test]
fn test_sandbox_allows_writes_in_project() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let config = test_sandbox_config(project_dir.clone());
    let output_file = project_dir.join("sandbox_output.txt");
    #[allow(clippy::redundant_clone)]
    let cmd = format!("echo 'hello from sandbox' > {}", output_file.display());
    let (success, _stdout, stderr) = run_sandboxed_command(&config, &[], &cmd);

    assert!(
        success,
        "Writing inside the project dir should succeed. stderr: {stderr}"
    );
    assert!(output_file.exists(), "Output file should have been created");
    let content = fs::read_to_string(&output_file).expect("failed to read output file");
    assert!(
        content.contains("hello from sandbox"),
        "File should contain expected content, got: {content}"
    );
}

#[test]
fn test_sandbox_blocks_reads_outside_project() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let secret_content = "TOP_SECRET_DATA_12345";
    let (_, secret_file) = create_test_directory(&base, "secrets", secret_content);

    let config = test_sandbox_config(project_dir);

    let cmd = format!("cat {} 2>/dev/null || true", secret_file.display());
    let (_success, stdout, _stderr) = run_sandboxed_command(&config, &[], &cmd);

    assert!(
        !stdout.contains(secret_content),
        "Sandbox should prevent reading files outside the project. stdout: {stdout}"
    );
}

#[test]
fn test_additional_read_write_paths_grant_access() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let extra_dir = base.join("extra_rw");
    fs::create_dir_all(&extra_dir).expect("failed to create extra dir");

    let test_file = extra_dir.join("rw_test.txt");

    let config_without = test_sandbox_config(project_dir.clone());
    let cmd = format!("echo 'written' > {}", test_file.display());
    let (_success, _stdout, _stderr) = run_sandboxed_command(&config_without, &[], &cmd);
    let file_written_without = test_file.exists()
        && fs::read_to_string(&test_file)
            .map(|c| c.contains("written"))
            .unwrap_or(false);
    assert!(
        !file_written_without,
        "Write to extra dir should be blocked without additional_read_write_paths"
    );

    let mut config_with = test_sandbox_config(project_dir);
    config_with.additional_read_write_paths = vec![extra_dir];
    let (success, _stdout, stderr) = run_sandboxed_command(&config_with, &[], &cmd);
    assert!(
        success,
        "Write to extra dir should succeed with additional_read_write_paths. stderr: {stderr}"
    );
    assert!(
        test_file.exists(),
        "File should exist after sandboxed write with additional path"
    );
}

#[test]
fn test_additional_read_only_paths_allow_read_block_write() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let known_content = "known_readonly_content";
    let (readonly_dir, readonly_file) =
        create_test_directory(&base, "readonly_data", known_content);

    let mut config = test_sandbox_config(project_dir.clone());
    config.additional_read_only_paths = vec![readonly_dir];

    let output_file = project_dir.join("read_output.txt");
    let cmd = format!(
        "cat {} > {}",
        readonly_file.display(),
        output_file.display()
    );
    let (success, _stdout, stderr) = run_sandboxed_command(&config, &[], &cmd);
    assert!(
        success,
        "Reading from read-only path should succeed. stderr: {stderr}"
    );
    let read_content = fs::read_to_string(&output_file).unwrap_or_default();
    assert!(
        read_content.contains(known_content),
        "Should have read the known content. Got: {read_content}"
    );

    let cmd = format!("echo 'overwritten' > {}", readonly_file.display());
    let (_success, _stdout, _stderr) = run_sandboxed_command(&config, &[], &cmd);
    let current_content = fs::read_to_string(&readonly_file).expect("file should still exist");
    assert_eq!(
        current_content, known_content,
        "Read-only file should not have been overwritten"
    );
}

// ---------------------------------------------------------------------------
// Integration test: environment variable filtering
// ---------------------------------------------------------------------------

#[test]
fn test_env_var_filtering() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let config = test_sandbox_config(project_dir);

    let (success, stdout, stderr) = run_sandboxed_command(
        &config,
        &[("AWS_SECRET", "super_secret_key_12345")],
        "echo HOME=$HOME; echo AWS=$AWS_SECRET",
    );
    assert!(success, "env command should succeed. stderr: {stderr}");

    assert!(
        stdout.contains("HOME=/"),
        "HOME should be present in filtered env. stdout: {stdout}"
    );

    assert!(
        !stdout.contains("super_secret_key_12345"),
        "AWS_SECRET should be filtered out. stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Integration test: network blocking (macOS only)
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
#[test]
fn test_network_blocking() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let mut config = test_sandbox_config(project_dir);
    config.allow_network = false;

    let cmd = "curl -s --max-time 5 https://example.com 2>&1 || true";
    let (_success, stdout, _stderr) = run_sandboxed_command(&config, &[], &cmd);

    assert!(
        !stdout.contains("Example Domain"),
        "Network should be blocked. Got stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Integration test: basic command succeeds under sandbox
// ---------------------------------------------------------------------------

#[test]
fn test_sandbox_basic_echo_succeeds() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let config = test_sandbox_config(project_dir);
    let (success, stdout, stderr) = run_sandboxed_command(&config, &[], "echo 'sandbox works'");

    assert!(
        success,
        "Basic echo should succeed under sandbox. stderr: {stderr}"
    );
    assert!(
        stdout.contains("sandbox works"),
        "Should see echo output. stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Integration test: additional_executable_paths
// ---------------------------------------------------------------------------

#[test]
fn test_additional_executable_paths_allow_execution() {
    let (_base_guard, base) = canonical_tempdir();
    let project_dir = base.join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let tools_dir = base.join("tools");
    fs::create_dir_all(&tools_dir).expect("failed to create tools dir");

    // Create a simple executable script in the tools directory
    let script_path = tools_dir.join("my_tool");
    fs::write(&script_path, "#!/bin/sh\necho tool_executed_successfully\n")
        .expect("failed to write script");

    // Make it executable
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))
        .expect("failed to set permissions");

    // Without additional_executable_paths — execution should fail
    let config_without = test_sandbox_config(project_dir.clone());
    let cmd = format!("{} 2>&1 || true", script_path.display());
    let (_success, stdout_without, _stderr) = run_sandboxed_command(&config_without, &[], &cmd);
    assert!(
        !stdout_without.contains("tool_executed_successfully"),
        "Tool should NOT be executable without additional_executable_paths. stdout: {stdout_without}"
    );

    // With additional_executable_paths — execution should succeed
    let mut config_with = test_sandbox_config(project_dir);
    config_with.additional_executable_paths = vec![tools_dir];
    let (success, stdout_with, stderr) = run_sandboxed_command(&config_with, &[], &cmd);
    assert!(
        success && stdout_with.contains("tool_executed_successfully"),
        "Tool should be executable with additional_executable_paths. success={success}, stdout: {stdout_with}, stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Integration test: canonicalize_paths with symlinks
// ---------------------------------------------------------------------------

#[test]
fn test_canonicalize_paths_resolves_symlinks() {
    let (_base_guard, base) = canonical_tempdir();
    let real_project_dir = base.join("real_project");
    fs::create_dir_all(&real_project_dir).expect("failed to create project dir");

    // Create a test file in the real project directory
    let test_file = real_project_dir.join("test.txt");
    fs::write(&test_file, "symlink_test_content").expect("failed to write test file");

    // Create a symlink to the project directory
    let symlink_dir = base.join("symlink_project");
    std::os::unix::fs::symlink(&real_project_dir, &symlink_dir)
        .expect("failed to create symlink");

    // Use the symlinked path as the project dir — canonicalize_paths should resolve it
    let config = test_sandbox_config(symlink_dir);

    // Writing should work because canonicalize_paths resolves the symlink to the real path
    let output_file = real_project_dir.join("output.txt");
    let cmd = format!("echo 'from_symlinked_project' > {}", output_file.display());
    let (success, _stdout, stderr) = run_sandboxed_command(&config, &[], &cmd);

    assert!(
        success,
        "Writing in symlinked project dir should succeed after canonicalization. stderr: {stderr}"
    );
    let content = fs::read_to_string(&output_file).unwrap_or_default();
    assert!(
        content.contains("from_symlinked_project"),
        "Should have written through the canonicalized path. content: {content}"
    );
}

// ---------------------------------------------------------------------------
// Fingerprint tests (macOS)
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod fingerprint_tests {
    use super::*;
    use crate::sandbox_macos::{
        SessionFingerprint, apply_fingerprint_only, apply_sandbox_with_fingerprint,
        generate_fingerprint_only_profile,
    };

    #[test]
    fn test_fingerprint_matches_own_process_with_full_sandbox() {
        let (_base_guard, base) = canonical_tempdir();
        let project_dir = base.join("project");
        fs::create_dir_all(&project_dir).expect("failed to create project dir");

        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");
        let config = test_sandbox_config(project_dir);

        // Spawn a child process with the fingerprint-embedded sandbox profile
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg("sleep 5");

        let sandbox_config = {
            let exec_config = SandboxExecConfig::from_sandbox_config(&config);
            let parsed = SandboxExecConfig::from_json(&exec_config.to_json()).unwrap();
            let mut sc = parsed.to_sandbox_config();
            sc.canonicalize_paths();
            sc
        };

        unsafe {
            let fp_uuid = fingerprint.uuid_string();
            cmd.pre_exec(move || {
                let fp = SessionFingerprint::from_uuid_str(&fp_uuid)
                    .map_err(|e| std::io::Error::other(e))?;
                apply_sandbox_with_fingerprint(&sandbox_config, &fp)?;
                Ok(())
            });
        }

        let mut child = cmd.spawn().expect("failed to spawn child");
        std::thread::sleep(std::time::Duration::from_millis(100));

        // The fingerprint should match the child process
        let child_pid = child.id() as libc::pid_t;
        assert!(
            fingerprint.matches_pid(child_pid),
            "Fingerprint should match child process with embedded profile"
        );

        child.kill().ok();
        child.wait().ok();
    }

    #[test]
    fn test_fingerprint_does_not_match_unsandboxed_process() {
        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");

        // Spawn an unsandboxed process
        let mut child = Command::new("/bin/sh")
            .arg("-c")
            .arg("sleep 5")
            .spawn()
            .expect("failed to spawn child");

        std::thread::sleep(std::time::Duration::from_millis(100));

        let child_pid = child.id() as libc::pid_t;
        assert!(
            !fingerprint.matches_pid(child_pid),
            "Fingerprint should NOT match unsandboxed process"
        );

        child.kill().ok();
        child.wait().ok();
    }

    #[test]
    fn test_fingerprint_does_not_match_different_session() {
        let (_base_guard, base) = canonical_tempdir();
        let project_dir = base.join("project");
        fs::create_dir_all(&project_dir).expect("failed to create project dir");

        let fingerprint_a = SessionFingerprint::new().expect("failed to create fingerprint A");
        let fingerprint_b = SessionFingerprint::new().expect("failed to create fingerprint B");

        // Spawn a process with fingerprint_b's profile
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg("sleep 5");

        unsafe {
            let fp_b_uuid = fingerprint_b.uuid_string();
            cmd.pre_exec(move || {
                let fp = SessionFingerprint::from_uuid_str(&fp_b_uuid)
                    .map_err(|e| std::io::Error::other(e))?;
                apply_fingerprint_only(&fp)?;
                Ok(())
            });
        }

        let mut child = cmd.spawn().expect("failed to spawn child");
        std::thread::sleep(std::time::Duration::from_millis(100));

        let child_pid = child.id() as libc::pid_t;

        // fingerprint_a should NOT match (wrong session)
        assert!(
            !fingerprint_a.matches_pid(child_pid),
            "Fingerprint A should NOT match process from session B"
        );

        // fingerprint_b SHOULD match
        assert!(
            fingerprint_b.matches_pid(child_pid),
            "Fingerprint B should match its own process"
        );

        child.kill().ok();
        child.wait().ok();
    }

    #[test]
    fn test_fingerprint_only_mode_no_restrictions() {
        let (_base_guard, base) = canonical_tempdir();
        let project_dir = base.join("project");
        fs::create_dir_all(&project_dir).expect("failed to create project dir");

        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");

        // Create a file OUTSIDE the project dir
        let external_dir = base.join("external");
        fs::create_dir_all(&external_dir).expect("failed to create external dir");
        let external_file = external_dir.join("readable.txt");
        fs::write(&external_file, "external_content").expect("failed to write");

        // Spawn with fingerprint-only mode (should allow everything)
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c")
            .arg(format!("cat {}", external_file.display()));

        unsafe {
            let fp_uuid = fingerprint.uuid_string();
            cmd.pre_exec(move || {
                let fp = SessionFingerprint::from_uuid_str(&fp_uuid)
                    .map_err(|e| std::io::Error::other(e))?;
                apply_fingerprint_only(&fp)?;
                Ok(())
            });
        }

        let output = cmd.output().expect("failed to spawn");
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            stdout.contains("external_content"),
            "Fingerprint-only mode should NOT restrict file access. stdout: {stdout}"
        );
    }

    #[test]
    fn test_fingerprint_only_profile_structure() {
        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");
        let profile = generate_fingerprint_only_profile(&fingerprint);

        assert!(profile.contains("(allow default)"), "Should allow everything by default");
        assert!(profile.contains("(deny file-read*"), "Should deny the deny-side path");
        assert!(profile.contains("(allow file-read*"), "Should allow the allow-side path");
        assert!(!profile.contains("(deny default)"), "Should NOT have deny default");
    }
}

// ---------------------------------------------------------------------------
// Convergent cleanup tests (macOS)
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod cleanup_tests {
    use super::*;
    use crate::sandbox_macos::SessionFingerprint;

    /// Helper: spawn a child process with the fingerprint-only profile.
    fn spawn_fingerprinted_process(
        fingerprint: &SessionFingerprint,
        command: &str,
    ) -> std::process::Child {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg(command);

        let fp_uuid = fingerprint.uuid_string();
        unsafe {
            cmd.pre_exec(move || {
                let fp = SessionFingerprint::from_uuid_str(&fp_uuid)
                    .map_err(|e| std::io::Error::other(e))?;
                crate::sandbox_macos::apply_fingerprint_only(&fp)?;
                Ok(())
            });
        }

        cmd.spawn().expect("failed to spawn fingerprinted child")
    }

    #[test]
    fn test_cleanup_kills_simple_child() {
        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");
        let mut child = spawn_fingerprinted_process(&fingerprint, "sleep 60");
        std::thread::sleep(std::time::Duration::from_millis(100));

        let child_pid = child.id() as libc::pid_t;
        assert!(fingerprint.matches_pid(child_pid), "Child should match before cleanup");

        fingerprint.kill_all_processes(None);

        // The child should be dead now
        let status = child.wait().expect("failed to wait");
        assert!(!status.success(), "Child should have been killed");
    }

    #[test]
    fn test_cleanup_loop_terminates() {
        let fingerprint = SessionFingerprint::new().expect("failed to create fingerprint");
        let mut child = spawn_fingerprinted_process(&fingerprint, "sleep 60");
        std::thread::sleep(std::time::Duration::from_millis(100));

        // kill_all_processes should complete (not hang)
        let start = std::time::Instant::now();
        fingerprint.kill_all_processes(None);
        let elapsed = start.elapsed();

        assert!(
            elapsed < std::time::Duration::from_secs(5),
            "Cleanup should complete quickly, took {elapsed:?}"
        );

        child.wait().ok();
    }
}
