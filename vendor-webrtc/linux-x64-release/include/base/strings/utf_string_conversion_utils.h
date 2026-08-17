// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_UTF_STRING_CONVERSION_UTILS_H_
#define BASE_STRINGS_UTF_STRING_CONVERSION_UTILS_H_

// Low-level UTF handling functions. Most code will want to use the functions
// in utf_string_conversions.h

#include <stddef.h>
#include <stdint.h>

#include <limits>
#include <optional>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/third_party/icu/icu_utf.h"
#include "build/build_config.h"

namespace base {

inline bool IsValidCodepoint(base_icu::UChar32 code_point) {
  // Excludes code points that are not Unicode scalar values, i.e.
  // surrogate code points ([0xD800, 0xDFFF]). Additionally, excludes
  // code points larger than 0x10FFFF (the highest codepoint allowed).
  // Non-characters and unassigned code points are allowed.
  // https://unicode.org/glossary/#unicode_scalar_value
  return (code_point >= 0 && code_point < 0xD800) ||
         (code_point >= 0xE000 && code_point <= 0x10FFFF);
}

inline bool IsValidCharacter(base_icu::UChar32 code_point) {
  // Excludes non-characters (U+FDD0..U+FDEF, and all code points
  // ending in 0xFFFE or 0xFFFF) from the set of valid code points.
  // https://unicode.org/faq/private_use.html#nonchar1
  return (code_point >= 0 && code_point < 0xD800) ||
         (code_point >= 0xE000 && code_point < 0xFDD0) ||
         (code_point > 0xFDEF && code_point <= 0x10FFFF &&
          (code_point & 0xFFFE) != 0xFFFE);
}

// CountUnicodeCharacters ------------------------------------------------------

// Returns the number of Unicode characters in `text`, up to the supplied
// `limit`, if `text` contains valid UTF-8. Returns `nullopt` otherwise.
BASE_EXPORT std::optional<size_t> CountUnicodeCharacters(
    std::string_view text,
    size_t limit = std::numeric_limits<size_t>::max());

// ReadUnicodeCharacter --------------------------------------------------------

// Reads a UTF-8 stream, placing the next code point into the given output
// |*code_point|. |src| represents the entire string to read, and |*char_index|
// is the character offset within the string to start reading at. |*char_index|
// will be updated to index the last character read, such that incrementing it
// (as in a for loop) will take the reader to the next character.
//
// Returns true on success. On false, |*code_point| will be invalid.
BASE_EXPORT bool ReadUnicodeCharacter(const char* src,
                                      size_t src_len,
                                      size_t* char_index,
                                      base_icu::UChar32* code_point_out);

// Reads a UTF-16 character. The usage is the same as the 8-bit version above.
BASE_EXPORT bool ReadUnicodeCharacter(const char16_t* src,
                                      size_t src_len,
                                      size_t* char_index,
                                      base_icu::UChar32* code_point);

#if defined(WCHAR_T_IS_32_BIT)
// Reads UTF-32 character. The usage is the same as the 8-bit version above.
BASE_EXPORT bool ReadUnicodeCharacter(const wchar_t* src,
                                      size_t src_len,
                                      size_t* char_index,
                                      base_icu::UChar32* code_point);
#endif  // defined(WCHAR_T_IS_32_BIT)

// WriteUnicodeCharacter -------------------------------------------------------

// Appends a UTF-8 character to the given 8-bit string.  Returns the number of
// bytes written.
BASE_EXPORT size_t WriteUnicodeCharacter(base_icu::UChar32 code_point,
                                         std::string* output);

// Appends the given code point as a UTF-16 character to the given 16-bit
// string.  Returns the number of 16-bit values written.
BASE_EXPORT size_t WriteUnicodeCharacter(base_icu::UChar32 code_point,
                                         std::u16string* output);

#if defined(WCHAR_T_IS_32_BIT)
// Appends the given UTF-32 character to the given 32-bit string.  Returns the
// number of 32-bit values written.
inline size_t WriteUnicodeCharacter(base_icu::UChar32 code_point,
                                    std::wstring* output) {
  // This is the easy case, just append the character.
  output->push_back(static_cast<wchar_t>(code_point));
  return 1;
}
#endif  // defined(WCHAR_T_IS_32_BIT)

// Generalized Unicode converter -----------------------------------------------

// Guesses the length of the output in UTF-8 in bytes, clears that output
// string, and reserves that amount of space.  We assume that the input
// character types are unsigned, which will be true for UTF-16 and -32 on our
// systems.
template <typename CHAR>
void PrepareForUTF8Output(const CHAR* src, size_t src_len, std::string* output);

// Prepares an output buffer (containing either UTF-16 or -32 data) given some
// UTF-8 input that will be converted to it.  See PrepareForUTF8Output().
template <typename STRING>
void PrepareForUTF16Or32Output(const char* src, size_t src_len, STRING* output);

}  // namespace base

#endif  // BASE_STRINGS_UTF_STRING_CONVERSION_UTILS_H_
