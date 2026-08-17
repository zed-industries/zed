// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_UPDATE_SOURCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_UPDATE_SOURCE_H_

namespace blink {

// Limits of UserActivationV2 state updates originated from the current renderer
// process.
enum class UserActivationUpdateSource {
  kBrowser,
  kRenderer,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_UPDATE_SOURCE_H_
