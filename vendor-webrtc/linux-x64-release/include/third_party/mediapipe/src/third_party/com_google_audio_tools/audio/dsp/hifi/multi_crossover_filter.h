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

// A filterbank whose bands have the property that their magnitude responses
// sum to unity.

#ifndef AUDIO_DSP_HIFI_MULTI_CROSSOVER_FILTER_H_
#define AUDIO_DSP_HIFI_MULTI_CROSSOVER_FILTER_H_

#include <algorithm>
#include <cmath>
#include <vector>

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/biquad_filter_design.h"
#include "audio/linear_filters/crossover.h"
#include "audio/linear_filters/ladder_filter.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// A filterbank with the property that preserves energy across frequency, i.e.
// the sweep response is flat.
//
// This condition is only true for the default setting of using the
// Linkwitz-Riley crossover type, however, Butterworth crossovers are supported
// for flexibility.
//
// NOTE: At least with this architecture, it is impossible to guarantee perfect
// flatness at all frequencies. Internally, this class ensures that, at
// crossover frequencies, the output between different stages of the filterbank
// will be in phase. As soon as you deviate from those frequencies, your mileage
// may vary (depends on the all pass design). However, for numbers of bands less
// than 5 or so, the amount of deviation in the sweep response's linear
// amplitude is typically less than 1%. As you move to more bands, you may
// see larger deviations. It is challenging to characterize the amount of
// deviation based only on input parameters, so be sure to test carefully if
// the perfect reconstruction property is important for your application.
template <typename ScalarType /* float/double */>
class MultiCrossoverFilter {
 public:
  using ArrayBlockType =
      Eigen::Array<ScalarType, Eigen::Dynamic, Eigen::Dynamic>;
  using SampleBlockType = Eigen::Array<ScalarType, Eigen::Dynamic, 1>;

  MultiCrossoverFilter(int num_bands, int order,
                       linear_filters::CrossoverType type =
                           linear_filters::kLinkwitzRiley)
      : type_(type),
        order_(order),
        num_bands_(num_bands),
        num_bands_with_allpass_(num_bands_ - 2),
        num_stages_(num_bands_ - 1),
        num_channels_(0 /* uninitialized */),
        sample_rate_hz_(0 /* uninitialized */),
        highpass_filters_(num_stages_),
        lowpass_filters_(num_stages_),
        allpass_filters_(num_bands_with_allpass_),
        filtered_output_(num_bands_) {
    ABSL_CHECK_GT(num_bands, 1);
  }

  // crossover_frequencies_hz.size() must equal num_bands - 1 and have
  // monotonically increasing elements.
  void Init(int num_channels, float sample_rate_hz,
            const std::vector<ScalarType>& crossover_frequencies_hz) {
    num_channels_ = num_channels;
    sample_rate_hz_ = sample_rate_hz;

    SetCrossoverFrequenciesInternal(crossover_frequencies_hz, true);
    Reset();
  }

  void Reset() {
    for (auto& stage : highpass_filters_) {
      for (auto& filter : stage) {
        filter.Reset();
      }
    }
    for (auto& stage : lowpass_filters_) {
      for (auto& filter : stage) {
        filter.Reset();
      }
    }
    for (auto& stage : allpass_filters_) {
      for (auto& filter : stage) {
        filter.Reset();
      }
    }
    for (auto& output : filtered_output_) {
      output.setZero();
    }
  }

  // crossover_frequencies_hz.size() must equal num_bands - 1 and have
  // monotonically increasing elements.
  // Interpolation is done in the filters so that audio artifacts are not
  // caused. During interpolation, magnitude responses are not guaranteed to sum
  // to unity.
  void SetCrossoverFrequencies(
      const std::vector<ScalarType>& crossover_frequencies_hz) {
    SetCrossoverFrequenciesInternal(crossover_frequencies_hz, false);
  }

