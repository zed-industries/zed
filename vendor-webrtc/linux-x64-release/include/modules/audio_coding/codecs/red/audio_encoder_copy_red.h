/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_RED_AUDIO_ENCODER_COPY_RED_H_
#define MODULES_AUDIO_CODING_CODECS_RED_AUDIO_ENCODER_COPY_RED_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <memory>
#include <optional>
#include <utility>

#include "api/array_view.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/field_trials_view.h"
#include "api/units/time_delta.h"
#include "rtc_base/buffer.h"

namespace webrtc {

// This class implements redundant audio coding as described in
//   https://tools.ietf.org/html/rfc2198
// The class object will have an underlying AudioEncoder object that performs
// the actual encodings. The current class will gather the N latest encodings
// from the underlying codec into one packet. Currently N is hard-coded to 2.

class AudioEncoderCopyRed final : public AudioEncoder {
 public:
  struct Config {
    Config();
    Config(Config&&);
    ~Config();
    int payload_type;
    std::unique_ptr<AudioEncoder> speech_encoder;
  };

  AudioEncoderCopyRed(Config&& config, const FieldTrialsView& field_trials);

  ~AudioEncoderCopyRed() override;

  AudioEncoderCopyRed(const AudioEncoderCopyRed&) = delete;
  AudioEncoderCopyRed& operator=(const AudioEncoderCopyRed&) = delete;

  int SampleRateHz() const override;
  size_t NumChannels() const override;
  int RtpTimestampRateHz() const override;
  size_t Num10MsFramesInNextPacket() const override;
  size_t Max10MsFramesInAPacket() const override;
  int GetTargetBitrate() const override;

  void Reset() override;
  bool SetFec(bool enable) override;

  bool SetDtx(bool enable) override;
  bool GetDtx() const override;

  bool SetApplication(Application application) override;
  void SetMaxPlaybackRate(int frequency_hz) override;
  bool EnableAudioNetworkAdaptor(const std::string& config_string,
                                 RtcEventLog* event_log) override;
  void DisableAudioNetworkAdaptor() override;
  void OnReceivedUplinkPacketLossFraction(
      float uplink_packet_loss_fraction) override;
  void OnReceivedUplinkBandwidth(int target_audio_bitrate_bps,
                                 std::optional<int64_t> bwe_period_ms) override;
  void OnReceivedUplinkAllocation(BitrateAllocationUpdate update) override;
  void OnReceivedRtt(int rtt_ms) override;
  void OnReceivedOverhead(size_t overhead_bytes_per_packet) override;
  void SetReceiverFrameLengthRange(int min_frame_length_ms,
                                   int max_frame_length_ms) override;
  ANAStats GetANAStats() const override;
  std::optional<std::pair<TimeDelta, TimeDelta>> GetFrameLengthRange()
      const override;
  ArrayView<std::unique_ptr<AudioEncoder>> ReclaimContainedEncoders() override;

 protected:
  EncodedInfo EncodeImpl(uint32_t rtp_timestamp,
                         ArrayView<const int16_t> audio,
                         Buffer* encoded) override;

 private:
  std::unique_ptr<AudioEncoder> speech_encoder_;
  Buffer primary_encoded_;
  size_t max_packet_length_;
  int red_payload_type_;
  std::list<std::pair<EncodedInfo, Buffer>> redundant_encodings_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_RED_AUDIO_ENCODER_COPY_RED_H_
