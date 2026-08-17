// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_INCLUDING_TREE_ORDER_TRAVERSAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_INCLUDING_TREE_ORDER_TRAVERSAL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/shadow_root.h"
#include "third_party/blink/renderer/core/dom/traversal_range.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Does a traversal of the tree in "shadow-including tree order", see
// https://dom.spec.whatwg.org/#concept-shadow-including-tree-order for
// definition.
class CORE_EXPORT ShadowIncludingTreeOrderTraversal {
  STATIC_ONLY(ShadowIncludingTreeOrderTraversal);

 public:
  using TraversalNodeType = Node;

  static Node* Next(const Node& current, const Node* stay_within);
  static Node* NextSibling(const Node& node);
  static Node* FirstWithin(const Node& current);

  static TraversalSiblingRange<ShadowIncludingTreeOrderTraversal> ChildrenOf(
      const Node&);
  static TraversalDescendantRange<ShadowIncludingTreeOrderTraversal>
  DescendantsOf(const Node&);
  static TraversalInclusiveDescendantRange<ShadowIncludingTreeOrderTraversal>
  InclusiveDescendantsOf(const Node&);

 private:
  static Node* TraverseParent(const Node& current);
  static Node* TraverseNextSibling(const Node& current);
};

inline Node* ShadowIncludingTreeOrderTraversal::Next(const Node& current,
                                                     const Node* stay_within) {
  if (Node* first_child = FirstWithin(current))
    return first_child;
  for (const Node* node = &current; node && node != stay_within;
       node = TraverseParent(*node)) {
    if (Node* sibling = TraverseNextSibling(*node))
      return sibling;
  }
  return nullptr;
}

inline Node* ShadowIncludingTreeOrderTraversal::NextSibling(const Node& node) {
  return TraverseNextSibling(node);
}

inline Node* ShadowIncludingTreeOrderTraversal::FirstWithin(
    const Node& current) {
  if (ShadowRoot* shadow_root = current.GetShadowRoot())
    return shadow_root;
  return current.firstChild();
}

inline TraversalSiblingRange<ShadowIncludingTreeOrderTraversal>
ShadowIncludingTreeOrderTraversal::ChildrenOf(const Node& parent) {
  return TraversalSiblingRange<ShadowIncludingTreeOrderTraversal>(
      ShadowIncludingTreeOrderTraversal::FirstWithin(parent));
}

inline TraversalDescendantRange<ShadowIncludingTreeOrderTraversal>
ShadowIncludingTreeOrderTraversal::DescendantsOf(const Node& root) {
  return TraversalDescendantRange<ShadowIncludingTreeOrderTraversal>(&root);
}

inline TraversalInclusiveDescendantRange<ShadowIncludingTreeOrderTraversal>
ShadowIncludingTreeOrderTraversal::InclusiveDescendantsOf(const Node& root) {
  return TraversalInclusiveDescendantRange<ShadowIncludingTreeOrderTraversal>(
      &root);
}

inline Node* ShadowIncludingTreeOrderTraversal::TraverseParent(
    const Node& current) {
  return current.ParentOrShadowHostNode();
}

inline Node* ShadowIncludingTreeOrderTraversal::TraverseNextSibling(
    const Node& current) {
  if (Node* sibling = current.nextSibling())
    return sibling;
  if (current.IsShadowRoot())
    return current.ParentOrShadowHostNode()->firstChild();
  return nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_INCLUDING_TREE_ORDER_TRAVERSAL_H_
