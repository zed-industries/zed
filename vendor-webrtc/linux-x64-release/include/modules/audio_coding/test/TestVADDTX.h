/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_TEST_TESTVADDTX_H_
#define MODULES_AUDIO_CODING_TEST_TESTVADDTX_H_

#include <memory>

#include "absl/strings/string_view.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/environment/environment.h"
#include "api/neteq/neteq.h"
#include "common_audio/vad/include/vad.h"
#include "modules/audio_coding/acm2/acm_resampler.h"
#include "modules/audio_coding/include/audio_coding_module.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"
#include "modules/audio_coding/test/Channel.h"

namespace webrtc {

// This class records the frame type, and delegates actual sending to the
// `next_` AudioPacketizationCallback.
class MonitoringAudioPacketizationCallback : public AudioPacketizationCallback {
 public:
  explicit MonitoringAudioPacketizationCallback(
      AudioPacketizationCallback* next);

  int32_t SendData(AudioFrameType frame_type,
                   uint8_t payload_type,
                   uint32_t timestamp,
                   const uint8_t* payload_data,
                   size_t payload_len_bytes,
                   int64_t absolute_capture_timestamp_ms) override;

  void PrintStatistics();
  void ResetStatistics();
  void GetStatistics(uint32_t* stats);

 private:
  // 0 - kEmptyFrame
  // 1 - kAudioFrameSpeech
  // 2 - kAudioFrameCN
  uint32_t counter_[3];
  AudioPacketizationCallback* const next_;
};

// TestVadDtx is to verify that VAD/DTX perform as they should. It runs through
// an audio file and check if the occurrence of various packet types follows
// expectation. TestVadDtx needs its derived class to implement the Perform()
// to put the test together.
class TestVadDtx {
 public:
  static const int kOutputFreqHz = 16000;

  TestVadDtx();

 protected:
  // Returns true iff CN was added.
  bool RegisterCodec(const SdpAudioFormat& codec_format,
                     std::optional<Vad::Aggressiveness> vad_mode);

  // Encoding a file and see if the numbers that various packets occur follow
  // the expectation. Saves result to a file.
  // expects[x] means
  // -1 : do not care,
  // 0  : there have been no packets of type `x`,
  // 1  : there have been packets of type `x`,
  // with `x` indicates the following packet types
  // 0 - kEmptyFrame
  // 1 - kAudioFrameSpeech
  // 2 - kAudioFrameCN
  void Run(absl::string_view in_filename,
           int frequency,
           int channels,
           absl::string_view out_filename,
           bool append,
           const int* expects);

  const Environment env_;
  const scoped_refptr<AudioEncoderFactory> encoder_factory_;
  const scoped_refptr<AudioDecoderFactory> decoder_factory_;
  std::unique_ptr<AudioCodingModule> acm_send_;
  std::unique_ptr<NetEq> neteq_;
  acm2::ResamplerHelper resampler_helper_;
  std::unique_ptr<Channel> channel_;
  std::unique_ptr<MonitoringAudioPacketizationCallback> packetization_callback_;
  uint32_t time_stamp_ = 0x12345678;
};

// TestWebRtcVadDtx is to verify that the WebRTC VAD/DTX perform as they should.
class TestWebRtcVadDtx final : public TestVadDtx {
 public:
  TestWebRtcVadDtx();

  void Perform();

 private:
  void RunTestCases(const SdpAudioFormat& codec_format);
  void Test(bool new_outfile, bool expect_dtx_enabled);

  int output_file_num_;
};

// TestOpusDtx is to verify that the Opus DTX performs as it should.
class TestOpusDtx final : public TestVadDtx {
 public:
  void Perform();
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_TEST_TESTVADDTX_H_
