// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This class is intended to contain a collection of useful (static)
// mathematical functions, properly coded (by consulting numerical
// recipes or another authoritative source first).

#ifndef MEDIAPIPE_DEPS_MATHUTIL_H_
#define MEDIAPIPE_DEPS_MATHUTIL_H_

#include <cmath>
#include <cstdint>
#include <limits>
#include <type_traits>

#include "absl/log/absl_check.h"

namespace mediapipe {

// ========================================================================= //

class MathUtil {
 public:
  // --------------------------------------------------------------------
  // Round
  //   This function rounds a floating-point number to an integer. It
  //   works for positive or negative numbers.
  //
  //   Values that are halfway between two integers may be rounded up or
  //   down, for example Round<int>(0.5) == 0 and Round<int>(1.5) == 2.
  //   This allows the function to be implemented efficiently on multiple
  //   hardware platforms (see the template specializations at the bottom
  //   of this file). You should not use this function if you care about which
  //   way such half-integers are rounded.
  //
  //   Example usage:
  //     double y, z;
  //     int x = Round<int>(y + 3.7);
  //     int64_t b = Round<int64_t>(0.3 * z);
  //
  //   Note that the floating-point template parameter is typically inferred
  //   from the argument type, i.e. there is no need to specify it explicitly.
  // --------------------------------------------------------------------
  template <class IntOut, class FloatIn>
  static IntOut Round(FloatIn x) {
    static_assert(!std::numeric_limits<FloatIn>::is_integer,
                  "FloatIn is integer");
    static_assert(std::numeric_limits<IntOut>::is_integer,
                  "IntOut is not integer");

    // We don't use sgn(x) below because there is no need to distinguish the
    // (x == 0) case.  Also note that there are specialized faster versions
    // of this function for Intel, ARM and PPC processors at the bottom
    // of this file.
    if (x > -0.5 && x < 0.5) {
      // This case is special, because for largest floating point number
      // below 0.5, the addition of 0.5 yields 1 and this would lead
      // to incorrect result.
      return static_cast<IntOut>(0);
    }
    return static_cast<IntOut>(x < 0 ? (x - 0.5) : (x + 0.5));
  }

  // Convert a floating-point number to an integer. For all inputs x where
  // static_cast<IntOut>(x) is legal according to the C++ standard, the result
  // is identical to that cast (i.e. the result is x with its fractional part
  // truncated whenever that is representable as IntOut).
  //
  // static_cast would cause undefined behavior for the following cases, which
  // have well-defined behavior for this function:
  //
  //  1. If x is NaN, the result is zero.
  //
  //  2. If the truncated form of x is above the representable range of IntOut,
  //     the result is std::numeric_limits<IntOut>::max().
  //
  //  3. If the truncated form of x is below the representable range of IntOut,
  //     the result is std::numeric_limits<IntOut>::min().
  //
  // Note that cases #2 and #3 cover infinities as well as finite numbers.
  //
  // The range of FloatIn must include the range of IntOut, otherwise
  // the results are undefined.
  template <class IntOut, class FloatIn>
  static IntOut SafeCast(FloatIn x) {
    static_assert(!std::numeric_limits<FloatIn>::is_integer,
                  "FloatIn is integer");
    static_assert(std::numeric_limits<IntOut>::is_integer,
                  "IntOut is not integer");
    static_assert(std::numeric_limits<IntOut>::radix == 2, "IntOut is base 2");

    // Special case NaN, for which the logic below doesn't work.
    if (std::isnan(x)) {
      return 0;
    }

    // Negative values all clip to zero for unsigned results.
    if (!std::numeric_limits<IntOut>::is_signed && x < 0) {
      return 0;
    }

    // Handle infinities.
    if (std::isinf(x)) {
      return x < 0 ? std::numeric_limits<IntOut>::min()
                   : std::numeric_limits<IntOut>::max();
    }

    // Set exp such that x == f * 2^exp for some f with |f| in [0.5, 1.0),
    // unless x is zero in which case exp == 0. Note that this implies that the
    // magnitude of x is strictly less than 2^exp.
    int exp = 0;
    std::frexp(x, &exp);

    // Let N be the number of non-sign bits in the representation of IntOut. If
    // the magnitude of x is strictly less than 2^N, the truncated version of x
    // is representable as IntOut. The only representable integer for which this
    // is not the case is std::numeric_limits::min() for signed types (i.e.
    // -2^N), but that is covered by the fall-through below.
    if (exp <= std::numeric_limits<IntOut>::digits) {
      return x;
    }

    // Handle numbers with magnitude >= 2^N.
    return x < 0 ? std::numeric_limits<IntOut>::min()
                 : std::numeric_limits<IntOut>::max();
  }

