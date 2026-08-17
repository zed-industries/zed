/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
/// Possible errors that can occur during Cargo.toml parsing.
pub enum Error {
    /// Error during reading of Cargo.toml
    Io(io::Error),
    /// Deserialization error
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
            Error::Toml(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Toml(ref err) => Some(err),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Package {
    pub name: String,
}

/// Parse the Cargo.toml for a given path
pub fn manifest(manifest_path: &Path) -> Result<Manifest, Error> {
    let mut s = String::new();
    let mut f = File::open(manifest_path)?;
    f.read_to_string(&mut s)?;

    toml::from_str::<Manifest>(&s).map_err(|x| x.into())
}
