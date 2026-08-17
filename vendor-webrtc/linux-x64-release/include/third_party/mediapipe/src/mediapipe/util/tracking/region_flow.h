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
// Small helper function for RegionFlow.
#ifndef MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_H_
#define MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_H_

#include <algorithm>
#include <cmath>
#include <deque>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/motion_models.h"
#include "mediapipe/util/tracking/region_flow.pb.h"

namespace mediapipe {

typedef RegionFlowFrame::RegionFlow RegionFlow;
typedef std::vector<RegionFlowFeature*> RegionFlowFeatureView;

inline RegionFlowFeature FeatureFromFloats(float x, float y, float dx,
                                           float dy) {
  RegionFlowFeature feat;
  feat.set_x(x);
  feat.set_y(y);
  feat.set_dx(dx);
  feat.set_dy(dy);
  return feat;
}

inline RegionFlowFeature FeatureFromVec2f(const Vector2_f& loc,
                                          const Vector2_f& flow) {
  RegionFlowFeature feat;
  feat.set_x(loc.x());
  feat.set_y(loc.y());
  feat.set_dx(flow.x());
  feat.set_dy(flow.y());
  return feat;
}

inline Vector2_f FeatureFlow(const RegionFlowFeature& feature) {
  return Vector2_f(feature.dx(), feature.dy());
}

inline Vector2_f FeatureLocation(const RegionFlowFeature& feature) {
  return Vector2_f(feature.x(), feature.y());
}

inline Vector2_f FeatureMatchLocation(const RegionFlowFeature& feature) {
  return FeatureLocation(feature) + FeatureFlow(feature);
}

inline Vector2_i FeatureIntLocation(const RegionFlowFeature& feature) {
  return Vector2_i::Cast(FeatureLocation(feature) + Vector2_f(0.5f, 0.5f));
}

inline Vector2_i FeatureMatchIntLocation(const RegionFlowFeature& feature) {
  return Vector2_i::Cast(FeatureMatchLocation(feature) + Vector2_f(0.5, 0.5f));
}

// Returns L1 norm of color standard deviation of feature descriptor,
// -1 if descriptor information is not present
// (e.g. if ComputeRegionFlowFeatureDescriptors was not called previously).
// Specifically returns stdev_red + stdev_blue + stdev_green.
inline float PatchDescriptorColorStdevL1(const PatchDescriptor& descriptor) {
  constexpr int kRedIdx = 3;
  constexpr int kGreenIdx = 6;
  constexpr int kBlueIdx = 8;
  ABSL_DCHECK_GE(descriptor.data(kRedIdx), 0);
  ABSL_DCHECK_GE(descriptor.data(kGreenIdx), 0);
  ABSL_DCHECK_GE(descriptor.data(kBlueIdx), 0);

  if (descriptor.data_size() > kBlueIdx) {
    return std::sqrt(descriptor.data(kRedIdx)) +
           std::sqrt(descriptor.data(kGreenIdx)) +
           std::sqrt(descriptor.data(kBlueIdx));
  } else {
    return -1.0f;
  }
}

// Extracts features from region flow. Set distance_from_border > 0 to ensure
// feature and matched location are at least the specified distance away
// from the frame rectangle (test is not executed if distance_from_border <= 0),
// so that feature descriptors can be computed (see function below).
void GetRegionFlowFeatureList(const RegionFlowFrame& flow_frame,
                              int distance_from_border,
                              RegionFlowFeatureList* flow_feature_list);

// Returns L2 norm of difference of mean color (first 3 dimensions of
// feature descriptors).
float RegionFlowFeatureDistance(const PatchDescriptor& patch_desc_1,
                                const PatchDescriptor& patch_desc_2);

// Resets IRLS weight of each RegionFlowFeature to value.
void ResetRegionFlowFeatureIRLSWeights(
    float value, RegionFlowFeatureList* flow_feature_list);

// Returns sum of feature's irls weights.
double RegionFlowFeatureIRLSSum(const RegionFlowFeatureList& feature_list);

// Computes per region flow feature texturedness score. Score is within [0, 1],
// where 0 means low texture and 1 high texture. Requires for each feature
// descriptor to be computed (via ComputeRegionFlowFeatureDescriptors). If
// missing, ABSL_LOG(WARNING) is issued and value defaults to 1.
// If use_15percent_as_max is set, score is scaled and threshold back to [0, 1]
// such that 1 is assumed at 15% of maximum PER channel variance.
void ComputeRegionFlowFeatureTexturedness(
    const RegionFlowFeatureList& region_flow_feature_list,
    bool use_15percent_as_max, std::vector<float>* texturedness);

// IRLS weights are multiplied by inverse texturedness (expressed as variance
// of feature descriptor), effectively upweighting outliers if in low textured
// areas. Features with texturedness below low_texture_threshold can be
// optionally clamped to low_texture_outlier_clamp (set to -1 for no clamping).
void TextureFilteredRegionFlowFeatureIRLSWeights(
    float low_texture_threshold, float low_texture_outlier_clamp,
    RegionFlowFeatureList* flow_feature_list);

// Same as above but normalizes w.r.t. corner response.
void CornerFilteredRegionFlowFeatureIRLSWeights(
    float low_corner_threshold, float low_corner_outlier_clamp,
    RegionFlowFeatureList* flow_feature_list);

// Simple setter and getter methods for irls weights.
void GetRegionFlowFeatureIRLSWeights(
    const RegionFlowFeatureList& flow_feature_list,
    std::vector<float>* irls_weights);

void SetRegionFlowFeatureIRLSWeights(const std::vector<float>& irls_weights,
                                     RegionFlowFeatureList* flow_feature_list);

// Counts number of region flow features with an irls weight of less than or
// equal to threshold.
int CountIgnoredRegionFlowFeatures(
    const RegionFlowFeatureList& flow_feature_list, float threshold);

// Locates region with id region_id in RegionFlowFrame via binary search.
// Returns NULL if no region with specied region_id is present.
const RegionFlow* GetRegionFlowById(int region_id,
                                    const RegionFlowFrame& flow_frame);

// Same as above for mutable RegionFlow.
RegionFlow* GetMutableRegionFlowById(int region_id,
                                     RegionFlowFrame* flow_frame);

void SortRegionFlowById(RegionFlowFrame* flow_frame);

// Switches each feature with its correspondence, i.e.
// (x, y) <-> (x + dx, x + dy), (dx, dy) -> (-dx, -dy). Same holds for centroid
// and mean flow. Note: Member fundamental_matrix is invalid after inversion,
// FeaturePointLists are not affected from inversion.
void InvertRegionFlow(const RegionFlowFrame& flow_frame,
                      RegionFlowFrame* inverted_flow_frame);

// Same as above for feature lists.
void InvertRegionFlowFeatureList(const RegionFlowFeatureList& feature_list,
                                 RegionFlowFeatureList* inverted_feature_list);

// Inverts a single feature.
void InvertRegionFlowFeature(RegionFlowFeature* feature);

// Removes features that are out bounds of the domain:
// [bounds, frame_width - bounds] x [bounds, frame_height - bounds].
void LimitFeaturesToBounds(int frame_width, int frame_height, float bounds,
                           RegionFlowFeatureList* feature_list);

// List of saliency points for each frame.
typedef std::deque<SalientPointFrame> SaliencyPointList;

// Normalizes region flow by frame diameter, i.e. uniform downscale such that
// feature list fits within [0, 1].
void NormalizeRegionFlowFeatureList(RegionFlowFeatureList* feature_list);
// Inverse of above operation.
void DeNormalizeRegionFlowFeatureList(RegionFlowFeatureList* feature_list);

// Templated implementations.
// Applies model to each feature and displacement vector.
template <class Model>
void TransformRegionFlowFeatureList(const Model& model,
                                    RegionFlowFeatureList* flow_feature_list) {
  for (auto& feature : *flow_feature_list->mutable_feature()) {
    Vector2_f pt =
        ModelAdapter<Model>::TransformPoint(model, FeatureLocation(feature));
    Vector2_f match = ModelAdapter<Model>::TransformPoint(
        model, FeatureMatchLocation(feature));
    feature.set_x(pt.x());
    feature.set_y(pt.y());
    feature.set_dx(match.x() - pt.x());
    feature.set_dy(match.y() - pt.y());
  }
}

// Similar to above but applies transformation to each feature to derive
// matching location, according to the formula:
// (dx, dy) <-- a * (transformed location - location) + b * (dx, dy)
// e.g. with b = 0, and a = 1, (dx, dy) is replaced with distance
// between transformed location and original location.
// If set_match == true, the original feature location is replaced
// with its matching location.
template <class Model>
void RegionFlowFeatureListViaTransform(
    const Model& model, RegionFlowFeatureList* flow_feature_list, float a,
    float b, bool set_match, const MixtureRowWeights* row_weights = nullptr) {
  for (auto& feature : *flow_feature_list->mutable_feature()) {
    Vector2_f match =
        ModelAdapter<Model>::TransformPoint(model, FeatureLocation(feature));
    feature.set_dx(b * feature.dx() + a * (match.x() - feature.x()));
    feature.set_dy(b * feature.dy() + a * (match.y() - feature.y()));
    if (set_match) {
      feature.set_x(match.x());
      feature.set_y(match.y());
    }
  }
}

template <>
inline void RegionFlowFeatureListViaTransform(
    const MixtureHomography& mix, RegionFlowFeatureList* flow_feature_list,
    float a, float b, bool set_match, const MixtureRowWeights* row_weights) {
  ABSL_CHECK(row_weights) << "Row weights required for mixtures.";

  for (auto& feature : *flow_feature_list->mutable_feature()) {
    const float* weights = row_weights->RowWeights(feature.y());
    Vector2_f match = MixtureHomographyAdapter::TransformPoint(
        mix, weights, FeatureLocation(feature));
    feature.set_dx(b * feature.dx() + a * (match.x() - feature.x()));
    feature.set_dy(b * feature.dy() + a * (match.y() - feature.y()));
    if (set_match) {
      feature.set_x(match.x());
      feature.set_y(match.y());
    }
  }
}

// Helper implementation function for functions below.
// Returns pair of <filtered weight, predicate result> where predicate result is
// the boolean result of applying the predicate to the feature.
template <class Predicate>
std::pair<float, bool> GetFilteredWeightImpl(const Predicate& predicate,
                                             float reset_value,
                                             const RegionFlowFeature& feature) {
  if (feature.irls_weight() == 0.0f) {
    return std::make_pair(0.0f, false);  // Zero is false by default.
  } else if (!predicate(feature)) {
    return std::make_pair(reset_value, false);
  } else {
    return std::make_pair(feature.irls_weight(), true);
  }
}

// If predicate evaluates to false, corresponding irls weight is set to zero.
// Returns number of features with non-zero irls weight.
// Interface for predicate: bool operator()(const RegionFlowFeature&) const
// Example: Predicate compares registration error of a feature under some linear
// model and returns true if error is below some threshold. Consequently all
// features having registration error above the threshold are set to weight zero
// effectively ignoring those features during estimation.
template <class Predicate>
int FilterRegionFlowFeatureList(const Predicate& predicate, float reset_value,
                                RegionFlowFeatureList* flow_feature_list) {
  ABSL_CHECK(flow_feature_list != nullptr);
  int num_passing_features = 0;
  for (auto& feature : *flow_feature_list->mutable_feature()) {
    std::pair<float, bool> filter_result =
        GetFilteredWeightImpl(predicate, reset_value, feature);
    feature.set_irls_weight(filter_result.first);
    if (filter_result.second) {
      ++num_passing_features;
    }
  }

  return num_passing_features;
}

// Same function as above, but instead of setting the corresponding weights,
// returns resulting weights in a float vector.
template <class Predicate>
int FilterRegionFlowFeatureWeights(const Predicate& predicate,
                                   float reset_value,
                                   const RegionFlowFeatureList& feature_list,
                                   std::vector<float>* result_weights) {
  ABSL_CHECK(result_weights != nullptr);
  result_weights->clear();

  int num_passing_features = 0;
  for (auto feature : feature_list.feature()) {
    std::pair<float, bool> filter_result =
        GetFilteredWeightImpl(predicate, reset_value, feature);
    result_weights->push_back(filter_result.first);
    if (filter_result.second) {
      ++num_passing_features;
    }
  }

  return num_passing_features;
}

// Select features from the passed list for which the predicate is true.
// Returned view contains pointers to mutable features.
template <class Predicate>
void SelectFeaturesFromList(const Predicate& predicate,
                            RegionFlowFeatureList* feature_list,
                            RegionFlowFeatureView* feature_view) {
  ABSL_CHECK(feature_list != nullptr);
  ABSL_CHECK(feature_view != nullptr);
  for (auto& feature : *feature_list->mutable_feature()) {
    if (predicate(feature)) {
      feature_view->push_back(&feature);
    }
  }
}

inline void SelectAllFeaturesFromList(RegionFlowFeatureList* feature_list,
                                      RegionFlowFeatureView* feature_view) {
  ABSL_CHECK(feature_list != nullptr);
  ABSL_CHECK(feature_view != nullptr);
  for (auto& feature : *feature_list->mutable_feature()) {
    feature_view->push_back(&feature);
  }
}

// Sorts region flow feature views, w.r.t. predicate. Predicate must define:
// bool operator()(const RegionFlowFeature* lhs,
//                 const RegionFlowFeature* rhs) const;
template <class Predicate>
void SortRegionFlowFeatureView(const Predicate& predicate,
                               RegionFlowFeatureView* feature_view) {
  ABSL_CHECK(feature_view != nullptr);
  std::sort(feature_view->begin(), feature_view->end(), predicate);
}

// Clamps IRLS weight of each RegionFlowFeature to lie within [lower, upper].
void ClampRegionFlowFeatureIRLSWeights(
    float lower, float upper, RegionFlowFeatureView* flow_feature_list);

// Makes a copy of src to dest without copying any features, i.e. dest will
// have the same values as src for all field except the actual features.
// Implemented by temporarily swapping features in and out, therefore source has
// to be mutable but will not be modified.
void CopyToEmptyFeatureList(RegionFlowFeatureList* src,
                            RegionFlowFeatureList* dst);

// Intersects passed RegionFlowFeatureLists based on track_id, returning new
// RegionFlowFeatureList indicating new motion from features location in list
// 'from' to feature's location in list 'to' (specified by to_location
// function, e.g. pass FeatureLocation or FeatureMatchLocation here).
// Requires RegionFlowFeatureList's computed with long_tracks.
// Output result is initialized to contain all fields from input from (same
// holds for intersected features, minus their match location).
// For performance reasons, from is passed at mutable pointer to use above
// CopyToEmptyFeatureList, but is not modified on output.
// Optionally outputs source index for each feature in result into feature
// array of from.
void IntersectRegionFlowFeatureList(
    const RegionFlowFeatureList& to,
    std::function<Vector2_f(const RegionFlowFeature&)> to_location_eval,
    RegionFlowFeatureList* from, RegionFlowFeatureList* result,
    std::vector<int>* source_indices = nullptr);

// Streaming representation for long feature tracks. Ingests
// RegionFlowFeatureList for passed frames and maps them to their corresponding
// track id.
// Usage example:
// LongFeatureStream stream;
// for (int f = 0; f < frames; ++f) {
//   RegionFlowFeatureList feature_list = ...   // from input.
//   stream.AddFeatures(feature_list, true, true);
//
//   // Traverse tracks starting at the current frame f (going backwards in
//   // time).
//   for (const auto& track : stream) {
//     track.first    // Holds id.
//     track.second   // Holds vector<RegionFlowFeature>, most recent one are at
//                    // the end.
//     Convert track to a list of points.
//     vector<Vector2_f> poly_line;
//     stream.FlattenTrack(track.second, &poly_line, nullptr, nullptr);
//
//     // Returned track where poly_line[0] is the beginning (oldest) and
//     // poly_line[poly_line.size() - 1] the end of the track (most recent
//     // point).
//
//     for (const Vector2_f point : poly_line) {
//       // ... do something ...
//     }
//   }
class LongFeatureStream {
 private:
  // Buffers features according to their track id. Most recent region flow
  // features are added last.
  typedef std::unordered_map<int, std::vector<RegionFlowFeature>> TrackBuffer;

