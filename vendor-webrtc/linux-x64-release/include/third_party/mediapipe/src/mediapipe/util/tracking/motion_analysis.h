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
// Module for performing motion analysis on a video stream including computing
// locally filtered (robust) feature tracking, camera motion estimation, and
// dense foreground saliency estimation.
// Module buffers frames internally (using an adaptive overlap to achieve
// temporal consistency):
//
// Usage example:
//
// MotionAnalysisOptions options();
// // Should be always be a multiple 8 for optimal parallel performance
// options.set_estimation_clip_size(16);
// MotionAnalysis motion_analysis(options, 960, 540);

// std::vector<cv::Mat> input_frames(N);
// // Define output vectors.
// std::vector<std::unique_ptr<RegionFlowFeatureList>> features;
// std::vector<std::unique_ptr<CameraMotion>> camera_motion;
// std::vector<std::unique_ptr<SalientPointFrame>> saliency;
// std::vector<cv::Mat> rendered_results);   // Should to be initialized with
//                                           // frame.
//
// for (int k = 0; k < N; ++k) {
//   motion_analysis.AddFrame(input_frames[k], 0);
//   // Outputs results, if new ones are available.
//   // Output will be all of the same lengths (Length returned by function).
//   if (motion_analysis.GetResults(k + 1 == N,         // Flush, force output.
//                                  &features,
//                                  &camera_motion,
//                                  &saliency) > 0) {            // Optional.
//      // Optionally render at i'th frame.
//      motion_analysis.RenderResults(*features[i],
//                                    *camera_motion[i],
//                                    saliency[i].get(),
//                                    &rendered_results[i]);
//
//      // Output results...
//    }
// }

#ifndef MEDIAPIPE_UTIL_TRACKING_MOTION_ANALYSIS_H_
#define MEDIAPIPE_UTIL_TRACKING_MOTION_ANALYSIS_H_

#include <memory>
#include <string>
#include <vector>

#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/util/tracking/camera_motion.pb.h"
#include "mediapipe/util/tracking/motion_analysis.pb.h"
#include "mediapipe/util/tracking/motion_estimation.h"
#include "mediapipe/util/tracking/motion_estimation.pb.h"
#include "mediapipe/util/tracking/motion_saliency.h"
#include "mediapipe/util/tracking/push_pull_filtering.h"
#include "mediapipe/util/tracking/region_flow.h"
#include "mediapipe/util/tracking/region_flow.pb.h"
#include "mediapipe/util/tracking/region_flow_computation.h"
#include "mediapipe/util/tracking/streaming_buffer.h"

namespace mediapipe {

typedef PushPullFiltering<1, FilterWeightMultiplierOne> PushPullFlowC1;

class MotionAnalysis {
 public:
  MotionAnalysis(const MotionAnalysisOptions& options, int frame_width,
                 int frame_height);
  ~MotionAnalysis() = default;
  MotionAnalysis(const MotionAnalysis&) = delete;
  MotionAnalysis& operator=(const MotionAnalysis&) = delete;

  // Call with every frame. Timestamp is optional (set to zero if not needed).
  // Optionally outputs list of features extracted from this frame.
  // Returns true on success.
  bool AddFrame(const cv::Mat& frame, int64_t timestamp_usec,
                RegionFlowFeatureList* feature_list = nullptr);

  // Same as above, but uses specified initial transform to seed
  // feature locations.
  bool AddFrameWithSeed(const cv::Mat& frame, int64_t timestamp_usec,
                        const Homography& initial_transform,
                        RegionFlowFeatureList* feature_list = nullptr);

  // Generic function to perform motion analysis on the passed frame,
  // with initial_transform used as seed.
  // Optionally accepts external_feature to be added to the computed ones,
  // and to reject all features that do not agree with the rejection transform
  // within a specified threshold (rejection_transform_threshold in options).
  // Also allows to modify feature locations before motion estimation by
  // supplying appropiate callback - this is invoked *after* the rejection
  // transform.
  // Returns list of features extracted from this frame, *before* any
  // modification is applied. To yield modified features, simply
  // apply modify_features function to returned result.
  bool AddFrameGeneric(
      const cv::Mat& frame, int64_t timestamp_usec,
      const Homography& initial_transform,
      const Homography* rejection_transform = nullptr,
      const RegionFlowFeatureList* external_features = nullptr,
      std::function<void(RegionFlowFeatureList*)>* modify_features = nullptr,
      RegionFlowFeatureList* feature_list = nullptr);

