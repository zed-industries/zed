/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_TEST_UTILS_H_
#define MODULES_AUDIO_PROCESSING_TEST_TEST_UTILS_H_

#include <math.h>

#include <iterator>
#include <limits>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "api/audio/audio_processing.h"
#include "api/audio/audio_view.h"
#include "common_audio/channel_buffer.h"
#include "common_audio/wav_file.h"

namespace webrtc {

static const AudioProcessing::Error kNoErr = AudioProcessing::kNoError;
#define EXPECT_NOERR(expr) EXPECT_EQ(AudioProcessing::kNoError, (expr))

// Encapsulates samples and metadata for an integer frame.
struct Int16FrameData {
  // Max data size that matches the data size of the AudioFrame class, providing
  // storage for 8 channels of 96 kHz data.
  static const int kMaxDataSizeSamples = AudioFrame::kMaxDataSizeSamples;

  Int16FrameData() = default;

  void CopyFrom(const Int16FrameData& src);
  bool IsEqual(const Int16FrameData& frame) const;
  void Scale(float f);

  // Sets `samples_per_channel`, `num_channels` and, implicitly, the sample
  // rate. The sample rate is set to 100x that of samples per channel. I.e. if
  // samples_per_channel is 320, the sample rate will be set to 32000.
  void SetProperties(size_t samples_per_channel, size_t num_channels);

  size_t size() const { return view_.size(); }
  size_t samples_per_channel() const { return view_.samples_per_channel(); }
  size_t num_channels() const { return view_.num_channels(); }
  void set_num_channels(size_t num_channels);

  InterleavedView<int16_t> view() { return view_; }
  InterleavedView<const int16_t> view() const { return view_; }

  void FillData(int16_t value);
  void FillStereoData(int16_t left, int16_t right);

  // public struct members.
  std::array<int16_t, kMaxDataSizeSamples> data = {};
  int32_t sample_rate_hz = 0;

 private:
  InterleavedView<int16_t> view_;
};

// Reads ChannelBuffers from a provided WavReader.
class ChannelBufferWavReader final {
 public:
  explicit ChannelBufferWavReader(std::unique_ptr<WavReader> file);
  ~ChannelBufferWavReader();

  ChannelBufferWavReader(const ChannelBufferWavReader&) = delete;
  ChannelBufferWavReader& operator=(const ChannelBufferWavReader&) = delete;

  // Reads data from the file according to the `buffer` format. Returns false if
  // a full buffer can't be read from the file.
  bool Read(ChannelBuffer<float>* buffer);

 private:
  std::unique_ptr<WavReader> file_;
  std::vector<float> interleaved_;
};

// Writes ChannelBuffers to a provided WavWriter.
class ChannelBufferWavWriter final {
 public:
  explicit ChannelBufferWavWriter(std::unique_ptr<WavWriter> file);
  ~ChannelBufferWavWriter();

  ChannelBufferWavWriter(const ChannelBufferWavWriter&) = delete;
  ChannelBufferWavWriter& operator=(const ChannelBufferWavWriter&) = delete;

  void Write(const ChannelBuffer<float>& buffer);

 private:
  std::unique_ptr<WavWriter> file_;
  std::vector<float> interleaved_;
};

// Takes a pointer to a vector. Allows appending the samples of channel buffers
// to the given vector, by interleaving the samples and converting them to float
// S16.
class ChannelBufferVectorWriter final {
 public:
  explicit ChannelBufferVectorWriter(std::vector<float>* output);
  ChannelBufferVectorWriter(const ChannelBufferVectorWriter&) = delete;
  ChannelBufferVectorWriter& operator=(const ChannelBufferVectorWriter&) =
      delete;
  ~ChannelBufferVectorWriter();

  // Creates an interleaved copy of `buffer`, converts the samples to float S16
  // and appends the result to output_.
  void Write(const ChannelBuffer<float>& buffer);

 private:
  std::vector<float> interleaved_buffer_;
  std::vector<float>* output_;
};

// Exits on failure; do not use in unit tests.
FILE* OpenFile(absl::string_view filename, absl::string_view mode);

template <typename T>
void SetContainerFormat(int sample_rate_hz,
                        size_t num_channels,
                        Int16FrameData* frame,
                        std::unique_ptr<ChannelBuffer<T> >* cb) {
  frame->SetProperties(sample_rate_hz / 100, num_channels);
  cb->reset(new ChannelBuffer<T>(frame->samples_per_channel(), num_channels));
}

template <typename T>
float ComputeSNR(const T* ref, const T* test, size_t length, float* variance) {
  float mse = 0;
  float mean = 0;
  *variance = 0;
  for (size_t i = 0; i < length; ++i) {
    T error = ref[i] - test[i];
    mse += error * error;
    *variance += ref[i] * ref[i];
    mean += ref[i];
  }
  mse /= length;
  *variance /= length;
  mean /= length;
  *variance -= mean * mean;

  float snr = 100;  // We assign 100 dB to the zero-error case.
  if (mse > 0)
    snr = 10 * log10(*variance / mse);
  return snr;
}

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_TEST_UTILS_H_
