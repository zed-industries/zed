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

// A simple, specialized class for doing multichannel, block filtering
// with length 2 FIR filters.

#ifndef AUDIO_LINEAR_FILTERS_TWO_TAP_FIR_FILTER_H_
#define AUDIO_LINEAR_FILTERS_TWO_TAP_FIR_FILTER_H_

#include <utility>

#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

// Filter with a length 2 element. In-place processing is supported.
class TwoTapFirFilter {
 public:
  explicit TwoTapFirFilter(std::pair<float, float> gains) {
    gain_now_ = gains.second;
    gain_prev_ = gains.first;
  }

  virtual ~TwoTapFirFilter() {}

  void Init(int num_channels) {
    num_channels_ = num_channels;
    Reset();
  }

  void Reset() {
    state_ = Eigen::ArrayXf::Zero(num_channels_);
  }

  // Process a block of samples. input is a 2D Eigen array with contiguous
  // column-major data, where the number of rows equals GetNumChannels().
  // &input = output is supported.
  void ProcessBlock(const Eigen::ArrayXXf& input, Eigen::ArrayXXf* output) {
    ABSL_DCHECK_EQ(input.rows(), num_channels_);
    // Resize is a no-op if the shape doesn't change, but this probably
    // should be improved if the number of input samples is expected to be
    // changing.
    workspace_block_.resize(input.rows(), input.cols());

    if (input.cols() > 0) {
      const int cols_minus_one = input.cols() - 1;
      if (cols_minus_one > 0) {
          workspace_block_.rightCols(cols_minus_one) =
            gain_now_ * input.rightCols(cols_minus_one) +
            gain_prev_ * input.leftCols(cols_minus_one);
      }
      workspace_block_.col(0) = gain_now_ * input.col(0) + gain_prev_ * state_;
      state_ = input.rightCols(1);
    }
    workspace_block_.swap(*output);
  }

 private:
  Eigen::ArrayXf state_;
  int num_channels_;
  float gain_now_;
  float gain_prev_;
  Eigen::ArrayXXf workspace_block_;
};

// A common and special case of the TwoTapFirFilter.
class FirstDifferenceFilter : public TwoTapFirFilter {
 public:
  FirstDifferenceFilter() : TwoTapFirFilter({-1, 1}) {}
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_TWO_TAP_FIR_FILTER_H_
