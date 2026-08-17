// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_CONSTANTS_H_

#include "base/time/time.h"

namespace blink {

// Maximum allowed time delta into the future for show triggers. Allow a bit
// more than a year to account for leap years and seconds.
constexpr base::TimeDelta kMaxNotificationShowTriggerDelay = base::Days(367);

// TODO(johnme): The maximum number of actions is platform-specific and should
// be indicated by the embedder.

// Maximum number of actions on a Platform Notification.
constexpr size_t kNotificationMaxActions = 2;

// TODO(mvanouwerkerk): Update the notification resource loader to get the
// appropriate image sizes from the embedder.

// The maximum reasonable image size, scaled from dip units to pixels using the
// largest supported scaling factor. TODO(johnme): Check sizes are correct.
constexpr int kNotificationMaxImageWidthPx = 1800;  // 450 dip * 4
constexpr int kNotificationMaxImageHeightPx = 900;  // 225 dip * 4

// The maximum reasonable notification icon size, scaled from dip units to
// pixels using the largest supported scaling factor.
constexpr int kNotificationMaxIconSizePx = 320;  // 80 dip * 4

// The maximum reasonable badge size, scaled from dip units to pixels using the
// largest supported scaling factor.
constexpr int kNotificationMaxBadgeSizePx = 96;  // 24 dip * 4

// The maximum reasonable action icon size, scaled from dip units to
// pixels using the largest supported scaling factor.
constexpr int kNotificationMaxActionIconSizePx = 128;  // 32 dip * 4

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_CONSTANTS_H_
