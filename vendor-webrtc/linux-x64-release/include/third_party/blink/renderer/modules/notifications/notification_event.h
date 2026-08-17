// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/notifications/notification.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class NotificationEventInit;

class MODULES_EXPORT NotificationEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static NotificationEvent* Create(const AtomicString& type,
                                   const NotificationEventInit* initializer) {
    return MakeGarbageCollected<NotificationEvent>(type, initializer);
  }
  static NotificationEvent* Create(const AtomicString& type,
                                   const NotificationEventInit* initializer,
                                   WaitUntilObserver* observer) {
    return MakeGarbageCollected<NotificationEvent>(type, initializer, observer);
  }

  NotificationEvent(const AtomicString& type,
                    const NotificationEventInit* initializer);
  NotificationEvent(const AtomicString& type,
                    const NotificationEventInit* initializer,
                    WaitUntilObserver* observer);
  ~NotificationEvent() override;

  Notification* getNotification() const { return notification_.Get(); }
  String action() const { return action_; }
  String reply() const { return reply_; }

  // ExtendableEvent interface.
  const AtomicString& InterfaceName() const override;

  void Trace(Visitor* visitor) const override;

 private:
  Member<Notification> notification_;
  String action_;
  String reply_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_EVENT_H_
