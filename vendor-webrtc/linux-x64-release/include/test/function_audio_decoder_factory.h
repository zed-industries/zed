/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FUNCTION_AUDIO_DECODER_FACTORY_H_
#define TEST_FUNCTION_AUDIO_DECODER_FACTORY_H_

#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "absl/memory/memory.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "rtc_base/checks.h"

namespace webrtc {
namespace test {

// A decoder factory producing decoders by calling a supplied create function.
class FunctionAudioDecoderFactory : public AudioDecoderFactory {
 public:
  explicit FunctionAudioDecoderFactory(
      std::function<std::unique_ptr<AudioDecoder>()> create)
      : create_([create](const Environment&,
                         const SdpAudioFormat&,
                         std::optional<AudioCodecPairId> codec_pair_id) {
          return create();
        }) {}
  explicit FunctionAudioDecoderFactory(
      std::function<std::unique_ptr<AudioDecoder>(
          const Environment&,
          const SdpAudioFormat&,
          std::optional<AudioCodecPairId> codec_pair_id)> create)
      : create_(std::move(create)) {}

  // Unused by tests.
  std::vector<AudioCodecSpec> GetSupportedDecoders() override {
    RTC_DCHECK_NOTREACHED();
    return {};
  }

  bool IsSupportedDecoder(const SdpAudioFormat& format) override {
    return true;
  }

  std::unique_ptr<AudioDecoder> Create(
      const Environment& env,
      const SdpAudioFormat& format,
      std::optional<AudioCodecPairId> codec_pair_id) override {
    return create_(env, format, codec_pair_id);
  }

 private:
  const std::function<std::unique_ptr<AudioDecoder>(
      const Environment&,
      const SdpAudioFormat&,
      std::optional<AudioCodecPairId> codec_pair_id)>
      create_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FUNCTION_AUDIO_DECODER_FACTORY_H_
