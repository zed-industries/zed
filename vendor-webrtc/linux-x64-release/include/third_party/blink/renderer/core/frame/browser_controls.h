// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_BROWSER_CONTROLS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_BROWSER_CONTROLS_H_

#include "cc/input/browser_controls_state.h"
#include "cc/trees/browser_controls_params.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
class Page;

// This class encapsulate data and logic required to show/hide browser controls
// duplicating cc::BrowserControlsOffsetManager behaviour.  Browser controls'
// self-animation to completion is still handled by compositor and kicks in
// when scrolling is complete (i.e, upon ScrollEnd or FlingEnd). Browser
// controls can exist at the top or bottom of the screen and potentially at the
// same time. Bottom controls differ from top in that, if they exist alone,
// never translate the content down and scroll immediately, regardless of the
// controls' offset.
class CORE_EXPORT BrowserControls final
    : public GarbageCollected<BrowserControls> {
 public:
  explicit BrowserControls(Page&);

  void Trace(Visitor*) const;

  // The height the top controls are hidden; used for viewport adjustments
  // while the controls are resizing.
  float UnreportedSizeAdjustment();
  // The amount that browser controls are currently shown.
  float ContentOffset();
  float BottomContentOffset();

  float TopHeight() const { return params_.top_controls_height; }
  float TopMinHeight() const { return params_.top_controls_min_height; }
  float BottomHeight() const { return params_.bottom_controls_height; }
  float BottomMinHeight() const { return params_.bottom_controls_min_height; }
  float TotalHeight() const { return TopHeight() + BottomHeight(); }
  float TotalMinHeight() const { return TopMinHeight() + BottomMinHeight(); }
  bool ShrinkViewport() const {
    return params_.browser_controls_shrink_blink_size;
  }
  bool AnimateHeightChanges() const {
    return params_.animate_browser_controls_height_changes;
  }
  void SetParams(cc::BrowserControlsParams);
  cc::BrowserControlsParams Params() const { return params_; }

  float TopShownRatio() const { return top_shown_ratio_; }
  float BottomShownRatio() const { return bottom_shown_ratio_; }
  void SetShownRatio(float top_ratio, float bottom_ratio);

  void UpdateConstraintsAndState(cc::BrowserControlsState constraints,
                                 cc::BrowserControlsState current);

  cc::BrowserControlsState PermittedState() const { return permitted_state_; }

 private:
  void DidUpdateBrowserControls(bool update_safe_area_inset);
  void ResetBaseline();
  float TopMinShownRatio();
  float BottomMinShownRatio();

  Member<Page> page_;

  // The browser controls params such as heights, min-height etc.
  cc::BrowserControlsParams params_;

  // The browser controls shown amount (normalized from 0 to 1) since the last
  // compositor commit. This value is updated from two sources:
  // (1) compositor (impl) thread at the beginning of frame if it has
  //     scrolled browser controls since last commit.
  // (2) blink (main) thread updates this value if it scrolls browser controls
  //     when responding to gesture scroll events.
  // This value is reflected in web layer tree and is synced with compositor
  // during the commit.
  float top_shown_ratio_;
  float bottom_shown_ratio_;

  // Content offset when last re-baseline occurred.
  float baseline_top_content_offset_;
  float baseline_bottom_content_offset_;

  // Accumulated scroll delta since last re-baseline.
  float accumulated_scroll_delta_;

  // Constraints on the browser controls state
  cc::BrowserControlsState permitted_state_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_BROWSER_CONTROLS_H_