  // Instead of tracking passed frames, uses result directly as supplied by
  // features. Can not be mixed with above AddFrame* calls.
  void AddFeatures(const RegionFlowFeatureList& features);

  // Instead of tracking and computing camera motions, simply adds precomputed
  // features and camera motions to the internal buffers. Can not be mixed
  // with above Add* calls.
  // This is useful for just computing saliency via GetResults.
  void EnqueueFeaturesAndMotions(const RegionFlowFeatureList& features,
                                 const CameraMotion& motion);

  // Returns motion results (features, camera motions and saliency, all
  // optional).
  // Call after every AddFrame for optimal performance.
  // Returns number of available results. Note, this call with often return
  // zero, and only return results (multiple in this case) when chunk boundaries
  // are reached. The actual number returned depends on various smoothing
  // settings for saliency and features.
  // Set flush to true, to force output of all results (e.g. when the end of the
  // video stream is reached).
  // Note: Passing a non-zero argument for saliency, requires
  // MotionAnalysisOptions::compute_motion_saliency to be set and
  // vice versa. (CHECKED)
  int GetResults(
      bool flush,  // Forces output.
      std::vector<std::unique_ptr<RegionFlowFeatureList>>* features = nullptr,
      std::vector<std::unique_ptr<CameraMotion>>* camera_motion = nullptr,
      std::vector<std::unique_ptr<SalientPointFrame>>* saliency = nullptr);

  // Exposes the grayscale image frame from the most recently created region
  // flow tracking data.
  cv::Mat GetGrayscaleFrameFromResults();

  // Renders features and saliency to rendered_results based on
  // VisualizationOptions onto pre-initialized rendered_results (in most cases
  // you want to create a copy of the input frame).
  // NOTE:
  // If features are requested to be rendered, this function should be
  // called serially with each frame, or wrong feature location might be
  // rendered.
  void RenderResults(const RegionFlowFeatureList& features,
                     const CameraMotion& camera_motion,
                     const SalientPointFrame* saliency,  // Optional.
                     cv::Mat* rendered_results);

  // Determines dense foreground mask from features.
  // Returns foreground mask as CV_8U image, indicating propability of
  // foreground.
  void ComputeDenseForeground(const RegionFlowFeatureList& feature_list,
                              const CameraMotion& camera_motion,
                              cv::Mat* foreground_mask);

  // Overlays foreground mask over output as green burn in, or with
  // jet_coloring (based on options).
  void VisualizeDenseForeground(const cv::Mat& foreground_mask,
                                cv::Mat* output);

  // Masks out regions from input that are not used for blur analysis.
  void VisualizeBlurAnalysisRegions(cv::Mat* input);

  // Number of frames/features added so far.
  int NumFrames() const { return frame_num_; }

 private:
  void InitPolicyOptions();

  // Compute saliency from buffered features and motions.
  void ComputeSaliency();

  // Outputs computed results from the streaming buffer to the optional
  // output args. Also performs overlap handling.
  int OutputResults(
      bool flush,  // Forces output.
      std::vector<std::unique_ptr<RegionFlowFeatureList>>* features = nullptr,
      std::vector<std::unique_ptr<CameraMotion>>* camera_motion = nullptr,
      std::vector<std::unique_ptr<SalientPointFrame>>* saliency = nullptr);

  MotionAnalysisOptions options_;
  int frame_width_ = 0;
  int frame_height_ = 0;
  int frame_num_ = 0;

  // Internal objects for actual motion analysis.
  std::unique_ptr<RegionFlowComputation> region_flow_computation_;
  std::unique_ptr<MotionEstimation> motion_estimation_;
  std::unique_ptr<MotionSaliency> motion_saliency_;
  std::unique_ptr<PushPullFlowC1> foreground_push_pull_;
  // Used for visualization if long feature tracks are present.
  std::unique_ptr<LongFeatureStream> long_feature_stream_;

  std::unique_ptr<StreamingBuffer> buffer_;

  // Indicates where previous overlap in above buffers starts (earlier data is
  // just to improve smoothing).
  int prev_overlap_start_ = 0;
  // Indicates where actual overlap starts (data after this has not been
  // output).
  int overlap_start_ = 0;

  // Buffers previous frame.
  std::unique_ptr<cv::Mat> prev_frame_;

  bool compute_feature_descriptors_ = false;

  // Amount of overlap between clips. Determined from saliency smoothing
  // and filtering options.
  int overlap_size_ = 0;

  bool feature_computation_ = true;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_MOTION_ANALYSIS_H_
