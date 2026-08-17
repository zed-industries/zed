// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_BASE_CONTAINERS_SPAN_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_BASE_CONTAINERS_SPAN_H_

#include "base/memory/raw_ptr.h"

namespace base {

template <typename T, int Extent = 0, typename InternalPtrType = T*>
class span {
 public:
  InternalPtrType data_;
};

template <typename T>
using raw_span = span<T, 0, raw_ptr<T>>;
}  // namespace base

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_BASE_CONTAINERS_SPAN_H_
