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
// Fits several linear motion models to the tracked features obtained
// from RegionFlowComputation.
//
// --- Multi-threaded usage (parallel motion estimation over frames) ---
// assume input: vector<RegionFlowFeatureList*> feature_lists
// // Can be obtained after getting RegionFlowFrame from RegionFlowComputation
// // and executing GetRegionFlowFeatureList (region_flow.h)
// MotionEstimation motion_estimation(MotionEstimationOptions(),
//                                    frame_width,
//                                    frame_height);
// vector<CameraMotion> camera_motions;
// motion_estimation.EstimateMotionsParallel(false,    // no IRLS smoothing.
//                                           &feature_lists,
//                                           &camera_motions);
// // RegionFlowFeatureList can be discarded or passed to Cropper.
//
//
// --- DEPRECATED, per-frame usage ---
// assume input: RegionFlowFrame* flow_frame  // from RegionFlowComputation.
//
// // Initialize with standard options.
// MotionEstimation motion_estimation(MotionEstimationOptions(),
//                                    frame_width,
//                                    frame_height);
// CameraMotion estimated_motion;
// motion_estimation.EstimateMotion(flow_frame,
//                                  NULL,               // deprecated param.
//                                  NULL,               // deprecated param.
//                                  &estimation_motion);
//
// // If features are not needed anymore flow_frame can be be discarded now.
//
// // Pass motion models in estimated_motion onto MotionStabilization,
// // if stabilization is desired.

#ifndef MEDIAPIPE_UTIL_TRACKING_MOTION_ESTIMATION_H_
#define MEDIAPIPE_UTIL_TRACKING_MOTION_ESTIMATION_H_

#include <algorithm>
#include <cstdint>
#include <deque>
#include <list>
#include <memory>
#include <unordered_map>
#include <vector>

#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/camera_motion.pb.h"
#include "mediapipe/util/tracking/motion_estimation.pb.h"
#include "mediapipe/util/tracking/motion_models.pb.h"
#include "mediapipe/util/tracking/region_flow.h"

namespace mediapipe {

class Homography;
class LinearSimilarityModel;
class MixtureHomography;
class MixtureRowWeights;
class RegionFlowFeature;
class RegionFlowFeatureList;
class RegionFlowFrame;

class EstimateMotionIRLSInvoker;
class InlierMask;
class IrlsInitializationInvoker;
// Thread local storage for pre-allocated memory.
class MotionEstimationThreadStorage;
class TrackFilterInvoker;

class MotionEstimation {
 public:
  MotionEstimation(const MotionEstimationOptions& options, int frame_width,
                   int frame_height);
  virtual ~MotionEstimation();
  MotionEstimation(const MotionEstimation&) = delete;
  MotionEstimation& operator=(const MotionEstimation&) = delete;

  // Can be used to re-initialize options between EstimateMotion /
  // EstimateMotionsParallel calls.
  void InitializeWithOptions(const MotionEstimationOptions& options);

  // Estimates motion models from RegionFlowFeatureLists based on
  // MotionEstimationOptions, in a multithreaded manner (frame parallel).
  // The computed IRLS weights used on the last iteration of the highest
  // degree of freedom model are *written* to the irls_weight member for each
  // RegionFlowFeature in RegionFlowFeatureList which can be a useful
  // feature for later processing.
  // In addition the returned irls weights can be smoothed spatially
  // and temporally before they are output.
  // Note: The actual vector feature_lists is not modified.
  virtual void EstimateMotionsParallel(
      bool post_irls_weight_smoothing,
      std::vector<RegionFlowFeatureList*>* feature_lists,
      std::vector<CameraMotion>* camera_motions) const;

  // DEPRECATED function, estimating Camera motion from a single
  // RegionFlowFrame.
  virtual void EstimateMotion(const RegionFlowFrame& region_flow_frame,
                              const int*,  // deprecated, must be NULL.
                              const int*,  // deprecated, must be NULL.
                              CameraMotion* camera_motion) const;

