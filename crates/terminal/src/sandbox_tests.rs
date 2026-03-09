//! Integration tests for terminal sandboxing.
//!
//! These tests exercise the real kernel sandbox (Seatbelt on macOS, Landlock on
//! Linux) by spawning child processes and verifying OS enforcement. They do NOT
//! use mocks.

use crate::sandbox_exec::SandboxExecConfig;
use crate::terminal_settings::{ResolvedSystemPaths, SandboxConfig};
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

/// Spawn `/bin/sh -c <shell_command>` in a child process that has the OS-level
/// sandbox applied (Seatbelt on macOS, Landlock on Linux).
///
/// Returns `(success, stdout, stderr)`.
fn run_sandboxed_command(config: &SandboxConfig, shell_command: &str) -> (bool, String, String) {
    let mut config = config.clone();
    config.canonicalize_paths();

    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c").arg(shell_command);
    cmd.current_dir(&config.project_dir);

    unsafe {
        cmd.pre_exec(move || {
            #[cfg(target_os = "macos")]
            {
                crate::sandbox_macos::apply_sandbox(&config)?;
            }
            #[cfg(target_os = "linux")]
            {
                crate::sandbox_linux::apply_sandbox(&config)?;
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

/// Like `run_sandboxed_command`, but also filters environment variables
/// the way `sandbox_exec_main` does: only allowed vars + Zed-specific
/// vars are passed through. Extra env vars are injected into the parent
/// env *before* filtering, so they are subject to the same filter.
fn run_sandboxed_with_env(
    config: &SandboxConfig,
    extra_env: &[(&str, &str)],
    shell_command: &str,
) -> (bool, String, String) {
    let mut config = config.clone();
    config.canonicalize_paths();

    let allowed: HashSet<&str> = config.allowed_env_vars.iter().map(|s| s.as_str()).collect();
    let zed_vars = [
        "ZED_TERM",
        "TERM_PROGRAM",
        "TERM",
        "COLORTERM",
        "TERM_PROGRAM_VERSION",
    ];

    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c").arg(shell_command);
    cmd.current_dir(&config.project_dir);

    // Combine real parent env with extra_env, then filter.
    // extra_env simulates vars that would exist in the parent process.
    let mut combined_env: Vec<(String, String)> = std::env::vars().collect();
    for &(key, value) in extra_env {
        combined_env.push((key.to_string(), value.to_string()));
    }

    cmd.env_clear();
    for (key, value) in &combined_env {
        if allowed.contains(key.as_str()) || zed_vars.contains(&key.as_str()) {
            cmd.env(key, value);
        }
    }

    unsafe {
        cmd.pre_exec(move || {
            #[cfg(target_os = "macos")]
            {
                crate::sandbox_macos::apply_sandbox(&config)?;
            }
            #[cfg(target_os = "linux")]
            {
                crate::sandbox_linux::apply_sandbox(&config)?;
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
    let settings = settings::SandboxSettingsContent::default();
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

    // System paths should use OS-specific defaults
    assert!(!config.system_paths.executable.is_empty());
    assert!(!config.system_paths.read_only.is_empty());
    assert!(!config.system_paths.read_write.is_empty());
}

#[test]
fn test_sandbox_config_tilde_expansion() {
    let home = std::env::var("HOME").expect("HOME not set");
    let settings = settings::SandboxSettingsContent {
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
    let settings = settings::SandboxSettingsContent {
        allowed_env_vars: Some(vec!["CUSTOM_VAR".into()]),
        ..Default::default()
    };
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/tmp/test"));
    assert_eq!(config.allowed_env_vars, vec!["CUSTOM_VAR".to_string()]);
}

#[test]
fn test_sandbox_config_network_disabled() {
    let settings = settings::SandboxSettingsContent {
        allow_network: Some(false),
        ..Default::default()
    };
    let config = SandboxConfig::from_settings(&settings, PathBuf::from("/tmp/test"));
    assert!(!config.allow_network);
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
        let profile = generate_sbpl_profile(&config);
        assert!(profile.contains("(deny default)"));
    }

    #[test]
    fn test_sbpl_profile_has_version() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config);
        assert!(profile.starts_with("(version 1)\n"));
    }

    #[test]
    fn test_sbpl_profile_includes_project_dir() {
        let config = test_sandbox_config(PathBuf::from("/tmp/my-project"));
        let profile = generate_sbpl_profile(&config);
        assert!(
            profile.contains("(subpath \"/tmp/my-project\")"),
            "Profile should include project dir as a subpath rule. Profile:\n{profile}"
        );
    }

    #[test]
    fn test_sbpl_profile_includes_system_paths() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config);
        // At minimum, /usr/bin should be in the executable paths
        assert!(
            profile.contains("(subpath \"/usr/bin\")"),
            "Profile should include /usr/bin. Profile:\n{profile}"
        );
    }

    #[test]
    fn test_sbpl_profile_network_allowed() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config);
        assert!(profile.contains("(allow network-outbound)"));
        assert!(profile.contains("(allow network-inbound)"));
    }

    #[test]
    fn test_sbpl_profile_network_denied() {
        let mut config = test_sandbox_config(PathBuf::from("/tmp/project"));
        config.allow_network = false;
        let profile = generate_sbpl_profile(&config);
        assert!(!profile.contains("(allow network-outbound)"));
        assert!(!profile.contains("(allow network-inbound)"));
    }

    #[test]
    fn test_sbpl_profile_no_unrestricted_process_exec() {
        let config = test_sandbox_config(PathBuf::from("/tmp/project"));
        let profile = generate_sbpl_profile(&config);
        // Should NOT have a bare "(allow process-exec)" without a filter
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
        let profile = generate_sbpl_profile(&config);
        // Should NOT have a bare "(allow mach-lookup)" without a filter
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

        let profile = generate_sbpl_profile(&config);

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
}

// ---------------------------------------------------------------------------
// Integration tests: filesystem enforcement
// ---------------------------------------------------------------------------

/// Create a tempdir and return its canonicalized path.
/// On macOS, /var/folders -> /private/var/folders, so we must use the
/// canonical path for both shell commands and sandbox rules to match.
fn canonical_tempdir() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let canonical = dir.path().canonicalize().expect("failed to canonicalize");
    (dir, canonical)
}

/// Creates a directory with a known file for testing.
/// Returns (dir_path, file_path).
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

    // Sandboxed: rm -rf should be blocked
    let config = test_sandbox_config(project_dir.clone());
    let cmd = format!("rm -rf {}", target_dir.display());
    let (success, _stdout, _stderr) = run_sandboxed_command(&config, &cmd);

    // The rm might "succeed" (exit 0) on some platforms even if individual
    // deletes fail, or it might fail. Either way, the files should still exist.
    assert!(
        target_dir.exists() && target_file.exists(),
        "Sandboxed rm -rf should not be able to delete target directory. \
         success={success}, dir_exists={}, file_exists={}",
        target_dir.exists(),
        target_file.exists(),
    );

    // Unsandboxed: rm -rf should succeed
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
    let cmd = format!("echo 'hello from sandbox' > {}", output_file.display());
    let (success, _stdout, stderr) = run_sandboxed_command(&config, &cmd);

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

    let config = test_sandbox_config(project_dir.clone());

    // Try to cat the secret file and capture stdout
    let cmd = format!("cat {} 2>/dev/null || true", secret_file.display());
    let (_success, stdout, _stderr) = run_sandboxed_command(&config, &cmd);

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

    // First, WITHOUT the extra path — write should fail
    let config_without = test_sandbox_config(project_dir.clone());
    let cmd = format!("echo 'written' > {}", test_file.display());
    let (_success, _stdout, _stderr) = run_sandboxed_command(&config_without, &cmd);
    let file_written_without = test_file.exists()
        && fs::read_to_string(&test_file)
            .map(|c| c.contains("written"))
            .unwrap_or(false);
    assert!(
        !file_written_without,
        "Write to extra dir should be blocked without additional_read_write_paths"
    );

    // Now, WITH the extra path — write should succeed
    let mut config_with = test_sandbox_config(project_dir);
    config_with.additional_read_write_paths = vec![extra_dir.clone()];
    let (success, _stdout, stderr) = run_sandboxed_command(&config_with, &cmd);
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
    config.additional_read_only_paths = vec![readonly_dir.clone()];

    // Read the file into the project dir — should succeed
    let output_file = project_dir.join("read_output.txt");
    let cmd = format!(
        "cat {} > {}",
        readonly_file.display(),
        output_file.display()
    );
    let (success, _stdout, stderr) = run_sandboxed_command(&config, &cmd);
    assert!(
        success,
        "Reading from read-only path should succeed. stderr: {stderr}"
    );
    let read_content = fs::read_to_string(&output_file).unwrap_or_default();
    assert!(
        read_content.contains(known_content),
        "Should have read the known content. Got: {read_content}"
    );

    // Try to overwrite the read-only file — should fail
    let cmd = format!("echo 'overwritten' > {}", readonly_file.display());
    let (_success, _stdout, _stderr) = run_sandboxed_command(&config, &cmd);
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

    // HOME is in the default allowlist; AWS_SECRET is not
    let (success, stdout, stderr) = run_sandboxed_with_env(
        &config,
        &[("AWS_SECRET", "super_secret_key_12345")],
        "echo HOME=$HOME; echo AWS=$AWS_SECRET",
    );
    assert!(success, "env command should succeed. stderr: {stderr}");

    // HOME should be present (it's in the default allowed list)
    assert!(
        stdout.contains("HOME=/"),
        "HOME should be present in filtered env. stdout: {stdout}"
    );

    // AWS_SECRET should be absent (not in the allowed list)
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

    // Try to fetch a URL — should fail due to network being blocked
    let cmd = "curl -s --max-time 5 https://example.com 2>&1 || true";
    let (_success, stdout, _stderr) = run_sandboxed_command(&config, &cmd);

    // The response should NOT contain the expected HTML from example.com
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
    let (success, stdout, stderr) = run_sandboxed_command(&config, "echo 'sandbox works'");

    assert!(
        success,
        "Basic echo should succeed under sandbox. stderr: {stderr}"
    );
    assert!(
        stdout.contains("sandbox works"),
        "Should see echo output. stdout: {stdout}"
    );
}
