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
use std::path::PathBuf;

pub struct MacXCode {}

impl MacXCode {
    pub fn new() -> MacXCode {
        MacXCode {}
    }
}
impl Default for MacXCode {
    fn default() -> Self {
        Self::new()
    }
}
impl Locator for MacXCode {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::MacXCode
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::MacXCode]
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

        // Support for /Applications/Xcode.app/Contents/Developer/usr/bin/python3
        // /Applications/Xcode_15.0.1.app/Contents/Developer/usr/bin/python3 (such paths are on CI, see here https://github.com/microsoft/python-environment-tools/issues/38)
        if !is_xcode_python_path(&exe_str) {
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

        // We know that /Applications/Xcode.app/Contents/Developer/usr/bin/python3 is actually a symlink to
        // /Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        // Verify this and add that to the list of symlinks as well.
        if let Some(symlink) = resolve_symlink(&env.executable) {
            symlinks.push(symlink.clone());

            // All exes in the bin directory of the symlink are also symlinks (thats generally of the form /Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9)
            for exe in find_executables(symlink.parent().unwrap()) {
                symlinks.push(exe);
            }
        }

        // Possible the env.executable is "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9"
        // The symlink to the above exe is in /Applications/Xcode.app/Contents/Developer/usr/bin/python3
        // Lets try to find that, because /usr/bin/python3 could also exist and when we run python, the sys.execuctable points to the file /Applications/Xcode.app/Contents/Developer/usr/bin/python3
        // The name of the `Xcode.app` folder can be different on other machines, e.g. on CI it is `Xcode_15.0.1.app`
        let xcode_folder_name = exe_str.split('/').nth(2).unwrap_or_default();

        let bin = PathBuf::from(format!(
            "/Applications/{xcode_folder_name}/Contents/Developer/usr/bin"
        ));
        let exe = bin.join("python3");
        if let Some(symlink) = resolve_symlink(&exe) {
            if symlinks.contains(&symlink) {
                symlinks.push(exe.clone());

                // All exes in this directory are symlinks
                for exe in find_executables(bin) {
                    symlinks.push(exe);
                }
            }
        }

        let mut resolved_environments = vec![];

        // Similarly the final exe can be /Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        // & we might have another file `python3` in that bin directory which would point to the same exe.
        // Lets get those as well.
        if let Some(real_exe) = symlinks.iter().find(|s| {
            s.to_string_lossy()
                .contains("Contents/Developer/Library/Frameworks")
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

        // if prefix.is_none() {
        //     // We would have identified the symlinks by now.
        //     // Look for the one with the path `/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9`
        //     if let Some(symlink) = symlinks.iter().find(|s| {
        //         s.to_string_lossy().starts_with("/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions")
        //     }) {
        //         // Prefix is of the form `/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9`
        //         // The symlink would be the same, all we need is to remove the last 2 components (exe and bin directory).
        //         prefix = symlink.parent()?.parent().map(|p| p.to_path_buf());
        //     }
        // }

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

        let env = PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::MacXCode))
            .executable(Some(env.executable.clone()))
            .version(version)
            .prefix(prefix)
            .arch(arch)
            .symlinks(Some(symlinks))
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
        // if std::env::consts::OS != "macos" {
        //     return;
        // }

        // for exe in find_executables("/Library/Developer/CommandLineTools/usr")
        //     .iter()
        //     .filter(
        //         |f|                     // If this file name is `python3`, then ignore this for now.
        //     // We would prefer to use `python3.x` instead of `python3`.
        //     // That way its more consistent and future proof
        //         f.file_name().unwrap_or_default() != "python3" &&
        //         f.file_name().unwrap_or_default() != "python",
        //     )
        // {
        //     // These files should end up being symlinks to something like /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        //     let mut env = PythonEnv::new(exe.to_owned(), None, None);
        //     let mut symlinks = vec![];
        //     if let Some(symlink) = resolve_symlink(exe) {
        //         // Symlinks must exist, they always point to something like the following
        //         // /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9
        //         symlinks.push(symlink);
        //     }

        //     // Also check whether the corresponding python and python3 files in this directory point to the same files.
        //     for python_exe in &["python", "python3"] {
        //         let python_exe = exe.with_file_name(python_exe);
        //         if let Some(symlink) = resolve_symlink(&python_exe) {
        //             if symlinks.contains(&symlink) {
        //                 symlinks.push(python_exe);
        //             }
        //         }
        //     }
        //     env.symlinks = Some(symlinks);
        //     if let Some(env) = self.try_from(&env) {
        //         _reporter.report_environment(&env);
        //     }
        // }
    }
}

