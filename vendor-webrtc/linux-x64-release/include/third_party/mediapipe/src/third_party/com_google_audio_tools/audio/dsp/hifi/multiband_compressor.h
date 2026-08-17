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

// Dynamic range control that can be controlled on a per-frequency-band.
#ifndef AUDIO_DSP_HIFI_MULTIBAND_COMPRESSOR_H_
#define AUDIO_DSP_HIFI_MULTIBAND_COMPRESSOR_H_

#include "audio/dsp/hifi/dynamic_range_control.h"
#include "audio/dsp/hifi/multi_crossover_filter.h"
#include "audio/linear_filters/crossover.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

class MultibandCompressorParams {
 public:
  // The default constructor is useful for construction in a larger container
  // but is insufficient for use in the MultibandCompressor.
  MultibandCompressorParams()
    : num_bands_(0 /* Uninitialized*/) {}

  // Note that frequencies_hz.size() must have num_bands - 1 elements.
  // Compression tuning is very application specific, so you should set
  // the compressor params for each stage before using this. See
  // SetDynamicRangeControlParams() below.
  MultibandCompressorParams(
      int num_bands,
      const std::vector<float>& frequencies_hz,
      int crossover_order = 4,
      linear_filters::CrossoverType crossover_type =
          linear_filters::kLinkwitzRiley)
      : num_bands_(num_bands),
        crossover_order_(crossover_order),
        crossover_type_(crossover_type),
        drc_params_(num_bands) {
    ABSL_CHECK_GT(num_bands_, 1)
        << "Just use DynamicRangeControl if you only have one band.";
    ABSL_CHECK_GE(crossover_order, 2);
    SetCrossoverFrequencies(frequencies_hz);
  }

  // This sets all stages to the same lookahead time; if you want different
  // lookahead times for different stages, change
  // MutableDynamicRangeControlParams(stage_index)->lookahead_s.
  void SetLookaheadSeconds(float lookahead_s) {
    for (DynamicRangeControlParams& params : drc_params_) {
      params.lookahead_s = lookahead_s;
    }
  }

  void SetCrossoverFrequencies(const std::vector<float>& frequencies_hz) {
    ABSL_CHECK_EQ(frequencies_hz.size(), num_bands_ - 1);
    crossover_frequencies_hz_ = frequencies_hz;
  }

  const std::vector<float>& GetCrossoverFrequencies() const {
    return crossover_frequencies_hz_;
  }

  // stage must be less than to num_bands_.
  DynamicRangeControlParams* MutableDynamicRangeControlParams(int stage) {
    ABSL_CHECK_GE(stage, 0);
    ABSL_CHECK_LT(stage, num_bands_);
    return &drc_params_[stage];
  }

  const DynamicRangeControlParams& GetDynamicRangeControlParams(int stage)
      const {
    ABSL_CHECK_GE(stage, 0);
    ABSL_CHECK_LT(stage, num_bands_);
    return drc_params_[stage];
  }

  const std::vector<DynamicRangeControlParams>& GetDynamicRangeControlParams()
      const {
    return drc_params_;
  }

  int num_bands() const {
    return num_bands_;
  }

  int crossover_order() const {
    return crossover_order_;
  }

  linear_filters::CrossoverType crossover_type() const {
    return crossover_type_;
  }

 private:
  // Parameters that are fixed at initialization time.
  int num_bands_;
  int crossover_order_;
  linear_filters::CrossoverType crossover_type_;

  // Parameters that are able to be changed on the fly.
  std::vector<float> crossover_frequencies_hz_;
  std::vector<DynamicRangeControlParams> drc_params_;
};

class MultibandCompressor {
 public:
  explicit MultibandCompressor(const MultibandCompressorParams& params);

  void Init(int num_channels, int max_block_size_samples, float sample_rate_hz);

  void Reset();

  // crossover_frequencies_hz.size() must equal num_bands - 1 and have
  // monotonically increasing elements.
  // Interpolation is done in the filters so that audio artifacts are not
  // caused. During interpolation, magnitude responses are not guaranteed to sum
  // to unity.
  void SetCrossoverFrequencies(
      const std::vector<float>& crossover_frequencies_hz) {
    band_splitter_.SetCrossoverFrequencies(crossover_frequencies_hz);
  }

  void SetDynamicRangeControlParams(int stage,
                                    const DynamicRangeControlParams& params) {
    ABSL_CHECK_GE(stage, 0);
    ABSL_CHECK_LT(stage, per_band_drc_.size());
    per_band_drc_[stage].SetDynamicRangeControlParams(params);
  }

  // Process a block of samples. input is a 2D Eigen array with contiguous
  // column-major data, where the number of rows equals GetNumChannels().
  void ProcessBlock(const Eigen::ArrayXXf& input, Eigen::ArrayXXf* output);
  void ProcessBlock(Eigen::Map<const Eigen::ArrayXXf> input,
                    Eigen::Map<Eigen::ArrayXXf> output);

 private:
  int num_channels_;
  float sample_rate_hz_;
  std::vector<float> crossover_frequencies_hz_;

  MultiCrossoverFilter<float> band_splitter_;
  // Gain control for each band.
  std::vector<DynamicRangeControl> per_band_drc_;

  Eigen::ArrayXXf workspace_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_HIFI_MULTIBAND_COMPRESSOR_H_
