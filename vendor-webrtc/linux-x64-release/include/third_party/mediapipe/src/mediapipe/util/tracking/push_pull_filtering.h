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
// Push pull filtering parametrized by number of channels.
// Performs sparse vector data interpolation across a specified domain.
// Optionally interpolation can be guided to be discontinuous across image
// boundaries and customized with various multipliers as described below.
//
// Note: As the file contains templated implementations it is recommended to be
// included in cc files instead of headers to speed up compilation.

#ifndef MEDIAPIPE_UTIL_TRACKING_PUSH_PULL_FILTERING_H_
#define MEDIAPIPE_UTIL_TRACKING_PUSH_PULL_FILTERING_H_

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <numeric>
#include <string>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/util/tracking/image_util.h"
#include "mediapipe/util/tracking/push_pull_filtering.pb.h"

namespace mediapipe {

const float kBilateralEps = 1e-6f;

// Push Pull algorithm can be decorated with mip-map visualizers,
// per-level weight adjusters and per-filter element weight multipliers.
// Implemented by default as no-ops below.
//
// Mip map's are made of C + 1 channel matrices, where data is stored at
// channels [0 .. C - 1], and push pull weights at index C. Commong flag
// pull_down_sampling for classes below is true for first pull stage
// (fine -> coarse) and false for second push stage (coarse -> fine).

// Is called with the interpolated data at every level of the hierarchy. Enables
// to adjust weights or perform other kinds of modification *globally* for each
// mip-map level.
class PushPullWeightAdjuster {
 public:
  virtual ~PushPullWeightAdjuster() {}

  // In case of bilateral weighting, input_frame (resized to corresponding mip
  // map level) is passed as well, otherwise parameter is NULL.
  virtual void AdjustWeights(int mip_map_level, bool pull_down_sampling,
                             cv::Mat* input_frame,  // Bilateral case.
                             cv::Mat* data_with_weights) = 0;
};

// Allows mip map to be visualized after first stage (pull_down_sampling ==
// true) and second stage (push up sampling, i.e. pull_down_sampling == false).
// Note: For visualizers, data values in mip map are pre-multiplied by
// confidence weights in channel C if corresponding flag is_premultiplied
// is set to true). In this case normalization (division by confidence)
// has to be performed before visualization.
// Passed mip maps are borderless, i.e. views into the actual mip map with
// border being removed.
class PushPullMipMapVisualizer {
 public:
  virtual ~PushPullMipMapVisualizer() {}

  virtual void Visualize(const std::vector<const cv::Mat*>& mip_map,
                         bool pull_down_sampling,
                         const std::vector<bool>& is_premultiplied) = 0;
};

// FilterWeightMultiplier is a template argument to PushPullFiltering to
// adjust the filter weight at every up- and downsampling stage.
// Specifically every point (x, y) with data  pointer anchor_ptr into
// the current mip maplevel (C + 1 channels, first C
// contain data, index C contains push-pull importance weight) is filtered in a
// neighborhood with several neighboring samples (pointed to by filter_ptr).
// In case of bilateral filtering img_ptr is pointing to 3 channel image pixel
// of the anchor.
// Default version of multiplier must be constructable with no arguments, and
// the following two interfaces have to be implemented:
//
// // Signals change in level, can be used for mutable initialization functions.
// void SetLevel(int mip_map_level, bool pull_down_sampling);
//
// // Function is called once for every neighbor (filter_ptr) of a pixel
// // (anchor_ptr). Location (x,y) of the pixel pointed to by anchor pointer is
// // also passed if needed for more complex operations.
// float WeightMultiplier(const float* anchor_ptr,    // Points to anchor.
//                        const float* filter_ptr,    // Offset element.
//                        const uint_8t* img_ptr,     // NULL if not bilateral.
//                        int x,
//                        int y) const;
//
//  Here is an example (used by default).

// Default no-op.
class FilterWeightMultiplierOne {
 public:
  void SetLevel(int mip_map_level, bool pull_down_sampling) {}

  float GetWeight(const float* anchor_ptr, const float* filter_ptr,
                  const uint8_t* img_ptr, int x, int y) const {
    return 1.0f;
  }
};

class PushPullFilteringTest;

// Templated by number of channels and FilterWeightMultiplier.
template <int C, class FilterWeightMultiplier = FilterWeightMultiplierOne>
class PushPullFiltering {
 public:
  enum FilterType {
    BINOMIAL_3X3 = 0,
    BINOMIAL_5X5 = 1,
    GAUSSIAN_3X3 = 2,  // sigma = 1.
    GAUSSIAN_5X5 = 3,  // sigma = 1.6.
  };

  // Initializes push pull filter for specified domain size.
  // Optionally, weight_multiplier, mip-map visualizer and
  // weight adjuster can be passed as argument.
  PushPullFiltering(const cv::Size& domain_size, FilterType filter_type,
                    bool use_bilateral,
                    FilterWeightMultiplier* weight_multiplier,     // Optional.
                    PushPullMipMapVisualizer* mip_map_visualizer,  // Optional.
                    PushPullWeightAdjuster* weight_adjuster);      // Optional.
  PushPullFiltering() = delete;
  PushPullFiltering(const PushPullFiltering&) = delete;
  PushPullFiltering& operator=(const PushPullFiltering&) = delete;

  // Returns number of pyramid levels allocated for given domain size.
  int PyramidLevels() const { return downsample_pyramid_.size(); }

  // Returns domain size of n-th pyramid level (including border depending on
  // filter_type).
  cv::Size NthPyramidDomain(int level) {
    ABSL_CHECK_LT(level, PyramidLevels());
    return downsample_pyramid_[level].size();
  }

  void SetOptions(const PushPullOptions& options);

  // Push-Pull filter for C + 1 channel float displacement image
  // (expected to be of size domain_size plus 1 (if filter == *_3x3) or
  // 2 (if filter == *_5x5) pixel border around it, use
  // BorderFromFilterType function for lookup).
  // First C dimensions contain interpolated data, last dimension contains
  // associated importance weight.
  // Places data_values on integer location data_locations + origin with
  // uniform weight (== push_pull_weight) and employs iterative weighted
  // down and up-sampling. If optional data_weight is specified uses per datum
  // feature weight instead (weights are expected to be within [0, 1]).
  // If input_frame is specified, spatial filter type is combined with
  // intensity based filtering yielding bilateral weighting.
  // Results are returned in parameter results.
  // Filter is performed in 2 stages:
  // i) PullDownSampling: Densifies the data by successive downsampling stages,
  //    averaging confidence and values across the domain from sparse data
  //    locations to unset values.
  // ii) PushUpSampling: Pushes densified data back through the pyramid by
  //     successive upsampling stages, over-writing unset values with filled in
  //     data from downsampled version.
  void PerformPushPull(const std::vector<Vector2_f>& data_locations,
                       const std::vector<cv::Vec<float, C>>& data_values,
                       float push_pull_weight, cv::Point2i origin,
                       int readout_level,                       // Default: 0.
                       const std::vector<float>* data_weights,  // Optional.
                       const cv::Mat* input_frame,              // Optional.
                       cv::Mat* results);

