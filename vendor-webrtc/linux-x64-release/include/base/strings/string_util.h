// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This file defines utility functions for working with strings.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/390223051): Remove C-library calls to fix the errors.
#pragma allow_unsafe_libc_calls
#endif

#ifndef BASE_STRINGS_STRING_UTIL_H_
#define BASE_STRINGS_STRING_UTIL_H_

#include <stdarg.h>  // va_list
#include <stddef.h>
#include <stdint.h>

#include <concepts>
#include <initializer_list>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
// For implicit conversions.
#include "base/strings/string_util_internal.h"
#include "base/types/to_address.h"
#include "build/build_config.h"

namespace base {

// C standard-library functions that aren't cross-platform are provided as
// "base::...", and their prototypes are listed below. These functions are
// then implemented as inline calls to the platform-specific equivalents in the
// platform-specific headers.

// Wrapper for vsnprintf that always null-terminates and always returns the
// number of characters that would be in an untruncated formatted
// string, even when truncation occurs.
// TODO(tsepez): should be UNSAFE_BUFFER_USAGE.
PRINTF_FORMAT(3, 0)
int vsnprintf(char* buffer, size_t size, const char* format, va_list arguments);

// Some of these implementations need to be inlined.

// We separate the declaration from the implementation of this inline
// function just so the PRINTF_FORMAT works.
// TODO(tsepez): should be UNSAFE_BUFFER_USAGE.
PRINTF_FORMAT(3, 4)
inline int snprintf(char* buffer, size_t size, const char* format, ...);
inline int snprintf(char* buffer, size_t size, const char* format, ...) {
  va_list arguments;
  va_start(arguments, format);
  int result = vsnprintf(buffer, size, format, arguments);
  va_end(arguments);
  return result;
}

// BSD-style safe and consistent string copy functions.

// Copies `src` to `dst`, truncating `dst` if it does not fit, and ensuring that
// `dst` is NUL-terminated if it's not an empty span. Returns the length of
// `src` in characters. If the return value is `>= dst.size()`, then the output
// was truncated. NOTE: All sizes are in number of characters, NOT in bytes.
BASE_EXPORT size_t strlcpy(span<char> dst, std::string_view src);
BASE_EXPORT size_t u16cstrlcpy(span<char16_t> dst, std::u16string_view src);
BASE_EXPORT size_t wcslcpy(span<wchar_t> dst, std::wstring_view src);

// Copies |src| to |dst|, where |dst_size| is the total allocated size of |dst|.
// Copies at most |dst_size|-1 characters, and always NULL terminates |dst|, as
// long as |dst_size| is not 0.  Returns the length of |src| in characters.
// If the return value is >= dst_size, then the output was truncated.
// NOTE: All sizes are in number of characters, NOT in bytes.
//
// TODO: crbug.com/40284755 - Make these UNSAFE_BUFFER_USAGE.
BASE_EXPORT size_t strlcpy(char* dst, const char* src, size_t dst_size);
BASE_EXPORT size_t u16cstrlcpy(char16_t* dst,
                               const char16_t* src,
                               size_t dst_size);
BASE_EXPORT size_t wcslcpy(wchar_t* dst, const wchar_t* src, size_t dst_size);

// Scan a wprintf format string to determine whether it's portable across a
// variety of systems.  This function only checks that the conversion
// specifiers used by the format string are supported and have the same meaning
// on a variety of systems.  It doesn't check for other errors that might occur
// within a format string.
//
// Nonportable conversion specifiers for wprintf are:
//  - 's' and 'c' without an 'l' length modifier.  %s and %c operate on char
//     data on all systems except Windows, which treat them as wchar_t data.
//     Use %ls and %lc for wchar_t data instead.
//  - 'S' and 'C', which operate on wchar_t data on all systems except Windows,
//     which treat them as char data.  Use %ls and %lc for wchar_t data
//     instead.
//  - 'F', which is not identified by Windows wprintf documentation.
//  - 'D', 'O', and 'U', which are deprecated and not available on all systems.
//     Use %ld, %lo, and %lu instead.
//
// Note that there is no portable conversion specifier for char data when
// working with wprintf.
//
// This function is intended to be called from base::vswprintf.
BASE_EXPORT bool IsWprintfFormatPortable(const wchar_t* format);

// Simplified implementation of C++20's std::basic_string_view(It, End).
// Reference: https://wg21.link/string.view.cons
template <typename CharT, typename Iter>
constexpr std::basic_string_view<CharT> MakeBasicStringPiece(Iter begin,
                                                             Iter end) {
  DCHECK_GE(end - begin, 0);
  return {base::to_address(begin), static_cast<size_t>(end - begin)};
}

// Explicit instantiations of MakeBasicStringPiece.
template <typename Iter>
constexpr std::string_view MakeStringPiece(Iter begin, Iter end) {
  return MakeBasicStringPiece<char>(begin, end);
}

template <typename Iter>
constexpr std::u16string_view MakeStringPiece16(Iter begin, Iter end) {
  return MakeBasicStringPiece<char16_t>(begin, end);
}

template <typename Iter>
constexpr std::wstring_view MakeWStringView(Iter begin, Iter end) {
  return MakeBasicStringPiece<wchar_t>(begin, end);
}

// ASCII-specific tolower.  The standard library's tolower is locale sensitive,
// so we don't want to use it here.
template <typename CharT>
  requires(std::integral<CharT>)
constexpr CharT ToLowerASCII(CharT c) {
  return internal::ToLowerASCII(c);
}

// ASCII-specific toupper.  The standard library's toupper is locale sensitive,
// so we don't want to use it here.
template <typename CharT>
  requires(std::integral<CharT>)
CharT ToUpperASCII(CharT c) {
  return (c >= 'a' && c <= 'z') ? static_cast<CharT>(c + 'A' - 'a') : c;
}

// Converts the given string to its ASCII-lowercase equivalent. Non-ASCII
// bytes (or UTF-16 code units in `std::u16string_view`) are permitted but will
// be unmodified.
BASE_EXPORT std::string ToLowerASCII(std::string_view str);
BASE_EXPORT std::u16string ToLowerASCII(std::u16string_view str);

// Converts the given string to its ASCII-uppercase equivalent. Non-ASCII
// bytes (or UTF-16 code units in `std::u16string_view`) are permitted but will
// be unmodified.
BASE_EXPORT std::string ToUpperASCII(std::string_view str);
BASE_EXPORT std::u16string ToUpperASCII(std::u16string_view str);

// Functor for ASCII case-insensitive comparisons for STL algorithms like
// std::search. Non-ASCII bytes (or UTF-16 code units in `std::u16string_view`)
// are permitted but will be compared as-is.
//
// Note that a full Unicode version of this functor is not possible to write
// because case mappings might change the number of characters, depend on
// context (combining accents), and require handling UTF-16. If you need
// proper Unicode support, use base::i18n::ToLower/FoldCase and then just
// use a normal operator== on the result.
template <typename Char>
  requires(std::integral<Char>)
struct CaseInsensitiveCompareASCII {
 public:
  bool operator()(Char x, Char y) const {
    return ToLowerASCII(x) == ToLowerASCII(y);
  }
};

// Like strcasecmp for ASCII case-insensitive comparisons only. Returns:
//   -1  (a < b)
//    0  (a == b)
//    1  (a > b)
// (unlike strcasecmp which can return values greater or less than 1/-1). To
// compare all Unicode code points case-insensitively, use base::i18n::ToLower
// or base::i18n::FoldCase and then just call the normal string operators on the
// result.
//
// Non-ASCII bytes (or UTF-16 code units in `std::u16string_view`) are permitted
// but will be compared unmodified.
BASE_EXPORT constexpr int CompareCaseInsensitiveASCII(std::string_view a,
                                                      std::string_view b) {
  return internal::CompareCaseInsensitiveASCIIT(a, b);
}
BASE_EXPORT constexpr int CompareCaseInsensitiveASCII(std::u16string_view a,
                                                      std::u16string_view b) {
  return internal::CompareCaseInsensitiveASCIIT(a, b);
}

// Equality for ASCII case-insensitive comparisons. Non-ASCII bytes (or UTF-16
// code units in `std::u16string_view`) are permitted but will be compared
// unmodified. To compare all Unicode code points case-insensitively, use
// base::i18n::ToLower or base::i18n::FoldCase and then compare with either ==
// or !=.
inline bool EqualsCaseInsensitiveASCII(std::string_view a, std::string_view b) {
  return internal::EqualsCaseInsensitiveASCIIT(a, b);
}
inline bool EqualsCaseInsensitiveASCII(std::u16string_view a,
                                       std::u16string_view b) {
  return internal::EqualsCaseInsensitiveASCIIT(a, b);
}
inline bool EqualsCaseInsensitiveASCII(std::u16string_view a,
                                       std::string_view b) {
  return internal::EqualsCaseInsensitiveASCIIT(a, b);
}
inline bool EqualsCaseInsensitiveASCII(std::string_view a,
                                       std::u16string_view b) {
  return internal::EqualsCaseInsensitiveASCIIT(a, b);
}

// These threadsafe functions return references to globally unique empty
// strings.
//
// It is likely faster to construct a new empty string object (just a few
// instructions to set the length to 0) than to get the empty string instance
// returned by these functions (which requires threadsafe static access).
//
// Therefore, DO NOT USE THESE AS A GENERAL-PURPOSE SUBSTITUTE FOR DEFAULT
// CONSTRUCTORS. There is only one case where you should use these: functions
// which need to return a string by reference (e.g. as a class member
// accessor), and don't have an empty string to use (e.g. in an error case).
// These should not be used as initializers, function arguments, or return
// values for functions which return by value or outparam.
BASE_EXPORT const std::string& EmptyString();
BASE_EXPORT const std::u16string& EmptyString16();

// Contains the set of characters representing whitespace in the corresponding
// encoding. Null-terminated. The ASCII versions are the whitespaces as defined
// by HTML5, and don't include control characters.
BASE_EXPORT extern const wchar_t kWhitespaceWide[];    // Includes Unicode.
BASE_EXPORT extern const char16_t kWhitespaceUTF16[];  // Includes Unicode.
BASE_EXPORT extern const char16_t
    kWhitespaceNoCrLfUTF16[];  // Unicode w/o CR/LF.
BASE_EXPORT extern const char kWhitespaceASCII[];
BASE_EXPORT extern const char16_t kWhitespaceASCIIAs16[];  // No unicode.

// https://infra.spec.whatwg.org/#ascii-whitespace
// Note that this array is not null-terminated.
inline constexpr char kInfraAsciiWhitespace[] = {0x09, 0x0A, 0x0C, 0x0D, 0x20};

// Null-terminated string representing the UTF-8 byte order mark.
BASE_EXPORT extern const char kUtf8ByteOrderMark[];

// Removes characters in |remove_chars| from anywhere in |input|.  Returns true
// if any characters were removed.  |remove_chars| must be null-terminated.
// NOTE: Safe to use the same variable for both |input| and |output|.
BASE_EXPORT bool RemoveChars(std::u16string_view input,
                             std::u16string_view remove_chars,
                             std::u16string* output);
BASE_EXPORT bool RemoveChars(std::string_view input,
                             std::string_view remove_chars,
                             std::string* output);

// Replaces characters in |replace_chars| from anywhere in |input| with
// |replace_with|.  Each character in |replace_chars| will be replaced with
// the |replace_with| string.  Returns true if any characters were replaced.
// |replace_chars| must be null-terminated.
// NOTE: Safe to use the same variable for both |input| and |output|.
BASE_EXPORT bool ReplaceChars(std::u16string_view input,
                              std::u16string_view replace_chars,
                              std::u16string_view replace_with,
                              std::u16string* output);
BASE_EXPORT bool ReplaceChars(std::string_view input,
                              std::string_view replace_chars,
                              std::string_view replace_with,
                              std::string* output);

enum TrimPositions {
  TRIM_NONE = 0,
  TRIM_LEADING = 1 << 0,
  TRIM_TRAILING = 1 << 1,
  TRIM_ALL = TRIM_LEADING | TRIM_TRAILING,
};

// Removes characters in |trim_chars| from the beginning and end of |input|.
// The 8-bit version only works on 8-bit characters, not UTF-8. Returns true if
// any characters were removed.
//
// It is safe to use the same variable for both |input| and |output| (this is
// the normal usage to trim in-place).
BASE_EXPORT bool TrimString(std::u16string_view input,
                            std::u16string_view trim_chars,
                            std::u16string* output);
BASE_EXPORT bool TrimString(std::string_view input,
                            std::string_view trim_chars,
                            std::string* output);

// std::string_view versions of the above. The returned pieces refer to the
// original buffer.
BASE_EXPORT std::u16string_view TrimString(std::u16string_view input,
                                           std::u16string_view trim_chars,
                                           TrimPositions positions);
BASE_EXPORT std::string_view TrimString(std::string_view input,
                                        std::string_view trim_chars,
                                        TrimPositions positions);

// Truncates a string to the nearest UTF-8 character that will leave
// the string less than or equal to the specified byte size.
BASE_EXPORT void TruncateUTF8ToByteSize(std::string_view input,
                                        const size_t byte_size,
                                        std::string* output);
BASE_EXPORT std::string_view TruncateUTF8ToByteSize(std::string_view input,
                                                    size_t byte_size);

// Trims any whitespace from either end of the input string.
//
// The std::string_view versions return a substring referencing the input
// buffer. The ASCII versions look only for ASCII whitespace.
//
// The std::string versions return where whitespace was found.
// NOTE: Safe to use the same variable for both input and output.
BASE_EXPORT TrimPositions TrimWhitespace(std::u16string_view input,
                                         TrimPositions positions,
                                         std::u16string* output);
BASE_EXPORT std::u16string_view TrimWhitespace(std::u16string_view input,
                                               TrimPositions positions);
BASE_EXPORT TrimPositions TrimWhitespaceASCII(std::string_view input,
                                              TrimPositions positions,
                                              std::string* output);
BASE_EXPORT std::string_view TrimWhitespaceASCII(std::string_view input,
                                                 TrimPositions positions);

// Searches for CR or LF characters.  Removes all contiguous whitespace
// strings that contain them.  This is useful when trying to deal with text
// copied from terminals.
// Returns |text|, with the following three transformations:
// (1) Leading and trailing whitespace is trimmed.
// (2) If |trim_sequences_with_line_breaks| is true, any other whitespace
//     sequences containing a CR or LF are trimmed.
// (3) All other whitespace sequences are converted to single spaces.
BASE_EXPORT std::u16string CollapseWhitespace(
    std::u16string_view text,
    bool trim_sequences_with_line_breaks);
BASE_EXPORT std::string CollapseWhitespaceASCII(
    std::string_view text,
    bool trim_sequences_with_line_breaks);

// Returns true if |input| is empty or contains only characters found in
// |characters|.
BASE_EXPORT bool ContainsOnlyChars(std::string_view input,
                                   std::string_view characters);
BASE_EXPORT bool ContainsOnlyChars(std::u16string_view input,
                                   std::u16string_view characters);

// Returns true if |str| is structurally valid UTF-8 and also doesn't
// contain any non-character code point (e.g. U+10FFFE). Prohibiting
// non-characters increases the likelihood of detecting non-UTF-8 in
// real-world text, for callers which do not need to accept
// non-characters in strings.
BASE_EXPORT bool IsStringUTF8(std::string_view str);

// Returns true if |str| contains valid UTF-8, allowing non-character
// code points.
BASE_EXPORT bool IsStringUTF8AllowingNoncharacters(std::string_view str);

// Returns true if |str| contains only valid ASCII character values.
// Note 1: IsStringASCII executes in time determined solely by the
// length of the string, not by its contents, so it is robust against
// timing attacks for all strings of equal length.
// Note 2: IsStringASCII assumes the input is likely all ASCII, and
// does not leave early if it is not the case.
BASE_EXPORT bool IsStringASCII(std::string_view str);
BASE_EXPORT bool IsStringASCII(std::u16string_view str);

#if defined(WCHAR_T_IS_32_BIT)
BASE_EXPORT bool IsStringASCII(std::wstring_view str);
#endif

// Performs a case-sensitive string compare of the given 16-bit string against
// the given 8-bit ASCII string (typically a constant). The behavior is
// undefined if the |ascii| string is not ASCII.
BASE_EXPORT bool EqualsASCII(std::u16string_view str, std::string_view ascii);

// Indicates case sensitivity of comparisons. Only ASCII case insensitivity
// is supported. Full Unicode case-insensitive conversions would need to go in
// base/i18n so it can use ICU.
//
// If you need to do Unicode-aware case-insensitive StartsWith/EndsWith, it's
// best to call base::i18n::ToLower() or base::i18n::FoldCase() (see
// base/i18n/case_conversion.h for usage advice) on the arguments, and then use
// the results to a case-sensitive comparison.
enum class CompareCase {
  SENSITIVE,
  INSENSITIVE_ASCII,
};

BASE_EXPORT bool StartsWith(
    std::string_view str,
    std::string_view search_for,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);
BASE_EXPORT bool StartsWith(
    std::u16string_view str,
    std::u16string_view search_for,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);
BASE_EXPORT bool EndsWith(
    std::string_view str,
    std::string_view search_for,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);
BASE_EXPORT bool EndsWith(
    std::u16string_view str,
    std::u16string_view search_for,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);

// If `string` begins with `prefix`, return a view into the portion
// of `string` following `prefix`. Otherwise, return nullopt. The
// `case_sensitivity` argument is the same as would be passed to
// StartsWith() above.
BASE_EXPORT std::optional<std::string_view> RemovePrefix(
    std::string_view string,
    std::string_view prefix,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);
BASE_EXPORT std::optional<std::u16string_view> RemovePrefix(
    std::u16string_view string,
    std::u16string_view prefix,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);

// If `string` ends with `suffix`, return a view into the portion
// of `string` preceding `suffix`. Otherwise, return nullopt. The
// `case_sensitivity` argument is the same as would be passed to
// EndsWith() above.
BASE_EXPORT std::optional<std::string_view> RemoveSuffix(
    std::string_view string,
    std::string_view suffix,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);
BASE_EXPORT std::optional<std::u16string_view> RemoveSuffix(
    std::u16string_view string,
    std::u16string_view suffix,
    CompareCase case_sensitivity = CompareCase::SENSITIVE);

// Determines the type of ASCII character, independent of locale (the C
// library versions will change based on locale).
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiWhitespace(Char c) {
  // SAFETY: kWhitespaceASCII is a NUL-terminated string.
  for (const char* cur = kWhitespaceASCII; *cur; UNSAFE_BUFFERS(++cur)) {
    if (*cur == c) {
      return true;
    }
  }
  return false;
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiAlpha(Char c) {
  return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiUpper(Char c) {
  return c >= 'A' && c <= 'Z';
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiLower(Char c) {
  return c >= 'a' && c <= 'z';
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiDigit(Char c) {
  return c >= '0' && c <= '9';
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiAlphaNumeric(Char c) {
  return IsAsciiAlpha(c) || IsAsciiDigit(c);
}
template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiPrintable(Char c) {
  return c >= ' ' && c <= '~';
}

template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiControl(Char c) {
  if constexpr (std::is_signed_v<Char>) {
    if (c < 0) {
      return false;
    }
  }
  return c <= 0x1f || c == 0x7f;
}

template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsUnicodeControl(Char c) {
  return IsAsciiControl(c) ||
         // C1 control characters: http://unicode.org/charts/PDF/U0080.pdf
         (c >= 0x80 && c <= 0x9F);
}

template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsAsciiPunctuation(Char c) {
  return c > 0x20 && c < 0x7f && !IsAsciiAlphaNumeric(c);
}

template <typename Char>
  requires(std::integral<Char>)
constexpr bool IsHexDigit(Char c) {
  return (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') ||
         (c >= 'a' && c <= 'f');
}

// Returns the integer corresponding to the given hex character. For example:
//    '4' -> 4
//    'a' -> 10
//    'B' -> 11
// Assumes the input is a valid hex character.
BASE_EXPORT char HexDigitToInt(char c);
inline char HexDigitToInt(char16_t c) {
  DCHECK(IsHexDigit(c));
  return HexDigitToInt(static_cast<char>(c));
}

// Returns whether `c` is a Unicode whitespace character.
// This cannot be used on eight-bit characters, since if they are ASCII you
// should call IsAsciiWhitespace(), and if they are from a UTF-8 string they may
// be individual units of a multi-unit code point.  Convert to 16- or 32-bit
// values known to hold the full code point before calling this.
template <typename Char>
  requires(sizeof(Char) > 1)
constexpr bool IsUnicodeWhitespace(Char c) {
  // SAFETY: kWhitespaceWide is a NUL-terminated string.
  for (const auto* cur = kWhitespaceWide; *cur; UNSAFE_BUFFERS(++cur)) {
    if (static_cast<typename std::make_unsigned_t<wchar_t>>(*cur) ==
        static_cast<typename std::make_unsigned_t<Char>>(c)) {
      return true;
    }
  }
  return false;
}

// DANGEROUS: Assumes ASCII or not based on the size of `Char`.  You should
// probably be explicitly calling IsUnicodeWhitespace() or IsAsciiWhitespace()
// instead!
template <typename Char>
constexpr bool IsWhitespace(Char c) {
  if constexpr (sizeof(Char) > 1) {
    return IsUnicodeWhitespace(c);
  } else {
    return IsAsciiWhitespace(c);
  }
}

// Return a byte string in human-readable format with a unit suffix. Not
// appropriate for use in any UI; use of FormatBytes and friends in ui/base is
// highly recommended instead. TODO(avi): Figure out how to get callers to use
// FormatBytes instead; remove this.
BASE_EXPORT std::u16string FormatBytesUnlocalized(int64_t bytes);

// Starting at |start_offset| (usually 0), replace the first instance of
// |find_this| with |replace_with|.
BASE_EXPORT void ReplaceFirstSubstringAfterOffset(
    std::u16string* str,
    size_t start_offset,
    std::u16string_view find_this,
    std::u16string_view replace_with);
BASE_EXPORT void ReplaceFirstSubstringAfterOffset(
    std::string* str,
    size_t start_offset,
    std::string_view find_this,
    std::string_view replace_with);

// Starting at |start_offset| (usually 0), look through |str| and replace all
// instances of |find_this| with |replace_with|.
//
// This does entire substrings; use std::replace in <algorithm> for single
// characters, for example:
//   std::replace(str.begin(), str.end(), 'a', 'b');
BASE_EXPORT void ReplaceSubstringsAfterOffset(std::u16string* str,
                                              size_t start_offset,
                                              std::u16string_view find_this,
                                              std::u16string_view replace_with);
BASE_EXPORT void ReplaceSubstringsAfterOffset(std::string* str,
                                              size_t start_offset,
                                              std::string_view find_this,
                                              std::string_view replace_with);

// Reserves enough memory in |str| to accommodate |length_with_null| characters,
// sets the size of |str| to |length_with_null - 1| characters, and returns a
// pointer to the underlying contiguous array of characters.  This is typically
// used when calling a function that writes results into a character array, but
// the caller wants the data to be managed by a string-like object.  It is
// convenient in that is can be used inline in the call, and fast in that it
// avoids copying the results of the call from a char* into a string.
//
// Internally, this takes linear time because the resize() call 0-fills the
// underlying array for potentially all
// (|length_with_null - 1| * sizeof(string_type::value_type)) bytes.  Ideally we
// could avoid this aspect of the resize() call, as we expect the caller to
// immediately write over this memory, but there is no other way to set the size
// of the string, and not doing that will mean people who access |str| rather
// than str.c_str() will get back a string of whatever size |str| had on entry
// to this function (probably 0).
BASE_EXPORT char* WriteInto(std::string* str, size_t length_with_null);
BASE_EXPORT char16_t* WriteInto(std::u16string* str, size_t length_with_null);

// Joins a list of strings into a single string, inserting |separator| (which
// may be empty) in between all elements.
//
// Note this is inverse of SplitString()/SplitStringPiece() defined in
// string_split.h.
//
// If possible, callers should build a vector of StringPieces and use the
// std::string_view variant, so that they do not create unnecessary copies of
// strings. For example, instead of using SplitString, modifying the vector,
// then using JoinString, use SplitStringPiece followed by JoinString so that no
// copies of those strings are created until the final join operation.
//
// Use StrCat (in base/strings/strcat.h) if you don't need a separator.
BASE_EXPORT std::string JoinString(span<const std::string> parts,
                                   std::string_view separator);
BASE_EXPORT std::u16string JoinString(span<const std::u16string> parts,
                                      std::u16string_view separator);
BASE_EXPORT std::string JoinString(span<const std::string_view> parts,
                                   std::string_view separator);
BASE_EXPORT std::u16string JoinString(span<const std::u16string_view> parts,
                                      std::u16string_view separator);
// Explicit initializer_list overloads are required to break ambiguity when used
// with a literal initializer list (otherwise the compiler would not be able to
// decide between the string and std::string_view overloads).
BASE_EXPORT std::string JoinString(
    std::initializer_list<std::string_view> parts,
    std::string_view separator);
BASE_EXPORT std::u16string JoinString(
    std::initializer_list<std::u16string_view> parts,
    std::u16string_view separator);

// Replace $1-$2-$3..$9 in the format string with values from |subst|.
// Additionally, any number of consecutive '$' characters is replaced by that
// number less one. Eg $$->$, $$$->$$, etc. The offsets parameter here can be
// NULL. This only allows you to use up to nine replacements.
//
// Calling ReplaceStringPlaceholders(u"$1", {ReturnU16string()}, nullptr) will
// unexpectedly give you the single-u16string overload below, the same as if you
// had written ReplaceStringPlaceholders(u"$1", ReturnU16String(), nullptr).
// This is surprising but mostly harmless. Call the base::span constructor
// explicitly if you need to force this overload, ie.
// ReplaceStringPlaceholders(
//     u"$1", base::span<const std::u16string>({ReturnU16string()}), nullptr).
BASE_EXPORT std::u16string ReplaceStringPlaceholders(
    std::u16string_view format_string,
    base::span<const std::u16string> subst,
    std::vector<size_t>* offsets);

BASE_EXPORT std::string ReplaceStringPlaceholders(
    std::string_view format_string,
    base::span<const std::string> subst,
    std::vector<size_t>* offsets);

// Single-string shortcut for ReplaceStringHolders. |offset| may be NULL.
BASE_EXPORT std::u16string ReplaceStringPlaceholders(
    std::u16string_view format_string,
    std::u16string_view subst,
    size_t* offset);

// Helper function for creating a std::string_view from a string literal that
// preserves internal NUL characters.
template <class CharT, size_t N>
std::basic_string_view<CharT> MakeStringViewWithNulChars(
    const CharT (&lit LIFETIME_BOUND)[N])
    ENABLE_IF_ATTR(lit[N - 1u] == CharT{0},
                   "requires string literal as input") {
  return std::basic_string_view<CharT>(lit, N - 1u);
}

}  // namespace base

#if BUILDFLAG(IS_WIN)
#include "base/strings/string_util_win.h"
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include "base/strings/string_util_posix.h"
#else
#error Define string operations appropriately for your platform
#endif

#endif  // BASE_STRINGS_STRING_UTIL_H_
