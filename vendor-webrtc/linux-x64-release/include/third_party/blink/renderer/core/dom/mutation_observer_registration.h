/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_REGISTRATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_REGISTRATION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/mutation_observer.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class QualifiedName;

class CORE_EXPORT MutationObserverRegistration final
    : public GarbageCollected<MutationObserverRegistration>,
      public NameClient {
 public:
  MutationObserverRegistration(MutationObserver&,
                               Node*,
                               MutationObserverOptions,
                               const HashSet<AtomicString>& attribute_filter);
  ~MutationObserverRegistration() override;

  void ResetObservation(MutationObserverOptions,
                        const HashSet<AtomicString>& attribute_filter);
  void ObservedSubtreeNodeWillDetach(Node&);
  void ClearTransientRegistrations();
  bool HasTransientRegistrations() const {
    return transient_registration_nodes_ &&
           !transient_registration_nodes_->empty();
  }
  void Unregister();

  bool ShouldReceiveMutationFrom(Node&,
                                 MutationType,
                                 const QualifiedName* attribute_name) const;
  bool IsSubtree() const { return options_ & MutationObserver::kSubtree; }

  MutationObserver& Observer() const { return *observer_; }
  MutationRecordDeliveryOptions DeliveryOptions() const {
    return options_ & (MutationObserver::kAttributeOldValue |
                       MutationObserver::kCharacterDataOldValue);
  }
  MutationType MutationTypes() const {
    return static_cast<MutationType>(options_ & kMutationTypeAll);
  }

  void AddRegistrationNodesToSet(HeapHashSet<Member<Node>>&) const;

  void Dispose();

  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "MutationObserverRegistration";
  }

 private:
  Member<MutationObserver> observer_;
  WeakMember<Node> registration_node_;
  Member<Node> registration_node_keep_alive_;
  using NodeHashSet = GCedHeapHashSet<Member<Node>>;
  Member<NodeHashSet> transient_registration_nodes_;

  MutationObserverOptions options_;
  HashSet<AtomicString> attribute_filter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_REGISTRATION_H_