  // This is the same as PerformPushPull above except that it
  // assumes that the data (the mip_map at level 0) is given as a cv::Mat.
  // The Mat should be C + 1 channels in total.
  // The first C channels (channels 0 to C-1) of mip_map_level_0 should contain
  // data_values * data_weights (or push_pull_weight) at appropriate locations,
  // offset by the border.
  // The corresponding locations in channel C are set to the data_weights.
  // Locations without data should be set to 0 in all channels.
  void PerformPushPullMat(const cv::Mat& mip_map_level_0,
                          int readout_level,           // Default: 0.
                          const cv::Mat* input_frame,  // Optional.
                          cv::Mat* results);

  static constexpr int BorderFromFilterType(FilterType filter_type);

  FilterType filter_type() const { return filter_type_; }

 private:
  // Implementation functions for PushPullFiltering.
  void SetupFilters();

  void SetupBilateralLUT();

  // If allocate_base_level is set, allocates a frame for level zero of
  // size domain_size + 2 * border, otherwise only levels 1 to end are
  // allocated.
  void AllocatePyramid(const cv::Size& domain_size, int border, int type,
                       bool allocate_base_level, std::vector<cv::Mat>* pyramid);

  // Downsampling operation for input_frame along pre-allocated pyramid.
  void InitializeImagePyramid(const cv::Mat& input_frame,
                              std::vector<cv::Mat>* pyramid);

  void PerformPushPullImpl(const int readout_level, const cv::Mat* input_frame,
                           std::vector<cv::Mat*>* mip_map_ptr);

  void PullDownSampling(int num_filter_elems, const float* filter_weights,
                        std::vector<cv::Mat*>* mip_map_ptr);

  void PushUpSampling(int num_filter_elems, const float* filter_weights,
                      int readout_level, std::vector<cv::Mat*>* mip_map_ptr);

  // Convenience function selecting appropiate border size based on filter_type.
  template <typename T, int channels>
  void CopyNecessaryBorder(cv::Mat* mat);

  // Implementation function for fast upsampling. See comments before definition
  // for details.
  void GetUpsampleTaps3(const float* filter_weights,
                        const std::vector<int>* space_offsets,  // optional.
                        int inc_x, int inc_y, std::vector<float> tap_weights[4],
                        std::vector<int> tap_offsets[4],
                        std::vector<int> tap_space_offsets[4]);

  void GetUpsampleTaps5(const float* filter_weights,
                        const std::vector<int>* space_offsets,  // optional.
                        int inc_x, int inc_y, std::vector<float> tap_weights[4],
                        std::vector<int> tap_offsets[4],
                        std::vector<int> tap_space_offsets[4]);

  inline int ColorDiffL1(const uint8_t* lhs_ptr, const uint8_t* rhs_ptr) {
    return abs(static_cast<int>(lhs_ptr[0]) - static_cast<int>(rhs_ptr[0])) +
           abs(static_cast<int>(lhs_ptr[1]) - static_cast<int>(rhs_ptr[1])) +
           abs(static_cast<int>(lhs_ptr[2]) - static_cast<int>(rhs_ptr[2]));
  }

  const cv::Size domain_size_;
  FilterType filter_type_;
  int border_;

  std::array<float, 25> binomial5_weights_;
  std::array<float, 9> binomial3_weights_;
  std::array<float, 25> gaussian5_weights_;
  std::array<float, 9> gaussian3_weights_;

  // Pyramids used by PushPull implementation.
  std::vector<cv::Mat> downsample_pyramid_;
  std::vector<cv::Mat> input_frame_pyramid_;

  // Pre-computed spacial offets based on selected filter for each level of the
  // pyramid.
  std::vector<std::vector<int>> pyramid_space_offsets_;

  bool use_bilateral_ = false;

  FilterWeightMultiplier* weight_multiplier_;
  // Used to create default multiplier is none was passed.
  std::unique_ptr<FilterWeightMultiplier> default_weight_multiplier_;

  PushPullMipMapVisualizer* mip_map_visualizer_;
  PushPullWeightAdjuster* weight_adjuster_;
  PushPullOptions options_;

  std::vector<float> bilateral_lut_;

