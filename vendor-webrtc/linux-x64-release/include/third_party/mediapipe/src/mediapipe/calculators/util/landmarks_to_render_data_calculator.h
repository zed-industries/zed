// Copyright 2020 The MediaPipe Authors.
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
#ifndef MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TO_RENDER_DATA_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TO_RENDER_DATA_CALCULATOR_H_

#include "absl/memory/memory.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "mediapipe/calculators/util/landmarks_to_render_data_calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/calculator_options.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/location_data.pb.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/util/color.pb.h"
#include "mediapipe/util/render_data.pb.h"
namespace mediapipe {

// A calculator that converts Landmark proto to RenderData proto for
// visualization. The input should be LandmarkList proto. It is also possible
// to specify the connections between landmarks.
//
// Example config:
// node {
//   calculator: "LandmarksToRenderDataCalculator"
//   input_stream: "NORM_LANDMARKS:landmarks"
//   output_stream: "RENDER_DATA:render_data"
//   options {
//     [LandmarksToRenderDataCalculatorOptions.ext] {
//       landmark_connections: [0, 1, 1, 2]
//       landmark_color { r: 0 g: 255 b: 0 }
//       connection_color { r: 0 g: 255 b: 0 }
//       thickness: 4.0
//     }
//   }
// }
class LandmarksToRenderDataCalculator : public CalculatorBase {
 public:
  LandmarksToRenderDataCalculator() {}
  ~LandmarksToRenderDataCalculator() override {}
  LandmarksToRenderDataCalculator(const LandmarksToRenderDataCalculator&) =
      delete;
  LandmarksToRenderDataCalculator& operator=(
      const LandmarksToRenderDataCalculator&) = delete;

  static absl::Status GetContract(CalculatorContract* cc);

  absl::Status Open(CalculatorContext* cc) override;

  absl::Status Process(CalculatorContext* cc) override;

 protected:
  ::mediapipe::LandmarksToRenderDataCalculatorOptions options_;
  std::vector<int> landmark_connections_;
};

}  // namespace mediapipe
#endif  // MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TO_RENDER_DATA_CALCULATOR_H_
