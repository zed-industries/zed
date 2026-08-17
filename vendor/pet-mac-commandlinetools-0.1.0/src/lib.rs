// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::{
    arch::Architecture,
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_fs::path::resolve_symlink;
use pet_python_utils::macos::{
    add_macos_system_python_alias, is_macos_system_python, resolve_macos_system_python_env,
};
use pet_python_utils::version;
use pet_python_utils::{env::ResolvedPythonEnv, executable::find_executables};
use pet_virtualenv::is_virtualenv;
use std::path::{Path, PathBuf};

/// Returns `true` when `name` is `python`, `python3`, or `python3.<minor>`
/// (where minor is one or more ASCII digits).
fn is_macos_python_executable_name(name: &str) -> bool {
    if name == "python" || name == "python3" {
        return true;
    }
    if let Some(rest) = name.strip_prefix("python3.") {
        return !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit());
    }
    false
}

/// Returns `true` when `dir` is `"Current"` or a `<major>.<minor>` pair of
/// ASCII digits (e.g. `"3.9"`).
fn is_valid_framework_version_dir(dir: &str) -> bool {
    if dir == "Current" {
        return true;
    }
    match dir.split_once('.') {
        Some((major, minor)) => {
            !major.is_empty()
                && major.chars().all(|c| c.is_ascii_digit())
                && !minor.is_empty()
                && minor.chars().all(|c| c.is_ascii_digit())
        }
        None => false,
    }
}

/// Checks whether the given path (as a pre-computed lossy string) is a valid
/// Command Line Tools Python executable path.
///
/// Accepted shapes:
///   /Library/Developer/CommandLineTools/usr/bin/<python-exe>
///   /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/<ver>/bin/<python-exe>
fn is_cmdlinetools_python_path(path_str: &str) -> bool {
    let path = Path::new(path_str);

    let exe_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return false,
    };

    if !is_macos_python_executable_name(exe_name) {
        return false;
    }

    // Shape 1: /Library/Developer/CommandLineTools/usr/bin/<exe>
    if let Ok(rest) = path.strip_prefix("/Library/Developer/CommandLineTools/usr/bin") {
        return rest.components().count() == 1;
    }

    // Shape 2: .../Python3.framework/Versions/<ver>/bin/<exe>
    if let Ok(rest) = path.strip_prefix(
        "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions",
    ) {
        let components: Vec<_> = rest.components().collect();
        // Expect exactly 3 components: <version>, "bin", <exe>
        if components.len() != 3 {
            return false;
        }
        let version_dir = components[0].as_os_str().to_string_lossy();
        let bin_dir = components[1].as_os_str().to_string_lossy();
        return is_valid_framework_version_dir(&version_dir) && bin_dir == "bin";
    }

    false
}

pub struct MacCmdLineTools {}

