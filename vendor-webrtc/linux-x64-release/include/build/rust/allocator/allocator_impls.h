// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BUILD_RUST_ALLOCATOR_ALLOCATOR_IMPLS_H_
#define BUILD_RUST_ALLOCATOR_ALLOCATOR_IMPLS_H_

#include <cstddef>

// This header exposes a C++ allocator (e.g. PartitionAlloc) to Rust.
// The APIs below are called from `impl GlobalAlloc` in `lib.rs`.
namespace rust_allocator_internal {

unsigned char* alloc(size_t size, size_t align);
void dealloc(unsigned char* p, size_t size, size_t align);
unsigned char* realloc(unsigned char* p,
                       size_t old_size,
                       size_t align,
                       size_t new_size);
unsigned char* alloc_zeroed(size_t size, size_t align);

}  // namespace rust_allocator_internal

#endif  // BUILD_RUST_ALLOCATOR_ALLOCATOR_IMPLS_H_
