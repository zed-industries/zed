/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/layout_tree_builder_traversal.h"
#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/core/dom/shadow_root.h"
#include "third_party/blink/renderer/core/dom/traversal_range.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ContainerNode;
class Node;

// Flat tree version of |NodeTraversal|.
//
// None of member functions takes a |ShadowRoot| or an active insertion point,
// e.g. roughly speaking <content> and <shadow> in the shadow tree, see
// |InsertionPoint::isActive()| for details of active insertion points, since
// they aren't appeared in the flat tree. |assertPrecondition()| and
// |assertPostCondition()| check this condition.
class CORE_EXPORT FlatTreeTraversal {
  STATIC_ONLY(FlatTreeTraversal);

 public:
  using TraversalNodeType = Node;

#if DCHECK_IS_ON()
  static void AssertFlatTreeNodeDataUpdated(
      const Node& root,
      int& assigned_nodes_in_slot_count,
      int& nodes_which_have_assigned_slot_count);
#endif

  static Node* Next(const Node&);
  static Node* Next(const Node&, const Node* stay_within);
  static Node* Previous(const Node&);
  // Returns the previous of |node| in preorder. When |stay_within| is given,
  // returns nullptr if the previous is not a descendant of |stay_within|.
  static Node* Previous(const Node& node, const Node* stay_within);

  static Node* FirstChild(const Node&);
  static Node* LastChild(const Node&);
  static bool HasChildren(const Node&);

  static ContainerNode* Parent(const Node&);
  static Element* ParentElement(const Node&);
  // Return the passed in Node if it is an Element, otherwise return the
  // ParentElement()
  static const Element* InclusiveParentElement(const Node&);

  static Node* NextSibling(const Node&);
  static Node* PreviousSibling(const Node&);

  // Returns a child node at |index|. If |index| is greater than or equal to
  // the children, this function returns |nullptr|.
  static Node* ChildAt(const Node&, unsigned index);

  // Flat tree version of |NodeTraversal::NextSkippingChildren()|. This
  // function is similar to |Next()| but skips the child nodes of the specified
  // node. E.g. for this tree:
  //        0
  //      /   \
  //     1     4
  //    / \   / \
  //   2   3 5   6
  // NextSkippingChildren(1) will return 4.
  // NextSkippingChildren(3) will return 4.
  // NextSkippingChildren(2) will return 3.
  // NextSkippingChildren(4) will return nullptr.
  // If you're looking for the "Previous" version of this method, see
  // PreviousAbsoluteSibling().
  static Node* NextSkippingChildren(const Node&);
  static Node* NextSkippingChildren(const Node&, const Node* stay_within);

  static Node* FirstWithin(const Node& current) { return FirstChild(current); }

  // Flat tree version of |NodeTraversal::PreviousAbsoluteSibling()|. This
  // function returns the previous direct sibling of the node, if there is one.
  // If not, it will traverse up the ancestor chain until it finds an ancestor
  // that has a previous sibling, returning that sibling. Or nullptr if none.
  // E.g. for this tree:
  //        0
  //      /   \
  //     1     4
  //    / \   / \
  //   2   3 5   6
  // PreviousAbsoluteSibling(5) will return 1.
  // PreviousAbsoluteSibling(4) will return 1.
  // PreviousAbsoluteSibling(6) will return 5.
  // PreviousAbsoluteSibling(2) will return nullptr.
  static Node* PreviousAbsoluteSibling(const Node&);

  // Like previous, but visits parents before their children.
  static Node* PreviousPostOrder(const Node&,
                                 const Node* stay_within = nullptr);

  // Flat tree version of |Node::isDescendantOf(other)|. This function
  // returns true if |other| contains |node|, otherwise returns
  // false. If |other| is |node|, this function returns false.
  static bool IsDescendantOf(const Node& /*node*/, const Node& other);

  static bool Contains(const ContainerNode& container, const Node& node) {
    AssertPrecondition(container);
    AssertPrecondition(node);
    return container == node || IsDescendantOf(node, container);
  }

  static bool ContainsIncludingPseudoElement(const ContainerNode&, const Node&);