  // Public facing API to directly estimate motion models (as opposed to
  // a cascade of increasing degree of freedom motion models with appropiate
  // stability analysis via above EstimateMotionsParallel).
  // Use this if all you need is just the a specific motion
  // model describing/summarizing the motion of the RegionFlowFeatureList.
  // Returns false if model estimation failed (in this case an identity model
  // is set in camera_motion).
  // NOTE: All direct estimation functions assume normalized feature input,
  // i.e. transformed via NormalizeRegionFlowFeatureList.
  //
  // NOTE 2: For easy direct use see Fit* functions below class.
  bool EstimateTranslationModel(RegionFlowFeatureList* feature_list,
                                CameraMotion* camera_motion);

  bool EstimateLinearSimilarityModel(RegionFlowFeatureList* feature_list,
                                     CameraMotion* camera_motion);

  bool EstimateAffineModel(RegionFlowFeatureList* feature_list,
                           CameraMotion* camera_motion);

  bool EstimateHomography(RegionFlowFeatureList* feature_list,
                          CameraMotion* camera_motion);

  bool EstimateMixtureHomography(RegionFlowFeatureList* feature_list,
                                 CameraMotion* camera_motion);

  // Static function which sets motion models (requested in options) to identity
  // models.
  static void ResetMotionModels(const MotionEstimationOptions& options,
                                CameraMotion* camera_motion);

  // The following functions ResetTo* functions reset all models that are
  // present in camera_motion (tested via has_*) to identity or
  // the passed model (which is embedded in higher degree of freedom models
  // if applicable). CameraMotion::Type is set in accordance to function name.
  static void ResetToIdentity(CameraMotion* camera_motion,
                              bool consider_valid = false);  // Set to true
                                                             // for type VALID

  // Resets every specified model to embedded translation model.
  // CameraMotion type is set to UNSTABLE.
  static void ResetToTranslation(const TranslationModel& translation,
                                 CameraMotion* camera_motion);

  // Resets every specified model with more or equal DOF than a similarity
  // to the passed model.
  // Camera Motion type is set to UNSTABLE_SIM.
  static void ResetToSimilarity(const LinearSimilarityModel& model,
                                CameraMotion* camera_motion);

  // Resets every specified model with more or equal DOF than a homography
  // to the passed model. If flag_as_unstable_model is set, camera motion type
  // is set to UNSTABLE_HOMOG.
  static void ResetToHomography(const Homography& model,
                                bool flag_as_unstable_model,
                                CameraMotion* camera_motion);

 private:
  // Simple enum indicating with motion model should be estimated, mapped from
  // MotionEstimationOptions.
  enum MotionType {
    MODEL_AVERAGE_MAGNITUDE = 0,
    MODEL_TRANSLATION = 1,
    MODEL_LINEAR_SIMILARITY = 2,
    MODEL_AFFINE = 3,
    MODEL_HOMOGRAPHY = 4,
    MODEL_MIXTURE_HOMOGRAPHY = 5,
    // ... internal enum values used for mixture spectrum (up to 10 mixtures are
    // supported). Do not use directly.

    // Change value if new motions are added.
    MODEL_NUM_VALUES = 16,
  };

  // Determines shot boundaries from estimated motion models and input
  // feature_lists by setting the corresponding flag in CameraMotion.
  // Make sure, this function is called after motion estimation.
  void DetermineShotBoundaries(
      const std::vector<RegionFlowFeatureList*>& feature_lists,
      std::vector<CameraMotion>* camera_motions) const;

  // Implementation function to estimate CameraMotion's from
  // RegionFlowFeatureLists.
  void EstimateMotionsParallelImpl(
      bool irls_weights_preinitialized,
      std::vector<RegionFlowFeatureList*>* feature_lists,
      std::vector<CameraMotion>* camera_motions) const;

  struct SingleTrackClipData;
  struct EstimateModelOptions;

  EstimateModelOptions DefaultModelOptions() const;

