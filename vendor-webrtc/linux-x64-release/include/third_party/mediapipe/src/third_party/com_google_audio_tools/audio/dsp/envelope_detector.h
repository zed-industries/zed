/*
 * Copyright 2020-2021 Google LLC
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

// Determines as a function of time whether an audio channel has high signal
// energy within a specified frequency range.

#ifndef AUDIO_DSP_ENVELOPE_DETECTOR_H_
#define AUDIO_DSP_ENVELOPE_DETECTOR_H_

#include <vector>

#include "audio/dsp/resampler_q.h"
#include "audio/linear_filters/biquad_filter.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Envelope detection is done by rectifying an (optionally) filtered signal and
// using a smoothing filter with a very low cutoff. The signal can then be
// downsampled to a low data rate without loss of information and the samples
// can be used to compute more concise energy-based features.
class EnvelopeDetector {
 public:
  EnvelopeDetector()
      : num_channels_(0 /* uninitialized */) {}

  // The coefficients for the prefilter, coeffs, must be designed for a sample
  // rate sample_rate_hz and not the downsampled rate.
  //
  // NOTE: If envelope_sample_rate_hz identically equals sample_rate_hz, the
  // number of output samples for each call to process block is guaranteed to
  // be equal to the number of input samples (the resampler is bypassed).
  void Init(int num_channels, float sample_rate_hz,
            float envelope_cutoff_hz, float envelope_sample_rate_hz,
            const linear_filters::BiquadFilterCascadeCoefficients& coeffs);

  void Init(int num_channels, float sample_rate_hz,
            float envelope_cutoff_hz, float envelope_sample_rate_hz,
            const linear_filters::BiquadFilterCoefficients& coeffs) {
    Init(num_channels, sample_rate_hz, envelope_cutoff_hz,
         envelope_sample_rate_hz,
         linear_filters::BiquadFilterCascadeCoefficients(coeffs));
  }

  void Init(int num_channels, float sample_rate_hz,
            float envelope_cutoff_hz, float envelope_sample_rate_hz) {
    Init(num_channels, sample_rate_hz,
         envelope_cutoff_hz, envelope_sample_rate_hz,
         linear_filters::BiquadFilterCascadeCoefficients());
  }

  // Clear the state of the filters and resampler.
  void Reset();

  // Process a block of multi-channel data with the envelope detector. The
  // output rate is given by envelope_sample_rate_hz_. For low values
  // of envelope_cutoff_hz_ or small blocks of input samples, this
  // may not produce samples at every call.
  // You must initialize first. Returns true when successful.
  bool ProcessBlock(const Eigen::ArrayXXf& input, Eigen::ArrayXXf* output);

  // Returns an array containing the smoothed energy at each channel for the
  // most recently processed samples.
  const Eigen::ArrayXf& MostRecentRmsEnvelopeValue() const {
    return most_recent_output_;
  }

 private:
  int num_channels_;
  float sample_rate_hz_;

  float envelope_cutoff_hz_;
  float envelope_sample_rate_hz_;

  // Space for intermediate computations so that we don't need to reallocate
  // every time Process(...) is called.
  Eigen::ArrayXXf workspace_;
  Eigen::ArrayXf most_recent_output_;

  linear_filters::BiquadFilterCascade<Eigen::ArrayXf> prefilter_;
  linear_filters::BiquadFilter<Eigen::ArrayXf> envelope_smoother_;
  audio_dsp::QResampler<float> downsampler_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_ENVELOPE_DETECTOR_H_
