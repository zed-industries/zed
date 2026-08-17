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
// Fits tone models to intensity matches
// (gathered from order statistics of matching patches supplied by
// RegionFlowFeatureList's).

#ifndef MEDIAPIPE_UTIL_TRACKING_TONE_ESTIMATION_H_
#define MEDIAPIPE_UTIL_TRACKING_TONE_ESTIMATION_H_

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <deque>
#include <memory>
#include <vector>

#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/framework/port/opencv_imgproc_inc.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/region_flow.h"
#include "mediapipe/util/tracking/region_flow.pb.h"
#include "mediapipe/util/tracking/tone_estimation.pb.h"
#include "mediapipe/util/tracking/tone_models.h"

namespace mediapipe {

class GainBiasModel;
class RegionFlowFeatureList;

typedef std::deque<PatchToneMatch> PatchToneMatches;

// Each vector element presents its own channel.
typedef std::vector<PatchToneMatches> ColorToneMatches;

// Clip mask for C channels.
template <int C>
struct ClipMask {
  ClipMask() {
    min_exposure_threshold.resize(C);
    max_exposure_threshold.resize(C);
  }

  cv::Mat mask;
  std::vector<float> min_exposure_threshold;
  std::vector<float> max_exposure_threshold;
};

class ToneEstimation {
 public:
  ToneEstimation(const ToneEstimationOptions& options, int frame_width,
                 int frame_height);
  virtual ~ToneEstimation();
  ToneEstimation(const ToneEstimation&) = delete;
  ToneEstimation& operator=(const ToneEstimation&) = delete;

  // Estimates ToneChange models from matching feature points.
  // Input RegionFlowFeatureList supplies (x, y) matches, where x is a feature
  // point in curr_frame, and y the matching feature point in prev_frame. The
  // estimated ToneChange describes the change from curr_frame to
  // prev_frame in the tone domain using a variety of linear models as specified
  // by ToneEstimationOptions.
  // If debug_output is not nullptr, contains visualization of inlier patches
  // and clip masks for debugging: current and previous frames are rendered
  // side-by-side (left:current, right:previous).
  // TODO: Parallel estimation across frames.
  void EstimateToneChange(const RegionFlowFeatureList& feature_list,
                          const cv::Mat& curr_frame, const cv::Mat* prev_frame,
                          ToneChange* tone_change,
                          cv::Mat* debug_output = nullptr);

  // Returns mask of clipped pixels for input frame
  // (pixels that are clipped due to limited dynamic range are set to 1),
  // as well as per channel value denoting the min and max exposure threshold.
  template <int C>
  static void ComputeClipMask(const ClipMaskOptions& options,
                              const cv::Mat& frame, ClipMask<C>* clip_mask);

  // Returns color tone matches of size C from passed feature list. Tone matches
  // are obtained from order statistics of sampled patches at feature locations.
  // Matches are obtained for each color channel, with over and under-exposed
  // patches being discarded.
  // If debug_output is not NULL, contains visualization of inlier patches and
  // clip masks for debugging.
  template <int C>
  static void ComputeToneMatches(const ToneMatchOptions& tone_match_options,
                                 const RegionFlowFeatureList& feature_list,
                                 const cv::Mat& curr_frame,
                                 const cv::Mat& prev_frame,
                                 const ClipMask<C>& curr_clip_mask,
                                 const ClipMask<C>& prev_clip_mask,
                                 ColorToneMatches* color_tone_matches,
                                 cv::Mat* debug_output = nullptr);

  // Can be called with color tone matches for 1 - 3 channels, will simply set
  // missing channels in GainBiasModel to identity.
  // Returns the estimated gain and bias model for tone change,
  // that maps colors in current frame to those in the prev frame
  // using color_tone_matches as corresponding samples. Modifies
  // the irls_weights of samples in color_tone_matches based on their
  // fit to the estimated gain_bias_model.
  static void EstimateGainBiasModel(int irls_iterations,
                                    ColorToneMatches* color_tone_matches,
                                    GainBiasModel* gain_bias_model);

