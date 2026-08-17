// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_AUGMENTATIONS_COMPILER_SPECIFIC_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_AUGMENTATIONS_COMPILER_SPECIFIC_H_

// Extensions for PA's copy of `//base/compiler_specific.h`.

#include "partition_alloc/partition_alloc_base/compiler_specific.h"

// Indicate whether `operator<=>()` is supported by both language and library.
// This can be removed once the minimum C++ version is C++20.
#if __has_include(<version>)
#include <version>
#endif
#if defined(__cpp_lib_three_way_comparison) && \
    __cpp_lib_three_way_comparison >= 201907L
#define PA_HAVE_SPACESHIP_OPERATOR 1
#else
#define PA_HAVE_SPACESHIP_OPERATOR 0
#endif

// PA_ATTRIBUTE_RETURNS_NONNULL
//
// Tells the compiler that a function never returns a null pointer.
// Sourced from Abseil's `attributes.h`.
#if PA_HAS_ATTRIBUTE(returns_nonnull)
#define PA_ATTRIBUTE_RETURNS_NONNULL __attribute__((returns_nonnull))
#else
#define PA_ATTRIBUTE_RETURNS_NONNULL
#endif

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_AUGMENTATIONS_COMPILER_SPECIFIC_H_