  friend class PushPullFilteringTest;
};

// Typedef's for explicit instantiation, add more if needed.
typedef PushPullFiltering<1> PushPullFilteringC1;
typedef PushPullFiltering<2> PushPullFilteringC2;
typedef PushPullFiltering<3> PushPullFilteringC3;
typedef PushPullFiltering<4> PushPullFilteringC4;

// For compatible forward declarations add this to your header:
//   template<int C, class FilterWeightMultiplier> class PushPullFiltering;
//   class FilterWeightMultiplierOne;
//   typedef PushPullFiltering<1, FilterWeightMultiplierOne> PushPullFlowC1;

// Several concrete helper classes below.
///////////////////////////////////////////////////////////////////////////////
// Template implementation functions below.
//
template <int C, class FilterWeightMultiplier>
PushPullFiltering<C, FilterWeightMultiplier>::PushPullFiltering(
    const cv::Size& domain_size, FilterType filter_type, bool use_bilateral,
    FilterWeightMultiplier* weight_multiplier,
    PushPullMipMapVisualizer* mip_map_visualizer,
    PushPullWeightAdjuster* weight_adjuster)
    : domain_size_(domain_size),
      filter_type_(filter_type),
      use_bilateral_(use_bilateral),
      weight_multiplier_(weight_multiplier),
      mip_map_visualizer_(mip_map_visualizer),
      weight_adjuster_(weight_adjuster) {
  border_ = BorderFromFilterType(filter_type);
  if (border_ < 0) {
    ABSL_LOG(FATAL) << "Unknown filter requested.";
  }

  SetupFilters();
  AllocatePyramid(domain_size_, border_, CV_32FC(C + 1), true,
                  &downsample_pyramid_);

  if (use_bilateral_) {
    SetupBilateralLUT();
    AllocatePyramid(domain_size_, border_, CV_8UC3, true,
                    &input_frame_pyramid_);

    // Setup space offsets.
    pyramid_space_offsets_.resize(input_frame_pyramid_.size());
    for (int l = 0, num_levels = input_frame_pyramid_.size(); l < num_levels;
         ++l) {
      std::vector<int>& space_offsets = pyramid_space_offsets_[l];
      const cv::Mat& pyramid_frame = input_frame_pyramid_[l];
      for (int row = -border_; row <= border_; ++row) {
        for (int col = -border_; col <= border_; ++col) {
          space_offsets.push_back(row * pyramid_frame.step[0] +
                                  col * pyramid_frame.elemSize());
        }
      }
    }
  }

  // Create default weight multiplier if none was passed.
  if (weight_multiplier_ == NULL) {
    default_weight_multiplier_.reset(new FilterWeightMultiplier());
    weight_multiplier_ = default_weight_multiplier_.get();
  }
}

template <int C, class FilterWeightMultiplier>
constexpr int
PushPullFiltering<C, FilterWeightMultiplier>::BorderFromFilterType(
    FilterType filter_type) {
  // -1 indicates error (unknown filter_type).
  return (filter_type == BINOMIAL_3X3 || filter_type == GAUSSIAN_3X3)
             ? 1
             : ((filter_type == BINOMIAL_5X5 || filter_type == GAUSSIAN_5X5)
                    ? 2
                    : -1);
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::SetOptions(
    const PushPullOptions& options) {
  options_ = options;
  SetupBilateralLUT();
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::SetupFilters() {
  // Setup binomial weights.
  std::array<float, 25> bin5_weights{{1,  4, 6,  4,  1,  4, 16, 24, 16,
                                      4,  6, 24, 36, 24, 6, 4,  16, 24,
                                      16, 4, 1,  4,  6,  4, 1}};

  std::array<float, 9> bin3_weights{{1, 2, 1, 2, 4, 2, 1, 2, 1}};

  // Normalize maximum of binomial filters to 1.
  const float bin5_scale =
      (1.f / std::accumulate(bin5_weights.begin(), bin5_weights.end(), 0.0f));

  for (int i = 0; i < 25; ++i) {
    binomial5_weights_[i] = bin5_weights[i] * bin5_scale;
  }

  const float bin3_scale =
      (1.f / std::accumulate(bin3_weights.begin(), bin3_weights.end(), 0.0f));

  for (int i = 0; i < 9; ++i) {
    binomial3_weights_[i] = bin3_weights[i] * bin3_scale;
  }

  // Setup gaussian weights.
  const float space_coeff_5 = -0.5f / (1.6f * 1.6f);
  for (int j = 0; j < 5; ++j) {
    for (int i = 0; i < 5; ++i) {
      const float radius = (j - 2) * (j - 2) + (i - 2) * (i - 2);
      gaussian5_weights_[j * 5 + i] =
          std::exp(static_cast<double>(radius * space_coeff_5));
    }
  }

  const float space_coeff_3 = -0.5f / (1.0f * 1.0f);
  for (int j = 0; j < 3; ++j) {
    for (int i = 0; i < 3; ++i) {
      const float radius = (j - 1) * (j - 1) + (i - 1) * (i - 1);
      gaussian3_weights_[j * 3 + i] =
          std::exp(static_cast<double>(radius * space_coeff_3));
    }
  }

  // Normalize maximum of gaussian weights to 1 (center value has largest
  // magnitude).
  const float gauss5_scale =
      1.0f / std::accumulate(gaussian5_weights_.begin(),
                             gaussian5_weights_.end(), 0.0f);
  const float gauss3_scale =
      1.0f / std::accumulate(gaussian3_weights_.begin(),
                             gaussian3_weights_.end(), 0.0f);

  for (int i = 0; i < 25; ++i) {
    gaussian5_weights_[i] *= gauss5_scale;
  }

  for (int i = 0; i < 9; ++i) {
    gaussian3_weights_[i] *= gauss3_scale;
  }
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::SetupBilateralLUT() {
  // We use L1 color distance here, maximum is 3 (channels) * 256 (max
  // intensity).
  const int max_bins = 3 * 256;
  bilateral_lut_.resize(max_bins, 0.0f);

  const float sigma_color = options_.bilateral_sigma();
  const float gauss_color_coeff = -0.5f / (sigma_color * sigma_color);

  // Normalized such that first bin equals one.
  // Avoid non-zero weights for large intensity differences.
  for (int i = 0; i < max_bins; ++i) {
    bilateral_lut_[i] = std::max<float>(
        kBilateralEps,
        std::exp(static_cast<double>(i * i * gauss_color_coeff)));
  }
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::AllocatePyramid(
    const cv::Size& domain_size, int border, int type, bool allocate_base_level,
    std::vector<cv::Mat>* pyramid) {
  ABSL_CHECK(pyramid != nullptr);
  pyramid->clear();
  pyramid->reserve(16);  // Do not anticipate videos with dimensions
                         // larger than 2^16.

  int width = domain_size.width;
  int height = domain_size.height;

  if (allocate_base_level) {
    pyramid->push_back(cv::Mat(height + 2 * border, width + 2 * border, type));
  }

  while (width > 1 && height > 1) {
    width = (width + 1) / 2;
    height = (height + 1) / 2;
    pyramid->push_back(cv::Mat(height + 2 * border, width + 2 * border, type));
  }
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::InitializeImagePyramid(
    const cv::Mat& input_frame, std::vector<cv::Mat>* pyramid) {
  ABSL_CHECK(pyramid != nullptr);
  ABSL_CHECK_GT(pyramid->size(), 0);

  cv::Mat base_level((*pyramid)[0],
                     cv::Range(border_, (*pyramid)[0].rows - border_),
                     cv::Range(border_, (*pyramid)[0].cols - border_));
  ABSL_CHECK_EQ(base_level.rows, input_frame.rows);
  ABSL_CHECK_EQ(base_level.cols, input_frame.cols);
  ABSL_CHECK_EQ(base_level.type(), input_frame.type());

  input_frame.copyTo(base_level);
  CopyNecessaryBorder<uint8_t, 3>(&(*pyramid)[0]);

  for (int l = 0; l < pyramid->size() - 1; ++l) {
    cv::Mat source((*pyramid)[l],
                   cv::Range(border_, (*pyramid)[l].rows - border_),
                   cv::Range(border_, (*pyramid)[l].cols - border_));
    cv::Mat destination((*pyramid)[l + 1],
                        cv::Range(border_, (*pyramid)[l + 1].rows - border_),
                        cv::Range(border_, (*pyramid)[l + 1].cols - border_));
    cv::pyrDown(source, destination, destination.size());
    CopyNecessaryBorder<uint8_t, 3>(&(*pyramid)[l + 1]);
  }
}

template <int C, class FilterWeightMultiplier>
template <typename T, int channels>
void PushPullFiltering<C, FilterWeightMultiplier>::CopyNecessaryBorder(
    cv::Mat* mat) {
  switch (filter_type_) {
    case BINOMIAL_3X3:
    case GAUSSIAN_3X3:
      CopyMatBorder<T, 1, channels>(mat);
      break;
    case BINOMIAL_5X5:
    case GAUSSIAN_5X5:
      CopyMatBorder<T, 2, channels>(mat);
      break;
    default:
      ABSL_LOG(FATAL) << "Unknown filter";
  }
}

// In case of upsampling there are 4 possible anchor / image configurations
// that can occur. First the general layout of upsampling: x corresponds to
// positions with defined values, 0 to the space in-between.
// x 0 x 0 x 0 x 0
// 0 0 0 0 0 0 0 0
// x 0 x 0 x 0 x 0
// 0 0 0 0 0 0 0 0
// x 0 x 0 x 0 x 0

// The 4 cases for 3x3 filter are:
// Case 0:
// Filter incident with x: 1x1 filter
// Case 1:
// Filter incident to 0 at x0x: 1x2 filter
// Case 2:
// Filter incident to 0 at x : 2x1 filter
//                         0
//                         x
// Case 3:
// Filter incident to center 0 at x 0 x : 2x2 filter
//                                0 0 0
//                                x 0 x
// When traversing an to be upsampled image, for even rows we have to alternate
// between cases 0 and 1, for odd rows we alternate between 2 and 3.
// Computes the weights and tap offsets for above described cases, using
// sample increment in x direction of inc_x (e.g. color channels) and
// sample increment in y of inc_y (e.g. bytes in row).
// Optionally also selects space_offsets for bilateral filtering for each
// upsampling case, if space_offsets is specified. Reasoning here is, that
// tap offsets are defined in the image domain of the low-res frame one level
// above the currently processed one, while space_offsets are used to compute
// the corresponding joint bilateral weights in the high-res frame.
template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::GetUpsampleTaps3(
    const float* filter_weights,
    const std::vector<int>* space_offsets,  // optional.
    int inc_x, int inc_y, std::vector<float> tap_weights[4],
    std::vector<int> tap_offsets[4], std::vector<int> tap_space_offsets[4]) {
  // Taps for filter 0 1 2
  //                 3 4 5
  //                 6 7 8
  // Case 0:
  tap_weights[0].push_back(filter_weights[4]);
  tap_offsets[0].push_back(0);
  if (space_offsets) {
    tap_space_offsets[0].push_back((*space_offsets)[4]);
  }

  // Case 1:
  tap_weights[1].push_back(filter_weights[3]);
  tap_offsets[1].push_back(0);
  tap_weights[1].push_back(filter_weights[5]);
  tap_offsets[1].push_back(inc_x);
  if (space_offsets) {
    tap_space_offsets[1].push_back((*space_offsets)[3]);
    tap_space_offsets[1].push_back((*space_offsets)[5]);
  }

  // Case 2:
  tap_weights[2].push_back(filter_weights[1]);
  tap_offsets[2].push_back(0);
  tap_weights[2].push_back(filter_weights[7]);
  tap_offsets[2].push_back(inc_y);
  if (space_offsets) {
    tap_space_offsets[2].push_back((*space_offsets)[1]);
    tap_space_offsets[2].push_back((*space_offsets)[7]);
  }

  // Case 3:
  tap_weights[3].push_back(filter_weights[0]);
  tap_offsets[3].push_back(0);
  tap_weights[3].push_back(filter_weights[2]);
  tap_offsets[3].push_back(inc_x);
  tap_weights[3].push_back(filter_weights[6]);
  tap_offsets[3].push_back(inc_y);
  tap_weights[3].push_back(filter_weights[8]);
  tap_offsets[3].push_back(inc_y + inc_x);
  if (space_offsets) {
    tap_space_offsets[3].push_back((*space_offsets)[0]);
    tap_space_offsets[3].push_back((*space_offsets)[2]);
    tap_space_offsets[3].push_back((*space_offsets)[6]);
    tap_space_offsets[3].push_back((*space_offsets)[8]);
  }
}

// For 5x5 filter.
// Case 0:
// Filter incident to center x at x 0 x 0 x:  3x3 filter
//                                0 0 0 0 0
//                                x 0 x 0 x
//                                0 0 0 0 0
//                                x 0 x 0 x
// Case 1:
// Filter incident to center 0 at  x 0 x: 3x2 filter
//                                 0 0 0
//                                 x 0 x
//                                 0 0 0
//                                 x 0 x
// Case 2:
// Filter incident to center 0 at x 0 x 0 x: 2x3 filter
//                                0 0 0 0 0
//                                x 0 x 0 x
// Case 3:
// Filter incident to center 0 at x 0 x : 2x2 filter
//                                0 0 0
//                                x 0 x
// Same as above for 5x5 filter.
template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::GetUpsampleTaps5(
    const float* filter_weights, const std::vector<int>* space_offsets,
    int inc_x, int inc_y, std::vector<float> tap_weights[4],
    std::vector<int> tap_offsets[4], std::vector<int> tap_space_offsets[4]) {
  // Taps for filter 0  1  2  3  4
  //                 5  6  7  8  9
  //                 10 11 12 13 14
  //                 15 16 17 18 19
  //                 20 21 22 23 24
  // Case 0:
  tap_weights[0].push_back(filter_weights[0]);
  tap_weights[0].push_back(filter_weights[2]);
  tap_weights[0].push_back(filter_weights[4]);
  tap_weights[0].push_back(filter_weights[10]);
  tap_weights[0].push_back(filter_weights[12]);
  tap_weights[0].push_back(filter_weights[14]);
  tap_weights[0].push_back(filter_weights[20]);
  tap_weights[0].push_back(filter_weights[22]);
  tap_weights[0].push_back(filter_weights[24]);
  tap_offsets[0].push_back(-inc_y - inc_x);
  tap_offsets[0].push_back(-inc_y);
  tap_offsets[0].push_back(-inc_y + inc_x);
  tap_offsets[0].push_back(-inc_x);
  tap_offsets[0].push_back(0);
  tap_offsets[0].push_back(inc_x);
  tap_offsets[0].push_back(inc_y - inc_x);
  tap_offsets[0].push_back(inc_y);
  tap_offsets[0].push_back(inc_y + inc_x);

  if (space_offsets) {
    tap_space_offsets[0].push_back((*space_offsets)[0]);
    tap_space_offsets[0].push_back((*space_offsets)[2]);
    tap_space_offsets[0].push_back((*space_offsets)[4]);
    tap_space_offsets[0].push_back((*space_offsets)[10]);
    tap_space_offsets[0].push_back((*space_offsets)[12]);
    tap_space_offsets[0].push_back((*space_offsets)[14]);
    tap_space_offsets[0].push_back((*space_offsets)[20]);
    tap_space_offsets[0].push_back((*space_offsets)[22]);
    tap_space_offsets[0].push_back((*space_offsets)[24]);
  }

  // Case 1:
  tap_weights[1].push_back(filter_weights[1]);
  tap_weights[1].push_back(filter_weights[3]);
  tap_weights[1].push_back(filter_weights[11]);
  tap_weights[1].push_back(filter_weights[13]);
  tap_weights[1].push_back(filter_weights[21]);
  tap_weights[1].push_back(filter_weights[23]);
  tap_offsets[1].push_back(-inc_y);
  tap_offsets[1].push_back(-inc_y + inc_x);
  tap_offsets[1].push_back(0);
  tap_offsets[1].push_back(inc_x);
  tap_offsets[1].push_back(inc_y);
  tap_offsets[1].push_back(inc_y + inc_x);

  if (space_offsets) {
    tap_space_offsets[1].push_back((*space_offsets)[1]);
    tap_space_offsets[1].push_back((*space_offsets)[3]);
    tap_space_offsets[1].push_back((*space_offsets)[11]);
    tap_space_offsets[1].push_back((*space_offsets)[13]);
    tap_space_offsets[1].push_back((*space_offsets)[21]);
    tap_space_offsets[1].push_back((*space_offsets)[23]);
  }

  // Repeating for readability.
  // Taps for filter 0  1  2  3  4
  //                 5  6  7  8  9
  //                 10 11 12 13 14
  //                 15 16 17 18 19
  //                 20 21 22 23 24
  // Case 2:
  tap_weights[2].push_back(filter_weights[5]);
  tap_weights[2].push_back(filter_weights[7]);
  tap_weights[2].push_back(filter_weights[9]);
  tap_weights[2].push_back(filter_weights[15]);
  tap_weights[2].push_back(filter_weights[17]);
  tap_weights[2].push_back(filter_weights[19]);
  tap_offsets[2].push_back(-inc_x);
  tap_offsets[2].push_back(0);
  tap_offsets[2].push_back(inc_x);
  tap_offsets[2].push_back(inc_y - inc_x);
  tap_offsets[2].push_back(inc_y);
  tap_offsets[2].push_back(inc_y + inc_x);

  if (space_offsets) {
    tap_space_offsets[2].push_back((*space_offsets)[5]);
    tap_space_offsets[2].push_back((*space_offsets)[7]);
    tap_space_offsets[2].push_back((*space_offsets)[9]);
    tap_space_offsets[2].push_back((*space_offsets)[15]);
    tap_space_offsets[2].push_back((*space_offsets)[17]);
    tap_space_offsets[2].push_back((*space_offsets)[19]);
  }

  // Case 3:
  tap_weights[3].push_back(filter_weights[6]);
  tap_weights[3].push_back(filter_weights[8]);
  tap_weights[3].push_back(filter_weights[16]);
  tap_weights[3].push_back(filter_weights[18]);
  tap_offsets[3].push_back(0);
  tap_offsets[3].push_back(inc_x);
  tap_offsets[3].push_back(inc_y);
  tap_offsets[3].push_back(inc_y + inc_x);

  if (space_offsets) {
    tap_space_offsets[3].push_back((*space_offsets)[6]);
    tap_space_offsets[3].push_back((*space_offsets)[8]);
    tap_space_offsets[3].push_back((*space_offsets)[16]);
    tap_space_offsets[3].push_back((*space_offsets)[18]);
  }
}

// Scattered data interpolation via PushPull algorithm.
// Interpolation is performed over a size of domain_size, where
// features are placed into the corresponding bin at location
// feature.[x()|y()] + origin.
// The output displacement is assumed be a 3 channel float image of
// size == domain_sz + 2 to account for a one pixel border.
template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::PerformPushPull(
    const std::vector<Vector2_f>& data_locations,
    const std::vector<cv::Vec<float, C>>& data_values, float push_pull_weight,
    cv::Point2i origin, int readout_level,
    const std::vector<float>* data_weights, const cv::Mat* input_frame,
    cv::Mat* results) {
  ABSL_CHECK_EQ(data_locations.size(), data_values.size());
  ABSL_CHECK(results != nullptr);

  if (data_weights) {
    ABSL_CHECK_EQ(data_weights->size(), data_locations.size());
  }

  origin.x += border_;
  origin.y += border_;

  // Create mip-map view from downsample pyramid.
  std::vector<cv::Mat*> mip_map(PyramidLevels());

  for (int i = 0; i < mip_map.size(); ++i) {
    mip_map[i] = &downsample_pyramid_[i];
  }

  ABSL_CHECK_GE(readout_level, 0);
  ABSL_CHECK_LT(readout_level, PyramidLevels());

  // CHECK if passed results matrix is compatible w.r.t. type and domain.
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].cols, results->cols);
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].rows, results->rows);
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].type(), results->type());

