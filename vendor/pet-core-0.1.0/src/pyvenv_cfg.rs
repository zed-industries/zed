// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref VERSION: Regex = Regex::new(r"^version\s*=\s*(\d+\.\d+\.\d+)$")
        .expect("error parsing Version regex for pyenv_cfg");
    static ref VERSION_INFO: Regex = Regex::new(r"^version_info\s*=\s*(\d+\.\d+\.\d+.*)$")
        .expect("error parsing Version_info regex for pyenv_cfg");
}

const PYVENV_CONFIG_FILE: &str = "pyvenv.cfg";

#[derive(Debug)]
pub struct PyVenvCfg {
    pub version: Option<String>,
    pub version_major: Option<u64>,
    pub version_minor: Option<u64>,
    pub prompt: Option<String>,
    pub file_path: PathBuf,
}

impl PyVenvCfg {
    fn new(
        version: Option<String>,
        version_major: Option<u64>,
        version_minor: Option<u64>,
        prompt: Option<String>,
        file_path: PathBuf,
    ) -> Self {
        Self {
            version,
            version_major,
            version_minor,
            prompt,
            file_path,
        }
    }
    pub fn find(path: &Path) -> Option<Self> {
        if let Some(ref file) = find(path) {
            parse(file)
        } else {
            None
        }
    }
}

fn find(path: &Path) -> Option<PathBuf> {
    // env
    // |__ pyvenv.cfg  <--- check if this file exists
    // |__ bin or Scripts
    //     |__ python  <--- interpreterPath

    // Check if the pyvenv.cfg file is in the current directory.
    // Possible the passed value is the `env`` directory.
    let cfg = path.join(PYVENV_CONFIG_FILE);
    if cfg.exists() {
        return Some(cfg);
    }

    if cfg!(windows) {
        // Only windows installations have a `Scripts` directory.
        if path.ends_with("Scripts") {
            let cfg = path.parent()?.join(PYVENV_CONFIG_FILE);
            if cfg.exists() {
                return Some(cfg);
            }
        }
    }
    // Some windows installations have a `bin` directory. https://github.com/microsoft/vscode-python/issues/24792
    if path.ends_with("bin") {
        let cfg = path.parent()?.join(PYVENV_CONFIG_FILE);
        if cfg.exists() {
            return Some(cfg);
        }
    }
    // let cfg = path.parent()?.join(PYVENV_CONFIG_FILE);
    // println!("{:?}", cfg);
    // if fs::metadata(&cfg).is_ok() {
    //     return Some(cfg);
    // }

    // // Check if the pyvenv.cfg file is in the parent directory.
    // // Possible the passed value is the `bin` directory.
    // let cfg = path.parent()?.parent()?.join(PYVENV_CONFIG_FILE);
    // if fs::metadata(&cfg).is_ok() {
    //     return Some(cfg);
    // }

    None
}

fn parse(file: &Path) -> Option<PyVenvCfg> {
    let contents = fs::read_to_string(file).ok()?;
    let mut version: Option<String> = None;
    let mut version_major: Option<u64> = None;
    let mut version_minor: Option<u64> = None;
    let mut prompt: Option<String> = None;

    for line in contents.lines() {
        if version.is_none() {
            if let Some((ver, major, minor)) = parse_version(line, &VERSION) {
                version = Some(ver);
                version_major = Some(major);
                version_minor = Some(minor);
                continue;
            }
            if let Some((ver, major, minor)) = parse_version(line, &VERSION_INFO) {
                version = Some(ver);
                version_major = Some(major);
                version_minor = Some(minor);
                continue;
            }
        }
        if prompt.is_none() {
            if let Some(p) = parse_prompt(line) {
                prompt = Some(p);
            }
        }
        if version.is_some() && prompt.is_some() {
            break;
        }
    }

    match (version, version_major, version_minor) {
        (Some(ver), Some(major), Some(minor)) => Some(PyVenvCfg::new(
            Some(ver),
            Some(major),
            Some(minor),
            prompt,
            file.to_path_buf(),
        )),
        // Even without version info, return the struct - presence of pyvenv.cfg
        // is sufficient to identify this as a venv environment
        _ => Some(PyVenvCfg::new(None, None, None, prompt, file.to_path_buf())),
    }
}

fn parse_version(line: &str, regex: &Regex) -> Option<(String, u64, u64)> {
    if let Some(captures) = regex.captures(line) {
        if let Some(value) = captures.get(1) {
            let version = value.as_str();
            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() >= 2 {
                let version_major = parts[0]
                    .parse()
                    .expect("python major version to be an integer");
                let version_minor = parts[1]
                    .parse()
                    .expect("python minor version to be an integer");
                return Some((version.to_string(), version_major, version_minor));
            }
        }
    }
    None
}

