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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_HASH_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hasher.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace WTF {

// The GetHash() functions in below HashTraits do not support null strings.
// find(), Contains(), and insert() on HashMap<String,...> cause a null-pointer
// dereference when passed null strings.

template <>
struct HashTraits<StringImpl*> : GenericHashTraits<StringImpl*> {
  static unsigned GetHash(const StringImpl* key) { return key->GetHash(); }
  static inline bool Equal(const StringImpl* a, const StringImpl* b) {
    return EqualNonNull(a, b);
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
  static constexpr int x = 10;
};

template <>
struct HashTraits<scoped_refptr<StringImpl>>
    : GenericHashTraits<scoped_refptr<StringImpl>> {
  static unsigned GetHash(const scoped_refptr<StringImpl>& key) {
    return key->GetHash();
  }
  static bool Equal(const scoped_refptr<StringImpl>& a,
                    const scoped_refptr<StringImpl>& b) {
    return EqualNonNull(a.get(), b.get());
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

template <>
struct HashTraits<String> : SimpleClassHashTraits<String> {
  static unsigned GetHash(const String& key) { return key.Impl()->GetHash(); }
  static bool Equal(const String& a, const String& b) {
    return EqualNonNull(a.Impl(), b.Impl());
  }

  // Avoid implicit conversion to String just to hash or compare.
  // We would like to add overloads for StringView and AtomicString too,
  // but there are classes (e.g. WebString, V8StringResource) with
  // implicit conversion operators both to String and one of the others,
  // which would cause ambiguous overloads.
  static unsigned GetHash(const char* key) {
    return StringHasher::ComputeHashAndMaskTop8Bits(key, strlen(key));
  }
  static unsigned GetHash(const LChar* key) {
    return GetHash(reinterpret_cast<const char*>(key));
  }
  static unsigned GetHash(const UChar* key) {
    return ComputeHashForWideString(key, LengthOfNullTerminatedString(key));
  }

  static bool Equal(const String& a, const char* b) { return a == b; }
  static bool Equal(const char* a, const String& b) { return a == b; }
  static bool Equal(const String& a, const LChar* b) {
    return a == reinterpret_cast<const char*>(b);
  }
  static bool Equal(const LChar* a, const String& b) {
    return reinterpret_cast<const char*>(a) == b;
  }
  static bool Equal(const String& a, const UChar* b) { return a == b; }
  static bool Equal(const UChar* a, const String& b) { return a == b; }
  // NOTE: There are no String == StringView overloads, so we also make no
  // Equal() for them.

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
  static bool IsEmptyValue(const String& s) { return s.IsNull(); }
  static bool IsDeletedValue(const String& s) {
    return HashTraits<scoped_refptr<StringImpl>>::IsDeletedValue(s.impl_);
  }
  static void ConstructDeletedValue(String& slot) {
    HashTraits<scoped_refptr<StringImpl>>::ConstructDeletedValue(slot.impl_);
  }
};

}  // namespace WTF

namespace std {
template <>
struct hash<WTF::String> {
  size_t operator()(const WTF::String& string) const {
    return WTF::GetHash(string);
  }
};
}  // namespace std

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_HASH_H_
