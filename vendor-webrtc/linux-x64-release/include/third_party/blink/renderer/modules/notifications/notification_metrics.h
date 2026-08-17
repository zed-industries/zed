// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_METRICS_H_

namespace blink {

// This enum is used in UMA. Do not delete or re-order entries. New entries
// should only be added at the end. Please keep in sync with
// "PersistentNotificationDisplayResult" in //tools/metrics/histograms/enums.xml
enum class PersistentNotificationDisplayResult {
  kOk = 0,
  kRegistrationNotActive = 1,
  kPermissionNotGranted = 2,
  kSilentWithVibrate = 3,
  kRenotifyWithoutTag = 4,
  kFailedToSerializeData = 5,
  kButtonActionWithPlaceholder = 6,
  kShowTriggerDelayTooFarAhead = 7,
  kTooMuchData = 8,
  kInternalError = 9,
  kPermissionDenied = 10,
  kMaxValue = kPermissionDenied,
};

void RecordPersistentNotificationDisplayResult(
    PersistentNotificationDisplayResult reason);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_METRICS_H_
