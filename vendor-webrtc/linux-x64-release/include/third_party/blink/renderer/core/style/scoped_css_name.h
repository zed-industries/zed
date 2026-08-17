// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SCOPED_CSS_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SCOPED_CSS_NAME_H_

#include <algorithm>

#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class TreeScope;

// Stores a CSS name as an AtomicString along with a TreeScope to support
// tree-scoped names and references for e.g. anchor-name. If the TreeScope
// pointer is null, we do not support such references, for instance for UA
// stylesheets.
class CORE_EXPORT ScopedCSSName : public GarbageCollected<ScopedCSSName> {
 public:
  ScopedCSSName(const AtomicString& name, const TreeScope* tree_scope)
      : name_(name), tree_scope_(tree_scope) {
    DCHECK(name);
  }

  const AtomicString& GetName() const { return name_; }
  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }

  bool operator==(const ScopedCSSName& other) const {
    return name_ == other.name_ && tree_scope_ == other.tree_scope_;
  }
  bool operator!=(const ScopedCSSName& other) const {
    return !operator==(other);
  }

  unsigned GetHash() const {
    unsigned hash = WTF::GetHash(name_);
    WTF::AddIntToHash(hash, WTF::GetHash(tree_scope_.Get()));
    return hash;
  }

  void Trace(Visitor* visitor) const;

 private:
  AtomicString name_;

  // Weak reference to break ref cycle with both GC-ed and ref-counted objects:
  // Document -> ComputedStyle -> ScopedCSSName -> TreeScope(Document)
  WeakMember<const TreeScope> tree_scope_;
};

// Represents a list of tree-scoped names (or tree-scoped references).
//
// https://drafts.csswg.org/css-scoping/#css-tree-scoped-name
// https://drafts.csswg.org/css-scoping/#css-tree-scoped-reference
class CORE_EXPORT ScopedCSSNameList
    : public GarbageCollected<ScopedCSSNameList> {
 public:
  explicit ScopedCSSNameList(HeapVector<Member<const ScopedCSSName>> names)
      : names_(std::move(names)) {
  }

  const HeapVector<Member<const ScopedCSSName>>& GetNames() const {
    return names_;
  }

  bool operator==(const ScopedCSSNameList& other) const {
    return std::ranges::equal(names_, other.names_,
                              [](const auto& a, const auto& b) {
                                return base::ValuesEquivalent(a, b);
                              });
  }
  bool operator!=(const ScopedCSSNameList& other) const {
    return !operator==(other);
  }

  void Trace(Visitor* visitor) const;

 private:
  HeapVector<Member<const ScopedCSSName>> names_;
};

}  // namespace blink

namespace WTF {

// Allows creating a hash table of ScopedCSSName in wrapper pointers (e.g.,
// HeapHashSet<Member<ScopedCSSName>>) that hashes the ScopedCSSNames directly
// instead of the wrapper pointers.

template <typename ScopedCSSNameWrapperType>
struct ScopedCSSNameWrapperPtrHashTraits
    : MemberHashTraits<ScopedCSSNameWrapperType> {
  using TraitType =
      typename MemberHashTraits<ScopedCSSNameWrapperType>::TraitType;
  static unsigned GetHash(const TraitType& name) { return name->GetHash(); }
  static bool Equal(const TraitType& a, const TraitType& b) {
    return base::ValuesEquivalent(a, b);
  }
  // Set this flag to 'false', otherwise Equal above will see gibberish values
  // that aren't safe to call ValuesEquivalent on.
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

template <>
struct HashTraits<blink::Member<blink::ScopedCSSName>>
    : ScopedCSSNameWrapperPtrHashTraits<blink::ScopedCSSName> {};
template <>
struct HashTraits<blink::Member<const blink::ScopedCSSName>>
    : ScopedCSSNameWrapperPtrHashTraits<const blink::ScopedCSSName> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SCOPED_CSS_NAME_H_
