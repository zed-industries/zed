/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_QUALITY_LIMITATION_REASON_TRACKER_H_
#define VIDEO_QUALITY_LIMITATION_REASON_TRACKER_H_

#include <map>

#include "common_video/include/quality_limitation_reason.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// A tracker of quality limitation reasons. The quality limitation reason is the
// primary reason for limiting resolution and/or framerate (such as CPU or
// bandwidth limitations). The tracker keeps track of the current reason and the
// duration of time spent in each reason. See qualityLimitationReason[1],
// qualityLimitationDurations[2], and qualityLimitationResolutionChanges[3] in
// the webrtc-stats spec.
// Note that the specification defines the durations in seconds while the
// internal data structures defines it in milliseconds.
// [1]
// https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationreason
// [2]
// https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationdurations
// [3]
// https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationresolutionchanges
class QualityLimitationReasonTracker {
 public:
  // The caller is responsible for making sure `clock` outlives the tracker.
  explicit QualityLimitationReasonTracker(Clock* clock);

  // The current reason defaults to QualityLimitationReason::kNone.
  QualityLimitationReason current_reason() const;
  void SetReason(QualityLimitationReason reason);
  std::map<QualityLimitationReason, int64_t> DurationsMs() const;

 private:
  Clock* const clock_;
  QualityLimitationReason current_reason_;
  int64_t current_reason_updated_timestamp_ms_;
  // The total amount of time spent in each reason at time
  // `current_reason_updated_timestamp_ms_`. To get the total amount duration
  // so-far, including the time spent in `current_reason_` elapsed since the
  // last time `current_reason_` was updated, see DurationsMs().
  std::map<QualityLimitationReason, int64_t> durations_ms_;
};

}  // namespace webrtc

#endif  // VIDEO_QUALITY_LIMITATION_REASON_TRACKER_H_
