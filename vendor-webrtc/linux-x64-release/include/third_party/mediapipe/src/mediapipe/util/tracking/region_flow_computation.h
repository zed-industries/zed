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
// Computes the RegionFlow for a set of frames.
// Specifically, extracts Harris-like features from each frame, tracks these
// between frames and regularizes the tracked features locally (outlier
// rejection) by leveraging fast per-frame segmentation.
// Optionally, features can be assigned to either foreground or background based
// on the computation of the fundamental matrix for a pair of frames,
//
// Basic usage:
// RegionFlowComputation flow_computation(RegionFlowComputationOptions(),
//                                        frame_width,
//                                        frame_height);
//
// std::vector<cv::Mat> input_images;                // Supplied by caller.
// for (int i = 0; i < num_frames; ++i) {
//   flow_computation.AddImage(input_images[i]);
//
//   // Result is owned by this caller.
//   std::unique_ptr<RegionFlow> result(
//       flow_computation.RetrieveRegionFlow());
//
//   // OR
//   std::unique_ptr<RegionFlowFeatureList> result(
//       flow_computation.RetrieveRegionFlowFeatureList(
//           true,              // Compute feature descriptor.
//           false,             // no match descriptor for this example.
//           &input_images[i],
//           nullptr);
//
//   // Do your custom processing or pass on to MotionEstimation.
//
//  }

#ifndef MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_COMPUTATION_H_
#define MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_COMPUTATION_H_

#include <cstdint>
#include <deque>
#include <memory>
#include <unordered_map>
#include <vector>

#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/util/tracking/motion_models.pb.h"
#include "mediapipe/util/tracking/region_flow.h"
#include "mediapipe/util/tracking/region_flow.pb.h"
#include "mediapipe/util/tracking/region_flow_computation.pb.h"

namespace mediapipe {
class RegionFlowFeatureList;
class RegionFlowFrame;
}  // namespace mediapipe

namespace mediapipe {

struct TrackedFeature;
typedef std::vector<TrackedFeature> TrackedFeatureList;
class MotionAnalysis;

class RegionFlowComputation {
 public:
  RegionFlowComputation(const RegionFlowComputationOptions& options,
                        int frame_width, int frame_height);
  virtual ~RegionFlowComputation();
  RegionFlowComputation(const RegionFlowComputation&) = delete;
  RegionFlowComputation& operator=(const RegionFlowComputation&) = delete;

  // Performs motion analysis on source w.r.t. to source passed in previous
  // call. Therefore, first call will compute empty flow. If
  // RegionFlowComputationOptions::frame_to_track := ftt > 0, motion analysis
  // performed w.r.t. the previous ftt source passed via AddImage.
  // Motion analysis uses grid-based regions to enforce locally consistent flow.
  // Source is expected to be 3-channel RGB 8bit image (24bit in total), OR
  // 1-channel Grayscale 8bit image, compatible with
  // RegionFlowComputationOptions::ImageFormat.
  // Pass the frame's timestamp to have it stored in the result or zero if not
  // needed.
  // Returns true on success, false otherwise.
  virtual bool AddImage(const cv::Mat& source, int64_t timestamp_usec);

  // Same as above, but seed initial feature position in the matching frame
  // with initial_transform.
  virtual bool AddImageWithSeed(const cv::Mat& source, int64_t timestamp_usec,
                                const Homography& initial_transform);

  // Same as AddImage but also accepts an optional source_mask (pass empty
  // cv::Mat to get the same behavior as AddImage). If non-empty, features are
  // only extracted in regions where the mask value is > 0. Mask should be 8-bit
  // grayscale of the same size as source, unless empty.
  virtual bool AddImageWithMask(const cv::Mat& source,
                                const cv::Mat& source_mask,
                                int64_t timestamp_usec);

  // Call after AddImage* to retrieve last downscaled, grayscale image.
  cv::Mat GetGrayscaleFrameFromResults();

  // Returns result as RegionFlowFrame. Result is owned by caller.
  // Will return NULL if called twice without AddImage* call.
  virtual RegionFlowFrame* RetrieveRegionFlow();

  // Returns result as RegionFlowFeatureList. Result is owned by caller.
  // Will return NULL if called twice without AddImage* call.
  // Computes optionally feature descriptors (if compute_feature_descriptor
  // is set, in that case curr_color_image must not be NULL)
  // and additionally matching descriptor (if compute_match_descriptor is set,
  // in this case prev_color_image must not be NULL).
  // Passed images should be in sync with those passed to AddImage, i.e.
  // source in AddImage and parameter curr_color_image should refer to the same
  // image.
  virtual RegionFlowFeatureList* RetrieveRegionFlowFeatureList(
      bool compute_feature_descriptor, bool compute_match_descriptor,
      const cv::Mat* curr_color_image,   // optional.
      const cv::Mat* prev_color_image);  // optional.