  // Returns a common ancestor of |nodeA| and |nodeB| if exists, otherwise
  // returns |nullptr|.
  static Node* CommonAncestor(const Node& node_a, const Node& node_b);

  // Flat tree version of |Node::nodeIndex()|. This function returns a
  // zero base position number of the specified node in child nodes list, or
  // zero if the specified node has no parent.
  static unsigned Index(const Node&);

  // Flat tree version of |ContainerNode::countChildren()|. This function
  // returns the number of the child nodes of the specified node in the
  // flat tree.
  static unsigned CountChildren(const Node&);

  static Node* LastWithin(const Node&);
  static Node& LastWithinOrSelf(const Node&);

  // Flat tree range helper functions for range based for statement.
  static TraversalAncestorRange<FlatTreeTraversal> AncestorsOf(const Node&);
  static TraversalAncestorRange<FlatTreeTraversal> InclusiveAncestorsOf(
      const Node&);
  static TraversalSiblingRange<FlatTreeTraversal> ChildrenOf(const Node&);
  static TraversalDescendantRange<FlatTreeTraversal> DescendantsOf(const Node&);
  static TraversalInclusiveDescendantRange<FlatTreeTraversal>
  InclusiveDescendantsOf(const Node&);
  static TraversalNextRange<FlatTreeTraversal> StartsAt(const Node&);
  static TraversalNextRange<FlatTreeTraversal> StartsAfter(const Node&);

 private:
  enum TraversalDirection {
    kTraversalDirectionForward,
    kTraversalDirectionBackward
  };

  static void AssertPrecondition(const Node& node) {
    DCHECK(!node.GetDocument().IsFlatTreeTraversalForbidden());
    DCHECK(!node.IsShadowRoot())
        << "Shadow roots don't have layout objects. Their host has one, and "
           "their children have them, and those two are connected.";
  }

  static void AssertPostcondition(const Node* node) {
#if DCHECK_IS_ON()
    if (node)
      AssertPrecondition(*node);
#endif
  }

  static Node* ResolveDistributionStartingAt(const Node*, TraversalDirection);

  static Node* TraverseNext(const Node&);
  static Node* TraverseNext(const Node&, const Node* stay_within);
  static Node* TraverseNextSkippingChildren(const Node&,
                                            const Node* stay_within);
  static Node* TraversePrevious(const Node&);

  static Node* TraverseFirstChild(const Node&);
  static Node* TraverseLastChild(const Node&);
  static Node* TraverseChild(const Node&, TraversalDirection);

  static ContainerNode* TraverseParent(const Node&);

  static Node* TraverseNextSibling(const Node&);
  static Node* TraversePreviousSibling(const Node&);

  static Node* TraverseSiblings(const Node&, TraversalDirection);
  static Node* TraverseSiblingsForHostChild(const Node&, TraversalDirection);

  static Node* TraverseNextAncestorSibling(const Node&);
  static Node* TraversePreviousAncestorSibling(const Node&);
  static Node* PreviousAncestorSiblingPostOrder(const Node& current,
                                                const Node* stay_within);
};

inline ContainerNode* FlatTreeTraversal::Parent(const Node& node) {
  AssertPrecondition(node);
  ContainerNode* result = TraverseParent(node);
  AssertPostcondition(result);
  return result;
}

inline Element* FlatTreeTraversal::ParentElement(const Node& node) {
  return DynamicTo<Element>(FlatTreeTraversal::Parent(node));
}

