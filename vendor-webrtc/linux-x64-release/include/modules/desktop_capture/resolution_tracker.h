/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_RESOLUTION_TRACKER_H_
#define MODULES_DESKTOP_CAPTURE_RESOLUTION_TRACKER_H_

#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

class ResolutionTracker final {
 public:
  // Sets the resolution to `size`. Returns true if a previous size was recorded
  // and differs from `size`.
  bool SetResolution(DesktopSize size);

  // Resets to the initial state.
  void Reset();

 private:
  DesktopSize last_size_;
  bool initialized_ = false;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_RESOLUTION_TRACKER_H_
