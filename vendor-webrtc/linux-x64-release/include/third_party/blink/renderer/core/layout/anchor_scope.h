// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// See ToAnchorScopedName().
class CORE_EXPORT AnchorScopedName : public GarbageCollected<AnchorScopedName> {
 public:
  AnchorScopedName(const ScopedCSSName& name,
                   const Element* anchor_scope_element)
      : name_(&name), anchor_scope_element_(anchor_scope_element) {}

  const AtomicString& GetName() const { return name_->GetName(); }

  bool operator==(const AnchorScopedName& other) const {
    return base::ValuesEquivalent(name_, other.name_) &&
           anchor_scope_element_ == other.anchor_scope_element_;
  }
  bool operator!=(const AnchorScopedName& other) const {
    return !operator==(other);
  }

  unsigned GetHash() const {
    unsigned hash = name_->GetHash();
    WTF::AddIntToHash(hash, WTF::GetHash(anchor_scope_element_.Get()));
    return hash;
  }

  void Trace(Visitor* visitor) const;

 private:
  Member<const ScopedCSSName> name_;
  Member<const Element> anchor_scope_element_;
};

// Traverses the flat-tree ancestors of the specified layout object's element,
// looking for a matching anchor-scope value, and creates a corresponding
// AnchorScopedName.
//
// The result is used as a key in the anchor maps (AnchorQueryBase:
// named_anchors_). By taking into account anchor-scope in the key,
// we can avoid traversing AnchorReferences outside the relevant anchor-scope
// during lookup (PhysicalAnchorQuery::AnchorReference).
AnchorScopedName* ToAnchorScopedName(const ScopedCSSName&, const LayoutObject&);

}  // namespace blink

namespace WTF {

// Allows creating a hash table of Member<AnchorScopedName> that hashes the
// AnchorScopedNames by value instead of address.
template <typename T>
struct AnchorScopedNameHashTraits : MemberHashTraits<T> {
  using TraitType = typename MemberHashTraits<T>::TraitType;
  static unsigned GetHash(const TraitType& name) { return name->GetHash(); }
  static bool Equal(const TraitType& a, const TraitType& b) {
    return base::ValuesEquivalent(a, b);
  }
  // Set this flag to 'false', otherwise Equal above will see gibberish values
  // that aren't safe to call ValuesEquivalent on.
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

template <>
struct HashTraits<blink::Member<blink::AnchorScopedName>>
    : AnchorScopedNameHashTraits<blink::AnchorScopedName> {};
template <>
struct HashTraits<blink::Member<const blink::AnchorScopedName>>
    : AnchorScopedNameHashTraits<const blink::AnchorScopedName> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_SCOPE_H_