  // Use caller-allocated results Mat.
  mip_map[readout_level] = results;

  // Place data_values into their final positions in mip map @ level 0.
  mip_map[0]->setTo(0);
  for (int idx = 0; idx < data_locations.size(); ++idx) {
    const Vector2_f& location = data_locations[idx];
    const cv::Vec<float, C>& value = data_values[idx];

    float* ptr = mip_map[0]->ptr<float>(static_cast<int>(location.y() + 0.5f) +
                                        origin.y) +
                 (C + 1) * (static_cast<int>(location.x() + 0.5f) + origin.x);

    const float data_weight =
        data_weights ? (*data_weights)[idx] : push_pull_weight;

    // Pre-multiply with data_weight.
    for (int c = 0; c < C; ++c) {
      ptr[c] = value[c] * data_weight;
    }

    // A weight of 1 would assume zero noise in the displacements. Smaller
    // values lead to a smoother interpolation that approximates the
    // initial values.
    ptr[C] = data_weight;
  }

  PerformPushPullImpl(readout_level, input_frame, &mip_map);
}

// This is the same as PerformPushPull above except that it
// assumes that the data (the mip_map at level 0) is given as a cv::Mat.
template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::PerformPushPullMat(
    const cv::Mat& mip_map_level_0,
    int readout_level,           // Default: 0.
    const cv::Mat* input_frame,  // Optional.
    cv::Mat* results) {
  ABSL_CHECK(results != nullptr);

  // Create mip-map view (concat displacements with downsample_pyramid).
  std::vector<cv::Mat*> mip_map(PyramidLevels());

  for (int i = 0; i < mip_map.size(); ++i) {
    mip_map[i] = &downsample_pyramid_[i];
  }

  ABSL_CHECK_GE(readout_level, 0);
  ABSL_CHECK_LT(readout_level, PyramidLevels());

  // CHECK if passed mip_map at level[0] is compatible w.r.t. type and domain.
  ABSL_CHECK_EQ(mip_map_level_0.cols, results->cols);
  ABSL_CHECK_EQ(mip_map_level_0.rows, results->rows);
  ABSL_CHECK_EQ(mip_map_level_0.type(), results->type());

  // CHECK if passed results matrix is compatible w.r.t. type and domain.
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].cols, results->cols);
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].rows, results->rows);
  ABSL_CHECK_EQ(downsample_pyramid_[readout_level].type(), results->type());

  // Use caller-allocated results Mat.
  mip_map[readout_level] = results;

  // Place data_values into their final positions in mip map at level 0.
  mip_map_level_0.copyTo(*mip_map[0]);

  PerformPushPullImpl(readout_level, input_frame, &mip_map);
}

