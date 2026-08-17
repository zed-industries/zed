/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INST_H_
#define MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INST_H_

#include <stddef.h>

#include "rtc_base/ignore_wundef.h"

RTC_PUSH_IGNORING_WUNDEF()
#include "third_party/opus/src/include/opus.h"
#include "third_party/opus/src/include/opus_multistream.h"
RTC_POP_IGNORING_WUNDEF()

struct WebRtcOpusEncInst {
  OpusEncoder* encoder;
  OpusMSEncoder* multistream_encoder;
  size_t channels;
  int in_dtx_mode;
  int sample_rate_hz;
};

struct WebRtcOpusDecInst {
  OpusDecoder* decoder;
  OpusMSDecoder* multistream_decoder;
  size_t channels;
  int in_dtx_mode;
  int sample_rate_hz;
  // TODO: https://issues.webrtc.org/376493209 - Remove when libopus gets fixed.
  int last_packet_num_channels;
};

#endif  // MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INST_H_
