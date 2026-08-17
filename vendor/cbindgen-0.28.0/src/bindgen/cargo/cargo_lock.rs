/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
/// Possible errors that can occur during Cargo.toml parsing.
pub enum Error {
    /// Error during reading of Cargo.toml
    #[allow(dead_code)]
    Io(io::Error),
    /// Deserialization error
    #[allow(dead_code)]
    Toml(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Lock {
    pub root: Option<Package>,
    pub package: Option<Vec<Package>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    /// A list of dependencies formatted like "NAME VERSION-OPT REGISTRY-OPT"
    pub dependencies: Option<Vec<String>>,
}

/// Parse the Cargo.toml for a given path
pub fn lock(manifest_path: &Path) -> Result<Lock, Error> {
    let mut s = String::new();
    let mut f = File::open(manifest_path)?;
    f.read_to_string(&mut s)?;

    toml::from_str::<Lock>(&s).map_err(|x| x.into())
}
