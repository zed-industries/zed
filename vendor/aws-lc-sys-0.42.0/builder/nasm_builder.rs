// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{execute_command, target_arch, test_nasm_command, use_prebuilt_nasm};

#[derive(Debug)]
pub(crate) struct NasmBuilder {
    files: Vec<PathBuf>,
    includes: Vec<PathBuf>,
    defines: Vec<(String, Option<String>)>,
    flags: Vec<String>,
    out_dir: PathBuf,
    manifest_dir: PathBuf,
}

impl NasmBuilder {
    pub(crate) fn new(manifest_dir: PathBuf, out_dir: PathBuf) -> Self {
        Self {
            files: Vec::new(),
            includes: Vec::new(),
            defines: Vec::new(),
            flags: Vec::new(),
            out_dir,
            manifest_dir,
        }
    }

    pub(crate) fn file<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.files.push(p.as_ref().to_path_buf());
        self
    }

    pub(crate) fn include<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.includes.push(dir.as_ref().to_path_buf());
        self
    }

    pub(crate) fn define(&mut self, key: &str, val: Option<&str>) -> &mut Self {
        self.defines
            .push((key.to_string(), val.map(std::string::ToString::to_string)));
        self
    }

    pub(crate) fn flag(&mut self, flag: &str) -> &mut Self {
        self.flags.push(flag.to_string());
        self
    }

    pub(crate) fn compile_intermediates(&self) -> Vec<PathBuf> {
        let mut objects = Vec::new();

        if self.files.is_empty() {
            return vec![];
        }

        if test_nasm_command() {
            for src in &self.files {
                let obj_name = src
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".asm", ".obj");
                let obj_path = self.out_dir.join(obj_name);

                let format = match target_arch().as_str() {
                    "x86_64" => "win64",
                    _ => "win32",
                };

                let mut args: Vec<OsString> = vec![
                    "-f".into(),
                    format.into(),
                    "-o".into(),
                    obj_path.as_os_str().into(),
                ];

                for inc in &self.includes {
                    args.push("-I".into());
                    args.push(inc.as_os_str().into());
                }

                for (key, val) in &self.defines {
                    let def = if let Some(v) = val {
                        format!("-D{key}={v}")
                    } else {
                        format!("-D{key}")
                    };
                    args.push(def.into());
                }

                args.extend(self.flags.iter().map(std::convert::Into::into));

                args.push(src.as_os_str().into());

                let result = execute_command(
                    "nasm".as_ref(),
                    &args
                        .iter()
                        .map(std::ffi::OsString::as_os_str)
                        .collect::<Vec<_>>(),
                );
                assert!(
                    result.status,
                    "NASM failed for {}:\n-----\n{}\n-----\n{}\n-----\n",
                    src.display(),
                    result.stdout,
                    result.stderr
                );

                objects.push(obj_path);
            }
        } else if use_prebuilt_nasm() {
            let prebuilt_dir = self.manifest_dir.join("builder").join("prebuilt-nasm");
            for src in &self.files {
                let obj_name = src
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".asm", ".obj");
                let obj_path = self.out_dir.join(&obj_name);
                let base_name = obj_name.strip_suffix(".obj").unwrap_or(&obj_name);
                let prebuilt_src = prebuilt_dir.join(format!("{base_name}.obj"));
                if prebuilt_src.exists() {
                    fs::copy(&prebuilt_src, &obj_path)
                        .expect("Failed to copy prebuilt NASM object");
                } else {
                    panic!("Prebuilt NASM object not found: {}", prebuilt_src.display());
                }
                objects.push(obj_path);
            }
        } else {
            panic!("NASM command not found! Build cannot continue.");
        }

        objects
    }
}
