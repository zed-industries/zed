// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_
#define TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_

// Marks a field as excluded from the raw_ptr usage enforcement clang plugin.
// Example: RAW_PTR_EXCLUSION Foo* foo_;
#define RAW_PTR_EXCLUSION __attribute__((annotate("raw_ptr_exclusion")))

#endif  // TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_
