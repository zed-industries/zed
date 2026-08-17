// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Tests for Poetry environment identification by path pattern.
//! This test module verifies that Poetry environments are correctly identified
//! even when they are not discovered during the find() phase. This can happen when:
//! - Workspace directories are not configured
//! - The pyproject.toml is not in the workspace directories
//! - The environment is in the Poetry cache but wasn't enumerated
//! - The environment is an in-project .venv with virtualenvs.in-project = true
//!
//! The fix adds fallback path-based detection that checks:
//! 1. If the environment path matches Poetry's cache naming pattern
//!    ({name}-{8-char-hash}-py{version}) in "pypoetry/virtualenvs"
//! 2. If the environment is an in-project .venv with Poetry configuration:
//!    - poetry.toml exists in the parent directory, OR
//!    - pyproject.toml contains [tool.poetry] or poetry-core build backend

use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to test the regex pattern matching for cache environments
    // This tests the core logic without needing actual filesystem structures
    fn test_poetry_cache_path_pattern(path_str: &str) -> bool {
        use regex::Regex;
        let path = PathBuf::from(path_str);
        let path_str = path.to_str().unwrap_or_default();

        if path_str.contains("pypoetry") && path_str.contains("virtualenvs") {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                let re = Regex::new(r"^.+-[A-Za-z0-9_-]{8}-py\d+\.\d+$").unwrap();
                return re.is_match(dir_name);
            }
        }
        false
    }

    // Helper function to test in-project poetry environment detection
    // Requires actual filesystem structure
    fn test_in_project_poetry_env(path: &std::path::Path) -> bool {
        // Check if this is a .venv directory
        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if dir_name != ".venv" {
            return false;
        }

        // Check if the parent directory has Poetry configuration
        if let Some(parent) = path.parent() {
            // Check for poetry.toml - a local Poetry configuration file
            let poetry_toml = parent.join("poetry.toml");
            if poetry_toml.is_file() {
                return true;
            }

            // Check if pyproject.toml contains Poetry configuration
            let pyproject_toml = parent.join("pyproject.toml");
            if pyproject_toml.is_file() {
                if let Ok(contents) = std::fs::read_to_string(&pyproject_toml) {
                    if contents.contains("[tool.poetry]")
                        || contents.contains("poetry.core.masonry.api")
                        || contents.contains("poetry-core")
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[test]
    fn test_poetry_path_pattern_macos() {
        assert!(test_poetry_cache_path_pattern(
            "/Users/eleanorboyd/Library/Caches/pypoetry/virtualenvs/nestedpoetry-yJwtIF_Q-py3.11"
        ));
    }

    #[test]
    fn test_poetry_path_pattern_linux() {
        assert!(test_poetry_cache_path_pattern(
            "/home/user/.cache/pypoetry/virtualenvs/myproject-a1B2c3D4-py3.10"
        ));
    }

    #[test]
    fn test_poetry_path_pattern_windows() {
        assert!(test_poetry_cache_path_pattern(
            r"C:\Users\user\AppData\Local\pypoetry\Cache\virtualenvs\myproject-f7sQRtG5-py3.11"
        ));
    }

    #[test]
    fn test_poetry_path_pattern_no_version_rejected() {
        // Poetry always includes the Python version (major.minor) in the environment name
        // A name ending in just "py" without version should not match
        assert!(!test_poetry_cache_path_pattern(
            "/home/user/.cache/pypoetry/virtualenvs/testproject-XyZ12345-py"
        ));
    }

    #[test]
    fn test_non_poetry_path_rejected() {
        assert!(!test_poetry_cache_path_pattern("/home/user/projects/myenv"));
        assert!(!test_poetry_cache_path_pattern("/home/user/.venv"));
        assert!(!test_poetry_cache_path_pattern("/usr/local/venv"));
    }

    #[test]
    fn test_poetry_path_without_pypoetry_rejected() {
        // Should reject paths that look like the pattern but aren't in pypoetry directory
        assert!(!test_poetry_cache_path_pattern(
            "/home/user/virtualenvs/myproject-a1B2c3D4-py3.10"
        ));
    }

    #[test]
    fn test_poetry_path_wrong_hash_length_rejected() {
        // Hash should be exactly 8 characters
        assert!(!test_poetry_cache_path_pattern(
            "/home/user/.cache/pypoetry/virtualenvs/myproject-a1B2c3D456-py3.10"
        ));
        assert!(!test_poetry_cache_path_pattern(
            "/home/user/.cache/pypoetry/virtualenvs/myproject-a1B2c3-py3.10"
        ));
    }

    #[test]
    fn test_real_world_poetry_paths() {
        // Test actual Poetry paths from the bug report and real usage
        assert!(test_poetry_cache_path_pattern(
            "/Users/eleanorboyd/Library/Caches/pypoetry/virtualenvs/nestedpoetry-yJwtIF_Q-py3.11"
        ));

        // Another real-world example from documentation
        assert!(test_poetry_cache_path_pattern(
            "/Users/donjayamanne/.cache/pypoetry/virtualenvs/poetry-demo-gNT2WXAV-py3.9"
        ));
    }

    // Tests for in-project Poetry environment detection (issue #282)

    #[test]
    fn test_in_project_poetry_env_with_tool_poetry() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let venv_dir = project_dir.join(".venv");

        // Create .venv directory
        fs::create_dir(&venv_dir).unwrap();

        // Create pyproject.toml with [tool.poetry] section
        let pyproject_content = r#"
[tool.poetry]
name = "my-project"
version = "0.1.0"
description = ""
authors = ["Test User <test@example.com>"]

[tool.poetry.dependencies]
python = "^3.10"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#;
        fs::write(project_dir.join("pyproject.toml"), pyproject_content).unwrap();

        // Test that the .venv is recognized as a Poetry environment
        assert!(test_in_project_poetry_env(&venv_dir));
    }

    #[test]
    fn test_in_project_poetry_env_with_poetry_core_backend() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let venv_dir = project_dir.join(".venv");

        // Create .venv directory
        fs::create_dir(&venv_dir).unwrap();

        // Create pyproject.toml with poetry.core.masonry.api as build backend
        let pyproject_content = r#"
[project]
name = "my-project"
version = "0.1.0"

[build-system]
requires = ["poetry-core>=1.0.0"]
build-backend = "poetry.core.masonry.api"
"#;
        fs::write(project_dir.join("pyproject.toml"), pyproject_content).unwrap();

        // Test that the .venv is recognized as a Poetry environment
        assert!(test_in_project_poetry_env(&venv_dir));
    }

    #[test]
    fn test_in_project_non_poetry_env_rejected() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let venv_dir = project_dir.join(".venv");

        // Create .venv directory
        fs::create_dir(&venv_dir).unwrap();

        // Create pyproject.toml without Poetry configuration
        let pyproject_content = r#"
[project]
name = "my-project"
version = "0.1.0"

[build-system]
requires = ["setuptools>=45"]
build-backend = "setuptools.build_meta"
"#;
        fs::write(project_dir.join("pyproject.toml"), pyproject_content).unwrap();

        // Test that the .venv is NOT recognized as a Poetry environment
        assert!(!test_in_project_poetry_env(&venv_dir));
    }

    #[test]
    fn test_in_project_env_no_poetry_config_rejected() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let venv_dir = project_dir.join(".venv");

        // Create .venv directory without any Poetry configuration files
        fs::create_dir(&venv_dir).unwrap();

        // Test that the .venv is NOT recognized as a Poetry environment
        assert!(!test_in_project_poetry_env(&venv_dir));
    }

    #[test]
    fn test_in_project_poetry_env_with_poetry_toml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let venv_dir = project_dir.join(".venv");

        // Create .venv directory
        fs::create_dir(&venv_dir).unwrap();

        // Create poetry.toml with in-project setting (no pyproject.toml with Poetry config)
        let poetry_toml_content = r#"
[virtualenvs]
in-project = true
"#;
        fs::write(project_dir.join("poetry.toml"), poetry_toml_content).unwrap();

        // Create minimal pyproject.toml without Poetry-specific config
        let pyproject_content = r#"
[project]
name = "my-project"
version = "0.1.0"

[build-system]
requires = ["setuptools>=45"]
build-backend = "setuptools.build_meta"
"#;
        fs::write(project_dir.join("pyproject.toml"), pyproject_content).unwrap();

        // Test that the .venv is recognized as a Poetry environment due to poetry.toml
        assert!(test_in_project_poetry_env(&venv_dir));
    }

    #[test]
    fn test_non_venv_directory_rejected() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path();
        let custom_venv = project_dir.join("myenv");

        // Create custom env directory (not named .venv)
        fs::create_dir(&custom_venv).unwrap();

        // Create pyproject.toml with Poetry configuration
        let pyproject_content = r#"
[tool.poetry]
name = "my-project"
version = "0.1.0"
"#;
        fs::write(project_dir.join("pyproject.toml"), pyproject_content).unwrap();

        // Test that non-.venv directories are NOT recognized
        assert!(!test_in_project_poetry_env(&custom_venv));
    }
}
