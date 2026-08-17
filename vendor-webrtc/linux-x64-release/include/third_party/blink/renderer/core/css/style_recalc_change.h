// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RECALC_CHANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RECALC_CHANGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ComputedStyle;
class Element;
class Node;
class PseudoElement;

// Class for keeping track of the need for traversing down flat tree children,
// recompute their computed styles, and marking nodes for layout tree re-
// attachment during the style recalc phase.
class CORE_EXPORT StyleRecalcChange {
 private:
  enum Flag {
    kNoFlags = 0,
    // Recalc size container query dependent elements within this container,
    // but not in nested containers.
    kRecalcSizeContainer = 1 << 0,
    // Recalc size container query dependent elements within this container,
    // and also in nested containers.
    kRecalcDescendantSizeContainers = 1 << 1,
    // Recalc size container query dependent elements within this container,
    // but not in nested containers.
    kRecalcStyleContainerChildren = 1 << 2,
    // Recalc size container query dependent elements within this container,
    // and also in nested containers.
    kRecalcStyleContainerDescendants = 1 << 3,
    // Recalc scroll-state container query dependent elements within this
    // container, but not in nested containers.
    kRecalcScrollStateContainer = 1 << 4,
    // Recalc scroll-state container query dependent elements within this
    // container, and also in nested containers.
    kRecalcDescendantScrollStateContainers = 1 << 5,
    // Recalc descendant content-visibility elements within a changed
    // scroll-marker-group property elements.
    kRecalcDescendantContentVisibility = 1 << 6,
    // If set, need to reattach layout tree.
    kReattach = 1 << 7,
    // If set, will prevent style recalc for the node passed to
    // ShouldRecalcStyleFor. This flag is lost when ForChildren is called.
    kSuppressRecalc = 1 << 8,
    // If set, and kReattach is also set, the element should be explicitly
    // marked for re-attachment even if its style doesn't change. Used for query
    // container children to resume re-attachment that was blocked when style
    // recalc for container children was skipped.
    kMarkReattach = 1 << 9,
  };
  using Flags = uint16_t;

  static const Flags kRecalcSizeContainerFlags =
      kRecalcSizeContainer | kRecalcDescendantSizeContainers;

  static const Flags kRecalcStyleContainerFlags =
      kRecalcStyleContainerChildren | kRecalcStyleContainerDescendants;

  static const Flags kRecalcScrollStateContainerFlags =
      kRecalcScrollStateContainer | kRecalcDescendantScrollStateContainers;

  static const Flags kRecalcContainerFlags = kRecalcSizeContainerFlags |
                                             kRecalcStyleContainerFlags |
                                             kRecalcScrollStateContainerFlags;

 public:
  enum Propagate {
    // No need to update style of any children.
    kNo,
    // Need to update existence and style for pseudo elements.
    kUpdatePseudoElements,
    // Need to recalculate style for children for inheritance. All changed
    // inherited properties can be propagated (PropagateInheritedProperties)
    // instead of a full rule matching.
    kIndependentInherit,
    // Need to recalculate style for children, typically for inheritance.
    kRecalcChildren,
    // Need to recalculate style for all descendants.
    kRecalcDescendants,
  };

  StyleRecalcChange() = default;
  StyleRecalcChange(const StyleRecalcChange&) = default;
  StyleRecalcChange& operator=(const StyleRecalcChange&) = default;
  explicit StyleRecalcChange(Propagate propagate) : propagate_(propagate) {}

  bool IsEmpty() const { return !propagate_ && !flags_; }