// Perform sparse data interpolation.
// Generate filter weights and then perform pull-down sampling
// followed by push-up sampling.
// Assumes that the mip_map has already been allocated and
// the data inserted at level 0.
// Results are placed in (*mip_map_ptr)[readout_level].
template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::PerformPushPullImpl(
    const int readout_level, const cv::Mat* input_frame,
    std::vector<cv::Mat*>* mip_map_ptr) {
  const float* filter_weights;
  int num_filter_elems;
  switch (filter_type_) {
    case BINOMIAL_3X3:
      num_filter_elems = 9;
      filter_weights = binomial3_weights_.data();
      break;
    case BINOMIAL_5X5:
      num_filter_elems = 25;
      filter_weights = binomial5_weights_.data();
      break;
    case GAUSSIAN_3X3:
      num_filter_elems = 9;
      filter_weights = gaussian3_weights_.data();
      break;
    case GAUSSIAN_5X5:
      num_filter_elems = 25;
      filter_weights = gaussian5_weights_.data();
      break;
    default:
      ABSL_LOG(FATAL) << "Unknown filter requested.";
  }

  const std::vector<cv::Mat*>& mip_map = *mip_map_ptr;

  // Borderless views into mip maps.
  std::vector<cv::Mat> mip_map_views(mip_map.size());
  std::vector<const cv::Mat*> mip_map_view_ptrs(mip_map.size());
  for (int l = 0; l < mip_map.size(); ++l) {
    mip_map_views[l] =
        cv::Mat(*mip_map[l], cv::Range(border_, mip_map[l]->rows - border_),
                cv::Range(border_, mip_map[l]->cols - border_));

    mip_map_view_ptrs[l] = &mip_map_views[l];
  }

  if (use_bilateral_) {
    ABSL_CHECK(input_frame != nullptr);
    InitializeImagePyramid(*input_frame, &input_frame_pyramid_);
  }

  PullDownSampling(num_filter_elems, filter_weights, mip_map_ptr);

  if (mip_map_visualizer_) {
    std::vector<bool> is_premultiplied(mip_map_view_ptrs.size(), true);
    mip_map_visualizer_->Visualize(mip_map_view_ptrs, true, is_premultiplied);
  }

  PushUpSampling(num_filter_elems, filter_weights, readout_level, mip_map_ptr);

  if (mip_map_visualizer_) {
    std::vector<bool> is_premultiplied(mip_map_view_ptrs.size(), true);
    is_premultiplied[readout_level] = false;
    mip_map_visualizer_->Visualize(mip_map_view_ptrs, false, is_premultiplied);
  }
}

