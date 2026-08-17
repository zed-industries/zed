/*
 * Copyright (C) 2015 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SLOT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SLOT_ELEMENT_H_

#include "base/check_op.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_element_text.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"

namespace blink {

class AssignedNodesOptions;

class CORE_EXPORT HTMLSlotElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HTMLSlotElement(Document&);

  const HeapVector<Member<Node>>& AssignedNodes() const;
  const HeapVector<Member<Node>> AssignedNodesForBinding(
      const AssignedNodesOptions*);
  const HeapVector<Member<Element>> AssignedElements();
  const HeapVector<Member<Element>> AssignedElementsForBinding(
      const AssignedNodesOptions*);

  Node* FirstAssignedNode() const {
    auto& nodes = AssignedNodes();
    return nodes.empty() ? nullptr : nodes.front().Get();
  }
  Node* LastAssignedNode() const {
    auto& nodes = AssignedNodes();
    return nodes.empty() ? nullptr : nodes.back().Get();
  }

  Node* AssignedNodeNextTo(const Node&) const;
  Node* AssignedNodePreviousTo(const Node&) const;

  void AppendAssignedNode(Node&);
  void ClearAssignedNodes();

  const HeapVector<Member<Node>> FlattenedAssignedNodes();

  void WillRecalcAssignedNodes() { ClearAssignedNodes(); }
  void DidRecalcAssignedNodes(bool display_locked_subtree) {
    UpdateFlatTreeNodeDataForAssignedNodes();
    RecalcFlatTreeChildren();
    if (display_locked_subtree)
      DetachDisplayLockedAssignedNodesLayoutTreeIfNeeded();
  }

  void AttachLayoutTreeForSlotChildren(AttachContext&);
  void DetachLayoutTree(bool performing_reattach) final;
  void RebuildDistributedChildrenLayoutTrees(WhitespaceAttacher&);

  void AttributeChanged(const AttributeModificationParams&) final;

  AtomicString GetName() const;

  // This method can be slow because this has to traverse the children of a
  // shadow host.  This method should be used only when |assigned_nodes_| is
  // dirty.  e.g. To detect a slotchange event in DOM mutations.
  bool HasAssignedNodesSlow() const;

  // Returns true if the slot has assigned nodes, without doing assignment
  // recalc. Used by FlatTreeParentForChildDirty() which needs to avoid doing
  // slot assignments while marking the tree style-dirty.
  bool HasAssignedNodesNoRecalc() const { return !assigned_nodes_.empty(); }

  bool SupportsAssignment() const { return IsInShadowTree(); }

  void CheckFallbackAfterInsertedIntoShadowTree();
  void CheckFallbackAfterRemovedFromShadowTree();

  void DidSlotChange(SlotChangeType);
  void DidSlotChangeAfterRemovedFromShadowTree();
  void DidSlotChangeAfterRenaming();
  void DispatchSlotChangeEvent();
  void ClearSlotChangeEventEnqueued() { slotchange_event_enqueued_ = false; }

  static AtomicString NormalizeSlotName(const AtomicString&);

  void RecalcStyleForSlotChildren(const StyleRecalcChange,
                                  const StyleRecalcContext&);

  // For User-Agent Shadow DOM
  static const AtomicString& UserAgentCustomAssignSlotName();
  static const AtomicString& UserAgentDefaultSlotName();

  // For imperative Shadow DOM distribution APIs
  // IDL assign() implementation.
  void assign(HeapVector<Member<V8UnionElementOrText>>& nodes, ExceptionState&);
  // assign() c++ implementation.
  void Assign(const HeapVector<Member<Node>>& nodes);
  void Assign(Node* node);

  const HeapLinkedHashSet<WeakMember<Node>>& ManuallyAssignedNodes() const {
    return manually_assigned_nodes_;
  }
  void RemoveManuallyAssignedNode(Node&);

  // Override in order to defer this work when needed.
  bool CalculateAndAdjustAutoDirectionality() final;

  void Trace(Visitor*) const override;

 private:
  InsertionNotificationRequest InsertedInto(ContainerNode&) final;
  void RemovedFrom(ContainerNode&) final;

  void ChildrenChanged(const ChildrenChange&) override;

  void EnqueueSlotChangeEvent();

  bool HasSlotableChild() const;

  void NotifySlottedNodesOfFlatTreeChange(
      const HeapVector<Member<Node>>& old_slotted,
      const HeapVector<Member<Node>>& new_slotted);
  static void NotifySlottedNodesOfFlatTreeChangeNaive(
      const HeapVector<Member<Node>>& old_assigned_nodes,
      const HeapVector<Member<Node>>& new_assigned_nodes);
  static void NotifySlottedNodesOfFlatTreeChangeByDynamicProgramming(
      const HeapVector<Member<Node>>& old_slotted,
      const HeapVector<Member<Node>>& new_slotted);

  void SetShadowRootNeedsAssignmentRecalc();
  bool CheckNodesValidity(HeapVector<Member<Node>> nodes, ExceptionState&);

  void RecalcFlatTreeChildren();
  void UpdateFlatTreeNodeDataForAssignedNodes();
  void ClearAssignedNodesAndFlatTreeChildren();
  void DetachDisplayLockedAssignedNodesLayoutTreeIfNeeded();

  void UpdateDirAutoAncestorsForSupportsAssignmentChange();

  HeapVector<Member<Node>> assigned_nodes_;
  HeapVector<Member<Node>> flat_tree_children_;

  bool slotchange_event_enqueued_ = false;

  // Imperative Shadow DOM distribution API.
  HeapLinkedHashSet<WeakMember<Node>> manually_assigned_nodes_;

  template <typename T, wtf_size_t S>
  struct LCSArray {
    LCSArray() : values(S) {}
    T& operator[](wtf_size_t i) { return values[i]; }
    wtf_size_t size() { return values.size(); }
    Vector<T, S> values;
  };

  // TODO(hayato): Move this to more appropriate directory (e.g. platform/wtf)
  // if there are more than one usages.
  template <typename Container, typename LCSTable, typename BacktrackTable>
  static void FillLongestCommonSubsequenceDynamicProgrammingTable(
      const Container& seq1,
      const Container& seq2,
      LCSTable& lcs_table,
      BacktrackTable& backtrack_table) {
    const wtf_size_t rows = base::checked_cast<wtf_size_t>(seq1.size());
    const wtf_size_t columns = base::checked_cast<wtf_size_t>(seq2.size());

    DCHECK_GT(lcs_table.size(), rows);
    DCHECK_GT(lcs_table[0].size(), columns);
    DCHECK_GT(backtrack_table.size(), rows);
    DCHECK_GT(backtrack_table[0].size(), columns);

    for (wtf_size_t r = 0; r <= rows; ++r)
      lcs_table[r][0] = 0;
    for (wtf_size_t c = 0; c <= columns; ++c)
      lcs_table[0][c] = 0;

    for (wtf_size_t r = 1; r <= rows; ++r) {
      for (wtf_size_t c = 1; c <= columns; ++c) {
        if (seq1[r - 1] == seq2[c - 1]) {
          lcs_table[r][c] = lcs_table[r - 1][c - 1] + 1;
          backtrack_table[r][c] = std::make_pair(r - 1, c - 1);
        } else if (lcs_table[r - 1][c] > lcs_table[r][c - 1]) {
          lcs_table[r][c] = lcs_table[r - 1][c];
          backtrack_table[r][c] = std::make_pair(r - 1, c);
        } else {
          lcs_table[r][c] = lcs_table[r][c - 1];
          backtrack_table[r][c] = std::make_pair(r, c - 1);
        }
      }
    }
  }

  friend class HTMLSlotElementTest;
  friend class HTMLSlotElementInDocumentTest;
};

inline const HTMLSlotElement* ToHTMLSlotElementIfSupportsAssignmentOrNull(
    const Node& node) {
  if (auto* slot = DynamicTo<HTMLSlotElement>(node)) {
    if (slot->SupportsAssignment())
      return slot;
  }
  return nullptr;
}

inline HTMLSlotElement* ToHTMLSlotElementIfSupportsAssignmentOrNull(
    Node& node) {
  return const_cast<HTMLSlotElement*>(
      ToHTMLSlotElementIfSupportsAssignmentOrNull(
          static_cast<const Node&>(node)));
}

inline const HTMLSlotElement* ToHTMLSlotElementIfSupportsAssignmentOrNull(
    const Node* node) {
  if (!node)
    return nullptr;
  return ToHTMLSlotElementIfSupportsAssignmentOrNull(*node);
}

inline HTMLSlotElement* ToHTMLSlotElementIfSupportsAssignmentOrNull(
    Node* node) {
  return const_cast<HTMLSlotElement*>(
      ToHTMLSlotElementIfSupportsAssignmentOrNull(
          static_cast<const Node*>(node)));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SLOT_ELEMENT_H_