fn is_xcode_python_path(executable: &str) -> bool {
    let Some(rest) = executable.strip_prefix("/Applications/") else {
        return false;
    };

    let Some(app_bundle) = rest.split('/').next() else {
        return false;
    };

    if !app_bundle.starts_with("Xcode") || !app_bundle.ends_with(".app") {
        return false;
    }

    let app_relative_path = &rest[app_bundle.len()..];
    if let Some(usr_bin_entry) = app_relative_path.strip_prefix("/Contents/Developer/usr/bin/") {
        return is_macos_python_executable_name(usr_bin_entry) && !usr_bin_entry.contains('/');
    }

    let Some(framework_entry) = app_relative_path
        .strip_prefix("/Contents/Developer/Library/Frameworks/Python3.framework/Versions/")
    else {
        return false;
    };

    let mut framework_parts = framework_entry.split('/');
    framework_parts
        .next()
        .is_some_and(is_macos_framework_version_dir)
        && framework_parts.next() == Some("bin")
        && framework_parts
            .next()
            .is_some_and(is_macos_python_executable_name)
        && framework_parts.next().is_none()
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

    #[test]
    fn locator_metadata_matches_xcode_kind() {
        let locator = MacXCode::new();

        assert_eq!(locator.get_kind(), LocatorKind::MacXCode);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::MacXCode]
        );
    }

    #[test]
    fn xcode_path_accepts_default_xcode_usr_bin_python() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_accepts_versioned_xcode_usr_bin_python() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode_15.0.1.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_accepts_default_xcode_usr_bin_python_without_version() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python"
        ));
    }

    #[test]
    fn xcode_path_accepts_default_xcode_usr_bin_versioned_python() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python3.12"
        ));
    }

    #[test]
    fn xcode_path_accepts_framework_python_executable() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9"
        ));
    }

    #[test]
    fn xcode_path_accepts_framework_python_executable_in_current_version_dir() {
        assert!(is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/Current/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_non_python_framework_path() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/Resources/Info.plist"
        ));
    }

    #[test]
    fn xcode_path_rejects_python_config_script() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python-config"
        ));
    }

    #[test]
    fn xcode_path_rejects_versioned_python_config_script() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9-config"
        ));
    }

    #[test]
    fn xcode_path_rejects_framework_python_executable_in_invalid_version_dir() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/Foo/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_framework_python_executable_in_patch_version_dir() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9.0/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_multi_dot_python_executable_name() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9.0"
        ));
    }

    #[test]
    fn xcode_path_rejects_compact_python_version_name() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/python312"
        ));
    }

    #[test]
    fn xcode_path_rejects_python_prefixed_tool() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/pythonfoo"
        ));
    }

    #[test]
    fn xcode_path_rejects_unrelated_application_python() {
        assert!(!is_xcode_python_path(
            "/Applications/Other.app/Contents/MacOS/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_other_application_developer_python() {
        assert!(!is_xcode_python_path(
            "/Applications/Other.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_developer_path_outside_applications() {
        assert!(!is_xcode_python_path(
            "/tmp/Xcode.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_nested_developer_layout() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Nested.app/Contents/Developer/usr/bin/python3"
        ));
    }

    #[test]
    fn xcode_path_rejects_nested_usr_bin_entry() {
        assert!(!is_xcode_python_path(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/nested/python3"
        ));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn try_from_rejects_xcode_path_off_macos() {
        let locator = MacXCode::new();
        let env = PythonEnv::new(
            PathBuf::from("/Applications/Xcode.app/Contents/Developer/usr/bin/python3"),
            Some(PathBuf::from(
                "/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9",
            )),
            Some("3.9.6".to_string()),
        );

        assert!(locator.try_from(&env).is_none());
    }
}
