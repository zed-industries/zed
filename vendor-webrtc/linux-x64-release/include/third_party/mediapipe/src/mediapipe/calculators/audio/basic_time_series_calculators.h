// Copyright 2019 The MediaPipe Authors.
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
//
// Abstract base class for basic MediaPipe calculators that operate on
// TimeSeries streams and don't require any Options protos.
// Subclasses must override ProcessMatrix, and optionally
// MutateHeader.

#ifndef MEDIAPIPE_CALCULATORS_AUDIO_BASIC_TIME_SERIES_CALCULATORS_H_
#define MEDIAPIPE_CALCULATORS_AUDIO_BASIC_TIME_SERIES_CALCULATORS_H_

#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/formats/time_series_header.pb.h"

namespace mediapipe {

class BasicTimeSeriesCalculatorBase : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc);
  absl::Status Open(CalculatorContext* cc) final;
  absl::Status Process(CalculatorContext* cc) final;

 protected:
  // Open() calls this method to mutate the output stream header.  The input
  // to this function will contain a copy of the input stream header, so
  // subclasses that do not need to mutate the header do not need to override
  // it.
  virtual absl::Status MutateHeader(TimeSeriesHeader* output_header);

  // Process() calls this method on each packet to compute the output matrix.
  virtual Matrix ProcessMatrix(const Matrix& input_matrix) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_AUDIO_BASIC_TIME_SERIES_CALCULATORS_H_
