/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[allow(clippy::module_inception)]
mod cargo;
pub(crate) mod cargo_expand;
pub(crate) mod cargo_lock;
pub(crate) mod cargo_metadata;
pub(crate) mod cargo_toml;

pub(crate) use self::cargo::*;
