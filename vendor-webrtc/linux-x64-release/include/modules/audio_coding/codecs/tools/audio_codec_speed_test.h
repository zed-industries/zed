/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_TOOLS_AUDIO_CODEC_SPEED_TEST_H_
#define MODULES_AUDIO_CODING_CODECS_TOOLS_AUDIO_CODEC_SPEED_TEST_H_

#include <memory>
#include <string>

#include "test/gtest.h"

namespace webrtc {

// Define coding parameter as
// <channels, bit_rate, file_name, extension, if_save_output>.
typedef std::tuple<size_t, int, std::string, std::string, bool> coding_param;

class AudioCodecSpeedTest : public ::testing::TestWithParam<coding_param> {
 protected:
  AudioCodecSpeedTest(int block_duration_ms,
                      int input_sampling_khz,
                      int output_sampling_khz);
  virtual void SetUp();
  virtual void TearDown();

  // EncodeABlock(...) does the following:
  // 1. encodes a block of audio, saved in `in_data`,
  // 2. save the bit stream to `bit_stream` of `max_bytes` bytes in size,
  // 3. assign `encoded_bytes` with the length of the bit stream (in bytes),
  // 4. return the cost of time (in millisecond) spent on actual encoding.
  virtual float EncodeABlock(int16_t* in_data,
                             uint8_t* bit_stream,
                             size_t max_bytes,
                             size_t* encoded_bytes) = 0;

  // DecodeABlock(...) does the following:
  // 1. decodes the bit stream in `bit_stream` with a length of `encoded_bytes`
  // (in bytes),
  // 2. save the decoded audio in `out_data`,
  // 3. return the cost of time (in millisecond) spent on actual decoding.
  virtual float DecodeABlock(const uint8_t* bit_stream,
                             size_t encoded_bytes,
                             int16_t* out_data) = 0;

  // Encoding and decode an audio of `audio_duration` (in seconds) and
  // record the runtime for encoding and decoding separately.
  void EncodeDecode(size_t audio_duration);

  int block_duration_ms_;
  int input_sampling_khz_;
  int output_sampling_khz_;

  // Number of samples-per-channel in a frame.
  size_t input_length_sample_;

  // Expected output number of samples-per-channel in a frame.
  size_t output_length_sample_;

  std::unique_ptr<int16_t[]> in_data_;
  std::unique_ptr<int16_t[]> out_data_;
  size_t data_pointer_;
  size_t loop_length_samples_;
  std::unique_ptr<uint8_t[]> bit_stream_;

  // Maximum number of bytes in output bitstream for a frame of audio.
  size_t max_bytes_;

  size_t encoded_bytes_;
  float encoding_time_ms_;
  float decoding_time_ms_;
  FILE* out_file_;

  size_t channels_;

  // Bit rate is in bit-per-second.
  int bit_rate_;

  std::string in_filename_;

  // Determines whether to save the output to file.
  bool save_out_data_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_TOOLS_AUDIO_CODEC_SPEED_TEST_H_
