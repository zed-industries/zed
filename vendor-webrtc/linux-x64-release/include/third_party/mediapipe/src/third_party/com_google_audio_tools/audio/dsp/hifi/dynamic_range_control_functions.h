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

// Input-output level characteristics for dynamic gain control modules.
// They are templated for use with Eigen ArrayXf and Map<ArrayXf> types.

#ifndef AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_FUNCTIONS_H_
#define AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_FUNCTIONS_H_

#include <type_traits>

#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {
namespace internal {

// Generate a function that is zero below the knee and has a slope of -1 above
// transition_center. There is a smooth transition of length transition_width
// centered around transition_center. This function is actually the negative of
// what you usually expect a ReLU to be.
template <typename InputEigenType, typename OutputEigenType>
void SmoothReLU(const InputEigenType& input, float transition_center,
                float transition_width, OutputEigenType* output) {
  static_assert(std::is_same<typename InputEigenType::Scalar, float>::value,
                "Scalar type must be float.");
  if (transition_width > 0) {
    const float start_knee = transition_center - transition_width / 2;
    auto temp = input - start_knee;
    const float knee_inv = 1 / transition_width;
    *output =
        -0.5f * (temp.cwiseMax(0).cwiseMin(transition_width) * temp * knee_inv +
                 (temp - transition_width).cwiseMax(0));
  } else {
    *output = (transition_center - input).cwiseMin(0);
  }
}

// Allows the functions below to support scalar types.
inline void SmoothReLU(float input, float transition_center,
                float transition_width, float* output) {
  Eigen::Array<float, 1, 1> in;
  Eigen::Array<float, 1, 1> out;
  in[0] = input;
  SmoothReLU(in, transition_center, transition_width, &out);
  *output = out[0];
}

}  // namespace internal

// The input-output characteristic for a compressor/limiter is such that below
// the threshold, input level = output level, and above the threshold additional
// increase in gain is reduced by a factor equal to the ratio.
//
// When the knee width is nonzero, a quadratic polynomial is used to interpolate
// between the two behaviors. The transition goes from
// threshold_db - knee_width_db / 2 to threshold_db + knee_width_db / 2.
//
// The computation for the knee can be found here:
// http://c4dm.eecs.qmul.ac.uk/audioengineering/compressors/documents/Reiss-Tutorialondynamicrangecompression.pdf
//
// Equivalent code:
//   const float half_knee = knee_width_db / 2;
//   if (input_level_db - threshold_db <= -half_knee) {
//     return input_level_db;
//   } else if (input_level_db - threshold_db >= half_knee) {
//     return threshold_db + (input_level_db - threshold_db) / ratio;
//   } else {
//     const float knee_end = input_level_db - threshold_db + half_knee;
//     return input_level_db +
//         ((1 / ratio) - 1) * knee_end * knee_end / (knee_width_db * 2);
//   }
template <typename InputEigenType, typename OutputEigenType>
void OutputLevelCompressor(const InputEigenType& input_level_db,
                           float threshold_db, float ratio, float knee_width_db,
                           OutputEigenType* output_level_db) {
  internal::SmoothReLU(input_level_db, threshold_db, knee_width_db,
                       output_level_db);
  const float slope = 1.0f - (1.0f / ratio);
  *output_level_db = input_level_db + *output_level_db * slope;
}

template <typename InputEigenType, typename OutputEigenType>
void OutputLevelLimiter(const InputEigenType& input_level_db,
                        float threshold_db, float knee_width_db,
                        OutputEigenType* output_level_db) {
  internal::SmoothReLU(input_level_db, threshold_db, knee_width_db,
                       output_level_db);
  *output_level_db += input_level_db;
}

// The input-output characteristic for an expander/noise gate
// is such that above the threshold, input level = output level, and below the
// threshold additional increase in gain is reduced by a factor equal to the
// ratio.
//
// When the knee width is nonzero, a quadratic polynomial is used to interpolate
// between the two behaviors. The transition goes from
// threshold_db - knee_width_db / 2 to threshold_db + knee_width_db / 2.
//
// Equivalent code:
//   const float half_knee = knee_width_db / 2;
//   if (input_level_db - threshold_db >= half_knee) {
//     return input_level_db;
//   } else if (input_level_db - threshold_db <= -half_knee) {
//     return threshold_db + (input_level_db - threshold_db) * ratio;
//   } else {
//     const float knee_end = input_level_db - threshold_db - half_knee;
//     return input_level_db +
//         (1 - ratio) * knee_end * knee_end / (knee_width_db * 2);
//   }