impl MacCmdLineTools {
    pub fn new() -> MacCmdLineTools {
        MacCmdLineTools {}
    }
}
impl Default for MacCmdLineTools {
    fn default() -> Self {
        Self::new()
    }
}
impl Locator for MacCmdLineTools {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::MacCommandLineTools
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::MacCommandLineTools]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if std::env::consts::OS != "macos" {
            return None;
        }

        let resolved_system_alias = if is_macos_system_python(&env.executable) {
            Some(resolve_macos_system_python_env(env)?)
        } else {
            None
        };
        let env = resolved_system_alias.as_ref().unwrap_or(env);
        // Assume we create a virtual env from a python install,
        // Then the exe in the virtual env bin will be a symlink to the homebrew python install.
        // Hence the first part of the condition will be true, but the second part will be false.
        if is_virtualenv(env) {
            return None;
        }

        let exe_str = env.executable.to_string_lossy();
        if !is_cmdlinetools_python_path(&exe_str) {
            return None;
        }

        let mut version = env.version.clone();
        let mut prefix = env.prefix.clone();
        let mut symlinks = vec![env.executable.clone()];
        let mut arch = None;

        let existing_symlinks = env.symlinks.clone();
        if let Some(existing_symlinks) = existing_symlinks {
            symlinks.append(&mut existing_symlinks.clone());
        }

        // We know that /Library/Developer/CommandLineTools/usr/bin/python3 is actually a symlink to
        // /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        // Verify this and add that to the list of symlinks as well.
        if let Some(symlink) = resolve_symlink(&env.executable) {
            symlinks.push(symlink);
        }

        // Possible we got the file /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        // We know that /Library/Developer/CommandLineTools/usr/bin/python3 is a symlink to the above.
        if env
            .executable
            .starts_with("/Library/Developer/CommandLineTools/usr/bin")
        {
            let exe = PathBuf::from("/Library/Developer/CommandLineTools/usr/bin/python3");
            if let Some(symlink) = resolve_symlink(&exe) {
                if symlinks.contains(&symlink) {
                    symlinks.push(symlink);

                    // Rest of the files in this directory are also symlinks to the same exe.
                    for exe in find_executables(PathBuf::from(
                        "/Library/Developer/CommandLineTools/usr/bin",
                    )) {
                        if !symlinks.contains(&exe) {
                            if let Some(symlink) = resolve_symlink(&exe) {
                                if symlinks.contains(&symlink) {
                                    symlinks.push(exe);
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut resolved_environments = vec![];

        // Similarly the final exe can be /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        // & we might have another file `python3` in that bin directory which would point to the same exe.
        // Lets get those as well.
        if let Some(real_exe) = symlinks.iter().find(|s| {
            s.to_string_lossy()
                .contains("/Library/Developer/CommandLineTools/Library/Frameworks")
        }) {
            let python3 = real_exe.with_file_name("python3");
            if !symlinks.contains(&python3) {
                if let Some(symlink) = resolve_symlink(&python3) {
                    if symlinks.contains(&symlink) {
                        symlinks.push(python3);
                    }
                }
            }
        }

        add_macos_system_python_alias(&mut symlinks);
        symlinks.sort();
        symlinks.dedup();

        // Find other exes that are symlinks to the same exe in /Library/Developer/CommandLineTools/usr/bin
        for exe in find_executables("/Library/Developer/CommandLineTools/usr/bin") {
            if !symlinks.contains(&exe) {
                if let Some(symlink) = resolve_symlink(&exe) {
                    if symlinks.contains(&symlink) {
                        symlinks.push(exe);
                    }
                }
            }
        }

        if prefix.is_none() {
            // We would have identified the symlinks by now.
            // Look for the one with the path `/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9`
            if let Some(symlink) = symlinks.iter().find(|s| {
                s.to_string_lossy().starts_with("/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions")
            }) {
                // Prefix is of the form `/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9`
                // The symlink would be the same, all we need is to remove the last 2 components (exe and bin directory).
                prefix = symlink.parent()?.parent().map(|p| p.to_path_buf());
            }
        }

        if version.is_none() {
            if let Some(prefix) = &prefix {
                version = version::from_header_files(prefix);
            }
        }

        if version.is_none() || prefix.is_none() {
            if let Some(resolved_env) = ResolvedPythonEnv::from(&env.executable) {
                resolved_environments.push(resolved_env.clone());
                version = Some(resolved_env.version);
                prefix = Some(resolved_env.prefix);
                arch = if resolved_env.is64_bit {
                    Some(Architecture::X64)
                } else {
                    Some(Architecture::X86)
                };
            }
        }

        let env = PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::MacCommandLineTools))
            .executable(Some(env.executable.clone()))
            .version(version)
            .prefix(prefix)
            .arch(arch)
            .symlinks(Some(symlinks.clone()))
            .build();

        // If we had spawned Python, then ensure we cache the details.
        // We do this here, to ensure we keep track of the symlinks as well,
        // I.e. if any of the symlinks change, then the cache is invalidated.
        for resolved_env in resolved_environments {
            resolved_env.add_to_cache(env.clone());
        }

        Some(env)
    }

    fn find(&self, _reporter: &dyn Reporter) {
        // We will end up looking in current PATH variable
        // Given thats done else where, lets not repeat it here.
        if std::env::consts::OS != "macos" {
            return;
        }

        for exe in find_executables("/Library/Developer/CommandLineTools/usr")
            .iter()
            .filter(
                |f|                     // If this file name is `python3`, then ignore this for now.
            // We would prefer to use `python3.x` instead of `python3`.
            // That way its more consistent and future proof
                f.file_name().unwrap_or_default() != "python3" &&
                f.file_name().unwrap_or_default() != "python",
            )
        {
            // These files should end up being symlinks to something like /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
            let mut env = PythonEnv::new(exe.to_owned(), None, None);
            let mut symlinks = vec![];
            if let Some(symlink) = resolve_symlink(exe) {
                // Symlinks must exist, they always point to something like the following
                // /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
                symlinks.push(symlink);
            }

            // Also check whether the corresponding python and python3 files in this directory point to the same files.
            for python_exe in &["python", "python3"] {
                let python_exe = exe.with_file_name(python_exe);
                if let Some(symlink) = resolve_symlink(&python_exe) {
                    if symlinks.contains(&symlink) {
                        symlinks.push(python_exe);
                    }
                }
            }
            env.symlinks = Some(symlinks);
            if let Some(env) = self.try_from(&env) {
                _reporter.report_environment(&env);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::{python_environment::PythonEnvironmentKind, Locator, LocatorKind};

    // ── locator metadata ──────────────────────────────────────────

    #[test]
    fn locator_metadata_matches_cmdlinetools_kind() {
        let loc = MacCmdLineTools::new();
        assert_eq!(loc.get_kind(), LocatorKind::MacCommandLineTools);
        assert_eq!(
            loc.supported_categories(),
            vec![PythonEnvironmentKind::MacCommandLineTools]
        );
    }

    // ── is_macos_python_executable_name ───────────────────────────

    #[test]
    fn exe_name_accepts_python() {
        assert!(is_macos_python_executable_name("python"));
    }

    #[test]
    fn exe_name_accepts_python3() {
        assert!(is_macos_python_executable_name("python3"));
    }

    #[test]
    fn exe_name_accepts_python3_minor() {
        assert!(is_macos_python_executable_name("python3.9"));
        assert!(is_macos_python_executable_name("python3.12"));
    }

    #[test]
    fn exe_name_rejects_config_script() {
        assert!(!is_macos_python_executable_name("python3-config"));
        assert!(!is_macos_python_executable_name("python3.9-config"));
    }

    #[test]
    fn exe_name_rejects_non_python_tools() {
        assert!(!is_macos_python_executable_name("idle3"));
        assert!(!is_macos_python_executable_name("pydoc3"));
        assert!(!is_macos_python_executable_name("pip3"));
    }

    #[test]
    fn exe_name_rejects_compact_version() {
        assert!(!is_macos_python_executable_name("python39"));
    }

    #[test]
    fn exe_name_rejects_multi_dot_version() {
        assert!(!is_macos_python_executable_name("python3.9.1"));
    }

    #[test]
    fn exe_name_rejects_trailing_dot() {
        assert!(!is_macos_python_executable_name("python3."));
    }

    // ── is_valid_framework_version_dir ────────────────────────────

    #[test]
    fn version_dir_accepts_current() {
        assert!(is_valid_framework_version_dir("Current"));
    }

    #[test]
    fn version_dir_accepts_major_minor() {
        assert!(is_valid_framework_version_dir("3.9"));
        assert!(is_valid_framework_version_dir("3.12"));
    }

    #[test]
    fn version_dir_rejects_patch_version() {
        assert!(!is_valid_framework_version_dir("3.9.1"));
    }

    #[test]
    fn version_dir_rejects_bare_major() {
        assert!(!is_valid_framework_version_dir("3"));
    }

    #[test]
    fn version_dir_rejects_empty() {
        assert!(!is_valid_framework_version_dir(""));
    }

    #[test]
    fn version_dir_rejects_dot_only() {
        assert!(!is_valid_framework_version_dir("."));
        assert!(!is_valid_framework_version_dir("3."));
        assert!(!is_valid_framework_version_dir(".9"));
    }

    // ── is_cmdlinetools_python_path ───────────────────────────────

    #[test]
    fn cmdlinetools_path_accepts_usr_bin_python3() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/python3"
        ));
    }

    #[test]
    fn cmdlinetools_path_accepts_usr_bin_python3_versioned() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/python3.9"
        ));
    }

    #[test]
    fn cmdlinetools_path_accepts_usr_bin_python() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/python"
        ));
    }

    #[test]
    fn cmdlinetools_path_accepts_framework_versioned_python() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9"
        ));
    }

    #[test]
    fn cmdlinetools_path_accepts_framework_current_version() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/Current/bin/python3"
        ));
    }

    #[test]
    fn cmdlinetools_path_accepts_framework_bare_python() {
        assert!(is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_empty_string() {
        assert!(!is_cmdlinetools_python_path(""));
    }

    #[test]
    fn cmdlinetools_path_rejects_config_script() {
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/python3-config"
        ));
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/python3.9-config"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_non_python_tools() {
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/idle3"
        ));
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/pydoc3"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_framework_invalid_version() {
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9.1/bin/python3.9"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_framework_nested_path() {
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/subdir/python3.9"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_unrelated_path() {
        assert!(!is_cmdlinetools_python_path("/usr/bin/python3"));
        assert!(!is_cmdlinetools_python_path("/usr/local/bin/python3"));
        assert!(!is_cmdlinetools_python_path("/opt/homebrew/bin/python3.11"));
    }

    #[test]
    fn cmdlinetools_path_rejects_xcode_path() {
        assert!(!is_cmdlinetools_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn cmdlinetools_path_rejects_usr_bin_nested_deeper() {
        assert!(!is_cmdlinetools_python_path(
            "/Library/Developer/CommandLineTools/usr/bin/subdir/python3"
        ));
    }

    #[test]
    fn try_from_rejects_cmdlinetools_path_off_macos() {
        // On non-macOS the locator always returns None regardless of path.
        if std::env::consts::OS == "macos" {
            return; // skip on macOS — this test is for other platforms
        }
        let loc = MacCmdLineTools::new();
        let env = PythonEnv::new(
            PathBuf::from("/Library/Developer/CommandLineTools/usr/bin/python3.9"),
            None,
            None,
        );
        assert!(loc.try_from(&env).is_none());
    }
}
