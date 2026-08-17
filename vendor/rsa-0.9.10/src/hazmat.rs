//! ⚠️ Low-level "hazmat" RSA functions.
//!
//! # ☢️️ WARNING: HAZARDOUS API ☢️
//!
//! This module holds functions that apply RSA's core encryption and decryption
//! primitives to raw data without adding or removing appropriate padding. A
//! well-reviewed padding scheme is crucial to the security of RSA, so there are
//! very few valid uses cases for this API. It's intended to be used for
//! implementing well-reviewed higher-level constructions.
//!
//! We do NOT recommend using it to implement any algorithm which has not
//! received extensive peer review by cryptographers.

pub use crate::algorithms::rsa::{rsa_decrypt, rsa_decrypt_and_check, rsa_encrypt};
