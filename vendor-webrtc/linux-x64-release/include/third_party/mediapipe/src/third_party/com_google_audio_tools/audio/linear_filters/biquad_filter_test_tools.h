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

// Contains matchers for verifying that the magnitude response of a biquad
// filter meets a specification.

#ifndef AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_TEST_TOOLS_H_
#define AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_TEST_TOOLS_H_

#include <functional>

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "gmock/gmock.h"
#include "absl/strings/str_format.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

namespace internal {
// Get human readable description of matcher.
template <typename LhsType>
std::string GetMatcherDescriptionAsString(
    const ::testing::Matcher<LhsType>& matcher, bool negation) {
  ::std::stringstream ss;
  if (negation) {
    matcher.DescribeNegationTo(&ss);
  } else {
    matcher.DescribeTo(&ss);
  }
  return ss.str();
}

// Pretty print a set of coefficients.
std::string AsString(const BiquadFilterCoefficients& coeffs);

// Pretty print the coefficients for a multi-stage filter.
std::string AsString(const BiquadFilterCascadeCoefficients& coeffs);

struct BiquadFilterTestTools {
  static bool IsMonotonicOnFrequencyRange(
      const std::function<double(double, double)>& response,
      double low_frequency_hz, double high_frequency_hz,
      double sample_rate_hz, int num_points, bool expect_increasing);
};
}  // namespace internal

// Matchers for checking frequency responses.
// Example use:
//   BiquadFilterCoefficients example = {{0, 0, 1}, {4, -1, 1}};
//   EXPECT_THAT(example,
//               MagnitudeResponseIs(DoubleNear(0.1666f, 1e-3), 0.0, 48000));
//   EXPECT_THAT(example, MagnitudeResponseIncreases(20, 6000, 48000, 100));

// Verifies that the magnitude response at a given frequency is within
// a tolerance of some expected value.
MATCHER_P3(MagnitudeResponseIs, inner_matcher, frequency_hz, sample_rate_hz,
           internal::GetMatcherDescriptionAsString(
               static_cast<::testing::Matcher<double>>(inner_matcher),
               negation)) {
  // In the event of an error, gmock will pretty print the filter coefficients.
  // It also prints some nasty data, but that seems to be unavoidable.
  const double magnitude_response =
      arg.GainMagnitudeAtFrequency(frequency_hz, sample_rate_hz);
  if (Value(magnitude_response,
            static_cast<::testing::Matcher<double>>(inner_matcher))) {
    return true;
  } else {
    *result_listener << " Coefficients { " << internal::AsString(arg) << " }"
        << " Magnitude response actually is: " << magnitude_response;
    return false;
  }
}

// Verifies that the phase response at a given frequency is within
// a tolerance of some expected value.
MATCHER_P3(PhaseResponseIs, inner_matcher, frequency_hz, sample_rate_hz,
           internal::GetMatcherDescriptionAsString(
               static_cast<::testing::Matcher<double>>(inner_matcher),
               negation)) {
  // In the event of an error, gmock will pretty print the filter coefficients.
  // It also prints some nasty data, but that seems to be unavoidable.
  const double phase_response =
      arg.PhaseResponseAtFrequency(frequency_hz, sample_rate_hz);
  if (Value(phase_response,
            static_cast<::testing::Matcher<double>>(inner_matcher))) {
    return true;
  } else {
    *result_listener << " Coefficients { " << internal::AsString(arg) << " }"
        << " Phase response actually is: " << phase_response;
    return false;
  }
}
// Verifies that the frequency response is monotonic as we move away from
// low_frequency_hz towards high_frequency_hz, where
//   low_frequency_hz < high_frequency_hz.
// We check num_points logarithmically spaced frequencies on this range.
MATCHER_P4(MagnitudeResponseIncreases, low, high, sample_rate_hz, num_points,
           absl::StrFormat("is monotonically increasing over [%f, %f]", low,
                           high)) {
  auto GetMagnitudeResponse = [arg](double frequency, double sample_rate) {
    return arg.GainMagnitudeAtFrequency(frequency, sample_rate);
  };
  return internal::BiquadFilterTestTools::IsMonotonicOnFrequencyRange(
      GetMagnitudeResponse, low, high, sample_rate_hz, num_points, true);
}

MATCHER_P4(MagnitudeResponseDecreases, low, high, sample_rate_hz, num_points,
           absl::StrFormat("is monotonically decreasing over [%f, %f]", low,
                           high)) {
  auto GetMagnitudeResponse = [arg](double frequency, double sample_rate) {
    return arg.GainMagnitudeAtFrequency(frequency, sample_rate);
  };
  return internal::BiquadFilterTestTools::IsMonotonicOnFrequencyRange(
      GetMagnitudeResponse, low, high, sample_rate_hz, num_points, false);
}

MATCHER_P4(PhaseResponseIncreases, low, high, sample_rate_hz, num_points,
           absl::StrFormat(
               "has monotonically increasing phase over [%f, %f]", low, high)) {
  auto GetPhaseResponse = [arg](double frequency, double sample_rate) {
    return arg.PhaseResponseAtFrequency(frequency, sample_rate);
  };
  LOG(INFO) << "ABSL_CHECKING RANGE: " << low << " " << high;
  return internal::BiquadFilterTestTools::IsMonotonicOnFrequencyRange(
      GetPhaseResponse, low, high, sample_rate_hz, num_points, true);
}

MATCHER_P4(PhaseResponseDecreases, low, high, sample_rate_hz, num_points,
           absl::StrFormat("has monotonically decreasing phase over [%f, %f]",
                           low, high)) {
  auto GetPhaseResponse = [arg](double frequency, double sample_rate) {
    return arg.PhaseResponseAtFrequency(frequency, sample_rate);
  };
  return internal::BiquadFilterTestTools::IsMonotonicOnFrequencyRange(
      GetPhaseResponse, low, high, sample_rate_hz, num_points, false);
}

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_TEST_TOOLS_H_
