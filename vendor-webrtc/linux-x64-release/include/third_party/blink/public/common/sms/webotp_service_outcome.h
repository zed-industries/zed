// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_SERVICE_OUTCOME_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_SERVICE_OUTCOME_H_

namespace blink {

// This enum describes the outcome of the call made to the WebOTPService API.
enum class WebOTPServiceOutcome {
  // Don't change the meaning of these values because they are being recorded
  // in a metric.
  kSuccess = 0,
  kUnhandledRequest = 1,
  kConnectionError = 2,
  kCancelled = 3,
  kAborted = 4,
  kTimeout = 5,
  kUserCancelled = 6,
  kBackendNotAvailable = 7,
  kCrossDeviceFailure = 8,
  kMaxValue = kCrossDeviceFailure
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_SERVICE_OUTCOME_H_