  // Tests if the estimated gain-bias model is stable based on:
  // (1) whether the model parameters are within gain and bias bounds, and
  // (2) there are enough inliers in the tone matches.
  // Some statistics related to stability analysis are returned in stats (if
  // not NULL).
  static bool IsStableGainBiasModel(
      const ToneEstimationOptions::GainBiasBounds& gain_bias_bounds,
      const GainBiasModel& model, const ColorToneMatches& tone_matches,
      ToneChange::StabilityStats* stats = nullptr);

 private:
  // Computes normalized intensity percentiles.
  void IntensityPercentiles(const cv::Mat& frame, const cv::Mat& clip_mask,
                            bool log_domain, ToneChange* tone_change) const;

 private:
  ToneEstimationOptions options_;
  int frame_width_;
  int frame_height_;

  int original_width_;
  int original_height_;

  float downsample_scale_ = 1.0f;
  bool use_downsampling_ = false;

  std::unique_ptr<cv::Mat> resized_input_;
  std::unique_ptr<cv::Mat> prev_resized_input_;
};

// Template implementation functions.
template <int C>
void ToneEstimation::ComputeClipMask(const ClipMaskOptions& options,
                                     const cv::Mat& frame,
                                     ClipMask<C>* clip_mask) {
  ABSL_CHECK(clip_mask != nullptr);
  ABSL_CHECK_EQ(frame.channels(), C);

  // Over / Underexposure handling.
  // Masks pixels affected by clipping.
  clip_mask->mask.create(frame.rows, frame.cols, CV_8U);

  const float min_exposure_thresh = options.min_exposure() * 255.0f;
  const float max_exposure_thresh = options.max_exposure() * 255.0f;
  const int max_clipped_channels = options.max_clipped_channels();

  std::vector<cv::Mat> planes;
  cv::split(frame, planes);
  ABSL_CHECK_EQ(C, planes.size());
  float min_exposure[C];
  float max_exposure[C];
  for (int c = 0; c < C; ++c) {
    min_exposure[c] = min_exposure_thresh;
    max_exposure[c] = max_exposure_thresh;
  }

  for (int c = 0; c < C; ++c) {
    clip_mask->min_exposure_threshold[c] = min_exposure[c];
    clip_mask->max_exposure_threshold[c] = max_exposure[c];
  }

  for (int i = 0; i < frame.rows; ++i) {
    const uint8_t* img_ptr = frame.ptr<uint8_t>(i);
    uint8_t* clip_ptr = clip_mask->mask.template ptr<uint8_t>(i);

    for (int j = 0; j < frame.cols; ++j) {
      const int idx = C * j;
      int clipped_channels = 0;  // Count clipped channels.

      for (int p = 0; p < C; ++p) {
        clipped_channels +=
            static_cast<int>(img_ptr[idx + p] < min_exposure[p] ||
                             img_ptr[idx + p] > max_exposure[p]);
      }

      if (clipped_channels > max_clipped_channels) {
        clip_ptr[j] = 1;
      } else {
        clip_ptr[j] = 0;
      }
    }
  }

  // Dilate to address blooming.
  const int dilate_diam = options.clip_mask_diameter();
  const int dilate_rad = ceil(dilate_diam * 0.5);
  // Remove border from dilate, as cv::dilate reads out of
  // bound values otherwise.
  if (clip_mask->mask.rows > 2 * dilate_rad &&
      clip_mask->mask.cols > 2 * dilate_rad) {
    cv::Mat dilate_domain =
        cv::Mat(clip_mask->mask,
                cv::Range(dilate_rad, clip_mask->mask.rows - dilate_rad),
                cv::Range(dilate_rad, clip_mask->mask.cols - dilate_rad));
    cv::Mat kernel(options.clip_mask_diameter(), options.clip_mask_diameter(),
                   CV_8U);
    kernel.setTo(1.0);
    cv::dilate(dilate_domain, dilate_domain, kernel);
  }
}

template <int C>
void ToneEstimation::ComputeToneMatches(
    const ToneMatchOptions& options, const RegionFlowFeatureList& feature_list,
    const cv::Mat& curr_frame, const cv::Mat& prev_frame,
    const ClipMask<C>& curr_clip_mask,  // Optional.
    const ClipMask<C>& prev_clip_mask,  // Optional.
    ColorToneMatches* color_tone_matches, cv::Mat* debug_output) {
  ABSL_CHECK(color_tone_matches != nullptr);
  ABSL_CHECK_EQ(curr_frame.channels(), C);
  ABSL_CHECK_EQ(prev_frame.channels(), C);

  color_tone_matches->clear();
  color_tone_matches->resize(C);

  const int patch_diam = 2 * options.patch_radius() + 1;
  const int patch_area = patch_diam * patch_diam;
  const float patch_denom = 1.0f / (patch_area);
  const float log_denom = 1.0f / LogDomainLUT().MaxLogDomainValue();

  int num_matches = 0;
  std::vector<int> curr_intensities[C];
  std::vector<int> prev_intensities[C];
  for (int c = 0; c < C; ++c) {
    curr_intensities[c].resize(256, 0);
    prev_intensities[c].resize(256, 0);
  }

  // Debugging output. Horizontally concatenate current and previous frames.
  cv::Mat curr_debug;
  cv::Mat prev_debug;
  if (debug_output != nullptr) {
    const int rows = std::max(curr_frame.rows, prev_frame.rows);
    const int cols = curr_frame.cols + prev_frame.cols;
    const cv::Rect curr_rect(cv::Point(0, 0), curr_frame.size());
    const cv::Rect prev_rect(cv::Point(curr_frame.cols, 0), prev_frame.size());
    debug_output->create(rows, cols, CV_8UC3);
    debug_output->setTo(cv::Scalar(255, 0, 0));
    curr_debug = (*debug_output)(curr_rect);
    prev_debug = (*debug_output)(prev_rect);
    curr_frame.copyTo(curr_debug, curr_clip_mask.mask ^ cv::Scalar(1));
    prev_frame.copyTo(prev_debug, prev_clip_mask.mask ^ cv::Scalar(1));
  }

  const int patch_radius = options.patch_radius();
  const int frame_width = curr_frame.cols;
  const int frame_height = curr_frame.rows;
  for (const auto& feature : feature_list.feature()) {
    // Extract matching masks at this feature.
    Vector2_i curr_loc = FeatureIntLocation(feature);
    Vector2_i prev_loc = FeatureMatchIntLocation(feature);

    // Start bound inclusive, end bound exclusive.
    Vector2_i curr_start = Vector2_i(std::max(0, curr_loc.x() - patch_radius),
                                     std::max(0, curr_loc.y() - patch_radius));

    Vector2_i curr_end =
        Vector2_i(std::min(frame_width, curr_loc.x() + patch_radius + 1),
                  std::min(frame_height, curr_loc.y() + patch_radius + 1));

    Vector2_i prev_start = Vector2_i(std::max(0, prev_loc.x() - patch_radius),
                                     std::max(0, prev_loc.y() - patch_radius));

    Vector2_i prev_end =
        Vector2_i(std::min(frame_width, prev_loc.x() + patch_radius + 1),
                  std::min(frame_height, prev_loc.y() + patch_radius + 1));

    Vector2_i curr_size = curr_end - curr_start;
    Vector2_i prev_size = prev_end - prev_start;

    if (prev_size != curr_size ||
        (curr_size.x() * curr_size.y()) != patch_area) {
      continue;  // Ignore border patches.
    }

    // TODO: Instead of summing masks, perform box filter over mask
    // and just grab element at feature location.
    cv::Mat curr_patch_mask(curr_clip_mask.mask,
                            cv::Range(curr_start.y(), curr_end.y()),
                            cv::Range(curr_start.x(), curr_end.x()));
    cv::Mat prev_patch_mask(prev_clip_mask.mask,
                            cv::Range(prev_start.y(), prev_end.y()),
                            cv::Range(prev_start.x(), prev_end.x()));

    // Skip patch if too many clipped pixels.
    if (cv::sum(curr_patch_mask)[0] * patch_denom >
            options.max_frac_clipped() ||
        cv::sum(prev_patch_mask)[0] * patch_denom >
            options.max_frac_clipped()) {
      continue;
    }

    // Extract matching patches at this feature.
    cv::Mat curr_patch(curr_frame, cv::Range(curr_start.y(), curr_end.y()),
                       cv::Range(curr_start.x(), curr_end.x()));
    cv::Mat prev_patch(prev_frame, cv::Range(prev_start.y(), prev_end.y()),
                       cv::Range(prev_start.x(), prev_end.x()));

    // Reset histograms to zero.
    for (int c = 0; c < C; ++c) {
      std::fill(curr_intensities[c].begin(), curr_intensities[c].end(), 0);
      std::fill(prev_intensities[c].begin(), prev_intensities[c].end(), 0);
    }

    // Build histogram (to sidestep sorting).
    // Note: We explicitly add over and under-exposed pixels to the
    // histogram, as we are going to sample the histograms at specific
    // percentiles modeling the shift in intensity between frames.
    // (If the average intensity increases by N, histogram shifts by N
    // bins to the right). However, matches that are over or underexposed
    // are discarded afterwards.
    for (int i = 0; i < patch_diam; ++i) {
      const uint8_t* prev_ptr = prev_patch.ptr<uint8_t>(i);
      const uint8_t* curr_ptr = curr_patch.ptr<uint8_t>(i);
      for (int j = 0; j < patch_diam; ++j) {
        const int j_c = C * j;
        for (int c = 0; c < C; ++c) {
          ++curr_intensities[c][curr_ptr[j_c + c]];
          ++prev_intensities[c][prev_ptr[j_c + c]];
        }
      }
    }

    // Sample at percentiles.
    for (int c = 0; c < C; ++c) {
      // Accumulate.
      for (int k = 1; k < 256; ++k) {
        curr_intensities[c][k] += curr_intensities[c][k - 1];
        prev_intensities[c][k] += prev_intensities[c][k - 1];
      }

      float percentile = options.min_match_percentile();
      float percentile_step =
          (options.max_match_percentile() - options.min_match_percentile()) /
          options.match_percentile_steps();

      PatchToneMatch patch_tone_match;
      for (int k = 0; k < options.match_percentile_steps();
           ++k, percentile += percentile_step) {
        const auto& curr_iter = std::lower_bound(curr_intensities[c].begin(),
                                                 curr_intensities[c].end(),
                                                 percentile * patch_area);

        const auto& prev_iter = std::lower_bound(prev_intensities[c].begin(),
                                                 prev_intensities[c].end(),
                                                 percentile * patch_area);

        const int curr_int = curr_iter - curr_intensities[c].begin();
        const int prev_int = prev_iter - prev_intensities[c].begin();

        // If either clipped, discard match.
        if (curr_int < curr_clip_mask.min_exposure_threshold[c] ||
            curr_int > curr_clip_mask.max_exposure_threshold[c] ||
            prev_int < prev_clip_mask.min_exposure_threshold[c] ||
            prev_int > prev_clip_mask.max_exposure_threshold[c]) {
          continue;
        }

        ToneMatch* tone_match = patch_tone_match.add_tone_match();
        if (options.log_domain()) {
          tone_match->set_curr_val(LogDomainLUT().Map(curr_int) * log_denom);
          tone_match->set_prev_val(LogDomainLUT().Map(prev_int) * log_denom);
        } else {
          tone_match->set_curr_val(curr_int * (1.0f / 255.0f));
          tone_match->set_prev_val(prev_int * (1.0f / 255.0f));
        }
      }

      (*color_tone_matches)[c].push_back(patch_tone_match);

      // Debug output.
      if (debug_output != nullptr) {
        cv::rectangle(curr_debug, cv::Point(curr_start.x(), curr_start.y()),
                      cv::Point(curr_end.x(), curr_end.y()),
                      cv::Scalar(0, 0, 255));
        cv::rectangle(prev_debug, cv::Point(prev_start.x(), prev_start.y()),
                      cv::Point(prev_end.x(), prev_end.y()),
                      cv::Scalar(0, 0, 255));
      }
    }

    ++num_matches;
  }

  VLOG(1) << "Extracted fraction: "
          << static_cast<float>(num_matches) /
                 std::max(1, feature_list.feature_size());
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_TONE_ESTIMATION_H_
