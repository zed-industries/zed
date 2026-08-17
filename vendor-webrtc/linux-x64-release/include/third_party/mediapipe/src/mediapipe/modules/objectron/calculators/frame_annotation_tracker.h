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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_FRAME_ANNOTATION_TRACKER_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_FRAME_ANNOTATION_TRACKER_H_

#include <cstdint>
#include <functional>

#include "absl/container/btree_map.h"
#include "absl/container/flat_hash_set.h"
#include "mediapipe/modules/objectron/calculators/annotation_data.pb.h"
#include "mediapipe/util/tracking/box_tracker.pb.h"

namespace mediapipe {

class FrameAnnotationTracker {
 public:
  // If two bounding boxes have IoU over iou_threshold, then we consider them
  // describing the same object.
  FrameAnnotationTracker(float iou_threshold, float img_width, float img_height)
      : iou_threshold_(iou_threshold),
        img_width_(img_width),
        img_height_(img_height) {}

  // Adds detection results from an external detector.
  void AddDetectionResult(const FrameAnnotation& frame_annotation);

  // Consolidates tracking result from an external tracker, associates with
  // the detection result by the object id, and produces the corresponding
  // result in FrameAnnotation. When there are duplicates, output the ids that
  // need to be cancelled in cancel_object_ids.
  // Note that the returned FrameAnnotation is missing timestamp. Need to fill
  // that field.
  FrameAnnotation ConsolidateTrackingResult(
      const TimedBoxProtoList& tracked_boxes,
      absl::flat_hash_set<int>* cancel_object_ids);

 private:
  float iou_threshold_;
  float img_width_;
  float img_height_;
  // Cached detection results over time.
  // Key is timestamp_us + object_id.
  absl::btree_map<int64_t, ObjectAnnotation, std::greater<int64_t>>
      detected_objects_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_FRAME_ANNOTATION_TRACKER_H_
