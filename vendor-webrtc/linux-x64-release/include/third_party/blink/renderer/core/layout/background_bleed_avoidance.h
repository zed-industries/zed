// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BACKGROUND_BLEED_AVOIDANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BACKGROUND_BLEED_AVOIDANCE_H_

namespace blink {

enum BackgroundBleedAvoidance {
  kBackgroundBleedNone,
  kBackgroundBleedShrinkBackground,
  kBackgroundBleedClipOnly,
  kBackgroundBleedClipLayer,
};

inline bool BleedAvoidanceIsClipping(BackgroundBleedAvoidance bleed_avoidance) {
  return bleed_avoidance == kBackgroundBleedClipOnly ||
         bleed_avoidance == kBackgroundBleedClipLayer;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BACKGROUND_BLEED_AVOIDANCE_H_
