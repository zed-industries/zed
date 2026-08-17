// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_ARIA_NOTIFICATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_ARIA_NOTIFICATION_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_aria_notification_options.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/accessibility/ax_enums.mojom-blink-forward.h"

namespace blink {

class AriaNotification {
  DISALLOW_NEW();

 public:
  using AriaNotificationInterrupt = ax::mojom::blink::AriaNotificationInterrupt;
  using AriaNotificationPriority = ax::mojom::blink::AriaNotificationPriority;

  AriaNotification(const String& announcement,
                   const AriaNotificationOptions* options);

  const String& Announcement() const { return announcement_; }
  AriaNotificationPriority Priority() const { return priority_; }
  AriaNotificationInterrupt Interrupt() const { return interrupt_; }
  const String& Type() const { return type_; }

 private:
  String announcement_;
  AriaNotificationPriority priority_;
  AriaNotificationInterrupt interrupt_;
  String type_;
};

class AriaNotifications {
  DISALLOW_NEW();

 public:
  using ConstIterator = Vector<AriaNotification>::const_iterator;

  AriaNotifications() = default;
  AriaNotifications(AriaNotifications&&) = default;
  AriaNotifications(const AriaNotifications&) = delete;
  AriaNotifications& operator=(AriaNotifications&&) = default;
  AriaNotifications& operator=(const AriaNotifications&) = delete;

  ConstIterator begin() const { return notifications_.begin(); }
  ConstIterator end() const { return notifications_.end(); }
  wtf_size_t Size() const { return notifications_.size(); }

  void Add(const String& announcement, const AriaNotificationOptions* options);

 private:
  Vector<AriaNotification> notifications_;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::AriaNotification)

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_ARIA_NOTIFICATION_H_