  // Implementation function to estimate all motions for a specific type
  // across multiple single tracks. Motions are only estimated for those
  // CameraMotions with type less or equal to max_unstable_type. Flag
  // irls_weights_preinitialized enables some optimizations in case it is
  // set to false as features are not pre-initialized.
  // Optionally pass thread_storage.
  // Returns true if requested type was attempt to be estimated
  // (based on options) , false otherwise.
  bool EstimateMotionModels(
      const MotionType& max_type, const CameraMotion::Type& max_unstable_type,
      const EstimateModelOptions& options,
      const MotionEstimationThreadStorage* thread_storage,  // optional.
      std::vector<SingleTrackClipData>* clip_datas) const;

  // Multiplies input irls_weights by an upweight multiplier for each feature
  // that is part of a sufficiently large track (contribution of each track
  // length is by track_length_multiplier, mapping each track length
  // to an importance weight in [0, 1]).
  void LongFeatureInitialization(
      const RegionFlowFeatureList& feature_list,
      const LongFeatureInfo& feature_info,
      const std::vector<float>& track_length_importance,
      std::vector<float>* irls_weights) const;

  // Multiplies input irls_weights by normalization factor that downweights
  // features is areas of high density.
  void FeatureDensityNormalization(const RegionFlowFeatureList& feature_list,
                                   std::vector<float>* irls_weights) const;

  // Initializes irls weights (if
  // MotionEstimationOptions::irls_initialization::activated is true),
  // based on a multitude of options (RANSAC based  pre-fitting of motion
  // models, homography initialization, etc.).
  // Processes one frame or all (if frame = -1) within a clip_data,
  void IrlsInitialization(const MotionType& type, int max_unstable_type,
                          int frame,  // Specify -1 for all frames.
                          const EstimateModelOptions& options,
                          SingleTrackClipData* clip_data) const;

  // Estimation functions for models, called via options by EstimateMotion and
  // EstimateMotionsParallel.
  // NOTE: All direct estimation functions assume normalized feature input,
  // i.e. transformed via
  // TransformRegionFlowFeatureList(normalization_transform, feature_list);
  // where normalization_transform =
  //     LinearSimilarityAdapter::NormalizationTransform(frame_width,
  //                                                     frame_height);
  //
  // Direct estimation functions perform estimation via iterated reweighted
  // least squares (IRLS). In this case specify number of iterations (10 is a
  // good default), and optionally the PriorFeatureWeights for each iteration.
  // The alphas specify, how much weight should be given to the
  // prior weight that the feature has before optimization. An alpha of zero
  // indicates, no prior weighting, whereas as an alpha of one corresponds to
  // full prior weighting. The actual prior is stored in priors.
  // Each iteration is reweighted by numerator / error, where error is the L2
  // fitting error after estimation and
  // numerator = (1.0 - alpha) * 1.0 + alpha * prior
  struct PriorFeatureWeights {
    explicit PriorFeatureWeights(int num_iterations)
        : alphas(num_iterations, 0.0f) {}
    PriorFeatureWeights(int num_iterations, int num_features)
        : alphas(num_iterations, 0.0f), priors(num_features, 1.0f) {}

    // Tests for correct dimensions of PriorFeatureWeights.
    bool HasCorrectDimension(int num_iterations, int num_features) const {
      return alphas.size() == num_iterations && priors.size() == num_features;
    }

    // Returns true if at least one alpha is non-zero.
    bool HasNonZeroAlpha() const {
      return !alphas.empty() &&
             *std::max_element(alphas.begin(), alphas.end()) > 0;
    }

    // Returns true, if a prior was specified.
    bool HasPrior() const { return !priors.empty(); }

    std::vector<float> alphas;  // Alpha for each IRLS round.
    std::vector<float> priors;  // Prior weight for each feature.

    // If set, above alpha are not adjusted with iterations, but always set to
    // 1.0, given full weight to the prior.
    bool use_full_prior = false;
  };

