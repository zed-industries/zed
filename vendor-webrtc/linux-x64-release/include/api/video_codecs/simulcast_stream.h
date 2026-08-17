/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_SIMULCAST_STREAM_H_
#define API_VIDEO_CODECS_SIMULCAST_STREAM_H_

#include <optional>

#include "api/video_codecs/scalability_mode.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// TODO(bugs.webrtc.org/6883): Unify with struct VideoStream, part of
// VideoEncoderConfig.
struct RTC_EXPORT SimulcastStream {
  // Temporary utility methods for transition from numberOfTemporalLayers
  // setting to ScalabilityMode.
  unsigned char GetNumberOfTemporalLayers() const;
  std::optional<ScalabilityMode> GetScalabilityMode() const;
  void SetNumberOfTemporalLayers(unsigned char n);

  bool operator==(const SimulcastStream& other) const;
  bool operator!=(const SimulcastStream& other) const {
    return !(*this == other);
  }

  int width = 0;
  int height = 0;
  float maxFramerate = 0;  // fps.
  unsigned char numberOfTemporalLayers = 1;
  unsigned int maxBitrate = 0;     // kilobits/sec.
  unsigned int targetBitrate = 0;  // kilobits/sec.
  unsigned int minBitrate = 0;     // kilobits/sec.
  unsigned int qpMax = 0;          // minimum quality
  bool active = false;             // encoded and sent.
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_SIMULCAST_STREAM_H_
