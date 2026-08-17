// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class SyncEventInit;

class MODULES_EXPORT SyncEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SyncEvent* Create(const AtomicString& type,
                           const String& tag,
                           bool last_chance,
                           WaitUntilObserver* observer) {
    return MakeGarbageCollected<SyncEvent>(type, tag, last_chance, observer);
  }
  static SyncEvent* Create(const AtomicString& type,
                           const SyncEventInit* init) {
    return MakeGarbageCollected<SyncEvent>(type, init);
  }

  SyncEvent(const AtomicString& type,
            const String& tag,
            bool last_chance,
            WaitUntilObserver* observer);
  SyncEvent(const AtomicString& type, const SyncEventInit* init);
  ~SyncEvent() override;

  const AtomicString& InterfaceName() const override;

  const String& tag() const;
  bool lastChance() const;

 private:
  String tag_;
  bool last_chance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_EVENT_H_
