// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADER_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADER_CONSTANTS_H_

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// HTTP header set in requests to indicate they should be marked DoNotTrack.
BLINK_COMMON_EXPORT extern const char kDoNotTrackHeader[];

// These values indicate the load progress constants shared between both
// //content and //blink.
//
// Initial progress value is set to 0.1 to help provide feedback as soon as a
// load starts.
constexpr double kInitialLoadProgress = 0.1;
constexpr double kFinalLoadProgress = 1.0;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADER_CONSTANTS_H_