  // Process a block of samples. input is a 2D Eigen array with contiguous
  // column-major data, where the number of rows equals GetNumChannels().
  // A four-way crossover would look like this:
  // Stage:     0        1        2
  // input --> LP_0 ----AP_0-----AP_2-------> Output band 0
  //       \-> HP_0 --> LP_1-----AP_1-------> Output band 1
  //                \-> HP_1 --> LP_2-------> Output band 2
  //                         \-> HP_2-------> Output band 3
  // Note that the high/low pass filters are numbered by stage and the allpass
  // filters are numbered by the filter-global order in which they are designed.
  void ProcessBlock(const ArrayBlockType& input) {
    ABSL_DCHECK_GT(num_bands_, 1);
    const ArrayBlockType* next_in = &input;
    const ArrayBlockType* each_biquad_in = next_in;
    for (int stage = 0; stage < num_stages_; ++stage) {
      {  // The highpass filter processes for the next higher band.
        const int dst_band = stage + 1;
        each_biquad_in = next_in;
        for (auto& filter : highpass_filters_[stage]) {
          filter.ProcessBlock(*each_biquad_in, &filtered_output_[dst_band]);
          each_biquad_in = &filtered_output_[dst_band];
        }
      }
      {  // The low/allpass filters processes for the current band.
        const int dst_band = stage;
        each_biquad_in = next_in;
        for (auto& filter : lowpass_filters_[stage]) {
          filter.ProcessBlock(*each_biquad_in, &filtered_output_[dst_band]);
          each_biquad_in = &filtered_output_[dst_band];
        }
      }
      next_in = &filtered_output_[stage + 1];
    }

    // Process the allpass filters for each band.
    for (int band = 0; band < num_bands_with_allpass_; ++band) {
      for (auto& filter : allpass_filters_[band]) {
        filter.ProcessBlock(filtered_output_[band], &filtered_output_[band]);
      }
    }
  }

  int num_bands() const { return num_bands_; }

  // Filtered output from the filter_stage-th of the filterbank. Channels are
  // ordered by increasing passband frequency.
  const ArrayBlockType& FilteredOutput(int band_number) const {
    ABSL_DCHECK_LT(band_number, num_bands_);
    return filtered_output_[band_number];
  }

  double GetPhaseResponseAt(int band, double frequency_hz) const {
    // NOTE: When GetAllCoefficients() is being called during
    // SetCrossoverFrequenciesInternal(), it being used to fill in the
    // allpass_coeffs_ array, which starts as bypassed filters and eventually
    // becomes full of all pass filters. It is working as intended for this
    // to return different values as this array is filled in.
    return GetCoefficientsForBand(band).PhaseResponseAtFrequency(
          frequency_hz, sample_rate_hz_);
  }

  linear_filters::BiquadFilterCascadeCoefficients GetCoefficientsForBand(
      int band) const {
    linear_filters::BiquadFilterCascadeCoefficients this_band;
    if (band < num_bands_ - 1) {  // Highest band doesn't have a lowpass.
      int stage = band;
      for (int i = 0; i < lowpass_coeffs_[stage].size(); ++i) {
        this_band.AppendBiquad(lowpass_coeffs_[stage][i]);
      }
    }
    // Lowest band doesn't have a highpass.
    for (int stage = 0; stage < band; ++stage) {
      for (int i = 0; i < highpass_coeffs_[stage].size(); ++i) {
        this_band.AppendBiquad(highpass_coeffs_[stage][i]);
      }
    }
    if (band < num_bands_with_allpass_) {
      for (int i = 0; i < allpass_coeffs_[band].size(); ++i) {
        this_band.AppendBiquad(allpass_coeffs_[band][i]);
      }
    }
    return this_band;
  }


  // The returned angle will be between [-pi and pi).
  template <typename T>
  static T WrapAngle(T x) {
    x = std::fmod(x + M_PI, 2 * M_PI);
    if (x < 0) {
      x += 2 * M_PI;
    }
    return x - M_PI;
  }

