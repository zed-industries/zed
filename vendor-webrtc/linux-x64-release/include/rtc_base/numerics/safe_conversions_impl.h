/*
 *  Copyright 2014 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Borrowed from Chromium's src/base/numerics/safe_conversions_impl.h.

#ifndef RTC_BASE_NUMERICS_SAFE_CONVERSIONS_IMPL_H_
#define RTC_BASE_NUMERICS_SAFE_CONVERSIONS_IMPL_H_

#include <cstddef>
#include <limits>

namespace webrtc {
namespace internal {

enum DstSign { DST_UNSIGNED, DST_SIGNED };

enum SrcSign { SRC_UNSIGNED, SRC_SIGNED };

enum DstRange { OVERLAPS_RANGE, CONTAINS_RANGE };

// Helper templates to statically determine if our destination type can contain
// all values represented by the source type.

template <typename Dst,
          typename Src,
          DstSign IsDstSigned =
              std::numeric_limits<Dst>::is_signed ? DST_SIGNED : DST_UNSIGNED,
          SrcSign IsSrcSigned =
              std::numeric_limits<Src>::is_signed ? SRC_SIGNED : SRC_UNSIGNED>
struct StaticRangeCheck {};

template <typename Dst, typename Src>
struct StaticRangeCheck<Dst, Src, DST_SIGNED, SRC_SIGNED> {
  typedef std::numeric_limits<Dst> DstLimits;
  typedef std::numeric_limits<Src> SrcLimits;
  // Compare based on max_exponent, which we must compute for integrals.
  static const size_t kDstMaxExponent =
      DstLimits::is_iec559 ? DstLimits::max_exponent : (sizeof(Dst) * 8 - 1);
  static const size_t kSrcMaxExponent =
      SrcLimits::is_iec559 ? SrcLimits::max_exponent : (sizeof(Src) * 8 - 1);
  static const DstRange value =
      kDstMaxExponent >= kSrcMaxExponent ? CONTAINS_RANGE : OVERLAPS_RANGE;
};

template <typename Dst, typename Src>
struct StaticRangeCheck<Dst, Src, DST_UNSIGNED, SRC_UNSIGNED> {
  static const DstRange value =
      sizeof(Dst) >= sizeof(Src) ? CONTAINS_RANGE : OVERLAPS_RANGE;
};

template <typename Dst, typename Src>
struct StaticRangeCheck<Dst, Src, DST_SIGNED, SRC_UNSIGNED> {
  typedef std::numeric_limits<Dst> DstLimits;
  typedef std::numeric_limits<Src> SrcLimits;
  // Compare based on max_exponent, which we must compute for integrals.
  static const size_t kDstMaxExponent =
      DstLimits::is_iec559 ? DstLimits::max_exponent : (sizeof(Dst) * 8 - 1);
  static const size_t kSrcMaxExponent = sizeof(Src) * 8;
  static const DstRange value =
      kDstMaxExponent >= kSrcMaxExponent ? CONTAINS_RANGE : OVERLAPS_RANGE;
};

template <typename Dst, typename Src>
struct StaticRangeCheck<Dst, Src, DST_UNSIGNED, SRC_SIGNED> {
  static const DstRange value = OVERLAPS_RANGE;
};

enum RangeCheckResult {
  TYPE_VALID = 0,      // Value can be represented by the destination type.
  TYPE_UNDERFLOW = 1,  // Value would overflow.
  TYPE_OVERFLOW = 2,   // Value would underflow.
  TYPE_INVALID = 3     // Source value is invalid (i.e. NaN).
};

// This macro creates a RangeCheckResult from an upper and lower bound
// check by taking advantage of the fact that only NaN can be out of range in
// both directions at once.
#define BASE_NUMERIC_RANGE_CHECK_RESULT(is_in_upper_bound, is_in_lower_bound) \
  RangeCheckResult(((is_in_upper_bound) ? 0 : TYPE_OVERFLOW) |                \
                   ((is_in_lower_bound) ? 0 : TYPE_UNDERFLOW))

template <typename Dst,
          typename Src,
          DstSign IsDstSigned =
              std::numeric_limits<Dst>::is_signed ? DST_SIGNED : DST_UNSIGNED,
          SrcSign IsSrcSigned =
              std::numeric_limits<Src>::is_signed ? SRC_SIGNED : SRC_UNSIGNED,
          DstRange IsSrcRangeContained = StaticRangeCheck<Dst, Src>::value>
struct RangeCheckImpl {};

// The following templates are for ranges that must be verified at runtime. We
// split it into checks based on signedness to avoid confusing casts and
// compiler warnings on signed an unsigned comparisons.

// Dst range always contains the result: nothing to check.
template <typename Dst, typename Src, DstSign IsDstSigned, SrcSign IsSrcSigned>
struct RangeCheckImpl<Dst, Src, IsDstSigned, IsSrcSigned, CONTAINS_RANGE> {
  static constexpr RangeCheckResult Check(Src /* value */) {
    return TYPE_VALID;
  }
};

