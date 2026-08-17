// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides aarch64-specific implementations of CRC-32/ISCSI calculations using
//! fusion techniques.

#![cfg(target_arch = "aarch64")]

pub(crate) mod crc_pmull;
pub(crate) mod crc_pmull_sha3;
