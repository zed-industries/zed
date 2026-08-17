/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_DIVIDE_ROUND_H_
#define RTC_BASE_NUMERICS_DIVIDE_ROUND_H_

#include <type_traits>

#include "rtc_base/checks.h"
#include "rtc_base/numerics/safe_compare.h"

namespace webrtc {

template <typename Dividend, typename Divisor>
inline auto constexpr DivideRoundUp(Dividend dividend, Divisor divisor) {
  static_assert(std::is_integral<Dividend>(), "");
  static_assert(std::is_integral<Divisor>(), "");
  RTC_DCHECK_GE(dividend, 0);
  RTC_DCHECK_GT(divisor, 0);

  auto quotient = dividend / divisor;
  auto remainder = dividend % divisor;
  return quotient + (remainder > 0 ? 1 : 0);
}

template <typename Dividend, typename Divisor>
inline auto constexpr DivideRoundToNearest(Dividend dividend, Divisor divisor) {
  static_assert(std::is_integral<Dividend>(), "");
  static_assert(std::is_integral<Divisor>(), "");
  RTC_DCHECK_GT(divisor, 0);

  if (dividend < Dividend{0}) {
    auto half_of_divisor = divisor / 2;
    auto quotient = dividend / divisor;
    auto remainder = dividend % divisor;
    if (SafeGt(-remainder, half_of_divisor)) {
      --quotient;
    }
    return quotient;
  }

  auto half_of_divisor = (divisor - 1) / 2;
  auto quotient = dividend / divisor;
  auto remainder = dividend % divisor;
  if (SafeGt(remainder, half_of_divisor)) {
    ++quotient;
  }
  return quotient;
}

}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_DIVIDE_ROUND_H_
