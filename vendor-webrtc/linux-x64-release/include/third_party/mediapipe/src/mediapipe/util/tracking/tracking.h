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

#ifndef MEDIAPIPE_UTIL_TRACKING_TRACKING_H_
#define MEDIAPIPE_UTIL_TRACKING_TRACKING_H_

// Performs tracking via rectangular regions (MotionBoxes) from pre-initialized
// positions, using metadata from tracked features (TrackingData converted to
// MotionVectorFrames), forward and backward in time.

#include <deque>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/flow_packager.pb.h"
#include "mediapipe/util/tracking/motion_models.h"
#include "mediapipe/util/tracking/motion_models.pb.h"
#include "mediapipe/util/tracking/tracking.pb.h"

namespace mediapipe {

// Useful helper functions.
//
// Clamps values to be within interval [left, right].
inline float Clamp(float value, float left, float right) {
  return value < left ? left : (value > right ? right : value);
}

// Standard linear interpolation function.
template <class T>
T Lerp(T a, T b, float alpha) {
  return static_cast<T>(a * (1.0f - alpha) + b * alpha);
}

// Approximates sigmoid function with a linear ramp, mapping
// x <= lhs to 0, x >= rhs to 1 (for lhs < rhs) linear in between interval
// [lhs, rhs]. If lhs > rhs, roles are reversed.
inline float LinearRamp(float value, float lhs, float rhs) {
  return Clamp((value - lhs) / (rhs - lhs), 0, 1);
}

inline Vector2_f MotionBoxPosition(const MotionBoxState& state) {
  return Vector2_f(state.pos_x(), state.pos_y());
}

inline void SetMotionBoxPosition(const Vector2_f& pos, MotionBoxState* state) {
  state->set_pos_x(pos.x());
  state->set_pos_y(pos.y());
}

// TODO: this needs to be changed for quad
inline Vector2_f MotionBoxSize(const MotionBoxState& state) {
  return Vector2_f(state.width(), state.height());
}

inline void SetMotionBoxSize(const Vector2_f& size, MotionBoxState* state) {
  state->set_width(size.x());
  state->set_height(size.y());
}

inline Vector2_f MotionBoxCenter(const MotionBoxState& state) {
  return MotionBoxPosition(state) + 0.5f * MotionBoxSize(state);
}

inline Vector2_f InlierCenter(const MotionBoxState& state) {
  return Vector2_f(state.inlier_center_x(), state.inlier_center_y());
}

inline Vector2_f MotionBoxVelocity(const MotionBoxState& state) {
  return Vector2_f(state.dx(), state.dy());
}

inline void SetMotionBoxVelocity(const Vector2_f& velo, MotionBoxState* state) {
  state->set_dx(velo.x());
  state->set_dy(velo.y());
}

// Derive normalization factor from image aspect ratio so that the scale for the
// longer edge is 1. scale will be reversed if `invert` is true.
void ScaleFromAspect(float aspect, bool invert, float* scale_x, float* scale_y);

// Returns 4 corners of the MotionBox as top_left, bottom_left, bottom_right
// and top_right.  Applies 2D scaling prior to rotation, which is necessary to
// preserve orthogonality of the rotation if the scaling is not isotropic.
std::array<Vector2_f, 4> MotionBoxCorners(
    const MotionBoxState& state,
    const Vector2_f& scaling = Vector2_f(1.0f, 1.0f));

// Computes corresponding line equations for MotionBoxCorners.
// Output line equations on 4 sides.
// Returns true if box is normal, false if we encounter abnormal box which
// leads to numerical problems.
// Applies 2D scaling prior to rotation, which is necessary to
// preserve orthogonality of the rotation if the scaling is not isotropic.
bool MotionBoxLines(const MotionBoxState& state, const Vector2_f& scaling,
                    std::array<Vector3_f, 4>* box_lines);

// Returns top-left and bottom right corner of the bounding box
// of the MotionBoxState.
void MotionBoxBoundingBox(const MotionBoxState& state, Vector2_f* top_left,
                          Vector2_f* bottom_right);

// Adds all inliers from state to the inlier map (as id, score) tuple.
// If id already exist, score is updated to be the maximum of current and
// existing score.
inline void MotionBoxInliers(const MotionBoxState& state,
                             std::unordered_map<int, int>* inliers) {
  ABSL_CHECK(inliers);
  const int num_inliers = state.inlier_ids_size();
  ABSL_DCHECK_EQ(num_inliers, state.inlier_length_size());

  for (int k = 0; k < num_inliers; ++k) {
    (*inliers)[state.inlier_ids(k)] =
        std::max<int>((*inliers)[state.inlier_ids(k)], state.inlier_length(k));
  }
}

// Adds all outliers from state to the outlier map.
inline void MotionBoxOutliers(const MotionBoxState& state,
                              std::unordered_set<int>* outliers) {
  for (int id : state.outlier_ids()) {
    outliers->insert(id);
  }
}

// Returns inlier locations from state (normalized in [0, 1] domain).
void MotionBoxInlierLocations(const MotionBoxState& state,
                              std::vector<Vector2_f>* inlier_pos);
// Same for outlier positions.
void MotionBoxOutlierLocations(const MotionBoxState& state,
                               std::vector<Vector2_f>* outlier_pos);

// Get corners of rotated rectangle. Note that the quad component in
// MotionBoxState is not used in this function. Only the rotated rectangle
// is used.
// Inputs:
//   -- state: the MotionBoxState where we extract the rotated rectangle
//   -- scaling: additional scaling we apply on x and y axis
// Output:
//   corners in counter-clockwise order
std::array<Vector2_f, 4> GetCornersOfRotatedRect(const MotionBoxState& state,
                                                 const Vector2_f& scaling);

// Use the position, width, and height in MotionBoxState to initialize
// the quad. Only use it when you want to get homography for tracking.
void InitializeQuadInMotionBoxState(MotionBoxState* state);

// Initializes inliers and outliers related fields in MotionBoxState from
// TrackingData. The box or quad position will be read from `state` so they need
// to be set beforehand.
void InitializeInliersOutliersInMotionBoxState(const TrackingData& tracking,
                                               MotionBoxState* state);

// Initializes pnp_homography field in MotionBoxState using perspective
// transform between a physical rectangle with specified aspect ratio and a
// screen quad.
void InitializePnpHomographyInMotionBoxState(
    const TrackingData& tracking, const TrackStepOptions& track_step_options,
    MotionBoxState* state);

// Represents the motion of a feature at pos between frames, differentiating
// object from background motion (supplied via a MotionVectorFrame).
struct MotionVector {
  MotionVector() : pos(0, 0), background(0, 0), object(0, 0) {}

