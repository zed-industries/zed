// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_RESOURCES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_RESOURCES_H_

#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace blink {

// Structure to hold the resources associated with a Web Notification.
struct BLINK_COMMON_EXPORT NotificationResources {
  NotificationResources();
  NotificationResources(const NotificationResources& other);
  ~NotificationResources();

  // Image for the notification. The bitmap may be empty if the developer did
  // not provide an image, or fetching of the image failed.
  SkBitmap image;

  // Main icon for the notification. The bitmap may be empty if the developer
  // did not provide an icon, or fetching of the icon failed.
  SkBitmap notification_icon;

  // Badge for the notification. The bitmap may be empty if the developer
  // did not provide a badge, or fetching of the badge failed.
  SkBitmap badge;

  // Icons for the actions. A bitmap may be empty if the developer did not
  // provide an icon, or fetching of the icon failed.
  std::vector<SkBitmap> action_icons;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_RESOURCES_H_
