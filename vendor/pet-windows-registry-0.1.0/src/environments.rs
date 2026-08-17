// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(windows)]
use pet_conda::CondaLocator;
#[cfg(windows)]
use pet_core::reporter::Reporter;
#[cfg(windows)]
use pet_core::{
    arch::Architecture,
    python_environment::{PythonEnvironmentBuilder, PythonEnvironmentKind},
    LocatorResult,
};
#[cfg(windows)]
use pet_windows_store::is_windows_app_folder_in_program_files;
#[cfg(windows)]
use std::{path::PathBuf, sync::Arc};
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
fn empty_result() -> LocatorResult {
    LocatorResult {
        environments: vec![],
        managers: vec![],
    }
}

/// What a single company-level worker thread produces: the registry-derived
/// `PythonEnvironment`s plus the install dirs that turned out to be conda
/// roots. We capture conda dirs (instead of just calling
/// `conda_locator.find_and_report` and forgetting) so that subsequent
/// `find()` calls on a cache hit can replay them — otherwise registry-only
/// conda installs would silently disappear after the first refresh.
#[cfg(windows)]
struct CompanyWalkOutcome {
    result: LocatorResult,
    conda_install_dirs: Vec<PathBuf>,
}

#[cfg(windows)]
impl CompanyWalkOutcome {
    fn empty() -> Self {
        Self {
            result: empty_result(),
            conda_install_dirs: vec![],
        }
    }
}

/// Logs a warning if a spawned registry-walk thread panicked, then
/// substitutes an empty outcome so the surviving hive/companies still
/// surface their environments. Without this we'd silently lose the entire
/// hive when one company's walk panics — exactly the kind of regression
/// that's hardest to debug after the fact.
///
/// Returns `(outcome, had_panic)` so callers can decide not to cache a
/// partial result (otherwise a single transient panic would persist as a
/// stale empty/partial cache across refreshes — see issue #454).
#[cfg(windows)]
fn join_or_warn(
    join_result: std::thread::Result<CompanyWalkOutcome>,
    label: &str,
) -> (CompanyWalkOutcome, bool) {
    use log::warn;
    match join_result {
        Ok(outcome) => (outcome, false),
        Err(panic_payload) => {
            // Try to render the payload for the log; payloads are commonly
            // either a `&'static str` or a `String`.
            let message = panic_payload
                .downcast_ref::<&'static str>()
                .map(|s| (*s).to_string())
                .or_else(|| panic_payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            warn!("Registry walk thread for {} panicked: {}", label, message);
            (CompanyWalkOutcome::empty(), true)
        }
    }
}

/// Outcome of a registry walk: the discovered environments/managers, the
/// conda install dirs found via the registry (so cache-hit replays can
/// re-invoke `conda_locator.find_and_report`), and a flag indicating
/// whether any worker thread panicked. Callers should avoid caching a
/// result with `had_panic = true` so a transient failure can be retried
/// on the next refresh instead of becoming sticky.
#[cfg(windows)]
pub struct RegistryWalkOutcome {
    pub result: LocatorResult,
    pub conda_install_dirs: Vec<PathBuf>,
    pub had_panic: bool,
}

#[cfg(windows)]
pub fn get_registry_pythons(
    conda_locator: &Arc<dyn CondaLocator>,
    reporter: &Option<&dyn Reporter>,
) -> RegistryWalkOutcome {
    use std::thread;

    // Walk both hives in parallel. Each hive walks its companies in parallel
    // too (see `get_registry_pythons_for_hive`). HKLM and HKCU sit on
    // independent registry trees and Defender intercepts every read, so the
    // serial baseline was paying for both round-trips back to back; the
    // scope-spawn pattern matches `pet-pyenv` / `pet-homebrew` / `pet-conda`.
    let (hklm_outcome, hkcu_outcome) = thread::scope(|s| {
        let hklm = s.spawn(|| {
            get_registry_pythons_for_hive(
                "HKLM",
                RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE),
                conda_locator,
                reporter,
            )
        });
        let hkcu = s.spawn(|| {
            get_registry_pythons_for_hive(
                "HKCU",
                RegKey::predef(winreg::enums::HKEY_CURRENT_USER),
                conda_locator,
                reporter,
            )
        });
        (
            join_hive_outcome(hklm.join(), "HKLM"),
            join_hive_outcome(hkcu.join(), "HKCU"),
        )
    });

    let mut environments = hklm_outcome.result.environments;
    environments.extend(hkcu_outcome.result.environments);
    let mut managers = hklm_outcome.result.managers;
    managers.extend(hkcu_outcome.result.managers);
    let mut conda_install_dirs = hklm_outcome.conda_install_dirs;
    conda_install_dirs.extend(hkcu_outcome.conda_install_dirs);

    RegistryWalkOutcome {
        result: LocatorResult {
            environments,
            managers,
        },
        conda_install_dirs,
        had_panic: hklm_outcome.had_panic || hkcu_outcome.had_panic,
    }
}

