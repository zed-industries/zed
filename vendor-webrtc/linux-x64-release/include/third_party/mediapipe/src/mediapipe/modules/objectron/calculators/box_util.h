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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_UTIL_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_UTIL_H_

#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/util/tracking/box_tracker.pb.h"

namespace mediapipe {

// This function fills the geometry of the TimedBoxProto. Id, timestamp etc.
// need to be set outside this function.
void ComputeBoundingRect(const std::vector<cv::Point2f>& points,
                         mediapipe::TimedBoxProto* box);

// This function computes the intersection over union between two boxes.
float ComputeBoxIoU(const TimedBoxProto& box1, const TimedBoxProto& box2);

// Computes corners of the box.
// width and height are image width and height, which is typically
// needed since the box is in normalized coordinates.
std::vector<cv::Point2f> ComputeBoxCorners(const TimedBoxProto& box,
                                           float width, float height);

// Computes the perspective transform from box1 to box2.
// The input argument aspect_ratio is width / height of the image.
// The returned matrix should be a 3x3 matrix.
cv::Mat PerspectiveTransformBetweenBoxes(const TimedBoxProto& src_box,
                                         const TimedBoxProto& dst_box,
                                         const float aspect_ratio);

// Map point according to source and destination box location.
cv::Point2f MapPoint(const TimedBoxProto& src_box, const TimedBoxProto& dst_box,
                     const cv::Point2f& src_point, float width, float height);

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_UTIL_H_