 private:
  void SetCrossoverFrequenciesInternal(
      const std::vector<ScalarType>& crossover_frequencies_hz, bool initial) {
    ABSL_CHECK_EQ(crossover_frequencies_hz.size(), num_stages_);
    ABSL_CHECK(std::is_sorted(crossover_frequencies_hz.begin(),
                         crossover_frequencies_hz.end()));
    crossover_frequencies_hz_ = crossover_frequencies_hz;
    // Compute the lowpass/highpass filter coefficients.
    allpass_coeffs_.clear();
    // Last two bands do not have an allpass filter, they are necessarily in
    // phase due to the definition of a Linkwitz-Reily crossover.
    allpass_coeffs_.resize(num_bands_with_allpass_);
    lowpass_coeffs_.clear();
    lowpass_coeffs_.resize(num_stages_);
    highpass_coeffs_.clear();
    highpass_coeffs_.resize(num_stages_);
    for (int stage = 0; stage < num_stages_; ++stage) {
      ScalarType frequency_hz = crossover_frequencies_hz_[stage];
      linear_filters::CrossoverFilterDesign crossover(
          type_, order_, frequency_hz, sample_rate_hz_);
      lowpass_coeffs_[stage] = crossover.GetLowpassCoefficients();
      highpass_coeffs_[stage] = crossover.GetHighpassCoefficients();
    }

    // Do phase correction for each crossover frequency for this, and lower
    // bands.
    for (int stage = 1; stage < num_stages_; ++stage) {
      for (int lower_band_index = stage - 1;
           lower_band_index >= 0;
           --lower_band_index) {
        ScalarType frequency_hz = crossover_frequencies_hz_[lower_band_index];
        int higher_band_index = lower_band_index + 1;
        double lower_band_phase =
            GetPhaseResponseAt(lower_band_index, frequency_hz);
        double higher_band_phase =
            GetPhaseResponseAt(higher_band_index, frequency_hz);
        double phase_difference =
            WrapAngle(higher_band_phase - lower_band_phase);
        allpass_coeffs_[lower_band_index].AppendBiquad(
            linear_filters::AllpassBiquadFilterCoefficients(
                sample_rate_hz_, frequency_hz, phase_difference));
      }
    }
    // Remove trivial stages. Note that there will still be stages which
    // include a trivial bypass stage when that is the only stage.
    for (auto& allpass : allpass_coeffs_) {
      allpass.Simplify();
    }

    int num_channels = num_channels_;  // Avoid passing entire "this" to lambda.
    auto InitFilter = [initial, num_channels](
                          linear_filters::BiquadFilterCoefficients& coeffs,
                          LadderFilterType* filter) {
      std::vector<double> k;
      std::vector<double> v;
      coeffs.AsLadderFilterCoefficients(&k, &v);
      if (initial) {
        filter->InitFromLadderCoeffs(num_channels, k, v);
      } else {
        filter->ChangeLadderCoeffs(k, v);
      }
    };

    // Apply the filter coefficients to the filter.
    for (int stage = 0; stage < num_stages_; ++stage) {
      lowpass_filters_[stage].resize(lowpass_coeffs_[stage].size());
      for (int bq = 0; bq < lowpass_coeffs_[stage].size(); ++bq) {
        InitFilter(lowpass_coeffs_[stage][bq], &lowpass_filters_[stage][bq]);
      }
      highpass_filters_[stage].resize(highpass_coeffs_[stage].size());
      for (int bq = 0; bq < highpass_coeffs_[stage].size(); ++bq) {
        InitFilter(highpass_coeffs_[stage][bq], &highpass_filters_[stage][bq]);
      }
    }

    for (int band = 0; band < num_bands_with_allpass_; ++band) {
      // Not all bands have an allpass filter.
      allpass_filters_[band].resize(allpass_coeffs_[band].size());
      for (int biquad = 0; biquad < allpass_coeffs_[band].size(); ++biquad) {
        // Non-trivial all pass stages start at index 1.
        InitFilter(allpass_coeffs_[band][biquad],
                   &allpass_filters_[band][biquad]);
      }
    }
  }

  const linear_filters::CrossoverType type_;
  const int order_;
  const int num_bands_;
  // Last two bands don't have an allpass, they are already in phase.
  const int num_bands_with_allpass_;  // num_bands_ - 2.
  // The number of crossovers required to implement the filterbank. See
  // diagram above ProcessBlock().
  const int num_stages_;
  int num_channels_;
  float sample_rate_hz_;
  std::vector<ScalarType> crossover_frequencies_hz_;
  // The successive crossover stages, processed in order, starting with the
  // first.
  using LadderFilterType = linear_filters::LadderFilter<SampleBlockType>;
  // High/Lowpass filters coefficients are indexed by stage. The lowpass
  // filters are shared across multiple bands.
  std::vector<linear_filters::BiquadFilterCascadeCoefficients> highpass_coeffs_;
  std::vector<linear_filters::BiquadFilterCascadeCoefficients> lowpass_coeffs_;
  // All pass filters are indexed by band. The i-th element in the vector
  // is for the i-th frequency band. Some vector elements may contain only a
  // single bypass biquad (the last two bands), but those allpass filters
  // are not processed in ProcessBlock.
  std::vector<linear_filters::BiquadFilterCascadeCoefficients> allpass_coeffs_;

  // High/Lowpass filters are indexed by stage (each band may see many stages
  // of filters).
  std::vector<std::vector<LadderFilterType>> highpass_filters_;
  std::vector<std::vector<LadderFilterType>> lowpass_filters_;
  // Keeps adjacent bands in phase at the crossover frequencies.
  std::vector<std::vector<LadderFilterType>> allpass_filters_;

  // filtered_output_[i] is the filtered output for the ith stage of the
  // cascade.
  std::vector<ArrayBlockType> filtered_output_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_HIFI_MULTI_CROSSOVER_FILTER_H_