  // In addition, each estimation function can compute its corresponding
  // stability features and store it in CameraMotion. These features are needed
  // to test via the IsStable* functions further below.

  // Estimates 2 DOF translation model.
  // Note: feature_list is assumed to be normalized/transformed by
  // LinearSimilarity::NormalizationTransform N. Returned irls weights and
  // linear similarity are expressed in original frame, i.e. for estimated model
  // M, M' = N^(-1) M N is returned.
  void EstimateTranslationModelIRLS(
      int irls_rounds, bool compute_stability,
      RegionFlowFeatureList* feature_list,
      const PriorFeatureWeights* prior_weights,  // optional.
      CameraMotion* camera_motion) const;

  // Estimates linear similarity from feature_list using irls_rounds iterative
  // reweighted least squares iterations. For L2 estimation, use irls_round = 1.
  // The irls_weight member of each RegionFlowFeature in feature_list will be
  // set to the inverse residual w.r.t. estimated LinearSimilarityModel.
  // Note: feature_list is assumed to be normalized/transformed by
  // LinearSimilarity::NormalizationTransform N. Returned irls weights and
  // linear similarity are expressed in original frame, i.e. for estimated model
  // M, M' = N^(-1) M N is returned.
  // Returns true if estimation was successful, otherwise returns false and sets
  // the CameraMotion::type to INVALID.
  bool EstimateLinearSimilarityModelIRLS(
      int irls_rounds, bool compute_stability,
      RegionFlowFeatureList* feature_list,
      const PriorFeatureWeights* prior_weights,  // optional.
      CameraMotion* camera_motion) const;

  // Same as above for affine motion.
  // Note: feature_list is assumed to be normalized/transformed by
  // LinearSimilarity::NormalizationTransform N. Returned irls weights and
  // affine model are expressed in original frame, i.e. for estimated model
  // M, M' = N^(-1) M N is returned.
  bool EstimateAffineModelIRLS(int irls_rounds,
                               RegionFlowFeatureList* feature_list,
                               CameraMotion* camera_motion) const;

  // Same as above for homography.
  // Note: feature_list is assumed to be normalized/transformed by
  // LinearSimilarity::NormalizationTransform N. Returned irls weights and
  // homography are expressed in original frame, i.e. for estimated model
  // M, M' = N^(-1) M N is returned.
  // Returns true if estimation was successful, otherwise returns false and sets
  // the CameraMotion::type to INVALID.
  bool EstimateHomographyIRLS(
      int irls_rounds, bool compute_stability,
      const PriorFeatureWeights* prior_weights,       // optional.
      MotionEstimationThreadStorage* thread_storage,  // optional.
      RegionFlowFeatureList* feature_list, CameraMotion* camera_motion) const;

  // Same as above for mixture homography.
  // Note: feature_list is assumed to be normalized/transformed by
  // LinearSimilarity::NormalizationTransform N. Returned irls weights and
  // mixture homography are expressed in original frame, i.e. for estimated
  // model M, M' = N^(-1) M N is returned.
  // Mixture model estimation customized by MotionEstimationOptions.
  // Returns true if estimation was successful, otherwise returns false and sets
  // the CameraMotion::type to INVALID.
  // Supports computation for mixture spectrum, i.e. mixtures with different
  // regularizers. For default regularizer pass
  // MotionEstimationOptions::mixture_regularizer. Estimated motion will be
  // stored in CameraMotion::mixture_homography_spectrum(spectrum_idx).
  bool EstimateMixtureHomographyIRLS(
      int irls_rounds, bool compute_stability, float regularizer,
      int spectrum_idx,                               // 0 by default.
      const PriorFeatureWeights* prior_weights,       // optional.
      MotionEstimationThreadStorage* thread_storage,  // optional.
      RegionFlowFeatureList* feature_list, CameraMotion* camera_motion) const;

  // Returns weighted variance for mean translation from feature_list (assumed
  // to be in normalized coordinates). Returned variance is in unnormalized
  // domain.
  float TranslationVariance(const RegionFlowFeatureList& feature_list,
                            const Vector2_f& translation) const;

