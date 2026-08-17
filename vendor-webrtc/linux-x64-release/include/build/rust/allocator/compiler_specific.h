// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file has been copied from //base/compiler_specific.h (and then
// significantly trimmed to just the APIs / macros needed by //build/rust/std).
//
// TODO(crbug.com/40279749): Avoid code duplication / reuse code.

#ifndef BUILD_RUST_ALLOCATOR_COMPILER_SPECIFIC_H_
#define BUILD_RUST_ALLOCATOR_COMPILER_SPECIFIC_H_

#include "build/build_config.h"

#if defined(COMPILER_MSVC) && !defined(__clang__)
#error "Only clang-cl is supported on Windows, see https://crbug.com/988071"
#endif

#if defined(__has_attribute)
#define HAS_ATTRIBUTE(x) __has_attribute(x)
#else
#define HAS_ATTRIBUTE(x) 0
#endif

// Annotate a function indicating it should not be inlined.
// Use like:
//   NOINLINE void DoStuff() { ... }
#if defined(__clang__) && HAS_ATTRIBUTE(noinline)
#define NOINLINE [[clang::noinline]]
#elif defined(COMPILER_GCC) && HAS_ATTRIBUTE(noinline)
#define NOINLINE __attribute__((noinline))
#elif defined(COMPILER_MSVC)
#define NOINLINE __declspec(noinline)
#else
#define NOINLINE
#endif

#endif  // BUILD_RUST_ALLOCATOR_COMPILER_SPECIFIC_H_
