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

// An auditory cascade filterbank with optional decimation.
//

#ifndef AUDIO_LINEAR_FILTERS_FILTERBANKS_AUDITORY_CASCADE_FILTERBANK_H_
#define AUDIO_LINEAR_FILTERS_FILTERBANKS_AUDITORY_CASCADE_FILTERBANK_H_

#include "audio/linear_filters/biquad_filter.h"
#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/filterbanks/auditory_cascade_filterbank_params.pb.h"
#include "audio/linear_filters/filterbanks/factor_two_decimator.h"
#include "audio/linear_filters/two_tap_fir_filter.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

// A cascade of linear, second-order digital filters, each filter in the cascade
// being an asymmetric resonator with a complex pair of poles and a complex pair
// of zeros. For efficiency, this filterbank runs lower-frequency channels at
// lower sample rates.
//

class AuditoryCascadeFilterbank {
 public:
  explicit AuditoryCascadeFilterbank(
      const AuditoryCascadeFilterbankParams& params,
      bool allow_decimation = true)
      : num_mics_(0),
        initialized_(false),
        params_(params),
        allow_decimation_(allow_decimation) {}

  // Initialize a multichannel filterbank and specify the full sampling rate,
  // before any decimation takes place.
  //
  // Note that 4 or fewer mics should have much better performance due to
  // some special case templating used in BiquadFilter. See
  // ProcessBlockDynamicHelper in audio/linear_filters/filter_traits.h.

  void Init(int num_mics, float sample_rate);

  // Clear the internal state of the filter. This does not reset filter
  // coefficients.
  void Reset();

  // Process a block of samples. input is a 2D Eigen array with contiguous
  // column-major data, where the number of rows equals GetNumChannels().
  void ProcessBlock(const Eigen::ArrayXXf& input);

  // Filtered output from the filter_stage-th of the filterbank.
  const Eigen::ArrayXXf& FilteredOutput(int filter_stage) const {
    ABSL_DCHECK(initialized_);

    ABSL_DCHECK_LT(filter_stage, exposed_filter_indices_.size());
    return filtered_output_[exposed_filter_indices_[filter_stage]];
  }

  float GetNumMics() const {
    ABSL_DCHECK(initialized_);
    return num_mics_;
  }

  int GetFilterbankSize() const {
    ABSL_DCHECK(initialized_);
    return exposed_filter_indices_.size();
  }

  float GetSampleRate() const {
    ABSL_DCHECK(initialized_);
    // No decimation is done before the first stage. GetSampleRate(0) is equal
    // to the sample rate that was passed to Init().
    return GetSampleRate(0);
  }

  float GetSampleRate(int stage) const {
    ABSL_DCHECK(initialized_);
    ABSL_DCHECK_LT(stage, exposed_filter_indices_.size());
    return sample_rates_[exposed_filter_indices_[stage]];
  }

  // Accessors are provided for testing.
  const BiquadFilterCascadeCoefficients& GetCoefficients() const {
    return biquad_coefficients_;
  }

  float PeakFrequencyHz(int stage) const {
    ABSL_DCHECK_LT(stage, exposed_filter_indices_.size());
    return peak_frequencies_[exposed_filter_indices_[stage]];
  }

  float BandwidthHz(int stage) const {
    ABSL_DCHECK_LT(stage, exposed_filter_indices_.size());
    return bandwidth_hz_[exposed_filter_indices_[stage]];
  }

 private:
  // Design functions.
  void DesignFilterbank(float sample_rate);

  // Returns the sample rate of the created stage.
  float DesignFilterbankStage(int stage, float stage_sample_rate,
                              float pole_frequency);

  int num_mics_;

  bool initialized_;

  const AuditoryCascadeFilterbankParams params_;
  bool allow_decimation_;

  // Per-channel argmax of the cascade's magnitude response.
  std::vector<float> peak_frequencies_;
  // Per-channel bandwidth of the cascade's response.
  std::vector<float> bandwidth_hz_;

  BiquadFilterCascadeCoefficients biquad_coefficients_;

  // Filter cascade, processed in order, starting with the first.
  std::vector<BiquadFilter<Eigen::ArrayXf>> filters_;

  // The sample rate, in Hz, of each stage of the filterbank.
  std::vector<float> sample_rates_;

  // filtered_output_[i] is the filtered output for the ith stage of the
  // cascade.
  std::vector<Eigen::ArrayXXf> filtered_output_;

  // A filter for taking the first difference of each filterbank channel. One
  // of the main intended use cases of CascadedFilterbank is to make models of
  // the auditory system, which are often implemented as gammatone filterbanks
  // or differentiated all-pole gammatone filters. diff_filters_ optionally
  // provides this differentiation.

  std::vector<FirstDifferenceFilter> diff_filters_;

  // Decimators that are used to reduce the sampling rate.
  std::vector<FactorTwoDecimator> decimators_;

  // A mapping from the indices of filters exposed through the public interface
  // to the bank of filters that are used internally (whether exposed or not).
  std::vector<int> exposed_filter_indices_;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_FILTERBANKS_AUDITORY_CASCADE_FILTERBANK_H_