  MotionVector(const Vector2_f& pos_, const Vector2_f& background_,
               const Vector2_f& object_)
      : pos(pos_), background(background_), object(object_) {}

  Vector2_f Location() const { return pos; }
  Vector2_f MatchLocation() const { return pos + background + object; }
  Vector2_f Motion() const { return background + object; }

  // Position of the feature in normalized domain [0, 1].
  Vector2_f pos;
  // Motion due to background (i.e. camera motion).
  Vector2_f background;
  // Motion due to foreground (i.e. object motion in addition to background).
  // If feature belong to background, object motion is nearly zero.
  Vector2_f object;

  int track_id = -1;

  // Returns the MotionVector stored in the internal state at index.
  static MotionVector FromInternalState(const MotionBoxInternalState& internal,
                                        int index);
};

inline constexpr float kTrackingDefaultFps = 30.0;

// Holds motion vectors and background model for each frame.
// Note: Specified in the aspect preserving domain under uniform scaling,
// longest dimension normalized to 1, i.e. if aspect_ratio >= 1, width is
// normalized to 1 otherwise height is normalized to 1.
struct MotionVectorFrame {
  std::vector<MotionVector> motion_vectors;

  Homography background_model;
  bool valid_background_model = true;
  bool is_duplicated = false;      // Set if frame is duplicated w.r.t.
                                   // previous one.
  bool is_chunk_boundary = false;  // Set if this is the first frame in a chunk.

  float duration_ms = 1000.0f / kTrackingDefaultFps;

  // Aspect ratio (w/h) of the original frame.
  float aspect_ratio = 1.0f;