  // --------------------------------------------------------------------
  // SafeRound
  //   These functions round a floating-point number to an integer.
  //   Results are identical to Round, except in cases where
  //   the argument is NaN, or when the rounded value would overflow the
  //   return type. In those cases, Round has undefined
  //   behavior. SafeRound returns 0 when the argument is
  //   NaN, and returns the closest possible integer value otherwise (i.e.
  //   std::numeric_limits<IntOut>::max() for large positive values, and
  //   std::numeric_limits<IntOut>::min() for large negative values).
  //   The range of FloatIn must include the range of IntOut, otherwise
  //   the results are undefined.
  // --------------------------------------------------------------------
  template <class IntOut, class FloatIn>
  static IntOut SafeRound(FloatIn x) {
    static_assert(!std::numeric_limits<FloatIn>::is_integer,
                  "FloatIn is integer");
    static_assert(std::numeric_limits<IntOut>::is_integer,
                  "IntOut is not integer");

    if (std::isnan(x)) {
      return 0;
    } else {
      return SafeCast<IntOut>((x < 0.) ? (x - 0.5) : (x + 0.5));
    }
  }

  // --------------------------------------------------------------------
  // FastIntRound, FastInt64Round
  //   Fast routines for converting floating-point numbers to integers.
  //
  //   These routines are approximately 6 times faster than the default
  //   implementation of Round<int> on Intel processors (12 times faster on
  //   the Pentium 3).  They are also more than 5 times faster than simply
  //   casting a "double" to an "int" using static_cast<int>.  This is
  //   because casts are defined to truncate towards zero, which on Intel
  //   processors requires changing the rounding mode and flushing the
  //   floating-point pipeline (unless programs are compiled specifically
  //   for the Pentium 4, which has a new instruction to avoid this).
  //
  //   Numbers that are halfway between two integers may be rounded up or
  //   down.  This is because the conversion is done using the default
  //   rounding mode, which rounds towards the closest even number in case
  //   of ties.  So for example, FastIntRound(0.5) == 0, but
  //   FastIntRound(1.5) == 2.  These functions should only be used with
  //   applications that don't care about which way such half-integers are
  //   rounded.
  //
  //   There are template specializations of Round() which call these
  //   functions (for "int" and "int64_t" only), but it's safer to call them
  //   directly.
  //   --------------------------------------------------------------------

