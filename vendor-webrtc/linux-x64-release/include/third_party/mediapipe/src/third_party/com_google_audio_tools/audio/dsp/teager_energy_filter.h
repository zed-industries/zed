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

// Streaming filter that computes the discrete-time Teager energy operator.
//
// The Teager energy operator is a cheap local measurement to estimate an
// envelope [https://en.wikipedia.org/wiki/Envelope_(waves)] of an oscillatory
// signal x(t). It is necessary that x(t) have narrow bandwidth, e.g. x(t) is
// the output of a bandpass filter, for this Teager envelope estimation to work.
//
//
// Continuous-Time Teager Energy:
//
// The Teager energy operator applied to a continuous-time signal x(t) is
//
//   E[x](t) := (x'(t))^2 - x(t) x"(t).
//
// Despite being an "energy," E[x](t) can be negative, e.g. when x(t) has a
// local minimum with positive sign, then x(t) > 0, x'(t) = 0, x"(t) > 0, so
// E[x](t) < 0.
// For x(t) a sinusoid, x(t) = A sin(w t), the Teager energy is
//
//   E[x](t) = (A w cos(w t))^2 - (A sin(w t)) (-A w^2 sin(w t)) = (A w)^2,
//
// i.e., the Teager energy is the square of amplitude times frequency. Based on
// this result, the envelope for an arbitrary narrowband signal is approximately
//
//   envelope(t) = sqrt(|E[x](t)|) / w,
//
// where w is the center frequency of the signal band in units of rad/s.
//
//
// Discrete-Time Teager Energy:
//
// The Teager energy of a discrete-time sequence x[n] is defined by
//
//   E'[x][n] := x[n]^2 - x[n-1] x[n+1].
//
// The discrete Teager energy for a sine wave x[n] = A sin(w T n) is (applying
// several trig identities)
//
//   E'[x][n] := A^2 sin(w T n)^2 - A sin(w T (n - 1)) A sin(w T (n + 1))
//             = A^2 (sin(w T n)^2 - (cos(2 w T) - cos(2 w T n)) / 2)
//             = A^2 (sin(w T n)^2 - (cos(2 w T) - (1 - 2 sin(w T n)^2) / 2)
//             = A^2 (1 - cos(2 w T)) / 2
//             = A^2 sin(w T)^2.
//
// So we obtain an approximate signal envelope for a general signal x[n] by
//
//   envelope[n] = sqrt(|E'[x][n]|) / sin(w T).
//
// where w is the center frequency of the signal band in units of rad/s and T is
// the sampling period in seconds.
//
//
// Reference:
//   J. F. Kaiser, "Some useful properties of Teager's energy operators,"
//   Acoustics, Speech, and Signal Processing, 1993. ICASSP-93., 1993 IEEE
//   International Conference on. Vol. 3. IEEE, 1993.
//   http://dx.doi.org/10.1109/ICASSP.1993.319457

#ifndef AUDIO_DSP_TEAGER_ENERGY_FILTER_H_
#define AUDIO_DSP_TEAGER_ENERGY_FILTER_H_

#include <cmath>
#include <vector>

#include "glog/logging.h"
#include "absl/types/span.h"

namespace audio_dsp {

// Streaming filter that computes the discrete-time Teager energy operator.
//
// The filter computes the discrete Teager energy scaled such that its square
// root approximates the signal envelope, where w is the center frequency in
// rad/s and T is the sampling period. The nth output sample is related to the
// input samples by
//
//   out[n] = (in[n-1]^2 - in[n-2] in[n]) / sin(w T)^2.
//
// The filter is delayed by one sample so that it is causal.
class TeagerEnergyFilter {
 public:
  TeagerEnergyFilter(double sample_rate_hz, double center_frequency_hz) {
      ABSL_CHECK_GT(sample_rate_hz, 0) << "The sample rate must be positive.";
      ABSL_CHECK_GT(center_frequency_hz, 0)
          << "The center frequency must be positive.";
      ABSL_CHECK_LT(center_frequency_hz, sample_rate_hz / 2)
          << "The center frequency must be less than the Nyquist frequency.";

      double radians_per_sample =
          2.0 * M_PI * center_frequency_hz / sample_rate_hz;
      normalize_factor_ = 1.0 /
          (std::sin(radians_per_sample) * std::sin(radians_per_sample));

      delay_s_ = 1.0 / sample_rate_hz;
      Reset();
    }

  void Reset() {
    delayed_input_[0] = 0.0;
    delayed_input_[1] = 0.0;
  }

  // Process a block of samples in a streaming manner.
  void ProcessBlock(absl::Span<const float> input,
                    std::vector<float>* output) {
    ABSL_DCHECK(output != nullptr);
    output->resize(input.size());
    if (input.empty()) { return; }

    // Compute out[n] = (in[n-1]^2 - in[n-2] in[n]) / sin(w T)^2.
    for (int n = 0; n < input.size(); ++n) {
      (*output)[n] = (delayed_input_[1] * delayed_input_[1] -
          delayed_input_[0] * input[n]) * normalize_factor_;
      delayed_input_[0] = delayed_input_[1];
      delayed_input_[1] = input[n];
    }
  }

  double Delay() const { return delay_s_; }

 private:
  // The filter's group delay at center frequency.
  double delay_s_;
  // The sin(w T)^2 value where w is the center frequency.
  double normalize_factor_;
  // The last two samples seen.
  float delayed_input_[2];
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_TEAGER_ENERGY_FILTER_H_
