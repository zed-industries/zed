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

#ifndef AUDIO_LINEAR_FILTERS_FILTERBANKS_FACTOR_TWO_DECIMATOR_H_
#define AUDIO_LINEAR_FILTERS_FILTERBANKS_FACTOR_TWO_DECIMATOR_H_

#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


// A very simple decimator that drops every other sample. Note that no
// anti-aliasing filter is used. The caller is responsible for removing high
// frequency signal components that would alias upon downsampling.
class FactorTwoDecimator {
 public:
  // const ArrayXXf specifies that the output arg is immutable. The outer stride
  // of the map depends on the number of mics. Since the data is stored
  // in memory in interleaved format, the stride for downsampling by 2 is
  // 2 * num_mics (Dynamic indicates that this size is unknown at compile time).
  using ConstDecimatedArrayXXf = Eigen::Map<const Eigen::ArrayXXf, 0,
                                            Eigen::OuterStride<Eigen::Dynamic>>;

  void Init(int num_mics) {
    ABSL_DCHECK_GE(num_mics, 0);
    num_mics_ = num_mics;
    Reset();
  }

  void Reset() {
    skip_next_sample_ = false;
    workspace_.resize(num_mics_, 0);
  }

  // It is expected that input is column-major and that the number of rows
  // equals num_mics (as passed to Init()).
  const Eigen::ArrayXXf& Decimate(const Eigen::ArrayXXf& input) {
    // There is a copy involved when the ConstDecimatedArrayXXf is casted to an
    // ArrayXXf. This forces the output to be a non-strided structure, which
    // allows it to be used by BiquadFilter and similar processing tools.
    workspace_ = ConstDecimatedArrayXXf(
        input.data() + (skip_next_sample_ ? num_mics_ : 0),
        input.rows(), (input.cols() + !skip_next_sample_) / 2,
        Eigen::OuterStride<Eigen::Dynamic>(2 * num_mics_));
    if (input.cols() % 2 == 1) {
      skip_next_sample_ = !skip_next_sample_;
    }
    return workspace_;
  }

 private:
  int num_mics_;
  bool skip_next_sample_;

  Eigen::ArrayXXf workspace_;
};
#endif  // AUDIO_LINEAR_FILTERS_FILTERBANKS_FACTOR_TWO_DECIMATOR_H_
