// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    env,
    path::{Path, PathBuf},
};

use pet_core::env::PythonEnv;
use pet_fs::path::{resolve_any_symlink, resolve_symlink};

const SYSTEM_PYTHON_DIR: &str = "/usr/bin";
const XCODE_SELECT_LINK: &str = "/var/db/xcode_select_link";
const DEFAULT_XCODE_DEVELOPER_DIR: &str = "/Applications/Xcode.app/Contents/Developer";
const DEFAULT_COMMAND_LINE_TOOLS_DIR: &str = "/Library/Developer/CommandLineTools";

pub fn is_macos_system_python(executable: &Path) -> bool {
    let mut components = executable.components();
    matches!(components.next(), Some(std::path::Component::RootDir))
        && matches!(components.next(), Some(std::path::Component::Normal(part)) if part == "usr")
        && matches!(components.next(), Some(std::path::Component::Normal(part)) if part == "bin")
        && matches!(components.next(), Some(std::path::Component::Normal(name)) if is_macos_python_name(name))
        && components.next().is_none()
}

fn is_macos_python_name(name: &std::ffi::OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    if name == "python3" {
        return true;
    }
    let Some(minor) = name.strip_prefix("python3.") else {
        return false;
    };
    !minor.is_empty() && minor.bytes().all(|byte| byte.is_ascii_digit())
}

pub fn resolve_macos_system_python(executable: &Path) -> Option<PathBuf> {
    if std::env::consts::OS != "macos" || !is_macos_system_python(executable) {
        return None;
    }
    let developer_dir = active_developer_dir()?;
    selected_python_with(executable, &developer_dir, Path::is_file)
}

pub fn resolve_macos_system_python_env(env: &PythonEnv) -> Option<PythonEnv> {
    let executable = resolve_macos_system_python(&env.executable)?;
    let mut resolved = PythonEnv::new(executable, env.prefix.clone(), env.version.clone());
    let mut aliases = env.symlinks.clone().unwrap_or_default();
    aliases.push(env.executable.clone());
    aliases.sort();
    aliases.dedup();
    resolved.symlinks = Some(aliases);
    Some(resolved)
}

pub fn add_macos_system_python_alias(symlinks: &mut Vec<PathBuf>) {
    let alias = PathBuf::from(SYSTEM_PYTHON_DIR).join("python3");
    let Some(selected) = resolve_macos_system_python(&alias) else {
        return;
    };
    let resolved = resolve_symlink(&selected).unwrap_or_else(|| selected.clone());
    add_alias_if_target_matches(symlinks, alias, &selected, &resolved);
}

fn active_developer_dir() -> Option<PathBuf> {
    let environment = env::var_os("DEVELOPER_DIR").map(PathBuf::from);
    let selected = resolve_any_symlink(&PathBuf::from(XCODE_SELECT_LINK));
    active_developer_dir_with(environment, selected, Path::is_dir)
}

fn active_developer_dir_with(
    environment: Option<PathBuf>,
    selected: Option<PathBuf>,
    is_dir: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    environment
        .into_iter()
        .chain(selected)
        .chain([
            PathBuf::from(DEFAULT_XCODE_DEVELOPER_DIR),
            PathBuf::from(DEFAULT_COMMAND_LINE_TOOLS_DIR),
        ])
        .map(normalize_developer_dir)
        .find(|path| is_dir(path))
}

fn normalize_developer_dir(path: PathBuf) -> PathBuf {
    if path.extension().is_some_and(|extension| extension == "app") {
        path.join("Contents").join("Developer")
    } else {
        path
    }
}

fn selected_python_with(
    alias: &Path,
    developer_dir: &Path,
    mut is_file: impl FnMut(&Path) -> bool,
) -> Option<PathBuf> {
    if !is_macos_system_python(alias) {
        return None;
    }
    let candidate = developer_dir
        .join("usr")
        .join("bin")
        .join(alias.file_name()?);
    is_file(&candidate).then_some(candidate)
}

fn add_alias_if_target_matches(
    symlinks: &mut Vec<PathBuf>,
    alias: PathBuf,
    selected: &Path,
    resolved: &Path,
) {
    if symlinks
        .iter()
        .any(|path| path == selected || path == resolved)
    {
        symlinks.push(alias);
        symlinks.sort();
        symlinks.dedup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_python_requires_a_python_name_directly_under_usr_bin() {
        assert!(is_macos_system_python(Path::new("/usr/bin/python3")));
        assert!(is_macos_system_python(Path::new("/usr/bin/python3.12")));
        assert!(!is_macos_system_python(Path::new("/usr/bin/python")));
        assert!(!is_macos_system_python(Path::new("/usr/local/bin/python3")));
        assert!(!is_macos_system_python(Path::new(
            "/usr/bin/python3-config"
        )));
    }

    #[test]
    fn public_resolvers_reject_non_system_python() {
        let executable = Path::new("/usr/local/bin/python3");
        assert!(resolve_macos_system_python(executable).is_none());

        let env = PythonEnv::new(executable.to_path_buf(), None, None);
        assert!(resolve_macos_system_python_env(&env).is_none());
    }

    #[test]
    fn developer_dir_prefers_environment_and_normalizes_app_bundle() {
        let selected = active_developer_dir_with(
            Some(PathBuf::from("/Applications/Xcode_16.app")),
            Some(PathBuf::from(DEFAULT_COMMAND_LINE_TOOLS_DIR)),
            |_| true,
        );

        assert_eq!(
            selected,
            Some(PathBuf::from(
                "/Applications/Xcode_16.app/Contents/Developer"
            ))
        );
    }

    #[test]
    fn developer_dir_falls_back_to_selected_link_then_standard_locations() {
        let selected = active_developer_dir_with(
            None,
            Some(PathBuf::from(
                "/Applications/Xcode_Beta.app/Contents/Developer",
            )),
            |_| true,
        );
        assert_eq!(
            selected,
            Some(PathBuf::from(
                "/Applications/Xcode_Beta.app/Contents/Developer"
            ))
        );

        let fallback = active_developer_dir_with(None, None, |path| {
            path == Path::new(DEFAULT_COMMAND_LINE_TOOLS_DIR)
        });
        assert_eq!(
            fallback,
            Some(PathBuf::from(DEFAULT_COMMAND_LINE_TOOLS_DIR))
        );
    }

    #[test]
    fn selected_python_maps_alias_without_spawning() {
        let developer_dir = Path::new(DEFAULT_COMMAND_LINE_TOOLS_DIR);
        let expected = developer_dir.join("usr/bin/python3");
        let mut file_checks = 0;

        let selected = selected_python_with(Path::new("/usr/bin/python3"), developer_dir, |path| {
            file_checks += 1;
            path == expected
        });

        assert_eq!(selected, Some(expected));
        assert_eq!(file_checks, 1);
    }

    #[test]
    fn alias_is_added_only_for_a_matching_selected_target() {
        let selected = PathBuf::from("/Library/Developer/CommandLineTools/usr/bin/python3");
        let resolved = PathBuf::from(
            "/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9",
        );
        let alias = PathBuf::from("/usr/bin/python3");
        let mut symlinks = vec![resolved.clone()];

        add_alias_if_target_matches(&mut symlinks, alias.clone(), &selected, &resolved);
        assert!(symlinks.contains(&alias));

        let mut unrelated = vec![PathBuf::from("/opt/homebrew/bin/python3")];
        add_alias_if_target_matches(&mut unrelated, alias.clone(), &selected, &resolved);
        assert!(!unrelated.contains(&alias));
    }
}
