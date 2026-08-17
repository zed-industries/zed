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
// Computes MotionSaliency points that can be used for stabilization and
// retargeting.

#ifndef MEDIAPIPE_UTIL_TRACKING_MOTION_SALIENCY_H_
#define MEDIAPIPE_UTIL_TRACKING_MOTION_SALIENCY_H_

#include <utility>
#include <vector>

#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/motion_saliency.pb.h"
#include "mediapipe/util/tracking/region_flow.h"

namespace mediapipe {
class RegionFlowFeatureList;
class RegionFlowFrame;
class SalientPointFrame;
}  // namespace mediapipe

namespace mediapipe {

class MotionSaliency {
 public:
  MotionSaliency(const MotionSaliencyOptions& options, int frame_width,
                 int frame_height);
  ~MotionSaliency();

  // Finds modes in the RegionFlowFeatureList (clusters for high IRLS weight,
  // per default features agreeing with the background motion).
  // Optionally, per feature irls weights can be supplied instead of using the
  // features weight to adapt modes that will be found, e.g. see
  // ForegroundWeightsFromFeatures below.
  void SaliencyFromFeatures(const RegionFlowFeatureList& feature_list,
                            std::vector<float>* irls_weights,  // optional.
                            SalientPointFrame* salient_frame);

  // Finds saliency points (modes) from a list of points and their respective
  // weights, outputs a SalientPointFrame.
  void SaliencyFromPoints(const std::vector<Vector2_f>* points,
                          const std::vector<float>* weights,
                          SalientPointFrame* salient_frame);

  // Selects saliency inliers, by searching for close-by salient points
  // (within fractional MotionSaliencyOptions::filtering_support_distance)
  // across adjacent frames (considered are
  // #MotionSaliencyOptions::filtering_frame_radius before and after the
  // current frame).
  // If at least #MotionSaliencyOptions::filtering_minimum_support
  // supporting points are found the tested salient point is kept, otherwise
  // discarded.
  // If desired performs rescaling, such that the median salient point weight
  // equals MotionSaliencyOptions::saliency_weight().
  void SelectSaliencyInliers(std::vector<SalientPointFrame*>* motion_saliency,
                             bool rescale_to_median_saliency_weight);

  // Averages all salient points (unweighted average) per frame. The resulting
  // mean salient point is assigned weight one, and the specified normalized
  // bounds (as tuple left, bottom, right, top).
  void CollapseMotionSaliency(const SaliencyPointList& input_saliency,
                              const Vector4_f& bounds,
                              SaliencyPointList* output_saliency);

  // Smooths saliency in space and time.
  void FilterMotionSaliency(
      std::vector<SalientPointFrame*>* saliency_point_list);

  // Aggregates location in image domain and salient weight.
  struct SalientLocation {
    SalientLocation() {}
    SalientLocation(const Vector2_f& _pt, float _weight)
        : pt(_pt), weight(_weight) {}
    Vector2_f pt;
    float weight = 0;
  };

 private:
  // Locates modes in a set of SalientLocation's.
  // (using mean shift with bilateral weights, i.e. weight * spatial
  // gaussian weighting).
  // Only modes with for which the sum of total saliency weight is
  // above min_irls_mode_sum are returned.
  // Returns modes in the image domain as 2D points, sum of their
  // assignment weights and spatial extend along major and minor axis.
  // Modes are sorted w.r.t. their assignment irls weights (from highest to
  // lowest).
  struct SalientMode {
    Vector2_f location;
    // Total sum of irls weights assigned to this mode.
    float assignment_weight = 0;
    // Magnitude of major and minor axis storred in x and y, respectively.
    Vector2_f axis_magnitude;
    // Angle in radians w.r.t. x-axis.
    float angle = 0;
  };

  // Note: input vector locations is not mutated by function.
  void SalientModeFinding(std::vector<SalientLocation>* locations,
                          std::vector<SalientMode>* modes);

  // Determines the salient frame for a list of SalientLocations by performing
  // mode finding and scaling each point based on frame size.
  void DetermineSalientFrame(std::vector<SalientLocation> locations,
                             SalientPointFrame* salient_frame);

  MotionSaliencyOptions options_;
  int frame_width_;
  int frame_height_;
};

// Returns foregroundness weights in [0, 1] for each feature, by mapping irls
// weight to foreground score in [0, 1].
// In particular, the foreground threshold indicates the *inverse* registration
// error (i.e. the irls weight) that is deemed a complete inlier.
// Weights in the interval [0, foreground_threshold] (corresponding to
// pixel errors in the interval [1 / foreground_threshold, inf])
// are mapped to 1 - [0, 1], i.e. foreground threshold is mapped to zero
// with weights below the threshold being assigned values > 0.
// Therefore, larger values will increase amount of detected foreground
// as well as noise.
// In addition, foreground_gamma's < 1 can be used to increase the resolution
// of small foreground motions (irls weight close to the foreground_threshold)
// at the expense of larger foreground motions (irls weight close to zero).
// If optional parameter camera_motion is specified, the passed foreground
// threshold is scaled by the InlierCoverage of the camera_motion
// (which is in 0, 1). That is for unstable frames with small coverage,
// the threshold is tighter and fewer features are considered foreground.
void ForegroundWeightsFromFeatures(
    const RegionFlowFeatureList& feature_list,
    float foreground_threshold,         // 0.5 is a good default value.
    float foreground_gamma,             // use 1.0 for default
    const CameraMotion* camera_motion,  // optional, can be nullptr.
    std::vector<float>* weights);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_MOTION_SALIENCY_H_
