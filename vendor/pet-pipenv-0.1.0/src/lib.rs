// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_variables::EnvVariables;
use lazy_static::lazy_static;
use log::trace;
use manager::PipenvManager;
use pet_core::env::PythonEnv;
use pet_core::os_environment::Environment;
use pet_core::LocatorKind;
use pet_core::{
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Configuration, Locator, RefreshStatePersistence,
};
use pet_fs::path::norm_case;
use pet_python_utils::executable::find_executables;
use pet_python_utils::version;
use regex::Regex;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::{fs, path::PathBuf};

mod env_variables;
pub mod manager;

lazy_static! {
    /// Regex pattern for pipenv environment directory names.
    /// Pipenv uses the naming convention: `{sanitized-project-name}-{8-char-hash}`
    /// The hash is 8 characters of URL-safe base64 encoding of SHA256.
    /// Pattern: one or more name segments (letters, digits, underscores) separated by hyphens,
    /// followed by a hyphen and exactly 8 alphanumeric characters (including _ and -).
    static ref PIPENV_ENV_NAME_PATTERN: Regex = Regex::new(r"^.+-[A-Za-z0-9_-]{8}$")
        .expect("Error creating pipenv environment name pattern regex");
}

/// Returns the list of directories where pipenv stores centralized virtual environments.
/// These are the known locations where pipenv creates virtualenvs when not using in-project mode.
/// See: https://github.com/pypa/pipenv/blob/main/pipenv/utils/shell.py#L184
fn get_pipenv_virtualenv_dirs(env_vars: &EnvVariables) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![];

    // WORKON_HOME can be used by pipenv as well
    if let Some(workon_home) = &env_vars.workon_home {
        if workon_home.exists() {
            trace!("Pipenv: Found WORKON_HOME directory: {:?}", workon_home);
            dirs.push(norm_case(workon_home));
        }
    }

    // XDG_DATA_HOME/virtualenvs (common on Linux)
    if let Some(xdg_data_home) = &env_vars.xdg_data_home {
        let xdg_venvs = PathBuf::from(xdg_data_home).join("virtualenvs");
        if xdg_venvs.exists() {
            trace!("Pipenv: Found XDG_DATA_HOME/virtualenvs: {:?}", xdg_venvs);
            dirs.push(norm_case(xdg_venvs));
        }
    }

    if let Some(home) = &env_vars.home {
        // ~/.local/share/virtualenvs - default pipenv location on macOS/Linux
        let local_share_venvs = home.join(".local").join("share").join("virtualenvs");
        if local_share_venvs.exists() {
            trace!(
                "Pipenv: Found ~/.local/share/virtualenvs: {:?}",
                local_share_venvs
            );
            dirs.push(norm_case(local_share_venvs));
        }

        // ~/.venvs - alternative pipenv location
        let dot_venvs = home.join(".venvs");
        if dot_venvs.exists() {
            trace!("Pipenv: Found ~/.venvs: {:?}", dot_venvs);
            dirs.push(norm_case(dot_venvs));
        }

        // ~/.virtualenvs - can also be used by pipenv
        let dot_virtualenvs = home.join(".virtualenvs");
        if dot_virtualenvs.exists() {
            trace!("Pipenv: Found ~/.virtualenvs: {:?}", dot_virtualenvs);
            dirs.push(norm_case(dot_virtualenvs));
        }
    }

    trace!("Pipenv: Centralized virtualenv directories: {:?}", dirs);
    dirs
}

