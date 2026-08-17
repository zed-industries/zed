// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_FLAGS_H_

namespace blink {

// Flags that are passed through multiple layers of the style invalidation
// process. This is just a struct with some good behaviour.
class InvalidationFlags {
 public:
  bool operator==(const InvalidationFlags&) const;
  bool operator!=(const InvalidationFlags& o) const { return !(*this == o); }

  // Merges two sets of flags together by orring all fields.
  void Merge(const InvalidationFlags& other);

  bool InvalidateCustomPseudo() const { return invalidate_custom_pseudo_; }
  void SetInvalidateCustomPseudo(bool value) {
    invalidate_custom_pseudo_ = value;
  }

  bool WholeSubtreeInvalid() const { return whole_subtree_invalid_; }
  void SetWholeSubtreeInvalid(bool value) { whole_subtree_invalid_ = value; }

  bool TreeBoundaryCrossing() const { return tree_boundary_crossing_; }
  void SetTreeBoundaryCrossing(bool value) { tree_boundary_crossing_ = value; }

  bool InsertionPointCrossing() const { return insertion_point_crossing_; }
  void SetInsertionPointCrossing(bool value) {
    insertion_point_crossing_ = value;
  }

  bool InvalidatesSlotted() const { return invalidates_slotted_; }
  void SetInvalidatesSlotted(bool value) { invalidates_slotted_ = value; }

  bool InvalidatesParts() const { return invalidates_parts_; }
  void SetInvalidatesParts(bool value) { invalidates_parts_ = value; }

  bool InvalidatesTreeCounting() const { return invalidates_tree_counting_; }
  void SetInvalidatesTreeCounting(bool value) {
    invalidates_tree_counting_ = value;
  }

 private:
  // If true, all descendants which are custom pseudo elements must be
  // invalidated.
  bool invalidate_custom_pseudo_ : 1 = false;
  // If true, all descendants might be invalidated, so a full subtree recalc is
  // required.
  bool whole_subtree_invalid_ : 1 = false;
  // If true, the invalidation must traverse into ShadowRoots with this set.
  bool tree_boundary_crossing_ : 1 = false;
  // If true, insertion point descendants must be invalidated.
  bool insertion_point_crossing_ : 1 = false;
  // If true, distributed nodes of <slot> elements need to be invalidated.
  bool invalidates_slotted_ : 1 = false;
  // If true, parts inside this node's shadow tree need to be invalidated.
  bool invalidates_parts_ : 1 = false;
  // If true, elements whose style depends on tree-counting functions need to be
  // invalidated.
  bool invalidates_tree_counting_ : 1 = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_FLAGS_H_
