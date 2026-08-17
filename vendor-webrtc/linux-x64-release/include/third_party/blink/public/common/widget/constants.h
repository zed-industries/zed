// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_CONSTANTS_H_

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// The minimum allowed window size that a renderer can set through
// window object
BLINK_COMMON_EXPORT extern const int kMinimumWindowSize;

// The minimum allowed window size for when the app is borderless.
BLINK_COMMON_EXPORT extern const int kMinimumBorderlessWindowSize;

// The timeout for clearing old paint for a cross-document navigation.
BLINK_COMMON_EXPORT extern const base::TimeDelta kNewContentRenderingDelay;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_CONSTANTS_H_