inline Node* FlatTreeTraversal::NextSibling(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraverseSiblings(node, kTraversalDirectionForward);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::PreviousSibling(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraverseSiblings(node, kTraversalDirectionBackward);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::Next(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraverseNext(node);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::Next(const Node& node,
                                     const Node* stay_within) {
  AssertPrecondition(node);
  Node* result = TraverseNext(node, stay_within);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::NextSkippingChildren(const Node& node,
                                                     const Node* stay_within) {
  AssertPrecondition(node);
  Node* result = TraverseNextSkippingChildren(node, stay_within);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::TraverseNext(const Node& node) {
  if (Node* next = TraverseFirstChild(node))
    return next;
  for (const Node* next = &node; next; next = TraverseParent(*next)) {
    if (Node* sibling = TraverseNextSibling(*next))
      return sibling;
  }
  return nullptr;
}

inline Node* FlatTreeTraversal::TraverseNext(const Node& node,
                                             const Node* stay_within) {
  if (Node* next = TraverseFirstChild(node))
    return next;
  return TraverseNextSkippingChildren(node, stay_within);
}

inline Node* FlatTreeTraversal::TraverseNextSkippingChildren(
    const Node& node,
    const Node* stay_within) {
  for (const Node* next = &node; next; next = TraverseParent(*next)) {
    if (next == stay_within)
      return nullptr;
    if (Node* sibling = TraverseNextSibling(*next))
      return sibling;
  }
  return nullptr;
}

inline Node* FlatTreeTraversal::Previous(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraversePrevious(node);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::TraversePrevious(const Node& node) {
  if (Node* previous = TraversePreviousSibling(node)) {
    while (Node* child = TraverseLastChild(*previous))
      previous = child;
    return previous;
  }
  return TraverseParent(node);
}

inline Node* FlatTreeTraversal::Previous(const Node& node,
                                         const Node* stay_within) {
  if (!stay_within)
    return Previous(node);
  DCHECK(IsDescendantOf(node, *stay_within));
  Node* previous = Previous(node);
  if (previous == stay_within)
    return nullptr;
  return previous;
}

inline Node* FlatTreeTraversal::FirstChild(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraverseChild(node, kTraversalDirectionForward);
  AssertPostcondition(result);
  return result;
}

inline Node* FlatTreeTraversal::LastChild(const Node& node) {
  AssertPrecondition(node);
  Node* result = TraverseLastChild(node);
  AssertPostcondition(result);
  return result;
}

inline bool FlatTreeTraversal::HasChildren(const Node& node) {
  return FirstChild(node);
}

inline Node* FlatTreeTraversal::TraverseNextSibling(const Node& node) {
  return TraverseSiblings(node, kTraversalDirectionForward);
}

inline Node* FlatTreeTraversal::TraversePreviousSibling(const Node& node) {
  return TraverseSiblings(node, kTraversalDirectionBackward);
}

inline Node* FlatTreeTraversal::TraverseFirstChild(const Node& node) {
  return TraverseChild(node, kTraversalDirectionForward);
}

inline Node* FlatTreeTraversal::TraverseLastChild(const Node& node) {
  return TraverseChild(node, kTraversalDirectionBackward);
}

// TraverseRange<T> implementations
inline TraversalAncestorRange<FlatTreeTraversal> FlatTreeTraversal::AncestorsOf(
    const Node& node) {
  return TraversalAncestorRange<FlatTreeTraversal>(
      FlatTreeTraversal::Parent(node));
}

inline TraversalAncestorRange<FlatTreeTraversal>
FlatTreeTraversal::InclusiveAncestorsOf(const Node& node) {
  return TraversalAncestorRange<FlatTreeTraversal>(&node);
}

inline TraversalSiblingRange<FlatTreeTraversal> FlatTreeTraversal::ChildrenOf(
    const Node& parent) {
  return TraversalSiblingRange<FlatTreeTraversal>(
      FlatTreeTraversal::FirstChild(parent));
}

inline TraversalDescendantRange<FlatTreeTraversal>
FlatTreeTraversal::DescendantsOf(const Node& root) {
  return TraversalDescendantRange<FlatTreeTraversal>(&root);
}

inline TraversalInclusiveDescendantRange<FlatTreeTraversal>
FlatTreeTraversal::InclusiveDescendantsOf(const Node& root) {
  return TraversalInclusiveDescendantRange<FlatTreeTraversal>(&root);
}

inline TraversalNextRange<FlatTreeTraversal> FlatTreeTraversal::StartsAt(
    const Node& start) {
  return TraversalNextRange<FlatTreeTraversal>(&start);
}

inline TraversalNextRange<FlatTreeTraversal> FlatTreeTraversal::StartsAfter(
    const Node& start) {
  return TraversalNextRange<FlatTreeTraversal>(FlatTreeTraversal::Next(start));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_H_