  // Stores the tracked ids that have been discarded actively. This information
  // will be used to avoid misjudgement on tracking continuity.
  absl::flat_hash_set<int>* actively_discarded_tracked_ids = nullptr;
};

// Transforms TrackingData to MotionVectorFrame, ready to be used by tracking
// algorithm (so the MotionVectorFrame data is denormalized).
void MotionVectorFrameFromTrackingData(const TrackingData& tracking_data,
                                       MotionVectorFrame* motion_vector_frame);

// Transform TrackingData to feature positions and descriptors, ready to be used
// by detection (re-acquisition) algorithm (so the "features" is denomalized).
// Descriptors with all 0s will be discarded.
void FeatureAndDescriptorFromTrackingData(
    const TrackingData& tracking_data, std::vector<Vector2_f>* features,
    std::vector<std::string>* descriptors);

// Inverts MotionVectorFrame (by default defined as motion from current to
// previous frame) to hold motion from previous to current frame.
void InvertMotionVectorFrame(const MotionVectorFrame& input,
                             MotionVectorFrame* output);

// Returns duration in ms for this chunk item.
float TrackingDataDurationMs(const TrackingDataChunk::Item& item);

// Returns feature indices that are within the given box. If the box size isn't
// big enough to cover sufficient features (i.e., min_num_features), this will
// iteratively enlarge the box size (up to max_enlarge_size) to include more
// features. The argument box_scaling is used in MotionBoxLines() to get
// properly scaled box corners. Note: box_scaling and max_enlarge_size need to
// be in normalized image space.
// TODO: Add unit test.
void GetFeatureIndicesWithinBox(const std::vector<Vector2_f>& features,
                                const MotionBoxState& box_state,
                                const Vector2_f& box_scaling,
                                float max_enlarge_size, int min_num_features,
                                std::vector<int>* inlier_indices);

// Represents a moving box over time. Initial position is supplied via
// ResetAtFrame, and subsequent positions for previous and next frames are
// determined via tracking by TrackStep method.
// Example usage:
// // Assuming metadata is available: vector<MotionVectorFrame> mvf;
// MotionBoxState box_state;
// // Set to center 20%.
// box_state.set_pos_x(0.4);
// box_state.set_pos_y(0.4);
// box_state.set_width(0.2);
// box_state.set_height(0.2);
//
// // Initialize first position at frame 5.
// MotionBox motion_box(TrackStepOptions());
// motion_box.ResetAtFrame(4, box_state);
// // Track 4 frames backward and forward in time.
// for (int i = 0; i < 4; ++i) {
//   // Tracking steps need to be called contiguously, as otherwise no
//   // prior location for the track is present and TrackStep will fail.
//   // Backward.
//   motion_box.TrackStep(4 - i, mvf[4 -i], false, nullptr);
//   // Get position -> consume for display, etc.
//   motion_box.StateAtFrame(4 - i);
//
//   // Forward.
//   motion_box.TrackStep(4 + i, mvf[4 -i], true, nullptr);
//   // Get position -> consume.
//   motion_box.StateAtFrame(4 + i);
// }
class MotionBox {
 public:
  explicit MotionBox(const TrackStepOptions& track_step_options)
      : options_(track_step_options) {}

  MotionBox() = default;

  // Sets and overwrites MotionBoxState at specified frame. Use to supply
  // initial position.
  void ResetAtFrame(int frame, const MotionBoxState& state);

  // Tracks MotionBox from state at from_frame either forward or backward in
  // time, based on the passed MotionVectorFrame. (MotionVectorFrame has to
  // correspond to requested tracking direction, this is not checked against).
  // Returns true if tracking was successful.
  // Note: It is assumed that from_frame already has a valid location, either
  // via ResetAtFrame or previous successful execution of TrackStep. That is
  // TrackStep needs to be called contiguously from a initialized position
  // via ResetFrame. Otherwise no prior location for the track is present (at
  // from_frame) and TrackStep will fail (return false).
  bool TrackStep(int from_frame, const MotionVectorFrame& motion_vectors,
                 bool forward);

  MotionBoxState StateAtFrame(int frame) const {
    if (frame < queue_start_ ||
        frame >= queue_start_ + static_cast<int>(states_.size())) {
      ABSL_LOG(ERROR) << "Requesting state at unknown frame " << frame
                      << ". Returning UNTRACKED.";
      MotionBoxState invalid;
      invalid.set_track_status(MotionBoxState::BOX_UNTRACKED);
      return invalid;
    } else {
      MotionBoxState result = states_[frame - queue_start_];
      if (!options_.return_internal_state()) {
        result.clear_internal();
      }
      return result;
    }
  }

