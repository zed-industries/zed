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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_INTEREST_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_INTEREST_GROUP_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/mutation_observer.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class MutationObserverInterestGroup final
    : public GarbageCollected<MutationObserverInterestGroup> {
 public:
  static MutationObserverInterestGroup* CreateForChildListMutation(
      Node& target) {
    if (!target.GetDocument().HasMutationObserversOfType(
            kMutationTypeChildList))
      return nullptr;

    MutationRecordDeliveryOptions old_value_flag = 0;
    return CreateIfNeeded(target, kMutationTypeChildList, old_value_flag);
  }

  static MutationObserverInterestGroup* CreateForCharacterDataMutation(
      Node& target) {
    if (!target.GetDocument().HasMutationObserversOfType(
            kMutationTypeCharacterData))
      return nullptr;

    return CreateIfNeeded(target, kMutationTypeCharacterData,
                          MutationObserver::kCharacterDataOldValue);
  }

  static MutationObserverInterestGroup* CreateForAttributesMutation(
      Node& target,
      const QualifiedName& attribute_name) {
    if (!target.GetDocument().HasMutationObserversOfType(
            kMutationTypeAttributes))
      return nullptr;

    return CreateIfNeeded(target, kMutationTypeAttributes,
                          MutationObserver::kAttributeOldValue,
                          &attribute_name);
  }

  MutationObserverInterestGroup(
      HeapHashMap<Member<MutationObserver>, MutationRecordDeliveryOptions>&
          observers,
      MutationRecordDeliveryOptions old_value_flag);

  bool IsOldValueRequested();
  void EnqueueMutationRecord(MutationRecord*);

  void Trace(Visitor*) const;

 private:
  static MutationObserverInterestGroup* CreateIfNeeded(
      Node& target,
      MutationType,
      MutationRecordDeliveryOptions old_value_flag,
      const QualifiedName* attribute_name = nullptr);

  bool HasOldValue(MutationRecordDeliveryOptions options) {
    return options & old_value_flag_;
  }

  HeapHashMap<Member<MutationObserver>, MutationRecordDeliveryOptions>
      observers_;
  MutationRecordDeliveryOptions old_value_flag_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_INTEREST_GROUP_H_
