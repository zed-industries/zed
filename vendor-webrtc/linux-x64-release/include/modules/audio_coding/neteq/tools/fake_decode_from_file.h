/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_FAKE_DECODE_FROM_FILE_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_FAKE_DECODE_FROM_FILE_H_

#include <memory>
#include <optional>

#include "api/array_view.h"
#include "api/audio_codecs/audio_decoder.h"
#include "modules/audio_coding/neteq/tools/input_audio_file.h"

namespace webrtc {
namespace test {
// Provides an AudioDecoder implementation that delivers audio data from a file.
// The "encoded" input should contain information about what RTP timestamp the
// encoding represents, and how many samples the decoder should produce for that
// encoding. A helper method PrepareEncoded is provided to prepare such
// encodings. If packets are missing, as determined from the timestamps, the
// file reading will skip forward to match the loss.
class FakeDecodeFromFile : public AudioDecoder {
 public:
  FakeDecodeFromFile(std::unique_ptr<InputAudioFile> input,
                     int sample_rate_hz,
                     bool stereo)
      : input_(std::move(input)),
        sample_rate_hz_(sample_rate_hz),
        stereo_(stereo) {}

  ~FakeDecodeFromFile() = default;

  std::vector<ParseResult> ParsePayload(Buffer&& payload,
                                        uint32_t timestamp) override;

  void Reset() override {}

  int SampleRateHz() const override { return sample_rate_hz_; }

  size_t Channels() const override { return stereo_ ? 2 : 1; }

  int DecodeInternal(const uint8_t* encoded,
                     size_t encoded_len,
                     int sample_rate_hz,
                     int16_t* decoded,
                     SpeechType* speech_type) override;

  // Reads `samples` from the input file and writes the results to
  // `destination`. Location in file is determined by `timestamp`.
  void ReadFromFile(uint32_t timestamp, size_t samples, int16_t* destination);

  // Helper method. Writes `timestamp`, `samples` and
  // `original_payload_size_bytes` to `encoded` in a format that the
  // FakeDecodeFromFile decoder will understand. `encoded` must be at least 12
  // bytes long.
  static void PrepareEncoded(uint32_t timestamp,
                             size_t samples,
                             size_t original_payload_size_bytes,
                             ArrayView<uint8_t> encoded);

 private:
  std::unique_ptr<InputAudioFile> input_;
  std::optional<uint32_t> next_timestamp_from_input_;
  const int sample_rate_hz_;
  const bool stereo_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_FAKE_DECODE_FROM_FILE_H_