 public:
  // Default constructor for LongFeatureStream. The default long feature stream
  // is backward.
  LongFeatureStream() = default;

  // Constructor for LongFeatureStream. The param forward indicates if the long
  // feature stream is forward or backward.
  explicit LongFeatureStream(bool forward) : forward_(forward) {}

  // Adds new features for the current frame. Region flow must be computed
  // w.r.t to previous or next frame (i.e. inter-frame distance = 1, CHECKED).
  // If check_connectivity is specified, CHECKS if a feature's match location
  // equals it last known location in the buffer.
  // Optionally removes features from the buffer that are not present
  // in the current list.
  void AddFeatures(const RegionFlowFeatureList& feature_list,
                   bool check_connectivity, bool purge_non_present_features);

  // Traversal example:
  // LongFeatureStream stream;
  // for (auto track : stream) {
  //   track.first    // Holds id.
  //   track.second   // Holds vector<RegionFlowFeature>.
  //                  // Note: These are always backward flow features
  //                  //       even if you added forward ones. Ordered in time,
  //                  //       oldest features come first.
  //   vector<Vector2_f> poly_line;
  //   stream.FlattenTrack(track.second, &poly_line, nullptr, nullptr);
  // }
  typename TrackBuffer::const_iterator begin() const { return tracks_.begin(); }
  typename TrackBuffer::const_iterator end() const { return tracks_.end(); }

