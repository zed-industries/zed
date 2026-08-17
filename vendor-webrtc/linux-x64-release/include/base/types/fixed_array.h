// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_FIXED_ARRAY_H_
#define BASE_TYPES_FIXED_ARRAY_H_

#include <stddef.h>

#include <memory>
#include <type_traits>

#include "third_party/abseil-cpp/absl/container/fixed_array.h"

namespace base {

// `FixedArray` provides `absl::FixedArray` in Chromium, but when `T` is
// trivially-default-constructible, forces the no-default-value constructor to
// initialize the elements to `T()`, instead of leaving them uninitialized. This
// makes `base::FixedArray` behave like `std::vector` instead of `std::array`
// and avoids the risk of UB.

// Trivially-default-constructible case: no-value constructor should init
template <typename T,
          size_t N = absl::kFixedArrayUseDefault,
          typename A = std::allocator<T>>
class FixedArray : public absl::FixedArray<T, N, A> {
 public:
  using absl::FixedArray<T, N, A>::FixedArray;
  explicit FixedArray(absl::FixedArray<T, N, A>::size_type n,
                      const absl::FixedArray<T, N, A>::allocator_type& a =
                          typename absl::FixedArray<T, N, A>::allocator_type())
      : FixedArray(n, T(), a) {}
};

// Non-trivially-default-constructible case: Pass through all constructors
template <typename T, size_t N, typename A>
  requires(!std::is_trivially_default_constructible_v<T>)
struct FixedArray<T, N, A> : public absl::FixedArray<T, N, A> {
  using absl::FixedArray<T, N, A>::FixedArray;
};

}  // namespace base

#endif  // BASE_TYPES_FIXED_ARRAY_H_
