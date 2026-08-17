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

// Functions for operating on Eigen arrays. Matrix objects can be used with a
// call to .array(). Functions in this file should attempt to optimize speed by
// keeping allocations to a minimum.

#ifndef AUDIO_DSP_SIGNAL_EIGEN_UTIL_H_
#define AUDIO_DSP_SIGNAL_EIGEN_UTIL_H_

#include <memory>

#include "audio/dsp/batch_top_n.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Performs a convolution on 'container' with the impulse response
// h = [s, 1 - 2s, s], where s is the parameter 'smoothing_coefficient'.
// container is being overwritten in place.
// Filtering is zero-phase (e.g. h[0] = 1 - 2s).
template <typename EigenArrayType>
class ThreeTapSmoother {
 public:
  // Specify the number of points in the Array we want to smooth. The
  // number of points is only definable on instantiation because this internally
  // creates a temporary structure to speed up the computation.
  explicit ThreeTapSmoother(int num_points)
      : temp_(num_points),
        num_points_(num_points) {
    // You shouldn't create an instance of ThreeTapSmoother if you are trying to
    // smooth only a single point.
    ABSL_CHECK_GT(num_points_, 1);
  }

  // Smoothes *container with the 3-tap filter as a circular convolution
  // (periodic boundary handling).
  inline void SmoothCircular(float smoothing_coefficient,
                             Eigen::ArrayBase<EigenArrayType>* container) {
    SmoothInternal(kCircular, smoothing_coefficient, container);
  }

  // Smoothes *container with the 3-tap filter using whole-sample symmetric
  // boundary handling. Samples outside of *container are extrapolated by
  // reflecting about the endpoints as (*container)(-1) = (*container)(1)
  // and (*container)(N) = (*container)(N-2).
  inline void SmoothSymmetric(float smoothing_coefficient,
                              Eigen::ArrayBase<EigenArrayType>* container) {
    SmoothInternal(kReflect, smoothing_coefficient, container);
  }

  // Smoothes *container with the 3-tap filter using zero-padded boundary
  // handling. Samples outside of *container are extrapolated as zeros.
  // Output is the same size as the input, the two endpoints associated with
  // the first and last overlap of h and container are discarded. For example:
  // container = [1, 2, 3]
  // h = [0.1, 0.9, 0.1]
  // An acyclic convolution yields [0.1, 1.1, 2.2, 2.9, 0.3], but this function
  // returns only [1.1, 2.2, 2.9].
  inline void SmoothZeroPadding(float smoothing_coefficient,
                                Eigen::ArrayBase<EigenArrayType>* container) {
    SmoothInternal(kZeroPad, smoothing_coefficient, container);
  }

  inline int GetNumPoints() {
    return num_points_;
  }

 private:
  enum SmootherBoundaryBehavior {
    kCircular,
    kReflect,
    kZeroPad,
  };

  void SmoothInternal(SmootherBoundaryBehavior endpoint_type,
                      float smoothing_coefficient,
                      Eigen::ArrayBase<EigenArrayType>* container) {
    typedef typename EigenArrayType::Scalar ValueType;
    ABSL_CHECK(container);
    ABSL_CHECK_EQ(container->size(), num_points_);
    // We won't be smoothing outside of the bounds (0, 1/3].
    ABSL_CHECK_GT(smoothing_coefficient, 0.0);
    ABSL_CHECK_LE(smoothing_coefficient, 1 / 3.0);

    Eigen::ArrayBase<EigenArrayType>& input = *container;
    temp_.segment(1, num_points_ - 2) =
        input.segment(0, num_points_ - 2) + input.segment(2, num_points_ - 2);
    HandleEndpoints(input, endpoint_type);
    input += smoothing_coefficient * (temp_ - ValueType(2) * input);
  }

  inline void HandleEndpoints(const Eigen::ArrayBase<EigenArrayType>& input,
                              SmootherBoundaryBehavior endpoint_type) {
    switch (endpoint_type) {
      case kCircular:
        temp_[0] = input[num_points_ - 1] + input[1];
        temp_[num_points_ - 1] = input[num_points_ - 2] + input[0];
        break;
      case kReflect:
        temp_[0] = input[1] + input[1];
        temp_[num_points_ - 1] =
            input[num_points_ - 2] + input[num_points_ - 2];
        break;
      case kZeroPad:
        temp_[0] = input[1];
        temp_[num_points_ - 1] = input[num_points_ - 2];
        break;
    }
  }

  // Storage to allow for quick computation without repeated allocation.
  EigenArrayType temp_;
  // The length of the containers that will be computed on.
  const int num_points_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_SIGNAL_EIGEN_UTIL_H_
