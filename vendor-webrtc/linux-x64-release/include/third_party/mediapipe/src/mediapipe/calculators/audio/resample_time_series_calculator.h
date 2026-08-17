// Copyright 2025 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_AUDIO_RESAMPLE_TIME_SERIES_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_AUDIO_RESAMPLE_TIME_SERIES_CALCULATOR_H_

#include <cstdint>
#include <memory>

#include "absl/status/status.h"
#include "audio/dsp/resampler_q.h"
#include "mediapipe/calculators/audio/resample_time_series_calculator.pb.h"
#include "mediapipe/framework/api2/contract.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/util/time_series_util.h"

namespace mediapipe {

struct ResampleTimeSeriesCalculator : public mediapipe::api2::NodeIntf {
  // Sequence of Matrices, each column describing a particular time frame, each
  // row a feature dimension, with TimeSeriesHeader.
  static constexpr mediapipe::api2::Input<Matrix> kInput{""};
  static constexpr mediapipe::api2::SideInput<double>::Optional
      kSideInputTargetSampleRate{"TARGET_SAMPLE_RATE"};
  // Sequence of Matrices, each column describing a particular time frame, each
  // row a feature dimension, with TimeSeriesHeader.
  static constexpr mediapipe::api2::Output<Matrix> kOutput{""};
  MEDIAPIPE_NODE_INTERFACE(ResampleTimeSeriesCalculator, kInput, kOutput,
                           kSideInputTargetSampleRate,
                           mediapipe::api2::TimestampChange::Arbitrary());
};

// MediaPipe Calculator for resampling a (vector-valued)
// input time series with a uniform sample rate.  The output
// stream's sampling rate is specified by target_sample_rate in the
// ResampleTimeSeriesCalculatorOptions.  The output time series may have
// a varying number of samples per frame.
class ResampleTimeSeriesCalculatorImpl
    : public mediapipe::api2::NodeImpl<ResampleTimeSeriesCalculator,
                                       ResampleTimeSeriesCalculatorImpl> {
 public:
  struct TestAccess;
  static absl::Status UpdateContract(CalculatorContract* cc) {
    return time_series_util::HasOptionsExtension<
        ResampleTimeSeriesCalculatorOptions>(cc->Options());
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

  class ResamplerWrapper {
   public:
    virtual ~ResamplerWrapper() = default;
    virtual bool Valid() const = 0;
    virtual void Resample(const Matrix& input_frame, Matrix* output_frame,
                          bool should_flush) = 0;
  };

  // Wrapper for QResampler.
  class QResamplerWrapper
      : public ResampleTimeSeriesCalculatorImpl::ResamplerWrapper {
   public:
    QResamplerWrapper(double source_sample_rate, double target_sample_rate,
                      int num_channels, audio_dsp::QResamplerParams params)
        : impl_(source_sample_rate, target_sample_rate, num_channels, params) {}

    bool Valid() const override { return impl_.Valid(); }

    void Resample(const Matrix& input_frame, Matrix* output_frame,
                  bool should_flush) override {
      if (should_flush) {
        impl_.Flush(output_frame);
      } else {
        impl_.ProcessSamples(input_frame, output_frame);
      }
    }

   private:
    audio_dsp::QResampler<float> impl_;
  };

 protected:
  // Returns a ResamplerWrapper implementation specified by the
  // ResampleTimeSeriesCalculatorOptions proto. Returns null if the options
  // specify an invalid resampler.
  static std::unique_ptr<ResamplerWrapper> ResamplerFromOptions(
      double source_sample_rate, double target_sample_rate, int num_channels,
      const ResampleTimeSeriesCalculatorOptions& options);

  // Does Timestamp bookkeeping and resampling common to Process() and
  // Close().  Returns FAIL if the resampler state becomes
  // inconsistent.
  absl::Status ProcessInternal(CalculatorContext* cc, const Matrix& input_frame,
                               bool should_flush);

  double source_sample_rate_;
  double target_sample_rate_;
  int64_t cumulative_input_samples_;
  int64_t cumulative_output_samples_;
  Timestamp initial_timestamp_;
  bool check_inconsistent_timestamps_;
  int num_channels_;
  std::unique_ptr<ResamplerWrapper> resampler_;
};

// Test-only access to ResampleTimeSeriesCalculator methods.
struct ResampleTimeSeriesCalculatorImpl::TestAccess {
  static std::unique_ptr<ResamplerWrapper> ResamplerFromOptions(
      double source_sample_rate, double target_sample_rate, int num_channels,
      const ResampleTimeSeriesCalculatorOptions& options) {
    return ResampleTimeSeriesCalculatorImpl::ResamplerFromOptions(
        source_sample_rate, target_sample_rate, num_channels, options);
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_AUDIO_RESAMPLE_TIME_SERIES_CALCULATOR_H_
