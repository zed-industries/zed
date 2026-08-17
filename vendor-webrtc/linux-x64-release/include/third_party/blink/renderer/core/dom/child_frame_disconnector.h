// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_FRAME_DISCONNECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_FRAME_DISCONNECTOR_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class HTMLFrameOwnerElement;
class Node;

class ChildFrameDisconnector {
  STACK_ALLOCATED();

 public:
  enum DisconnectPolicy { kRootAndDescendants, kDescendantsOnly };
  enum DisconnectReason { kDisconnectParent, kDisconnectSelf };

  explicit ChildFrameDisconnector(Node& root,
                                  DisconnectReason disconnect_reason)
      : root_(&root), disconnect_reason_(disconnect_reason) {}

  void Disconnect(DisconnectPolicy = kRootAndDescendants);

 private:
  void CollectFrameOwners(Node&);
  void DisconnectCollectedFrameOwners();
  Node& Root() const { return *root_; }

  HeapVector<Member<HTMLFrameOwnerElement>, 10> frame_owners_;
  Node* root_;
  DisconnectReason disconnect_reason_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_FRAME_DISCONNECTOR_H_
