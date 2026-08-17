// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NUMERICS_SAFE_CONVERSIONS_ARM_IMPL_H_
#define BASE_NUMERICS_SAFE_CONVERSIONS_ARM_IMPL_H_

// IWYU pragma: private, include "base/numerics/safe_conversions.h"

#include <stdint.h>

#include <algorithm>
#include <concepts>
#include <type_traits>

#include "base/numerics/safe_conversions_impl.h"

namespace base {
namespace internal {

// Fast saturation to a destination type.
template <typename Dst, typename Src>
struct SaturateFastAsmOp {
  static constexpr bool is_supported =
      kEnableAsmCode && std::signed_integral<Src> && std::integral<Dst> &&
      kIntegerBitsPlusSign<Src> <= kIntegerBitsPlusSign<int32_t> &&
      kIntegerBitsPlusSign<Dst> <= kIntegerBitsPlusSign<int32_t> &&
      !kIsTypeInRangeForNumericType<Dst, Src>;

  __attribute__((always_inline)) static Dst Do(Src value) {
    int32_t src = value;
    if constexpr (std::is_signed_v<Dst>) {
      int32_t result;
      asm("ssat %[dst], %[shift], %[src]"
          : [dst] "=r"(result)
          : [src] "r"(src), [shift] "n"(
                                std::min(kIntegerBitsPlusSign<Dst>, 32)));
      return static_cast<Dst>(result);
    } else {
      uint32_t result;
      asm("usat %[dst], %[shift], %[src]"
          : [dst] "=r"(result)
          : [src] "r"(src), [shift] "n"(
                                std::min(kIntegerBitsPlusSign<Dst>, 31)));
      return static_cast<Dst>(result);
    }
  }
};

}  // namespace internal
}  // namespace base

#endif  // BASE_NUMERICS_SAFE_CONVERSIONS_ARM_IMPL_H_
