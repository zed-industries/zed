// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_NAMES_H_

#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

// SanitizerNameHashTraits defines traits suitable for Sanitizer use.
//
// Sanitizer stores all names as blink::QualifiedName, which is used all
// over Blink. QualifiedName has three components, local name, prefix, and
// namespace URI. For Sanitizer, the spec demands that we always match on
// name + namespace, but never on the prefix. These traits define equality
// and hash function to match this. This allows us to re-use QualifiedName
// and the "standard" WTF structures.
struct SanitizerNameHashTraits : GenericHashTraits<blink::QualifiedName> {
  static unsigned GetHash(const QualifiedName& name) {
    CHECK(name.LocalName());
    unsigned hash = HashTraits<AtomicString>::GetHash(name.LocalName());
    if (name.NamespaceURI()) {
      hash ^= HashTraits<AtomicString>::GetHash(name.NamespaceURI());
    }
    return hash;
  }
  static bool Equal(const QualifiedName& a, const QualifiedName& b) {
    return a.Matches(b);
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
  static constexpr bool kEmptyValueIsZero = false;
  static const QualifiedName& EmptyValue() { return g_null_name; }
  static const QualifiedName& DeletedValue() {
    // This is a bit of a cheat: We re-use the "*" name -- which is not a valid
    // element or attribute name -- as the "deleted" name.
    return g_any_name;
  }
};

typedef HashSet<QualifiedName, SanitizerNameHashTraits> SanitizerNameSet;
typedef HashMap<QualifiedName, SanitizerNameSet, SanitizerNameHashTraits>
    SanitizerNameMap;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_NAMES_H_
