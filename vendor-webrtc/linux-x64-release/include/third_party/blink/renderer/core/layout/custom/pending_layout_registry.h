// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_PENDING_LAYOUT_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_PENDING_LAYOUT_REGISTRY_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Node;

// This class keeps a reference to any Node which will need their layout tree
// re-attached when a custom layout is registered. This is primarily owned by
// the LayoutWorklet instance.
class PendingLayoutRegistry : public GarbageCollected<PendingLayoutRegistry> {
 public:
  PendingLayoutRegistry() = default;
  PendingLayoutRegistry(const PendingLayoutRegistry&) = delete;
  PendingLayoutRegistry& operator=(const PendingLayoutRegistry&) = delete;

  void NotifyLayoutReady(const AtomicString& name);
  void AddPendingLayout(const AtomicString& name, Node*);

  void Trace(Visitor*) const;

 private:
  // This is a map of Nodes which are waiting for a CSSLayoutDefinition to be
  // registered. We keep a reference to the Node instead of the LayoutObject as
  // Nodes are garbage-collected and we can keep a weak reference to it. (With
  // a LayoutObject we'd have to respect the life-cycle of it which would be
  // error-prone and complicated).
  //
  // NOTE: By the time a layout has been registered, the node may have a
  // different "display" value.
  using PendingSet = GCedHeapHashSet<WeakMember<Node>>;
  using PendingLayoutMap = HeapHashMap<AtomicString, Member<PendingSet>>;
  PendingLayoutMap pending_layouts_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_PENDING_LAYOUT_REGISTRY_H_
