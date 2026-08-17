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

// A "safe int" is a StrongInt<T> which does additional validation of the
// various arithmetic and logical operations, and reacts to overflows and
// underflow and invalid operations.  You can define the "safe int" types
// to react to errors in pre-defined ways or you can define your own policy
// classes.
//
// Usage:
//   MEDIAPIPE_DEFINE_SAFE_INT_TYPE(Name, NativeType, PolicyType);
//
//     Defines a new StrongInt type named 'Name' in the current namespace with
//     underflow/overflow checking on all operations, with configurable error
//     policy.
//
//     Name: The desired name for the new StrongInt typedef.  Must be unique
//         within the current namespace.
//     NativeType: The primitive integral type this StrongInt will hold, as
//         defined by std::is_integral (see <type_traits>).
//     PolicyType: The type of policy used by this StrongInt type.  A few
//         pre-built policy types are provided here, but the caller can
//         define any custom policy they desire.
//
// PolicyTypes:
//     LogFatalOnError: ABSL_LOG(FATAL) when a error occurs.

#ifndef MEDIAPIPE_DEPS_SAFE_INT_H_
#define MEDIAPIPE_DEPS_SAFE_INT_H_

#include <limits.h>

#include <cstdint>
#include <limits>
#include <type_traits>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/deps/strong_int.h"

namespace mediapipe {
namespace intops {

// A StrongInt validator class for "safe" type enforcement.  For signed types,
// this checks for overflows and underflows as well as undefined- or
// implementation-defined behaviors. For unsigned type, this further disallows
// operations that would take advantage of unsigned wrap-around behavior and
// operations which would discard data unexpectedly.  This assumes two's
// complement representations, and that division truncates towards zero.
//
// For some more on overflow safety, see:
//   https://www.securecoding.cert.org/confluence/display/seccode/INT32-C.+Ensure+that+operations+on+signed+integers+do+not+result+in+overflow?showComments=false
template <typename ErrorType>
class SafeIntStrongIntValidator {
 private:
  template <typename T>
  static void SanityCheck() {
    // Check that the underlying integral type provides a range that is
    // compatible with two's complement.
    if (std::numeric_limits<T>::is_signed) {
      ABSL_CHECK_EQ(
          -1, std::numeric_limits<T>::min() + std::numeric_limits<T>::max())
          << "unexpected integral bounds";
    }

    // Check that division truncates towards 0 (implementation defined in
    // C++'03, but standard in C++'11).
    ABSL_CHECK_EQ(12, 127 / 10) << "division does not truncate towards 0";
    ABSL_CHECK_EQ(-12, -127 / 10) << "division does not truncate towards 0";
    ABSL_CHECK_EQ(-12, 127 / -10) << "division does not truncate towards 0";
    ABSL_CHECK_EQ(12, -127 / -10) << "division does not truncate towards 0";
  }

