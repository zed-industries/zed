/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_STRING_ENCODE_H_
#define RTC_BASE_STRING_ENCODE_H_

#include <stddef.h>

#include <optional>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/base/macros.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/string_to_number.h"
#include "rtc_base/strings/string_format.h"

namespace webrtc {

inline std::string BoolToString(bool b) {
  return b ? "true" : "false";
}

std::string hex_encode(absl::string_view str);
std::string hex_encode_with_delimiter(absl::string_view source, char delimiter);

// hex_decode converts ascii hex to binary.
size_t hex_decode(ArrayView<char> buffer, absl::string_view source);

// hex_decode, assuming that there is a delimiter between every byte
// pair.
// `delimiter` == 0 means no delimiter
// If the buffer is too short or the data is invalid, we return 0.
size_t hex_decode_with_delimiter(ArrayView<char> buffer,
                                 absl::string_view source,
                                 char delimiter);

// Splits the source string into multiple fields separated by delimiter,
// with duplicates of delimiter creating empty fields. Empty input produces a
// single, empty, field.
std::vector<absl::string_view> split(absl::string_view source, char delimiter);

// Splits the source string into multiple fields separated by delimiter,
// with duplicates of delimiter ignored.  Trailing delimiter ignored.
size_t tokenize(absl::string_view source,
                char delimiter,
                std::vector<std::string>* fields);

// Extract the first token from source as separated by delimiter, with
// duplicates of delimiter ignored. Return false if the delimiter could not be
// found, otherwise return true.
bool tokenize_first(absl::string_view source,
                    char delimiter,
                    std::string* token,
                    std::string* rest);

// Versions that behave differently from StrCat

// Versions not supported by StrCat:

template <typename T,
          typename std::enable_if<std::is_arithmetic<T>::value &&
                                      !std::is_same<T, bool>::value,
                                  int>::type = 0>
static bool FromString(absl::string_view s, T* t) {
  RTC_DCHECK(t);
  std::optional<T> result = webrtc::StringToNumber<T>(s);

  if (result)
    *t = *result;

  return result.has_value();
}

bool FromString(absl::string_view s, bool* b);

template <typename T>
static inline T FromString(absl::string_view str) {
  T val;
  FromString(str, &val);
  return val;
}

//////////////////////////////////////////////////////////////////////

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FromString;
using ::webrtc::hex_decode;
using ::webrtc::hex_decode_with_delimiter;
using ::webrtc::hex_encode;
using ::webrtc::hex_encode_with_delimiter;
using ::webrtc::split;
using ::webrtc::tokenize;
using ::webrtc::tokenize_first;

namespace internal {
template <typename T, typename = void>
struct is_absl_strcat_callable : std::false_type {};

template <typename T>
struct is_absl_strcat_callable<
    T,
    std::void_t<decltype(absl::StrCat(std::declval<T>()))>> : std::true_type {};
}  // namespace internal

template <typename T>
ABSL_DEPRECATE_AND_INLINE()
inline auto ToString(T value) ->
    typename std::enable_if<!std::is_same_v<T, bool> &&
                                internal::is_absl_strcat_callable<T>::value,
                            std::string>::type {
  return absl::StrCat(value);
}

template <typename T>
ABSL_DEPRECATE_AND_INLINE()
inline auto ToString(T p) ->
    typename std::enable_if<!internal::is_absl_strcat_callable<T>::value &&
                                std::is_pointer<T>::value,
                            std::string>::type {
  return webrtc::StringFormat("%p", p);
}

template <typename T>
ABSL_DEPRECATE_AND_INLINE()
inline auto ToString(T value) ->
    typename std::enable_if<!std::is_pointer_v<T> && std::is_same_v<T, bool>,
                            std::string>::type {
  return webrtc::BoolToString(value);
}

}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_STRING_ENCODE_H__
