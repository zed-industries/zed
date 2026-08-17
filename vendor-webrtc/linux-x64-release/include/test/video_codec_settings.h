/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_VIDEO_CODEC_SETTINGS_H_
#define TEST_VIDEO_CODEC_SETTINGS_H_

#include "api/video_codecs/video_encoder.h"

namespace webrtc {
namespace test {

const uint16_t kTestWidth = 352;
const uint16_t kTestHeight = 288;
const uint32_t kTestFrameRate = 30;
const unsigned int kTestMinBitrateKbps = 30;
const unsigned int kTestStartBitrateKbps = 300;
const uint8_t kTestPayloadType = 100;
const int64_t kTestTimingFramesDelayMs = 200;
const uint16_t kTestOutlierFrameSizePercent = 250;

static void CodecSettings(VideoCodecType codec_type, VideoCodec* settings) {
  *settings = {};

  settings->width = kTestWidth;
  settings->height = kTestHeight;

  settings->startBitrate = kTestStartBitrateKbps;
  settings->maxBitrate = 0;
  settings->minBitrate = kTestMinBitrateKbps;

  settings->maxFramerate = kTestFrameRate;

  settings->active = true;

  settings->qpMax = 56;  // See webrtcvideoengine.h.
  settings->numberOfSimulcastStreams = 0;

  settings->timing_frame_thresholds = {
      kTestTimingFramesDelayMs,
      kTestOutlierFrameSizePercent,
  };

  settings->codecType = codec_type;
  switch (codec_type) {
    case kVideoCodecVP8:
      *(settings->VP8()) = VideoEncoder::GetDefaultVp8Settings();
      return;
    case kVideoCodecVP9:
      *(settings->VP9()) = VideoEncoder::GetDefaultVp9Settings();
      return;
    case kVideoCodecH264:
      // TODO(brandtr): Set `qpMax` here, when the OpenH264 wrapper supports it.
      *(settings->H264()) = VideoEncoder::GetDefaultH264Settings();
      return;
    default:
      return;
  }
}
}  // namespace test
}  // namespace webrtc

#endif  // TEST_VIDEO_CODEC_SETTINGS_H_
