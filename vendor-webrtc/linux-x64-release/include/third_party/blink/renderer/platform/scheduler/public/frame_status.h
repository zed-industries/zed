// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_STATUS_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class FrameScheduler;

namespace scheduler {

// This enum is used for histogram and should not be renumbered.
// This enum should be kept in sync with FrameSchedulingLifecycleState and
// FrameOriginState.
//
// There are three main states:
// VISIBLE describes frames which are visible to the user (both page and frame
// are visible).
// Without this service frame would have had kBackgrounded state.
// HIDDEN describes frames which are out of viewport but the page is visible
// to the user.
// BACKGROUND describes frames in background pages.
//
// There are four auxillary states:
// VISIBLE_SERVICE describes frames which are treated as visible to the user
// but it is a service (e.g. audio) which forces the page to be foregrounded.
// HIDDEN_SERVICE describes offscreen frames in pages which are treated as
// foregrounded due to a presence of a service (e.g. audio playing).
// BACKGROUND_EXEMPT_SELF describes background frames which are
// exempted from background throttling due to a special conditions being met
// for this frame.
// BACKGROUND_EXEMPT_kOther describes background frames which are exempted from
// background throttling due to other frames granting an exemption for
// the whole page.
//
// Note that all these seven states are disjoint, e.g, when calculating
// a metric for background BACKGROUND, BACKGROUND_EXEMPT_SELF and
// BACKGROUND_EXEMPT_kOther should be added together.
enum class FrameStatus {
  // Used to describe a task queue which doesn't have a frame associated
  // (e.g. global task queue).
  kNone = 0,

  // This frame was detached and does not have origin or visibility status
  // anymore.
  kDetached = 1,

  kSpecialCasesCount = 2,

  kMainFrameVisible = 2,
  kMainFrameVisibleService = 3,
  kMainFrameHidden = 4,
  kMainFrameHiddenService = 5,
  kMainFrameBackground = 6,
  kMainFrameBackgroundExemptSelf = 7,
  kMainFrameBackgroundExemptOther = 8,

  kSameOriginVisible = 9,
  kSameOriginVisibleService = 10,
  kSameOriginHidden = 11,
  kSameOriginHiddenService = 12,
  kSameOriginBackground = 13,
  kSameOriginBackgroundExemptSelf = 14,
  kSameOriginBackgroundExemptOther = 15,

  kCrossOriginVisible = 16,
  kCrossOriginVisibleService = 17,
  kCrossOriginHidden = 18,
  kCrossOriginHiddenService = 19,
  kCrossOriginBackground = 20,
  kCrossOriginBackgroundExemptSelf = 21,
  kCrossOriginBackgroundExemptOther = 22,

  kCount = 23
};

PLATFORM_EXPORT FrameStatus GetFrameStatus(FrameScheduler* frame_scheduler);

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_STATUS_H_