  // Extract track as poly-line (vector of positions).
  // Specifically, tracks[0] is the beginning (oldest) and
  // tracks[tracks.size() - 1] the end of the track (most recent point).
  // Optionally, returns irls weight for each point pair along the track,
  // i.e weight at position N, specifies weight of track between points
  // N and N + 1. For convenience, weight at last position is replicated, i.e.
  // length of tracks and irls_weight is identical.
  // Optionally, returns the flow vector associated with each point on the
  // track with a direction as requested by the flow direction of the
  // constructor. Note: For N points, N - 1 flow vectors are returned.
  void FlattenTrack(const std::vector<RegionFlowFeature>& features,
                    std::vector<Vector2_f>* tracks,
                    std::vector<float>* irls_weight,      // optional.
                    std::vector<Vector2_f>* flow) const;  // optional.

  // Random access. Returns nullptr if not found.
  const std::vector<RegionFlowFeature>* TrackById(int id) const;

  // Convenience function calling TrackById and FlattenTrack. Returns empty
  // vector if track id is not present.
  std::vector<Vector2_f> FlattenedTrackById(int id) const;

 private:
  // Long Feature tracks indexed by id.
  TrackBuffer tracks_;

  // Stores old ids that have been removed. Used during check_connectivity.
  std::unordered_set<int> old_ids_;

