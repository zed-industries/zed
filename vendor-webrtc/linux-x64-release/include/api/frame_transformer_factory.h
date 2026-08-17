/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_FRAME_TRANSFORMER_FACTORY_H_
#define API_FRAME_TRANSFORMER_FACTORY_H_

#include <memory>

#include "api/frame_transformer_interface.h"
#include "rtc_base/system/rtc_export.h"

// This file contains EXPERIMENTAL functions to create video frames from
// either an old video frame or directly from parameters.
// These functions will be used in Chrome functionality to manipulate
// encoded frames from Javascript.
namespace webrtc {

// TODO(bugs.webrtc.org/14708): Add the required parameters to these APIs.
std::unique_ptr<TransformableVideoFrameInterface> CreateVideoSenderFrame();
// TODO(bugs.webrtc.org/14708): Consider whether Receiver frames ever make sense
// to create.
std::unique_ptr<TransformableVideoFrameInterface> CreateVideoReceiverFrame();
// Creates a new frame with the same metadata as the original.
// The original can be a sender or receiver frame.
RTC_EXPORT std::unique_ptr<TransformableAudioFrameInterface> CloneAudioFrame(
    TransformableAudioFrameInterface* original);
RTC_EXPORT std::unique_ptr<TransformableVideoFrameInterface> CloneVideoFrame(
    TransformableVideoFrameInterface* original);
}  // namespace webrtc

#endif  // API_FRAME_TRANSFORMER_FACTORY_H_
