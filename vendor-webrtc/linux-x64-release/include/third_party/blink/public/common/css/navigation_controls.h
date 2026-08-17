// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_NAVIGATION_CONTROLS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_NAVIGATION_CONTROLS_H_

namespace blink {

// Use for passing navigation controls from the OS to the renderer.
enum class NavigationControls {
  kNone,
  kBackButton,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_NAVIGATION_CONTROLS_H_