  // Replace each features irls weight by robust min-filtered irls weight
  // across each track.
  void MinFilterIrlsWeightByTrack(SingleTrackClipData* clip_data) const;

  // Performs filtering of irls weight across several tracking clip datas,
  // to yield consistent irls weights.
  void EnforceTrackConsistency(
      std::vector<SingleTrackClipData>* clip_datas) const;

  // Initializes or modifies prior_weights for passed feature_list by
  // biasing toward previous (filtered) IRLS weight for that feature.
  // This enables temporal coherence.
  void BiasLongFeatures(RegionFlowFeatureList* feature_list, MotionType type,
                        const EstimateModelOptions& model_options,
                        PriorFeatureWeights* prior_weights) const;

  // Called by above function to determine the bias each feature is multiplied
  // with.
  void BiasFromFeatures(const RegionFlowFeatureList& feature_list,
                        MotionType type,
                        const EstimateModelOptions& model_options,
                        std::vector<float>* bias) const;

  // Maps track index to tuple of spatial bias and number of similar
  // looking long tracks.
  typedef std::unordered_map<int, std::pair<float, float>> SpatialBiasMap;
  void ComputeSpatialBias(MotionType type,
                          const EstimateModelOptions& model_options,
                          RegionFlowFeatureList* feature_list,
                          SpatialBiasMap* spatial_bias) const;

  // Updates features weights in feature_list by temporally consistent bias.
  void UpdateLongFeatureBias(MotionType type,
                             const EstimateModelOptions& model_options,
                             bool remove_terminated_tracks,
                             bool update_irls_observation,
                             RegionFlowFeatureList* feature_list) const;

  // Bilateral filtering of irls weights across the passed list.
  void SmoothIRLSWeights(std::deque<float>* irls) const;

  // Helper function. Returns number of irls iterations for passed MotionType
  // derived from current MotionEstimationOptions. Returns zero, if no
  // estimation should be attempted.
  int IRLSRoundsFromSettings(const MotionType& type) const;

  // Partitions irls_rounds into several rounds with each having irls_per_round
  // interations each based on MotionEstimationOptions::EstimationPolicy.
  // Post-condition: total_rounds * irls_per_rounds == irls_rounds.
  void PolicyToIRLSRounds(int irls_rounds, int* total_rounds,
                          int* irls_per_round) const;

  // Check for specified MotionType is estimated model is stable. If not, resets
  // feature's irls weights to reset_irls_weights (optional) and resets motion
  // model in camera_motion to lower degree of freedom model. In this case,
  // CameraMotion::Type is flagged as UNSTABLE_* where * denotes the lower
  // degree of freedom model.
  // Model is only checked those CameraMotions with type less than or equal to
  // max_unstable_type.
  void CheckModelStability(
      const MotionType& type, const CameraMotion::Type& max_unstable_type,
      const std::vector<std::vector<float>>* reset_irls_weights,
      std::vector<RegionFlowFeatureList*>* feature_lists,
      std::vector<CameraMotion>* camera_motions) const;

  // Implementation function called by above function, to check for a single
  // model.
  void CheckSingleModelStability(const MotionType& type,
                                 const CameraMotion::Type& max_unstable_type,
                                 const std::vector<float>* reset_irls_weights,
                                 RegionFlowFeatureList* feature_list,
                                 CameraMotion* camera_motion) const;

  // Projects motion model specified by type to lower degree of freedom models.
  void ProjectMotionsDown(const MotionType& type,
                          std::vector<CameraMotion>* camera_motions) const;

  // Filters passed feature_lists based on
  // MotionEstimationOptions::irls_weight_filter.
  void IRLSWeightFilter(
      std::vector<RegionFlowFeatureList*>* feature_lists) const;

