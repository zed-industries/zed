/*
 * Copyright (C) 2007 Alexey Proskuryakov <ap@webkit.org>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_NODE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_NODE_SET_H_

#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

namespace xpath {

class NodeSet final : public GarbageCollected<NodeSet> {
 public:
  static NodeSet* Create() { return MakeGarbageCollected<NodeSet>(); }
  static NodeSet* Create(const NodeSet&);

  NodeSet() : is_sorted_(true), subtrees_are_disjoint_(false) {}

  void Trace(Visitor* visitor) const { visitor->Trace(nodes_); }

  wtf_size_t size() const { return nodes_.size(); }
  bool IsEmpty() const { return !nodes_.size(); }
  Node* operator[](unsigned i) const { return nodes_.at(i).Get(); }
  HeapVector<Member<Node>>::iterator begin() { return nodes_.begin(); }
  HeapVector<Member<Node>>::iterator end() { return nodes_.end(); }
  HeapVector<Member<Node>>::const_iterator begin() const {
    return nodes_.begin();
  }
  HeapVector<Member<Node>>::const_iterator end() const { return nodes_.end(); }
  void ReserveCapacity(wtf_size_t new_capacity) {
    nodes_.reserve(new_capacity);
  }
  void clear() { nodes_.clear(); }
  void Swap(NodeSet& other) {
    std::swap(is_sorted_, other.is_sorted_);
    std::swap(subtrees_are_disjoint_, other.subtrees_are_disjoint_);
    nodes_.swap(other.nodes_);
  }

  // NodeSet itself does not verify that nodes in it are unique.
  void Append(Node* node) { nodes_.push_back(node); }
  void Append(const NodeSet& node_set) { nodes_.AppendVector(node_set.nodes_); }

  // Returns the set's first node in document order, or 0 if the set is empty.
  Node* FirstNode() const;

  // Returns 0 if the set is empty.
  Node* AnyNode() const;

  // NodeSet itself doesn't check if it contains nodes in document order - the
  // caller should tell it if it does not.
  void MarkSorted(bool is_sorted) { is_sorted_ = is_sorted; }
  bool IsSorted() const { return is_sorted_ || nodes_.size() < 2; }

  void Sort() const;

  // No node in the set is ancestor of another. Unlike m_isSorted, this is
  // assumed to be false, unless the caller sets it to true.
  void MarkSubtreesDisjoint(bool disjoint) {
    subtrees_are_disjoint_ = disjoint;
  }
  bool SubtreesAreDisjoint() const {
    return subtrees_are_disjoint_ || nodes_.size() < 2;
  }

  void Reverse();

 private:
  void TraversalSort() const;

  bool is_sorted_;
  bool subtrees_are_disjoint_;
  HeapVector<Member<Node>> nodes_;
};

}  // namespace xpath

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_NODE_SET_H_
