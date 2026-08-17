/*
 * Copyright (C) 2006, 2007, 2008, 2012, 2013 Apple Inc. All rights reserved
 * Copyright (C) Research In Motion Limited 2009. All rights reserved.
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

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_FOLDING_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_FOLDING_HASH_H_

// Case-insensitive hash lookups, using the Unicode case folding algorithm.

#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/unicode.h"

namespace WTF {

// Sends the input data through the Unicode case-folding table.
// Unlike normal PlainHashReader, or the ASCII lower-case lookups,
// we cannot treat 8-bit and 16-bit data separately, as the lookups
// may change strings from one status to the other. For instance,
// µ is in Latin1, but gets case-folded to U+03BC GREEK SMALL LETTER MU,
// and there are examples going the other way as well.
//
// We could perhaps tweak the tables to avoid this, but this is not
// the most performance-sensitive hashing we have around, so we simply
// always treat data as UTF-16, expanding Latin1 as we go. This means
// we also don't bother to try to make tricky SIMD implementations
// for Latin1; we just use the most straightforward code. (Full lookup
// into WTF::unicode::FoldCase is slow enough that it probably dwarfs
// all other performance concerns anyway.)
template <class T>
  requires std::is_same_v<T, LChar> || std::is_same_v<T, UChar>
struct CaseFoldingHashReader {
  // We never contract 16 to 8 bits, so this must always be 1.
  static constexpr unsigned kCompressionFactor = 1;

  // We always produce UTF-16 output, even if we take in Latin1.
  static constexpr unsigned kExpansionFactor = sizeof(UChar) / sizeof(T);

  static inline uint64_t Read64(const uint8_t* ptr) {
    const T* p = reinterpret_cast<const T*>(ptr);
    return FoldCase(p[0]) | (FoldCase(p[1]) << 16) | (FoldCase(p[2]) << 32) |
           (FoldCase(p[3]) << 48);
  }

  static inline uint64_t Read32(const uint8_t* ptr) {
    const T* p = reinterpret_cast<const T*>(ptr);
    return FoldCase(p[0]) | (FoldCase(p[1]) << 16);
  }

  static inline uint64_t ReadSmall(const uint8_t* ptr, size_t k) {
    const T* p = reinterpret_cast<const T*>(ptr);
    DCHECK_EQ(k, 2u);
    return FoldCase(p[0]);
  }

 private:
  // Private so no one uses this in the belief that it will return the
  // correctly-folded code point in all cases (see comment below).
  static inline uint64_t FoldCase(T ch) {
    if (std::is_same<T, LChar>::value) {
      return StringImpl::kLatin1CaseFoldTable[ch];
    }
    // It's possible for WTF::unicode::foldCase() to return a 32-bit value
    // that's not representable as a UChar.  However, since this is rare and
    // deterministic, and the result of this is merely used for hashing, go
    // ahead and clamp the value.
    return static_cast<UChar>(WTF::unicode::FoldCase(ch));
  }
};

// The GetHash() functions on CaseFoldingHashTraits do not support null strings.
// find(), Contains(), and insert() on
// HashMap<String,..., CaseFoldingHashTraits<String>>
// cause a null-pointer dereference when passed null strings.
class CaseFoldingHash {
  STATIC_ONLY(CaseFoldingHash);

 public:
  static unsigned GetHash(const UChar* data, unsigned length) {
    return StringHasher::ComputeHashAndMaskTop8Bits<
        CaseFoldingHashReader<UChar>>(reinterpret_cast<const char*>(data),
                                      length * 2);
  }

  static unsigned GetHash(StringImpl* str) {
    if (str->Is8Bit())
      return GetHash(str->Characters8(), str->length());
    return GetHash(str->Characters16(), str->length());
  }

  static unsigned GetHash(const LChar* data, unsigned length) {
    return StringHasher::ComputeHashAndMaskTop8Bits<
        CaseFoldingHashReader<LChar>>(reinterpret_cast<const char*>(data),
                                      length * 2);
  }

  static inline unsigned GetHash(const char* data, unsigned length) {
    return GetHash(reinterpret_cast<const LChar*>(data), length);
  }

  static inline unsigned GetHash(const char* data) {
    return GetHash(reinterpret_cast<const LChar*>(data),
                   static_cast<unsigned>(strlen(data)));
  }

  static inline bool Equal(const StringImpl* a, const StringImpl* b) {
    DCHECK(a);
    DCHECK(b);
    // Save one branch inside each StringView by derefing the StringImpl,
    // and another branch inside the compare function by skipping the null
    // checks.
    return DeprecatedEqualIgnoringCaseAndNullity(*a, *b);
  }

  static inline bool Equal(const char* a, const char* b) {
    DCHECK(a);
    DCHECK(b);
    return DeprecatedEqualIgnoringCaseAndNullity(a, b);
  }

  static unsigned GetHash(const scoped_refptr<StringImpl>& key) {
    return GetHash(key.get());
  }

  static bool Equal(const scoped_refptr<StringImpl>& a,
                    const scoped_refptr<StringImpl>& b) {
    return Equal(a.get(), b.get());
  }

  static unsigned GetHash(const String& key) { return GetHash(key.Impl()); }
  static unsigned GetHash(const AtomicString& key) {
    return GetHash(key.Impl());
  }
  static bool Equal(const String& a, const String& b) {
    return Equal(a.Impl(), b.Impl());
  }
  static bool Equal(const AtomicString& a, const AtomicString& b) {
    return (a == b) || Equal(a.Impl(), b.Impl());
  }

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

// T can be String, StringImpl*, scoped_refptr<StringImpl> and AtomicString.
template <typename T>
struct CaseFoldingHashTraits : HashTraits<T>, CaseFoldingHash {
  using CaseFoldingHash::Equal;
  using CaseFoldingHash::GetHash;
  using CaseFoldingHash::kSafeToCompareToEmptyOrDeleted;
};

}  // namespace WTF

using WTF::CaseFoldingHash;
using WTF::CaseFoldingHashTraits;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_FOLDING_HASH_H_