  // Inlier scale based on average motion magnitude and the fraction
  // of the magnitude that is still considered an inlier.
  // In general a residual of 1 pixel is assigned an IRLS weight of 1,
  // this function returns a residual scale, such that a residual
  // of distance_fraction * translation_magnitude equals an IRLS weight of 1
  // if multiplied by returned scale.
  float GetIRLSResidualScale(const float avg_motion_magnitude,
                             float distance_fraction) const;

  const LinearSimilarityModel& InverseNormalizationTransform() const {
    return inv_normalization_transform_;
  }

  const LinearSimilarityModel& NormalizationTransform() const {
    return normalization_transform_;
  }

  // Returns domain normalized features fall in.
  Vector2_f NormalizedDomain() const { return normalized_domain_; }

  // Returns index within the inlier mask for each feature point.
  // Also returns for each bin normalizer to account for different number of
  // features per bin during weighting.
  void ComputeFeatureMask(const RegionFlowFeatureList& feature_list,
                          std::vector<int>* mask_indices,
                          std::vector<float>* bin_normalizer) const;

  // Runs multiple rounds of RANSAC, resetting outlier IRLS weight to
  // a low score.
  // Optionally can perform temporally consistent selection if inlier_mask is
  // specified.
  // Returns best model across all iterations in best_model and true if
  // estimated model was deemed stable.
  bool GetTranslationIrlsInitialization(
      RegionFlowFeatureList* feature_list,
      const EstimateModelOptions& model_options, float avg_camera_motion,
      InlierMask* inlier_mask,  // optional.
      TranslationModel* best_model) const;

  // Same as above for linear similarities.
  bool GetSimilarityIrlsInitialization(
      RegionFlowFeatureList* feature_list,
      const EstimateModelOptions& model_options, float avg_camera_motion,
      InlierMask* inlier_mask,  // optional.
      LinearSimilarityModel* best_model) const;

  // Computes number of inliers and strict inliers (satisfying much stricter
  // threshold) for a given feature list after model fitting.
  void ComputeSimilarityInliers(const RegionFlowFeatureList& feature_list,
                                int* num_inliers,
                                int* num_strict_inliers) const;

  // Initializes irls weights based on setting
  // MotionEstimationOptions::homography_irls_weight_initialization.
  void GetHomographyIRLSCenterWeights(const RegionFlowFeatureList& feature_list,
                                      std::vector<float>* center_weights) const;

  // Checks for unreasonable large accelerationas between frames as specified by
  // MotionEstimationOptions::StableTranslationBounds.
  void CheckTranslationAcceleration(
      std::vector<CameraMotion>* camera_motions) const;

  // Functions below, test passed model is deemed stable according to
  // several heuristics set by Stable[MODEL]Bounds in MotionEstimationOptions.
  bool IsStableTranslation(const TranslationModel& normalized_translation,
                           float translation_variance,
                           const RegionFlowFeatureList& features) const;

  // Tests if passed similarity is stable. Pass number of inliers from
  // ComputeSimilarityInliers.
  bool IsStableSimilarity(const LinearSimilarityModel& model,
                          const RegionFlowFeatureList& features,
                          int num_inliers) const;

  bool IsStableHomography(const Homography& homography,
                          float average_homography_error,
                          float inlier_coverage) const;

  bool IsStableMixtureHomography(
      const MixtureHomography& homography, float min_block_inlier_coverage,
      const std::vector<float>& block_inlier_coverage) const;

  // Computes fraction (in [0, 1]) of inliers w.r.t. frame area using a grid of
  // occupancy cells. A feature is consider an inlier if its irls_weight is
  // larger or equal to min_inlier_score.
  float GridCoverage(const RegionFlowFeatureList& feature_list,
                     float min_inlier_score,
                     MotionEstimationThreadStorage* thread_storage) const;

  // Estimates per scanline-block coverage of mixture. If
  // assume_rolling_shutter_camera is set, low textured features are allowed to
  // have higher error as registration errors would not be as visible here.
  void ComputeMixtureCoverage(const RegionFlowFeatureList& feature_list,
                              float min_inlier_score,
                              bool assume_rolling_shutter_camera,
                              MotionEstimationThreadStorage* thread_storage,
                              CameraMotion* camera_motion) const;

