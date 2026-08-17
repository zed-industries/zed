/*
 * Copyright (C) 2012 Google Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ID_TARGET_OBSERVER_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ID_TARGET_OBSERVER_REGISTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class IdTargetObserver;

class CORE_EXPORT IdTargetObserverRegistry final
    : public GarbageCollected<IdTargetObserverRegistry> {
  friend class IdTargetObserver;

 public:
  IdTargetObserverRegistry() : notifying_observers_in_set_(nullptr) {}
  IdTargetObserverRegistry(const IdTargetObserverRegistry&) = delete;
  IdTargetObserverRegistry& operator=(const IdTargetObserverRegistry&) = delete;

  void Trace(Visitor*) const;
  void NotifyObservers(const AtomicString& id);
  bool HasObservers(const AtomicString& id) const;

 private:
  void AddObserver(const AtomicString& id, IdTargetObserver*);
  void RemoveObserver(const AtomicString& id, IdTargetObserver*);
  void NotifyObserversInternal(const AtomicString& id);

  using ObserverSet = GCedHeapHashSet<Member<IdTargetObserver>>;
  using IdToObserverSetMap = HeapHashMap<StringImpl*, Member<ObserverSet>>;
  IdToObserverSetMap registry_;
  Member<ObserverSet> notifying_observers_in_set_;
};

inline void IdTargetObserverRegistry::NotifyObservers(const AtomicString& id) {
  DCHECK(!notifying_observers_in_set_);
  if (id.empty() || registry_.empty())
    return;
  IdTargetObserverRegistry::NotifyObserversInternal(id);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ID_TARGET_OBSERVER_REGISTRY_H_
