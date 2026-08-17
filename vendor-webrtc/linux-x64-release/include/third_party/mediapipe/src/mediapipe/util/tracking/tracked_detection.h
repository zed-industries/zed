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

#ifndef MEDIAPIPE_UTIL_TRACKING_TRACKED_DETECTION_H_
#define MEDIAPIPE_UTIL_TRACKING_TRACKED_DETECTION_H_

#include <cstdint>
#include <string>

#include "absl/container/node_hash_map.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/framework/port/vector.h"

namespace mediapipe {

// Class for keeping records of detected objects.
// Each detection should be set an id to uniquely identify it.
class TrackedDetection {
 public:
  TrackedDetection() = delete;
  // Creates a detection that has a bounding box occupying the entire image.
  TrackedDetection(int unique_id, int64_t timestamp)
      : unique_id_(unique_id),
        initial_timestamp_(timestamp),
        last_updated_timestamp_(timestamp),
        previous_id_(-1) {}

  // Creates a detection with the specified bounding box normalized by image
  // dimensions.
  TrackedDetection(int unique_id, int64_t timestamp,
                   const ::mediapipe::NormalizedRect& bounding_box)
      : unique_id_(unique_id),
        initial_timestamp_(timestamp),
        last_updated_timestamp_(timestamp),
        previous_id_(-1) {
    bounding_box_.CopyFrom(bounding_box);
    set_bounds();
  }

  // Adds a (label, score) pair. If the label already exists then keep the
  // higher score of both entries.
  void AddLabel(const std::string& label, float score);

  // Checks if two detections are in fact the same object.
  // TODO: This is just the first step. It will be expand to a family
  // of functionalities that do reconciliation.
  bool IsSameAs(const TrackedDetection& other, float max_area_ratio = 3.0f,
                float min_overlap_ratio = 0.5f) const;

  // Merges labels and score from another TrackedDetection.
  void MergeLabelScore(const TrackedDetection& other);

  int64_t initial_timestamp() const { return initial_timestamp_; }
  int64_t last_updated_timestamp() const { return last_updated_timestamp_; }
  void set_last_updated_timestamp(int64_t timestamp) {
    last_updated_timestamp_ = timestamp;
  }

  int unique_id() const { return unique_id_; }

  int previous_id() const { return previous_id_; }
  void set_previous_id(int id) { previous_id_ = id; }

  const ::mediapipe::NormalizedRect& bounding_box() const {
    return bounding_box_;
  }
  float left() const { return left_; }
  float right() const { return right_; }
  float top() const { return top_; }
  float bottom() const { return bottom_; }

  void set_bounding_box(const ::mediapipe::NormalizedRect& bounding_box) {
    bounding_box_ = bounding_box;
    set_bounds();
  }

  // Gets corners of the bounding box of the object in image coordinates.
  std::array<Vector2_f, 4> GetCorners(float width, float height) const;

  // Gets corners of the bounding box of the object in normalized coordinates.
  std::array<Vector2_f, 4> GetCorners() const { return GetCorners(1.0f, 1.0f); }

  const absl::node_hash_map<std::string, float>& label_to_score_map() const {
    return label_to_score_map_;
  }

 private:
  void set_bounds() {
    left_ = bounding_box_.x_center() - bounding_box_.width() / 2.f;
    right_ = bounding_box_.x_center() + bounding_box_.width() / 2.f;
    top_ = bounding_box_.y_center() - bounding_box_.height() / 2.f;
    bottom_ = bounding_box_.y_center() + bounding_box_.height() / 2.f;
  }

  int unique_id_;

  ::mediapipe::NormalizedRect bounding_box_;
  float left_ = 0.f;
  float right_ = 0.f;
  float top_ = 0.f;
  float bottom_ = 0.f;

  // The timestamp in milliseconds when this object is initialized.
  int64_t initial_timestamp_;
  // The latest timestamp when this object is updated.
  int64_t last_updated_timestamp_;

  // The id of the previous detection that this detection is associated with.
  int previous_id_;

  // Each detection could have multiple labels, each with a corresponding
  // detection score.
  absl::node_hash_map<std::string, float> label_to_score_map_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_TRACKED_DETECTION_H_
