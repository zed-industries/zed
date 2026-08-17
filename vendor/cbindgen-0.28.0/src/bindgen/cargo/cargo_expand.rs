/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::config::Profile;
use std::env;
use std::error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::{from_utf8, Utf8Error};

extern crate tempfile;
use self::tempfile::Builder;

#[derive(Debug)]
/// Possible errors that can occur during `rustc -Zunpretty=expanded`.
pub enum Error {
    /// Error during creation of temporary directory
    Io(io::Error),
    /// Output of `cargo metadata` was not valid utf8
    Utf8(Utf8Error),
    /// Error during execution of `cargo rustc -Zunpretty=expanded`
    Compile(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
            Error::Utf8(ref err) => err.fmt(f),
            Error::Compile(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Compile(..) => None,
        }
    }
}

/// Use rustc to expand and pretty print the crate into a single file,
/// removing any macros in the process.
#[allow(clippy::too_many_arguments)]
pub fn expand(
    manifest_path: &Path,
    crate_name: &str,
    version: Option<&str>,
    use_tempdir: bool,
    expand_all_features: bool,
    expand_default_features: bool,
    expand_features: &Option<Vec<String>>,
    profile: Profile,
) -> Result<String, Error> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
    let mut cmd = Command::new(cargo);

    let mut _temp_dir = None; // drop guard
    if use_tempdir {
        _temp_dir = Some(Builder::new().prefix("cbindgen-expand").tempdir()?);
        cmd.env("CARGO_TARGET_DIR", _temp_dir.unwrap().path());
    } else if let Ok(ref path) = env::var("CARGO_EXPAND_TARGET_DIR") {
        cmd.env("CARGO_TARGET_DIR", path);
    } else if let Ok(ref path) = env::var("OUT_DIR") {
        // When cbindgen was started programatically from a build.rs file, Cargo is running and
        // locking the default target directory. In this case we need to use another directory,
        // else we would end up in a deadlock. If Cargo is running `OUT_DIR` will be set, so we
        // can use a directory relative to that.
        cmd.env("CARGO_TARGET_DIR", PathBuf::from(path).join("expanded"));
    }

    // Set this variable so that we don't call it recursively if we expand a crate that is using
    // cbindgen
    cmd.env("_CBINDGEN_IS_RUNNING", "1");

    cmd.arg("rustc");
    cmd.arg("--lib");
    // When build with the release profile we can't choose the `check` profile.
    if profile != Profile::Release {
        cmd.arg("--profile=check");
    }
    cmd.arg("--manifest-path");
    cmd.arg(manifest_path);
    if let Some(features) = expand_features {
        cmd.arg("--features");
        let mut features_str = String::new();
        for (index, feature) in features.iter().enumerate() {
            if index != 0 {
                features_str.push(' ');
            }
            features_str.push_str(feature);
        }
        cmd.arg(features_str);
    }
    if expand_all_features {
        cmd.arg("--all-features");
    }
    if !expand_default_features {
        cmd.arg("--no-default-features");
    }
    match profile {
        Profile::Debug => {}
        Profile::Release => {
            cmd.arg("--release");
        }
    }
    cmd.arg("-p");
    let mut package = crate_name.to_owned();
    if let Some(version) = version {
        package.push(':');
        package.push_str(version);
    }
    cmd.arg(&package);
    cmd.arg("--verbose");
    cmd.arg("--");
    cmd.arg("-Zunpretty=expanded");
    info!("Command: {:?}", cmd);
    let output = cmd.output()?;

    let src = from_utf8(&output.stdout)?.to_owned();
    let error = from_utf8(&output.stderr)?.to_owned();

    if src.is_empty() {
        Err(Error::Compile(error))
    } else {
        Ok(src)
    }
}
