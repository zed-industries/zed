// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_BASE_CONTAINERS_SPAN_H_
#define TOOLS_CLANG_PLUGINS_TESTS_BASE_CONTAINERS_SPAN_H_

#include "base/memory/raw_ptr.h"

namespace base {

template <typename T, int Extent = 0, typename InternalPtrType = T*>
class span {
 public:
  span() = default;

  template <unsigned long N>
  span(T (&arr)[N]) {}

  InternalPtrType data_;
  unsigned size_;
};

// raw_span is a span with a pointer type that has a non-trivial dtor. We make a
// similar type here. We don't use raw_ptr directly because without BRP enabled,
// it also has a trivial dtor.

template <class T>
struct NonTrivialRawPtr {
  NonTrivialRawPtr() {}
  constexpr ~NonTrivialRawPtr() {}
  raw_ptr<T> ptr;
};

template <typename T>
using raw_span = span<T, 0, NonTrivialRawPtr<T>>;

}  // namespace base

#endif  // TOOLS_CLANG_PLUGINS_TESTS_BASE_CONTAINERS_SPAN_H_
