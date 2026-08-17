/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate log;
extern crate proc_macro2;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate toml;

#[cfg(feature = "unstable_ir")]
pub mod bindgen;
#[cfg(not(feature = "unstable_ir"))]
mod bindgen;

pub use crate::bindgen::*;

use std::path::Path;

/// A utility function for build scripts to generate bindings for a crate, using
/// a `cbindgen.toml` if it exists.
pub fn generate<P: AsRef<Path>>(crate_dir: P) -> Result<Bindings, Error> {
    let config = Config::from_root_or_default(crate_dir.as_ref());

    generate_with_config(crate_dir, config)
}

/// A utility function for build scripts to generate bindings for a crate with a
/// custom config.
pub fn generate_with_config<P: AsRef<Path>>(
    crate_dir: P,
    config: Config,
) -> Result<Bindings, Error> {
    Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .generate()
}
