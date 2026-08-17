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

#ifndef MEDIAPIPE_UTIL_TRACKING_TRACKING_VISUALIZATION_UTILITIES_H_
#define MEDIAPIPE_UTIL_TRACKING_TRACKING_VISUALIZATION_UTILITIES_H_

#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/util/tracking/box_tracker.pb.h"
#include "mediapipe/util/tracking/flow_packager.pb.h"
#include "mediapipe/util/tracking/tracking.pb.h"

namespace mediapipe {

// Visualizes state to frame. Also overlays statistics if print_stats is set.
void RenderState(const MotionBoxState& box_state, bool print_stats,
                 cv::Mat* frame);

// Visualizes internal tracking state to frame.
void RenderInternalState(const MotionBoxInternalState& internal,
                         cv::Mat* frame);

// Visualizes tracking data (specifically, the motion_vectors) to frame.
// Optional third parameter can be used to disable antialiasing.
void RenderTrackingData(const TrackingData& data, cv::Mat* mat,
                        bool antialiasing = false);

// Visualize TimeBoxProto onto image.
void RenderBox(const TimedBoxProto& box_proto, cv::Mat* mat);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_TRACKING_VISUALIZATION_UTILITIES_H_