/// Checks if the given environment is in one of pipenv's centralized virtualenv directories.
/// Pipenv uses a specific naming convention: <project-name>-<hash>
fn is_in_pipenv_centralized_dir(env: &PythonEnv, env_vars: &EnvVariables) -> bool {
    let prefix = match &env.prefix {
        Some(p) => p,
        None => {
            // Try to derive prefix from executable path
            if let Some(bin) = env.executable.parent() {
                if bin.file_name().unwrap_or_default() == Path::new("bin")
                    || bin.file_name().unwrap_or_default() == Path::new("Scripts")
                {
                    if let Some(p) = bin.parent() {
                        p
                    } else {
                        trace!(
                            "Pipenv: Cannot derive prefix from executable {:?}",
                            env.executable
                        );
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    };

    let pipenv_dirs = get_pipenv_virtualenv_dirs(env_vars);
    for pipenv_dir in &pipenv_dirs {
        if let Some(parent) = prefix.parent() {
            if norm_case(parent) == *pipenv_dir {
                // Primary check: .project file (pipenv always creates this for centralized envs)
                let project_file = prefix.join(".project");
                if project_file.exists() {
                    trace!(
                        "Pipenv: Detected centralized pipenv env at {:?} (in {:?}, has .project file)",
                        prefix,
                        pipenv_dir
                    );
                    return true;
                }

                // Fallback: Check if directory name matches pipenv naming pattern
                // Pattern: {sanitized-project-name}-{8-char-hash}
                // This handles edge cases where .project was deleted, corrupted,
                // or environments from older pipenv versions.
                if let Some(dir_name) = prefix.file_name().and_then(|n| n.to_str()) {
                    if PIPENV_ENV_NAME_PATTERN.is_match(dir_name) {
                        trace!(
                            "Pipenv: Detected centralized pipenv env at {:?} (in {:?}, matched naming pattern, no .project file)",
                            prefix,
                            pipenv_dir
                        );
                        return true;
                    } else {
                        trace!(
                            "Pipenv: Env {:?} is in pipenv dir {:?} but missing .project file and name doesn't match pattern",
                            prefix,
                            pipenv_dir
                        );
                    }
                }
            }
        }
    }

    trace!(
        "Pipenv: Env {:?} is not in any centralized pipenv directory",
        prefix
    );
    false
}

fn get_pipenv_project(env: &PythonEnv) -> Option<PathBuf> {
    if let Some(prefix) = &env.prefix {
        if let Some(project) = get_pipenv_project_from_prefix(prefix) {
            return Some(project);
        }
        // If there's no .project file, but the venv lives inside the project folder
        // (e.g., <project>/.venv or <project>/venv), then the project is the parent
        // directory of the venv. Detect that by checking for a Pipfile next to the venv.
        if let Some(parent) = prefix.parent() {
            let project_folder = parent;
            if project_folder.join("Pipfile").exists() {
                return Some(project_folder.to_path_buf());
            }
        }
    }

    // We can also have a venv in the workspace that has pipenv installed in it.
    // In such cases, the project is the workspace folder containing the venv.
    // Derive the project folder from the executable path when prefix isn't available.
    // Typical layout: <project>/.venv/{bin|Scripts}/python
    // So walk up to {bin|Scripts} -> venv dir -> project dir and check for Pipfile.
    if let Some(bin) = env.executable.parent() {
        let venv_dir = if bin.file_name().unwrap_or_default() == Path::new("bin")
            || bin.file_name().unwrap_or_default() == Path::new("Scripts")
        {
            bin.parent()
        } else {
            Some(bin)
        };
        if let Some(venv_dir) = venv_dir {
            if let Some(project_dir) = venv_dir.parent() {
                if project_dir.join("Pipfile").exists() {
                    return Some(project_dir.to_path_buf());
                }
            }
        }
    }

    // If the parent is bin or script, then get the parent.
    let bin = env.executable.parent()?;
    if bin.file_name().unwrap_or_default() == Path::new("bin")
        || bin.file_name().unwrap_or_default() == Path::new("Scripts")
    {
        get_pipenv_project_from_prefix(env.executable.parent()?.parent()?)
    } else {
        get_pipenv_project_from_prefix(env.executable.parent()?)
    }
}

fn get_pipenv_project_from_prefix(prefix: &Path) -> Option<PathBuf> {
    let project_file = prefix.join(".project");
    if !project_file.exists() {
        return None;
    }
    let contents = fs::read_to_string(project_file).ok()?;
    let project_folder = norm_case(PathBuf::from(contents.trim().to_string()));
    // Return the project folder path even if it doesn't exist.
    // This allows us to identify pipenv environments in centralized directories
    // even when the original project has been moved or deleted.
    Some(project_folder)
}

fn is_pipenv_from_project(env: &PythonEnv) -> bool {
    // If the env prefix is inside a project folder, check that folder for a Pipfile.
    if let Some(prefix) = &env.prefix {
        if let Some(project_dir) = prefix.parent() {
            if project_dir.join("Pipfile").exists() {
                return true;
            }
        }
    }
    // Derive from the executable path as a fallback.
    if let Some(bin) = env.executable.parent() {
        let venv_dir = if bin.file_name().unwrap_or_default() == Path::new("bin")
            || bin.file_name().unwrap_or_default() == Path::new("Scripts")
        {
            bin.parent()
        } else {
            Some(bin)
        };
        if let Some(venv_dir) = venv_dir {
            if let Some(project_dir) = venv_dir.parent() {
                if project_dir.join("Pipfile").exists() {
                    return true;
                }
            }
        }
    }
    false
}

fn is_pipenv(env: &PythonEnv, env_vars: &EnvVariables) -> bool {
    trace!(
        "Pipenv: Checking if {:?} is a pipenv environment",
        env.executable
    );

    // Check if the environment is in a pipenv centralized directory.
    // This is the primary way to detect pipenv environments that are stored
    // in ~/.local/share/virtualenvs/ or similar locations.
    if is_in_pipenv_centralized_dir(env, env_vars) {
        trace!(
            "Pipenv: {:?} identified via centralized directory",
            env.executable
        );
        return true;
    }

    // Check if there's a .project file pointing to a project with a Pipfile
    if let Some(project_path) = get_pipenv_project(env) {
        let pipfile_path = project_path.join(env_vars.pipenv_pipfile.clone());
        if pipfile_path.exists() {
            trace!(
                "Pipenv: {:?} identified via .project file pointing to project with Pipfile at {:?}",
                env.executable,
                pipfile_path
            );
            return true;
        } else {
            trace!(
                "Pipenv: {:?} has .project pointing to {:?} but no Pipfile found",
                env.executable,
                project_path
            );
        }
    }

    // Check if the venv is inside a project folder with a Pipfile
    if is_pipenv_from_project(env) {
        trace!(
            "Pipenv: {:?} identified via in-project Pipfile",
            env.executable
        );
        return true;
    }

    trace!("Pipenv: {:?} is NOT a pipenv environment", env.executable);
    false
}

/// Get the default virtualenvs directory for pipenv
/// - If WORKON_HOME is set, use that
/// - Linux/macOS: ~/.local/share/virtualenvs/
/// - Windows: %USERPROFILE%\.virtualenvs\
fn get_virtualenvs_dir(env_vars: &EnvVariables) -> Option<PathBuf> {
    // First check WORKON_HOME environment variable
    if let Some(workon_home) = &env_vars.workon_home {
        if workon_home.is_dir() {
            return Some(workon_home.clone());
        }
    }

    // Fall back to default locations
    if let Some(home) = &env_vars.home {
        if std::env::consts::OS == "windows" {
            let dir = home.join(".virtualenvs");
            if dir.is_dir() {
                return Some(dir);
            }
        } else {
            let dir = home.join(".local").join("share").join("virtualenvs");
            if dir.is_dir() {
                return Some(dir);
            }
        }
    }

    None
}

/// Discover pipenv environments from the virtualenvs directory
fn list_environments(env_vars: &EnvVariables) -> Vec<PythonEnvironment> {
    let mut environments = vec![];

    if let Some(virtualenvs_dir) = get_virtualenvs_dir(env_vars) {
        trace!("Searching for pipenv environments in {:?}", virtualenvs_dir);

        if let Ok(entries) = fs::read_dir(&virtualenvs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                // Check if this directory is a valid virtualenv with a .project file
                let project_file = path.join(".project");
                if !project_file.exists() {
                    continue;
                }

                // Read the project path from .project file
                if let Ok(project_contents) = fs::read_to_string(&project_file) {
                    let project_path = PathBuf::from(project_contents.trim());
                    let project_path = norm_case(project_path);

                    // Check if the project has a Pipfile
                    if !project_path.join(&env_vars.pipenv_pipfile).exists() {
                        continue;
                    }

                    // Find the Python executable in the virtualenv
                    let bin_dir = if std::env::consts::OS == "windows" {
                        path.join("Scripts")
                    } else {
                        path.join("bin")
                    };

                    let python_exe = if std::env::consts::OS == "windows" {
                        bin_dir.join("python.exe")
                    } else {
                        bin_dir.join("python")
                    };

                    if python_exe.is_file() {
                        let symlinks = find_executables(&bin_dir);
                        let version = version::from_creator_for_virtual_env(&path);

                        let env =
                            PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Pipenv))
                                .executable(Some(norm_case(python_exe)))
                                .version(version)
                                .prefix(Some(norm_case(path.clone())))
                                .project(Some(project_path))
                                .symlinks(Some(symlinks))
                                .build();

                        trace!("Found pipenv environment: {:?}", env);
                        environments.push(env);
                    }
                }
            }
        }
    }

    environments
}

pub struct PipEnv {
    env_vars: EnvVariables,
    pipenv_executable: Arc<RwLock<Option<PathBuf>>>,
}

impl PipEnv {
    pub fn from(environment: &dyn Environment) -> PipEnv {
        PipEnv {
            env_vars: EnvVariables::from(environment),
            pipenv_executable: Arc::new(RwLock::new(None)),
        }
    }
}

impl Locator for PipEnv {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::PipEnv
    }

    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::ConfiguredOnly
    }

    fn configure(&self, config: &Configuration) {
        self.pipenv_executable
            .write()
            .unwrap()
            .clone_from(&config.pipenv_executable);
    }

    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::Pipenv]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if !is_pipenv(env, &self.env_vars) {
            return None;
        }
        // Project path is optional - centralized pipenv envs may have a .project file
        // pointing to a project that no longer exists
        let project_path = get_pipenv_project(env);
        trace!(
            "Pipenv: Building environment for {:?}, project: {:?}",
            env.executable,
            project_path
        );
        let mut prefix = env.prefix.clone();
        if prefix.is_none() {
            if let Some(bin) = env.executable.parent() {
                if bin.file_name().unwrap_or_default() == Path::new("bin")
                    || bin.file_name().unwrap_or_default() == Path::new("Scripts")
                {
                    if let Some(dir) = bin.parent() {
                        prefix = Some(dir.to_owned());
                    }
                }
            }
        }
        let bin = env.executable.parent()?;
        let symlinks = find_executables(bin);
        let mut version = env.version.clone();
        if version.is_none() && prefix.is_some() {
            if let Some(prefix) = &prefix {
                version = version::from_creator_for_virtual_env(prefix);
            }
        }
        Some(
            PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Pipenv))
                .executable(Some(env.executable.clone()))
                .version(version)
                .prefix(prefix)
                .project(project_path)
                .symlinks(Some(symlinks))
                .build(),
        )
    }

    fn find(&self, reporter: &dyn Reporter) {
        // First, find and report the pipenv manager
        let pipenv_exe = self.pipenv_executable.read().unwrap().clone();
        if let Some(manager) = PipenvManager::find(pipenv_exe, &self.env_vars) {
            trace!("Found pipenv manager: {:?}", manager);
            reporter.report_manager(&manager.to_manager());
        }

        // Then discover and report pipenv environments
        let environments = list_environments(&self.env_vars);
        for env in environments {
            reporter.report_environment(&env);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_temp_dir() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let id = std::process::id();
        let counter = TEST_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        dir.push(format!("pet_pipenv_test_{}_{}", id, counter));
        dir
    }

    fn create_test_env_vars(home: Option<PathBuf>) -> EnvVariables {
        EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home,
            xdg_data_home: None,
            workon_home: None,
            path: None,
        }
    }

    #[test]
    fn infer_project_for_venv_in_project() {
        let project_dir = unique_temp_dir();
        let venv_dir = project_dir.join(".venv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        // Create directories and files
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(project_dir.join("Pipfile"), b"[[source]]\n").unwrap();
        // Touch python exe file
        std::fs::write(&python_exe, b"").unwrap();
        // Touch pyvenv.cfg in venv root so PythonEnv::new logic would normally detect prefix
        std::fs::write(venv_dir.join("pyvenv.cfg"), b"version = 3.12.0\n").unwrap();

        // Construct PythonEnv directly
        let env = PythonEnv {
            executable: norm_case(python_exe.clone()),
            prefix: Some(norm_case(venv_dir.clone())),
            version: None,
            symlinks: None,
        };

        // Validate helper infers project
        let inferred = get_pipenv_project(&env).expect("expected project path");
        assert_eq!(inferred, norm_case(project_dir.clone()));

        // Validate locator populates project
        let locator = PipEnv {
            env_vars: create_test_env_vars(None),
            pipenv_executable: Arc::new(RwLock::new(None)),
        };
        let result = locator
            .try_from(&env)
            .expect("expected locator to return environment");
        assert_eq!(result.project, Some(norm_case(project_dir.clone())));

        // Cleanup
        std::fs::remove_dir_all(&project_dir).ok();
    }

    #[test]
    fn detect_pipenv_centralized_env() {
        // Simulate pipenv's centralized directory structure:
        // ~/.local/share/virtualenvs/myproject-Abc123/
        let temp_home = unique_temp_dir();
        let virtualenvs_dir = temp_home.join(".local").join("share").join("virtualenvs");
        let venv_dir = virtualenvs_dir.join("myproject-Abc123XyZ");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        // Create the project directory with a Pipfile
        let project_dir = temp_home.join("projects").join("myproject");
        std::fs::create_dir_all(&project_dir).unwrap();
        std::fs::write(project_dir.join("Pipfile"), b"[[source]]\n").unwrap();

        // Create the centralized venv with .project file
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(venv_dir.join("pyvenv.cfg"), b"version = 3.13.0\n").unwrap();
        std::fs::write(
            venv_dir.join(".project"),
            project_dir.to_string_lossy().as_bytes(),
        )
        .unwrap();

        // Construct PythonEnv
        let env = PythonEnv {
            executable: norm_case(python_exe.clone()),
            prefix: Some(norm_case(venv_dir.clone())),
            version: None,
            symlinks: None,
        };

        // Create env_vars with home pointing to our temp directory
        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp_home.clone()),
            xdg_data_home: None,
            workon_home: None,
            path: None,
        };

        // Validate is_in_pipenv_centralized_dir detects it
        assert!(
            is_in_pipenv_centralized_dir(&env, &env_vars),
            "Expected env to be detected in centralized dir"
        );

        // Validate is_pipenv returns true
        assert!(
            is_pipenv(&env, &env_vars),
            "Expected env to be identified as pipenv"
        );

        // Validate locator returns the environment
        let locator = PipEnv {
            env_vars,
            pipenv_executable: Arc::new(RwLock::new(None)),
        };
        let result = locator
            .try_from(&env)
            .expect("expected locator to return environment");
        assert_eq!(result.kind, Some(PythonEnvironmentKind::Pipenv));
        assert_eq!(result.project, Some(norm_case(project_dir.clone())));

        // Cleanup
        std::fs::remove_dir_all(&temp_home).ok();
    }

    #[test]
    fn detect_pipenv_centralized_env_without_existing_project() {
        // Test that we still identify as pipenv even if the project folder doesn't exist
        let temp_home = unique_temp_dir();
        let virtualenvs_dir = temp_home.join(".local").join("share").join("virtualenvs");
        let venv_dir = virtualenvs_dir.join("deleted-project-Xyz789");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        // Don't create the project directory - simulating it was deleted

        // Create the centralized venv with .project file pointing to non-existent path
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(venv_dir.join("pyvenv.cfg"), b"version = 3.13.0\n").unwrap();
        std::fs::write(venv_dir.join(".project"), "/path/to/deleted/project").unwrap();

        // Construct PythonEnv
        let env = PythonEnv {
            executable: norm_case(python_exe.clone()),
            prefix: Some(norm_case(venv_dir.clone())),
            version: None,
            symlinks: None,
        };

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp_home.clone()),
            xdg_data_home: None,
            workon_home: None,
            path: None,
        };

        // Should still be detected as pipenv (centralized directory + .project file)
        assert!(
            is_in_pipenv_centralized_dir(&env, &env_vars),
            "Expected env to be detected in centralized dir"
        );
        assert!(
            is_pipenv(&env, &env_vars),
            "Expected env to be identified as pipenv"
        );

        // Locator should return the environment, but project will point to non-existent path
        let locator = PipEnv {
            env_vars,
            pipenv_executable: Arc::new(RwLock::new(None)),
        };
        let result = locator
            .try_from(&env)
            .expect("expected locator to return environment");
        assert_eq!(result.kind, Some(PythonEnvironmentKind::Pipenv));

        // Cleanup
        std::fs::remove_dir_all(&temp_home).ok();
    }

    #[test]
    fn detect_pipenv_centralized_env_without_project_file_via_naming_pattern() {
        // Test fallback detection when .project file is missing but directory name matches
        // pipenv's naming pattern: {project-name}-{8-char-hash}
        let temp_home = unique_temp_dir();
        let virtualenvs_dir = temp_home.join(".local").join("share").join("virtualenvs");
        // Use a name that matches pipenv pattern: name + hyphen + 8 alphanumeric chars
        let venv_dir = virtualenvs_dir.join("myproject-AbC12xYz");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        // Create the venv WITHOUT a .project file (simulating corrupted/deleted .project)
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(venv_dir.join("pyvenv.cfg"), b"version = 3.13.0\n").unwrap();
        // Explicitly NOT creating .project file

        // Construct PythonEnv
        let env = PythonEnv {
            executable: norm_case(python_exe.clone()),
            prefix: Some(norm_case(venv_dir.clone())),
            version: None,
            symlinks: None,
        };

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp_home.clone()),
            xdg_data_home: None,
            workon_home: None,
            path: None,
        };

        // Should be detected via naming pattern fallback
        assert!(
            is_in_pipenv_centralized_dir(&env, &env_vars),
            "Expected env to be detected in centralized dir via naming pattern"
        );
        assert!(
            is_pipenv(&env, &env_vars),
            "Expected env to be identified as pipenv via naming pattern"
        );

        // Locator should return the environment
        let locator = PipEnv {
            env_vars,
            pipenv_executable: Arc::new(RwLock::new(None)),
        };
        let result = locator
            .try_from(&env)
            .expect("expected locator to return environment");
        assert_eq!(result.kind, Some(PythonEnvironmentKind::Pipenv));
        // Project should be None since there's no .project file and no Pipfile nearby
        assert_eq!(result.project, None);

        // Cleanup
        std::fs::remove_dir_all(&temp_home).ok();
    }

    #[test]
    fn test_pipenv_naming_pattern_regex() {
        // Test that our regex correctly matches pipenv naming patterns
        // Valid patterns: {name}-{8-char-hash}
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("myproject-AbC12xYz"));
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("my-project-AbC12xYz"));
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("my_project-AbC12xYz"));
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("project123-12345678"));
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("a-b-c-d-12345678"));
        // URL-safe base64 can include _ and -
        assert!(PIPENV_ENV_NAME_PATTERN.is_match("myproject-AbC_2-Yz"));

        // Invalid patterns (should NOT match)
        assert!(!PIPENV_ENV_NAME_PATTERN.is_match("myproject")); // no hash
        assert!(!PIPENV_ENV_NAME_PATTERN.is_match("myproject-abc")); // hash too short (3 chars)
        assert!(!PIPENV_ENV_NAME_PATTERN.is_match("myproject-abcdefg")); // hash too short (7 chars)
        assert!(!PIPENV_ENV_NAME_PATTERN.is_match("-AbC12xYz")); // no project name
    }

    // ── get_pipenv_virtualenv_dirs ────────────────────────────────

    #[test]
    fn virtualenv_dirs_returns_workon_home_when_set() {
        let temp = unique_temp_dir();
        let workon_home = temp.join("workon");
        std::fs::create_dir_all(&workon_home).unwrap();

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp.clone()),
            xdg_data_home: None,
            workon_home: Some(workon_home.clone()),
            path: None,
        };

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.contains(&norm_case(&workon_home)),
            "Expected WORKON_HOME in dirs: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_returns_xdg_data_home_virtualenvs() {
        let temp = unique_temp_dir();
        let xdg_dir = temp.join("xdg");
        let xdg_venvs = xdg_dir.join("virtualenvs");
        std::fs::create_dir_all(&xdg_venvs).unwrap();

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp.clone()),
            xdg_data_home: Some(xdg_dir.to_string_lossy().to_string()),
            workon_home: None,
            path: None,
        };

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.contains(&norm_case(&xdg_venvs)),
            "Expected XDG_DATA_HOME/virtualenvs in dirs: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_returns_local_share_virtualenvs() {
        let temp = unique_temp_dir();
        let local_share_venvs = temp.join(".local").join("share").join("virtualenvs");
        std::fs::create_dir_all(&local_share_venvs).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.contains(&norm_case(&local_share_venvs)),
            "Expected ~/.local/share/virtualenvs in dirs: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_returns_dot_venvs() {
        let temp = unique_temp_dir();
        let dot_venvs = temp.join(".venvs");
        std::fs::create_dir_all(&dot_venvs).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.contains(&norm_case(&dot_venvs)),
            "Expected ~/.venvs in dirs: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_returns_dot_virtualenvs() {
        let temp = unique_temp_dir();
        let dot_virtualenvs = temp.join(".virtualenvs");
        std::fs::create_dir_all(&dot_virtualenvs).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.contains(&norm_case(&dot_virtualenvs)),
            "Expected ~/.virtualenvs in dirs: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_returns_empty_when_no_dirs_exist() {
        let temp = unique_temp_dir();
        // Don't create any directories inside
        std::fs::create_dir_all(&temp).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            dirs.is_empty(),
            "Expected empty dirs when nothing exists: {:?}",
            dirs
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn virtualenv_dirs_skips_non_existent_workon_home() {
        let temp = unique_temp_dir();
        std::fs::create_dir_all(&temp).unwrap();

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp.clone()),
            xdg_data_home: None,
            workon_home: Some(temp.join("does_not_exist")),
            path: None,
        };

        let dirs = get_pipenv_virtualenv_dirs(&env_vars);
        assert!(
            !dirs.contains(&norm_case(temp.join("does_not_exist"))),
            "Non-existent WORKON_HOME should not be in dirs"
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    // ── get_virtualenvs_dir ───────────────────────────────────────

    #[test]
    fn get_virtualenvs_dir_prefers_workon_home() {
        let temp = unique_temp_dir();
        let workon_home = temp.join("workon");
        std::fs::create_dir_all(&workon_home).unwrap();
        // Also create the default fallback
        if cfg!(windows) {
            std::fs::create_dir_all(temp.join(".virtualenvs")).unwrap();
        } else {
            std::fs::create_dir_all(temp.join(".local").join("share").join("virtualenvs")).unwrap();
        }

        let env_vars = EnvVariables {
            pipenv_max_depth: 3,
            pipenv_pipfile: "Pipfile".to_string(),
            home: Some(temp.clone()),
            xdg_data_home: None,
            workon_home: Some(workon_home.clone()),
            path: None,
        };

        let result = get_virtualenvs_dir(&env_vars);
        assert_eq!(result, Some(workon_home));

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn get_virtualenvs_dir_returns_platform_default() {
        let temp = unique_temp_dir();
        let expected = if cfg!(windows) {
            temp.join(".virtualenvs")
        } else {
            temp.join(".local").join("share").join("virtualenvs")
        };
        std::fs::create_dir_all(&expected).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let result = get_virtualenvs_dir(&env_vars);
        assert_eq!(result, Some(expected));

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn get_virtualenvs_dir_returns_none_when_nothing_exists() {
        let temp = unique_temp_dir();
        std::fs::create_dir_all(&temp).unwrap();

        let env_vars = create_test_env_vars(Some(temp.clone()));

        let result = get_virtualenvs_dir(&env_vars);
        assert_eq!(result, None);

        std::fs::remove_dir_all(&temp).ok();
    }

    // ── is_pipenv_from_project ────────────────────────────────────

    #[test]
    fn is_pipenv_from_project_detects_pipfile_next_to_venv_prefix() {
        let project_dir = unique_temp_dir();
        let venv_dir = project_dir.join(".venv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(project_dir.join("Pipfile"), b"[[source]]\n").unwrap();

        let env = PythonEnv {
            executable: norm_case(&python_exe),
            prefix: Some(norm_case(&venv_dir)),
            version: None,
            symlinks: None,
        };

        assert!(is_pipenv_from_project(&env));

        std::fs::remove_dir_all(&project_dir).ok();
    }

    #[test]
    fn is_pipenv_from_project_detects_via_executable_path_without_prefix() {
        let project_dir = unique_temp_dir();
        let venv_dir = project_dir.join(".venv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(project_dir.join("Pipfile"), b"[[source]]\n").unwrap();

        let env = PythonEnv {
            executable: norm_case(&python_exe),
            prefix: None,
            version: None,
            symlinks: None,
        };

        assert!(is_pipenv_from_project(&env));

        std::fs::remove_dir_all(&project_dir).ok();
    }

    #[test]
    fn is_pipenv_from_project_returns_false_without_pipfile() {
        let project_dir = unique_temp_dir();
        let venv_dir = project_dir.join(".venv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        // No Pipfile created

        let env = PythonEnv {
            executable: norm_case(&python_exe),
            prefix: Some(norm_case(&venv_dir)),
            version: None,
            symlinks: None,
        };

        assert!(!is_pipenv_from_project(&env));

        std::fs::remove_dir_all(&project_dir).ok();
    }

    // ── is_pipenv ─────────────────────────────────────────────────

    #[test]
    fn is_pipenv_returns_false_for_plain_venv() {
        let temp = unique_temp_dir();
        let venv_dir = temp.join("myenv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        // No Pipfile, no .project, not in centralized dir

        let env = PythonEnv {
            executable: norm_case(&python_exe),
            prefix: Some(norm_case(&venv_dir)),
            version: None,
            symlinks: None,
        };

        let env_vars = create_test_env_vars(Some(temp.clone()));
        assert!(!is_pipenv(&env, &env_vars));

        std::fs::remove_dir_all(&temp).ok();
    }

    // ── get_pipenv_project ────────────────────────────────────────

    #[test]
    fn get_pipenv_project_reads_dot_project_file() {
        let temp = unique_temp_dir();
        let venv_dir = temp.join("myproject-Abc12345");
        std::fs::create_dir_all(&venv_dir).unwrap();

        let project_path = temp.join("projects").join("myproject");
        std::fs::write(
            venv_dir.join(".project"),
            project_path.to_string_lossy().as_bytes(),
        )
        .unwrap();

        let python_exe = if cfg!(windows) {
            venv_dir.join("Scripts").join("python.exe")
        } else {
            venv_dir.join("bin").join("python")
        };
        let env = PythonEnv {
            executable: python_exe,
            prefix: Some(venv_dir.clone()),
            version: None,
            symlinks: None,
        };

        let result = get_pipenv_project(&env);
        assert_eq!(result, Some(norm_case(project_path)));

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn get_pipenv_project_returns_none_without_project_file_or_pipfile() {
        let temp = unique_temp_dir();
        let venv_dir = temp.join("myenv");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        std::fs::create_dir_all(&bin_dir).unwrap();

        let env = PythonEnv {
            executable: bin_dir.join("python"),
            prefix: Some(venv_dir.clone()),
            version: None,
            symlinks: None,
        };

        let result = get_pipenv_project(&env);
        assert_eq!(result, None);

        std::fs::remove_dir_all(&temp).ok();
    }

    // ── locator metadata ──────────────────────────────────────────

    #[test]
    fn locator_metadata_matches_pipenv_kind() {
        let locator = PipEnv {
            env_vars: create_test_env_vars(None),
            pipenv_executable: Arc::new(RwLock::new(None)),
        };
        assert_eq!(locator.get_kind(), LocatorKind::PipEnv);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::Pipenv]
        );
    }

    // ── is_in_pipenv_centralized_dir without prefix ───────────────

    #[test]
    fn centralized_dir_detection_derives_prefix_from_executable() {
        let temp = unique_temp_dir();
        let virtualenvs_dir = temp.join(".local").join("share").join("virtualenvs");
        let venv_dir = virtualenvs_dir.join("project-AbC12xYz");
        let bin_dir = if cfg!(windows) {
            venv_dir.join("Scripts")
        } else {
            venv_dir.join("bin")
        };
        let python_exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };

        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(&python_exe, b"").unwrap();
        std::fs::write(venv_dir.join(".project"), b"/some/project").unwrap();

        // PythonEnv without prefix set
        let env = PythonEnv {
            executable: norm_case(&python_exe),
            prefix: None,
            version: None,
            symlinks: None,
        };

        let env_vars = create_test_env_vars(Some(temp.clone()));
        assert!(
            is_in_pipenv_centralized_dir(&env, &env_vars),
            "Should derive prefix from executable and detect centralized dir"
        );

        std::fs::remove_dir_all(&temp).ok();
    }

    #[test]
    fn centralized_dir_detection_returns_false_when_prefix_not_derivable() {
        let temp = unique_temp_dir();
        std::fs::create_dir_all(&temp).unwrap();

        // Executable whose parent directory is neither "bin" nor "Scripts",
        // so is_in_pipenv_centralized_dir cannot derive a prefix.
        let env = PythonEnv {
            executable: temp.join("random").join("python"),
            prefix: None,
            version: None,
            symlinks: None,
        };

        let env_vars = create_test_env_vars(Some(temp.clone()));
        assert!(!is_in_pipenv_centralized_dir(&env, &env_vars));

        std::fs::remove_dir_all(&temp).ok();
    }
}
