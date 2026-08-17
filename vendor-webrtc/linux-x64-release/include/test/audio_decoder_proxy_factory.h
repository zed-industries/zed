/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_AUDIO_DECODER_PROXY_FACTORY_H_
#define TEST_AUDIO_DECODER_PROXY_FACTORY_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_decoder.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"

namespace webrtc {
namespace test {

// A decoder factory with a single underlying AudioDecoder object, intended for
// test purposes. Each call to MakeAudioDecoder returns a proxy for the same
// decoder, typically a mock or fake decoder.
class AudioDecoderProxyFactory : public AudioDecoderFactory {
 public:
  explicit AudioDecoderProxyFactory(AudioDecoder* decoder)
      : decoder_(decoder) {}

  // Unused by tests.
  std::vector<AudioCodecSpec> GetSupportedDecoders() override {
    RTC_DCHECK_NOTREACHED();
    return {};
  }

  bool IsSupportedDecoder(const SdpAudioFormat& format) override {
    return true;
  }

  std::unique_ptr<AudioDecoder> Create(
      const Environment& /* env */,
      const SdpAudioFormat& /* format */,
      std::optional<AudioCodecPairId> /* codec_pair_id */) override {
    return std::make_unique<DecoderProxy>(decoder_);
  }

 private:
  // Wrapper class, since CreateAudioDecoder needs to surrender
  // ownership to the object it returns.
  class DecoderProxy final : public AudioDecoder {
   public:
    explicit DecoderProxy(AudioDecoder* decoder) : decoder_(decoder) {}

   private:
    std::vector<ParseResult> ParsePayload(Buffer&& payload,
                                          uint32_t timestamp) override {
      return decoder_->ParsePayload(std::move(payload), timestamp);
    }

    bool HasDecodePlc() const override { return decoder_->HasDecodePlc(); }

    int ErrorCode() override { return decoder_->ErrorCode(); }

    void Reset() override { decoder_->Reset(); }

    int SampleRateHz() const override { return decoder_->SampleRateHz(); }

    size_t Channels() const override { return decoder_->Channels(); }

    int DecodeInternal(const uint8_t* encoded,
                       size_t encoded_len,
                       int sample_rate_hz,
                       int16_t* decoded,
                       SpeechType* speech_type) override {
      // Needed for tests of NetEqImpl::DecodeCng, which calls the deprecated
      // Decode method.
      size_t max_decoded_bytes =
          decoder_->PacketDuration(encoded, encoded_len) *
          decoder_->Channels() * sizeof(int16_t);
      return decoder_->Decode(encoded, encoded_len, sample_rate_hz,
                              max_decoded_bytes, decoded, speech_type);
    }

    void GeneratePlc(size_t requested_samples_per_channel,
                     BufferT<int16_t>* concealment_audio) override {
      decoder_->GeneratePlc(requested_samples_per_channel, concealment_audio);
    }

    AudioDecoder* const decoder_;
  };

  AudioDecoder* const decoder_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_AUDIO_DECODER_PROXY_FACTORY_H_
