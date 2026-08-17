// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_RAW_SPAN_H_
#define BASE_MEMORY_RAW_SPAN_H_

#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"

namespace base {

// raw_span<T> is a type that provides the spatial safety of span<T> along
// with the temporal safety of raw_ptr<T>. This is intended to be a safer
// replacement for classes that store pointer + size fields. As is the case
// with raw_ptr<>, raw_span<> should be used for class members only, with
// ordinary span<> used for function arguments and the like. Note that
// raw_span<> will implicitly convert to span<> for ease of use in these
// cases.

template <typename T, RawPtrTraits Traits = RawPtrTraits::kEmpty>
using raw_span =
    span<T, dynamic_extent, raw_ptr<T, Traits | AllowPtrArithmetic>>;

template <typename T>
auto ExtractAsDanglingSpan(raw_span<T>& arg) {
  raw_span<T, DisableDanglingPtrDetection> result =
      std::exchange(arg, raw_span<T>());
  return result;
}

}  // namespace base

#endif  // BASE_MEMORY_RAW_SPAN_H_
