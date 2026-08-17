// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::{
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_fs::path::resolve_symlink;
use pet_python_utils::executable::find_executables;
use pet_python_utils::version;
use pet_virtualenv::is_virtualenv;
use std::fs;
use std::path::PathBuf;

pub struct MacPythonOrg {}

impl MacPythonOrg {
    pub fn new() -> MacPythonOrg {
        MacPythonOrg {}
    }
}
impl Default for MacPythonOrg {
    fn default() -> Self {
        Self::new()
    }
}
impl Locator for MacPythonOrg {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::MacPythonOrg
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::MacPythonOrg]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if std::env::consts::OS != "macos" {
            return None;
        }
        // Assume we create a virtual env from a python install,
        // Then the exe in the virtual env bin will be a symlink to the homebrew python install.
        // Hence the first part of the condition will be true, but the second part will be false.
        if is_virtualenv(env) {
            return None;
        }

        let mut executable = resolve_symlink(&env.executable).unwrap_or(env.executable.clone());
        if !is_mac_python_org_framework_path(&executable) {
            return None;
        }

        let mut version_is_current = false;
        let mut symlinks = vec![executable.clone(), env.executable.clone()];
        if executable.starts_with("/Library/Frameworks/Python.framework/Versions/Current") {
            // This is a symlink to the python executable, lets resolve it
            let exe_to_resolve =
                "/Library/Frameworks/Python.framework/Versions/Current/bin/python3";
            if let Some(exe) = resolve_symlink(&exe_to_resolve) {
                if exe.starts_with("/Library/Frameworks/Python.framework/Versions")
                    && !exe.starts_with("/Library/Frameworks/Python.framework/Versions/Current")
                {
                    // Given that the exe we were given is the `Current/bin/python`, we know this is current.
                    version_is_current = true;
                    symlinks.push(exe.clone());
                    symlinks.push(PathBuf::from(exe_to_resolve));
                    executable = exe;
                }
            }
        } else {
            // Check if this is the current version.
            let exe_to_resolve =
                "/Library/Frameworks/Python.framework/Versions/Current/bin/python3";
            if let Some(exe) = resolve_symlink(&exe_to_resolve) {
                if exe == executable {
                    // Yes, this is the current version
                    version_is_current = true;
                    symlinks.push(exe.clone());
                    symlinks.push(PathBuf::from(exe_to_resolve));
                }
            }
        }

        let prefix = executable.parent()?.parent()?;
        let version = version::from_header_files(prefix)?;

        // Also look for other python* files in the same directory as the above executable
        for exe in find_executables(executable.parent()?) {
            if symlinks.contains(&exe) {
                continue;
            }
            if let Some(symlink) = resolve_symlink(&exe) {
                if symlinks.contains(&symlink) {
                    symlinks.push(exe);
                }
            }
        }

        // We know files in /usr/local/bin & /Library/Frameworks/Python.framework/Versions/Current/bin end up being symlinks to this python exe as well
        // Documented here https://docs.python.org/3/using/mac.html
        // Hence look for those symlinks as well.
        for bin in [
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/Library/Frameworks/Python.framework/Versions/Current/bin"),
        ] {
            for file in find_executables(&bin) {
                // If we're looking in the `Current/bin`, then no need to resolve symlinks
                // As we already know this is the current version.
                // Note: We can resolve the symlink for /Library/Frameworks/Python.framework/Versions/Current/bin/python3
                // However in rust for some reason we cannot resolve the symlink for /Library/Frameworks/Python.framework/Versions/Current/bin/python3.10
                if version_is_current
                    && file.starts_with("/Library/Frameworks/Python.framework/Versions/Current/bin")
                {
                    symlinks.push(file);
                    continue;
                }
                if let Some(symlink) = resolve_symlink(&file) {
                    if symlinks.contains(&symlink) {
                        symlinks.push(file);
                    }
                }
            }
        }

        symlinks.sort();
        symlinks.dedup();

        Some(
            PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::MacPythonOrg))
                .executable(Some(executable.clone()))
                .version(Some(version))
                .prefix(Some(prefix.to_path_buf()))
                .symlinks(Some(symlinks))
                .build(),
        )
    }

    fn find(&self, reporter: &dyn Reporter) {
        if std::env::consts::OS != "macos" {
            return;
        }

        if let Ok(reader) = fs::read_dir("/Library/Frameworks/Python.framework/Versions/") {
            for file in reader.filter_map(Result::ok) {
                let prefix = file.path();
                // Ignore the `/Library/Frameworks/Python.framework/Versions/Current` folder, as this only contains symlinks to the actual python installations
                // We will account for the symlinks in these folder later
                if prefix
                    .to_string_lossy()
                    .starts_with("/Library/Frameworks/Python.framework/Versions/Current")
                {
                    continue;
                }

                let executable = prefix.join("bin").join("python3");
                let version = version::from_header_files(&prefix);

                if let Some(env) = self.try_from(&PythonEnv::new(executable, Some(prefix), version))
                {
                    reporter.report_environment(&env);
                }
            }
        }
    }
}

