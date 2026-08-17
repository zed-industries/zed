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

#ifndef AUDIO_LINEAR_FILTERS_FIR_FILTER_H_
#define AUDIO_LINEAR_FILTERS_FIR_FILTER_H_

#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

namespace linear_filters {
// FirFilter implements a one dimensional, linear finite impulse response filter
// along the time dimension of a multi-channel signal. Having a different filter
// for each channel is supported.
class FirFilter {
 public:
  FirFilter()
    : num_channels_(0 /* uninitialized */) {}

  // filter is a one-dimensional impulse response that will be applied to each
  // channel.
  void Init(int num_channels, const Eigen::ArrayXf& filter);
  // The number of channels is inferred from the number of rows. This
  // initializer should be used if you want a filter whose impulse response is
  // channel-dependent.
  void Init(const Eigen::ArrayXXf& filter);

  void Reset();

  // Process a block of samples. For streaming, pass successive nonoverlapping
  // blocks of samples to this function. Init() must be called before calling
  // this function.
  // input and output must be 2D Eigen types with contiguous column-major data
  // like ArrayXXf or MatrixXf (or a Map of either type), where the number of
  // rows equals the number of channels, as set by Init(...).
  // In-place processing is not supported.
  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output) {
    ABSL_DCHECK_NE(num_channels_, 0) << "You must call Init() first!";
    ABSL_DCHECK_EQ(input.rows(), filter_.rows());
    output->resize(num_channels_, input.cols());
    output->setZero();
    if (input.cols() == 0) { return; }  // Nothing to process!
    // Move leftover state into output.
    const int state_to_flush_frames = std::min<int>(input.cols(),
                                                    state_.cols());
    output->leftCols(state_to_flush_frames) =
        state_.leftCols(state_to_flush_frames);

    // TODO: The next block of code is nearly identical code to what
    // we see in FixedDelayLine. Refactor into common function.

    // Move remaining state to the left, if there is any.
    int start_frame = 0;
    while (true) {
      const int sample_shift = state_to_flush_frames;
      const int available_to_move = std::min<int>(
          sample_shift, kernel_frames_ - (start_frame + sample_shift));
      if (available_to_move <= 0) { break; }
      state_.middleCols(start_frame, available_to_move) =
          state_.middleCols(start_frame + sample_shift, available_to_move);
      start_frame += sample_shift;
    }
    state_.rightCols(state_to_flush_frames).setZero();

    for (int i = 0; i < input.cols(); ++i) {
      auto this_frame = input.col(i);
      workspace_ = filter_.colwise() * this_frame;
      const int available_frames_in_output = input.cols() - i;

      const int to_accum_frames = std::min<int>(available_frames_in_output,
                                                kernel_frames_);
      output->middleCols(i, to_accum_frames) +=
          workspace_.leftCols(to_accum_frames);
      // Copy spillover into the state buffer.
      const int leftover_frames = kernel_frames_ - available_frames_in_output;
      if (leftover_frames > 0) {
        state_.leftCols(leftover_frames) +=
            workspace_.rightCols(leftover_frames);
      }
    }
  }

  int GetNumChannels() const {
    return num_channels_;
  }

 private:
  int num_channels_;
  int kernel_frames_;
  Eigen::ArrayXXf filter_;
  Eigen::ArrayXXf state_;
  Eigen::ArrayXXf workspace_;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_FIR_FILTER_H_
