/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef AUDIO_DSP_SPECTRAL_PROCESSOR_H_
#define AUDIO_DSP_SPECTRAL_PROCESSOR_H_

#include <complex>
#include <memory>

#include "audio/dsp/circular_buffer.h"
#include "audio/dsp/kiss_fft.h"
#include "third_party/eigen3/Eigen/Core"

namespace audio_dsp {
// This module performs a streaming short-time-Fourier-transform (STFT) on the
// input stream.  Each STFT vector is passed to the block processor.  The output
// of that module is inverse-STFT'd and then written into output buffer.
// Framing is done on the signal such that the streaming block size need
// not be equal to the FFT block size.
// chunk_length: The number of time domain frames passed to ProcessChunk.
// block_length: The length of the FFT, in frames.
// hop_size: The shift in the input frames before taking the next FFT block.
// num_time_domain_frames: Equal in value to block_length, renamed for clarity.

class SpectralProcessor {
 public:
  using RowMajorArrayXXcf = Eigen::Array<std::complex<float>, Eigen::Dynamic,
                                         Eigen::Dynamic, Eigen::RowMajor>;

  class Callback {
   public:
    virtual ~Callback() {}

    // The spectra are stored as rows in in_block and out_block. The ith
    // channel's DC bin is stored at in_block(i, 0). They are contiguous in
    // memory since the structure is row major. num_time_domain_frames
    // corresponds to the number of *time domain* samples prior to taking the
    // FFT since multiple values of N block sizes may map to the same number of
    // columns in in_block/out_block.
    virtual void ProcessSTFTBlock(const RowMajorArrayXXcf& in_block,
                                  int num_time_domain_frames,
                                  RowMajorArrayXXcf* out_block) = 0;
  };

  // num_in_channels and num_out_channels specify the number of audio streams
  // that will be used at streaming time. chunk_length is the number of
  // frames (samples per channel) that will be passed to ProcessChunk(). The
  // samples will be windowed before and after the spectral transform by window.
  // The transform will have length block_length and a hop size of hop_size.
  // Power of two block lengths will be fastest, though block_length may be any
  // positive integer.
  SpectralProcessor(int num_in_channels, int num_out_channels,
                    int chunk_length, absl::Span<const float> window,
                    int block_length, int hop_size, Callback* block_processor);

  // Supports variable input chunk sizes.
  SpectralProcessor(int num_in_channels, int num_out_channels,
                    const std::vector<int>& possible_chunk_lengths,
                    absl::Span<const float> window, int block_length,
                    int hop_size, Callback* block_processor);

  ~SpectralProcessor() {}

  void Reset();

  // input and output must be column major Eigen structures with chunk_length
  // frames and the number of channels requested in the constructor. The delay
  // between input and output is given by latency_frames().
  void ProcessChunk(const Eigen::Map<const Eigen::ArrayXXf>& input,
                    Eigen::Map<Eigen::ArrayXXf> output);

  template <typename EigenType>
  void ProcessChunk(const EigenType& input, EigenType* output) {
    ABSL_DCHECK_EQ(input.rows(), num_in_channels());
    ABSL_DCHECK_LE(input.cols(), max_chunk_length());
    output->resize(num_out_channels(), input.cols());
    ProcessChunk(Eigen::Map<const Eigen::ArrayXXf>(
                     input.data(), num_in_channels(), input.cols()),
                 Eigen::Map<Eigen::ArrayXXf>(output->data(), num_out_channels(),
                                             input.cols()));
  }

  // Get the chunk length.
  //
  // The chunk length is the number of samples per channel (frames) that must be
  // passed to ProcessChunk.
  int max_chunk_length() const { return max_chunk_length_; }

  int num_in_channels() const { return num_in_channels_; }
  int num_out_channels() const { return num_out_channels_; }

  // Returns the latency between input and output.
  int latency_frames() const { return initial_delay_; }

 private:
  // Handles the frequency domain processing. Internally deinterleaves and
  // re-interleaves with windowing.
  const Eigen::Map<const Eigen::ArrayXXf> WindowAndProcessInFrequencyDomain(
      const Eigen::ArrayXXf& input);

  // Zero out the contents of out_chunk and move leftover samples into
  // the output buffer.
  void MoveOverflowIntoOutput(Eigen::Map<Eigen::ArrayXXf> out_chunk);

  // Move processed data into the output buffer and put leftovers in storage.
  // start_frame is the first column of out_chunk to use.
  void MoveProcessedIntoOutputAndOverflow(
      Eigen::Map<const Eigen::ArrayXXf> processed, int start_frame,
      Eigen::Map<Eigen::ArrayXXf> out_chunk);

  const int num_in_channels_;
  const int num_out_channels_;
  const int hop_size_;
  const int block_length_;
  const int max_chunk_length_;
  Callback* const block_processor_;

  int position_in_output_;
  int initial_delay_;
  CircularBuffer<float> in_circular_buffer_;

  Eigen::ArrayXXf out_overflow_;
  Eigen::ArrayXf window_;

  RealFFTTransformer transformer_;

  Eigen::ArrayXXf time_workspace_;
  Eigen::ArrayXXf unprocessed_block_workspace_;
  Eigen::ArrayXXf processed_block_workspace_;
  RowMajorArrayXXcf in_fft_workspace_;
  RowMajorArrayXXcf out_fft_workspace_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_SPECTRAL_PROCESSOR_H_
