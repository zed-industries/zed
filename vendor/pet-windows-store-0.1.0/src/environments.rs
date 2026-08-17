// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#[cfg(windows)]
use crate::env_variables::EnvVariables;
#[cfg(windows)]
use lazy_static::lazy_static;
#[cfg(windows)]
use log::warn;
#[cfg(windows)]
use pet_core::python_environment::PythonEnvironment;
#[cfg(windows)]
use pet_core::{arch::Architecture, python_environment::PythonEnvironmentBuilder};
#[cfg(windows)]
use pet_fs::path::norm_case;
#[cfg(windows)]
use pet_python_utils::executable::find_executables;
#[cfg(windows)]
use regex::Regex;
use std::path::PathBuf;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
lazy_static! {
    static ref PYTHON_SOFTWARE_FOUNDATION_FOLDER_VERSION: Regex = Regex::new(
        "PythonSoftwareFoundation.Python.(\\d+\\.\\d+)_.*"
    )
    .expect("error parsing Version regex for Python Software Foundation Version in Windows Store");
    static ref PYTHON_VERSION: Regex = Regex::new("python(\\d+\\.\\d+).exe")
        .expect("error parsing Version regex for Python Version in Windows Store");
}

#[derive(Default)]
#[allow(dead_code)]
struct PotentialPython {
    #[allow(dead_code)]
    path: Option<PathBuf>,
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    exe: Option<PathBuf>,
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    symlinks: Vec<PathBuf>,
}

#[cfg(windows)]
impl PotentialPython {
    fn to_python_environment(&self, hkcu: &RegKey) -> Option<PythonEnvironment> {
        let name = self.name.clone().unwrap_or_default();
        let path = self.path.clone().unwrap_or_default();
        let exe = self.exe.clone().unwrap_or_default();
        let parent = path.parent()?.to_path_buf(); // This dir definitely exists.
        if let Some(result) = get_package_display_name_and_location(&name, hkcu) {
            let env_path = norm_case(PathBuf::from(result.env_path));

            // Build the base symlinks list
            // parent = WindowsApps folder (e.g., C:\Users\...\AppData\Local\Microsoft\WindowsApps)
            // path = Package folder inside WindowsApps (e.g., WindowsApps\PythonSoftwareFoundation.Python.3.12_...)
            // env_path = Program Files location (e.g., C:\Program Files\WindowsApps\PythonSoftwareFoundation...)
            let mut symlinks = vec![
                // Symlinks in the user WindowsApps folder
                parent.join(format!("python{}.exe", self.version)),
                parent.join("python3.exe"),
                parent.join("python.exe"),
                // Symlinks in the package subfolder under user WindowsApps
                path.join("python.exe"),
                path.join("python3.exe"),
                path.join(format!("python{}.exe", self.version)),
                // Symlinks in Program Files
                env_path.join("python.exe"),
                env_path.join("python3.exe"),
                env_path.join(format!("python{}.exe", self.version)),
            ];

            // Add symlinks discovered by find_symlinks (includes python.exe and python3.exe
            // from WindowsApps when there's only one Python version installed)
            for symlink in &self.symlinks {
                if !symlinks.contains(symlink) {
                    symlinks.push(symlink.clone());
                }
            }

            Some(
                PythonEnvironmentBuilder::new(Some(
                    pet_core::python_environment::PythonEnvironmentKind::WindowsStore,
                ))
                .display_name(Some(result.display_name))
                .executable(Some(exe.clone()))
                .prefix(Some(env_path.clone()))
                .arch(if result.is64_bit {
                    Some(Architecture::X64)
                } else {
                    None
                })
                // We only have the partial version, no point returning bogus info.
                // .version(Some(self.version.clone()))
                .symlinks(Some(symlinks))
                .build(),
            )
        } else {
            warn!(
                "Failed to get package display name & location for Windows Store Package {:?}",
                path
            );
            None
        }
    }
}

