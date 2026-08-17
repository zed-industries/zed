/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_LEGACY_ENCODED_AUDIO_FRAME_H_
#define MODULES_AUDIO_CODING_CODECS_LEGACY_ENCODED_AUDIO_FRAME_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio_codecs/audio_decoder.h"
#include "rtc_base/buffer.h"

namespace webrtc {

class LegacyEncodedAudioFrame final : public AudioDecoder::EncodedAudioFrame {
 public:
  LegacyEncodedAudioFrame(AudioDecoder* decoder, Buffer&& payload);
  ~LegacyEncodedAudioFrame() override;

  static std::vector<AudioDecoder::ParseResult> SplitBySamples(
      AudioDecoder* decoder,
      Buffer&& payload,
      uint32_t timestamp,
      size_t bytes_per_ms,
      uint32_t timestamps_per_ms);

  size_t Duration() const override;

  std::optional<DecodeResult> Decode(ArrayView<int16_t> decoded) const override;

  // For testing:
  const Buffer& payload() const { return payload_; }

 private:
  AudioDecoder* const decoder_;
  const Buffer payload_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_LEGACY_ENCODED_AUDIO_FRAME_H_