  // Same as above, but returns specific tracked result from current frame C
  // to C - track_index - 1.
  virtual RegionFlowFeatureList* RetrieveMultiRegionFlowFeatureList(
      int track_index, bool compute_feature_descriptor,
      bool compute_match_descriptor,
      const cv::Mat* curr_color_image,   // optional.
      const cv::Mat* prev_color_image);  // optional.

  // Returns result of a specific RegionFlowFrame in case
  // RegionFlowComputationOptions::frames_to_track() > 1. Result is owned by
  // caller.
  virtual RegionFlowFrame* RetrieveMultiRegionFlow(int frame);

  // Resets computation to ignore any previously added frames. Next frame passed
  // via AddImageXXX() routines will be treated as the first frame in the
  // sequence.
  virtual void Reset();

  // Creates synthetic tracks with feature points in a grid with zero motion
  // w.r.t. prev frame. Points are located at the center of each grid. Step size
  // is fractional w.r.t. image size.
  static void ZeroMotionGridFeatures(int frame_width, int frame_height,
                                     float frac_grid_step_x,
                                     float frac_grid_step_y,
                                     RegionFlowFeatureList* result);

  // Returns densly sampled motions zero motion features.
  // Features are centered in a box of size frac_diameter that is shifted by
  // frac_steps_x * frame_width and frac_steps_y * frame_height.
  static void DenseZeroMotionSamples(int frame_width, int frame_height,
                                     float frac_diameter, float frac_steps_x,
                                     float frac_steps_y,
                                     RegionFlowFeatureList* result);

 private:
  typedef std::vector<std::unique_ptr<RegionFlowFeatureList>>
      RegionFlowFeatureListVector;

  typedef std::vector<TrackedFeature*> TrackedFeatureView;

  // Indexed via grid bin, each bin contains list of its corresponding features.
  typedef std::vector<TrackedFeatureView> TrackedFeatureMap;

  struct FrameTrackingData;
  struct LongTrackData;
  struct ORBFeatureDescriptors;

  // Implementation function to retrieve the i-th RegionFlowFeatureList
  // (specified track_index). Specifically, i-th feature list, denotes the flow
  // from the current frame N to the previous frame N - 1 - track_index.
  // Casts arguments to cv.
  virtual std::unique_ptr<RegionFlowFeatureList>
  RetrieveRegionFlowFeatureListImpl(int track_index,
                                    bool compute_feature_descriptor,
                                    bool compute_match_descriptor,
                                    const cv::Mat* curr_color_image,
                                    const cv::Mat* prev_color_image);

  // Initializes the FrameTrackingData's members from source and source_mask.
  // Returns true on success.
  bool InitFrame(const cv::Mat& source, const cv::Mat& source_mask,
                 FrameTrackingData* data);

  // Adds image to the current buffer and starts tracking.
  bool AddImageAndTrack(const cv::Mat& source, const cv::Mat& source_mask,
                        int64_t timestamp_usec,
                        const Homography& initial_transform);

  // Computes *change* in visual difference between adjacent frames. Normalized
  // w.r.t. number of channels and number of pixels. For this to be meaningful
  // it is expected that passed FrameTrackingData's are exactly one frame apart
  // (CHECKED).
  float ComputeVisualConsistency(FrameTrackingData* previous,
                                 FrameTrackingData* current) const;

  // Computes flow regularized based on regions and other options, from frame
  // index "from" to index "to", specified relative to current frame, i.e. index
  // of current frame = 0, prev frame = -1, next frame = 1, etc. Set invert_flow
  // to true if the flow should be inverted after tracking.
  // Optionally, can input the previous result to link features via ids,
  // effectively creating long feature tracks. In this case you usually want to
  // request the current result (same as returned in feature_list) in form of
  // a TrackedFeatureList.
  void ComputeRegionFlow(int from, int to, bool synthetic_tracks,
                         bool invert_flow,
                         const TrackedFeatureList* prev_result,  // optional.
                         TrackedFeatureList* curr_result,        // optional.
                         RegionFlowFeatureList* feature_list);

  // Gain corrects input frame w.r.t. reference frame. Returns true iff gain
  // correction succeeds. If false, calibrated_frame is left untouched.
  bool GainCorrectFrame(const cv::Mat& reference_frame,
                        const cv::Mat& input_frame, float reference_mean,
                        float input_mean, cv::Mat* calibrated_frame) const;