  // A flag indicating if the long feature stream is forward or backward.
  bool forward_ = false;
};

// Helper class for testing which features are present, computing overall track
// length and other statistics.
// Usage:
//   LongFeatureInfo lfi;
//   std::vector<RegionFlowFeatureList> feature_lists = FROM_OUTSIDE
//   for (const auto& feature_list : feature_lists) {
//     lfi.AddFeatures(feature_list);
//     // Get track length for each feature, so far.
//     std::vector<int> track_length;
//     lfi.TrackLenghts(feature_list, &track_length);
//  }
class LongFeatureInfo {
 public:
  // Adds features to current info state.
  void AddFeatures(const RegionFlowFeatureList& feature_list);

  // Adds a single feature. If used instead of above function, requires
  // IncrementFrame to be called manually.
  void AddFeature(const RegionFlowFeature& feature);

  // Returns track length for each passed feature.
  // Note: If feature is not yet present, zero is returned as length.
  void TrackLengths(const RegionFlowFeatureList& feature_list,
                    std::vector<int>* track_lengths) const;

  // Same as above for an individual feature.
  int TrackLength(const RegionFlowFeature& feature) const;

  // Returns starting frame for a feature.
  int TrackStart(const RegionFlowFeature& feature) const;

  int NumFrames() const { return num_frames_; }

