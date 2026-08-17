// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{path::PathBuf, sync::Arc};

use log::{trace, warn};
use pet_core::{
    arch::Architecture,
    env::PythonEnv,
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder},
    Locator,
};
use pet_env_var_path::get_search_paths_from_env_variables;
use pet_python_utils::{env::ResolvedPythonEnv, executable::find_executable};

use crate::locators::identify_python_environment_using_locators;

#[derive(Debug)]
pub struct ResolvedEnvironment {
    pub discovered: PythonEnvironment,
    pub resolved: Option<PythonEnvironment>,
}

pub fn resolve_environment(
    executable: &PathBuf,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    os_environment: &dyn Environment,
) -> Option<ResolvedEnvironment> {
    // First check if executable is actually a file or a path.
    let mut executable = executable.to_owned();
    if executable.is_dir() {
        trace!(
            "Looking to resolve Python executable in provided directory, {:?}, file = {:?}, sylink = {:?}, metadata = {:?}",
            executable,
            executable.is_file(),
            executable.is_symlink(),
            executable.metadata(),
        );
        executable = match find_executable(&executable) {
            Some(exe) => exe,
            None => {
                warn!("Could not find Python executable in {:?}", executable);
                executable
            }
        };
        trace!(
            "Found Python executable in provided directory, {:?}",
            executable
        );
    }
    // First check if this is a known environment
    let env = PythonEnv::new(executable.to_owned(), None, None);
    trace!(
        "In resolve_environment, looking for Python Env {:?} in {:?}",
        env,
        executable
    );
    let global_env_search_paths: Vec<PathBuf> = get_search_paths_from_env_variables(os_environment);

    if let Some(env) =
        identify_python_environment_using_locators(&env, locators, &global_env_search_paths)
    {
        // Ok we got the environment.
        // Now try to resolve this fully, by spawning python.
        if let Some(ref executable) = env.executable {
            if let Some(info) = ResolvedPythonEnv::from(executable) {
                trace!(
                    "In resolve_environment, Resolved Python Exe {:?} as {:?}",
                    executable,
                    info
                );
                let discovered = env.clone();
                let mut symlinks = env.symlinks.clone().unwrap_or_default();
                symlinks.push(info.executable.clone());
                symlinks.append(&mut info.symlinks.clone().unwrap_or_default());
                symlinks.sort();
                symlinks.dedup();

                let version = Some(info.version.clone());
                let prefix = Some(info.prefix.clone());
                let arch = Some(if info.is64_bit {
                    Architecture::X64
                } else {
                    Architecture::X86
                });

                let resolved = PythonEnvironmentBuilder::new(env.kind)
                    .arch(arch)
                    .display_name(env.display_name)
                    .executable(Some(info.executable.clone()))
                    .manager(env.manager)
                    .name(env.name)
                    .prefix(prefix)
                    .project(env.project)
                    .symlinks(Some(symlinks))
                    .version(version)
                    .build();

                info.add_to_cache(resolved.clone());

                Some(ResolvedEnvironment {
                    discovered,
                    resolved: Some(resolved),
                })
            } else {
                Some(ResolvedEnvironment {
                    discovered: env,
                    resolved: None,
                })
            }
        } else {
            warn!("Unknown Python Env {:?} resolved as {:?}", executable, env);
            Some(ResolvedEnvironment {
                discovered: env,
                resolved: None,
            })
        }
    } else {
        warn!("Unknown Python Env {:?}", executable);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::{
        env::PythonEnv,
        os_environment::Environment,
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        reporter::Reporter,
        Locator, LocatorKind,
    };

    struct EmptyEnvironment;
    impl Environment for EmptyEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            None
        }
        fn get_root(&self) -> Option<PathBuf> {
            None
        }
        fn get_env_var(&self, _key: String) -> Option<String> {
            None
        }
        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            vec![]
        }
    }

    /// A test locator that recognizes any executable as a known environment.
    struct AcceptAllLocator;
    impl Locator for AcceptAllLocator {
        fn get_kind(&self) -> LocatorKind {
            LocatorKind::LinuxGlobal
        }
        fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
            vec![PythonEnvironmentKind::GlobalPaths]
        }
        fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
            Some(
                PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::GlobalPaths))
                    .executable(Some(env.executable.clone()))
                    .build(),
            )
        }
        fn find(&self, _reporter: &dyn Reporter) {}
    }

    #[test]
    fn resolve_does_not_reject_non_standard_executable_names() {
        // Issue #375: DCC tools like mayapy.exe and hython.exe should not be
        // rejected by resolve_environment based on filename alone.
        let temp_dir =
            std::env::temp_dir().join(format!("pet_resolve_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);

        let exe_name = if cfg!(windows) {
            "mayapy.exe"
        } else {
            "mayapy"
        };
        let fake_exe = temp_dir.join(exe_name);
        std::fs::write(&fake_exe, "fake").unwrap();

        let locators: Arc<Vec<Arc<dyn Locator>>> =
            Arc::new(vec![Arc::new(AcceptAllLocator) as Arc<dyn Locator>]);
        let env = EmptyEnvironment;

        let result = resolve_environment(&fake_exe, &locators, &env);

        // Clean up before assertions to ensure cleanup on test failure.
        let _ = std::fs::remove_file(&fake_exe);
        let _ = std::fs::remove_dir(&temp_dir);

        // The locator recognizes it, so we should get a result back
        // (resolved will be None because there's no real Python to spawn,
        // but the environment should be discovered).
        assert!(
            result.is_some(),
            "resolve_environment should not reject non-standard executable names like {:?}",
            exe_name
        );
        let resolved = result.unwrap();
        // Compare filenames only — on Windows CI, temp paths may use 8.3 short
        // names (e.g., RUNNER~1) while the discovered path uses the long form.
        assert_eq!(
            resolved
                .discovered
                .executable
                .as_ref()
                .and_then(|p| p.file_name().map(|f| f.to_owned())),
            Some(std::ffi::OsString::from(exe_name)),
            "discovered executable filename should match"
        );
    }

    #[test]
    fn resolve_nonexistent_non_standard_name_reaches_locator_chain() {
        // With AcceptAllLocator, even a non-existent file with a non-standard
        // name should reach the locator chain (not be rejected by name check).
        let nonexistent = PathBuf::from(if cfg!(windows) {
            r"C:\nonexistent\hython.exe"
        } else {
            "/nonexistent/hython"
        });

        let locators: Arc<Vec<Arc<dyn Locator>>> =
            Arc::new(vec![Arc::new(AcceptAllLocator) as Arc<dyn Locator>]);
        let env = EmptyEnvironment;

        // AcceptAllLocator returns Some for any executable, proving the
        // locator chain was reached despite the non-standard name.
        let result = resolve_environment(&nonexistent, &locators, &env);
        assert!(
            result.is_some(),
            "non-standard executable name should reach the locator chain"
        );
    }
}
