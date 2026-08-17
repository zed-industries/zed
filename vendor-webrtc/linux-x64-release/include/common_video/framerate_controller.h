/*
 *  Copyright 2021 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_FRAMERATE_CONTROLLER_H_
#define COMMON_VIDEO_FRAMERATE_CONTROLLER_H_

#include <stdint.h>

#include <optional>

namespace webrtc {

// Determines which frames that should be dropped based on input framerate and
// requested framerate.
class FramerateController {
 public:
  FramerateController();
  explicit FramerateController(double max_framerate);
  ~FramerateController();

  // Sets max framerate (default is maxdouble).
  void SetMaxFramerate(double max_framerate);
  double GetMaxFramerate() const;

  // Returns true if the frame should be dropped, false otherwise.
  bool ShouldDropFrame(int64_t in_timestamp_ns);

  void Reset();

  void KeepFrame(int64_t in_timestamp_ns);

 private:
  double max_framerate_;
  std::optional<int64_t> next_frame_timestamp_ns_;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_FRAMERATE_CONTROLLER_H_
