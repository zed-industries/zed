// Copyright 2016 - 2021 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! architechture specific parameters
//!
//! NC: Columns in C, B that we handle at a time. (5th loop)
//! KC: Rows of Bj at a time (4th loop)
//! MC: Rows of Ap at a time. (3rd loop)

use crate::archparam_defaults;
use crate::constparse::parse_unwarp;

macro_rules! conf_env_or_default {
    ($env_name:tt, $default:expr) => {
        match option_env!($env_name) {
            Some(x) => parse_unwarp(x),
            None => $default,
        }
    }
}

pub(crate) const S_NC: usize = conf_env_or_default!("MATMUL_SGEMM_NC", archparam_defaults::S_NC);
pub(crate) const S_KC: usize = conf_env_or_default!("MATMUL_SGEMM_KC", archparam_defaults::S_KC);
pub(crate) const S_MC: usize = conf_env_or_default!("MATMUL_SGEMM_MC", archparam_defaults::S_MC);

pub(crate) const D_NC: usize = conf_env_or_default!("MATMUL_DGEMM_NC", archparam_defaults::D_NC);
pub(crate) const D_KC: usize = conf_env_or_default!("MATMUL_DGEMM_KC", archparam_defaults::D_KC);
pub(crate) const D_MC: usize = conf_env_or_default!("MATMUL_DGEMM_MC", archparam_defaults::D_MC);

#[cfg(feature = "cgemm")]
pub(crate) const C_NC: usize = conf_env_or_default!("MATMUL_CGEMM_NC", archparam_defaults::C_NC);
#[cfg(feature = "cgemm")]
pub(crate) const C_KC: usize = conf_env_or_default!("MATMUL_CGEMM_KC", archparam_defaults::C_KC);
#[cfg(feature = "cgemm")]
pub(crate) const C_MC: usize = conf_env_or_default!("MATMUL_CGEMM_MC", archparam_defaults::C_MC);

#[cfg(feature = "cgemm")]
pub(crate) const Z_NC: usize = conf_env_or_default!("MATMUL_ZGEMM_NC", archparam_defaults::Z_NC);
#[cfg(feature = "cgemm")]
pub(crate) const Z_KC: usize = conf_env_or_default!("MATMUL_ZGEMM_KC", archparam_defaults::Z_KC);
#[cfg(feature = "cgemm")]
pub(crate) const Z_MC: usize = conf_env_or_default!("MATMUL_ZGEMM_MC", archparam_defaults::Z_MC);
