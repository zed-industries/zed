// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides x86_64-specific implementations of the ArchOps trait.

#![cfg(target_arch = "x86_64")]

pub mod avx512;
pub mod avx512_vpclmulqdq;
