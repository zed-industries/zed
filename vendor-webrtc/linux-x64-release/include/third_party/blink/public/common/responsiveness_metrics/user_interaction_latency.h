// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_RESPONSIVENESS_METRICS_USER_INTERACTION_LATENCY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_RESPONSIVENESS_METRICS_USER_INTERACTION_LATENCY_H_
namespace blink {

// Enum class for user interaction types. It's used in UKM and should not be
// changed.
enum class UserInteractionType {
  kKeyboard = 0,
  kTapOrClick = 1,
  kDrag = 2 /* deprecated */
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_RESPONSIVENESS_METRICS_USER_INTERACTION_LATENCY_H_
