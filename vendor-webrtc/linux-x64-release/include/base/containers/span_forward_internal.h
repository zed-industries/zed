// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_SPAN_FORWARD_INTERNAL_H_
#define BASE_CONTAINERS_SPAN_FORWARD_INTERNAL_H_

namespace base {

// [span.syn]: Constants
inline constexpr size_t dynamic_extent = std::numeric_limits<size_t>::max();

// [views.span]: class template `span<>`
template <typename ElementType,
          size_t Extent = dynamic_extent,
          // Storage pointer customization. By default this is not a
          // `raw_ptr<>`, since `span` is mostly used for stack variables. Use
          // `raw_span` instead for class fields, which sets this to
          // `raw_ptr<T>`.
          typename InternalPtrType = ElementType*>
class span;

}  // namespace base

#endif  // BASE_CONTAINERS_SPAN_FORWARD_INTERNAL_H_
