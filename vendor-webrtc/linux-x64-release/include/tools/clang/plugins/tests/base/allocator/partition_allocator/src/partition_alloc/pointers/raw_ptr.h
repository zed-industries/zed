// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_H_
#define TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_H_

namespace base {

template <typename T>
class raw_ptr {
  T* ptr;
};

template <typename T>
class raw_ref {
  T* ref;
};

}  // namespace base

using base::raw_ptr;
using base::raw_ref;

#endif  // TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_H_
