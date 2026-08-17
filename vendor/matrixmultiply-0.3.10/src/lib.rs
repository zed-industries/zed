// Copyright 2016 - 2023 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//!
//! General matrix multiplication for f32, f64, and complex matrices. Operates on
//! matrices with general layout (they can use arbitrary row and column stride).
//!
//! This crate uses the same macro/microkernel approach to matrix multiplication as
//! the [BLIS][bl] project.
//!
//! We presently provide a few good microkernels, portable and for x86-64 and
//! AArch64 NEON, and only one operation: the general matrix-matrix
//! multiplication (“gemm”).
//!
//! [bl]: https://github.com/flame/blis
//!
//! ## Matrix Representation
//!
//! **matrixmultiply** supports matrices with general stride, so a matrix
//! is passed using a pointer and four integers:
//!
//! - `a: *const f32`, pointer to the first element in the matrix
//! - `m: usize`, number of rows
//! - `k: usize`, number of columns
//! - `rsa: isize`, row stride
//! - `csa: isize`, column stride
//!
//! In this example, A is a m by k matrix. `a` is a pointer to the element at
//! index *0, 0*.
//!
//! The *row stride* is the pointer offset (in number of elements) to the
//! element on the next row. It’s the distance from element *i, j* to *i + 1,
//! j*.
//!
//! The *column stride* is the pointer offset (in number of elements) to the
//! element in the next column. It’s the distance from element *i, j* to *i,
//! j + 1*.
//!
//! For example for a contiguous matrix, row major strides are *rsa=k,
//! csa=1* and column major strides are *rsa=1, csa=m*.
//!
//! Strides can be negative or even zero, but for a mutable matrix elements
//! may not alias each other.
//!
//! ## Portability and Performance
//!
//! - The default kernels are written in portable Rust and available
//!   on all targets. These may depend on autovectorization to perform well.
//!
//! - *x86* and *x86-64* features can be detected at runtime by default or
//!   compile time (if enabled), and the following kernel variants are
//!   implemented:
//!
//!   - `fma`
//!   - `avx`
//!   - `sse2`
//!
//! - *aarch64* features can be detected at runtime by default or compile time
//!   (if enabled), and the following kernel variants are implemented:
//!
//!   - `neon`
//!
//! ## Features
//!
//! ### `std`
//!
//! `std` is enabled by default.
//!
//! This crate can be used without the standard library (`#![no_std]`) by
//! disabling the default `std` feature. To do so, use this in your
//! `Cargo.toml`:
//!
//! ```toml
//! matrixmultiply = { version = "0.3", default-features = false }
//! ```
//!
//! Runtime CPU feature detection is available **only** when `std` is enabled.
//! Without the `std` feature, the crate uses special CPU features only if they
//! are enabled at compile time. (To enable CPU features at compile time, pass
//! the relevant
//! [`target-cpu`](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu)
//! or
//! [`target-feature`](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-feature)
//! option to `rustc`.)
//!
//! ### `threading`
//!
//! `threading` is an optional crate feature
//!
//! Threading enables multithreading for the operations. The environment variable
//! `MATMUL_NUM_THREADS` decides how many threads are used at maximum. At the moment 1-4 are
//! supported and the default is the number of physical cpus (as detected by `num_cpus`).
//!
//! ### `cgemm`
//!
//! `cgemm` is an optional crate feature.
//!
//! It enables the `cgemm` and `zgemm` methods for complex matrix multiplication.
//! This is an **experimental feature** and not yet as performant as the float kernels on x86.
//!
//! The complex representation we use is `[f64; 2]`.
//!
//! ### `constconf`
//!
//! `constconf` is an optional feature. When enabled, cache-sensitive parameters of
//! the gemm implementations can be tweaked *at compile time* by defining the following variables:
//!
//! - `MATMUL_SGEMM_MC`
//!   (And so on, for S, D, C, ZGEMM and with NC, KC or MC).
//!
//! ## Other Notes
//!
//! The functions in this crate are thread safe, as long as the destination
//! matrix is distinct.
//!
//! ## Rust Version
//!
//! This version requires Rust 1.41.1 or later; the crate follows a carefully
//! considered upgrade policy, where updating the minimum Rust version is not a breaking
//! change.
//!
//! Some features are enabled with later versions: from Rust 1.61 AArch64 NEON support.

#![doc(html_root_url = "https://docs.rs/matrixmultiply/0.3/")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate core;

#[macro_use]
mod debugmacros;
#[macro_use]
mod loopmacros;

mod archparam_defaults;

#[cfg(feature = "constconf")]
mod archparam;
#[cfg(feature = "constconf")]
mod constparse;

#[cfg(not(feature = "constconf"))]
pub(crate) use archparam_defaults as archparam;

mod gemm;
mod kernel;
mod packing;
mod ptr;
mod threading;

mod aligned_alloc;
mod util;

#[macro_use]
mod archmacros;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
mod x86;
#[cfg(any(target_arch = "aarch64"))]
#[macro_use]
mod aarch64;

mod dgemm_kernel;
mod sgemm_kernel;

pub use crate::gemm::dgemm;
pub use crate::gemm::sgemm;

#[cfg(feature = "cgemm")]
#[macro_use]
mod cgemm_common;
#[cfg(feature = "cgemm")]
mod cgemm_kernel;
#[cfg(feature = "cgemm")]
mod zgemm_kernel;

#[cfg(feature = "cgemm")]
pub use crate::gemm::cgemm;
#[cfg(feature = "cgemm")]
pub use crate::gemm::zgemm;
#[cfg(feature = "cgemm")]
pub use crate::gemm::CGemmOption;