 public:
  template <typename T, typename U>
  static void ValidateInit(U arg) {
    // Do some sanity checks before proceeding.
    SanityCheck<T>();

    // If the argument is floating point, we can do a simple check to make
    // sure the value is in range.  It is undefined behavior to convert to int
    // from a float that is out of range. Since large integers will loose some
    // precision when being converted to floating point, the integer max and min
    // are explicitly converted back to floating point for this comparison, in
    // order to satisfy compiler warnings.
    if (std::is_floating_point<U>::value) {
      if (arg < static_cast<U>(std::numeric_limits<T>::min()) ||
          arg > static_cast<U>(std::numeric_limits<T>::max())) {
        ErrorType::Error("SafeInt: init from out of bounds float", arg, "=");
      }
    } else {
      // If the initial value (type U) is changed by being converted to and from
      // the native type (type T), then it must be out of bounds for type T.
      //
      // If T is unsigned and the argument is negative, then it is clearly out
      // of bounds for type T.
      //
      // If the initial value is greater than the max value for type T, then it
      // is clearly out of bounds for type T.  Before we check that, though, we
      // must ensure that the initial value is positive, or else we could get
      // unwanted promotion to unsigned, making the test wrong.  If the initial
      // value is negative, it can't be larger than the max value for type T.
      if ((static_cast<U>(static_cast<T>(arg)) != arg) ||
          (!std::numeric_limits<T>::is_signed && arg < 0) ||
          (arg > 0 && arg > std::numeric_limits<T>::max())) {
        ErrorType::Error("SafeInt: init from out of bounds value", arg, "=");
      }
    }
  }
  template <typename T>
  static void ValidateNegate(  // Signed types only.
      typename std::enable_if<std::numeric_limits<T>::is_signed, T>::type
          value) {
    if (value == std::numeric_limits<T>::min()) {
      ErrorType::Error("SafeInt: overflow", value, -1, "*");
    }
  }
  template <typename T>
  static void ValidateBitNot(  // Unsigned types only.
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type
          value) {
    // Do nothing.
  }
  template <typename T>
  static void ValidateAdd(T lhs, T rhs) {
    // The same logic applies to signed and unsigned types.
    if ((rhs > 0) && (lhs > (std::numeric_limits<T>::max() - rhs))) {
      ErrorType::Error("SafeInt: overflow", lhs, rhs, "+");
    } else if ((rhs < 0) && (lhs < (std::numeric_limits<T>::min() - rhs))) {
      ErrorType::Error("SafeInt: underflow", lhs, rhs, "+");
    }
  }
  template <typename T>
  static void ValidateSubtract(T lhs, T rhs) {
    // The same logic applies to signed and unsigned types.
    if ((rhs > 0) && (lhs < (std::numeric_limits<T>::min() + rhs))) {
      ErrorType::Error("SafeInt: underflow", lhs, rhs, "-");
    } else if ((rhs < 0) && (lhs > (std::numeric_limits<T>::max() + rhs))) {
      ErrorType::Error("SafeInt: overflow", lhs, rhs, "-");
    }
  }
  template <typename T, typename U>
  static void ValidateMultiply(T lhs, U rhs) {
    if (!std::numeric_limits<T>::is_signed) {
      // Unsigned types only.
      if (rhs < 0) {
        ErrorType::Error("SafeInt: negation of unsigned type", lhs, rhs, "*");
      }
    }
    // Multiplication by 0 can never overflow/underflow, but handling 0 makes
    // the below code more complex.
    if (lhs == 0 || rhs == 0) {
      return;
    }
    // The remaining logic applies to signed and unsigned types.  Note that
    // while multiplication is commutative, the underlying StrongInt class
    // always calls this with T as StrongInt<T>::ValueType.
    if (lhs > 0) {
      if (rhs > 0) {
        if (lhs > (std::numeric_limits<T>::max() / rhs)) {
          ErrorType::Error("SafeInt: overflow", lhs, rhs, "*");
        }
      } else {
        if (rhs < (std::numeric_limits<T>::min() / lhs)) {
          ErrorType::Error("SafeInt: underflow", lhs, rhs, "*");
        }
      }
    } else {
      if (rhs > 0) {
        // Underflow could be tested by lhs < min / rhs, but that does not
        // work if rhs is an unsigned type. Intead we test rhs > min / lhs.
        // There is a special case for lhs = -1, which would overflow min / lhs.
        if ((lhs == -1 && rhs - 1 > std::numeric_limits<T>::max()) ||
            (lhs < -1 && rhs > std::numeric_limits<T>::min() / lhs)) {
          ErrorType::Error("SafeInt: underflow", lhs, rhs, "*");
        }
      } else {
        if ((lhs != 0) && (rhs < (std::numeric_limits<T>::max() / lhs))) {
          ErrorType::Error("SafeInt: overflow", lhs, rhs, "*");
        }
      }
    }
  }
  template <typename T, typename U>
  static void ValidateDivide(T lhs, U rhs) {
    // This applies to signed and unsigned types.
    if (rhs == 0) {
      ErrorType::Error("SafeInt: divide by zero", lhs, rhs, "/");
    }
    if (std::numeric_limits<T>::is_signed) {
      // Signed types only.
      if ((lhs == std::numeric_limits<T>::min()) && (rhs == -1)) {
        ErrorType::Error("SafeInt: overflow", lhs, rhs, "/");
      }
    } else {
      // Unsigned types only.
      if (rhs < 0) {
        ErrorType::Error("SafeInt: negation of unsigned type", lhs, rhs, "/");
      }
    }
  }
  template <typename T, typename U>
  static void ValidateModulo(T lhs, U rhs) {
    // This applies to signed and unsigned types.
    if (rhs == 0) {
      ErrorType::Error("SafeInt: divide by zero", lhs, rhs, "%");
    }
    if (std::numeric_limits<T>::is_signed) {
      // Signed types only.
      if ((lhs == std::numeric_limits<T>::min()) && (rhs == -1)) {
        ErrorType::Error("SafeInt: overflow", lhs, rhs, "%");
      }
    } else {
      // Unsigned types only.
      if (rhs < 0) {
        ErrorType::Error("SafeInt: negation of unsigned type", lhs, rhs, "%");
      }
    }
  }
  template <typename T>
  static void ValidateLeftShift(T lhs, int64_t rhs) {
    if (std::numeric_limits<T>::is_signed) {
      // Signed types only.
      if (lhs < 0) {
        ErrorType::Error("SafeInt: shift of negative value", lhs, rhs, "<<");
      }
    }
    // The remaining logic applies to signed and unsigned types.
    if (rhs < 0) {
      ErrorType::Error("SafeInt: shift by negative arg", lhs, rhs, "<<");
    }
    if (rhs >= (sizeof(T) * CHAR_BIT)) {
      ErrorType::Error("SafeInt: shift by large arg", lhs, rhs, "<<");
    }
    if (lhs > (std::numeric_limits<T>::max() >> rhs)) {
      ErrorType::Error("SafeInt: overflow", lhs, rhs, "<<");
    }
  }
  template <typename T>
  static void ValidateRightShift(T lhs, int64_t rhs) {
    if (std::numeric_limits<T>::is_signed) {
      // Signed types only.
      if (lhs < 0) {
        ErrorType::Error("SafeInt: shift of negative value", lhs, rhs, ">>");
      }
    }
    // The remaining logic applies to signed and unsigned types.
    if (rhs < 0) {
      ErrorType::Error("SafeInt: shift by negative arg", lhs, rhs, ">>");
    }
    if (rhs >= (sizeof(T) * CHAR_BIT)) {
      ErrorType::Error("SafeInt: shift by large arg", lhs, rhs, ">>");
    }
  }
  template <typename T>
  static void ValidateBitAnd(  // Unsigned types only.
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type lhs,
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type
          rhs) {
    // Do nothing.
  }
  template <typename T>
  static void ValidateBitOr(  // Unsigned types only.
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type lhs,
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type
          rhs) {
    // Do nothing.
  }
  template <typename T>
  static void ValidateBitXor(  // Unsigned types only.
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type lhs,
      typename std::enable_if<!std::numeric_limits<T>::is_signed, T>::type
          rhs) {
    // Do nothing.
  }
};

// A SafeIntStrongIntValidator policy class to ABSL_LOG(FATAL) on errors.
struct LogFatalOnError {
  template <typename Tlhs, typename Trhs>
  static void Error(const char* error, Tlhs lhs, Trhs rhs, const char* op) {
    ABSL_LOG(FATAL) << error << ": (" << lhs << " " << op << " " << rhs << ")";
  }
  template <typename Tval>
  static void Error(const char* error, Tval val, const char* op) {
    ABSL_LOG(FATAL) << error << ": (" << op << val << ")";
  }
};

}  // namespace intops
}  // namespace mediapipe

// Defines the StrongInt using value_type and typedefs it to type_name, with
// strong checking of under/overflow conditions.
// The struct int_type_name ## _tag_ trickery is needed to ensure that a new
// type is created per type_name.
#define MEDIAPIPE_DEFINE_SAFE_INT_TYPE(type_name, value_type, policy_type) \
  struct type_name##_safe_tag_ {};                                         \
  typedef mediapipe::intops::StrongInt<                                    \
      type_name##_safe_tag_, value_type,                                   \
      mediapipe::intops::SafeIntStrongIntValidator<policy_type>>           \
      type_name;

#endif  // MEDIAPIPE_DEPS_SAFE_INT_H_
