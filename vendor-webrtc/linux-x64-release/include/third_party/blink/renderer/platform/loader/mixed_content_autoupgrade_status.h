// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_MIXED_CONTENT_AUTOUPGRADE_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_MIXED_CONTENT_AUTOUPGRADE_STATUS_H_

namespace blink {

// Used to log status of autoupgraded mixed content requests, matches histogram
// enum, DO NOT REORDER.
enum class MixedContentAutoupgradeStatus {
  kStarted = 0,
  kFailed = 1,
  kResponseReceived = 2,
  kMaxValue = kResponseReceived,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_MIXED_CONTENT_AUTOUPGRADE_STATUS_H_
