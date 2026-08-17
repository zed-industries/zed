// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_STACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_STACK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/agent.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CustomElementReaction;
class CustomElementReactionQueue;
class Element;

// https://html.spec.whatwg.org/C/#custom-element-reactions
class CORE_EXPORT CustomElementReactionStack final
    : public GarbageCollected<CustomElementReactionStack>,
      public NameClient,
      public Supplement<Agent> {
 public:
  explicit CustomElementReactionStack(Agent& agent);
  CustomElementReactionStack(const CustomElementReactionStack&) = delete;
  CustomElementReactionStack& operator=(const CustomElementReactionStack&) =
      delete;
  ~CustomElementReactionStack() override = default;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "CustomElementReactionStack";
  }

  bool IsEmpty();
  void Push();
  void PopInvokingReactions();
  void EnqueueToCurrentQueue(Element&, CustomElementReaction&);
  void EnqueueToBackupQueue(Element&, CustomElementReaction&);
  void ClearQueue(Element&);

  static CustomElementReactionStack& From(Agent& agent);
  static const char kSupplementName[];

 private:
  friend class ResetCustomElementReactionStackForTest;

  using ElementReactionQueueMap =
      HeapHashMap<Member<Element>, Member<CustomElementReactionQueue>>;
  ElementReactionQueueMap map_;

  static CustomElementReactionStack* Swap(
      Agent& agent,
      CustomElementReactionStack* new_stack);

  using ElementQueue = GCedHeapVector<Member<Element>, 1>;
  HeapVector<Member<ElementQueue>> stack_;
  Member<ElementQueue> backup_queue_;

  void InvokeBackupQueue();
  void InvokeReactions(ElementQueue&);
  void Enqueue(Member<ElementQueue>&, Element&, CustomElementReaction&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_STACK_H_
