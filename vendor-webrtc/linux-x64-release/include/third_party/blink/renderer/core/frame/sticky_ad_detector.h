// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_STICKY_AD_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_STICKY_AD_DETECTOR_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"

namespace blink {

class LocalFrame;

// Detects large sticky ad at the bottom of the viewport, and record a use
// counter when an instance is found.
//
// Better Ads Standards definition:
// https://www.betterads.org/desktop-large-sticky-ad/
// https://www.betterads.org/mobile-large-sticky-ad/
//
// Heuristic:
// We do hit testing at the bottom center of the browser viewport at regular
// intervals. The top element is a sticky ad candidate if the following
// conditions are met:
// 1) It has a non-default position w.r.t. the viewport.
// 2) It's large in size (> 30% viewport size).
// 3) The main page is not scrollable.
//
// The candidate will be actually counted as a sticky ad instance at a later
// point, when we detect that the main frame scrolling position has changed by a
// distance greater than the height of the candidate, and the candidate is still
// at the bottom center. This allows us to exclude false positives like
// parallax/scroller ads.
class CORE_EXPORT StickyAdDetector {
 public:
  StickyAdDetector() = default;
  StickyAdDetector(const StickyAdDetector&) = delete;
  StickyAdDetector& operator=(const StickyAdDetector&) = delete;
  ~StickyAdDetector() = default;

  void MaybeFireDetection(LocalFrame* outermost_main_frame);

 private:
  void OnLargeStickyAdDetected(LocalFrame* outermost_main_frame);

  std::optional<base::Time> last_detection_time_;

  DOMNodeId candidate_id_;
  int candidate_height_;
  int candidate_start_outermost_main_frame_scroll_position_;

  bool done_detection_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_STICKY_AD_DETECTOR_H_