  MotionBoxState* MutableStateAtFrame(int frame) {
    if (frame < queue_start_ || frame >= queue_start_ + states_.size()) {
      return NULL;
    } else {
      return &states_[frame - queue_start_];
    }
  }

  bool TrackableFromFrame(int frame) const {
    return StateAtFrame(frame).track_status() >= MotionBoxState::BOX_TRACKED;
  }

  void set_start_track(int frame) { start_track_ = frame; }
  int start_track() const { return start_track_; }
  void set_end_track(int frame) { end_track_ = frame; }
  int end_track() const { return end_track_; }

  void TrimFront(const int cache_size) {
    int trim_count = states_.size() - cache_size;
    if (trim_count > 0) {
      queue_start_ += trim_count;
      while (trim_count-- > 0) {
        states_.pop_front();
      }
    }
  }

  void TrimBack(const int cache_size) {
    int trim_count = states_.size() - cache_size;
    if (trim_count > 0) {
      while (trim_count-- > 0) {
        states_.pop_back();
      }
    }
  }

  // If this variable is set to true, then TrackStep would print warning
  // messages when tracking is failed.
  // Default value is true and is set in tracking.cc.
  static bool print_motion_box_warnings_;

 private:
  // Determines next position from curr_pos based on tracking data in
  // motion_vectors. Also receives history of the last N positions.
  void TrackStepImplDeNormalized(
      int frome_frame, const MotionBoxState& curr_pos,
      const MotionVectorFrame& motion_vectors,
      const std::vector<const MotionBoxState*>& history,
      MotionBoxState* next_pos) const;

  // Pre-normalization wrapper for above function. De-normalizes domain
  // to aspect preserving domain and velocity to current frame period.
  void TrackStepImpl(int from_frame, const MotionBoxState& curr_pos,
                     const MotionVectorFrame& motion_frame,
                     const std::vector<const MotionBoxState*>& history,
                     MotionBoxState* next_pos) const;

  // Implementation functions for above TrackStepImpl.
  // Returns bounding box for start position and the expansion magnitude
  // (normalized) that was applied.
  void GetStartPosition(const MotionBoxState& curr_pos, float aspect_ratio,
                        float* expand_mag, Vector2_f* top_left,
                        Vector2_f* bottom_right) const;

  // Outputs spatial sigma in x and y for spatial weighting.
  // Pass current box_state and inverse box domain size.
  void GetSpatialGaussWeights(const MotionBoxState& box_state,
                              const Vector2_f& inv_box_domain,
                              float* spatial_gauss_x,
                              float* spatial_gauss_y) const;

  // Outputs subset of motion_vectors that are within the specified domain
  // (top_left to bottom_right). Only searches over the range specified via
  // start and end idx.
  // Each vector is weighted based on gaussian proximity, similar motion,
  // track continuity, etc. which forms the prior weight of each feature.
  // Features are binned into a grid of fixed dimension for density analysis.
  // Also output number of vectors with good prior weights (> 0.1), and number
  // of continued inliers.
  // Returns true on success, false on failure. When it returns false, the
  // output values are not reliable.
  bool GetVectorsAndWeights(
      const std::vector<MotionVector>& motion_vectors, int start_idx,
      int end_idx, const Vector2_f& top_left, const Vector2_f& bottom_right,
      const MotionBoxState& box_state, bool valid_background_model,
      bool is_chunk_boundary,
      float temporal_scale,  // Scale for velocity from standard frame period.
      float expand_mag, const std::vector<const MotionBoxState*>& history,
      std::vector<const MotionVector*>* vectors,
      std::vector<float>* prior_weights, int* number_of_good_prior,
      int* number_of_cont_inliers) const;

  // Initializes weights by performing multiple ransac rounds from vectors.
  // Error is scaled by irls scale along parallel and orthogonal direction.
  void TranslationIrlsInitialization(
      const std::vector<const MotionVector*>& vectors,
      const Vector2_f& irls_scale, std::vector<float>* weights) const;

  // Wrapper function, estimating object motion w.r.t. various degrees
  // of freedom.
  void EstimateObjectMotion(
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& prior_weights, int num_continued_inliers,
      const Vector2_f& irls_scale, std::vector<float>* weights,
      Vector2_f* object_translation, LinearSimilarityModel* object_similarity,
      Homography* object_homography) const;

