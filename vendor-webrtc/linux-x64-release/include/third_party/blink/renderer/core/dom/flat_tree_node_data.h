// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_NODE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_NODE_DATA_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class HTMLSlotElement;

class FlatTreeNodeData final : public GarbageCollected<FlatTreeNodeData> {
 public:
  FlatTreeNodeData() {}
  FlatTreeNodeData(const FlatTreeNodeData&) = delete;
  FlatTreeNodeData& operator=(const FlatTreeNodeData&) = delete;
  void Clear() {
    assigned_slot_ = nullptr;
    previous_in_assigned_nodes_ = nullptr;
    next_in_assigned_nodes_ = nullptr;
  }

  void Trace(Visitor*) const;

#if DCHECK_IS_ON()
  bool IsCleared() const {
    return !assigned_slot_ && !previous_in_assigned_nodes_ &&
           !next_in_assigned_nodes_;
  }
#endif

 private:
  void SetAssignedSlot(HTMLSlotElement* assigned_slot) {
    assigned_slot_ = assigned_slot;
  }

  void SetPreviousInAssignedNodes(Node* previous) {
    previous_in_assigned_nodes_ = previous;
  }
  void SetNextInAssignedNodes(Node* next) { next_in_assigned_nodes_ = next; }

  void SetManuallyAssignedSlot(HTMLSlotElement* slot) {
    manually_assigned_slot_ = slot;
  }

  HTMLSlotElement* AssignedSlot() { return assigned_slot_.Get(); }
  Node* PreviousInAssignedNodes() { return previous_in_assigned_nodes_.Get(); }
  Node* NextInAssignedNodes() { return next_in_assigned_nodes_.Get(); }

  HTMLSlotElement* ManuallyAssignedSlot() const {
    return manually_assigned_slot_.Get();
  }

  friend class FlatTreeTraversal;
  friend class HTMLSlotElement;
  friend HTMLSlotElement* Node::AssignedSlot() const;
  friend HTMLSlotElement* Node::AssignedSlotWithoutRecalc() const;
  friend void Node::ClearFlatTreeNodeDataIfHostChanged(const ContainerNode&);
  friend void Node::SetManuallyAssignedSlot(HTMLSlotElement* slot);
  friend HTMLSlotElement* Node::ManuallyAssignedSlot();
  friend Element* Node::FlatTreeParentForChildDirty() const;

  WeakMember<HTMLSlotElement> assigned_slot_;
  WeakMember<Node> previous_in_assigned_nodes_;
  WeakMember<Node> next_in_assigned_nodes_;
  // Used by the imperative slot distribution API (not cleared by Clear()).
  WeakMember<HTMLSlotElement> manually_assigned_slot_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_NODE_DATA_H_
