// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_SELECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Element;

// Not to be confused with regular selectors. This refers to container
// selection by e.g. a given name, or by implicit container selection
// according to the queried features.
//
// https://drafts.csswg.org/css-contain-3/#container-rule
class CORE_EXPORT ContainerSelector {
 public:
  ContainerSelector() = default;
  explicit ContainerSelector(WTF::HashTableDeletedValueType) {
    WTF::HashTraits<AtomicString>::ConstructDeletedValue(name_);
  }
  // Used for the purpose of finding the closest container for container units.
  explicit ContainerSelector(PhysicalAxes physical_axes)
      : physical_axes_(physical_axes) {}
  // Used for the purpose of finding the closest container for container units
  // and looking up the closest container matching a certain container-type for
  // the inspector (InspectorDOMAgent::getContainerForNode()).
  ContainerSelector(AtomicString name,
                    PhysicalAxes physical_axes,
                    LogicalAxes logical_axes,
                    bool scroll_state)
      : name_(std::move(name)),
        physical_axes_(physical_axes),
        logical_axes_(logical_axes),
        has_sticky_query_(scroll_state),
        has_snap_query_(scroll_state),
        has_scrollable_query_(scroll_state) {}
  ContainerSelector(AtomicString name, const MediaQueryExpNode&);

  bool IsHashTableDeletedValue() const {
    return WTF::HashTraits<AtomicString>::IsDeletedValue(name_);
  }

  bool operator==(const ContainerSelector& o) const {
    return (name_ == o.name_) && (physical_axes_ == o.physical_axes_) &&
           (logical_axes_ == o.logical_axes_) &&
           (has_style_query_ == o.has_style_query_) &&
           (has_sticky_query_ == o.has_sticky_query_) &&
           (has_snap_query_ == o.has_snap_query_) &&
           (has_scrollable_query_ == o.has_scrollable_query_);
  }
  bool operator!=(const ContainerSelector& o) const { return !(*this == o); }

  unsigned GetHash() const;

  const AtomicString& Name() const { return name_; }

  // Given the specified writing mode, return the EContainerTypes required
  // for this selector to match.
  unsigned Type(WritingMode) const;

  bool SelectsSizeContainers() const {
    return physical_axes_ != kPhysicalAxesNone ||
           logical_axes_ != kLogicalAxesNone;
  }

  bool SelectsStyleContainers() const { return has_style_query_; }
  bool SelectsStickyContainers() const { return has_sticky_query_; }
  bool SelectsSnapContainers() const { return has_snap_query_; }
  bool SelectsScrollableContainers() const { return has_scrollable_query_; }
  bool SelectsScrollStateContainers() const {
    return SelectsStickyContainers() || SelectsSnapContainers() ||
           SelectsScrollableContainers();
  }
  bool HasUnknownFeature() const { return has_unknown_feature_; }
  bool SelectsAnyContainer() const {
    return !HasUnknownFeature() &&
           (SelectsSizeContainers() || SelectsStyleContainers() ||
            SelectsScrollStateContainers());
  }

  PhysicalAxes GetPhysicalAxes() const { return physical_axes_; }
  LogicalAxes GetLogicalAxes() const { return logical_axes_; }

 private:
  AtomicString name_;
  PhysicalAxes physical_axes_{kPhysicalAxesNone};
  LogicalAxes logical_axes_{kLogicalAxesNone};
  bool has_style_query_{false};
  bool has_sticky_query_{false};
  bool has_snap_query_{false};
  bool has_scrollable_query_{false};
  bool has_unknown_feature_{false};
};

class CORE_EXPORT ScopedContainerSelector
    : public GarbageCollected<ScopedContainerSelector> {
 public:
  ScopedContainerSelector(ContainerSelector selector,
                          const TreeScope* tree_scope)
      : selector_(selector), tree_scope_(tree_scope) {}

  unsigned GetHash() const {
    unsigned hash = selector_.GetHash();
    WTF::AddIntToHash(hash, WTF::GetHash(tree_scope_.Get()));
    return hash;
  }

  bool operator==(const ScopedContainerSelector& other) const {
    return selector_ == other.selector_ && tree_scope_ == other.tree_scope_;
  }

  void Trace(Visitor* visitor) const;

 private:
  ContainerSelector selector_;
  WeakMember<const TreeScope> tree_scope_;
};

struct ScopedContainerSelectorHashTraits
    : WTF::MemberHashTraits<ScopedContainerSelector> {
  static unsigned GetHash(
      const Member<ScopedContainerSelector>& scoped_selector) {
    return scoped_selector->GetHash();
  }
  static bool Equal(const Member<ScopedContainerSelector>& a,
                    const Member<ScopedContainerSelector>& b) {
    return *a == *b;
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

// Helper needed to allow calling Find() with a ScopedContainerSelector instead
// of Member<ScopedContainerSelector>
struct ScopedContainerSelectorHashTranslator {
  STATIC_ONLY(ScopedContainerSelectorHashTranslator);

  static unsigned GetHash(const ScopedContainerSelector& selector) {
    return selector.GetHash();
  }
  static bool Equal(const Member<ScopedContainerSelector>& a,
                    const ScopedContainerSelector& b) {
    return a && *a == b;
  }
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::ContainerSelector>
    : SimpleClassHashTraits<blink::ContainerSelector> {
  static unsigned GetHash(const blink::ContainerSelector& selector) {
    return selector.GetHash();
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted =
      HashTraits<AtomicString>::kSafeToCompareToEmptyOrDeleted;
  static const bool kEmptyValueIsZero = false;
};

}  // namespace WTF

namespace blink {

using ContainerSelectorCache = HeapHashMap<Member<ScopedContainerSelector>,
                                           Member<Element>,
                                           ScopedContainerSelectorHashTraits>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_SELECTOR_H_
