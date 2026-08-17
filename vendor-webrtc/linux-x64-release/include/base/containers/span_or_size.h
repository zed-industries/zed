// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_SPAN_OR_SIZE_H_
#define BASE_CONTAINERS_SPAN_OR_SIZE_H_

#include <stddef.h>

#include <variant>

#include "base/containers/span.h"
#include "base/functional/overloaded.h"
#include "base/types/optional_ref.h"

namespace base {

// `SpanOrSize<T>` contains either a `span<T>` or just the size of data.  This
// is useful if the data is not retained in some scenarios, but size needs to be
// available in all the scenarios.
template <typename T>
class SpanOrSize {
 public:
  explicit constexpr SpanOrSize(base::span<T> span) : span_or_size_(span) {}
  explicit constexpr SpanOrSize(size_t size) : span_or_size_(size) {}

  ~SpanOrSize() = default;

  // `SpanOrSize` is copyable and movable (just like `span` and `size_t`).
  SpanOrSize(const SpanOrSize&) = default;
  SpanOrSize& operator=(const SpanOrSize&) = default;
  SpanOrSize(SpanOrSize&&) = default;
  SpanOrSize& operator=(SpanOrSize&&) = default;

  constexpr T* ptr_or_null_if_no_data() const {
    return std::visit(Overloaded{
                          [](const base::span<T>& span) { return span.data(); },
                          [](size_t size) -> T* { return nullptr; },
                      },
                      span_or_size_);
  }

  constexpr size_t size() const {
    return std::visit(Overloaded{
                          [](const base::span<T>& span) { return span.size(); },
                          [](size_t size) { return size; },
                      },
                      span_or_size_);
  }

  constexpr optional_ref<const base::span<T>> span() const {
    return std::visit(
        Overloaded{
            [](const base::span<T>& span) {
              return optional_ref<const base::span<T>>(span);
            },
            [](size_t size) { return optional_ref<const base::span<T>>(); },
        },
        span_or_size_);
  }

 private:
  std::variant<base::span<T>, size_t> span_or_size_;
};

}  // namespace base

#endif  // BASE_CONTAINERS_SPAN_OR_SIZE_H_
