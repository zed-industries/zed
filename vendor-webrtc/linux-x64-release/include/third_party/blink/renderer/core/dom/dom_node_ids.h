// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_NODE_IDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_NODE_IDS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
class Node;

class CORE_EXPORT DOMNodeIds {
  STATIC_ONLY(DOMNodeIds);

 public:
  // Return a DOMNodeID or 0 if one hasn't been assigned.
  static DOMNodeId ExistingIdForNode(Node*);

  // Return a DOMNodeID or 0 if one hasn't been assigned.
  static DOMNodeId ExistingIdForNode(const Node*);

  // Return the existing DOMNodeID if it has already been assigned, otherwise,
  // assign a new DOMNodeID and return that.
  static DOMNodeId IdForNode(Node*);

  // Return a node for the DOMNodeID or null if one hasn't been assigned.
  static Node* NodeForId(DOMNodeId);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_NODE_IDS_H_