  // Feature extraction method.
  // Expects as input an image pyramid of gray scale image (each subsequent
  // level should be downsampled by a factor of 2 (always rounding up), CHECKED
  // against).
  // For each level extracts corner features across a grid by considering all
  // locations that have a corner response corner repsonse above
  // options_.feature_quality_level() * maximum within the grid bin.
  // Features with high corner response are output first (but corner response is
  // not necessarily monotonic). Feature locations are binned into mask
  // (via downscaling by mask_scale), using a 5x5 patch, to discarded features
  // that are too close to each other.
  // Features and corner responses are added to the corresponding vectors in
  // data, i.e. passed data is not cleared and expected to be initialized.
  virtual void AdaptiveGoodFeaturesToTrack(
      const std::vector<cv::Mat>& extraction_pyramid, int max_features,
      float mask_scale, cv::Mat* mask, FrameTrackingData* data);

  // Uses prev_result to remove all features that are not present in data.
  // Uses track_ids, i.e. only works with long feature processing.
  void RemoveAbsentFeatures(const TrackedFeatureList& prev_result,
                            FrameTrackingData* data);

  // Remove features in data that lie outside the feature extraction mask for
  // that frame.
  void RemoveFeaturesOutsideMask(FrameTrackingData* data);

  // Extracts features for tracking from frame corresponding to data.
  // Optionally, may reuse tracked features if available, based on options.
  // Optionally, if new features are extracted, can use feature_list to mask out
  // feature locations that should not be extracted again.
  void ExtractFeatures(const TrackedFeatureList* prev_result,
                       FrameTrackingData* data);

  // Performs inplace feature selection, by evaluating the range
  // [0, data->features.size()] via an Evaluator implementing
  // [](int) -> bool. Only feature indices for which eval returns true are kept
  // (using in place moves) the remainder is discarded. Applies moves operation
  // to FrameTrackingData's feature, track_idx, feature_source_map and
  // neighborhoods. Also applies moves to any vector<int> and vector<float>
  // that can be optionally supplied.
  // Note: All vectors are assumed to of the same size (checked in debug
  // mode).
  template <class Evaluator>
  int InplaceFeatureSelection(FrameTrackingData* data,
                              std::vector<std::vector<int>*> int_vecs,
                              std::vector<std::vector<float>*> float_vecs,
                              const Evaluator& eval);

  // Tracks features between two frames (from -> to). Operates on internal data
  // structure FrameTrackingData which stores all frame information relavant for
  // tracking.
  //
  // If gain_correct is true, tracking is carried out between the "from" and the
  // gain-corrected "to" image. It is also an output variable indicating whether
  // gain correction succeeded or failed.
  //
  // Updates internal data structure, so any computation can be reused in
  // successive calls to feature extraction or tracking.
  void TrackFeatures(FrameTrackingData* from_data_ptr,
                     FrameTrackingData* to_data_ptr, bool* gain_correct,
                     float* frac_long_features_rejected,
                     TrackedFeatureList* results);

  // Wide-baseline version of above function, using feature descriptor matching
  // instead of tracking.
  void WideBaselineMatchFeatures(FrameTrackingData* from_data_ptr,
                                 FrameTrackingData* to_data_ptr,
                                 TrackedFeatureList* results);

  // Fits affine model to TrackedFeatureList via direct call of
  // MotionEstimation::EstimateAffineModelIRLS.
  AffineModel AffineModelFromFeatures(TrackedFeatureList* features) const;

  // Creates synthetic tracks with feature points in a grid with zero motion
  // w.r.t. prev frame. Points are located at the center of each grid. Step size
  // is fractional w.r.t. image size.
  // Returns minimum distance from border across all features.
  static int ZeroMotionGridTracks(int frame_width, int frame_height,
                                  float frac_grid_step_x,
                                  float frac_grid_step_y,
                                  TrackedFeatureList* results);

  // Computes region flow using a rectangular grid of square regions.
  void ComputeBlockBasedFlow(TrackedFeatureList* feature_list,
                             TrackedFeatureView* inlier_features) const;

  // Initializes feature locations for FrameTrackingData at index to,
  // from resulting tracks in from.
  void InitializeFeatureLocationsFromPreviousResult(int from, int to);

  // Initializes feature locations in "to" from initial transform by applying
  // it to every feature of "from".
  void InitializeFeatureLocationsFromTransform(int from, int to,
                                               const Homography& transform);

  // Enforces a translational model within each region, only retaining inliers
  // that are output to inliers.
  void DetermineRegionFlowInliers(const TrackedFeatureMap& region_feature_map,
                                  TrackedFeatureView* inliers) const;

  // Determines number of minimum inliers based on absolute and relative
  // thresholds.
  int GetMinNumFeatureInliers(
      const TrackedFeatureMap& region_feature_map) const;

  // Internal conversion function from a feature list to corresponding frame.
  void RegionFlowFeatureListToRegionFlow(
      const RegionFlowFeatureList& feature_list, RegionFlowFrame* frame) const;