  StyleRecalcChange ForChildren(const Element& element) const {
    return {RecalcDescendants() ? kRecalcDescendants : kNo,
            FlagsForChildren(element)};
  }
  StyleRecalcChange ForPseudoElement() const {
    if (propagate_ == kUpdatePseudoElements) {
      return {kRecalcChildren, flags_};
    }
    return *this;
  }
  StyleRecalcChange EnsureAtLeast(Propagate propagate) const {
    if (propagate > propagate_) {
      return {propagate, flags_};
    }
    return {propagate_, flags_};
  }
  StyleRecalcChange ForceRecalcDescendants() const {
    return {kRecalcDescendants, flags_};
  }
  StyleRecalcChange ForceRecalcChildren() const {
    return {kRecalcChildren, flags_};
  }
  StyleRecalcChange ForceReattachLayoutTree() const {
    return {propagate_, static_cast<Flags>(flags_ | kReattach)};
  }
  StyleRecalcChange ForceMarkReattachLayoutTree() const {
    return {propagate_, static_cast<Flags>(flags_ | kMarkReattach)};
  }
  StyleRecalcChange ForceRecalcSizeContainer() const {
    return {propagate_, static_cast<Flags>(flags_ | kRecalcSizeContainer)};
  }
  StyleRecalcChange ForceRecalcDescendantSizeContainers() const {
    return {propagate_,
            static_cast<Flags>(flags_ | kRecalcDescendantSizeContainers)};
  }
  StyleRecalcChange ForceRecalcStyleContainerChildren() const {
    return {propagate_,
            static_cast<Flags>(flags_ | kRecalcStyleContainerChildren)};
  }
  StyleRecalcChange ForceRecalcStyleContainerDescendants() const {
    return {propagate_,
            static_cast<Flags>(flags_ | kRecalcStyleContainerDescendants)};
  }
  StyleRecalcChange ForceRecalcScrollStateContainer() const {
    return {propagate_,
            static_cast<Flags>(flags_ | kRecalcScrollStateContainer)};
  }
  StyleRecalcChange ForceRecalcDescendantScrollStateContainers() const {
    return {propagate_, static_cast<Flags>(
                            flags_ | kRecalcDescendantScrollStateContainers)};
  }
  StyleRecalcChange ForceRecalcDescendantContentVisibility() const {
    return {propagate_,
            static_cast<Flags>(flags_ | kRecalcDescendantContentVisibility)};
  }
  StyleRecalcChange SuppressRecalc() const {
    return {propagate_, static_cast<Flags>(flags_ | kSuppressRecalc)};
  }
  StyleRecalcChange Combine(const StyleRecalcChange& other) const {
    return {std::max(propagate_, other.propagate_),
            static_cast<Flags>(flags_ | other.flags_)};
  }

  bool ReattachLayoutTree() const { return flags_ & kReattach; }
  bool MarkReattachLayoutTree() const {
    // Never mark the query container (kSuppressRecalc) for reattachment.
    return (flags_ & (kMarkReattach | kReattach | kSuppressRecalc)) ==
           (kMarkReattach | kReattach);
  }
  bool RecalcChildren() const { return propagate_ > kUpdatePseudoElements; }
  bool RecalcDescendants() const { return propagate_ == kRecalcDescendants; }
  bool RecalcContainerQueryDependent(const Node&) const;
  bool UpdatePseudoElements() const { return propagate_ != kNo; }
  // Returns true if we should and can do independent inheritance. The passed in
  // computed style is the existing style for the element we are considering.
  // It is used to check if we need to do a normal recalc for container query
  // dependent elements.
  bool IndependentInherit(const ComputedStyle& old_style) const;
  bool TraverseChildren(const Element&) const;
  bool TraverseChild(const Node&) const;
  bool TraversePseudoElements(const Element&) const;
  bool ShouldRecalcStyleFor(const Node&) const;
  bool ShouldUpdatePseudoElement(const PseudoElement&) const;
  bool IsSuppressed() const { return flags_ & kSuppressRecalc; }

  // If true, the value of the 'rem' unit may have changed.
  //
  // We currently can't distinguish between kRecalcDescendants caused by
  // root-font-size changes and kRecalcDescendants that happens for other
  // reasons.
  //
  // See call to `UpdateRemUnits` in `Element::RecalcOwnStyle`.
  bool RemUnitsMaybeChanged() const { return RecalcDescendants(); }

  // If true, the values of container-relative units may have changed.
  //
  // Any ContainerQueryEvaluator that has been referenced by a unit will
  // always cause kRecalcDescendantSizeContainers (see
  // ContainerQueryEvaluator::ComputeSizeChange). Currently we can not
  // distinguish between that and kRecalcDescendantSizeContainers caused by
  // other reasons (e.g. named lookups).
  bool ContainerRelativeUnitsMaybeChanged() const {
    return flags_ & kRecalcDescendantSizeContainers;
  }

  String ToString() const;

 private:
  StyleRecalcChange(Propagate propagate, Flags flags)
      : propagate_(propagate), flags_(flags) {}

  bool RecalcSizeContainerQueryDependent() const {
    return flags_ & kRecalcSizeContainerFlags;
  }
  bool RecalcStyleContainerQueryDependent() const {
    return flags_ & kRecalcStyleContainerFlags;
  }
  bool RecalcScrollStateContainerQueryDependent() const {
    return flags_ & kRecalcScrollStateContainerFlags;
  }
  bool RecalcContainerQueryDependent() const {
    return flags_ & kRecalcContainerFlags;
  }
  bool RecalcDescendantContentVisibility() const {
    return flags_ & kRecalcDescendantContentVisibility;
  }
  Flags FlagsForChildren(const Element&) const;

  // To what extent do we need to update style for children.
  Propagate propagate_ = kNo;
  // See StyleRecalc::Flag.
  Flags flags_ = kNoFlags;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RECALC_CHANGE_H_