  // Returns average motion magnitude as mean of the translation magnitude from
  // the 10th to 90th percentile.
  void EstimateAverageMotionMagnitude(const RegionFlowFeatureList& feature_list,
                                      CameraMotion* camera_motion) const;

  // Returns per iteration weight of the feature's irls weight initialization.
  float IRLSPriorWeight(int iteration, int irls_rounds) const;

  // Implementation function for above function. Estimates mixture homography
  // from features and returns true if estimation was non-degenerate.
  bool MixtureHomographyFromFeature(
      const TranslationModel& translation, int irls_rounds, float regularizer,
      const PriorFeatureWeights* prior_weights,  // optional.
      RegionFlowFeatureList* feature_list,
      MixtureHomography* mix_homography) const;

  // Determines overlay indices (spatial bin locations that are likely to be
  // affected by overlays) and stores them in corresponding member in
  // CameraMotion. Features that fall within these bins will be assigned a
  // weight of zero.
  void DetermineOverlayIndices(
      bool irls_weights_preinitialized,
      std::vector<CameraMotion>* camera_motions,
      std::vector<RegionFlowFeatureList*>* feature_lists) const;

  // Determine features likely to be part of a static overlay, by setting their
  // irls weight to zero.
  // Returns fraction of the image domain that is considered to be occupied by
  // overlays and specific overlay cell indices in overlay_indices.
  float OverlayAnalysis(const std::vector<TranslationModel>& translations,
                        std::vector<RegionFlowFeatureList*>* feature_lists,
                        std::vector<int>* overlay_indices) const;

  // Smooths feature's irls_weights spatio-temporally.
  void PostIRLSSmoothing(
      const std::vector<CameraMotion>& camera_motions,
      std::vector<RegionFlowFeatureList*>* feature_lists) const;

  // Initializes LUT for gaussian weighting. By default discretizes the domain
  // [0, max_range] into 4K bins, returning scale to map from a value in the
  // domain to the corresponding bin. If scale is nullptr max_range bins are
  // created instead (in this case scale would be 1.0, i.e. value equals bin
  // index).
  void InitGaussLUT(float sigma, float max_range, std::vector<float>* lut,
                    float* scale) const;

  // Performs fast volumetric smoothing / filtering of irls weights. Weights are
  // expected to be already binned using BuildFeatureGrid.
  void RunTemporalIRLSSmoothing(
      const std::vector<FeatureGrid<RegionFlowFeature>>& feature_grid,
      const std::vector<std::vector<int>>& feature_taps_3,
      const std::vector<std::vector<int>>& feature_taps_5,
      const std::vector<float>& frame_confidence,
      std::vector<RegionFlowFeatureView>* feature_views) const;

 private:
  MotionEstimationOptions options_;
  int frame_width_;
  int frame_height_;

  LinearSimilarityModel normalization_transform_;
  LinearSimilarityModel inv_normalization_transform_;

  // Transform from normalized features to irls domain.
  LinearSimilarityModel irls_transform_;

  // Frame dimensions transformed by normalization transform.
  Vector2_f normalized_domain_;
  std::unique_ptr<MixtureRowWeights> row_weights_;

  // For initialization biased towards previous frame.
  std::unique_ptr<InlierMask> inlier_mask_;

  // Stores current bias for each track and the last K irls observations.
  struct LongFeatureBias {
    explicit LongFeatureBias(float initial_weight) : bias(initial_weight) {
      irls_values.push_back(1.0f / initial_weight);
    }

    LongFeatureBias() : LongFeatureBias(1.0f) {}

    float bias = 1.0f;               // Current bias, stores pixel error,
                                     // i.e. 1 / IRLS.
    std::vector<float> irls_values;  // Recently observed IRLS values;
                                     // Ring buffer.
    int total_observations = 1;
  };