template <class T>
inline const T* PtrOffset(const T* ptr, int offset) {
  return reinterpret_cast<const T*>(reinterpret_cast<const uint8_t*>(ptr) +
                                    offset);
}

inline void GetFilterOffsets(const cv::Mat& mat, int border, int channels,
                             std::vector<int>* filter_offsets) {
  for (int i = -border; i <= border; ++i) {
    for (int j = -border; j <= border; ++j) {
      filter_offsets->push_back(i * mat.step[0] + sizeof(float) * channels * j);
    }
  }
}

// Perform filter operation at locations in zero_pos.
template <int C>
inline void FillInZeros(const std::vector<float*>& zero_pos,
                        int num_filter_elems, const float* filter_weights,
                        int border, cv::Mat* mat) {
  std::vector<int> filter_offsets;
  GetFilterOffsets(*mat, border, C + 1, &filter_offsets);
  for (float* zero_ptr : zero_pos) {
    float weight_sum = 0;
    float val_sum[C];
    memset(val_sum, 0, C * sizeof(val_sum[0]));

    for (int k = 0; k < num_filter_elems; ++k) {
      const float* cur_ptr = PtrOffset(zero_ptr, filter_offsets[k]);
      const float w = filter_weights[k] * cur_ptr[C];

      for (int c = 0; c < C; ++c) {
        val_sum[c] += cur_ptr[c] * w;
      }

      weight_sum += w;
    }

    if (weight_sum > 0) {
      const float inv_weight_sum = 1.f / weight_sum;
      for (int c = 0; c < C; ++c) {
        val_sum[c] *= inv_weight_sum;
        zero_ptr[c] = val_sum[c];
      }
    }
  }
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::PullDownSampling(
    int num_filter_elems, const float* filter_weights,
    std::vector<cv::Mat*>* mip_map_ptr) {
  const std::vector<cv::Mat*>& mip_map = *mip_map_ptr;

  // Filter pyramid via push-pull.
  // We always filter from [border, border] to
  // [width - 1 - border, height - 1 - border].

  for (int l = 1; l < mip_map.size(); ++l) {
    CopyNecessaryBorder<float, C + 1>(mip_map[l - 1]);
    mip_map[l]->setTo(0);

    // Local copy for faster access.
    const int border = border_;
    const int channels = C + 1;

    // Signal level to weight_multiplier.
    weight_multiplier_->SetLevel(l - 1, true);

    std::vector<int> filter_offsets;
    GetFilterOffsets(*mip_map[l - 1], border, channels, &filter_offsets);

    const std::vector<int>* space_offsets =
        use_bilateral_ ? &pyramid_space_offsets_[l - 1] : NULL;

    const int height = mip_map[l]->rows - 2 * border;
    const int width = mip_map[l]->cols - 2 * border;

    // Downweight bilateral influence as level progress as due to iterative
    // downsampling image becomes less and less reliable.
    const float bilateral_scale =
        std::pow(options_.pull_bilateral_scale(), l - 1);

    // Filter odd pixels (downsample).
    for (int i = 0; i < height; ++i) {
      float* dst_ptr = mip_map[l]->ptr<float>(i + border) + border * channels;
      const float* src_ptr =
          mip_map[l - 1]->ptr<float>(2 * i + border) + border * channels;
      const uint8_t* img_ptr =
          use_bilateral_ ? (input_frame_pyramid_[l - 1].template ptr<uint8_t>(
                                2 * i + border) +
                            border * 3)
                         : NULL;

      for (int j = 0; j < width; ++j, dst_ptr += channels,
               src_ptr += 2 * channels, img_ptr += 2 * 3) {
        float weight_sum = 0;
        float val_sum[C];
        memset(val_sum, 0, C * sizeof(val_sum[0]));

        const int i2 = i * 2;
        const int j2 = j * 2;
        if (use_bilateral_) {
          for (int k = 0; k < num_filter_elems; ++k) {
            const float* cur_ptr = PtrOffset(src_ptr, filter_offsets[k]);

            // If neighbor is not important, skip further evaluation.
            if (cur_ptr[C] < kBilateralEps * kBilateralEps) {
              continue;
            }

            const uint8_t* match_ptr = PtrOffset(img_ptr, (*space_offsets)[k]);

            float bilateral_w = bilateral_lut_[ColorDiffL1(img_ptr, match_ptr) *
                                               bilateral_scale];

            const float multiplier = weight_multiplier_->GetWeight(
                src_ptr, cur_ptr, img_ptr, j2, i2);

            const float w = filter_weights[k] * bilateral_w * multiplier;

            // cur_ptr is already pre-multiplied with importance
            // weight cur_ptr[C].
            for (int c = 0; c < C; ++c) {
              val_sum[c] += cur_ptr[c] * w;
            }
            weight_sum += w * cur_ptr[C];
          }
        } else {
          for (int k = 0; k < num_filter_elems; ++k) {
            const float* cur_ptr = PtrOffset(src_ptr, filter_offsets[k]);
            const float multiplier =
                weight_multiplier_->GetWeight(src_ptr, cur_ptr, NULL, j2, i2);
            const float w = filter_weights[k] * multiplier;

            // cur_ptr is already pre-multiplied with importance
            // weight cur_ptr[C].
            for (int c = 0; c < C; ++c) {
              val_sum[c] += cur_ptr[c] * w;
            }

            weight_sum += w * cur_ptr[C];
          }
        }

        ABSL_DCHECK_GE(weight_sum, 0);

        if (weight_sum >= kBilateralEps * kBilateralEps) {
          const float inv_weight_sum = 1.f / weight_sum;
          for (int c = 0; c < C; ++c) {
            dst_ptr[c] = val_sum[c] * inv_weight_sum;
          }
        } else {
          for (int c = 0; c <= C; ++c) {
            dst_ptr[c] = 0;
          }
        }

        const float prop_scale = options_.pull_propagation_scale();
        weight_sum *= prop_scale;
        dst_ptr[C] = std::min<float>(1.0f, weight_sum);
      }
    }

    if (weight_adjuster_) {
      CopyNecessaryBorder<float, C + 1>(mip_map[l]);
      cv::Mat mip_map_view(*mip_map[l],
                           cv::Range(border_, mip_map[l]->rows - border_),
                           cv::Range(border_, mip_map[l]->cols - border_));
      cv::Mat image_view;
      if (use_bilateral_) {
        image_view =
            cv::Mat(input_frame_pyramid_[l],
                    cv::Range(border_, input_frame_pyramid_[l].rows - border_),
                    cv::Range(border_, input_frame_pyramid_[l].cols - border_));
      }
      weight_adjuster_->AdjustWeights(
          l, true, use_bilateral_ ? &image_view : NULL, &mip_map_view);
    }

    // Pre-multiply weight for next level.
    for (int i = 0; i < height; ++i) {
      float* data_ptr = mip_map[l]->ptr<float>(i + border_) + border * channels;
      for (int j = 0; j < width; ++j, data_ptr += channels) {
        for (int c = 0; c < C; ++c) {
          data_ptr[c] *= data_ptr[C];
        }
      }
    }
  }  // end level processing.
}

template <int C, class FilterWeightMultiplier>
void PushPullFiltering<C, FilterWeightMultiplier>::PushUpSampling(
    int num_filter_elems, const float* filter_weights, int readout_level,
    std::vector<cv::Mat*>* mip_map_ptr) {
  const std::vector<cv::Mat*>& mip_map = *mip_map_ptr;

  for (int l = mip_map.size() - 2; l >= readout_level; --l) {
    CopyNecessaryBorder<float, C + 1>(mip_map[l + 1]);

    // Signal mip map level to weight_multiplier.
    weight_multiplier_->SetLevel(l, false);

    // Instead of upsampling we use 4 special tap filters. See comment at above
    // function.
    std::vector<float> tap_weights[4];
    std::vector<int> tap_offsets[4];
    std::vector<int> tap_space_offsets[4];
    const int channels = C + 1;

    switch (filter_type_) {
      case BINOMIAL_3X3:
      case GAUSSIAN_3X3:
        GetUpsampleTaps3(filter_weights,
                         use_bilateral_ ? &pyramid_space_offsets_[l] : NULL,
                         channels * sizeof(float), mip_map[l + 1]->step[0],
                         tap_weights, tap_offsets, tap_space_offsets);
        break;
      case BINOMIAL_5X5:
      case GAUSSIAN_5X5:
        GetUpsampleTaps5(filter_weights,
                         use_bilateral_ ? &pyramid_space_offsets_[l] : NULL,
                         channels * sizeof(float), mip_map[l + 1]->step[0],
                         tap_weights, tap_offsets, tap_space_offsets);
        break;
      default:
        ABSL_LOG(FATAL) << "Filter unknown";
    }

    // Local copy for faster access.
    const int border = border_;
    const int height = mip_map[l]->rows - 2 * border;
    const int width = mip_map[l]->cols - 2 * border;

    const float bilateral_scale =
        std::pow(options_.push_bilateral_scale(), l + 1);

    // Apply filter.
    // List of zero positions that need to be smoothed.
    std::vector<float*> zero_pos;

    for (int i = 0; i < height; ++i) {
      float* dst_ptr = mip_map[l]->ptr<float>(i + border) + border * channels;
      const float* src_ptr =
          mip_map[l + 1]->ptr<float>(i / 2 + border) + border * channels;
      const uint8_t* img_ptr =
          use_bilateral_
              ? (input_frame_pyramid_[l].template ptr<uint8_t>(i + border) +
                 border * 3)
              : NULL;

      // Select tap offset.
      const int tap_kind_row = 2 * (i % 2);  // odd row, case 2 & 3.

      for (int j = 0; j < width;
           // Increase src_ptr only for even rows (i.e. previous one was odd).
           src_ptr += channels * (j % 2),
               ++j, dst_ptr += channels, img_ptr += 3) {
        if (dst_ptr[C] >= 1) {  // Skip if already saturated.
          continue;
        }

        const int tap_kind = tap_kind_row + j % 2;
        const std::vector<float>& tap_weight = tap_weights[tap_kind];
        const std::vector<int>& tap_offset = tap_offsets[tap_kind];
        const int tap_size = tap_weight.size();

        float weight_sum = 0;
        float val_sum[C];
        memset(val_sum, 0, C * sizeof(val_sum[0]));

        if (use_bilateral_) {
          const std::vector<int>& tap_space_offset =
              tap_space_offsets[tap_kind];
          for (int k = 0; k < tap_size; ++k) {
            const float* cur_ptr = PtrOffset(src_ptr, tap_offset[k]);

            // If neighbor is not important, skip further evaluation.
            if (cur_ptr[C] < kBilateralEps * kBilateralEps) {
              continue;
            }

            const uint8_t* match_ptr = PtrOffset(img_ptr, tap_space_offset[k]);
            float bilateral_w = bilateral_lut_[ColorDiffL1(img_ptr, match_ptr) *
                                               bilateral_scale];

            const float multiplier =
                weight_multiplier_->GetWeight(src_ptr, cur_ptr, img_ptr, j, i);

            const float w = tap_weight[k] * bilateral_w * multiplier;

            // Values in above mip map level are pre-multiplied by
            // importance weight cur_ptr[C].
            for (int c = 0; c < C; ++c) {
              val_sum[c] += cur_ptr[c] * w;
            }
            weight_sum += w * cur_ptr[C];
          }
        } else {
          for (int k = 0; k < tap_size; ++k) {
            const float* cur_ptr = PtrOffset(src_ptr, tap_offset[k]);
            const float multiplier =
                weight_multiplier_->GetWeight(src_ptr, cur_ptr, NULL, j, i);

            const float w = tap_weight[k] * multiplier;

            // Values in above mip map level are pre-multiplied by weight
            // cur_ptr[C].
            for (int c = 0; c < C; ++c) {
              val_sum[c] += cur_ptr[c] * w;
            }

            weight_sum += w * cur_ptr[C];
          }
        }

        if (weight_sum >= kBilateralEps * kBilateralEps) {
          const float inv_weight_sum = 1.f / weight_sum;
          for (int c = 0; c < C; ++c) {
            val_sum[c] *= inv_weight_sum;
          }
        } else {
          weight_sum = 0;
          for (int c = 0; c < C; ++c) {
            val_sum[c] = 0;
          }

          zero_pos.push_back(dst_ptr);
        }

        const float prop_scale = options_.push_propagation_scale();
        weight_sum *= prop_scale;

        // Maximum influence of pushed result on current pixel.
        const float alpha_inv = std::min(1.0f - dst_ptr[C], weight_sum);
        const float denom =
            1.0f / (dst_ptr[C] + alpha_inv + kBilateralEps * kBilateralEps);

        // Blend (dst_ptr is premultiplied with weight dst_ptr[C],
        //        val_sum is normalized).
        for (int c = 0; c < C; ++c) {
          dst_ptr[c] = (dst_ptr[c] + val_sum[c] * alpha_inv) * denom;
        }

        // Increase current confidence by above sample.
        dst_ptr[C] =
            std::min(1.0f, dst_ptr[C] + std::min(weight_sum, alpha_inv));
      }
    }

    if (weight_adjuster_) {
      CopyNecessaryBorder<float, C + 1>(mip_map[l]);
      cv::Mat mip_map_view(*mip_map[l],
                           cv::Range(border_, mip_map[l]->rows - border_),
                           cv::Range(border_, mip_map[l]->cols - border_));
      cv::Mat image_view;
      if (use_bilateral_) {
        image_view =
            cv::Mat(input_frame_pyramid_[l],
                    cv::Range(border, input_frame_pyramid_[l].rows - border),
                    cv::Range(border, input_frame_pyramid_[l].cols - border));
      }
      weight_adjuster_->AdjustWeights(
          l, false, use_bilateral_ ? &image_view : NULL, &mip_map_view);
    }

    // Pre-multiply with weight for next level if haven't reached base level
    // yet. (Base level is not pre-multiplied so result can be used directly).
    if (l != readout_level) {
      for (int i = 0; i < height; ++i) {
        float* data_ptr =
            mip_map[l]->ptr<float>(i + border_) + border * channels;
        for (int j = 0; j < width; ++j, data_ptr += channels) {
          for (int c = 0; c < C; ++c) {
            data_ptr[c] *= data_ptr[C];
          }
        }
      }
    } else {
      CopyNecessaryBorder<float, C + 1>(mip_map[l]);
      FillInZeros<C>(zero_pos, num_filter_elems, filter_weights, border_,
                     mip_map[l]);
    }
  }  // end mip map levels.
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_PUSH_PULL_FILTERING_H_