/// Sibling of `join_or_warn` for hive-level threads. Returns the recovered
/// outcome (already aggregating any company-level panics) so the top
/// level can OR the panic flags and concatenate the conda install dirs.
#[cfg(windows)]
fn join_hive_outcome(
    join_result: std::thread::Result<RegistryWalkOutcome>,
    label: &str,
) -> RegistryWalkOutcome {
    use log::warn;
    match join_result {
        Ok(outcome) => outcome,
        Err(panic_payload) => {
            let message = panic_payload
                .downcast_ref::<&'static str>()
                .map(|s| (*s).to_string())
                .or_else(|| panic_payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            warn!("Registry walk thread for {} panicked: {}", label, message);
            RegistryWalkOutcome {
                result: empty_result(),
                conda_install_dirs: vec![],
                had_panic: true,
            }
        }
    }
}

/// Walks `<hive>\Software\Python\<company>` for every company in the given
/// hive. Companies are processed in parallel; each spawned thread owns its
/// own `RegKey` handle (which is `Send` but not `Sync` in `winreg`).
#[cfg(windows)]
fn get_registry_pythons_for_hive(
    name: &'static str,
    hive: RegKey,
    conda_locator: &Arc<dyn CondaLocator>,
    reporter: &Option<&dyn Reporter>,
) -> RegistryWalkOutcome {
    use log::{trace, warn};
    use std::thread;

    let python_key = match hive.open_subkey("Software\\Python") {
        Ok(k) => k,
        Err(err) => {
            warn!("Failed to open {}\\Software\\Python, {:?}", name, err);
            return RegistryWalkOutcome {
                result: empty_result(),
                conda_install_dirs: vec![],
                had_panic: false,
            };
        }
    };

    // Open each company subkey serially. Opening a registry handle is cheap
    // (no recursive enumeration); the heavy work happens once we start
    // pulling values out of `<company>\<install>\InstallPath`. Collecting
    // owned `(String, RegKey)` pairs lets us hand each company to its own
    // thread without sharing a `RegKey` (which is `Send` but not `Sync`).
    let companies: Vec<(String, RegKey)> = python_key
        .enum_keys()
        .filter_map(Result::ok)
        .filter_map(|company| match python_key.open_subkey(&company) {
            Ok(company_key) => Some((company, company_key)),
            Err(err) => {
                warn!(
                    "Failed to open {}\\Software\\Python\\{}, {:?}",
                    name, company, err
                );
                None
            }
        })
        .collect();

    let results: Vec<(CompanyWalkOutcome, bool)> = thread::scope(|s| {
        let handles: Vec<_> = companies
            .into_iter()
            .map(|(company, company_key)| {
                // Build the panic-warning label up-front so a panicking
                // company thread is identifiable in logs (issue #454).
                let label = format!("{name}\\Software\\Python\\{company}");
                let handle = s.spawn(move || {
                    // Trace order is intentionally relaxed: companies are
                    // walked in parallel, so this line interleaves with the
                    // others from the same hive.
                    trace!("Searching {}\\Software\\Python\\{}", name, company);
                    get_registry_pythons_from_key_for_company(
                        name,
                        &company_key,
                        &company,
                        conda_locator,
                        reporter,
                    )
                });
                (label, handle)
            })
            .collect();
        handles
            .into_iter()
            .map(|(label, h)| join_or_warn(h.join(), &label))
            .collect()
    });

    let mut environments = vec![];
    let mut managers = vec![];
    let mut conda_install_dirs = vec![];
    let mut had_panic = false;
    for (outcome, panicked) in results {
        environments.extend(outcome.result.environments);
        managers.extend(outcome.result.managers);
        conda_install_dirs.extend(outcome.conda_install_dirs);
        had_panic |= panicked;
    }
    RegistryWalkOutcome {
        result: LocatorResult {
            environments,
            managers,
        },
        conda_install_dirs,
        had_panic,
    }
}

#[cfg(windows)]
fn get_registry_pythons_from_key_for_company(
    key_container: &str,
    company_key: &RegKey,
    company: &str,
    conda_locator: &Arc<dyn CondaLocator>,
    reporter: &Option<&dyn Reporter>,
) -> CompanyWalkOutcome {
    use log::{trace, warn};
    use pet_conda::utils::is_conda_env;
    use pet_fs::path::norm_case;

    let mut environments = vec![];
    let mut conda_install_dirs = vec![];
    // let company_display_name: Option<String> = company_key.get_value("DisplayName").ok();
    for installed_python in company_key.enum_keys().filter_map(Result::ok) {
        match company_key.open_subkey(installed_python.clone()) {
            Ok(installed_python_key) => {
                match installed_python_key.open_subkey("InstallPath") {
                    Ok(install_path_key) => {
                        let env_path: String =
                            install_path_key.get_value("").ok().unwrap_or_default();
                        if env_path.is_empty() {
                            warn!(
                                "Install path is empty {}\\Software\\Python\\{}\\{}",
                                key_container, company, installed_python
                            );
                            continue;
                        }
                        let env_path = norm_case(PathBuf::from(env_path));
                        if is_windows_app_folder_in_program_files(&env_path) {
                            trace!(
                                "Found Python ({}) in {}\\Software\\Python\\{}\\{}, but skipping as this is a Windows Store Python",
                                env_path.to_str().unwrap_or_default(),
                                key_container,
                                company,
                                installed_python,
                            );
                            continue;
                        }
                        trace!(
                            "Found Python ({}) in {}\\Software\\Python\\{}\\{}",
                            env_path.to_str().unwrap_or_default(),
                            key_container,
                            company,
                            installed_python,
                        );

                        // Possible this is a conda install folder.
                        if is_conda_env(&env_path) {
                            if let Some(reporter) = reporter {
                                conda_locator.find_and_report(*reporter, &env_path);
                            }
                            // Capture the dir even when no reporter is
                            // attached (e.g. cache-warming via
                            // `find_with_cache(None)`) so a later cache
                            // hit in `find()` can replay
                            // `conda_locator.find_and_report` against
                            // each dir; without this, registry-only conda
                            // installs would silently disappear from
                            // every refresh after the first (#454).
                            conda_install_dirs.push(env_path);
                            continue;
                        }

                        let env_path = if env_path.exists() {
                            Some(env_path)
                        } else {
                            None
                        };
                        let executable: String = install_path_key
                            .get_value("ExecutablePath")
                            .ok()
                            .unwrap_or_default();
                        if executable.is_empty() {
                            warn!(
                                "Executable is empty {}\\Software\\Python\\{}\\{}\\ExecutablePath",
                                key_container, company, installed_python
                            );
                            continue;
                        }
                        let executable = norm_case(PathBuf::from(executable));
                        if !executable.exists() {
                            warn!(
                                "Python executable ({}) file not found for {}\\Software\\Python\\{}\\{}",
                                executable.to_str().unwrap_or_default(),
                                key_container,
                                company,
                                installed_python
                            );
                            continue;
                        }
                        let version: String = installed_python_key
                            .get_value("Version")
                            .ok()
                            .unwrap_or_default();
                        let architecture: String = installed_python_key
                            .get_value("SysArchitecture")
                            .ok()
                            .unwrap_or_default();
                        let display_name: String = installed_python_key
                            .get_value("DisplayName")
                            .ok()
                            .unwrap_or_default();

                        let env = PythonEnvironmentBuilder::new(Some(
                            PythonEnvironmentKind::WindowsRegistry,
                        ))
                        .display_name(Some(display_name))
                        .executable(Some(executable.clone()))
                        .version(if version.is_empty() {
                            None
                        } else {
                            Some(version)
                        })
                        .prefix(env_path)
                        .arch(if architecture.contains("32") {
                            Some(Architecture::X86)
                        } else if architecture.contains("64") {
                            Some(Architecture::X64)
                        } else {
                            None
                        })
                        .build();

                        if let Some(reporter) = reporter {
                            reporter.report_environment(&env);
                        }
                        environments.push(env);
                    }
                    Err(err) => {
                        warn!(
                            "Failed to open {}\\Software\\Python\\{}\\{}\\InstallPath, {:?}",
                            key_container, company, installed_python, err
                        );
                    }
                }
            }
            Err(err) => {
                warn!(
                    "Failed to open {}\\Software\\Python\\{}\\{}, {:?}",
                    key_container, company, installed_python, err
                );
            }
        }
    }

    CompanyWalkOutcome {
        result: LocatorResult {
            environments,
            managers: vec![],
        },
        conda_install_dirs,
    }
}
