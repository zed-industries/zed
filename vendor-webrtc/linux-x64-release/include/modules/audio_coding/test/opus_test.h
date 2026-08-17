/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_TEST_OPUS_TEST_H_
#define MODULES_AUDIO_CODING_TEST_OPUS_TEST_H_

#include <math.h>

#include <memory>

#include "api/neteq/neteq.h"
#include "modules/audio_coding/acm2/acm_resampler.h"
#include "modules/audio_coding/codecs/opus/opus_interface.h"
#include "modules/audio_coding/test/PCMFile.h"
#include "modules/audio_coding/test/TestStereo.h"

namespace webrtc {

class OpusTest {
 public:
  OpusTest();
  ~OpusTest();

  void Perform();

 private:
  void Run(TestPackStereo* channel,
           size_t channels,
           int bitrate,
           size_t frame_length,
           int percent_loss = 0);

  void OpenOutFile(int test_number);

  std::unique_ptr<NetEq> neteq_;
  acm2::ResamplerHelper resampler_helper_;
  TestPackStereo* channel_a2b_;
  PCMFile in_file_stereo_;
  PCMFile in_file_mono_;
  PCMFile out_file_;
  PCMFile out_file_standalone_;
  int counter_;
  uint8_t payload_type_;
  uint32_t rtp_timestamp_;
  acm2::ACMResampler resampler_;
  WebRtcOpusEncInst* opus_mono_encoder_;
  WebRtcOpusEncInst* opus_stereo_encoder_;
  WebRtcOpusDecInst* opus_mono_decoder_;
  WebRtcOpusDecInst* opus_stereo_decoder_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_TEST_OPUS_TEST_H_
