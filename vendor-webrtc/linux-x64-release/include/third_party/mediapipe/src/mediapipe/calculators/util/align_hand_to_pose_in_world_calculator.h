// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_ALIGN_HAND_TO_POSE_IN_WORLD_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_ALIGN_HAND_TO_POSE_IN_WORLD_CALCULATOR_H_

#include "mediapipe/calculators/util/align_hand_to_pose_in_world_calculator.pb.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe::api2 {

// A calculator to align hand world landmarks with pose world landmarks.
//
// When `mediapipe.aimatter.LandmarksDetector` projects world landmarks from ROI
// local coordinates to original scene coordinates, it applies only rotation
// (derived from ROI) but neither scale nor translation. This calculator
// utilizes pose semantic to compensate this lack of information:
//   - Translation is determined by matching wrist from hand landmarks with
//     wrist from pose landmarks.
//   - Scale can be determined (but is not at the moment) by calculating
//     expected hand size from pose landmarks proportions.
//
// Input:
//   HAND_LANDMARKS - LandmarkList
//     Hand world landmarks.
//   POSE_LANDMARKS - LandmarkList
//     Pose world landmarks.
//
// Output:
//   HAND_LANDMARKS - LandmarkList
//     Aligned hand world landmarks.
//
// Example:
//   node {
//     calculator: "AlignHandToPoseInWorldCalculator"
//     input_stream: "HAND_LANDMARKS:hand_world_landmarks"
//     input_stream: "POSE_LANDMARKS:pose_world_landmarks"
//     output_stream: "HAND_LANDMARKS:hand_world_landmarks"
//     options: {
//       [mediapipe.AlignHandToPoseInWorldCalculatorOptions.ext] {
//         hand_wrist_idx: 0
//         pose_wrist_idx: 15    # 16 for right
//       }
//     }
//   }
class AlignHandToPoseInWorldCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::LandmarkList> kInHandLandmarks{
      "HAND_LANDMARKS"};
  static constexpr Input<mediapipe::LandmarkList> kInPoseLandmarks{
      "POSE_LANDMARKS"};
  static constexpr Output<mediapipe::LandmarkList> kOutHandLandmarks{
      "HAND_LANDMARKS"};
  MEDIAPIPE_NODE_INTERFACE(::mediapipe::AlignHandToPoseInWorldCalculator,
                           kInHandLandmarks, kInPoseLandmarks,
                           kOutHandLandmarks);
};

}  // namespace mediapipe::api2

#endif  // MEDIAPIPE_CALCULATORS_UTIL_ALIGN_HAND_TO_POSE_IN_WORLD_CALCULATOR_H_
