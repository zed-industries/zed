// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides x86-specific implementations of CRC-32/ISCSI calculations using
//! fusion techniques.

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

pub(crate) mod avx512_pclmulqdq;
pub(crate) mod avx512_vpclmulqdq;
pub(crate) mod sse_pclmulqdq;
