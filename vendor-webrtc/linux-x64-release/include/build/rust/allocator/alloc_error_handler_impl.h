// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BUILD_RUST_ALLOCATOR_ALLOC_ERROR_HANDLER_IMPL_H_
#define BUILD_RUST_ALLOCATOR_ALLOC_ERROR_HANDLER_IMPL_H_

// This header exposes to Rust a C++ implementation of quickly crashing after an
// allocation error.  (The API below is called from `__rust_alloc_error_handler`
// in `lib.rs`.)
//
// TODO(lukasza): Investigate if we can delete this `.h` / `.cc` and just call
// `std::process::abort()` (or something else?) directly from `.rs`.  The main
// open question is how much we care about `NO_CODE_FOLDING`.
namespace rust_allocator_internal {

void alloc_error_handler_impl();

}  // namespace rust_allocator_internal

#endif  // BUILD_RUST_ALLOCATOR_ALLOC_ERROR_HANDLER_IMPL_H_
