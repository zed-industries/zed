// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use pet_fs::path::resolve_symlink;
use pet_python_utils::executable::find_executables;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref PYTHON_VERSION: Regex =
        Regex::new(r"/python@((\d+\.?)*)/").expect("error parsing Version regex for Homebrew");
}

pub fn is_homebrew_python(exe: &Path) -> bool {
    exe.starts_with("/opt/homebrew")
        || exe.starts_with("/usr/local/Cellar")
        || exe.starts_with("/home/linuxbrew/.linuxbrew")
}

pub fn get_known_symlinks(
    symlink_resolved_python_exe: &Path,
    full_version: &String,
) -> Vec<PathBuf> {
    let mut symlinks = get_known_symlinks_impl(symlink_resolved_python_exe, full_version);

    // Go through all the exes in all of the above bin directories and verify we have a list of all of them.
    // They too could be symlinks, e.g. we could have `/opt/homebrew/bin/python3` & also `/opt/homebrew/bin/python`
    // And possible they are all symlnks to the same exe.
    let known_symlinks = symlinks.clone();
    let other_symlinks: Vec<PathBuf> = symlinks
        .par_iter()
        .flat_map(|symlink| {
            if let Some(bin) = symlink.parent() {
                find_executables(bin)
                    .into_iter()
                    .filter(|possible_symlink| {
                        if let Some(resolved) = resolve_symlink(possible_symlink) {
                            known_symlinks.contains(&resolved)
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .collect();
    symlinks.extend(other_symlinks);

    symlinks.sort();
    symlinks.dedup();

    symlinks
}

pub fn get_known_symlinks_impl(
    symlink_resolved_python_exe: &Path,
    full_version: &String,
) -> Vec<PathBuf> {
    if symlink_resolved_python_exe.starts_with("/opt/homebrew") {
        // Real exe - /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12

        // Known symlinks include
        // /opt/homebrew/bin/python3.12
        // /opt/homebrew/opt/python3/bin/python3.12
        // /opt/homebrew/Cellar/python@3.12/3.12.3/bin/python3.12
        // /opt/homebrew/opt/python@3.12/bin/python3.12
        // /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12
        // /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/Current/bin/python3.12
        // /opt/homebrew/Frameworks/Python.framework/Versions/3.12/bin/python3.12
        // /opt/homebrew/Frameworks/Python.framework/Versions/Current/bin/python3.12
        // /opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12
        match PYTHON_VERSION.captures(symlink_resolved_python_exe.to_str().unwrap_or_default()) {
            Some(captures) => match captures.get(1) {
                Some(version) => {
                    let version = version.as_str().to_string();
                    let mut symlinks = vec![symlink_resolved_python_exe.to_owned()];
                    for possible_symlink in [
                        PathBuf::from(format!("/opt/homebrew/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/opt/python@{version}/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/Cellar/python@{version}/{full_version}/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/Cellar/python@{version}/{full_version}/Frameworks/Python.framework/Versions/{version}/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/Cellar/python@{version}/{full_version}/Frameworks/Python.framework/Versions/Current/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/Frameworks/Python.framework/Versions/{version}/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/Frameworks/Python.framework/Versions/Current/bin/python{version}")),
                        PathBuf::from(format!("/usr/local/opt/python@{version}/bin/python3")),
                        PathBuf::from(format!("/usr/local/opt/python@{version}/bin/python{version}")),
                        PathBuf::from("/opt/homebrew/opt/python/bin/python3"),
                        PathBuf::from(format!("/opt/homebrew/opt/python/bin/python{version}")),
                        PathBuf::from("/opt/homebrew/opt/python@3/bin/python3"),
                        PathBuf::from(format!("/opt/homebrew/opt/python@3/bin/python{version}")),
                        PathBuf::from(format!("/opt/homebrew/opt/python@{version}/bin/python3")),
                        PathBuf::from(format!("/opt/homebrew/opt/python@{version}/bin/python{version}")),
                        PathBuf::from("/usr/local/opt/python@3/bin/python3"),
                        PathBuf::from(format!("/usr/local/opt/python@3/bin/python{version}")),
                        // Check if this symlink is pointing to the same place as the resolved python exe
                        PathBuf::from(format!("/opt/homebrew/opt/python3/bin/python{version}")),
                        // Check if this symlink is pointing to the same place as the resolved python exe
                        PathBuf::from("/opt/homebrew/bin/python3"),
                        // Check if this symlink is pointing to the same place as the resolved python exe
                        PathBuf::from("/opt/homebrew/bin/python")
                        ] {

                        // Validate the symlinks
                        if symlinks.contains(
                            &resolve_symlink(&possible_symlink)
                                .or(fs::canonicalize(&possible_symlink).ok())
                                .unwrap_or_default(),
                        ) {
                            symlinks.push(possible_symlink);
                        }
                    }

                    symlinks
                }
                None => vec![],
            },
            None => vec![],
        }
    } else if symlink_resolved_python_exe.starts_with("/usr/local/Cellar") {
        // Real exe - /usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8

        // Known symlinks include
        // /usr/local/bin/python3.8
        // /usr/local/opt/python@3.8/bin/python3.8
        // /usr/local/Cellar/python@3.8/3.8.20/bin/python3.8
        // /usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8
        match PYTHON_VERSION.captures(symlink_resolved_python_exe.to_str().unwrap_or_default()) {
            Some(captures) => match captures.get(1) {
                Some(version) => {
                    let version = version.as_str().to_string();
                    // Never include `/usr/local/bin/python` into this list.
                    // See previous explanation
                    let mut symlinks = vec![symlink_resolved_python_exe.to_owned()];
                    for possible_symlink in [
                            // While testing found that on Mac Intel
                            // 1. python 3.8 has sysprefix in /usr/local/Cellar/python@3.9/3.9.19/Frameworks/Python.framework/Versions/3.9
                            // 2. python 3.9 has sysprefix in /usr/local/opt/python@3.9/Frameworks/Python.framework/Versions/3.9
                            // 3. python 3.11 has sysprefix in /usr/local/opt/python@3.11/Frameworks/Python.framework/Versions/3.11
                            PathBuf::from(format!("/usr/local/opt/python@{version}/bin/python3")),
                            PathBuf::from(format!("/usr/local/opt/python@{version}/bin/python{version}")),
                            PathBuf::from("/usr/local/opt/python@3/bin/python3"),
                            PathBuf::from(format!("/usr/local/opt/python@3/bin/python{version}")),
                            PathBuf::from(format!(
                                "/usr/local/Cellar/python@{version}/{full_version}/bin/python{version}"
                            )),
                            PathBuf::from(format!(
                                "/usr/local/Cellar/python@{version}/{full_version}/Frameworks/Python.framework/Versions/{version}/bin/python{version}"
                            )),
                            // This is a special folder, if users install python using other means, this file
                            // might get overridden. So we should only add this if this files points to the same place
                            PathBuf::from(format!("/usr/local/bin/python{version}")),
                            // Check if this symlink is pointing to the same place as the resolved python exe
                            PathBuf::from("/usr/local/bin/python3"),
                            // Check if this symlink is pointing to the same place as the resolved python exe
                            PathBuf::from("/usr/local/bin/python"),
                        ] {

                        // Validate the symlinks
                        if symlinks.contains(
                            &resolve_symlink(&possible_symlink)
                                // .or(fs::canonicalize(&possible_symlink).ok())
                                .unwrap_or_default(),
                        ) {
                            symlinks.push(possible_symlink);
                        }
                    }

                    symlinks
                }
                None => vec![],
            },
            None => vec![],
        }
    } else if symlink_resolved_python_exe.starts_with("/home/linuxbrew/.linuxbrew") {
        // Real exe - /home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.3/bin/python3.12

        // Known symlinks include
        // /usr/local/bin/python3.12
        // /home/linuxbrew/.linuxbrew/bin/python3.12
        // /home/linuxbrew/.linuxbrew/opt/python@3.12/bin/python3.12
        match PYTHON_VERSION.captures(symlink_resolved_python_exe.to_str().unwrap_or_default()) {
            Some(captures) => match captures.get(1) {
                Some(version) => {
                    let version = version.as_str().to_string();
                    // Never include `/usr/local/bin/python` into this list.
                    // See previous explanation
                    let mut symlinks = vec![symlink_resolved_python_exe.to_owned()];
                    for possible_symlink in [
                        PathBuf::from("/home/linuxbrew/.linuxbrew/bin/python3"),
                        PathBuf::from(format!("/home/linuxbrew/.linuxbrew/bin/python{version}")),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/Cellar/python@{version}/{full_version}/bin/python{version}"
                        )),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/Cellar/python@{version}/{full_version}/bin/python3"
                        )),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/opt/python@{version}/bin/python{version}"
                        )),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/opt/python@{version}/bin/python3"
                        )),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/opt/python3/bin/python{version}"
                        )),
                        PathBuf::from("/home/linuxbrew/.linuxbrew/opt/python3/bin/python3"),
                        PathBuf::from(format!(
                            "/home/linuxbrew/.linuxbrew/opt/python@3/bin/python{version}"
                        )),
                        PathBuf::from("/home/linuxbrew/.linuxbrew/opt/python@3/bin/python3"),
                        // This is a special folder, if users install python using other means, this file
                        // might get overridden. So we should only add this if this files points to the same place
                        PathBuf::from(format!("/usr/local/bin/python{version}")),
                        // Check if this symlink is pointing to the same place as the resolved python exe
                        PathBuf::from("/usr/local/bin/python3"),
                        // Check if this symlink is pointing to the same place as the resolved python exe
                        PathBuf::from("/usr/local/bin/python"),
                    ] {
                        // Validate the symlinks
                        if symlinks.contains(
                            &resolve_symlink(&possible_symlink)
                                .or(fs::canonicalize(&possible_symlink).ok())
                                .unwrap_or_default(),
                        ) {
                            symlinks.push(possible_symlink);
                        }
                    }

                    symlinks
                }
                None => vec![],
            },
            None => vec![],
        }
    } else {
        vec![]
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn homebrew_python_paths_are_recognized_across_supported_prefixes() {
        assert!(is_homebrew_python(Path::new(
            "/opt/homebrew/Cellar/python@3.12/3.12.4/bin/python3.12"
        )));
        assert!(is_homebrew_python(Path::new(
            "/usr/local/Cellar/python@3.11/3.11.9/bin/python3.11"
        )));
        assert!(is_homebrew_python(Path::new(
            "/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12"
        )));
        assert!(!is_homebrew_python(Path::new("/usr/bin/python3.12")));
    }

    #[test]
    fn is_homebrew_python_recognizes_opt_homebrew_bin_paths() {
        assert!(is_homebrew_python(Path::new(
            "/opt/homebrew/bin/python3.12"
        )));
        assert!(is_homebrew_python(Path::new(
            "/opt/homebrew/opt/python@3.12/bin/python3.12"
        )));
        assert!(is_homebrew_python(Path::new(
            "/opt/homebrew/Frameworks/Python.framework/Versions/3.12/bin/python3.12"
        )));
    }

    #[test]
    fn is_homebrew_python_rejects_non_homebrew_paths() {
        assert!(!is_homebrew_python(Path::new("/usr/local/bin/python3.12")));
        assert!(!is_homebrew_python(Path::new("/usr/bin/python3")));
        assert!(!is_homebrew_python(Path::new(
            "/home/user/.pyenv/versions/3.12.0/bin/python3.12"
        )));
        assert!(!is_homebrew_python(Path::new("")));
    }

    #[test]
    fn known_symlink_templates_include_resolved_executable_for_linuxbrew() {
        let resolved_exe =
            PathBuf::from("/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12");
        let symlinks = get_known_symlinks_impl(&resolved_exe, &"3.12.4".to_string());

        assert!(symlinks.contains(&resolved_exe));
    }

    #[test]
    fn known_symlink_templates_return_empty_for_unrecognized_paths() {
        assert!(
            get_known_symlinks_impl(Path::new("/usr/bin/python3.12"), &"3.12.4".to_string())
                .is_empty()
        );
    }

    #[test]
    fn known_symlink_templates_include_self_for_opt_homebrew() {
        let resolved_exe = PathBuf::from(
            "/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12",
        );
        let symlinks = get_known_symlinks_impl(&resolved_exe, &"3.12.3".to_string());

        assert!(symlinks.contains(&resolved_exe));
        assert!(!symlinks.is_empty());
    }

    #[test]
    fn known_symlink_templates_include_self_for_usr_local_cellar() {
        let resolved_exe = PathBuf::from(
            "/usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8",
        );
        let symlinks = get_known_symlinks_impl(&resolved_exe, &"3.8.20".to_string());

        assert!(symlinks.contains(&resolved_exe));
        assert!(!symlinks.is_empty());
    }

    #[test]
    fn known_symlink_templates_return_empty_when_version_regex_does_not_match() {
        // Path under /opt/homebrew but without a python@version segment
        let resolved_exe = PathBuf::from("/opt/homebrew/bin/python3.12");
        let symlinks = get_known_symlinks_impl(&resolved_exe, &"3.12.0".to_string());

        // No python@version/ in path, so regex won't capture → returns empty
        assert!(symlinks.is_empty());
    }

    #[test]
    fn known_symlink_templates_for_linuxbrew_contain_expected_paths() {
        let resolved_exe =
            PathBuf::from("/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12");
        let symlinks = get_known_symlinks_impl(&resolved_exe, &"3.12.4".to_string());

        // The resolved exe itself is always included
        assert!(symlinks.contains(&resolved_exe));
        // On a test system without real symlinks, only the resolved exe will pass validation.
        // But verify the function doesn't panic and returns at least the resolved exe.
        assert!(!symlinks.is_empty());
    }
}
