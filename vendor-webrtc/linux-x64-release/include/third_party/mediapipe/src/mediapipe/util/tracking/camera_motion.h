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

#ifndef MEDIAPIPE_UTIL_TRACKING_CAMERA_MOTION_H_
#define MEDIAPIPE_UTIL_TRACKING_CAMERA_MOTION_H_

#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/util/tracking/camera_motion.pb.h"
#include "mediapipe/util/tracking/motion_models.h"
#include "mediapipe/util/tracking/region_flow.pb.h"

namespace mediapipe {

// Helper functions to extract specific models from CameraMotion.
// Returned is always the requested model. In case, model is not present (i.e.
// has_<motion model> fails), the highest degree of freedom model
// (lower or equal to the requested model) that is present is embedded in the
// requested model.
// Presence of the model depends on wich models were requesteded to be
// estimated (via MotionEstimationOptions, to initialize requested models to
// identity, use ResetMotionModels above). For example, assume linear similarity
// was not requested to be estimated, but affine was requested. If
// CameraMotionToLinearSimilarity is called, has_linear_similarity would be
// false and the function fall back returning a translation model.
void CameraMotionToTranslation(const CameraMotion& camera_motion,
                               TranslationModel* model);
void CameraMotionToLinearSimilarity(const CameraMotion& camera_motion,
                                    LinearSimilarityModel* model);
void CameraMotionToAffine(const CameraMotion& camera_motion,
                          AffineModel* model);
void CameraMotionToHomography(const CameraMotion& camera_motion,
                              Homography* homography);
void CameraMotionToMixtureHomography(const CameraMotion& camera_motion,
                                     MixtureHomography* mixture);

// TODO: Under development ...
// Returns camera motion lhs * rhs. Initial camera motion is set to rhs
// before composition.
CameraMotion ComposeCameraMotion(const CameraMotion& lhs,
                                 const CameraMotion& rhs);

// Inverts every motion model that is set in CameraMotion.
CameraMotion InvertCameraMotion(const CameraMotion& motion);

// Templated wrapper for above calls.
template <class Model>
Model CameraMotionToModel(const CameraMotion& camera_motion);

// Returns model from passed CameraMotion specified by unstable_type
// (which must name a type != VALID, CHECK-ed) and embeds it in the specified
// Model.
template <class Model>
Model UnstableCameraMotionToModel(const CameraMotion& camera_motion,
                                  CameraMotion::Type unstable_type);

// Projects passed model to lower degree of freedom model (embedded in original
// type), as specified type. In case type is valid, function is effectively
// identity function.
// Only implemented for the following models:
// - Translation
// - LinearSimilarity
// - AffineModel
template <class Model>
Model ProjectToTypeModel(const Model& model, float frame_width,
                         float frame_height, CameraMotion::Type type);

// Substract camera motion (specifically highest, degree of freedom model,
// that has been estimated reliably) from feature lists. Operates on vectors
// for improved performance. Size of camera_motions can be larger than
// feature_lists, in this case last camera motions are ignored.
void SubtractCameraMotionFromFeatures(
    const std::vector<CameraMotion>& camera_motions,
    std::vector<RegionFlowFeatureList*>* feature_lists);

// Returns average motion magnitude after subtracting camera motion.
float ForegroundMotion(const CameraMotion& camera_motion,
                       const RegionFlowFeatureList& feature_list);

// Initializes a CameraMotion with its corresponding fields from a
// RegionFlowFeatureList.
void InitCameraMotionFromFeatureList(const RegionFlowFeatureList& feature_list,
                                     CameraMotion* camera_motion);

// Converts Camera motion flag to string.
std::string CameraMotionFlagToString(const CameraMotion& motion);

// Converts Camera motion type to string. Used instead of builtin proto function
// for mobile support.
std::string CameraMotionTypeToString(const CameraMotion& motion);

// Returns inlier coverage either based on mixture (if present, in this case
// return mean of block coverages) or else homography.
// If neither is present, returns 0 to signal insufficient inliers.
// If use_homography_coverage is set, uses homography even when mixture is
// present.
float InlierCoverage(const CameraMotion& camera_motion,
                     bool use_homography_coverage);

// Downsamples passed motion models temporally by specified downsample_scale,
// i.e. for models F_0, F_1, F_2, F_3, F_4 and downsample_scale of 2, models:
// F_0 * F_1, F_2 * F_3 and F_4 are returned.
// Optionally also performs downsampling of corresponding model_type returning
// the least unstable for each composition.
template <class Model>
void DownsampleMotionModels(
    const std::vector<Model>& models,
    const std::vector<CameraMotion::Type>* model_type,  // optional.
    int downsample_scale, std::vector<Model>* downsampled_models,
    std::vector<CameraMotion::Type>* downsampled_types);

// Compatible subsampling method to above DownsampleMotionModels.
// Note, when downsampling for example:
// F_0, F_1, F_2, F_3, F_4  by factor 3 via above function, downsampled result
// will be F_0 * F_1 * F_2, F_3 * F_4
// so we would need to pick entities at F_2 and F_4.
// Template class Container must be SequenceContainer, like
// std::vector, std::deque.
template <class Container>
void SubsampleEntities(const Container& input, int downsample_scale,
                       Container* output);

// For perfect looping, this function computes the motion in the first frame
// to be the inverse of the accumulated motion from frame 1 to N.
// If a particular motion type is not available or not invertible at any
// frame pair, the original motion for that type is retained.
// Does not work if mixtures are present.
template <class CameraMotionContainer>  // STL container of CameraMotion's
CameraMotion FirstCameraMotionForLooping(
    const CameraMotionContainer& container);

// Template implementation functions.

template <class Model>
Model UnstableCameraMotionToModel(const CameraMotion& camera_motion,
                                  CameraMotion::Type unstable_type) {
  switch (unstable_type) {
    case CameraMotion::INVALID:
      return Model();  // Identity.

    case CameraMotion::UNSTABLE: {
      return ModelAdapter<Model>::Embed(
          CameraMotionToModel<TranslationModel>(camera_motion));
    }

    case CameraMotion::UNSTABLE_SIM: {
      return ModelAdapter<Model>::Embed(
          CameraMotionToModel<LinearSimilarityModel>(camera_motion));
    }

    case CameraMotion::UNSTABLE_HOMOG: {
      return ModelAdapter<Model>::Embed(
          CameraMotionToModel<Homography>(camera_motion));
    }

    case CameraMotion::VALID:
      ABSL_LOG(FATAL) << "Specify a type != VALID";
      return Model();
  }
}

template <>
inline TranslationModel ProjectToTypeModel(const TranslationModel& model,
                                           float frame_width,
                                           float frame_height,
                                           CameraMotion::Type type) {
  switch (type) {
    case CameraMotion::INVALID:
      return TranslationModel();  // Identity.
    default:
      return model;
  }
}

template <>
inline LinearSimilarityModel ProjectToTypeModel(
    const LinearSimilarityModel& model, float frame_width, float frame_height,
    CameraMotion::Type type) {
  switch (type) {
    case CameraMotion::INVALID:
      return LinearSimilarityModel();  // Identity.

    case CameraMotion::UNSTABLE:
      return LinearSimilarityAdapter::Embed(
          TranslationAdapter::ProjectFrom(model, frame_width, frame_height));

    default:
      return model;
  }
}

template <class Model>
Model ProjectToTypeModel(const Model& model, float frame_width,
                         float frame_height, CameraMotion::Type type) {
  switch (type) {
    case CameraMotion::INVALID:
      return Model();  // Identity.

    case CameraMotion::UNSTABLE:
      return ModelAdapter<Model>::Embed(
          TranslationAdapter::ProjectFrom(model, frame_width, frame_height));

    case CameraMotion::UNSTABLE_SIM:
      return ModelAdapter<Model>::Embed(LinearSimilarityAdapter::ProjectFrom(
          model, frame_width, frame_height));

      // case UNSTABLE_HOMOG does not occur except for mixtures.

    default:
      return model;
  }
}

template <>
inline MixtureHomography ProjectToTypeModel(const MixtureHomography&, float,
                                            float, CameraMotion::Type) {
  ABSL_LOG(FATAL) << "Projection not supported for mixtures.";
  return MixtureHomography();
}

template <class Model>
void DownsampleMotionModels(
    const std::vector<Model>& models,
    const std::vector<CameraMotion::Type>* model_type, int downsample_scale,
    std::vector<Model>* downsampled_models,
    std::vector<CameraMotion::Type>* downsampled_types) {
  if (model_type) {
    ABSL_CHECK_EQ(models.size(), model_type->size());
    ABSL_CHECK(downsampled_models) << "Expecting output models.";
  }

  ABSL_CHECK(downsampled_models);
  downsampled_models->clear();
  if (downsampled_types) {
    downsampled_types->clear();
  }

  const int num_models = models.size();

  for (int model_idx = 0; model_idx < num_models;
       model_idx += downsample_scale) {
    const int last_idx =
        std::min<int>(model_idx + downsample_scale, num_models) - 1;

    CameraMotion::Type sampled_type = CameraMotion::VALID;
    if (model_type) {
      // Get least stable model within downsample window (max operation).
      for (int i = model_idx; i <= last_idx; ++i) {
        sampled_type = std::max(sampled_type, model_type->at(i));
      }
      downsampled_types->push_back(sampled_type);
    }

    // Concatenate models.
    Model composed = models[last_idx];

    for (int i = last_idx - 1; i >= model_idx; --i) {
      composed = ModelCompose2(models[i], composed);
    }

    downsampled_models->push_back(composed);
  }
}

template <class Container>
void SubsampleEntities(const Container& input, int downsample_factor,
                       Container* output) {
  ABSL_CHECK(output);
  output->clear();

  if (input.empty()) {
    return;
  }

  for (int k = downsample_factor - 1; k < input.size();
       k += downsample_factor) {
    output->push_back(input[k]);
  }

  if (input.size() % downsample_factor != 0) {
    // We need to add last constraint as termination.
    output->push_back(input.back());
  }
}

template <>
inline TranslationModel CameraMotionToModel(const CameraMotion& camera_motion) {
  TranslationModel model;
  CameraMotionToTranslation(camera_motion, &model);
  return model;
}

template <>
inline LinearSimilarityModel CameraMotionToModel(
    const CameraMotion& camera_motion) {
  LinearSimilarityModel model;
  CameraMotionToLinearSimilarity(camera_motion, &model);
  return model;
}

template <>
inline AffineModel CameraMotionToModel(const CameraMotion& camera_motion) {
  AffineModel model;
  CameraMotionToAffine(camera_motion, &model);
  return model;
}

template <>
inline Homography CameraMotionToModel(const CameraMotion& camera_motion) {
  Homography model;
  CameraMotionToHomography(camera_motion, &model);
  return model;
}

template <>
inline MixtureHomography CameraMotionToModel(
    const CameraMotion& camera_motion) {
  MixtureHomography model;
  CameraMotionToMixtureHomography(camera_motion, &model);
  return model;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_CAMERA_MOTION_H_
