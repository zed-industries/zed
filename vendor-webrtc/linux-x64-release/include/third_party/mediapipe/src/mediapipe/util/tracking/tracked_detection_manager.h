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

#ifndef MEDIAPIPE_UTIL_TRACKING_DETECTION_MANAGER_H_
#define MEDIAPIPE_UTIL_TRACKING_DETECTION_MANAGER_H_

#include <cstdint>

#include "absl/container/node_hash_map.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/util/tracking/tracked_detection.h"
#include "mediapipe/util/tracking/tracked_detection_manager_config.pb.h"

namespace mediapipe {

// Class for managing detections.
// The detections are from either server side or a mobile detector and are
// tracked using either 2D or 3D tracker. The TrackedDetectionManager is used to
// identify duplicated detections or obsolete detections to keep a set of
// active detections.
class TrackedDetectionManager {
 public:
  TrackedDetectionManager() = default;
  ~TrackedDetectionManager() = default;

  // Adds a new detection at a given timestamp. Returns the IDs of the
  // detections that are removed due to duplication.
  std::vector<int> AddDetection(std::unique_ptr<TrackedDetection> detection);

  // Updates the location of a detection identified by its id.
  // Returns false if the requested id doesn't exist a corresponding
  // detection. Returns the IDs of the detections that are removed due to
  // duplication.
  std::vector<int> UpdateDetectionLocation(
      int id, const ::mediapipe::NormalizedRect& bounding_box,
      int64_t timestamp);

  // Removes detections that are not updated after |timestamp|. Returns the IDs
  // of the detections that are removed.
  std::vector<int> RemoveObsoleteDetections(int64_t timestamp);

  // TODO: Do we really need this? Pursuit tracker doesn't do well
  // in loop closure. Boxes out of view are usually not attached to objects
  // any more when it's in view again. Returns the IDs of the
  // detections that are removed.
  std::vector<int> RemoveOutOfViewDetections();

  int GetNumDetections() const { return detections_.size(); }

  // Get TrackedDetection by its unique id.
  const TrackedDetection* GetTrackedDetection(int id) const {
    auto detection_ptr = detections_.find(id);
    if (detection_ptr == detections_.end()) {
      return nullptr;
    }
    return detection_ptr->second.get();
  }

  const absl::node_hash_map<int, std::unique_ptr<TrackedDetection>>&
  GetAllTrackedDetections() const {
    return detections_;
  }

  void SetConfig(const mediapipe::TrackedDetectionManagerConfig& config) {
    config_ = config;
  }

 private:
  // Finds all detections that are duplicated with the one of |id| and remove
  // all detections except the one that is added most recently. Returns the IDs
  // of the detections that are removed.
  std::vector<int> RemoveDuplicatedDetections(int id);

  absl::node_hash_map<int, std::unique_ptr<TrackedDetection>> detections_;

  mediapipe::TrackedDetectionManagerConfig config_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_DETECTION_MANAGER_H_
