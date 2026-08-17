// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{any::Any, path::PathBuf};

use env::PythonEnv;
use manager::EnvManager;
use python_environment::{PythonEnvironment, PythonEnvironmentKind};
use reporter::Reporter;

pub mod arch;
pub mod cache;
pub mod env;
pub mod manager;
pub mod os_environment;
pub mod python_environment;
pub mod pyvenv_cfg;
pub mod reporter;
pub mod telemetry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatorResult {
    pub managers: Vec<EnvManager>,
    pub environments: Vec<PythonEnvironment>,
}

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    /// These are paths like workspace folders, where we can look for environments.
    pub workspace_directories: Option<Vec<PathBuf>>,
    pub executables: Option<Vec<PathBuf>>,
    pub conda_executable: Option<PathBuf>,
    pub pipenv_executable: Option<PathBuf>,
    pub poetry_executable: Option<PathBuf>,
    /// Custom locations where environments can be found.
    /// These are different from search_paths, as these are specific directories where environments are expected.
    /// environment_directories on the other hand can be any directory such as a workspace folder, where envs might never exist.
    pub environment_directories: Option<Vec<PathBuf>>,
    /// Directory to cache the Python environment details.
    pub cache_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocatorKind {
    Conda,
    Hatch,
    Homebrew,
    LinuxGlobal,
    MacCommandLineTools,
    MacPythonOrg,
    MacXCode,
    PipEnv,
    Pixi,
    Poetry,
    PyEnv,
    Uv,
    Venv,
    VirtualEnv,
    VirtualEnvWrapper,
    WinPython,
    WindowsRegistry,
    WindowsStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshStatePersistence {
    /// The locator keeps no mutable state that survives a request.
    Stateless,
    /// The locator keeps configured inputs only.
    ///
    /// Refresh creates and configures transient locator instances for one request. A
    /// locator in this category must get its configuration from that request's
    /// configuration snapshot, not by copying anything back from the transient
    /// locator into the long-lived shared locator.
    ConfiguredOnly,
    /// The locator keeps cache-like state that later requests can repopulate on demand.
    ///
    /// Refresh may populate this state on a transient locator, but correctness must
    /// not depend on syncing it back into the long-lived shared locator.
    SelfHydratingCache,
    /// The locator keeps refresh-discovered state that later requests depend on.
    ///
    /// Locators in this category must override `sync_refresh_state_from()` and copy
    /// only correctness-critical discovery state for the provided sync scope.
    SyncedDiscoveryState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefreshStateSyncScope {
    Full,
    GlobalFiltered(PythonEnvironmentKind),
    Workspace,
}

pub trait Locator: Any + Send + Sync {
    /// Returns the name of the locator.
    fn get_kind(&self) -> LocatorKind;
    /// Configures the locator with the given configuration.
    ///
    /// Override this method if you need to store configuration in the locator.
    ///
    /// # Why `&self` instead of `&mut self`?
    ///
    /// Locators are shared across threads via `Arc<dyn Locator>` and may be
    /// configured while other operations are in progress. Using `&self` allows
    /// concurrent access without requiring the caller to hold an exclusive lock
    /// on the entire locator.
    ///
    /// Implementations that need to store configuration should use interior
    /// mutability (e.g., `Mutex<T>` or `RwLock<T>`) for the mutable fields only.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use std::sync::Mutex;
    /// use std::path::PathBuf;
    ///
    /// struct MyLocator {
    ///     workspace_dirs: Mutex<Vec<PathBuf>>,
    /// }
    ///
    /// impl Locator for MyLocator {
    ///     fn configure(&self, config: &Configuration) {
    ///         if let Some(dirs) = &config.workspace_directories {
    ///             *self.workspace_dirs.lock().expect("workspace_dirs mutex poisoned") = dirs.clone();
    ///         }
    ///     }
    ///     // ... other required methods
    /// }
    /// ```
    fn configure(&self, _config: &Configuration) {
        //
    }
    /// Describes what mutable state, if any, must survive a refresh boundary.
    ///
    /// Refresh requests run against transient locator graphs. After a refresh
    /// completes, the server invokes `sync_refresh_state_from()` on the long-lived
    /// shared locator graph while the starting configuration generation is still
    /// current. The returned classification is the contract the locator makes with
    /// that sync step.
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::Stateless
    }
    /// Copies correctness-critical post-refresh state from a transient locator into this
    /// long-lived shared locator.
    ///
    /// Override this only when `refresh_state()` returns
    /// `RefreshStatePersistence::SyncedDiscoveryState`.
    fn sync_refresh_state_from(&self, _source: &dyn Locator, _scope: &RefreshStateSyncScope) {
        //
    }
    /// Returns a list of supported categories for this locator.
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind>;
    /// Given a Python executable, and some optional data like prefix,
    /// this method will attempt to convert it to a PythonEnvironment that can be supported by this particular locator.
    /// If an environment is not supported by this locator, then None is returned.
    ///
    /// Note: The returned environment could have some missing information.
    /// This is because the `from` will do a best effort to get the environment information without spawning Python.
    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment>;
    /// Finds all environments specific to this locator.
    fn find(&self, reporter: &dyn Reporter);
}

impl dyn Locator {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}
