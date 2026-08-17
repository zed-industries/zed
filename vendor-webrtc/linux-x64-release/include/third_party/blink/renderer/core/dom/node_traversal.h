/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/traversal_range.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT NodeTraversal {
  STATIC_ONLY(NodeTraversal);

 public:
  using TraversalNodeType = Node;

  // Does a pre-order traversal of the tree to find the next node after this
  // one.  This uses the same order that tags appear in the source file. If the
  // stayWithin argument is non-null, the traversal will stop once the specified
  // node is reached.  This can be used to restrict traversal to a particular
  // sub-tree.
  static Node* Next(const Node& current) {
    return TraverseNextTemplate(current);
  }
  static Node* Next(const ContainerNode& current) {
    return TraverseNextTemplate(current);
  }
  static Node* Next(const Node& current, const Node* stay_within) {
    return TraverseNextTemplate(current, stay_within);
  }
  static Node* Next(const ContainerNode& current, const Node* stay_within) {
    return TraverseNextTemplate(current, stay_within);
  }

  // Like next, but skips children and starts with the next sibling. If you're
  // looking for the "Previous" version of this method, see
  // PreviousAbsoluteSibling().
  static Node* NextSkippingChildren(const Node&);
  static Node* NextSkippingChildren(const Node&, const Node* stay_within);

  static Node* FirstWithin(const Node& current) { return current.firstChild(); }

  static Node* LastWithin(const ContainerNode&);
  static Node& LastWithinOrSelf(Node&);

  // Does a reverse pre-order traversal to find the node that comes before the
  // current one in document order
  static Node* Previous(const Node&, const Node* stay_within = nullptr);

  // Returns the previous direct sibling of the node, if there is one. If not,
  // it will traverse up the ancestor chain until it finds an ancestor
  // that has a previous sibling, returning that sibling. Or nullptr if none.
  // See comment for |FlatTreeTraversal::PreviousAbsoluteSibling| for details.
  static Node* PreviousAbsoluteSibling(const Node&,
                                       const Node* stay_within = nullptr);

  // Like next, but visits parents after their children.
  static Node* NextPostOrder(const Node&, const Node* stay_within = nullptr);

  // Like previous, but visits parents before their children.
  static Node* PreviousPostOrder(const Node&,
                                 const Node* stay_within = nullptr);

  // Pre-order traversal including the pseudo-elements.
  static Node* PreviousIncludingPseudo(const Node&,
                                       const Node* stay_within = nullptr);
  static Node* NextIncludingPseudo(const Node&,
                                   const Node* stay_within = nullptr);
  // See comment for |FlatTreeTraversal::PreviousAbsoluteSibling| for details.
  static Node* PreviousAbsoluteSiblingIncludingPseudo(
      const Node&,
      const Node* stay_within = nullptr);
  static Node* NextIncludingPseudoSkippingChildren(
      const Node&,
      const Node* stay_within = nullptr);

  static Node* NextAncestorSibling(const Node&);
  static Node* NextAncestorSibling(const Node&, const Node* stay_within);
  static Node& HighestAncestorOrSelf(const Node&);

  // Children traversal.
  static Node* ChildAt(const Node& parent, unsigned index) {
    return ChildAtTemplate(parent, index);
  }
  static Node* ChildAt(const ContainerNode& parent, unsigned index) {
    return ChildAtTemplate(parent, index);
  }

  // These functions are provided for matching with |FlatTreeTraversal|.
  static bool HasChildren(const Node& parent) { return FirstChild(parent); }
  static bool IsDescendantOf(const Node& node, const Node& other) {
    return node.IsDescendantOf(&other);
  }
  static Node* FirstChild(const Node& parent) { return parent.firstChild(); }
  static Node* LastChild(const Node& parent) { return parent.lastChild(); }
  static Node* NextSibling(const Node& node) { return node.nextSibling(); }
  static Node* PreviousSibling(const Node& node) {
    return node.previousSibling();
  }
  static Node* PreviousAncestorSibling(const Node&, const Node* stay_within);
  static ContainerNode* Parent(const Node& node) { return node.parentNode(); }
  static Node* CommonAncestor(const Node& node_a, const Node& node_b);
  static unsigned Index(const Node& node) { return node.NodeIndex(); }
  static unsigned CountChildren(const Node& parent) {
    return parent.CountChildren();
  }
  static ContainerNode* ParentOrShadowHostNode(const Node& node) {
    return node.ParentOrShadowHostNode();
  }

  static TraversalAncestorRange<NodeTraversal> AncestorsOf(const Node&);
  static TraversalAncestorRange<NodeTraversal> InclusiveAncestorsOf(
      const Node&);
  static TraversalSiblingRange<NodeTraversal> ChildrenOf(const Node&);
  static TraversalDescendantRange<NodeTraversal> DescendantsOf(const Node&);
  static TraversalInclusiveDescendantRange<NodeTraversal>
  InclusiveDescendantsOf(const Node&);
  static TraversalNextRange<NodeTraversal> StartsAt(const Node&);
  static TraversalNextRange<NodeTraversal> StartsAfter(const Node&);

 private:
  template <class NodeType>
  static Node* TraverseNextTemplate(NodeType&);
  template <class NodeType>
  static Node* TraverseNextTemplate(NodeType&, const Node* stay_within);
  template <class NodeType>
  static Node* ChildAtTemplate(NodeType&, unsigned);
  static Node* PreviousAncestorSiblingPostOrder(const Node& current,
                                                const Node* stay_within);
};