fn parse_prompt(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("prompt") {
        if let Some(eq_idx) = trimmed.find('=') {
            // let value = trimmed[eq_idx + 1..].trim();
            let mut name = trimmed[eq_idx + 1..].trim().to_string();
            // Strip any leading or trailing single or double quotes
            if name.starts_with('"') {
                name = name.trim_start_matches('"').to_string();
            }
            if name.ends_with('"') {
                name = name.trim_end_matches('"').to_string();
            }
            // Strip any leading or trailing single or double quotes
            if name.starts_with('\'') {
                name = name.trim_start_matches('\'').to_string();
            }
            if name.ends_with('\'') {
                name = name.trim_end_matches('\'').to_string();
            }
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_version_standard() {
        let line = "version = 3.11.4";
        let result = parse_version(line, &VERSION);
        assert!(result.is_some());
        let (ver, major, minor) = result.unwrap();
        assert_eq!(ver, "3.11.4");
        assert_eq!(major, 3);
        assert_eq!(minor, 11);
    }

    #[test]
    fn test_parse_version_info() {
        let line = "version_info = 3.12.0.final";
        let result = parse_version(line, &VERSION_INFO);
        assert!(result.is_some());
        let (ver, major, minor) = result.unwrap();
        assert_eq!(ver, "3.12.0.final");
        assert_eq!(major, 3);
        assert_eq!(minor, 12);
    }

    #[test]
    fn test_parse_version_no_match() {
        let line = "home = /usr/bin/python";
        let result = parse_version(line, &VERSION);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_prompt_double_quotes() {
        let line = r#"prompt = "my-env""#;
        let result = parse_prompt(line);
        assert_eq!(result, Some("my-env".to_string()));
    }

    #[test]
    fn test_parse_prompt_single_quotes() {
        let line = "prompt = 'my-env'";
        let result = parse_prompt(line);
        assert_eq!(result, Some("my-env".to_string()));
    }

    #[test]
    fn test_parse_prompt_no_quotes() {
        let line = "prompt = my-venv";
        let result = parse_prompt(line);
        assert_eq!(result, Some("my-venv".to_string()));
    }

    #[test]
    fn test_parse_prompt_with_spaces() {
        let line = "prompt   =   my-venv  ";
        let result = parse_prompt(line);
        assert_eq!(result, Some("my-venv".to_string()));
    }

    #[test]
    fn test_parse_prompt_empty_value() {
        let line = "prompt = ";
        let result = parse_prompt(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_prompt_not_prompt_line() {
        let line = "home = /usr/bin/python";
        let result = parse_prompt(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_pyvenv_cfg_find_in_directory() {
        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.11.4").unwrap();
        writeln!(file, "prompt = test-env").unwrap();

        let result = PyVenvCfg::find(dir.path());
        assert!(result.is_some());
        let cfg = result.unwrap();
        assert_eq!(cfg.version, Some("3.11.4".to_string()));
        assert_eq!(cfg.version_major, Some(3));
        assert_eq!(cfg.version_minor, Some(11));
        assert_eq!(cfg.prompt, Some("test-env".to_string()));
    }

    #[test]
    fn test_pyvenv_cfg_find_from_bin() {
        let dir = tempdir().unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version = 3.10.0").unwrap();

        let result = PyVenvCfg::find(&bin_dir);
        assert!(result.is_some());
        let cfg = result.unwrap();
        assert_eq!(cfg.version, Some("3.10.0".to_string()));
        assert_eq!(cfg.version_major, Some(3));
        assert_eq!(cfg.version_minor, Some(10));
    }

    #[test]
    fn test_pyvenv_cfg_not_found() {
        let dir = tempdir().unwrap();
        let result = PyVenvCfg::find(dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn test_pyvenv_cfg_missing_version() {
        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "home = /usr/bin/python").unwrap();
        writeln!(file, "prompt = my-env").unwrap();

        let result = PyVenvCfg::find(dir.path());
        // pyvenv.cfg exists, so we should get a result even without version
        assert!(result.is_some());
        let cfg = result.unwrap();
        assert!(cfg.version.is_none());
        assert!(cfg.version_major.is_none());
        assert!(cfg.version_minor.is_none());
        assert_eq!(cfg.prompt, Some("my-env".to_string()));
    }

    #[test]
    fn test_pyvenv_cfg_version_info_format() {
        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join("pyvenv.cfg");
        let mut file = fs::File::create(&cfg_path).unwrap();
        writeln!(file, "version_info = 3.12.1.final.0").unwrap();

        let result = PyVenvCfg::find(dir.path());
        assert!(result.is_some());
        let cfg = result.unwrap();
        assert_eq!(cfg.version, Some("3.12.1.final.0".to_string()));
        assert_eq!(cfg.version_major, Some(3));
        assert_eq!(cfg.version_minor, Some(12));
    }
}
