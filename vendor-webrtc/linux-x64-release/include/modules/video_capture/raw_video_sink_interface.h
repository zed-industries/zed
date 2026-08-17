/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains interfaces used for creating the VideoCaptureModule
// and DeviceInfo.

#ifndef MODULES_VIDEO_CAPTURE_RAW_VIDEO_SINK_INTERFACE_H_
#define MODULES_VIDEO_CAPTURE_RAW_VIDEO_SINK_INTERFACE_H_

#include "modules/video_capture/video_capture_defines.h"

namespace webrtc {

class RawVideoSinkInterface {
 public:
  virtual ~RawVideoSinkInterface() = default;

  virtual int32_t OnRawFrame(uint8_t* videoFrame,
                             size_t videoFrameLength,
                             const webrtc::VideoCaptureCapability& frameInfo,
                             VideoRotation rotation,
                             int64_t captureTime) = 0;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CAPTURE_RAW_VIDEO_SINK_INTERFACE_H_
