/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_QUALITY_TEST_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_QUALITY_TEST_H_

#include <fstream>
#include <memory>

#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/neteq/neteq.h"
#include "modules/audio_coding/neteq/tools/audio_sink.h"
#include "modules/audio_coding/neteq/tools/input_audio_file.h"
#include "modules/audio_coding/neteq/tools/rtp_generator.h"
#include "system_wrappers/include/clock.h"
#include "test/gtest.h"

namespace webrtc {
namespace test {

enum LossModes {
  kNoLoss,
  kUniformLoss,
  kGilbertElliotLoss,
  kFixedLoss,
  kLastLossMode
};

class LossModel {
 public:
  virtual ~LossModel() {}
  virtual bool Lost(int now_ms) = 0;
};

class NoLoss : public LossModel {
 public:
  bool Lost(int now_ms) override;
};

class UniformLoss : public LossModel {
 public:
  UniformLoss(double loss_rate);
  bool Lost(int now_ms) override;
  void set_loss_rate(double loss_rate) { loss_rate_ = loss_rate; }

 private:
  double loss_rate_;
};

class GilbertElliotLoss : public LossModel {
 public:
  GilbertElliotLoss(double prob_trans_11, double prob_trans_01);
  ~GilbertElliotLoss() override;
  bool Lost(int now_ms) override;

 private:
  // Prob. of losing current packet, when previous packet is lost.
  double prob_trans_11_;
  // Prob. of losing current packet, when previous packet is not lost.
  double prob_trans_01_;
  bool lost_last_;
  std::unique_ptr<UniformLoss> uniform_loss_model_;
};

struct FixedLossEvent {
  int start_ms;
  int duration_ms;
  FixedLossEvent(int start_ms, int duration_ms)
      : start_ms(start_ms), duration_ms(duration_ms) {}
};

struct FixedLossEventCmp {
  bool operator()(const FixedLossEvent& l_event,
                  const FixedLossEvent& r_event) const {
    return l_event.start_ms < r_event.start_ms;
  }
};

class FixedLossModel : public LossModel {
 public:
  FixedLossModel(std::set<FixedLossEvent, FixedLossEventCmp> loss_events);
  ~FixedLossModel() override;
  bool Lost(int now_ms) override;

 private:
  std::set<FixedLossEvent, FixedLossEventCmp> loss_events_;
  std::set<FixedLossEvent, FixedLossEventCmp>::iterator loss_events_it_;
};

class NetEqQualityTest : public ::testing::Test {
 protected:
  NetEqQualityTest(int block_duration_ms,
                   int in_sampling_khz,
                   int out_sampling_khz,
                   const SdpAudioFormat& format,
                   const scoped_refptr<AudioDecoderFactory>& decoder_factory =
                       webrtc::CreateBuiltinAudioDecoderFactory());
  ~NetEqQualityTest() override;

  void SetUp() override;

  // EncodeBlock(...) does the following:
  // 1. encodes a block of audio, saved in `in_data` and has a length of
  // `block_size_samples` (samples per channel),
  // 2. save the bit stream to `payload` of `max_bytes` bytes in size,
  // 3. returns the length of the payload (in bytes),
  virtual int EncodeBlock(int16_t* in_data,
                          size_t block_size_samples,
                          Buffer* payload,
                          size_t max_bytes) = 0;

  // PacketLost(...) determines weather a packet sent at an indicated time gets
  // lost or not.
  bool PacketLost();

  // DecodeBlock() decodes a block of audio using the payload stored in
  // `payload_` with the length of `payload_size_bytes_` (bytes). The decoded
  // audio is to be stored in `out_data_`.
  int DecodeBlock();

  // Transmit() uses `rtp_generator_` to generate a packet and passes it to
  // `neteq_`.
  int Transmit();

  // Runs encoding / transmitting / decoding.
  void Simulate();

  // Write to log file. Usage Log() << ...
  std::ofstream& Log();

  SdpAudioFormat audio_format_;
  const size_t channels_;

 private:
  int decoded_time_ms_;
  int decodable_time_ms_;
  double drift_factor_;
  int packet_loss_rate_;
  const int block_duration_ms_;
  const int in_sampling_khz_;
  const int out_sampling_khz_;

  // Number of samples per channel in a frame.
  const size_t in_size_samples_;

  size_t payload_size_bytes_;
  size_t max_payload_bytes_;

  std::unique_ptr<InputAudioFile> in_file_;
  std::unique_ptr<AudioSink> output_;
  std::ofstream log_file_;

  std::unique_ptr<RtpGenerator> rtp_generator_;
  std::unique_ptr<NetEq> neteq_;
  std::unique_ptr<LossModel> loss_model_;

  std::unique_ptr<int16_t[]> in_data_;
  Buffer payload_;
  AudioFrame out_frame_;
  RTPHeader rtp_header_;

  size_t total_payload_size_bytes_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_QUALITY_TEST_H_
