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

#ifndef AUDIO_DSP_FIXED_DELAY_LINE_H_
#define AUDIO_DSP_FIXED_DELAY_LINE_H_

#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


// A fast delay line of fixed length to be used with multichannel signals.
class FixedDelayLine {
 public:
  FixedDelayLine()
      : num_channels_(0 /* uninitialized */),
        delay_samples_(0),
        last_frame_(0) {}

  // Note that to prevent internal reallocations, you should never pass blocks
  // larger than block_size_samples. Changing block sizes is, however,
  // supported.
  //
  // If you don't know the block size, you can set block_size_samples to 0, but
  // expect to allocate when successively larger blocks come in.
  void Init(int num_channels, int delay_samples, int block_size_frames) {
    num_channels_ = num_channels;
    delay_samples_ = delay_samples;
    ABSL_CHECK_GE(delay_samples_, 0);
    buffer_.resize(num_channels_, delay_samples_ + block_size_frames);
    Reset();
  }

  void Reset() {
    last_request_size_frames_ = 0;
    // The following line allows us to get some zeros out before the samples
    // that we pass in.
    last_frame_ = delay_samples_;
    buffer_.setZero();
  }

  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output) {
    *output = ProcessBlock(input);
  }

  // Same as above, but avoids the copy for clients that only need const access
  // to the delayed data. Note that the underlying data of the returned map
  // is invalidated in the next call to ProcessBlock(), Init(), or Reset().
  template <typename InputType>
  Eigen::Map<const Eigen::ArrayXXf> ProcessBlock(const InputType& input) {
    ABSL_DCHECK_GT(num_channels_, 0);
    ABSL_DCHECK_EQ(input.rows(), num_channels_);
    // Make the allocated block bigger if necessary.
    const int needed_size_frames = last_frame_ + input.cols();
    if (buffer_.cols() < last_frame_ + input.cols()) {
      buffer_.conservativeResize(num_channels_, needed_size_frames);
    }
    // Remove the old block of data by copying the later part of the buffer into
    // the beginning of the buffer. We do this in chunks of maximum size
    // last_request_size_frames_ to be sure that there is no memory overlap in
    // the copy instruction.
    int start_frame = 0;
    while (true) {
      const int sample_shift = last_request_size_frames_;
      // If we get a small block when the previous block is large,
      // we won't actually have sample_shift samples to move.
      const int available_to_move = std::min<int>(
          sample_shift, needed_size_frames - (start_frame + sample_shift));
      if (available_to_move <= 0) { break; }
      buffer_.middleCols(start_frame, available_to_move) =
          buffer_.middleCols(start_frame + sample_shift, available_to_move);
      start_frame += sample_shift;
    }
    // Move the new chunk into the buffer.
    buffer_.middleCols(last_frame_ - last_request_size_frames_,
                       input.cols()) = input;
    // Update the state of the buffer.
    last_frame_ += input.cols() - last_request_size_frames_;
    last_request_size_frames_ = input.cols();
    return Eigen::Map<const Eigen::ArrayXXf>(
        buffer_.data(), num_channels_, input.cols());
  }

 private:
  int num_channels_;
  int delay_samples_;
  // The last column that contains valid data.
  int last_frame_;
  int last_request_size_frames_;
  Eigen::ArrayXXf buffer_;
};

#endif  // AUDIO_DSP_FIXED_DELAY_LINE_H_
