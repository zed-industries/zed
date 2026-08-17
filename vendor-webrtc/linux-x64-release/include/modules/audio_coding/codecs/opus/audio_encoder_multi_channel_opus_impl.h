/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_OPUS_AUDIO_ENCODER_MULTI_CHANNEL_OPUS_IMPL_H_
#define MODULES_AUDIO_CODING_CODECS_OPUS_AUDIO_ENCODER_MULTI_CHANNEL_OPUS_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/audio_codecs/audio_format.h"
#include "api/audio_codecs/opus/audio_encoder_multi_channel_opus_config.h"
#include "api/units/time_delta.h"
#include "modules/audio_coding/codecs/opus/opus_interface.h"
#include "rtc_base/buffer.h"

namespace webrtc {

class RtcEventLog;

class AudioEncoderMultiChannelOpusImpl final : public AudioEncoder {
 public:
  AudioEncoderMultiChannelOpusImpl(
      const AudioEncoderMultiChannelOpusConfig& config,
      int payload_type);
  ~AudioEncoderMultiChannelOpusImpl() override;

  AudioEncoderMultiChannelOpusImpl(const AudioEncoderMultiChannelOpusImpl&) =
      delete;
  AudioEncoderMultiChannelOpusImpl& operator=(
      const AudioEncoderMultiChannelOpusImpl&) = delete;

  // Static interface for use by BuiltinAudioEncoderFactory.
  static constexpr const char* GetPayloadName() { return "multiopus"; }
  static std::optional<AudioCodecInfo> QueryAudioEncoder(
      const SdpAudioFormat& format);

  int SampleRateHz() const override;
  size_t NumChannels() const override;
  size_t Num10MsFramesInNextPacket() const override;
  size_t Max10MsFramesInAPacket() const override;
  int GetTargetBitrate() const override;

  void Reset() override;
  std::optional<std::pair<TimeDelta, TimeDelta>> GetFrameLengthRange()
      const override;

 protected:
  EncodedInfo EncodeImpl(uint32_t rtp_timestamp,
                         ArrayView<const int16_t> audio,
                         Buffer* encoded) override;

 private:
  static std::optional<AudioEncoderMultiChannelOpusConfig> SdpToConfig(
      const SdpAudioFormat& format);
  static AudioCodecInfo QueryAudioEncoder(
      const AudioEncoderMultiChannelOpusConfig& config);
  static std::unique_ptr<AudioEncoder> MakeAudioEncoder(
      const AudioEncoderMultiChannelOpusConfig&,
      int payload_type);

  size_t Num10msFramesPerPacket() const;
  size_t SamplesPer10msFrame() const;
  size_t SufficientOutputBufferSize() const;
  bool RecreateEncoderInstance(
      const AudioEncoderMultiChannelOpusConfig& config);
  void SetFrameLength(int frame_length_ms);
  void SetNumChannelsToEncode(size_t num_channels_to_encode);
  void SetProjectedPacketLossRate(float fraction);

  AudioEncoderMultiChannelOpusConfig config_;
  const int payload_type_;
  std::vector<int16_t> input_buffer_;
  OpusEncInst* inst_;
  uint32_t first_timestamp_in_buffer_;
  size_t num_channels_to_encode_;
  int next_frame_length_ms_;

  friend struct AudioEncoderMultiChannelOpus;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_OPUS_AUDIO_ENCODER_MULTI_CHANNEL_OPUS_IMPL_H_
