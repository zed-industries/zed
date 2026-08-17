// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_PLATFORM_NOTIFICATION_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_PLATFORM_NOTIFICATION_DATA_H_

#include <optional>
#include <string>
#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/notifications/notification.mojom-forward.h"
#include "url/gurl.h"

namespace blink {

// Structure representing the information associated with a Web Notification.
// This struct should include the developer-visible information, kept
// synchronized with the WebNotificationData structure defined in the Blink API.
struct BLINK_COMMON_EXPORT PlatformNotificationData {
  PlatformNotificationData();
  PlatformNotificationData(const PlatformNotificationData& other);
  PlatformNotificationData& operator=(const PlatformNotificationData& other);
  ~PlatformNotificationData();

  // Title to be displayed with the Web Notification.
  std::u16string title;

  // Hint to determine the directionality of the displayed notification.
  mojom::NotificationDirection direction;

  // BCP 47 language tag describing the notification's contents. Optional.
  std::string lang;

  // Contents of the notification.
  std::u16string body;

  // Tag of the notification. Notifications sharing both their origin and their
  // tag will replace the first displayed notification.
  std::string tag;

  // URL of the image contents of the notification. May be empty if no url was
  // specified.
  GURL image;

  // URL of the icon which is to be displayed with the notification.
  GURL icon;

  // URL of the badge for representing the notification. May be empty if no url
  // was specified.
  GURL badge;

  // Vibration pattern for the notification, following the syntax of the
  // Vibration API. https://www.w3.org/TR/vibration/
  std::vector<int> vibration_pattern;

  // The time at which the event the notification represents took place.
  base::Time timestamp;

  // Whether default notification indicators (sound, vibration, light) should
  // be played again if the notification is replacing an older notification.
  bool renotify = false;

  // Whether default notification indicators (sound, vibration, light) should
  // be suppressed.
  bool silent = false;

  // Whether the notification should remain onscreen indefinitely, rather than
  // being auto-minimized to the notification center (if allowed by platform).
  bool require_interaction = false;

  // Developer-provided data associated with the notification, in the form of
  // a serialized string. Must not exceed |kMaximumDeveloperDataSize| bytes.
  std::vector<char> data;

  // Actions that should be shown as buttons on the notification.
  std::vector<blink::mojom::NotificationActionPtr> actions;

  // The time at which the notification should be shown.
  std::optional<base::Time> show_trigger_timestamp;

  mojom::NotificationScenario scenario;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_PLATFORM_NOTIFICATION_DATA_H_