  // Perform IRLS estimation of the passed motion_vector's object motion.
  // Each vector is weighted by original_weight / estimation_error, where
  // estimation_error is refined in each estimation round.
  // Outputs final translation and resulting irls weights (1.0 /
  // estimation_error, i.e. with prior bias).
  void EstimateTranslation(
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& orig_weights, const Vector2_f& irls_scale,
      std::vector<float>* weights, Vector2_f* translation) const;

  // Same as above for similarity. Returns false on failure (numerical
  // instability is most common case here).
  bool EstimateSimilarity(
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& orig_weights, const Vector2_f& irls_scale,
      std::vector<float>* weights, LinearSimilarityModel* lin_sim) const;

  // Same as above for homograph.
  bool EstimateHomography(
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& prior_weights, const Vector2_f& irls_scale,
      std::vector<float>* weights, Homography* object_homography) const;

  // Perform 6DoF perspective transform based homography estimation using
  // motion_vector's object + background motion.
  // weights are used to determine whether a vector is inlier or outliers.
  // The perspective solver will exclude those vectors with weights smaller than
  // kMaxOutlierWeight (0.1).
  bool EstimatePnpHomography(
      const MotionBoxState& curr_pos,
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& weights, float domain_x, float domain_y,
      Homography* pnp_homography) const;

  // Apply pre-computed perspective transform based homography to the next pos.
  void ApplyObjectMotionPerspectively(const MotionBoxState& curr_pos,
                                      const Homography& pnp_homography,
                                      float domain_x, float domain_y,
                                      MotionBoxState* next_pos) const;

  // Scores every vector after translation estimation into inliers and outliers
  // (based on post_estimation_weights, inlierness is a measure in [0, 1]),
  // and records result in next_pos as well as returning inlierness per vector
  // in inlier_weights.
  // Also computes the following statistics:
  // - inlier_density: Local density for each inlier, i.e. measure of how many
  //                   other inliers are close to that point. In [0, 1].
  // - continued_inliers: Number of inliers that continue to be present already
  //                      in curr_pos (same track id)
  // - swapped_inliers: Number of inliers that are outliers in curr_pos (same
  //                    track id).
  // - motion_inliers: Number of inliers of similar motion as previous state.
  //                   This measure is complementary to above continued inliers
  //                   in case object is moving significantly, in which case
  //                   tracks tend to be short lived.
  // - kinetic_average: Average object norm of all inliers weighted by
  //                    pre_estimation_weights.
  void ScoreAndRecordInliers(const MotionBoxState& curr_pos,
                             const std::vector<const MotionVector*>& vectors,
                             const std::vector<Vector2_f>& grid_positions,
                             const std::vector<float>& pre_estimation_weights,
                             const std::vector<float>& post_estimation_weights,
                             float background_discrimination,
                             MotionBoxState* next_pos,
                             std::vector<float>* inlier_weights,
                             std::vector<float>* inlier_density,
                             int* continued_inliers, int* swapped_inliers,
                             float* motion_inliers,
                             float* kinetic_average) const;

  // Computes motion disparity (in [0, 1]), that is how well does the current
  // object motion agree with the previous object motion.
  // 0 indicates perfect match, 1 indicates signicant difference.
  float ComputeMotionDisparity(const MotionBoxState& curr_pos,
                               const Vector2_f& irls_scale,
                               float continued_inliers, int num_inliers,
                               const Vector2_f& object_motion) const;

  // Computes inlier center and extent (vector positions weighted by
  // weights and density).
  // Sets center inlier center, if inlier_weight is above min_inlier_sum,
  // else to the Motion box center.
  void ComputeInlierCenterAndExtent(
      const std::vector<const MotionVector*>& motion_vectors,
      const std::vector<float>& weights, const std::vector<float>& density,
      const MotionBoxState& state, float* min_inlier_sum, Vector2_f* center,
      Vector2_f* extent) const;

  float ScaleEstimate(const std::vector<const MotionVector*>& motion_vectors,
                      const std::vector<float>& weights, float min_sum) const;

  // Applies spring force from box_state's position to center_of_interest, if
  // difference is above rel_threshold. Correcting force equals difference
  // above threshold times the spring_force coefficient.
  void ApplySpringForce(const Vector2_f& center_of_interest,
                        const float rel_threshold, const float spring_force,
                        MotionBoxState* box_state) const;

