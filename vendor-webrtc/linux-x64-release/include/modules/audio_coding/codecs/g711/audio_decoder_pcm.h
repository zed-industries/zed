/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_G711_AUDIO_DECODER_PCM_H_
#define MODULES_AUDIO_CODING_CODECS_G711_AUDIO_DECODER_PCM_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "api/audio_codecs/audio_decoder.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"

namespace webrtc {

class AudioDecoderPcmU final : public AudioDecoder {
 public:
  explicit AudioDecoderPcmU(size_t num_channels) : num_channels_(num_channels) {
    RTC_DCHECK_GE(num_channels, 1);
  }

  AudioDecoderPcmU(const AudioDecoderPcmU&) = delete;
  AudioDecoderPcmU& operator=(const AudioDecoderPcmU&) = delete;

  void Reset() override;
  std::vector<ParseResult> ParsePayload(Buffer&& payload,
                                        uint32_t timestamp) override;
  int PacketDuration(const uint8_t* encoded, size_t encoded_len) const override;
  int PacketDurationRedundant(const uint8_t* encoded,
                              size_t encoded_len) const override;
  int SampleRateHz() const override;
  size_t Channels() const override;

 protected:
  int DecodeInternal(const uint8_t* encoded,
                     size_t encoded_len,
                     int sample_rate_hz,
                     int16_t* decoded,
                     SpeechType* speech_type) override;

 private:
  const size_t num_channels_;
};

class AudioDecoderPcmA final : public AudioDecoder {
 public:
  explicit AudioDecoderPcmA(size_t num_channels) : num_channels_(num_channels) {
    RTC_DCHECK_GE(num_channels, 1);
  }

  AudioDecoderPcmA(const AudioDecoderPcmA&) = delete;
  AudioDecoderPcmA& operator=(const AudioDecoderPcmA&) = delete;

  void Reset() override;
  std::vector<ParseResult> ParsePayload(Buffer&& payload,
                                        uint32_t timestamp) override;
  int PacketDuration(const uint8_t* encoded, size_t encoded_len) const override;
  int PacketDurationRedundant(const uint8_t* encoded,
                              size_t encoded_len) const override;
  int SampleRateHz() const override;
  size_t Channels() const override;

 protected:
  int DecodeInternal(const uint8_t* encoded,
                     size_t encoded_len,
                     int sample_rate_hz,
                     int16_t* decoded,
                     SpeechType* speech_type) override;

 private:
  const size_t num_channels_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_G711_AUDIO_DECODER_PCM_H_