template <typename InputEigenType, typename OutputEigenType>
void OutputLevelExpander(const InputEigenType& input_level_db,
                         float threshold_db, float ratio, float knee_width_db,
                         OutputEigenType* output_level_db) {
  internal::SmoothReLU(input_level_db, threshold_db, knee_width_db,
                       output_level_db);
  const float slope = ratio - 1;
  *output_level_db = input_level_db +
                     (input_level_db - threshold_db + *output_level_db) * slope;
}

// Parameters to configure one of the four compression regions in the two-way
// compression profile. See the documentation for
// TwoWayCompressionParams for suggestions and requirements for configuring
// each control type.
struct DynamicRangeControlNonlinearityParams {
  DynamicRangeControlNonlinearityParams()
      : ratio(1.0f), threshold_db(0.0f), knee_width_db(0.0f) {}

  // Please see the documentation of DynamicRangeControlParams in
  // dynamic_range_control.h for an explanation of the ratio. Note that the
  // upwards compression region is implemented using an expander, so the ratio
  // for this region must be specified using the relationshio for an expander.
  float ratio;

  // For a downwards compressor, the audio signal is compressed above the
  // threshold level. For an upwards compressor or expander, the audio signal is
  // compressed below the threshold level.
  float threshold_db;

  // If the knee width is nonzero, a quadratic polynomial is used to interpolate
  // between neigboring regions of the curve.
  float knee_width_db;
};

// A two-way compression curve performs both downwards and upwards compression
// on an audio signal around a center loudness. This profile is composed of
// four individual regions in which different types of compression occur: an
// expansion region, an upwards compression region, a "soft" downwards
// compression region (a region with a less severe ratio), and a "hard"
// downwards compression region (a region with a more severe ratio than the soft
// compression region). Each compression region is individually
// configurable, however, some constraints among the parameters must hold.
//
// NOTE: Clients do not need to account for the effects of overlapping
// nonlinearties.
//
//
// Configuration suggestions and requirements:
//  1. Defining regions
//    The regions of the curve are defined by thresholds. The intervals of each
//    region are as follows:
//      - Expander region: input <= expander_region.threshold_db
//      - Upwards compression region: expander_region.threhold_db < input <
//        upwards_compression_region.threshold_db
//      - Soft compression region: soft_compression_region.threshold_db <= input
//        < hard_compression_region.threshold_db
//      - Hard compression region: input >= hard_compression_region.threshold
//    The thresholds of the individual control types must be in ascending order
//    as expander threshold, upwards compressor threshold, soft compressor
//    threshold, and hard compressor threshold, respectively. Thresholds may be
//    equal to each other. There is an implicitly defined region of no change to
//    the input (unity ratio) between the upwards compression region and the
//    soft compression region. If all regions have a knee width of 0, the width
//    of this region will be the distance between the upwards compressor
//    region's threshold and the soft compressor region's threshold. Otherwise,
//    the knee widths of the surrounding regions will determine how large this
//    interval is.
//  2. Ratios
//    The ratio of the hard compressor region should be greater than that of the
//    soft compressor region, as it is intended to more aggressively compress
//    audio at high levels. The ratio of the upwards compressor should be
//    greater than or equal to 1, and the ratio of the expander should be
//    greater than or equal to 1. If not using an expander, set the ratio to 1
//    in order to not further boost noise, otherwise the upwards compressor will
//    boost all levels below its threshold indefinitely.
//  3. Knee width
//    The knees of individual regions may overlap. Use caution when choosing
//    knee widths. It is possible to choose parameters such that the curve
//    exhibits unintended behavior, such as adding a non-zero gain at the center
//    loudness, or changing the behavior of one or more compression regions
//    (i.e. upwards compression occurs in the expansion region).
struct TwoWayCompressionParams {
  // The expander is used to further reduce the loudness of the audio signal
  // below a given threshold. For typical use, the expander is meant to prevent
  // the upwards compressor from boosting levels below a certain point so as not
  // to increase noise. The ratio of the expander must be greater than or equal
  // to 1. For use with broadcast audio, the ratio will most likely be set to 1.
  DynamicRangeControlNonlinearityParams expander_region;

  // The upwards compressor boosts low levels below a given threshold. The ratio
  // of the upwards compressor must be greater than or equal to 1.
  DynamicRangeControlNonlinearityParams upwards_compressor_region;

