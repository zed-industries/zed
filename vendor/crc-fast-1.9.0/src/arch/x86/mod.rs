// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides x86-specific implementations of the ArchOps trait.

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

pub mod sse;
