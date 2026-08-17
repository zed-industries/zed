// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_ENGINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_ENGINE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ShadowRoot;

class CORE_EXPORT SlotAssignmentEngine final
    : public GarbageCollected<SlotAssignmentEngine> {
 public:
  explicit SlotAssignmentEngine();

  void AddShadowRootNeedingRecalc(ShadowRoot&);
  void RemoveShadowRootNeedingRecalc(ShadowRoot&);

  void Connected(ShadowRoot&);
  void Disconnected(ShadowRoot&);

  bool HasPendingSlotAssignmentRecalc() const {
    return !shadow_roots_needing_recalc_.empty();
  }

  void RecalcSlotAssignments();

  void Trace(Visitor*) const;

 private:
  HeapHashSet<WeakMember<ShadowRoot>> shadow_roots_needing_recalc_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_ENGINE_H_
