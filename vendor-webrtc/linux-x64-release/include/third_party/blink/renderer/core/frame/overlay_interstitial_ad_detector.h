// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OVERLAY_INTERSTITIAL_AD_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OVERLAY_INTERSTITIAL_AD_DETECTOR_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class LocalFrame;

// Detects overlay interstitials and record a use counter when an instance is
// found. The current scope is to consider only pop-ups, which appear after
// content on the page begins to load.
//
// Better Ads Standards definition:
// https://www.betterads.org/desktop-pop-up-ad/
// https://www.betterads.org/mobile-pop-up-ad/
//
// Heuristic:
// We do hit testing at the center of the browser viewport at regular intervals.
// The top element is an interstitial pop-up candidate if the following
// conditions are met:
// 1) It's immobile to scrolling (e.g. position:fixed).
// 2) The size is large.
// 3) It's created without user gesture.
// 4) It's created after the main content has loaded.
//
// The candidate will be actually counted as an overlay pop-up instance after we
// have checked some status at its dismissal time. On dismissal, if the main
// frame scrolling offset hasn't changed since the candidate's appearance, we
// count it as an overlay pop-up; otherwise, we skip that candidate because it
// could be a parallax/scroller ad.
//
// Besides, we explicitly prevent mid-roll ads (during a video play) from being
// categorized as pop-ups.
//
// We could potentially miss some true positive cases: the user could click at
// an empty space which activates the user gesture, and coincidentally the
// pop-up automatically shows up; the user could make some scrolling
// before closing the pop-up; etc. However, we accept the trade-off exchanging a
// lower rate of false positive for an increase in the rate of false negatives.
class CORE_EXPORT OverlayInterstitialAdDetector {
 public:
  OverlayInterstitialAdDetector() = default;
  OverlayInterstitialAdDetector(const OverlayInterstitialAdDetector&) = delete;
  OverlayInterstitialAdDetector& operator=(
      const OverlayInterstitialAdDetector&) = delete;
  ~OverlayInterstitialAdDetector() = default;

  void MaybeFireDetection(LocalFrame* outermost_main_frame);

 private:
  void OnPopupDetected(LocalFrame* outermost_main_frame, bool is_ad);

  bool started_detection_ = false;
  bool content_has_been_stable_ = false;

  // The following members are valid only when |started_detection_| is true.
  base::Time last_detection_time_;
  gfx::Size last_detection_outermost_main_frame_size_;

  DOMNodeId candidate_id_;
  bool candidate_is_ad_ = false;

  // The following members are valid only when |candidate_| is not nullptr.
  int candidate_start_outermost_main_frame_scroll_position_ = 0;

  // The node id of the last element that was detected as unqualified to be an
  // overlay pop-up. We compare any potential candidate with the last
  // unqualified element and skip it if they are equal.
  //
  // It allows us to exclude some false positive cases. e.g. an
  // overlay was excluded from the initial consideration because it was created
  // with a gesture. After 5 seconds the gesture would be gone, but we still
  // want to exclude it as it was originally created with a gesture.
  //
  // Another advantage is this saves some computation cost. e.g. if an ad was
  // unqualified because it didn't have a viewport constraint position, then we
  // can skip it on its next occurrence without computing the style again.
  DOMNodeId last_unqualified_element_id_;

  bool popup_detected_ = false;
  bool popup_ad_detected_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OVERLAY_INTERSTITIAL_AD_DETECTOR_H_
