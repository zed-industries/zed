// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::sym_links::get_known_symlinks;
use lazy_static::lazy_static;
use log::trace;
use pet_core::python_environment::{
    PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind,
};
use pet_fs::path::resolve_symlink;
use regex::Regex;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref PYTHON_VERSION: Regex =
        Regex::new(r"/(\d+\.\d+\.\d+)").expect("error parsing Version regex for Homebrew");
}

pub fn get_python_info(
    python_exe_from_bin_dir: &Path,
    resolved_exe: &Path,
) -> Option<PythonEnvironment> {
    let version = get_version(resolved_exe);

    trace!(
        "Getting homebrew python info for {:?} => {:?} with version {:?}",
        python_exe_from_bin_dir,
        resolved_exe,
        version
    );

    let mut symlinks = vec![
        python_exe_from_bin_dir.to_path_buf(),
        resolved_exe.to_path_buf(),
    ];
    if let Some(version) = &version {
        symlinks.append(&mut get_known_symlinks(resolved_exe, version));
    }

    // Possible the python_exe_from_bin_dir is <full path>/python3.12
    // And we have other files in the same directory <full path>python3 and <full path>python
    // Check if they also point to the same resolved_exe
    // If they do, then they are also valid symlinks.
    let parent = python_exe_from_bin_dir.parent()?;
    for possible_symlink in [parent.join("python"), parent.join("python3")] {
        if resolve_symlink(&possible_symlink).unwrap_or_default() == resolved_exe {
            symlinks.push(possible_symlink);
        }
    }

    symlinks.sort();
    symlinks.dedup();

    let env = PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Homebrew))
        .executable(Some(python_exe_from_bin_dir.to_path_buf()))
        .version(version)
        .prefix(get_prefix(resolved_exe))
        .symlinks(Some(symlinks))
        .build();
    Some(env)
}

fn get_version(resolved_exe: &Path) -> Option<String> {
    let python_version = resolved_exe.to_string_lossy().to_string();
    match PYTHON_VERSION.captures(&python_version) {
        Some(captures) => captures.get(1).map(|version| version.as_str().to_string()),
        None => None,
    }
}

