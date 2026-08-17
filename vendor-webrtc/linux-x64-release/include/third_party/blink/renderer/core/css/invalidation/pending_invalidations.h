// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_PENDING_INVALIDATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_PENDING_INVALIDATIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/invalidation/node_invalidation_sets.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ContainerNode;
class Document;
class Element;

using PendingInvalidationMap =
    HeapHashMap<Member<ContainerNode>, NodeInvalidationSets>;

// Performs deferred style invalidation for DOM subtrees.
//
// Suppose we have a large DOM tree with the style rules
// .a .b { ... }
// ...
// and user script adds or removes class 'a' from an element.
//
// The cached computed styles for any of the element's
// descendants that have class b are now outdated.
//
// The user script might subsequently make many more DOM
// changes, so we don't immediately traverse the element's
// descendants for class b.
//
// Instead, we record the need for this traversal by
// calling ScheduleInvalidationSetsForNode with
// InvalidationLists obtained from RuleFeatureSet.
//
// When we next read computed styles, for example from
// user script or to render a frame,
// StyleInvalidator::Invalidate(Document&) is called to
// traverse the DOM and perform all the pending style
// invalidations.
//
// If an element is removed from the DOM tree, we call
// ClearInvalidation(ContainerNode&).
//
// When there are sibling rules and elements are added
// or removed from the tree, we call
// ScheduleSiblingInvalidationsAsDescendants for the
// potentially affected siblings.
//
// When there are pending invalidations for an element's
// siblings, and the element is being removed, we call
// RescheduleSiblingInvalidationsAsDescendants to
// reshedule the invalidations as descendant invalidations
// on the element's parent.
//
// See https://goo.gl/3ane6s and https://goo.gl/z0Z9gn
// for more detailed overviews of style invalidation.
// TODO: unify these documents into an .md file in the repo.

class CORE_EXPORT PendingInvalidations {
  DISALLOW_NEW();

 public:
  PendingInvalidations() = default;
  PendingInvalidations(const PendingInvalidations&) = delete;
  PendingInvalidations& operator=(const PendingInvalidations&) = delete;
  ~PendingInvalidations() {}
  // May immediately invalidate the node and/or add pending invalidation sets to
  // this node.
  void ScheduleInvalidationSetsForNode(const InvalidationLists&,
                                       ContainerNode&);
  void ScheduleSiblingInvalidationsAsDescendants(
      const InvalidationLists&,
      ContainerNode& scheduling_parent);
  void RescheduleSiblingInvalidationsAsDescendants(Element&);
  void ClearInvalidation(ContainerNode&);

  PendingInvalidationMap& GetPendingInvalidationMap() {
    return pending_invalidation_map_;
  }
  void Trace(Visitor* visitor) const {
    visitor->Trace(pending_invalidation_map_);
  }

 private:
  NodeInvalidationSets& EnsurePendingInvalidations(ContainerNode&);

  PendingInvalidationMap pending_invalidation_map_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_PENDING_INVALIDATIONS_H_