  // Maps track id to LongFeatureBias.
  typedef std::unordered_map<int, LongFeatureBias> LongFeatureBiasMap;

  // Bias map indexed by MotionType.
  mutable std::vector<LongFeatureBiasMap> long_feature_bias_maps_ =
      std::vector<LongFeatureBiasMap>(static_cast<int>(MODEL_NUM_VALUES));

  // Lookup tables and scale for FeatureBias computation.
  struct FeatureBiasLUT {
    // For ComputeSpatialBias weighting.
    std::vector<float> spatial_lut;
    float spatial_scale;
    std::vector<float> color_lut;
    float color_scale;

    // For BiasFromFeature computation.
    std::vector<float> bias_weight_lut;
    float bias_weight_scale;
  };

  FeatureBiasLUT feature_bias_lut_;

  // Counts the number of consecutive duplicate frames for each motion model.
  mutable std::vector<int> num_duplicate_frames_ =
      std::vector<int>(static_cast<int>(MODEL_NUM_VALUES));

  friend class EstimateMotionIRLSInvoker;
  friend class IrlsInitializationInvoker;
  friend class TrackFilterInvoker;
  friend class MotionEstimationThreadStorage;
};

// Meta-data set in the header of filter streams to communicate information used
// during camera motion estimation.
struct CameraMotionStreamHeader {
  CameraMotionStreamHeader() : frame_width(0), frame_height(0) {}
  int32_t frame_width;
  int32_t frame_height;
};

// Direct fitting functions.
TranslationModel FitTranslationModel(const RegionFlowFeatureList& features);

LinearSimilarityModel FitLinearSimilarityModel(
    const RegionFlowFeatureList& features);

AffineModel FitAffineModel(const RegionFlowFeatureList& features);

Homography FitHomography(const RegionFlowFeatureList& features);

MixtureHomography FitMixtureHomography(const RegionFlowFeatureList& features);

// Templated fitting functions.
template <class Model>
Model FitModel(const RegionFlowFeatureList& features);

template <>
inline TranslationModel FitModel(const RegionFlowFeatureList& features) {
  return FitTranslationModel(features);
}

template <>
inline LinearSimilarityModel FitModel(const RegionFlowFeatureList& features) {
  return FitLinearSimilarityModel(features);
}

template <>
inline AffineModel FitModel(const RegionFlowFeatureList& features) {
  return FitAffineModel(features);
}

template <>
inline Homography FitModel(const RegionFlowFeatureList& features) {
  return FitHomography(features);
}

template <>
inline MixtureHomography FitModel(const RegionFlowFeatureList& features) {
  return FitMixtureHomography(features);
}

// Generic projection function that projects models in an arbitrary direction
// (that is from lower to higher or vice versa) via fast model fits, without
// any error bound checking.
// MixtureRowWeights are only necessary for ToModel == MixtureHomography.
template <class ToModel, class FromModel>
ToModel ProjectViaFit(const FromModel& model, int frame_width, int frame_height,
                      MixtureRowWeights* row_weights = nullptr,
                      int grid_dim = 10) {
  // Build a grid of features.
  const float dx = frame_width * 1.0f / grid_dim;
  const float dy = frame_height * 1.0f / grid_dim;

  // Create region flow from grid.
  RegionFlowFeatureList grid_features;
  grid_features.set_frame_width(frame_width);
  grid_features.set_frame_height(frame_height);
  grid_features.set_match_frame(-1);

  for (int k = 0; k <= grid_dim; ++k) {
    for (int l = 0; l <= grid_dim; ++l) {
      auto* feat = grid_features.add_feature();
      feat->set_x(l * dx);
      feat->set_y(k * dy);
    }
  }

  RegionFlowFeatureListViaTransform(model, &grid_features, 1.0f,
                                    0.0f,   // Replace flow.
                                    false,  // Don't change feature loc.
                                    row_weights);
  return FitModel<ToModel>(grid_features);
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_MOTION_ESTIMATION_H_
