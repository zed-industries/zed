// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PeriodicSyncEventInit;

class MODULES_EXPORT PeriodicSyncEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PeriodicSyncEvent* Create(const AtomicString& type,
                                   const String& tag,
                                   WaitUntilObserver* observer) {
    return MakeGarbageCollected<PeriodicSyncEvent>(type, tag, observer);
  }
  static PeriodicSyncEvent* Create(const AtomicString& type,
                                   const PeriodicSyncEventInit* init) {
    return MakeGarbageCollected<PeriodicSyncEvent>(type, init);
  }

  PeriodicSyncEvent(const AtomicString& type,
                    const String& tag,
                    WaitUntilObserver* observer);
  PeriodicSyncEvent(const AtomicString& type,
                    const PeriodicSyncEventInit* init);
  ~PeriodicSyncEvent() override;

  const AtomicString& InterfaceName() const override;

  const String& tag() const;

 private:
  String tag_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_EVENT_H_
