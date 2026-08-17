/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
extern crate rustc_version;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_build_vars(output_path: &Path) {
    let rust_version = rustc_version::version().expect("Could not retrieve rustc version");
    let mut f =
        File::create(output_path.join("build_env.rs")).expect("Could not create build environment");
    f.write_all(format!("const RUST_VERSION: &str = \"{rust_version}\";").as_bytes())
        .expect("Unable to write rust version");
    f.flush().expect("failed to flush");
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    generate_build_vars(&out_path);
}