  // Initializes all members except actual features in a RegionFlowFeatureList.
  void InitializeRegionFlowFeatureList(
      RegionFlowFeatureList* region_flow_feature_list) const;

  // Converts TrackedFeatureView to RegionFlowFeatureList, flattening over
  // all bins. Returns average motion magnitude.
  // Optionally TrackedFeature's corresponding to each feature output in
  // region_flow_feature_list can be recorded via flattened_feature_list.
  float TrackedFeatureViewToRegionFlowFeatureList(
      const TrackedFeatureView& region_feature_view,
      TrackedFeatureList* flattened_feature_list,
      RegionFlowFeatureList* region_flow_feature_list) const;

  // Determines if sufficient (spatially distributed) features are available.
  bool HasSufficientFeatures(const RegionFlowFeatureList& feature_list);

  // Returns number of required pyramid levels to track the specified distance.
  int PyramidLevelsFromTrackDistance(float track_distance);

  // Returns blur score (inverse of average corner measure) for input image.
  // The higher the value the blurrier the frame.
  float ComputeBlurScore(const cv::Mat& image);

  // Computes binary mask of pixels, for which the corner score (passed in
  // min_eig_vals) can be used to as a measure to quanity the amount of blur.
  // For pixelx not part of the mask the corner score is not a reliable measure
  // to quanity blur. For example, discards over-exposed regions and regions
  // that do not have sufficient cornerness.
  // Note: Modifies the corner values!
  void ComputeBlurMask(const cv::Mat& input, cv::Mat* min_eig_vals,
                       cv::Mat* mask);

  // Appends features in a sorted manner (by pointer location) while discarding
  // duplicates.
  void AppendUniqueFeaturesSorted(const TrackedFeatureView& to_be_added,
                                  TrackedFeatureView* features) const;

  void GetFeatureTrackInliers(bool skip_estimation,
                              TrackedFeatureList* features,
                              TrackedFeatureView* inliers) const;

  bool IsVerifyLongFeatures() const {
    return long_track_data_ != nullptr && options_.verify_long_features();
  }

  int DownsampleWidth() const { return frame_width_; }
  int DownsampleHeight() const { return frame_height_; }

  // Returns 1.0 / scale that is being applied to the features for downscaling.
  float DownsampleScale() const { return downsample_scale_; }

 private:
  RegionFlowComputationOptions options_;

  // Frame width and height after downsampling.
  int frame_width_;
  int frame_height_;

  // Number of frames w.r.t each frame is tracked.
  int frames_to_track_;
  // Maximum length of long feature tracks in frames.
  int max_long_track_length_;

  // Original frame width and height.
  int original_width_;
  int original_height_;

  // Scale and state of downsampling.
  float downsample_scale_;
  bool use_downsampling_;

  int pyramid_levels_;
  int extraction_levels_;

  int frame_num_ = 0;
  int max_features_ = 0;
  float curr_blur_score_ = 0;
  // Moving average of number of features across recently computed tracks.
  float curr_num_features_avg_ = 0;

  // Count used to generate unique feature ids.
  int feature_count_ = 0;

  // List of RegionFlow frames of size options_.frames_to_track.
  RegionFlowFeatureListVector region_flow_results_;

  // Gain adapted version.
  std::unique_ptr<cv::Mat> gain_image_;
  std::unique_ptr<cv::Mat> gain_pyramid_;

  // Temporary buffers.
  std::unique_ptr<cv::Mat> corner_values_;
  std::unique_ptr<cv::Mat> corner_filtered_;
  std::unique_ptr<cv::Mat> corner_mask_;

  std::unique_ptr<cv::Mat> curr_color_image_;

  // Temporary images for feature extraction.
  std::unique_ptr<cv::Mat> feature_tmp_image_1_;
  std::unique_ptr<cv::Mat> feature_tmp_image_2_;

  std::vector<uint8_t> feature_status_;     // Indicates if point could be
                                            // tracked.
  std::vector<float> feature_track_error_;  // Patch-based error.

  // Circular queue to buffer tracking data.
  std::deque<std::unique_ptr<FrameTrackingData>> data_queue_;

  // Global settings for block based flow.
  int block_width_;
  int block_height_;
  int block_levels_;

  // Stores average flow magnitudes for recently processed frames.
  std::deque<float> flow_magnitudes_;

  // Records data for long feature tracks.
  std::unique_ptr<LongTrackData> long_track_data_;

  // Counter used for controlling how ofter do we run descriptor extraction.
  // Count from 0 to options_.extract_descriptor_every_n_frame() - 1.
  // Extract descriptors only when counter == 0.
  int cnt_extract_descriptors_ = 0;

  friend class MotionAnalysis;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_COMPUTATION_H_