  // The downwards compressors reduce the output level when the input level is
  // above a threshold. Both the soft and hard compressors should have a ratio
  // of greater than or equal to 1. The soft compressor should have a smaller,
  // less aggressive ratio than the hard compressor.
  DynamicRangeControlNonlinearityParams soft_compressor_region;
  DynamicRangeControlNonlinearityParams hard_compressor_region;
};

namespace internal {

enum ParamError {
  kNoParamError = 0,
  kThresholdError,
  kRatioError,
};

// Computes the effective ratio of a compressor applied on top of a previously
// compressed audio signal.
inline float ComputeAdjustedRatio(float desired_ratio, float previous_ratio) {
  return desired_ratio / previous_ratio;
}

// Verifies that the thresholds, ratios, and knees for a compound nonlinearity
// curve meet the configuration requirements outlined in the documentation for
// TwoWayCompressionParams.
ParamError VerifyParams(const TwoWayCompressionParams& params);

}  // namespace internal

// A two-way compressor performs both downwards and upwards compression about a
// center loudness. This particular implementation automatically derives params
// for four separate nonlinearities to realize the curve provided in the params.
template <typename InputEigenType, typename OutputEigenType>
void OutputLevelTwoWayCompressor(const InputEigenType& input_level_db,
                                 const TwoWayCompressionParams& params,
                                 OutputEigenType* output_level_db) {
  ABSL_CHECK_EQ(internal::VerifyParams(params), internal::kNoParamError);
  const DynamicRangeControlNonlinearityParams& expander =
      params.expander_region;
  const DynamicRangeControlNonlinearityParams& upwards_compressor =
      params.upwards_compressor_region;
  const DynamicRangeControlNonlinearityParams& soft_compressor =
      params.soft_compressor_region;
  const DynamicRangeControlNonlinearityParams& hard_compressor =
      params.hard_compressor_region;

  Eigen::ArrayXf temp = Eigen::ArrayXf::Zero(output_level_db->size());
  Eigen::ArrayXf input_scalar = Eigen::ArrayXf::Constant(1, 0);
  Eigen::ArrayXf output_scalar = Eigen::ArrayXf::Constant(1, 0);

  // Soft compression.
  OutputLevelCompressor(input_level_db, soft_compressor.threshold_db,
                        soft_compressor.ratio, soft_compressor.knee_width_db,
                        &temp);

  float hard_compressor_adjusted_ratio = internal::ComputeAdjustedRatio(
      hard_compressor.ratio, soft_compressor.ratio);

  // Compute the adjusted threshold for the hard compressor.
  input_scalar[0] = hard_compressor.threshold_db;
  OutputLevelCompressor(input_scalar, soft_compressor.threshold_db,
                        soft_compressor.ratio, 0.0f, &output_scalar);

  // Hard compression.
  OutputLevelCompressor(temp, output_scalar.value(),
                        hard_compressor_adjusted_ratio,
                        hard_compressor.knee_width_db, output_level_db);
  temp.swap(*output_level_db);

  // Upwards compression.
  // Note: we are using an expander to implement an upwards compressor. It is a
  // common convention to define the ratio of an upwards compressor as the
  // increase to the output level relative to the input level. For example, a
  // ratio of 2:1 means that if an input signal is 2dB below the threshold, the
  // output level is increased to 1dB above the signal. This is the opposite of
  // the definition of the ratio for an expander. Because of this, the
  // ratio is inverted below.
  float upwards_compressor_adjusted_ratio =
      1.0 / params.upwards_compressor_region.ratio;

  OutputLevelExpander(temp, upwards_compressor.threshold_db,
                      upwards_compressor_adjusted_ratio,
                      upwards_compressor.knee_width_db, output_level_db);
  temp.swap(*output_level_db);

  // Compute the ratio and threshold of the expander that will yield the desired
  // curve when applied to the signal after the upwards compressor.
  float expander_adjusted_ratio = internal::ComputeAdjustedRatio(
      expander.ratio, upwards_compressor_adjusted_ratio);

  // Compute the adjusted threshold for the expander.
  input_scalar[0] = expander.threshold_db;
  OutputLevelExpander(input_scalar, upwards_compressor.threshold_db,
                      upwards_compressor_adjusted_ratio, 0.0f, &output_scalar);

  // Expansion. This is the final output of the compound nonlinearity.
  OutputLevelExpander(temp, output_scalar.value(), expander_adjusted_ratio,
                      expander.knee_width_db, output_level_db);
}

}  // namespace audio_dsp

#endif  // AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_FUNCTIONS_H_
