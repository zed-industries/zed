/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_STRING_TO_NUMBER_H_
#define RTC_BASE_STRING_TO_NUMBER_H_

#include <limits>
#include <optional>
#include <string>
#include <type_traits>

#include "absl/strings/string_view.h"

namespace webrtc {

// This file declares a family of functions to parse integers from strings.
// The standard C library functions either fail to indicate errors (atoi, etc.)
// or are a hassle to work with (strtol, sscanf, etc.). The standard C++ library
// functions (std::stoi, etc.) indicate errors by throwing exceptions, which
// are disabled in WebRTC.
//
// Integers are parsed using:
//   std::optional<int-type> StringToNumber(absl::string_view str,
//                                           int base = 10);
//
// These functions parse a value from the beginning of a string into one of the
// fundamental integer types, or returns an empty Optional if parsing
// failed. Values outside of the range supported by the type will be
// rejected. The strings must begin with a digit or a minus sign. No leading
// space nor trailing contents are allowed.
// By setting base to 0, one of octal, decimal or hexadecimal will be
// detected from the string's prefix (0, nothing or 0x, respectively).
// If non-zero, base can be set to a value between 2 and 36 inclusively.

namespace string_to_number_internal {
// These must be (unsigned) long long, to match the signature of strto(u)ll.
using unsigned_type = unsigned long long;  // NOLINT(runtime/int)
using signed_type = long long;             // NOLINT(runtime/int)

std::optional<signed_type> ParseSigned(absl::string_view str, int base);
std::optional<unsigned_type> ParseUnsigned(absl::string_view str, int base);

template <typename T>
std::optional<T> ParseFloatingPoint(absl::string_view str);
}  // namespace string_to_number_internal

template <typename T>
typename std::enable_if<std::is_integral<T>::value && std::is_signed<T>::value,
                        std::optional<T>>::type
StringToNumber(absl::string_view str, int base = 10) {
  using string_to_number_internal::signed_type;
  static_assert(
      std::numeric_limits<T>::max() <=
              std::numeric_limits<signed_type>::max() &&
          std::numeric_limits<T>::lowest() >=
              std::numeric_limits<signed_type>::lowest(),
      "StringToNumber only supports signed integers as large as long long int");
  std::optional<signed_type> value =
      string_to_number_internal::ParseSigned(str, base);
  if (value && *value >= std::numeric_limits<T>::lowest() &&
      *value <= std::numeric_limits<T>::max()) {
    return static_cast<T>(*value);
  }
  return std::nullopt;
}

template <typename T>
typename std::enable_if<std::is_integral<T>::value &&
                            std::is_unsigned<T>::value,
                        std::optional<T>>::type
StringToNumber(absl::string_view str, int base = 10) {
  using string_to_number_internal::unsigned_type;
  static_assert(std::numeric_limits<T>::max() <=
                    std::numeric_limits<unsigned_type>::max(),
                "StringToNumber only supports unsigned integers as large as "
                "unsigned long long int");
  std::optional<unsigned_type> value =
      string_to_number_internal::ParseUnsigned(str, base);
  if (value && *value <= std::numeric_limits<T>::max()) {
    return static_cast<T>(*value);
  }
  return std::nullopt;
}

template <typename T>
typename std::enable_if<std::is_floating_point<T>::value,
                        std::optional<T>>::type
StringToNumber(absl::string_view str, int /* base */ = 10) {
  static_assert(
      std::numeric_limits<T>::max() <= std::numeric_limits<long double>::max(),
      "StringToNumber only supports floating-point numbers as large "
      "as long double");
  return string_to_number_internal::ParseFloatingPoint<T>(str);
}

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::StringToNumber;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_STRING_TO_NUMBER_H_
