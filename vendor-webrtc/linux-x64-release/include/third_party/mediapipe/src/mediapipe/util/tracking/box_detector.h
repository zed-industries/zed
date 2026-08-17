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

#ifndef MEDIAPIPE_UTIL_TRACKING_BOX_DETECTOR_H_
#define MEDIAPIPE_UTIL_TRACKING_BOX_DETECTOR_H_

#include "absl/container/flat_hash_map.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/framework/port/opencv_features2d_inc.h"
#include "mediapipe/util/tracking/box_detector.pb.h"
#include "mediapipe/util/tracking/box_tracker.pb.h"
#include "mediapipe/util/tracking/flow_packager.pb.h"
#include "mediapipe/util/tracking/tracking.h"

namespace mediapipe {

// Feature correspondences between target index and a specific frame.
// The size of `points_frame` and `points_index` should be identical and the
// corresponding elements are a pair of feature correspondence.
struct FeatureCorrespondence {
  // Matched feature locations from an image frame.
  std::vector<cv::Point2f> points_frame;
  // Matched feature locations from the index structure. The location is where
  // it get detected from a previous frame.
  std::vector<cv::Point2f> points_index;
};

// General interface for multiple box detector implementations
class BoxDetectorInterface {
 public:
  // Creates box detector based on index type defined in `options`.
  static std::unique_ptr<BoxDetectorInterface> Create(
      const BoxDetectorOptions &options);

  // Locate quad from feature correspondences using perspective model.
  // Feature locations need to be normalized with 1.0 / max(width, height).
  // `box_proto` contains quad corners position and aspect ratio.
  // `frame_aspect` is the aspect ratio for the camera image frame.
  // Note that to perform pnp tracking, both box aspect ratio and frame aspect
  // ratio need to be positive. Otherwise fallback to homography tracking.
  TimedBoxProtoList FindQuadFromFeatureCorrespondence(
      const FeatureCorrespondence &matches, const TimedBoxProto &box_proto,
      float frame_aspect = -1.0f);

  virtual ~BoxDetectorInterface() = default;

  // Detects pre-set boxes from input frame and adds features from new boxes
  // into detector's index structure. Features and descriptors should be
  // pre-computed and passed within `tracking_data`. `tracked_boxes` contains
  // box tracking results from box_tracker.
  // If all the boxes in the index are currently being tracked (box.id() found
  // in `tracked_boxes`), the detection will be skipped and `detected_boxes`
  // will remain empty.
  // If the box's ID has never been recorded in the index before, The ID and all
  // the features within the box will be merged into the index.
  // `timestamp_msec` should correspond to `tracking_data`.
  void DetectAndAddBox(const TrackingData &tracking_data,
                       const TimedBoxProtoList &tracked_boxes,
                       int64_t timestamp_msec,
                       TimedBoxProtoList *detected_boxes);

  // Detects pre-set boxes from input frame and adds features from new boxes
  // into detector's index structure. Features and descriptors are extracted
  // from `image` in real time.
  // Other parameters work the same way as the previous function.
  // `timestamp_msec` should correspond to `image`.
  void DetectAndAddBox(const cv::Mat &image,
                       const TimedBoxProtoList &tracked_boxes,
                       int64_t timestamp_msec,
                       TimedBoxProtoList *detected_boxes);

  // Stops detection of box with `box_id`.
  void CancelBoxDetection(int box_id);

  // Get the current detector's search index.
  BoxDetectorIndex ObtainBoxDetectorIndex() const;

  // Add detector's search index with pre-defined index.
  void AddBoxDetectorIndex(const BoxDetectorIndex &index);

  // Internal call for public DetectAndAddBox functions. `features` and
  // `descriptors` can be either extracted from live frames or tracked from
  // previous frames. `scale_x` and `scale_y` provides actual image aspect ratio
  // so that boxes from `tracked_boxes` can be denormalized and boxes in
  // `detected_boxes` normalized. `timestamp_msec` should correspond to the
  // timestamp of `features` and `descriptors`.
  void DetectAndAddBoxFromFeatures(const std::vector<Vector2_f> &features,
                                   const cv::Mat &descriptors,
                                   const TimedBoxProtoList &tracked_boxes,
                                   int64_t timestamp_msec, float scale_x,
                                   float scale_y,
                                   TimedBoxProtoList *detected_boxes);

 protected:
  explicit BoxDetectorInterface(const BoxDetectorOptions &options);

  // `transform_features_for_pnp` controls wheather we transform features
  // coordinates into a rectangular target space for pnp detection mode.
  void AddBoxFeaturesToIndex(const std::vector<Vector2_f> &features,
                             const cv::Mat &descriptors,
                             const TimedBoxProto &box,
                             bool transform_features_for_pnp = false);

  // Check if add / detect action will be called based on input `tracked_boxes`.
  bool CheckDetectAndAddBox(const TimedBoxProtoList &tracked_boxes);

  // Returns feature indices that are within the given box. If the box size
  // isn't big enough to cover sufficient features to reacquire the box, this
  // function will try to iteratively enlarge the box size by roughly 5
  // percent of the shorter edge of the image to include more features, but
  // maximimum twice. Note that detected_boxes will still be reported with
  // original size. External users are then freed from specificially finetuning
  // a box size for reacquisition. They should choose suitable box size for
  // tracking based on their use cases.
  std::vector<int> GetFeatureIndexWithinBox(
      const std::vector<Vector2_f> &features, const TimedBoxProto &box);

  // Specifies which box to detect with `box_idx`. This enalbles separately
  // managing the detection behavior for each box in the index. Tracked boxes
  // will be skipped and lost and out-of-view boxes will be detected.
  TimedBoxProtoList DetectBox(const std::vector<Vector2_f> &features,
                              const cv::Mat &descriptors, int box_idx);

  // Only matches those features from the specific box with `box_idx`.
  virtual std::vector<FeatureCorrespondence> MatchFeatureDescriptors(
      const std::vector<Vector2_f> &features, const cv::Mat &descriptors,
      int box_idx) = 0;

  // Specifies which box the correspondences come from with `box_id`, so that we
  // can figure out the transformation accordingly.
  TimedBoxProtoList FindBoxesFromFeatureCorrespondence(
      const std::vector<FeatureCorrespondence> &matches, int box_idx);

  int cnt_detect_called_ = 0;
  float image_scale_;
  float image_aspect_;
  absl::flat_hash_map<int, int> box_id_to_idx_;
  std::vector<int> box_idx_to_id_;
  std::vector<std::vector<TimedBoxProto>> frame_box_;
  std::vector<std::vector<int>> feature_to_frame_;
  std::vector<std::vector<Vector2_f>> feature_keypoints_;
  std::vector<cv::Mat> feature_descriptors_;
  std::vector<bool> has_been_out_of_fov_;
  mutable absl::Mutex access_to_index_;
  cv::Ptr<cv::ORB> orb_extractor_;
  BoxDetectorOptions options_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_BOX_DETECTOR_H_
