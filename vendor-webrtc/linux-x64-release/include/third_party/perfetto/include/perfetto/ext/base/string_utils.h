/*
 * Copyright (C) 2018 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_STRING_UTILS_H_
#define INCLUDE_PERFETTO_EXT_BASE_STRING_UTILS_H_

#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include <charconv>
#include <cinttypes>
#include <optional>
#include <string>
#include <system_error>
#include <vector>

#include "perfetto/ext/base/string_view.h"

namespace perfetto {
namespace base {

inline char Lowercase(char c) {
  return ('A' <= c && c <= 'Z') ? static_cast<char>(c - ('A' - 'a')) : c;
}

inline char Uppercase(char c) {
  return ('a' <= c && c <= 'z') ? static_cast<char>(c + ('A' - 'a')) : c;
}

inline std::optional<uint32_t> CStringToUInt32(const char* s, int base = 10) {
  char* endptr = nullptr;
  auto value = static_cast<uint32_t>(strtoul(s, &endptr, base));
  return (*s && !*endptr) ? std::make_optional(value) : std::nullopt;
}

inline std::optional<int32_t> CStringToInt32(const char* s, int base = 10) {
  char* endptr = nullptr;
  auto value = static_cast<int32_t>(strtol(s, &endptr, base));
  return (*s && !*endptr) ? std::make_optional(value) : std::nullopt;
}

// Note: it saturates to 7fffffffffffffff if parsing a hex number >= 0x8000...
inline std::optional<int64_t> CStringToInt64(const char* s, int base = 10) {
  char* endptr = nullptr;
  auto value = static_cast<int64_t>(strtoll(s, &endptr, base));
  return (*s && !*endptr) ? std::make_optional(value) : std::nullopt;
}

inline std::optional<uint64_t> CStringToUInt64(const char* s, int base = 10) {
  char* endptr = nullptr;
  auto value = static_cast<uint64_t>(strtoull(s, &endptr, base));
  return (*s && !*endptr) ? std::make_optional(value) : std::nullopt;
}

double StrToD(const char* nptr, char** endptr);

inline std::optional<double> CStringToDouble(const char* s) {
  char* endptr = nullptr;
  double value = StrToD(s, &endptr);
  std::optional<double> result(std::nullopt);
  if (*s != '\0' && *endptr == '\0')
    result = value;
  return result;
}

inline std::optional<uint32_t> StringToUInt32(const std::string& s,
                                              int base = 10) {
  return CStringToUInt32(s.c_str(), base);
}

inline std::optional<int32_t> StringToInt32(const std::string& s,
                                            int base = 10) {
  return CStringToInt32(s.c_str(), base);
}

inline std::optional<uint64_t> StringToUInt64(const std::string& s,
                                              int base = 10) {
  return CStringToUInt64(s.c_str(), base);
}

inline std::optional<int64_t> StringToInt64(const std::string& s,
                                            int base = 10) {
  return CStringToInt64(s.c_str(), base);
}

inline std::optional<double> StringToDouble(const std::string& s) {
  return CStringToDouble(s.c_str());
}

template <typename T>
inline std::optional<T> StringViewToNumber(const base::StringView& sv,
                                           int base = 10) {
  // std::from_chars() does not regonize the leading '+' character and only
  // recognizes '-' so remove the '+' if it exists to avoid errors and match
  // the behavior of the other string conversion utilities above.
  size_t start_offset = !sv.empty() && sv.at(0) == '+' ? 1 : 0;
  T value;
  auto result =
      std::from_chars(sv.begin() + start_offset, sv.end(), value, base);
  if (result.ec == std::errc() && result.ptr == sv.end()) {
    return value;
  } else {
    return std::nullopt;
  }
}

inline std::optional<uint32_t> StringViewToUInt32(const base::StringView& sv,
                                                  int base = 10) {
  // std::from_chars() does not recognize the leading '-' character for
  // unsigned conversions, but strtol does. To Mimic the behavior of strtol,
  // attempt a signed converion if we see a leading '-', and then cast the
  // result back to unsigned.
  if (sv.size() > 0 && sv.at(0) == '-') {
    return static_cast<std::optional<uint32_t> >(
        StringViewToNumber<int32_t>(sv, base));
  } else {
    return StringViewToNumber<uint32_t>(sv, base);
  }
}

inline std::optional<int32_t> StringViewToInt32(const base::StringView& sv,
                                                int base = 10) {
  return StringViewToNumber<int32_t>(sv, base);
}

inline std::optional<uint64_t> StringViewToUInt64(const base::StringView& sv,
                                                  int base = 10) {
  // std::from_chars() does not recognize the leading '-' character for
  // unsigned conversions, but strtol does. To Mimic the behavior of strtol,
  // attempt a signed converion if we see a leading '-', and then cast the
  // result back to unsigned.
  if (sv.size() > 0 && sv.at(0) == '-') {
    return static_cast<std::optional<uint64_t> >(
        StringViewToNumber<int64_t>(sv, base));
  } else {
    return StringViewToNumber<uint64_t>(sv, base);
  }
}

inline std::optional<int64_t> StringViewToInt64(const base::StringView& sv,
                                                int base = 10) {
  return StringViewToNumber<int64_t>(sv, base);
}

// TODO: As of Clang 19.0 std::from_chars is unimplemented for type double
// despite being part of C++17 standard, and already being supported by GCC and
// MSVC. Enable this once we have double support in Clang.
// inline std::optional<double> StringViewToDouble(const base::StringView& sv) {
//   return StringViewToNumber<double>(sv);
// }

bool StartsWith(const std::string& str, const std::string& prefix);
bool EndsWith(const std::string& str, const std::string& suffix);
bool StartsWithAny(const std::string& str,
                   const std::vector<std::string>& prefixes);
bool Contains(const std::string& haystack, const std::string& needle);
bool Contains(const std::string& haystack, char needle);
size_t Find(const StringView& needle, const StringView& haystack);
bool CaseInsensitiveEqual(const std::string& first, const std::string& second);
std::string Join(const std::vector<std::string>& parts,
                 const std::string& delim);
std::vector<std::string> SplitString(const std::string& text,
                                     const std::string& delimiter);
std::string StripPrefix(const std::string& str, const std::string& prefix);
std::string StripSuffix(const std::string& str, const std::string& suffix);
std::string TrimWhitespace(const std::string& str);
std::string ToLower(const std::string& str);
std::string ToUpper(const std::string& str);
std::string StripChars(const std::string& str,
                       const std::string& chars,
                       char replacement);
std::string ToHex(const char* data, size_t size);
inline std::string ToHex(const std::string& s) {
  return ToHex(s.c_str(), s.size());
}
std::string IntToHexString(uint32_t number);
std::string Uint64ToHexString(uint64_t number);
std::string Uint64ToHexStringNoPrefix(uint64_t number);
std::string ReplaceAll(std::string str,
                       const std::string& to_replace,
                       const std::string& replacement);

// Checks if all characters in the input string view `str` are ASCII.
//
// If so, the function returns true and `output` is not modified.
// If `str` contains non-ASCII characters, the function returns false,
// removes invalid UTF-8 characters from `str`, and stores the result in
// `output`.
bool CheckAsciiAndRemoveInvalidUTF8(base::StringView str, std::string& output);

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
bool WideToUTF8(const std::wstring& source, std::string& output);
bool UTF8ToWide(const std::string& source, std::wstring& output);
#endif  // PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)

// A BSD-style strlcpy without the return value.
// Copies at most |dst_size|-1 characters. Unlike strncpy, it always \0
// terminates |dst|, as long as |dst_size| is not 0.
// Unlike strncpy and like strlcpy it does not zero-pad the rest of |dst|.
// Returns nothing. The BSD strlcpy returns the size of |src|, which might
// be > |dst_size|. Anecdotal experience suggests people assume the return value
// is the number of bytes written in |dst|. That assumption can lead to
// dangerous bugs.
// In order to avoid being subtly uncompliant with strlcpy AND avoid misuse,
// the choice here is to return nothing.
inline void StringCopy(char* dst, const char* src, size_t dst_size) {
  for (size_t i = 0; i < dst_size; ++i) {
    if ((dst[i] = src[i]) == '\0') {
      return;  // We hit and copied the null terminator.
    }
  }

  // We were left off at dst_size. We over copied 1 byte. Null terminate.
  if (PERFETTO_LIKELY(dst_size > 0))
    dst[dst_size - 1] = 0;
}

// Like snprintf() but returns the number of chars *actually* written (without
// counting the null terminator) NOT "the number of chars which would have been
// written to the final string if enough  space had been available".
// This should be used in almost all cases when the caller uses the return value
// of snprintf(). If the return value is not used, there is no benefit in using
// this wrapper, as this just calls snprintf() and mangles the return value.
// It always null-terminates |dst| (even in case of errors), unless
// |dst_size| == 0.
// Examples:
//   SprintfTrunc(x, 4, "123whatever"): returns 3 and writes "123\0".
//   SprintfTrunc(x, 4, "123"): returns 3 and writes "123\0".
//   SprintfTrunc(x, 3, "123"): returns 2 and writes "12\0".
//   SprintfTrunc(x, 2, "123"): returns 1 and writes "1\0".
//   SprintfTrunc(x, 1, "123"): returns 0 and writes "\0".
//   SprintfTrunc(x, 0, "123"): returns 0 and writes nothing.
// NOTE: This means that the caller has no way to tell when truncation happens
//   vs the edge case of *just* fitting in the buffer.
size_t SprintfTrunc(char* dst, size_t dst_size, const char* fmt, ...)
    PERFETTO_PRINTF_FORMAT(3, 4);

// Line number starts from 1
struct LineWithOffset {
  base::StringView line;
  uint32_t line_offset;
  uint32_t line_num;
};

// For given string and offset Pfinds a line with character for
// which offset points, what number is this line (starts from 1), and the offset
// inside this line. returns std::nullopt if the offset points to
// line break character or exceeds string length.
std::optional<LineWithOffset> FindLineWithOffset(base::StringView str,
                                                 uint32_t offset);

// A helper class to facilitate construction and usage of write-once stack
// strings.
// Example usage:
//   StackString<32> x("format %d %s", 42, string_arg);
//   TakeString(x.c_str() | x.string_view() | x.ToStdString());
// Rather than char x[32] + sprintf.
// Advantages:
// - Avoids useless zero-fills caused by people doing `char buf[32] {}` (mainly
//   by fearing unknown snprintf failure modes).
// - Makes the code more robust in case of snprintf truncations (len() and
//  string_view() will return the truncated length, unlike snprintf).
template <size_t N>
class StackString {
 public:
  explicit PERFETTO_PRINTF_FORMAT(/* 1=this */ 2, 3)
      StackString(const char* fmt, ...) {
    buf_[0] = '\0';
    va_list args;
    va_start(args, fmt);
    int res = vsnprintf(buf_, sizeof(buf_), fmt, args);
    va_end(args);
    buf_[sizeof(buf_) - 1] = '\0';
    len_ = res < 0 ? 0 : std::min(static_cast<size_t>(res), sizeof(buf_) - 1);
  }

  StringView string_view() const { return StringView(buf_, len_); }
  std::string ToStdString() const { return std::string(buf_, len_); }
  const char* c_str() const { return buf_; }
  size_t len() const { return len_; }
  char* mutable_data() { return buf_; }

 private:
  char buf_[N];
  size_t len_ = 0;  // Does not include the \0.
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_STRING_UTILS_H_
