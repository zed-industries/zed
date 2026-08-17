// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SIZE_ASSERTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SIZE_ASSERTIONS_H_

#include <cstdlib>

namespace WTF {
namespace internal {

template <size_t ActualSize, size_t ExpectedSize>
struct SizesEqual {
  static constexpr bool value = ActualSize == ExpectedSize;
};

}  // namespace internal
}  // namespace WTF

// The ASSERT_SIZE macro can be used to check that a given type is the same size
// as another type. This is useful to visualize where the space is being used in
// a class, as well as give a useful compile error message when the size doesn't
// match the expected value.
#define ASSERT_SIZE(type, same_size_type)                                   \
  static_assert(::WTF::internal::SizesEqual<sizeof(type),                   \
                                            sizeof(same_size_type)>::value, \
                #type " should stay small")

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SIZE_ASSERTIONS_H_