  // Compute the tracking confidence and return the value.
  // The confidence is a float value in [0, 1], with 0 being least confident,
  // and 1 being most confident.
  float ComputeTrackingConfidence(const MotionBoxState& motion_box_state) const;

 private:
  class ObjectMotionValidator {
   public:
    static bool IsValidSimilarity(
        const LinearSimilarityModel& linear_similarity_model, float max_scale,
        float max_rotation) {
      SimilarityModel similarity_model =
          LinearSimilarityAdapter::ToSimilarity(linear_similarity_model);

      if (similarity_model.scale() < 1.0f / max_scale ||
          similarity_model.scale() > max_scale ||
          std::abs(similarity_model.rotation()) > max_rotation) {
        return false;
      }
      return true;
    }

    static bool IsValidHomography(const Homography& homography, float max_scale,
                                  float max_rotation) {
      // Filter out abnormal homography. Otherwise the determinant of
      // projected affine matrix will be negative.
      if (!IsInverseStable(homography)) {
        ABSL_LOG(WARNING) << "Homography matrix is not stable.";
        return false;
      }

      LinearSimilarityModel similarity_model =
          LinearSimilarityAdapter::ProjectFrom(homography, 1.0f, 1.0f);
      return IsValidSimilarity(similarity_model, max_scale, max_rotation);
    }

    // Check if it is a convex quad.
    static bool IsValidQuad(const MotionBoxState::Quad& quad) {
      const int kQuadVerticesSize = 8;
      ABSL_CHECK_EQ(quad.vertices_size(), kQuadVerticesSize);
      for (int a = 0; a < kQuadVerticesSize; a += 2) {
        int b = (a + 2) % kQuadVerticesSize;
        int c = (a - 2 + kQuadVerticesSize) % kQuadVerticesSize;
        Vector2_f ab(quad.vertices(b) - quad.vertices(a),
                     quad.vertices(b + 1) - quad.vertices(a + 1));
        Vector2_f ac(quad.vertices(c) - quad.vertices(a),
                     quad.vertices(c + 1) - quad.vertices(a + 1));

        // Since quad's vertices is defined in counter-clockwise manner, we only
        // accept negative cross product.
        if (ab.CrossProd(ac) >= 0) {
          return false;
        }
      }

      return true;
    }

    // Check if all the 4 corners of the quad are out of FOV.
    static bool IsQuadOutOfFov(const MotionBoxState::Quad& quad,
                               const Vector2_f& fov) {
      const int kQuadVerticesSize = 8;
      ABSL_CHECK_EQ(quad.vertices_size(), kQuadVerticesSize);
      bool too_far = true;
      for (int j = 0; j < kQuadVerticesSize; j += 2) {
        if (quad.vertices(j) < fov.x() && quad.vertices(j) > 0.0f &&
            quad.vertices(j + 1) < fov.y() && quad.vertices(j + 1) > 0.0f) {
          too_far = false;
          break;
        }
      }
      return too_far;
    }
  };

  class DistanceWeightsComputer {
   public:
    DistanceWeightsComputer(const MotionBoxState& initial_state,
                            const MotionBoxState& current_state,
                            const TrackStepOptions& options);

    // Compute distance weight based on input motion vector position.
    float ComputeDistanceWeight(const MotionVector& test_vector);

   private:
    Homography ComputeHomographyFromQuad(const MotionBoxState::Quad& src_quad,
                                         const MotionBoxState::Quad& dst_quad);

    float cos_neg_a_;
    float sin_neg_a_;
    float spatial_gauss_x_;
    float spatial_gauss_y_;
    Vector2_f inv_box_domain_;
    Vector2_f box_center_;
    Vector2_f box_center_transformed_;
    bool is_large_rotation_ = false;
    Homography homography_;  // homography from current box to initial box
    TrackStepOptions::TrackingDegrees tracking_degrees_;
  };

  TrackStepOptions options_;
  std::deque<MotionBoxState> states_;
  int queue_start_;

  int start_track_;
  int end_track_;

  MotionBoxState initial_state_;
};

}  // namespace mediapipe.

#endif  // MEDIAPIPE_UTIL_TRACKING_TRACKING_H_
