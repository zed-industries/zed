// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_STYLE_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_STYLE_INVALIDATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_flags.h"
#include "third_party/blink/renderer/core/css/invalidation/pending_invalidations.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ContainerNode;
class Element;
class HTMLSlotElement;
class InvalidationSet;

// Applies deferred style invalidation for DOM subtrees.
//
// See https://goo.gl/3ane6s and https://goo.gl/z0Z9gn
// for more detailed overviews of style invalidation.
class CORE_EXPORT StyleInvalidator {
  STACK_ALLOCATED();

 public:
  StyleInvalidator(PendingInvalidationMap&);

  ~StyleInvalidator();
  void Invalidate(Document& document, Element* invalidation_root);

 private:
  class SiblingData;

  void PushInvalidationSetsForContainerNode(ContainerNode&, SiblingData&);
  void PushInvalidationSet(const InvalidationSet&);
  bool WholeSubtreeInvalid() const {
    return invalidation_flags_.WholeSubtreeInvalid();
  }

  void Invalidate(Element&, SiblingData&);
  void InvalidateShadowRootChildren(Element&);
  void InvalidateChildren(Element&);
  void InvalidateSlotDistributedElements(HTMLSlotElement&) const;
  // Returns true if the element should be invalidated according to the
  // current state. This can also update the current state.
  bool CheckInvalidationSetsAgainstElement(Element&, SiblingData&);

  bool MatchesCurrentInvalidationSets(Element&) const;
  bool MatchesCurrentInvalidationSetsAsSlotted(Element&) const;
  bool MatchesCurrentInvalidationSetsAsParts(Element&) const;

  bool HasInvalidationSets() const {
    return !WholeSubtreeInvalid() &&
           (invalidation_sets_.size() || pending_nth_sets_.size());
  }

  void SetWholeSubtreeInvalid() {
    invalidation_flags_.SetWholeSubtreeInvalid(true);
  }

  bool TreeBoundaryCrossing() const {
    return invalidation_flags_.TreeBoundaryCrossing();
  }
  bool InsertionPointCrossing() const {
    return invalidation_flags_.InsertionPointCrossing();
  }
  bool InvalidatesSlotted() const {
    return invalidation_flags_.InvalidatesSlotted();
  }
  bool InvalidatesParts() const {
    return invalidation_flags_.InvalidatesParts();
  }

  void AddPendingNthSiblingInvalidationSet(
      const NthSiblingInvalidationSet& nth_set) {
    pending_nth_sets_.push_back(&nth_set);
  }
  void PushNthSiblingInvalidationSets(SiblingData& sibling_data) {
    for (const auto* invalidation_set : pending_nth_sets_) {
      sibling_data.PushInvalidationSet(*invalidation_set);
    }
    ClearPendingNthSiblingInvalidationSets();
  }
  void ClearPendingNthSiblingInvalidationSets() { pending_nth_sets_.resize(0); }

  PendingInvalidationMap& pending_invalidation_map_;
  using DescendantInvalidationSets = Vector<const InvalidationSet*, 16>;
  DescendantInvalidationSets invalidation_sets_;
  // NthSiblingInvalidationSets are added here from the parent node on which it
  // is scheduled, and pushed to SiblingData before invalidating the children.
  // See the NthSiblingInvalidationSet documentation.
  Vector<const NthSiblingInvalidationSet*> pending_nth_sets_;
  InvalidationFlags invalidation_flags_;

  class SiblingData {
    STACK_ALLOCATED();

   public:
    SiblingData() : element_index_(0) {}

    void PushInvalidationSet(const SiblingInvalidationSet&);
    bool MatchCurrentInvalidationSets(Element&, StyleInvalidator&);

    bool IsEmpty() const { return invalidation_entries_.empty(); }
    void Advance() { element_index_++; }

   private:
    struct Entry {
      DISALLOW_NEW();
      Entry(const SiblingInvalidationSet* invalidation_set,
            unsigned invalidation_limit)
          : invalidation_set_(invalidation_set),
            invalidation_limit_(invalidation_limit) {}

      const SiblingInvalidationSet* invalidation_set_;
      unsigned invalidation_limit_;
    };

    Vector<Entry, 16> invalidation_entries_;
    unsigned element_index_;
  };

  // Saves the state of a StyleInvalidator and automatically restores it when
  // this object is destroyed.
  class RecursionCheckpoint {
    STACK_ALLOCATED();

   public:
    RecursionCheckpoint(StyleInvalidator* invalidator)
        : prev_invalidation_sets_size_(invalidator->invalidation_sets_.size()),
          prev_invalidation_flags_(invalidator->invalidation_flags_),
          invalidator_(invalidator) {}
    ~RecursionCheckpoint() {
      invalidator_->invalidation_sets_.Shrink(prev_invalidation_sets_size_);
      invalidator_->invalidation_flags_ = prev_invalidation_flags_;
    }

   private:
    int prev_invalidation_sets_size_;
    InvalidationFlags prev_invalidation_flags_;
    StyleInvalidator* invalidator_;
  };
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_STYLE_INVALIDATOR_H_