#[cfg(windows)]
pub fn list_store_pythons(environment: &EnvVariables) -> Option<Vec<PythonEnvironment>> {
    use crate::environment_locations::get_search_locations;
    use log::{trace, warn};
    use std::collections::HashMap;

    let mut python_envs: Vec<PythonEnvironment> = vec![];
    let apps_path = get_search_locations(environment)?;
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    trace!("Searching for Windows Store Python in {:?}", apps_path);

    let mut potential_matches: HashMap<String, PotentialPython> = HashMap::new();
    for path in std::fs::read_dir(apps_path)
        .ok()?
        .filter_map(Result::ok)
        .map(|f| f.path())
    {
        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy().to_string();
            if name.starts_with("PythonSoftwareFoundation.Python.") {
                let simple_version = PYTHON_SOFTWARE_FOUNDATION_FOLDER_VERSION.captures(&name)?;
                let simple_version = simple_version
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap_or_default();
                if simple_version.is_empty() {
                    continue;
                }
                if let Some(existing) = potential_matches.get_mut(simple_version) {
                    existing.path = Some(path.clone());
                    existing.name = Some(name.clone());
                } else {
                    let item = PotentialPython {
                        path: Some(path.clone()),
                        name: Some(name.clone()),
                        version: simple_version.to_string(),
                        symlinks: find_symlinks(path, simple_version.to_string()),
                        ..Default::default()
                    };
                    potential_matches.insert(simple_version.to_string(), item);
                }
            } else if name.starts_with("python") && name.ends_with(".exe") {
                if name == "python.exe" || name == "python3.exe" {
                    // Unfortunately we have no idea what these point to.
                    // Even old python code didn't report these, hopefully users will not use these.
                    // If they do, we might have to spawn Python to find the real path and match it to one of the items discovered.
                    continue;
                }
                if let Some(simple_version) = PYTHON_VERSION.captures(&name) {
                    let simple_version = simple_version
                        .get(1)
                        .map(|m| m.as_str())
                        .unwrap_or_default();
                    if simple_version.is_empty() {
                        continue;
                    }
                    if let Some(existing) = potential_matches.get_mut(simple_version) {
                        existing.exe = Some(path.clone());
                    } else {
                        let item = PotentialPython {
                            exe: Some(path.clone()),
                            version: simple_version.to_string(),
                            symlinks: find_symlinks(path, simple_version.to_string()),
                            ..Default::default()
                        };
                        potential_matches.insert(simple_version.to_string(), item);
                    }
                }
            }
        }
    }

    for (_, item) in potential_matches {
        if item.exe.is_none() {
            warn!(
                "Did not find a Windows Store exe for version {:?} that coresponds to path {:?}",
                item.version, item.path
            );
            continue;
        }
        if item.path.is_none() {
            warn!(
                "Did not find a Windows Store path for version {:?} that coresponds to exe {:?}",
                item.version, item.exe
            );
            continue;
        }
        if let Some(env) = item.to_python_environment(&hkcu) {
            python_envs.push(env);
        }
    }
    Some(python_envs)
}

/// Given an exe from a sub directory of WindowsApp path, find the symlinks (reparse points)
/// for the same environment but from the WindowsApp directory.
#[cfg(windows)]
fn find_symlinks(exe_in_windows_app_path: PathBuf, version: String) -> Vec<PathBuf> {
    let mut symlinks = vec![];
    if let Some(bin_dir) = exe_in_windows_app_path.parent() {
        if let Some(windows_app_path) = bin_dir.parent() {
            // Ensure we're in the right place
            if windows_app_path.ends_with("WindowsApp") {
                return vec![];
            }

            let possible_exe =
                windows_app_path.join(PathBuf::from(format!("python{}.exe", version)));
            if possible_exe.exists() {
                symlinks.push(possible_exe);
            }

            // How many exes do we have that look like with Python3.x.exe
            // If we have Python3.12.exe & Python3.10.exe, then we have absolutely no idea whether
            // the exes Python3.exe and Python.exe belong to 3.12 or 3.10 without spawning.
            // In those cases we will not bother figuring those out.
            // However if we have just one Python exe of the form Python3.x.ex, then python.exe and Python3.exe are symlinks.
            let mut number_of_python_exes_with_versions = 0;
            let mut exes = vec![];
            find_executables(windows_app_path)
                .into_iter()
                .for_each(|exe| {
                    if let Some(name) = exe.file_name().and_then(|s| s.to_str()) {
                        if name.to_lowercase().starts_with("python3.") {
                            number_of_python_exes_with_versions += 1;
                        }
                        exes.push(exe);
                    }
                });

            if number_of_python_exes_with_versions == 1 {
                symlinks.append(&mut exes);
            }
        }
    }
    symlinks
}

#[cfg(windows)]
#[derive(Debug)]
struct StorePythonInfo {
    display_name: String,
    env_path: String,
    is64_bit: bool,
}

#[cfg(windows)]
fn get_package_display_name_and_location(name: &String, hkcu: &RegKey) -> Option<StorePythonInfo> {
    use log::trace;

    if let Some(name) = get_package_full_name_from_registry(name, hkcu) {
        let key = format!("Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\Repository\\Packages\\{}", name);
        trace!("Opening registry key {:?}", key);
        let package_key = hkcu.open_subkey(key).ok()?;
        let display_name = package_key.get_value("DisplayName").ok()?;
        let env_path = package_key.get_value("PackageRootFolder").ok()?;

        return Some(StorePythonInfo {
            display_name,
            env_path,
            is64_bit: name.contains("_x64_"),
        });
    }
    None
}

#[cfg(windows)]
fn get_package_full_name_from_registry(name: &String, hkcu: &RegKey) -> Option<String> {
    use log::trace;

    let key = format!("Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\SystemAppData\\{}\\Schemas", name);
    trace!("Opening registry key {:?}", key);
    let package_key = hkcu.open_subkey(key).ok()?;
    let value = package_key.get_value("PackageFullName").ok()?;
    Some(value)
}
