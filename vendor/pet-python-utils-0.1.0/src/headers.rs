// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, path::Path};

lazy_static! {
    static ref VERSION: Regex = Regex::new(r#"#define\s+PY_VERSION\s+"((\d+\.?)*.*)\""#)
        .expect("error parsing Version regex for partchlevel.h");
}

#[derive(Debug)]
pub struct Headers {
    #[allow(dead_code)]
    pub version: String,
}

impl Headers {
    pub fn get_version(path: &Path) -> Option<String> {
        let mut path = path.to_path_buf();
        if cfg!(windows) {
            // Only Windows can have a Scripts folder
            let bin = "Scripts";
            if path.join(bin).exists() {
                path = path.join(bin);
            }
        }
        let bin = "bin"; // Windows can have bin as well, see https://github.com/microsoft/vscode-python/issues/24792
        if path.ends_with(bin) {
            path.pop();
        }
        get_version(&path, None)
    }
}

// Get the python version from the `<sys prefix>/include/patchlevel.h` file
// On windows the path is `<sys prefix>/Headers/patchlevel.h`
// The lines we are looking for are:
// /* Version as a string */
// #define PY_VERSION              "3.10.2"
// /*--end constants--*/
pub fn get_version(sys_prefix: &Path, pyver: Option<(u64, u64)>) -> Option<String> {
    // Generally the files are in Headers in windows and include in unix
    // However they can also be in Headers on Mac (command line tools python, hence make no assumptions)
    for headers_path in [sys_prefix.join("Headers"), sys_prefix.join("include")] {
        if !headers_path.exists() {
            continue;
        }
        let patchlevel_h = headers_path.join("patchlevel.h");
        if let Some(version) = valid_version_from_header(&patchlevel_h, pyver) {
            return Some(version);
        } else {
            // Try the other path
            // Sometimes we have it in a sub directory such as `python3.10` or `pypy3.9`
            if let Ok(readdir) = fs::read_dir(&headers_path) {
                for path in readdir.filter_map(Result::ok) {
                    if let Ok(t) = path.file_type() {
                        if !t.is_dir() {
                            continue;
                        }
                    }
                    let path = path.path();
                    let patchlevel_h = path.join("patchlevel.h");
                    if let Some(version) = valid_version_from_header(&patchlevel_h, pyver) {
                        return Some(version);
                    }
                }
            }
        }
    }
    None
}

fn valid_version_from_header(header: &Path, pyver: Option<(u64, u64)>) -> Option<String> {
    let contents = fs::read_to_string(header).ok()?;
    for line in contents.lines() {
        if let Some(captures) = VERSION.captures(line) {
            let version = captures.get(1)?.as_str();
            if let Some(pyver) = pyver {
                let parts: Vec<u64> = version
                    .splitn(3, ".")
                    .take(2)
                    .flat_map(str::parse::<u64>)
                    .collect();
                if parts.len() == 2 && (parts[0], parts[1]) == pyver {
                    return Some(version.to_string());
                }
            } else {
                return Some(version.to_string());
            }
        }
    }
    None
}