  void Reset();

  // Returns track length at passed percentile across all tracks added so far.
  int GlobalTrackLength(float percentile) const;

  void IncrementFrame() { ++num_frames_; }

 private:
  struct TrackInfo {
    int length = 0;
    int start = 0;
  };

  // Maps track id to above info struct.
  std::unordered_map<int, TrackInfo> track_info_;

  int num_frames_ = 0;
};

// Scales a salient point in x and y by specified scales. For example, use to
// map a salient point to a specific frame width and height.
void ScaleSalientPoint(float scale_x, float scale_y,
                       SalientPoint* salient_point);

// Scales salient points in saliency by factor scale. If normalize_to_scale
// is true, the weights of salient points per frame sum up to scale.
void ScaleSalientPointFrame(float scale, bool normalize_to_scale,
                            SalientPointFrame* saliency);

// Convenience function for SaliencyPointList's invoking above function on each
// frame.
void ScaleSaliencyList(float scale, bool normalize_to_scale,
                       SaliencyPointList* saliency_list);

// Resets the normalized bounds of salient points in saliency list to
// specified bounds.
void ResetSaliencyBounds(float left, float bottom, float right, float top,
                         SaliencyPointList* saliency_list);

// Returns major and minor axis from covariance matrix (scaled by one sigma) in
// each direction) and angle of major axis (in radians, counter clockwise).
// Returns false if degenerate (ellipses with less than 2 pixels in diameter
// in either direction).
// Assuming 2x2 Covariance matrix of the form [a  bc
//                                             bc  d]
// for variances specified in pixels^2.
bool EllipseFromCovariance(float a, float bc, float d,
                           Vector2_f* axis_magnitude, float* angle);

// Calculate the bounding box from an axis aligned ellipse defined by the major
// and minor axis. Returns 4 corners of a minimal bounding box.
void BoundingBoxFromEllipse(const Vector2_f& center, float norm_major_axis,
                            float norm_minor_axis, float angle,
                            std::vector<Vector2_f>* bounding_box);

// Helper function used by BuildFeature grid to determine the sample taps
// for a domain of size dim_x x dim_y with a specified tap_radius.
void GridTaps(int dim_x, int dim_y, int tap_radius,
              std::vector<std::vector<int>>* taps);

// Generic function to bin features (of generic type Feature, specified for a
// set of frames) over the domain frame_width x frame_height into equally
// sized square bins of size grid_resolution x grid_resolution.
// Function outputs for each bin the indicies of neighboring bins according
// 3x3 or 5x5 neighborhood centered at that bin. Also output number of bins
// created along each dimension as 2 dim vector (x, y).
// FeatureEvaluator should implement:
// Vector2_f operator()(const Feature& f) const {
//   // Returns spatial location for feature f;
// }
template <class Feature>
using FeatureFrame = std::vector<Feature*>;

template <class Feature>
using FeatureGrid = std::vector<FeatureFrame<Feature>>;

template <class Feature, class FeatureEvaluator>
void BuildFeatureGrid(
    float frame_width, float frame_height, float grid_resolution,
    const std::vector<FeatureFrame<Feature>>& feature_views,
    const FeatureEvaluator& evaluator,
    std::vector<std::vector<int>>* feature_taps_3,  // Optional.
    std::vector<std::vector<int>>* feature_taps_5,  // Optional.
    Vector2_i* num_grid_bins,                       // Optional.
    std::vector<FeatureGrid<Feature>>* feature_grids) {
  ABSL_CHECK(feature_grids);
  ABSL_CHECK_GT(grid_resolution, 0.0f);

  const int num_frames = feature_views.size();
  const int grid_dim_x = std::ceil(frame_width / grid_resolution);
  const int grid_dim_y = std::ceil(frame_height / grid_resolution);
  const int grid_size = grid_dim_x * grid_dim_y;
  const float grid_scale = 1.0f / grid_resolution;

  // Pre-compute neighbor grids.
  feature_grids->clear();
  feature_grids->resize(num_frames);
  for (int f = 0; f < num_frames; ++f) {
    // Populate.
    auto& curr_grid = (*feature_grids)[f];
    curr_grid.resize(grid_size);
    const FeatureFrame<Feature>& curr_view = feature_views[f];
    for (int i = 0, size = curr_view.size(); i < size; ++i) {
      Feature* feature = curr_view[i];
      Vector2_f feature_loc = evaluator(*feature);
      const int x = feature_loc.x() * grid_scale;
      const int y = feature_loc.y() * grid_scale;
      ABSL_DCHECK_LT(y, grid_dim_y);
      ABSL_DCHECK_LT(x, grid_dim_x);
      const int grid_loc = y * grid_dim_x + x;
      curr_grid[grid_loc].push_back(feature);
    }
  }

  if (feature_taps_3 != NULL) {
    GridTaps(grid_dim_x, grid_dim_y, 1, feature_taps_3);
  }
  if (feature_taps_5 != NULL) {
    GridTaps(grid_dim_x, grid_dim_y, 2, feature_taps_5);
  }

  if (num_grid_bins) {
    *num_grid_bins = Vector2_i(grid_dim_x, grid_dim_y);
  }
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_REGION_FLOW_H_
