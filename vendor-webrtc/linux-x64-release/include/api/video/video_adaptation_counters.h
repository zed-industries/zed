/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_ADAPTATION_COUNTERS_H_
#define API_VIDEO_VIDEO_ADAPTATION_COUNTERS_H_

#include <string>

#include "rtc_base/checks.h"

namespace webrtc {

// Counts the number of adaptations have resulted due to resource overuse.
// Today we can adapt resolution and fps.
struct VideoAdaptationCounters {
  VideoAdaptationCounters() : resolution_adaptations(0), fps_adaptations(0) {}
  VideoAdaptationCounters(int resolution_adaptations, int fps_adaptations)
      : resolution_adaptations(resolution_adaptations),
        fps_adaptations(fps_adaptations) {
    RTC_DCHECK_GE(resolution_adaptations, 0);
    RTC_DCHECK_GE(fps_adaptations, 0);
  }

  int Total() const { return fps_adaptations + resolution_adaptations; }

  bool operator==(const VideoAdaptationCounters& rhs) const;
  bool operator!=(const VideoAdaptationCounters& rhs) const;

  VideoAdaptationCounters operator+(const VideoAdaptationCounters& other) const;

  std::string ToString() const;

  int resolution_adaptations;
  int fps_adaptations;
};

}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_ADAPTATION_COUNTERS_H_
