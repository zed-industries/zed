// Copyright 2019, 2021 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_CALCULATORS_AUDIO_RATIONAL_FACTOR_RESAMPLE_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_AUDIO_RATIONAL_FACTOR_RESAMPLE_CALCULATOR_H_

#include <algorithm>
#include <cstdint>
#include <memory>
#include <vector>

#include "Eigen/Core"
#include "absl/strings/str_cat.h"
#include "audio/dsp/resampler.h"
#include "mediapipe/calculators/audio/rational_factor_resample_calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/formats/time_series_header.pb.h"
#include "mediapipe/util/time_series_util.h"

namespace mediapipe {
// MediaPipe Calculator for resampling a (vector-valued)
// input time series with a uniform sample rate.  The output
// stream's sampling rate is specified by target_sample_rate in the
// RationalFactorResampleCalculatorOptions.  The output time series may have
// a varying number of samples per frame.
//
// NOTE: This calculator uses QResampler, despite the name, which supersedes
// RationalFactorResampler.
class RationalFactorResampleCalculator : public CalculatorBase {
 public:
  struct TestAccess;

  static absl::Status GetContract(CalculatorContract* cc) {
    cc->Inputs().Index(0).Set<Matrix>(
        // Single input stream with TimeSeriesHeader.
    );
    cc->Outputs().Index(0).Set<Matrix>(
        // Resampled stream with TimeSeriesHeader.
    );
    return absl::OkStatus();
  }
  // Returns FAIL if the input stream header is invalid or if the
  // resampler cannot be initialized.
  absl::Status Open(CalculatorContext* cc) override;
  // Resamples a packet of TimeSeries data.  Returns FAIL if the
  // resampler state becomes inconsistent.
  absl::Status Process(CalculatorContext* cc) override;
  // Flushes any remaining state.  Returns FAIL if the resampler state
  // becomes inconsistent.
  absl::Status Close(CalculatorContext* cc) override;

 protected:
  typedef audio_dsp::Resampler<float> ResamplerType;

  // Returns a Resampler<float> implementation specified by the
  // RationalFactorResampleCalculatorOptions proto. Returns null if the options
  // specify an invalid resampler.
  static std::unique_ptr<ResamplerType> ResamplerFromOptions(
      const double source_sample_rate, const double target_sample_rate,
      const RationalFactorResampleCalculatorOptions& options);

  // Does Timestamp bookkeeping and resampling common to Process() and
  // Close().  Returns FAIL if the resampler state becomes
  // inconsistent.
  absl::Status ProcessInternal(const Matrix& input_frame, bool should_flush,
                               CalculatorContext* cc);

  // Uses the internal resampler_ objects to actually resample each
  // row of the input TimeSeries.  Returns false if the resampler
  // state becomes inconsistent.
  bool Resample(const Matrix& input_frame, Matrix* output_frame,
                bool should_flush);

  double source_sample_rate_;
  double target_sample_rate_;
  int64_t cumulative_input_samples_;
  int64_t cumulative_output_samples_;
  Timestamp initial_timestamp_;
  bool check_inconsistent_timestamps_;
  int num_channels_;
  std::vector<std::unique_ptr<ResamplerType>> resampler_;
};

// Test-only access to RationalFactorResampleCalculator methods.
struct RationalFactorResampleCalculator::TestAccess {
  static std::unique_ptr<ResamplerType> ResamplerFromOptions(
      const double source_sample_rate, const double target_sample_rate,
      const RationalFactorResampleCalculatorOptions& options) {
    return RationalFactorResampleCalculator::ResamplerFromOptions(
        source_sample_rate, target_sample_rate, options);
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_AUDIO_RATIONAL_FACTOR_RESAMPLE_CALCULATOR_H_