  static int32_t FastIntRound(double x) {
#if defined __GNUC__ && (defined __i386__ || defined __SSE2__ || \
                         defined __aarch64__ || defined __powerpc64__)
#if defined __AVX__
    // AVX.
    int32_t result;
    __asm__ __volatile__(
        "vcvtsd2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __SSE2__
    // SSE2.
    int32_t result;
    __asm__ __volatile__(
        "cvtsd2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __i386__
    // FPU stack.  Adapted from /usr/include/bits/mathinline.h.
    int32_t result;
    __asm__ __volatile__("fistpl %0"
                         : "=m"(result)  // Output operand is a memory location
                         : "t"(x)        // Input operand is top of FP stack
                         : "st");        // Clobbers (pops) top of FP stack
    return result;
#elif defined __aarch64__
    int64_t result;
    __asm__ __volatile__("fcvtns %d0, %d1"
                         : "=w"(result)  // Vector floating point register
                         : "w"(x)        // Vector floating point register
                         : /* No clobbers */);
    return static_cast<int32_t>(result);
#elif defined __powerpc64__
    int64_t result;
    __asm__ __volatile__("fctid %0, %1"
                         : "=d"(result)
                         : "d"(x)
                         : /* No clobbers */);
    return result;
#endif  // defined __powerpc64__
#else
    return Round<int32_t>(x);
#endif  // if defined __GNUC__ && ...
  }

  static int32_t FastIntRound(float x) {
#if defined __GNUC__ && (defined __i386__ || defined __SSE2__ || \
                         defined __aarch64__ || defined __powerpc64__)
#if defined __AVX__
    // AVX.
    int32_t result;
    __asm__ __volatile__(
        "vcvtss2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __SSE2__
    // SSE2.
    int32_t result;
    __asm__ __volatile__(
        "cvtss2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __i386__
    // FPU stack.  Adapted from /usr/include/bits/mathinline.h.
    int32_t result;
    __asm__ __volatile__("fistpl %0"
                         : "=m"(result)  // Output operand is a memory location
                         : "t"(x)        // Input operand is top of FP stack
                         : "st");        // Clobbers (pops) top of FP stack
    return result;
#elif defined __aarch64__
    int64_t result;
    __asm__ __volatile__("fcvtns %s0, %s1"
                         : "=w"(result)  // Vector floating point register
                         : "w"(x)        // Vector floating point register
                         : /* No clobbers */);
    return static_cast<int32_t>(result);
#elif defined __powerpc64__
    uint64_t output;
    __asm__ __volatile__("fctiw %0, %1"
                         : "=d"(output)
                         : "f"(x)
                         : /* No clobbers */);
    return bit_cast<int32_t>(static_cast<uint32_t>(output >> 32));
#endif  // defined __powerpc64__
#else
    return Round<int32_t>(x);
#endif  // if defined __GNUC__ && ...
  }

  static int64_t FastInt64Round(double x) {
#if defined __GNUC__ && (defined __i386__ || defined __x86_64__ || \
                         defined __aarch64__ || defined __powerpc64__)
#if defined __AVX__
    // AVX.
    int64_t result;
    __asm__ __volatile__(
        "vcvtsd2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __x86_64__
    // SSE2.
    int64_t result;
    __asm__ __volatile__(
        "cvtsd2si %1, %0"
        : "=r"(result)  // Output operand is a register
        : "xm"(x));     // Input operand is an xmm register or memory
    return result;
#elif defined __i386__
    // There is no CVTSD2SI in i386 to produce a 64 bit int, even with SSE2.
    // FPU stack.  Adapted from /usr/include/bits/mathinline.h.
    int64_t result;
    __asm__ __volatile__("fistpll %0"
                         : "=m"(result)  // Output operand is a memory location
                         : "t"(x)        // Input operand is top of FP stack
                         : "st");        // Clobbers (pops) top of FP stack
    return result;
#elif defined __aarch64__
    // Floating-point convert to signed integer,
    // rounding to nearest with ties to even.
    int64_t result;
    __asm__ __volatile__("fcvtns %d0, %d1"
                         : "=w"(result)
                         : "w"(x)
                         : /* No clobbers */);
    return result;
#elif defined __powerpc64__
    int64_t result;
    __asm__ __volatile__("fctid %0, %1"
                         : "=d"(result)
                         : "d"(x)
                         : /* No clobbers */);
    return result;
#endif  // if defined __powerpc64__
#else
    return Round<int64_t>(x);
#endif  // if defined __GNUC__ && ...
  }

  static int64_t FastInt64Round(float x) {
    return FastInt64Round(static_cast<double>(x));
  }

  static int32_t FastIntRound(long double x) { return Round<int32_t>(x); }

  static int64_t FastInt64Round(long double x) { return Round<int64_t>(x); }

  // Absolute value of the difference between two numbers.
  // Works correctly for signed types and special floating point values.
  template <typename T>
  static typename std::make_unsigned<T>::type AbsDiff(const T x, const T y) {
    // Carries out arithmetic as unsigned to avoid overflow.
    typedef typename std::make_unsigned<T>::type R;
    return x > y ? R(x) - R(y) : R(y) - R(x);
  }

  // Clamps value to the range [low, high].  Requires low <= high.
  template <typename T>  // T models LessThanComparable.
  static const T& Clamp(const T& low, const T& high, const T& value) {
    // Prevents errors in ordering the arguments.
    ABSL_DCHECK(!(high < low));
    if (high < value) return high;
    if (value < low) return low;
    return value;
  }

  // If two (usually floating point) numbers are within a certain
  // absolute margin of error.
  template <typename T>
  static bool WithinMargin(const T x, const T y, const T margin) {
    ABSL_DCHECK_GE(margin, 0);
    return (std::abs(x) <= std::abs(y) + margin) &&
           (std::abs(x) >= std::abs(y) - margin);
  }
};

// ========================================================================= //

#if defined __GNUC__ && (defined __i386__ || defined __x86_64__ || \
                         defined __aarch64__ || defined __powerpc64__)

// We define template specializations of Round() to get the more efficient
// Intel versions when possible.  Note that gcc does not currently support
// partial specialization of templatized functions.

template <>
inline int32_t MathUtil::Round<int32_t, float>(float x) {
  return FastIntRound(x);
}

template <>
inline int32_t MathUtil::Round<int32_t, double>(double x) {
  return FastIntRound(x);
}

template <>
inline int64_t MathUtil::Round<int64_t, float>(float x) {
  return FastInt64Round(x);
}

template <>
inline int64_t MathUtil::Round<int64_t, double>(double x) {
  return FastInt64Round(x);
}

#endif

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_MATHUTIL_H_
