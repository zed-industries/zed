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

// A tool for estimating the background level of an acoustic scene in DBFS.
// (Decibels full scale). If the selected prefilter is a K-weighting filter,
// the output units are LUFS (Loudness units relative to full scale),
// see BackgroundLevelDetectorParams below. Full scale is relative as 1.0 RMS.
//
// This library is written such that it allows real-time parameter tweaking.
// For this reason, it has lots of setters for the parameters and uses a filter
// with coefficient smoothing. Currently it is not thread safe, so only update
// the parameters in between calls to ProcessBlock().
//


#ifndef AUDIO_DSP_LOUDNESS_BACKGROUND_LEVEL_DETECTOR_H_
#define AUDIO_DSP_LOUDNESS_BACKGROUND_LEVEL_DETECTOR_H_

#include "audio/dsp/envelope_detector.h"
#include "audio/linear_filters/biquad_filter.h"
#include "audio/linear_filters/ladder_filter.h"
#include "audio/linear_filters/perceptual_filter_design.h"
#include "third_party/eigen3/Eigen/Core"

namespace audio_dsp {

struct BackgroundLevelDetectorParams {
  BackgroundLevelDetectorParams()
      : weighting(linear_filters::kAWeighting),
        slow_smoother_cutoff_hz(1.0f),
        fast_smoother_cutoff_hz(16.0f),
        background_smoother_time_constant(1.0f),
        transient_sensitivity(0.7),
        transient_rejection_time(0.8),
        envelope_sample_rate_hz(100.0f) {}

  // The type of perceptual weighting to apply to the signal before estimating
  // the volume level.
  linear_filters::PerceptualWeightingType weighting;

  // Cutoff frequencies for the smoothing filters.
  //
  // These cutoffs change the bandwidth of the level envelope. The slow filter
  // does it's best to maintain an estimate of the background level (but it is
  // linear and can't reject transients well). The fast smoother should
  // capture transients but smooth over things like speech pitch pulses.
  float slow_smoother_cutoff_hz;
  // Unlike the slow smoother, the cutoff frequency of the fast smoother
  // cannot be changed after initialization. Note that this parameter should
  // not need to be changed much, the default is pretty good for audio
  // applications.
  float fast_smoother_cutoff_hz;

  // Controls the smoothness of the background level output. For constant
  // background level, this parameter won't change the output much. When a loud
  // constant noise begins (after it is decided to not be a transient) or ends,
  // this controls how quickly the output adapts.
  float background_smoother_time_constant;  // seconds.

  // Must be on the range (0, 1]. High sensitivities can be unresponsive. The
  // filter will work hard to reject transients and be too conservative. Low
  // sensitivities result in a less transient-robust measurement.
  float transient_sensitivity;

  // Loud noises longer than this length will be accepted as a new louder
  // background level. Shorter transients will be rejected.
  float transient_rejection_time;  // seconds.

  // Level computations are done at a highly reduced sample rate to keep
  // computation low.
  float envelope_sample_rate_hz;
};

class BackgroundLevelDetector {
 public:
  void Init(const BackgroundLevelDetectorParams& params, int num_channels,
            float sample_rate_hz);

  // Clear the state of the filters and resampler.
  void Reset();

  // Process a block of multi-channel data with the level detector. The two
  // outputs can be accessed using MostRecentInstantaneousLevel() and
  // MostRecentBackgroundLevel(). Filter must be initialized.
  bool ProcessBlock(const Eigen::ArrayXXf& input);

  // Gets the most recently processed samples of the instantaneous level
  // measurement in DBFS, corresponding to the samples from the same time
  // as the samples from GetBackgroundLevelEstimate().
  const Eigen::ArrayXXf& GetInstantaneousLevelEstimate() const {
    return fast_envelope_;
  }

  // Gets the most recently processed samples of the background level
  // measurement.
  const Eigen::ArrayXXf& GetBackgroundLevelEstimate() const {
    return background_;
  }

  // Returns an array containing the smoothed energy at each channel for the
  // most recently processed samples. This can be used to get a single value
  // that represents the most up-to-date measurement of the background level.
  // To get the dynamics of the system that occurred over the entire last block,
  // use GetBackgroundLevelEstimate().
  const Eigen::ArrayXf& MostRecentBackgroundLevelEstimate() const {
    return background_previous_samples_;
  }

  // Functions for changing the parameters. See the descriptions of each
  // parameter in BackgroundLevelDetectorParams for more details on setting
  // these.
  void SetSlowSmootherCutoffHz(float cutoff_hz);  // Coefficients are slewed.

  // Time constants must be strictly greater than
  // 1 / (2 * pi * envelope_sample_rate_hz), although a reasonable
  // ranges for experimentation would be [0.01, 30.0] seconds.
  void SetBackgroundSmootherTimeConstant(float time_constant_seconds);

  // Must be on the range (0, 1].
  void SetTransientSensitivity(float sensitivity);

  // Must be positive. Resonable values are less that a few seconds.
  void SetTransientRejectionTime(float transient_rejection_time_seconds);

  // Check the current value of the parameters.
  const BackgroundLevelDetectorParams& GetCurrentParams() {
    return params_;
  }

 private:
  BackgroundLevelDetectorParams params_;
  int num_channels_;
  double sample_rate_hz_;

  typedef Eigen::Array<int, Eigen::Dynamic, 1> ArrayXi;
  ArrayXi transient_counters_;

  // Intermediate and returned signals.
  Eigen::ArrayXXf fast_envelope_;
  Eigen::ArrayXXf slow_envelope_;
  Eigen::ArrayXXf background_;

  EnvelopeDetector envelope_detector_;
  // It is expected that the time constants of this filters will change.
  linear_filters::LadderFilter<Eigen::ArrayXf> slow_smoother_;

  // Smoother coefficient for the background level signal.
  float background_coeff_;
  // The most recently processed samples for the background smoothing filter.
  Eigen::ArrayXf background_previous_samples_;

  // params_.transient_rejection_time converted to samples.
  int transient_rejection_time_samples_;

  // There is an initial burn-in period in which the background filter moves
  // much faster to forget the zero initial condition and move toward the signal
  // level.
  std::vector<uint8 /* bool */> burn_in_stage_;
  float burn_in_background_coeff_;
};
}  // namespace audio_dsp

#endif  // AUDIO_DSP_LOUDNESS_BACKGROUND_LEVEL_DETECTOR_H_
