/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_G711_AUDIO_ENCODER_PCM_H_
#define MODULES_AUDIO_CODING_CODECS_G711_AUDIO_ENCODER_PCM_H_

#include <optional>
#include <utility>
#include <vector>

#include "api/audio_codecs/audio_encoder.h"
#include "api/units/time_delta.h"

namespace webrtc {

class AudioEncoderPcm : public AudioEncoder {
 public:
  struct Config {
   public:
    bool IsOk() const;

    int frame_size_ms;
    size_t num_channels;
    int payload_type;

   protected:
    explicit Config(int pt)
        : frame_size_ms(20), num_channels(1), payload_type(pt) {}
  };

  ~AudioEncoderPcm() override;

  int SampleRateHz() const override;
  size_t NumChannels() const override;
  size_t Num10MsFramesInNextPacket() const override;
  size_t Max10MsFramesInAPacket() const override;
  int GetTargetBitrate() const override;
  void Reset() override;
  std::optional<std::pair<TimeDelta, TimeDelta>> GetFrameLengthRange()
      const override;

 protected:
  AudioEncoderPcm(const Config& config, int sample_rate_hz);

  EncodedInfo EncodeImpl(uint32_t rtp_timestamp,
                         ArrayView<const int16_t> audio,
                         Buffer* encoded) override;

  virtual size_t EncodeCall(const int16_t* audio,
                            size_t input_len,
                            uint8_t* encoded) = 0;

  virtual size_t BytesPerSample() const = 0;

  // Used to set EncodedInfoLeaf::encoder_type in
  // AudioEncoderPcm::EncodeImpl
  virtual AudioEncoder::CodecType GetCodecType() const = 0;

 private:
  const int sample_rate_hz_;
  const size_t num_channels_;
  const int payload_type_;
  const size_t num_10ms_frames_per_packet_;
  const size_t full_frame_samples_;
  std::vector<int16_t> speech_buffer_;
  uint32_t first_timestamp_in_buffer_;
};

class AudioEncoderPcmA final : public AudioEncoderPcm {
 public:
  struct Config : public AudioEncoderPcm::Config {
    Config() : AudioEncoderPcm::Config(8) {}
  };

  explicit AudioEncoderPcmA(const Config& config)
      : AudioEncoderPcm(config, kSampleRateHz) {}

  AudioEncoderPcmA(const AudioEncoderPcmA&) = delete;
  AudioEncoderPcmA& operator=(const AudioEncoderPcmA&) = delete;

 protected:
  size_t EncodeCall(const int16_t* audio,
                    size_t input_len,
                    uint8_t* encoded) override;

  size_t BytesPerSample() const override;

  AudioEncoder::CodecType GetCodecType() const override;

 private:
  static const int kSampleRateHz = 8000;
};

class AudioEncoderPcmU final : public AudioEncoderPcm {
 public:
  struct Config : public AudioEncoderPcm::Config {
    Config() : AudioEncoderPcm::Config(0) {}
  };

  explicit AudioEncoderPcmU(const Config& config)
      : AudioEncoderPcm(config, kSampleRateHz) {}

  AudioEncoderPcmU(const AudioEncoderPcmU&) = delete;
  AudioEncoderPcmU& operator=(const AudioEncoderPcmU&) = delete;

 protected:
  size_t EncodeCall(const int16_t* audio,
                    size_t input_len,
                    uint8_t* encoded) override;

  size_t BytesPerSample() const override;

  AudioEncoder::CodecType GetCodecType() const override;

 private:
  static const int kSampleRateHz = 8000;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_G711_AUDIO_ENCODER_PCM_H_
