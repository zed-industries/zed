/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_CORRUPTION_DETECTION_CONVERTERS_H_
#define COMMON_VIDEO_CORRUPTION_DETECTION_CONVERTERS_H_

#include <optional>

#include "api/transport/rtp/corruption_detection_message.h"
#include "common_video/frame_instrumentation_data.h"

namespace webrtc {

std::optional<FrameInstrumentationData>
ConvertCorruptionDetectionMessageToFrameInstrumentationData(
    const CorruptionDetectionMessage& message,
    int previous_sequence_index);
std::optional<FrameInstrumentationSyncData>
ConvertCorruptionDetectionMessageToFrameInstrumentationSyncData(
    const CorruptionDetectionMessage& message,
    int previous_sequence_index);
std::optional<CorruptionDetectionMessage>
ConvertFrameInstrumentationDataToCorruptionDetectionMessage(
    const FrameInstrumentationData& frame_instrumentation_data);
std::optional<CorruptionDetectionMessage>
ConvertFrameInstrumentationSyncDataToCorruptionDetectionMessage(
    const FrameInstrumentationSyncData& frame_instrumentation_sync_data);
}  // namespace webrtc

#endif  // COMMON_VIDEO_CORRUPTION_DETECTION_CONVERTERS_H_
