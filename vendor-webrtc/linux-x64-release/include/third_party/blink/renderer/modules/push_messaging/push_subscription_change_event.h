// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_CHANGE_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/push_messaging/push_subscription.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PushSubscriptionChangeEventInit;

class MODULES_EXPORT PushSubscriptionChangeEvent final
    : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PushSubscriptionChangeEvent* Create(const AtomicString& type,
                                             PushSubscription* new_subscription,
                                             PushSubscription* old_subscription,
                                             WaitUntilObserver* observer) {
    return MakeGarbageCollected<PushSubscriptionChangeEvent>(
        type, new_subscription, old_subscription, observer);
  }
  static PushSubscriptionChangeEvent* Create(
      const AtomicString& type,
      PushSubscriptionChangeEventInit* initializer) {
    return MakeGarbageCollected<PushSubscriptionChangeEvent>(type, initializer);
  }

  PushSubscriptionChangeEvent(const AtomicString& type,
                              PushSubscription* new_subscription,
                              PushSubscription* old_subscription,
                              WaitUntilObserver* observer);
  PushSubscriptionChangeEvent(const AtomicString& type,
                              PushSubscriptionChangeEventInit* initializer);
  ~PushSubscriptionChangeEvent() override;

  PushSubscription* newSubscription() const;
  PushSubscription* oldSubscription() const;

  void Trace(Visitor* visitor) const override;

 private:
  Member<PushSubscription> new_subscription_;
  Member<PushSubscription> old_subscription_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_CHANGE_EVENT_H_
