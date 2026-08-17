/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_VARIABILITY_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_VARIABILITY_H_

namespace webrtc {

struct EchoPathVariability {
  enum class DelayAdjustment { kNone, kBufferFlush, kNewDetectedDelay };

  EchoPathVariability(bool gain_change,
                      DelayAdjustment delay_change,
                      bool clock_drift);

  bool AudioPathChanged() const {
    return gain_change || delay_change != DelayAdjustment::kNone;
  }
  bool gain_change;
  DelayAdjustment delay_change;
  bool clock_drift;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_VARIABILITY_H_
