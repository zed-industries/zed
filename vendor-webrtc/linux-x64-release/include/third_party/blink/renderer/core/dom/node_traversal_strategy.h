// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_STRATEGY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_STRATEGY_H_

#include "third_party/blink/renderer/core/dom/node.h"

namespace blink {

// NodeTraversalStrategy is helpful to implement algorithm templates independent
// from tree traversal direction, and to instantiate them with specific
// direction.
//
// For example,
//
// template <Strategy> void IterateOverChildren(const Node& parent) {
//   for (Node* child = Strategy::StartNode(parent); child;
//       child = Strategy::NextNode(*child)) {
//     ...
//   }
// }
// IterateOverChildren<NextNodeTraversalStrategy>(parent) iterates forward,
// IterateOverChildren<PreviousNodeTraversalStrategy>(parent) iterates backward.

class NextNodeTraversalStrategy {
  STATIC_ONLY(NextNodeTraversalStrategy);

 public:
  static Node* StartNode(const Node& parent) { return parent.firstChild(); }
  static Node* NextNode(const Node& current) { return current.nextSibling(); }
};

class PreviousNodeTraversalStrategy {
  STATIC_ONLY(PreviousNodeTraversalStrategy);

 public:
  static Node* StartNode(const Node& parent) { return parent.lastChild(); }
  static Node* NextNode(const Node& current) {
    return current.previousSibling();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_TRAVERSAL_STRATEGY_H_