fn is_mac_python_org_framework_path(executable: &std::path::Path) -> bool {
    let Ok(framework_entry) =
        executable.strip_prefix("/Library/Frameworks/Python.framework/Versions")
    else {
        return false;
    };

    let mut framework_parts = framework_entry.components();
    matches!(
        framework_parts.next(),
        Some(std::path::Component::Normal(version))
            if version.to_str().is_some_and(is_macos_framework_version_dir)
    ) && matches!(
        framework_parts.next(),
        Some(std::path::Component::Normal(part)) if part == std::ffi::OsStr::new("bin")
    ) && matches!(
        framework_parts.next(),
        Some(std::path::Component::Normal(executable_name))
            if executable_name.to_str().is_some_and(is_macos_python_executable_name)
    ) && framework_parts.next().is_none()
}

fn is_macos_python_executable_name(executable: &str) -> bool {
    if executable == "python" || executable == "python3" {
        return true;
    }

    let Some(minor) = executable.strip_prefix("python3.") else {
        return false;
    };

    !minor.is_empty() && minor.chars().all(|ch| ch.is_ascii_digit())
}

fn is_macos_framework_version_dir(version: &str) -> bool {
    if version == "Current" {
        return true;
    }

    let mut parts = version.split('.');
    parts
        .next()
        .is_some_and(|major| !major.is_empty() && major.chars().all(|ch| ch.is_ascii_digit()))
        && parts
            .next()
            .is_some_and(|minor| !minor.is_empty() && minor.chars().all(|ch| ch.is_ascii_digit()))
        && parts.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::Locator;
    use std::path::Path;

    #[test]
    fn locator_metadata_matches_python_org_kind() {
        let locator = MacPythonOrg::new();

        assert_eq!(locator.get_kind(), LocatorKind::MacPythonOrg);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::MacPythonOrg]
        );
    }

    #[test]
    fn framework_path_accepts_versioned_python3() {
        assert!(is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3"
        )));
    }

    #[test]
    fn framework_path_accepts_unversioned_python() {
        assert!(is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python"
        )));
    }

    #[test]
    fn framework_path_accepts_versioned_python_executable() {
        assert!(is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3.12"
        )));
    }

    #[test]
    fn framework_path_accepts_current_python() {
        assert!(is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/Current/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_non_python_file() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/Resources/Info.plist"
        )));
    }

    #[test]
    fn framework_path_rejects_python_config_script() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python-config"
        )));
    }

    #[test]
    fn framework_path_rejects_versioned_python_config_script() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3.12-config"
        )));
    }

    #[test]
    fn framework_path_rejects_patch_version_python_name() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3.12.0"
        )));
    }

    #[test]
    fn framework_path_rejects_compact_version_python_name() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python312"
        )));
    }

    #[test]
    fn framework_path_rejects_python2_name() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python2"
        )));
    }

    #[test]
    fn framework_path_rejects_patch_version_dir() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12.0/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_invalid_version_dir() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/Foo/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_other_framework() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Other.framework/Versions/3.12/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_non_library_path() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/tmp/Python.framework/Versions/3.12/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_homebrew_framework_path() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/opt/homebrew/Cellar/python@3.12/3.12.1/Frameworks/Python.framework/Versions/3.12/bin/python3"
        )));
    }

    #[test]
    fn framework_path_rejects_nested_bin_entry() {
        assert!(!is_mac_python_org_framework_path(Path::new(
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/nested/python3"
        )));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn try_from_rejects_python_org_path_off_macos() {
        let locator = MacPythonOrg::new();
        let env = PythonEnv::new(
            PathBuf::from("/Library/Frameworks/Python.framework/Versions/3.12/bin/python3"),
            Some(PathBuf::from(
                "/Library/Frameworks/Python.framework/Versions/3.12",
            )),
            Some("3.12.0".to_string()),
        );

        assert!(locator.try_from(&env).is_none());
    }
}
