// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file has been copied from //base/debug/alias.h (and then trimmed to just
// the APIs / macros needed by //build/rust/std;  additionally the APIs were
// moved into the `build_rust_std` namespace).
//
// TODO(crbug.com/40279749): Avoid code duplication / reuse code.

#ifndef BUILD_RUST_ALLOCATOR_ALIAS_H_
#define BUILD_RUST_ALLOCATOR_ALIAS_H_

#include <stddef.h>

namespace build_rust_std {
namespace debug {

// Make the optimizer think that |var| is aliased. This can be used to prevent a
// local variable from being optimized out (which is something that
// `NO_CODE_FOLDING` macro definition below depends on).  See
// //base/debug/alias.h for more details.
void Alias(const void* var);

}  // namespace debug

}  // namespace build_rust_std

// Prevent code folding (where a linker identifies functions that are
// bit-identical and overlays them, which saves space but it leads to confusing
// call stacks because multiple symbols are at the same address).  See
// //base/debug/alias.h for more details.
#define NO_CODE_FOLDING()           \
  const int line_number = __LINE__; \
  build_rust_std::debug::Alias(&line_number)

#endif  // BUILD_RUST_ALLOCATOR_ALIAS_H_