inline TraversalAncestorRange<NodeTraversal> NodeTraversal::AncestorsOf(
    const Node& node) {
  return TraversalAncestorRange<NodeTraversal>(NodeTraversal::Parent(node));
}

inline TraversalAncestorRange<NodeTraversal>
NodeTraversal::InclusiveAncestorsOf(const Node& node) {
  return TraversalAncestorRange<NodeTraversal>(&node);
}

inline TraversalSiblingRange<NodeTraversal> NodeTraversal::ChildrenOf(
    const Node& parent) {
  return TraversalSiblingRange<NodeTraversal>(
      NodeTraversal::FirstChild(parent));
}

inline TraversalDescendantRange<NodeTraversal> NodeTraversal::DescendantsOf(
    const Node& root) {
  return TraversalDescendantRange<NodeTraversal>(&root);
}

inline TraversalInclusiveDescendantRange<NodeTraversal>
NodeTraversal::InclusiveDescendantsOf(const Node& root) {
  return TraversalInclusiveDescendantRange<NodeTraversal>(&root);
}

inline TraversalNextRange<NodeTraversal> NodeTraversal::StartsAt(
    const Node& start) {
  return TraversalNextRange<NodeTraversal>(&start);
}

inline TraversalNextRange<NodeTraversal> NodeTraversal::StartsAfter(
    const Node& start) {
  return TraversalNextRange<NodeTraversal>(NodeTraversal::Next(start));
}

template <class NodeType>
inline Node* NodeTraversal::TraverseNextTemplate(NodeType& current) {
  if (current.hasChildren())
    return current.firstChild();
  if (current.HasNextSibling()) {
    return current.nextSibling();
  }
  return NextAncestorSibling(current);
}

template <class NodeType>
inline Node* NodeTraversal::TraverseNextTemplate(NodeType& current,
                                                 const Node* stay_within) {
  if (current.hasChildren())
    return current.firstChild();
  if (current == stay_within)
    return nullptr;
  if (current.HasNextSibling()) {
    return current.nextSibling();
  }
  return NextAncestorSibling(current, stay_within);
}

inline Node* NodeTraversal::NextSkippingChildren(const Node& current) {
  if (current.HasNextSibling()) {
    return current.nextSibling();
  }
  return NextAncestorSibling(current);
}

inline Node* NodeTraversal::NextSkippingChildren(const Node& current,
                                                 const Node* stay_within) {
  if (current == stay_within)
    return nullptr;
  if (current.HasNextSibling()) {
    return current.nextSibling();
  }
  return NextAncestorSibling(current, stay_within);
}

// Note that `HighestAncestorOrSelf` is used most commonly in `RemovedFrom` and
// `InsertedInfo`, during which `current.isConnected()` hasn't yet been
// updated to its new state. Which means `HighestAncestorOrSelf` cannot use
// `current.TreeRoot()` because it might return the root of the old tree,
// rather than the highest ancestor of the newly-removed/inserted node.
inline Node& NodeTraversal::HighestAncestorOrSelf(const Node& current) {
  Node* highest = const_cast<Node*>(&current);
  while (highest->parentNode()) {
    highest = highest->parentNode();
  }
  return *highest;
}

template <class NodeType>
inline Node* NodeTraversal::ChildAtTemplate(NodeType& parent, unsigned index) {
  Node* child = parent.firstChild();
  while (child && index--)
    child = child->nextSibling();
  return child;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_H_