// Signed to signed narrowing.
template <typename Dst, typename Src>
struct RangeCheckImpl<Dst, Src, DST_SIGNED, SRC_SIGNED, OVERLAPS_RANGE> {
  static constexpr RangeCheckResult Check(Src value) {
    typedef std::numeric_limits<Dst> DstLimits;
    return DstLimits::is_iec559
               ? BASE_NUMERIC_RANGE_CHECK_RESULT(
                     value <= static_cast<Src>(DstLimits::max()),
                     value >= static_cast<Src>(DstLimits::max() * -1))
               : BASE_NUMERIC_RANGE_CHECK_RESULT(
                     value <= static_cast<Src>(DstLimits::max()),
                     value >= static_cast<Src>(DstLimits::min()));
  }
};

// Unsigned to unsigned narrowing.
template <typename Dst, typename Src>
struct RangeCheckImpl<Dst, Src, DST_UNSIGNED, SRC_UNSIGNED, OVERLAPS_RANGE> {
  static constexpr RangeCheckResult Check(Src value) {
    typedef std::numeric_limits<Dst> DstLimits;
    return BASE_NUMERIC_RANGE_CHECK_RESULT(
        value <= static_cast<Src>(DstLimits::max()), true);
  }
};

// Unsigned to signed.
template <typename Dst, typename Src>
struct RangeCheckImpl<Dst, Src, DST_SIGNED, SRC_UNSIGNED, OVERLAPS_RANGE> {
  static constexpr RangeCheckResult Check(Src value) {
    typedef std::numeric_limits<Dst> DstLimits;
    return sizeof(Dst) > sizeof(Src)
               ? TYPE_VALID
               : BASE_NUMERIC_RANGE_CHECK_RESULT(
                     value <= static_cast<Src>(DstLimits::max()), true);
  }
};

// Signed to unsigned.
template <typename Dst, typename Src>
struct RangeCheckImpl<Dst, Src, DST_UNSIGNED, SRC_SIGNED, OVERLAPS_RANGE> {
  typedef std::numeric_limits<Dst> DstLimits;
  typedef std::numeric_limits<Src> SrcLimits;
  // Compare based on max_exponent, which we must compute for integrals.
  static constexpr size_t DstMaxExponent() { return sizeof(Dst) * 8; }
  static constexpr size_t SrcMaxExponent() {
    return SrcLimits::is_iec559 ? SrcLimits::max_exponent
                                : (sizeof(Src) * 8 - 1);
  }
  static constexpr RangeCheckResult Check(Src value) {
    return (DstMaxExponent() >= SrcMaxExponent())
               ? BASE_NUMERIC_RANGE_CHECK_RESULT(true,
                                                 value >= static_cast<Src>(0))
               : BASE_NUMERIC_RANGE_CHECK_RESULT(
                     value <= static_cast<Src>(DstLimits::max()),
                     value >= static_cast<Src>(0));
  }
};

template <typename Dst, typename Src>
inline constexpr RangeCheckResult RangeCheck(Src value) {
  static_assert(std::numeric_limits<Src>::is_specialized,
                "argument must be numeric");
  static_assert(std::numeric_limits<Dst>::is_specialized,
                "result must be numeric");
  return RangeCheckImpl<Dst, Src>::Check(value);
}

}  // namespace internal
}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_SAFE_CONVERSIONS_IMPL_H_
