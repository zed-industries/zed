/*
 * Copyright (C) 2011, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2011 Nokia Corporation and/or its subsidiary(-ies).
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_FAST_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_FAST_PATH_H_

#include <stdint.h>

#include <limits>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/types/zip.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/character_visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

// Assuming that a pointer is the size of a "machine word", then
// uintptr_t is an integer type that is also a machine word.
typedef uintptr_t MachineWord;
const uintptr_t kMachineWordAlignmentMask = sizeof(MachineWord) - 1;

inline bool IsAlignedToMachineWord(const void* pointer) {
  return !(reinterpret_cast<uintptr_t>(pointer) & kMachineWordAlignmentMask);
}

template <typename T>
inline T* AlignToMachineWord(T* pointer) {
  return reinterpret_cast<T*>(reinterpret_cast<uintptr_t>(pointer) &
                              ~kMachineWordAlignmentMask);
}

template <size_t size, typename CharacterType>
struct NonASCIIMask;
template <>
struct NonASCIIMask<4, UChar> {
  static inline uint32_t Value() { return 0xFF80FF80U; }
};
template <>
struct NonASCIIMask<4, LChar> {
  static inline uint32_t Value() { return 0x80808080U; }
};
template <>
struct NonASCIIMask<8, UChar> {
  static inline uint64_t Value() { return 0xFF80FF80FF80FF80ULL; }
};
template <>
struct NonASCIIMask<8, LChar> {
  static inline uint64_t Value() { return 0x8080808080808080ULL; }
};

template <typename CharacterType>
inline bool IsAllASCII(MachineWord word) {
  return !(word & NonASCIIMask<sizeof(MachineWord), CharacterType>::Value());
}

struct ASCIIStringAttributes {
  ASCIIStringAttributes(bool contains_only_ascii, bool is_lower_ascii)
      : contains_only_ascii(contains_only_ascii),
        is_lower_ascii(is_lower_ascii) {}
  unsigned contains_only_ascii : 1;
  unsigned is_lower_ascii : 1;
};

// Note: This function assumes the input is likely all ASCII, and
// does not leave early if it is not the case.
template <typename CharacterType>
ALWAYS_INLINE ASCIIStringAttributes
CharacterAttributes(base::span<const CharacterType> chars) {
  DCHECK_GT(chars.size(), 0u);

  // Performance note: This loop will not vectorize properly in -Oz. Ensure
  // the calling code is built with -O2.
  CharacterType all_char_bits = 0;
  bool contains_upper_case = false;
  for (CharacterType ch : chars) {
    all_char_bits |= ch;
    contains_upper_case |= IsASCIIUpper(ch);
  }

  return ASCIIStringAttributes(IsASCII(all_char_bits), !contains_upper_case);
}

template <typename CharacterType>
ALWAYS_INLINE bool IsLowerASCII(base::span<const CharacterType> chars) {
  bool contains_upper_case = false;
  for (CharacterType ch : chars) {
    contains_upper_case |= IsASCIIUpper(ch);
  }
  return !contains_upper_case;
}

template <typename CharacterType>
ALWAYS_INLINE bool IsUpperASCII(base::span<const CharacterType> chars) {
  bool contains_lower_case = false;
  for (CharacterType ch : chars) {
    contains_lower_case |= IsASCIILower(ch);
  }
  return !contains_lower_case;
}

class LowerConverter {
 public:
  template <typename CharType>
  ALWAYS_INLINE static bool IsCorrectCase(base::span<const CharType> chars) {
    return IsLowerASCII(chars);
  }

  template <typename CharType>
  ALWAYS_INLINE static CharType Convert(CharType ch) {
    return ToASCIILower(ch);
  }
};

class UpperConverter {
 public:
  template <typename CharType>
  ALWAYS_INLINE static bool IsCorrectCase(base::span<const CharType> chars) {
    return IsUpperASCII(chars);
  }

  template <typename CharType>
  ALWAYS_INLINE static CharType Convert(CharType ch) {
    return ToASCIIUpper(ch);
  }
};

template <typename StringType, typename Converter, typename Allocator>
ALWAYS_INLINE typename Allocator::ResultStringType ConvertASCIICase(
    const StringType& string,
    Converter&& converter,
    Allocator&& allocator) {
  CHECK_LE(string.length(), std::numeric_limits<wtf_size_t>::max());
  return VisitCharacters(string, [&](auto chars) {
    // First scan the string for the desired case.
    if (converter.IsCorrectCase(chars)) {
      return allocator.CoerceOriginal(string);
    }

    base::span<typename decltype(chars)::value_type> data;
    auto new_impl = allocator.Alloc(string.length(), data);

    for (auto [dest, src] : base::zip(data, chars)) {
      dest = converter.Convert(src);
    }
    return new_impl;
  });
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_FAST_PATH_H_