fn get_prefix(_resolved_file: &Path) -> Option<PathBuf> {
    // NOTE:
    // While testing found that on Mac Intel
    // 1. python 3.8 has sysprefix in /usr/local/Cellar/python@3.9/3.9.19/Frameworks/Python.framework/Versions/3.9
    // 2. python 3.9 has sysprefix in /usr/local/opt/python@3.9/Frameworks/Python.framework/Versions/3.9
    // 3. python 3.11 has sysprefix in /usr/local/opt/python@3.11/Frameworks/Python.framework/Versions/3.11
    // All because 3.11 was already installed,
    // Similarly when 3.12 is already installed via python.org or the like, then installed again
    // via homevrew, the prefix can be different.
    // Hence determining the sysprefix is impossible, unless we spawn python
    //
    // CONCLUSION =>  Thus, always return None

    // // If the fully resolved file path contains the words `/homebrew/` or `/linuxbrew/`
    // // Then we know this is definitely a home brew version of python.
    // // And in these cases we can compute the sysprefix.

    // let resolved_file = resolved_file.to_str()?;
    // // 1. MacOS Silicon
    // if resolved_file.starts_with("/opt/homebrew") {
    //     // Resolved exe is something like `/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12`
    //     let reg_ex = Regex::new("/opt/homebrew/Cellar/python@((\\d+\\.?)*)/(\\d+\\.?)*/Frameworks/Python.framework/Versions/(\\d+\\.?)*/bin/python(\\d+\\.?)*").unwrap();
    //     let captures = reg_ex.captures(resolved_file)?;
    //     let version = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
    //     // SysPrefix- /opt/homebrew/opt/python@3.12/Frameworks/Python.framework/Versions/3.12
    //     let sys_prefix = PathBuf::from(format!(
    //         "/opt/homebrew/opt/python@{}/Frameworks/Python.framework/Versions/{}",
    //         version, version
    //     ));

    //     return if sys_prefix.exists() {
    //         Some(sys_prefix)
    //     } else {
    //         None
    //     };
    // }

    // // 2. Linux
    // if resolved_file.starts_with("/home/linuxbrew/.linuxbrew") {
    //     // Resolved exe is something like `/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.3/bin/python3.12`
    //     let reg_ex = Regex::new("/home/linuxbrew/.linuxbrew/Cellar/python@(\\d+\\.?\\d+\\.?)/(\\d+\\.?\\d+\\.?\\d+\\.?)/bin/python.*").unwrap();
    //     let captures = reg_ex.captures(resolved_file)?;
    //     let version = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
    //     let full_version = captures.get(2).map(|m| m.as_str()).unwrap_or_default();
    //     // SysPrefix- /home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.3
    //     let sys_prefix = PathBuf::from(format!(
    //         "/home/linuxbrew/.linuxbrew/Cellar/python@{}/{}",
    //         version, full_version
    //     ));

    //     return if sys_prefix.exists() {
    //         Some(sys_prefix)
    //     } else {
    //         None
    //     };
    // }

    // // 3. MacOS Intel
    // if resolved_file.starts_with("/usr/local/Cellar") {
    //     // Resolved exe is something like `/usr/local/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12`
    //     let reg_ex = Regex::new("/usr/local/Cellar/python@(\\d+\\.?\\d+\\.?)/(\\d+\\.?\\d+\\.?\\d+\\.?)/Frameworks/Python.framework/Versions/(\\d+\\.?\\d+\\.?)/bin/python.*").unwrap();
    //     let captures = reg_ex.captures(resolved_file)?;
    //     let version = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
    //     let full_version = captures.get(2).map(|m| m.as_str()).unwrap_or_default();
    //     // SysPrefix- /usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8
    //     let sys_prefix = PathBuf::from(format!(
    //         "/usr/local/Cellar/python@{}/{}/Frameworks/Python.framework/Versions/{}",
    //         version, full_version, version
    //     ));

    //     return if sys_prefix.exists() {
    //         Some(sys_prefix)
    //     } else {
    //         None
    //     };
    // }
    None
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn extract_version() {
        assert_eq!(
            get_version(&PathBuf::from(
                "/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3"
            )),
            Some("3.12.4".to_string())
        );

        assert_eq!(
            get_version(&PathBuf::from(
                "/home/linuxbrew/.linuxbrew/Cellar/python@3.11/3.11.9_1/bin/python3.11"
            )),
            Some("3.11.9".to_string())
        );
    }

    #[test]
    fn extract_version_from_opt_homebrew_path() {
        assert_eq!(
            get_version(&PathBuf::from(
                "/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12"
            )),
            Some("3.12.3".to_string())
        );
    }

    #[test]
    fn extract_version_from_usr_local_cellar_path() {
        assert_eq!(
            get_version(&PathBuf::from(
                "/usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8"
            )),
            Some("3.8.20".to_string())
        );
    }

    #[test]
    fn extract_version_returns_none_for_path_without_version() {
        assert_eq!(get_version(&PathBuf::from("/usr/bin/python3")), None);
    }

    #[test]
    fn get_prefix_always_returns_none() {
        assert!(get_prefix(&PathBuf::from(
            "/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12"
        ))
        .is_none());
        assert!(get_prefix(&PathBuf::from(
            "/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12"
        ))
        .is_none());
        assert!(get_prefix(&PathBuf::from(
            "/usr/local/Cellar/python@3.8/3.8.20/bin/python3.8"
        ))
        .is_none());
    }

    #[test]
    fn get_python_info_returns_correct_kind_and_executable() {
        let bin_exe = PathBuf::from("/home/linuxbrew/.linuxbrew/bin/python3.12");
        let resolved_exe =
            PathBuf::from("/home/linuxbrew/.linuxbrew/Cellar/python@3.12/3.12.4/bin/python3.12");

        let env = get_python_info(&bin_exe, &resolved_exe).unwrap();

        assert_eq!(env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(env.executable, Some(bin_exe.clone()));
        assert_eq!(env.version, Some("3.12.4".to_string()));
        assert_eq!(env.prefix, None);
        // Both bin exe and resolved exe should be in symlinks
        let symlinks = env.symlinks.unwrap();
        assert!(symlinks.contains(&bin_exe));
        assert!(symlinks.contains(&resolved_exe));
    }

    #[test]
    fn get_python_info_returns_none_version_for_unversioned_path() {
        let bin_exe = PathBuf::from("/home/linuxbrew/.linuxbrew/bin/python3");
        let resolved_exe = PathBuf::from("/home/linuxbrew/.linuxbrew/bin/python3");

        let env = get_python_info(&bin_exe, &resolved_exe).unwrap();

        assert_eq!(env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(env.version, None);
    }

    #[test]
    fn get_python_info_for_opt_homebrew_path() {
        let bin_exe = PathBuf::from("/opt/homebrew/bin/python3.12");
        let resolved_exe = PathBuf::from(
            "/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12",
        );

        let env = get_python_info(&bin_exe, &resolved_exe).unwrap();

        assert_eq!(env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(env.executable, Some(bin_exe.clone()));
        assert_eq!(env.version, Some("3.12.3".to_string()));
        let symlinks = env.symlinks.unwrap();
        assert!(symlinks.contains(&bin_exe));
        assert!(symlinks.contains(&resolved_exe));
    }

    #[test]
    fn get_python_info_for_usr_local_cellar_path() {
        let bin_exe = PathBuf::from("/usr/local/bin/python3.8");
        let resolved_exe = PathBuf::from(
            "/usr/local/Cellar/python@3.8/3.8.20/Frameworks/Python.framework/Versions/3.8/bin/python3.8",
        );

        let env = get_python_info(&bin_exe, &resolved_exe).unwrap();

        assert_eq!(env.kind, Some(PythonEnvironmentKind::Homebrew));
        assert_eq!(env.executable, Some(bin_exe));
        assert_eq!(env.version, Some("3.8.20".to_string()));
    }
}
