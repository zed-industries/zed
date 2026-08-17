/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_FRAME_RATE_ESTIMATOR_H_
#define COMMON_VIDEO_FRAME_RATE_ESTIMATOR_H_

#include <deque>
#include <optional>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {

// Class used to estimate a frame-rate using inter-frame intervals.
// Some notes on usage:
// This class is intended to accurately estimate the frame rate during a
// continuous stream. Unlike a traditional rate estimator that looks at number
// of data points within a time window, if the input stops this implementation
// will not smoothly fall down towards 0. This is done so that the estimated
// fps is not affected by edge conditions like if we sample just before or just
// after the next frame.
// To avoid problems if a stream is stopped and restarted (where estimated fps
// could look too low), users of this class should explicitly call Reset() on
// restart.
// Also note that this class is not thread safe, it's up to the user to guard
// against concurrent access.
class FrameRateEstimator {
 public:
  explicit FrameRateEstimator(TimeDelta averaging_window);

  // Insert a frame, potentially culling old frames that falls outside the
  // averaging window.
  void OnFrame(Timestamp time);

  // Get the current average FPS, based on the frames currently in the window.
  std::optional<double> GetAverageFps() const;

  // Move the window so it ends at `now`, and return the new fps estimate.
  std::optional<double> GetAverageFps(Timestamp now);

  // Completely clear the averaging window.
  void Reset();

 private:
  void CullOld(Timestamp now);
  const TimeDelta averaging_window_;
  std::deque<Timestamp> frame_times_;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_FRAME_RATE_ESTIMATOR_H_
