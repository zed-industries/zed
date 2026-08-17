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

#ifndef MEDIAPIPE_UTIL_TRACKING_MOTION_MODELS_H_
#define MEDIAPIPE_UTIL_TRACKING_MOTION_MODELS_H_

#include <algorithm>
#include <cmath>
#include <string>
#include <vector>

#include "absl/container/node_hash_map.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/singleton.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/camera_motion.pb.h"
#include "mediapipe/util/tracking/motion_models.pb.h"
#include "mediapipe/util/tracking/region_flow.pb.h"  // NOLINT

namespace mediapipe {

// Abstract Camera Model, functionality that each model must support.
template <class Model>
class ModelAdapter {
 public:
  // Initializes a model from vector to data.
  // If identity_parametrization is set, it is assumed that for args = 0 ->
  // Model = identity, else args = 0 -> zero transform.
  static Model FromFloatPointer(const float* args,
                                bool identity_parametrization);
  static Model FromDoublePointer(const double* args,
                                 bool identity_parametrization);

  // Transforms points according to model * pt.
  static Vector2_f TransformPoint(const Model& model, const Vector2_f& pt);

  // Returns model^(-1), outputs feasibility to variable success (can not be
  // NULL). If model is not invertible, function should return identity model.
  static Model InvertChecked(const Model& model, bool* success);

  // Returns model^(-1), returns identity model if inversion is not possible,
  // and warns via ABSL_LOG(ERROR). It is recommended that InvertChecked is used
  // instead.
  // Note: Default implementation, motion models only need to supply above
  // function.
  static Model Invert(const Model& model);

  // Concatenates two models. Returns lhs * rhs
  static Model Compose(const Model& lhs, const Model& rhs);

  // Debugging function to create plots. Output parameters separated by space.
  static std::string ToString(const Model& model);

  // Returns number of DOF.
  static constexpr int NumParameters();

  // Access parameters in a model agnostic way. Order is the order of
  // specification in the corresponding motion_models.proto file, i.e.
  // id = proto_id - 1.
  static float GetParameter(const Model& model, int id);

  // Sets parameter in a model agonstic way. Same interpretation of id as for
  // GetParameter.
  static void SetParameter(int id, float value, Model* model);

  // Returns normalization transform for specific frame size. Range after
  // normalization is [0, 1].
  static Model NormalizationTransform(float frame_width, float frame_height);

  // Embeds a lower paramteric model into this model.
  // Overload for specialization.
  template <class LowerModel>
  static Model Embed(const LowerModel& model);

  // Projects higher motion model H to lower one L such that for points x_i
  // \sum_i || H * x - L * x||  == min
  // For this we transform the models to the center of the specified frame
  // domain at which degree's of freedom are usually independent. Details can be
  // found in the individual implementation functions.
  template <class HigherModel>
  static Model ProjectFrom(const HigherModel& model, float frame_width,
                           float frame_height);
};

// Specialized implementations, with additional functionality if needed.
template <>
class ModelAdapter<TranslationModel> {
 public:
  static TranslationModel FromArgs(float dx, float dy);
  static TranslationModel FromFloatPointer(const float* args,
                                           bool identity_parametrization);
  static TranslationModel FromDoublePointer(const double* args,
                                            bool identity_parametrization);

  static Vector2_f TransformPoint(const TranslationModel& model,
                                  const Vector2_f& pt);
  static TranslationModel Invert(const TranslationModel& model);
  static TranslationModel InvertChecked(const TranslationModel& model,
                                        bool* success);
  static TranslationModel Compose(const TranslationModel& lhs,
                                  const TranslationModel& rhs);
  static float GetParameter(const TranslationModel& model, int id);
  static void SetParameter(int id, float value, TranslationModel* model);

  static std::string ToString(const TranslationModel& model);

  static constexpr int NumParameters() { return 2; }

  // Support to convert to and from affine.
  static AffineModel ToAffine(const TranslationModel& model);

  // Fails with debug check, if affine model is not a translation.
  static TranslationModel FromAffine(const AffineModel& model);

  static Homography ToHomography(const TranslationModel& model);

  // Fails with debug check if homography is not a translation.
  static TranslationModel FromHomography(const Homography& model);

  // Evaluates Jacobian at specified point and parameters set to 0.
  // Note: This is independent of whether identity parametrization was used
  // or not via From*Pointer.
  // Jacobian has to be of size 2 x NumParams(), and is returned in column
  // major order.
  static void GetJacobianAtPoint(const Vector2_f& pt, float* jacobian);

  static TranslationModel NormalizationTransform(float frame_width,
                                                 float frame_height);

  static TranslationModel Embed(const TranslationModel& model) { return model; }

  static TranslationModel ProjectFrom(const TranslationModel& model,
                                      float frame_width, float frame_height) {
    return model;
  }

  static TranslationModel ProjectFrom(const LinearSimilarityModel& model,
                                      float frame_width, float frame_height);

  static TranslationModel ProjectFrom(const AffineModel& model,
                                      float frame_width, float frame_height);

  static TranslationModel ProjectFrom(const Homography& model,
                                      float frame_width, float frame_height);

  // Returns parameter wise maximum.
  static TranslationModel Maximum(const TranslationModel& lhs,
                                  const TranslationModel& rhs);

  // Return determinant of model.
  static float Determinant(const TranslationModel& unused) { return 1; }
};

template <>
class ModelAdapter<SimilarityModel> {
 public:
  static SimilarityModel FromArgs(float dx, float dy, float scale,
                                  float rotation);
  static SimilarityModel FromFloatPointer(const float* args, bool);
  static SimilarityModel FromDoublePointer(const double* args, bool);

  static Vector2_f TransformPoint(const SimilarityModel& model,
                                  const Vector2_f& pt);
  static SimilarityModel Invert(const SimilarityModel& model);
  static SimilarityModel InvertChecked(const SimilarityModel& model,
                                       bool* success);
  static SimilarityModel Compose(const SimilarityModel& lhs,
                                 const SimilarityModel& rhs);

  static float GetParameter(const SimilarityModel& model, int id);
  static void SetParameter(int id, float value, SimilarityModel* model);

  static std::string ToString(const SimilarityModel& model);

  static constexpr int NumParameters() { return 4; }

  static SimilarityModel NormalizationTransform(float frame_width,
                                                float frame_height);

  static TranslationModel ProjectToTranslation(const SimilarityModel& model,
                                               float frame_width,
                                               float frame_height);
};

template <>
class ModelAdapter<LinearSimilarityModel> {
 public:
  static LinearSimilarityModel FromArgs(float dx, float dy, float a, float b);
  static LinearSimilarityModel FromFloatPointer(const float* args,
                                                bool identity_parametrization);
  static LinearSimilarityModel FromDoublePointer(const double* args,
                                                 bool identity_parametrization);

  static Vector2_f TransformPoint(const LinearSimilarityModel& model,
                                  const Vector2_f& pt);
  static LinearSimilarityModel Invert(const LinearSimilarityModel& model);
  static LinearSimilarityModel InvertChecked(const LinearSimilarityModel& model,
                                             bool* success);

  static LinearSimilarityModel Compose(const LinearSimilarityModel& lhs,
                                       const LinearSimilarityModel& rhs);

  static float GetParameter(const LinearSimilarityModel& model, int id);
  static void SetParameter(int id, float value, LinearSimilarityModel* model);

  static std::string ToString(const LinearSimilarityModel& model);

  static constexpr int NumParameters() { return 4; }

  // Support to convert to and from affine.
  static AffineModel ToAffine(const LinearSimilarityModel& model);

  static Homography ToHomography(const LinearSimilarityModel& model);

  // Fails with debug check, if homography is not a similarity.
  static LinearSimilarityModel FromHomography(const Homography& model);

  // Fails with debug check, if affine model is not a similarity.
  static LinearSimilarityModel FromAffine(const AffineModel& model);

  // Conversion from and to non-linear similarity.
  static SimilarityModel ToSimilarity(const LinearSimilarityModel& model);
  static LinearSimilarityModel FromSimilarity(const SimilarityModel& model);

  // Additional functionality:

  // Composes scaled identity transform with model (sI * model).
  static LinearSimilarityModel ScaleParameters(
      const LinearSimilarityModel& model, float scale);

  // Adds identity I to model (I + model).
  static LinearSimilarityModel AddIdentity(const LinearSimilarityModel& model);

  // Evaluates Jacobian at specified point and parameters set to 0.
  // Note: This is independent of whether identity parametrization was used
  // or not via From*Pointer.
  // Jacobian has to be of size 2 x NumParams(), and is returned in column
  // major order.
  static void GetJacobianAtPoint(const Vector2_f& pt, float* jacobian);

  static LinearSimilarityModel NormalizationTransform(float frame_width,
                                                      float frame_height);

  static LinearSimilarityModel Embed(const TranslationModel& model) {
    return FromArgs(model.dx(), model.dy(), 1, 0);
  }

  static LinearSimilarityModel Embed(const LinearSimilarityModel& model) {
    return model;
  }

  static TranslationModel ProjectToTranslation(
      const LinearSimilarityModel& model, float frame_width,
      float frame_height);

  static LinearSimilarityModel ProjectFrom(const LinearSimilarityModel& model,
                                           float frame_width,
                                           float frame_height) {
    return model;
  }

  static LinearSimilarityModel ProjectFrom(const AffineModel& model,
                                           float frame_width,
                                           float frame_height);

  static LinearSimilarityModel ProjectFrom(const Homography& model,
                                           float frame_width,
                                           float frame_height);
  // Returns parameter wise maximum.
  static LinearSimilarityModel Maximum(const LinearSimilarityModel& lhs,
                                       const LinearSimilarityModel& rhs);

  // Return determinant of model.
  static float Determinant(const LinearSimilarityModel& m) {
    return m.a() * m.a() + m.b() * m.b();
  }
};

template <>
class ModelAdapter<AffineModel> {
 public:
  static AffineModel FromArgs(float dx, float dy, float a, float b, float c,
                              float d);
  static AffineModel FromFloatPointer(const float* args,
                                      bool identity_parametrization);
  static AffineModel FromDoublePointer(const double* args,
                                       bool identity_parametrization);

  static Vector2_f TransformPoint(const AffineModel& model,
                                  const Vector2_f& pt);
  static AffineModel Invert(const AffineModel& model);
  static AffineModel InvertChecked(const AffineModel& model, bool* success);
  static AffineModel Compose(const AffineModel& lhs, const AffineModel& rhs);

  static float GetParameter(const AffineModel& model, int id);
  static void SetParameter(int id, float value, AffineModel* model);

  static std::string ToString(const AffineModel& model);

  static constexpr int NumParameters() { return 6; }

  // Support to convert to and from affine.
  static AffineModel ToAffine(const AffineModel& model) { return model; }

  static AffineModel FromAffine(const AffineModel& model) { return model; }

  static Homography ToHomography(const AffineModel& model);

  // Fails with debug check, if homography is not affine.
  static AffineModel FromHomography(const Homography& model);

  // Additional functionality:

  // Composes scaled identity transform with model (sI * model).
  static AffineModel ScaleParameters(const AffineModel& model, float scale);

  // Adds identity I to model (I + model).
  static AffineModel AddIdentity(const AffineModel& model);

  // Evaluates Jacobian at specified point and parameters set to 0.
  // Note: This is independent of whether identity parametrization was used
  // or not via From*Pointer.
  // Jacobian has to be of size 2 x NumParams(), and is returned in column
  // major order.
  static void GetJacobianAtPoint(const Vector2_f& pt, float* jacobian);

  static AffineModel NormalizationTransform(float frame_width,
                                            float frame_height);

  static AffineModel Embed(const TranslationModel& model) {
    return FromArgs(model.dx(), model.dy(), 1, 0, 0, 1);
  }

  static AffineModel Embed(const LinearSimilarityModel& model) {
    return FromArgs(model.dx(), model.dy(), model.a(), -model.b(), model.b(),
                    model.a());
  }

  static AffineModel Embed(const AffineModel& model) { return model; }

  static AffineModel ProjectFrom(const AffineModel& model, float frame_width,
                                 float frame_height) {
    return model;
  }

  static AffineModel ProjectFrom(const Homography& model, float frame_width,
                                 float frame_height);

  static LinearSimilarityModel ProjectToLinearSimilarity(
      const AffineModel& model, float frame_width, float frame_height);

  // Returns parameter wise maximum.
  static AffineModel Maximum(const AffineModel& lhs, const AffineModel& rhs);

  static float Determinant(const AffineModel& m) {
    return m.a() * m.d() - m.b() * m.c();
  }
};

template <>
class ModelAdapter<Homography> {
 public:
  static Homography FromArgs(float h_00, float h_01, float h_02, float h_10,
                             float h_11, float h_12, float h_20, float h_21);

  static Homography FromFloatPointer(const float* args,
                                     bool identity_parametrization);
  static Homography FromDoublePointer(const double* args,
                                      bool identity_parametrization);

  static Vector2_f TransformPoint(const Homography& model, const Vector2_f& pt);

  static Vector3_f TransformPoint3(const Homography& model,
                                   const Vector3_f& pt);

  static Homography Invert(const Homography& model);
  static Homography InvertChecked(const Homography& model, bool* success);
  static Homography Compose(const Homography& lhs, const Homography& rhs);

  static float GetParameter(const Homography& model, int id);
  static void SetParameter(int id, float value, Homography* model);

  static std::string ToString(const Homography& model);

  static constexpr int NumParameters() { return 8; }

  static bool IsAffine(const Homography& model);

  // Support to convert to and from affine. Debug check that model is actually
  // an affine model.
  static AffineModel ToAffine(const Homography& model);
  static Homography FromAffine(const AffineModel& model);

  static Homography ToHomography(const Homography& model) { return model; }
  static Homography FromHomography(const Homography& model) { return model; }

  // Additional functionality:
  // Evaluates Jacobian at specified point and parameters set to 0.
  // Note: This is independent of whether identity parametrization was used
  // or not via From*Pointer.
  // Jacobian has to be of size 2 x NumParams(), and is returned in column
  // major order.
  static void GetJacobianAtPoint(const Vector2_f& pt, float* jacobian);

  static Homography NormalizationTransform(float frame_width,
                                           float frame_height);

  static Homography Embed(const Homography& model) { return model; }

  static Homography Embed(const AffineModel& model) {
    return FromArgs(model.a(), model.b(), model.dx(), model.c(), model.d(),
                    model.dy(), 0, 0);
  }

  static Homography Embed(const LinearSimilarityModel& model) {
    return FromArgs(model.a(), -model.b(), model.dx(), model.b(), model.a(),
                    model.dy(), 0, 0);
  }

  static Homography Embed(const TranslationModel& model) {
    return FromArgs(1, 0, model.dx(), 0, 1, model.dy(), 0, 0);
  }

  static float Determinant(const Homography& m) {
    // Applying laplace formula to last column.
    // h_00  h_01  h_02
    // h_10  h_11  h_12
    // h_20  h_21  1        <-- apply laplace.
    return m.h_20() * (m.h_01() * m.h_12() - m.h_11() * m.h_02()) +
           -m.h_21() * (m.h_00() * m.h_12() - m.h_10() * m.h_02()) +
           1.0f * (m.h_00() * m.h_11() - m.h_10() * m.h_01());
  }

  static AffineModel ProjectToAffine(const Homography& model, float frame_width,
                                     float frame_height);
};

// Common algorithms implemented using corresponding ModelAdapter.
// Implemented in cc file, explicitly instantiated below.
template <class Model>
class ModelMethods {
  typedef ModelAdapter<Model> Adapter;

 public:
  // Returns _normalized_ intersection area of rectangle transformed by
  // model_1 and model_2, respectively.
  static float NormalizedIntersectionArea(const Model& model_1,
                                          const Model& model_2,
                                          const Vector2_f& rect);
};

// Traits to bind mixture and corresponding base model together.
template <class BaseModel, class MixtureModel>
struct MixtureTraits {
  typedef BaseModel base_model;
  typedef MixtureModel model;
};

typedef MixtureTraits<LinearSimilarityModel, MixtureLinearSimilarity>
    LinearSimilarityTraits;
typedef MixtureTraits<AffineModel, MixtureAffine> AffineTraits;
typedef MixtureTraits<Homography, MixtureHomography> HomographyTraits;

template <class MixtureTraits>
class MixtureModelAdapterBase {
 public:
  typedef typename MixtureTraits::model MixtureModel;
  typedef typename MixtureTraits::base_model BaseModel;
  typedef ModelAdapter<BaseModel> BaseModelAdapter;

  // Initializes a model from vector to data. All weights are set to one.
  // If identity_parametrization is set, it is assumed that for args = 0 ->
  // Model = identity, else args = 0 -> zero transform.
  // Adjacent models are assumed to be separated by
  // NumParameters() + skip elements.
  static MixtureModel FromFloatPointer(const float* args,
                                       bool identity_parametrization, int skip,
                                       int num_models);
  static MixtureModel FromDoublePointer(const double* args,
                                        bool identity_parametrization, int skip,
                                        int num_models);

  // Mixture models are not closed under composition and inversion.
  // Instead, each point has to be transformed via above functions.
  // However, a mixture model can be composed with a BaseModel from either left
  // or right, by component-wise composition.
  // Returns mixture_model * base_model.
  static MixtureModel ComposeRight(const MixtureModel& mixture_model,
                                   const BaseModel& base_model);

  // Returns base_model * mixture_model.
  static MixtureModel ComposeLeft(const MixtureModel& mixture_model,
                                  const BaseModel& base_model);

  // Debugging function to create plots. Output parameters separated by delim.
  static std::string ToString(const MixtureModel& model, std::string delim);

  // Returns total number of DOF (number of models * BaseModel DOF)
  static int NumParameters(const MixtureModel& model) {
    return model.model_size() * BaseModelAdapter::NumParameters();
  }

  // Access parameters in a model agnostic way. Order is the order of
  // specification in the corresponding motion_models.proto file, i.e.
  // id = proto_id - 1.
  static float GetParameter(const MixtureModel& model, int model_id,
                            int param_id);

  static void SetParameter(int model_id, int param_id, float value,
                           MixtureModel* model);

  static MixtureModel IdentityModel(int num_mixtures);

  // Returns average model across mixture, i.e. mean of each parameter across
  // the mixture.
  static BaseModel MeanModel(const MixtureModel& model);

  // Fits a linear model to each parameter across mixture and returns mixture
  // evaluated across line.
  static MixtureModel LinearModel(const MixtureModel& model);

  static MixtureModel Embed(const BaseModel& base_model, int num_mixtures);
};

class MixtureRowWeights;

template <class MixtureTraits>
class MixtureModelAdapter : public MixtureModelAdapterBase<MixtureTraits> {
 public:
  typedef typename MixtureModelAdapterBase<MixtureTraits>::MixtureModel
      MixtureModel;
  typedef typename MixtureModelAdapterBase<MixtureTraits>::BaseModel BaseModel;
  typedef ModelAdapter<BaseModel> BaseModelAdapter;

  // Returns convex combination of models from supplied mixture_model,
  // specifically:
  // \sum_i mixture_model.model(i) * weights[i]
  // where:
  // b) weights[i] need to be normalized to sum to one.
  static BaseModel ToBaseModel(const MixtureModel& mixture_model,
                               const float* weights);

  // Transforms points according ToBaseModel(model, weights) * pt;
  // Note: Weights need to sum to one (not checked).
  static Vector2_f TransformPoint(const MixtureModel& model,
                                  const float* weights, const Vector2_f& pt);

  static Vector2_f TransformPoint(const MixtureModel& model,
                                  const MixtureRowWeights& weights,
                                  const Vector2_f& pt);

  // Transforms / solves for points according to
  // ToBaseModel(model, weights)^(-1) * pt
  // Fails with CHECK if model is not invertible.
  // Note: Weights need to sum to one (not checked).
  static Vector2_f SolveForPoint(const MixtureModel& model,
                                 const float* weights, const Vector2_f& pt);

  // Same as above, indicating if model is invertible in parameter
  // success. If model is not invertible, passed parameter
  // pt is returned unchanged.
  static Vector2_f SolveForPointChecked(const MixtureModel& model,
                                        const float* weights,
                                        const Vector2_f& pt, bool* success);
};

// Re-implemented for speed benefits.
template <>
class MixtureModelAdapter<HomographyTraits>
    : public MixtureModelAdapterBase<HomographyTraits> {
 public:
  inline static Homography ToBaseModel(const MixtureHomography& model,
                                       const float* weights);
  inline static Vector2_f TransformPoint(const MixtureHomography& model,
                                         const float* weights,
                                         const Vector2_f& pt);
  // Overload. OK as only input format for weights changed.
  inline static Vector2_f TransformPoint(const MixtureHomography& model,
                                         const MixtureRowWeights& weights,
                                         const Vector2_f& pt);

  inline static Vector2_f SolveForPoint(const MixtureHomography& model,
                                        const float* weights,
                                        const Vector2_f& pt);

  inline static Vector2_f SolveForPointChecked(const MixtureModel& model,
                                               const float* weights,
                                               const Vector2_f& pt,
                                               bool* success);
};

// Compositing for multiple models in order of argument list.
template <class Model>
Model ModelCompose2(const Model& a, const Model& b) {
  typedef ModelAdapter<Model> Adapter;
  return Adapter::Compose(a, b);
}

template <class Model>
Model ModelCompose3(const Model& a, const Model& b, const Model& c) {
  typedef ModelAdapter<Model> Adapter;
  return Adapter::Compose(a, Adapter::Compose(b, c));
}

template <class Model>
Model ModelCompose4(const Model& a, const Model& b, const Model& c,
                    const Model& d) {
  typedef ModelAdapter<Model> Adapter;
  return Adapter::Compose(a, Adapter::Compose(b, Adapter::Compose(c, d)));
}

template <class Model>
Model ModelInvert(const Model& model) {
  typedef ModelAdapter<Model> Adapter;
  return Adapter::Invert(model);
}

// Returns model according to b^(-1) * a
template <class Model>
Model ModelDiff(const Model& a, const Model& b) {
  typedef ModelAdapter<Model> Adapter;
  return Adapter::Compose(Adapter::Invert(b), a);
}

template <class Model>
Model ModelDiffChecked(const Model& a, const Model& b, bool* success) {
  typedef ModelAdapter<Model> Adapter;
  Model b_inv = Adapter::InvertChecked(b, success);
  return Adapter::Compose(b_inv, a);
}

template <class Model>
Vector2_f TransformPoint(const Model& m, const Vector2_f& v) {
  return ModelAdapter<Model>::TransformPoint(m, v);
}

// Epsilon threshold for determinant. Below this threshold we consider
// the linear model to be non-invertible.
const float kDetInvertibleEps = 1e-10;

// Threshold for stability. Used to determine if a particular motion model
// is invertible AND likely to be stable after inversion (imposes higher
// threshold on determinant than just for invertibility).
const float kDetStableEps = 1e-2;

template <class Model>
bool IsInverseStable(const Model& model) {
  return ModelAdapter<Model>::Determinant(model) > kDetStableEps;
}

// Accumulates camera motions in accum:
// If motions for frames 1..N are: F1, F2, .. FN, where Fk is the motion that
// maps frame k to k-1 (backwards motion), then cumulative motion mapping frame
// N to 0 is:
// C = F1 * F2 * ... * FN.
//
// This function computes it recursively: C(k) = C(k-1) * Fk.
template <class Model>
void AccumulateModel(const Model& model, Model* accum) {
  *accum = ModelCompose2(*accum, model);
}

// Accumulates inverse camera motions in accum_inverted:
// We want to compute the inverse motion that maps frame 0 to frame N:
// C^-1 = FN^-1 *  .... * F2^-1 * F1^-1.
// (inverse of C defined above for AccumulateModel).
//
// This function computes it recursively: C(k)^-1 = Fk^-1 * C(k-1)^-1.
//
// Return value indicates accumulation was successful (it might fail if model
// is not invertible), otherwise accum_inverted is left unchanged.
template <class Model>
bool AccumulateInvertedModel(const Model& model, Model* accum_inverted) {
  bool success = true;
  const Model inv_model = ModelAdapter<Model>::InvertChecked(model, &success);
  if (success) {
    *accum_inverted = ModelCompose2(inv_model, *accum_inverted);
  }
  return success;
}

// Returns true if |predicted * ground_truth^(-1)| < bounds (element wise).
// Use UniformModel to initialize bounds.
template <class Model>
bool ModelDiffWithinBounds(const Model& ground_truth, const Model& predicted,
                           const Model& bounds) {
  Model diff = ModelAdapter<Model>::Compose(
      predicted, ModelAdapter<Model>::Invert(ground_truth));
  Model identity;
  for (int p = 0; p < ModelAdapter<Model>::NumParameters(); ++p) {
    const float bound = ModelAdapter<Model>::GetParameter(bounds, p);
    const float diff_p = fabs(ModelAdapter<Model>::GetParameter(diff, p) -
                              ModelAdapter<Model>::GetParameter(identity, p));

    if (diff_p > bound) {
      ABSL_LOG(WARNING) << "Param diff " << p << " out of bounds: " << diff_p
                        << " > " << bound << " bound";
      return false;
    }
  }
  return true;
}

// Returns true if model is identity within floating point accuracy.
template <class Model>
bool IsModelIdentity(const Model& model) {
  Model identity;
  for (int p = 0; p < ModelAdapter<Model>::NumParameters(); ++p) {
    const float diff_p = fabs(ModelAdapter<Model>::GetParameter(model, p) -
                              ModelAdapter<Model>::GetParameter(identity, p));

    if (diff_p > 1e-6f) {
      return false;
    }
  }
  return true;
}

// Expresses input model M w.r.t. new domain given by LinearSimilarityTransform
// (scale  0      0
//  0      scale  0) := S -> S M S^(-1)
template <class Model>
Model CoordinateTransform(const Model& model, float scale) {
  return CoordinateTransform(
      model, ModelAdapter<LinearSimilarityModel>::FromArgs(0, 0, scale, 0));
}

// For model M and similarity S returns
// S * M * S^(-1).
template <class Model>
Model CoordinateTransform(const Model& model,
                          const LinearSimilarityModel& similarity) {
  return ModelCompose3(ModelAdapter<Model>::Embed(similarity), model,
                       ModelAdapter<Model>::Embed(ModelInvert(similarity)));
}

// Returns a model with all parameters set to value.
template <class Model>
Model UniformModelParameters(const float value) {
  std::array<float, ModelAdapter<Model>::NumParameters()> params;
  params.fill(value);
  return ModelAdapter<Model>::FromFloatPointer(params.data(), false);
}

// Returns a blended model: a * (1 - weight_b) + b * weight_b.
// Assumes 0 <= weight_b <= 1.
// Note that blending the homographies is a non-linear operation if the
// intention is to obtain a transform that blends the points transformed by
// a and b. However, this is a linear approximation, which ignores the
// perspective division, and simply blends the coefficients.
template <class Model>
Model BlendModels(const Model& a, const Model& b, float weight_b) {
  Model blended;
  ABSL_DCHECK_GE(weight_b, 0);
  ABSL_DCHECK_LE(weight_b, 1);
  const float weight_a = 1 - weight_b;
  for (int p = 0; p < ModelAdapter<Model>::NumParameters(); ++p) {
    const float pa = ModelAdapter<Model>::GetParameter(a, p);
    const float pb = ModelAdapter<Model>::GetParameter(b, p);
    ModelAdapter<Model>::SetParameter(p, pa * weight_a + pb * weight_b,
                                      &blended);
  }
  return blended;
}

template <class Model>
std::string ModelToString(const Model& model) {
  return ModelAdapter<Model>::ToString(model);
}

// Typedef's.
typedef ModelAdapter<TranslationModel> TranslationAdapter;
typedef ModelAdapter<SimilarityModel> SimilarityAdapter;
typedef ModelAdapter<LinearSimilarityModel> LinearSimilarityAdapter;
typedef ModelAdapter<AffineModel> AffineAdapter;
typedef ModelAdapter<Homography> HomographyAdapter;

typedef ModelMethods<TranslationModel> TranslationMethods;
typedef ModelMethods<SimilarityModel> SimilarityMethods;
typedef ModelMethods<LinearSimilarityModel> LinearSimilarityMethods;
typedef ModelMethods<AffineModel> AffineMethods;
typedef ModelMethods<Homography> HomographyMethods;

typedef MixtureModelAdapter<LinearSimilarityTraits>
    MixtureLinearSimilarityAdapter;
typedef MixtureModelAdapter<AffineTraits> MixtureAffineAdapter;
typedef MixtureModelAdapter<HomographyTraits> MixtureHomographyAdapter;

// Stores pre-computed normalized mixture weights.
// Weights are computed for each scanline,
// based on gaussian weighting of y-location to mid-points for each model
// (specified in scanlines!).
// We use even spacing between mid-points by default.
// By supplying a y_scale != 1.f, normalized coordinates can be used as input.
// Possible unnormalized y values for RowWeigts are
// [-margin, frame_height + margin).
class MixtureRowWeights {
 public:
  MixtureRowWeights(int frame_height, int margin, float sigma, float y_scale,
                    int num_models);
  int NumModels() const { return num_models_; }
  float YScale() const { return y_scale_; }
  float Sigma() const { return sigma_; }

  // Test if MixtureRowWeights should be re-initialized (call constructor
  // again), based on changed options.
  bool NeedsInitialization(int num_models, float sigma, float y_scale) const {
    return (num_models != num_models_ || fabs(sigma - sigma_) > 1e-6f ||
            fabs(y_scale - y_scale_) > 1e-6f);
  }

  const float* RowWeights(float y) const {
    int bin_y = y * y_scale_ + 0.5;
    ABSL_DCHECK_LT(bin_y, frame_height_ + margin_);
    ABSL_DCHECK_GE(bin_y, -margin_);
    return &weights_[(bin_y + margin_) * num_models_];
  }

  // Same as above but clamps parameter y to be within interval
  // (-margin, frame_height + margin).
  const float* RowWeightsClamped(float y) const {
    int bin_y = y * y_scale_ + 0.5;
    bin_y = std::max(-margin_, std::min(frame_height_ - 1 + margin_, bin_y));
    return &weights_[(bin_y + margin_) * num_models_];
  }

  // Returns weight threshold for fractional block distance, e.g.
  // parameter 1.5f returns row weight at 1.5f * block_height from block center.
  float WeightThreshold(float frac_blocks);

 private:
  int frame_height_;
  float y_scale_;
  int margin_;
  float sigma_;
  int num_models_;

  std::vector<int> mid_points_;
  std::vector<float> weights_;
};

// Returns pointer (caller takes ownership) of initialized MixtureRowWeights.
inline MixtureRowWeights* MixtureRowWeightsFromCameraMotion(
    const CameraMotion& camera_motion, int frame_height) {
  return new MixtureRowWeights(frame_height,
                               0,  // no margin.
                               camera_motion.mixture_row_sigma(), 1.0,
                               camera_motion.mixture_homography().model_size());
}

// Performs element wise smoothing of input models with per parameter sigma's
// in time (and optionally bilateral). Parameters of optional
// model_sigma that are NOT 0 are interpreted as bilateral smoothing sigma.
// Use UniformModelParameters to set all value of sigma_time to the same sigma.
template <class Model>
void SmoothModels(const Model& sigma_time_model, const Model* model_sigma,
                  std::vector<Model>* models) {
  ABSL_CHECK(models);

  const int num_models = models->size();

  std::vector<std::vector<float>> smoothed_model_data(num_models);

  for (int param = 0; param < ModelAdapter<Model>::NumParameters(); ++param) {
    const float sigma_time =
        ModelAdapter<Model>::GetParameter(sigma_time_model, param);

    if (sigma_time == 0) {
      // Don't perform any smoothing, just copy.
      for (int i = 0; i < num_models; ++i) {
        smoothed_model_data[i].push_back(
            ModelAdapter<Model>::GetParameter((*models)[i], param));
      }
      continue;
    }

    // Create lookup table for frame weights.
    const int frame_radius =
        std::min<int>(num_models - 1, std::ceil(sigma_time * 1.5f));
    const int frame_diameter = 2 * frame_radius + 1;

    // Create lookup table for weights.
    std::vector<float> frame_weights(frame_diameter);
    const float frame_coeff = -0.5f / (sigma_time * sigma_time);

    int frame_idx = 0;
    for (int i = -frame_radius; i <= frame_radius; ++i, ++frame_idx) {
      frame_weights[frame_idx] = std::exp(frame_coeff * i * i);
    }

    // Create local copy with border.
    std::vector<float> param_path(num_models + 2 * frame_radius);

    const float param_sigma =
        model_sigma != nullptr
            ? ModelAdapter<Model>::GetParameter(*model_sigma, param)
            : 0;
    const float param_sigma_denom =
        param_sigma != 0 ? (-0.5f / (param_sigma * param_sigma)) : 0;

    for (int model_idx = 0; model_idx < num_models; ++model_idx) {
      param_path[model_idx + frame_radius] =
          ModelAdapter<Model>::GetParameter((*models)[model_idx], param);
    }

    // Copy right.
    std::copy(param_path.rbegin() + frame_radius,
              param_path.rbegin() + 2 * frame_radius,
              param_path.end() - frame_radius);

    // Copy left.
    std::copy(param_path.begin() + frame_radius,
              param_path.begin() + 2 * frame_radius,
              param_path.rend() - frame_radius);

    // Apply filter.
    for (int i = 0; i < num_models; ++i) {
      float value_sum = 0;
      float weight_sum = 0;
      const float curr_value = param_path[i + frame_radius];

      for (int k = 0; k < frame_diameter; ++k) {
        const float value = param_path[i + k];
        float weight = frame_weights[k];
        if (param_sigma != 0) {
          // Bilateral filtering.
          const float value_diff = curr_value - value;
          weight *= std::exp(value_diff * value_diff * param_sigma_denom);
        }
        weight_sum += weight;
        value_sum += value * weight;
      }

      // Weight_sum is always > 0, as sigma is > 0.
      smoothed_model_data[i].push_back(value_sum / weight_sum);
    }
  }

  for (int i = 0; i < num_models; ++i) {
    (*models)[i].CopyFrom(ModelAdapter<Model>::FromFloatPointer(
        &smoothed_model_data[i][0], false));
  }
}

// Inline implementations.

// Translation model.
inline TranslationModel ModelAdapter<TranslationModel>::FromArgs(float dx,
                                                                 float dy) {
  TranslationModel model;
  model.set_dx(dx);
  model.set_dy(dy);
  return model;
}

inline TranslationModel ModelAdapter<TranslationModel>::FromFloatPointer(
    const float* args, bool) {
  ABSL_DCHECK(args);
  TranslationModel model;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  return model;
}

inline TranslationModel ModelAdapter<TranslationModel>::FromDoublePointer(
    const double* args, bool) {
  ABSL_DCHECK(args);
  TranslationModel model;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  return model;
}

inline Vector2_f ModelAdapter<TranslationModel>::TransformPoint(
    const TranslationModel& model, const Vector2_f& pt) {
  return Vector2_f(pt.x() + model.dx(), pt.y() + model.dy());
}

inline TranslationModel ModelAdapter<TranslationModel>::Invert(
    const TranslationModel& model) {
  bool success = true;
  TranslationModel result = InvertChecked(model, &success);
  if (!success) {
    ABSL_LOG(ERROR) << "Model not invertible. Returning identity.";
    return TranslationModel();
  }

  return result;
}

inline TranslationModel ModelAdapter<TranslationModel>::InvertChecked(
    const TranslationModel& model, bool* success) {
  TranslationModel inv_model;
  inv_model.set_dx(-model.dx());
  inv_model.set_dy(-model.dy());
  *success = true;
  return inv_model;
}

inline TranslationModel ModelAdapter<TranslationModel>::Compose(
    const TranslationModel& lhs, const TranslationModel& rhs) {
  TranslationModel result;
  result.set_dx(lhs.dx() + rhs.dx());
  result.set_dy(lhs.dy() + rhs.dy());
  return result;
}

inline float ModelAdapter<TranslationModel>::GetParameter(
    const TranslationModel& model, int id) {
  switch (id) {
    case 0:
      return model.dx();
    case 1:
      return model.dy();
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }
  return 0;
}

inline void ModelAdapter<TranslationModel>::SetParameter(
    int id, float value, TranslationModel* model) {
  switch (id) {
    case 0:
      return model->set_dx(value);
    case 1:
      return model->set_dy(value);
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }
}

// Linear Similarity model.
inline LinearSimilarityModel ModelAdapter<LinearSimilarityModel>::FromArgs(
    float dx, float dy, float a, float b) {
  LinearSimilarityModel model;
  model.set_dx(dx);
  model.set_dy(dy);
  model.set_a(a);
  model.set_b(b);
  return model;
}

inline LinearSimilarityModel
ModelAdapter<LinearSimilarityModel>::FromFloatPointer(
    const float* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  LinearSimilarityModel model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  model.set_a(id_shift + args[2]);
  model.set_b(args[3]);
  return model;
}

inline LinearSimilarityModel
ModelAdapter<LinearSimilarityModel>::FromDoublePointer(
    const double* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  LinearSimilarityModel model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  model.set_a(id_shift + args[2]);
  model.set_b(args[3]);
  return model;
}

inline Vector2_f ModelAdapter<LinearSimilarityModel>::TransformPoint(
    const LinearSimilarityModel& model, const Vector2_f& pt) {
  return Vector2_f(model.a() * pt.x() - model.b() * pt.y() + model.dx(),
                   model.b() * pt.x() + model.a() * pt.y() + model.dy());
}

inline LinearSimilarityModel ModelAdapter<LinearSimilarityModel>::Invert(
    const LinearSimilarityModel& model) {
  bool success = true;
  LinearSimilarityModel result = InvertChecked(model, &success);
  if (!success) {
    ABSL_LOG(ERROR) << "Model not invertible. Returning identity.";
    return LinearSimilarityModel();
  } else {
    return result;
  }
}

inline LinearSimilarityModel ModelAdapter<LinearSimilarityModel>::InvertChecked(
    const LinearSimilarityModel& model, bool* success) {
  LinearSimilarityModel inv_model;

  const float det = model.a() * model.a() + model.b() * model.b();
  if (fabs(det) < kDetInvertibleEps) {
    *success = false;
    VLOG(1) << "Model is not invertible, det is zero.";
    return LinearSimilarityModel();
  }

  *success = true;
  const float inv_det = 1.0 / det;

  inv_model.set_a(model.a() * inv_det);
  inv_model.set_b(-model.b() * inv_det);

  // Inverse translation is -A^(-1) * [dx dy].
  inv_model.set_dx(-(inv_model.a() * model.dx() - inv_model.b() * model.dy()));
  inv_model.set_dy(-(inv_model.b() * model.dx() + inv_model.a() * model.dy()));

  return inv_model;
}

inline LinearSimilarityModel ModelAdapter<LinearSimilarityModel>::Compose(
    const LinearSimilarityModel& lhs, const LinearSimilarityModel& rhs) {
  LinearSimilarityModel result;
  result.set_a(lhs.a() * rhs.a() - lhs.b() * rhs.b());
  result.set_b(lhs.a() * rhs.b() + lhs.b() * rhs.a());

  result.set_dx(lhs.a() * rhs.dx() - lhs.b() * rhs.dy() + lhs.dx());
  result.set_dy(lhs.b() * rhs.dx() + lhs.a() * rhs.dy() + lhs.dy());
  return result;
}

inline float ModelAdapter<LinearSimilarityModel>::GetParameter(
    const LinearSimilarityModel& model, int id) {
  switch (id) {
    case 0:
      return model.dx();
    case 1:
      return model.dy();
    case 2:
      return model.a();
    case 3:
      return model.b();
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }

  return 0;
}

inline void ModelAdapter<LinearSimilarityModel>::SetParameter(
    int id, float value, LinearSimilarityModel* model) {
  switch (id) {
    case 0:
      return model->set_dx(value);
    case 1:
      return model->set_dy(value);
    case 2:
      return model->set_a(value);
    case 3:
      return model->set_b(value);
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }
}

// Affine model.
inline AffineModel ModelAdapter<AffineModel>::FromArgs(float dx, float dy,
                                                       float a, float b,
                                                       float c, float d) {
  AffineModel model;
  model.set_dx(dx);
  model.set_dy(dy);
  model.set_a(a);
  model.set_b(b);
  model.set_c(c);
  model.set_d(d);
  return model;
}

inline AffineModel ModelAdapter<AffineModel>::FromFloatPointer(
    const float* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  AffineModel model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  model.set_a(id_shift + args[2]);
  model.set_b(args[3]);
  model.set_c(args[4]);
  model.set_d(id_shift + args[5]);
  return model;
}

inline AffineModel ModelAdapter<AffineModel>::FromDoublePointer(
    const double* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  AffineModel model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_dx(args[0]);
  model.set_dy(args[1]);
  model.set_a(id_shift + args[2]);
  model.set_b(args[3]);
  model.set_c(args[4]);
  model.set_d(id_shift + args[5]);
  return model;
}

inline Vector2_f ModelAdapter<AffineModel>::TransformPoint(
    const AffineModel& model, const Vector2_f& pt) {
  return Vector2_f(model.a() * pt.x() + model.b() * pt.y() + model.dx(),
                   model.c() * pt.x() + model.d() * pt.y() + model.dy());
}

// Use of Invert is discouraged, always use InvertChecked.
inline AffineModel ModelAdapter<AffineModel>::Invert(const AffineModel& model) {
  bool success = true;
  AffineModel result = InvertChecked(model, &success);
  if (!success) {
    ABSL_LOG(ERROR) << "Model not invertible. Returning identity.";
    return AffineModel();
  } else {
    return result;
  }
}

inline AffineModel ModelAdapter<AffineModel>::InvertChecked(
    const AffineModel& model, bool* success) {
  AffineModel inv_model;
  const float det = model.a() * model.d() - model.b() * model.c();
  if (fabs(det) < kDetInvertibleEps) {
    *success = false;
    VLOG(1) << "Model is not invertible, det is zero.";
    return AffineModel();
  }

  *success = true;
  const float inv_det = 1.0 / det;

  inv_model.set_a(model.d() * inv_det);
  inv_model.set_d(model.a() * inv_det);
  inv_model.set_c(-model.c() * inv_det);
  inv_model.set_b(-model.b() * inv_det);

  // Inverse translation is -A^(-1) * [dx dy].
  inv_model.set_dx(-(inv_model.a() * model.dx() + inv_model.b() * model.dy()));
  inv_model.set_dy(-(inv_model.c() * model.dx() + inv_model.d() * model.dy()));

  return inv_model;
}

inline AffineModel ModelAdapter<AffineModel>::Compose(const AffineModel& lhs,
                                                      const AffineModel& rhs) {
  AffineModel result;
  result.set_a(lhs.a() * rhs.a() + lhs.b() * rhs.c());
  result.set_b(lhs.a() * rhs.b() + lhs.b() * rhs.d());
  result.set_c(lhs.c() * rhs.a() + lhs.d() * rhs.c());
  result.set_d(lhs.c() * rhs.b() + lhs.d() * rhs.d());

  result.set_dx(lhs.a() * rhs.dx() + lhs.b() * rhs.dy() + lhs.dx());
  result.set_dy(lhs.c() * rhs.dx() + lhs.d() * rhs.dy() + lhs.dy());
  return result;
}

inline float ModelAdapter<AffineModel>::GetParameter(const AffineModel& model,
                                                     int id) {
  switch (id) {
    case 0:
      return model.dx();
    case 1:
      return model.dy();
    case 2:
      return model.a();
    case 3:
      return model.b();
    case 4:
      return model.c();
    case 5:
      return model.d();
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }

  return 0;
}

inline void ModelAdapter<AffineModel>::SetParameter(int id, float value,
                                                    AffineModel* model) {
  switch (id) {
    case 0:
      return model->set_dx(value);
    case 1:
      return model->set_dy(value);
    case 2:
      return model->set_a(value);
    case 3:
      return model->set_b(value);
    case 4:
      return model->set_c(value);
    case 5:
      return model->set_d(value);
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }
}

// Homography model.
inline Homography ModelAdapter<Homography>::FromArgs(float h_00, float h_01,
                                                     float h_02, float h_10,
                                                     float h_11, float h_12,
                                                     float h_20, float h_21) {
  Homography model;
  model.set_h_00(h_00);
  model.set_h_01(h_01);
  model.set_h_02(h_02);
  model.set_h_10(h_10);
  model.set_h_11(h_11);
  model.set_h_12(h_12);
  model.set_h_20(h_20);
  model.set_h_21(h_21);
  return model;
}

inline Homography ModelAdapter<Homography>::FromFloatPointer(
    const float* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  Homography model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_h_00(id_shift + args[0]);
  model.set_h_01(args[1]);
  model.set_h_02(args[2]);
  model.set_h_10(args[3]);
  model.set_h_11(id_shift + args[4]);
  model.set_h_12(args[5]);
  model.set_h_20(args[6]);
  model.set_h_21(args[7]);
  return model;
}

inline Homography ModelAdapter<Homography>::FromDoublePointer(
    const double* args, bool identity_parametrization) {
  ABSL_DCHECK(args);
  Homography model;
  const float id_shift = identity_parametrization ? 1.f : 0.f;
  model.set_h_00(id_shift + args[0]);
  model.set_h_01(args[1]);
  model.set_h_02(args[2]);
  model.set_h_10(args[3]);
  model.set_h_11(id_shift + args[4]);
  model.set_h_12(args[5]);
  model.set_h_20(args[6]);
  model.set_h_21(args[7]);
  return model;
}

inline Vector2_f ModelAdapter<Homography>::TransformPoint(
    const Homography& model, const Vector2_f& pt) {
  const float x = model.h_00() * pt.x() + model.h_01() * pt.y() + model.h_02();
  const float y = model.h_10() * pt.x() + model.h_11() * pt.y() + model.h_12();
  float z = model.h_20() * pt.x() + model.h_21() * pt.y() + 1.0f;

  if (z != 1.f) {
    // Enforce z can not assume very small values.
    constexpr float eps = 1e-12f;
    if (fabs(z) < eps) {
      ABSL_LOG(ERROR) << "Point mapped to infinity. "
                      << "Degenerate homography. See proto.";
      z = z >= 0 ? eps : -eps;
    }
    return Vector2_f(x / z, y / z);
  } else {
    return Vector2_f(x, y);
  }
}

inline Vector3_f ModelAdapter<Homography>::TransformPoint3(
    const Homography& model, const Vector3_f& pt) {
  return Vector3_f(
      model.h_00() * pt.x() + model.h_01() * pt.y() + model.h_02() * pt.z(),
      model.h_10() * pt.x() + model.h_11() * pt.y() + model.h_12() * pt.z(),
      model.h_20() * pt.x() + model.h_21() * pt.y() + pt.z());
}

inline Homography ModelAdapter<Homography>::Invert(const Homography& model) {
  bool success = true;
  Homography result = InvertChecked(model, &success);
  if (!success) {
    ABSL_LOG(ERROR) << "Model not invertible. Returning identity.";
    return Homography();
  } else {
    return result;
  }
}

inline Homography ModelAdapter<Homography>::Compose(const Homography& lhs,
                                                    const Homography& rhs) {
  Homography result;
  const float z =
      lhs.h_20() * rhs.h_02() + lhs.h_21() * rhs.h_12() + 1.0f * 1.0f;
  ABSL_CHECK_NE(z, 0) << "Degenerate homography. See proto.";
  const float inv_z = 1.0 / z;

  result.set_h_00((lhs.h_00() * rhs.h_00() + lhs.h_01() * rhs.h_10() +
                   lhs.h_02() * rhs.h_20()) *
                  inv_z);
  result.set_h_01((lhs.h_00() * rhs.h_01() + lhs.h_01() * rhs.h_11() +
                   lhs.h_02() * rhs.h_21()) *
                  inv_z);
  result.set_h_02(
      (lhs.h_00() * rhs.h_02() + lhs.h_01() * rhs.h_12() + lhs.h_02() * 1.0f) *
      inv_z);

  result.set_h_10((lhs.h_10() * rhs.h_00() + lhs.h_11() * rhs.h_10() +
                   lhs.h_12() * rhs.h_20()) *
                  inv_z);
  result.set_h_11((lhs.h_10() * rhs.h_01() + lhs.h_11() * rhs.h_11() +
                   lhs.h_12() * rhs.h_21()) *
                  inv_z);
  result.set_h_12(
      (lhs.h_10() * rhs.h_02() + lhs.h_11() * rhs.h_12() + lhs.h_12() * 1.0f) *
      inv_z);

  result.set_h_20(
      (lhs.h_20() * rhs.h_00() + lhs.h_21() * rhs.h_10() + 1.0f * rhs.h_20()) *
      inv_z);
  result.set_h_21(
      (lhs.h_20() * rhs.h_01() + lhs.h_21() * rhs.h_11() + 1.f * rhs.h_21()) *
      inv_z);
  return result;
}

inline float ModelAdapter<Homography>::GetParameter(const Homography& model,
                                                    int id) {
  switch (id) {
    case 0:
      return model.h_00();
    case 1:
      return model.h_01();
    case 2:
      return model.h_02();
    case 3:
      return model.h_10();
    case 4:
      return model.h_11();
    case 5:
      return model.h_12();
    case 6:
      return model.h_20();
    case 7:
      return model.h_21();
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }

  return 0;
}

inline void ModelAdapter<Homography>::SetParameter(int id, float value,
                                                   Homography* model) {
  switch (id) {
    case 0:
      return model->set_h_00(value);
    case 1:
      return model->set_h_01(value);
    case 2:
      return model->set_h_02(value);
    case 3:
      return model->set_h_10(value);
    case 4:
      return model->set_h_11(value);
    case 5:
      return model->set_h_12(value);
    case 6:
      return model->set_h_20(value);
    case 7:
      return model->set_h_21(value);
    default:
      ABSL_LOG(FATAL) << "Parameter id is out of bounds";
  }
}

// MixtureModelAdapterBase implementation.
template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::FromFloatPointer(
    const float* args, bool identity_parametrization, int skip,
    int num_models) {
  MixtureModel model;
  const float* arg_ptr = args;
  for (int i = 0; i < num_models;
       ++i, arg_ptr += BaseModelAdapter::NumParameters() + skip) {
    BaseModel base =
        BaseModelAdapter::FromFloatPointer(arg_ptr, identity_parametrization);
    model.add_model()->CopyFrom(base);
  }

  return model;
}

template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::FromDoublePointer(
    const double* args, bool identity_parametrization, int skip,
    int num_models) {
  MixtureModel model;
  const double* arg_ptr = args;
  for (int i = 0; i < num_models;
       ++i, arg_ptr += BaseModelAdapter::NumParameters() + skip) {
    BaseModel base =
        BaseModelAdapter::FromDoublePointer(arg_ptr, identity_parametrization);
    model.add_model()->CopyFrom(base);
  }

  return model;
}

template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::ComposeRight(
    const MixtureModel& mixture_model, const BaseModel& base_model) {
  const int num_models = mixture_model.model_size();
  MixtureModel result;
  for (int m = 0; m < num_models; ++m) {
    result.add_model()->CopyFrom(
        BaseModelAdapter::Compose(mixture_model.model(m), base_model));
  }
  return result;
}

template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::ComposeLeft(
    const MixtureModel& mixture_model, const BaseModel& base_model) {
  const int num_models = mixture_model.model_size();
  MixtureModel result;
  for (int m = 0; m < num_models; ++m) {
    result.add_model()->CopyFrom(
        BaseModelAdapter::Compose(base_model, mixture_model.model(m)));
  }
  return result;
}

template <class MixtureTraits>
std::string MixtureModelAdapterBase<MixtureTraits>::ToString(
    const MixtureModel& model, std::string delim) {
  std::string result = "";
  for (int m = 0, size = model.model_size(); m < size; ++m) {
    result +=
        (m == 0 ? "" : delim) + BaseModelAdapter::ToString(model.model(m));
  }
  return result;
}

template <class MixtureTraits>
float MixtureModelAdapterBase<MixtureTraits>::GetParameter(
    const MixtureModel& model, int model_id, int param_id) {
  return BaseModelAdapter::GetParameter(model.model(model_id), param_id);
}

template <class MixtureTraits>
void MixtureModelAdapterBase<MixtureTraits>::SetParameter(int model_id,
                                                          int param_id,
                                                          float value,
                                                          MixtureModel* model) {
  BaseModelAdapter::SetParameter(param_id, value,
                                 model->mutable_model(model_id));
}

template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::IdentityModel(int num_mixtures) {
  MixtureModel model;
  for (int i = 0; i < num_mixtures; ++i) {
    model.add_model();
  }
  return model;
}

template <class MixtureTraits>
typename MixtureTraits::base_model
MixtureModelAdapterBase<MixtureTraits>::MeanModel(
    const MixtureModel& mixture_model) {
  const int num_models = mixture_model.model_size();
  if (num_models == 0) {
    return BaseModel();
  }

  float params[BaseModelAdapter::NumParameters()];
  memset(params, 0, sizeof(params[0]) * BaseModelAdapter::NumParameters());

  // Average of models.
  const float denom = 1.0f / num_models;
  for (int k = 0; k < BaseModelAdapter::NumParameters(); ++k) {
    for (int m = 0; m < num_models; ++m) {
      params[k] += BaseModelAdapter::GetParameter(mixture_model.model(m), k);
    }
    params[k] *= denom;
  }

  return BaseModelAdapter::FromFloatPointer(params, false);
}

template <class MixtureTraits>
typename MixtureTraits::model
MixtureModelAdapterBase<MixtureTraits>::LinearModel(
    const MixtureModel& mixture_model) {
  // For each parameter: Fit line param_idx -> param value.
  const int num_models = mixture_model.model_size();
  if (num_models <= 1) {
    return mixture_model;
  }

  std::vector<float> result(num_models * BaseModelAdapter::NumParameters());
  const double inv_models = 1.0f / num_models;
  for (int p = 0; p < BaseModelAdapter::NumParameters(); ++p) {
    // Calculate sum, sq_sum and inner product.
    double sum_x = 0.0;
    double sum_y = 0.0;
    double sum_xx = 0.0;
    double sum_yy = 0.0;
    double sum_xy = 0.0;
    for (int m = 0; m < num_models; ++m) {
      const float x = m * inv_models;
      sum_x += x;
      sum_xx += x * x;
      const double y = GetParameter(mixture_model, m, p);
      sum_y += y;
      sum_yy += y * y;
      sum_xy += x * y;
    }

    const double denom = sum_xx - inv_models * sum_x * sum_x;
    ABSL_CHECK_NE(denom, 0);  // As num_models > 1.
    const double a = (sum_xy - inv_models * sum_x * sum_y) * denom;
    const double b = inv_models * (sum_y - a * sum_x);

    for (int m = 0; m < num_models; ++m) {
      const float x = m * inv_models;
      result[m * BaseModelAdapter::NumParameters() + p] = a * x + b;
    }
  }
  MixtureModel result_model =
      FromFloatPointer(&result[0], false, 0, num_models);

  return result_model;
}

template <class MixtureTraits>
typename MixtureTraits::model MixtureModelAdapterBase<MixtureTraits>::Embed(
    const BaseModel& base_model, int num_mixtures) {
  MixtureModel model;
  for (int i = 0; i < num_mixtures; ++i) {
    model.add_model()->CopyFrom(base_model);
  }
  return model;
}

// MixtureModelAdapter implementation.
template <class MixtureTraits>
typename MixtureModelAdapterBase<MixtureTraits>::BaseModel
MixtureModelAdapter<MixtureTraits>::ToBaseModel(
    const MixtureModel& mixture_model, const float* weights) {
  const int num_models = mixture_model.model_size();

  float params[BaseModelAdapter::NumParameters()];
  memset(params, 0, sizeof(params[0]) * BaseModelAdapter::NumParameters());

  // Weighted combination of mixture models.
  for (int m = 0; m < num_models; ++m) {
    for (int k = 0; k < BaseModelAdapter::NumParameters(); ++k) {
      params[k] += BaseModelAdapter::GetParameter(mixture_model.model(m), k) *
                   weights[m];
    }
  }

  return BaseModelAdapter::FromFloatPointer(params, false);
}

template <class MixtureTraits>
Vector2_f MixtureModelAdapter<MixtureTraits>::TransformPoint(
    const MixtureModel& model, const float* weights, const Vector2_f& pt) {
  const int num_models = model.model_size();
  const Vector3_f pt3(pt.x(), pt.y(), 1.0f);
  Vector3_f result(0, 0, 0);
  for (int i = 0; i < num_models; ++i) {
    result +=
        BaseModelAdapter::TransformPoint3(model.model(i), pt3 * weights[i]);
  }

  ABSL_DCHECK_NE(result.z(), 0) << "Degenerate mapping.";
  return Vector2_f(result.x() / result.z(), result.y() / result.z());
}

template <class MixtureTraits>
Vector2_f MixtureModelAdapter<MixtureTraits>::TransformPoint(
    const MixtureModel& model, const MixtureRowWeights& weights,
    const Vector2_f& pt) {
  return TransformPoint(model, weights.RowWeightsClamped(pt.y()), pt);
}

template <class MixtureTraits>
Vector2_f MixtureModelAdapter<MixtureTraits>::SolveForPoint(
    const MixtureModel& model, const float* weights, const Vector2_f& pt) {
  BaseModel base_model = ToBaseModel(model, weights);
  return BaseModelAdapter::TransformPoint(BaseModelAdapter::Invert(base_model),
                                          pt);
}

template <class MixtureTraits>
Vector2_f MixtureModelAdapter<MixtureTraits>::SolveForPointChecked(
    const MixtureModel& model, const float* weights, const Vector2_f& pt,
    bool* success) {
  BaseModel base_model = ToBaseModel(model, weights);
  BaseModel inv_base_model =
      BaseModelAdapter::InvertChecked(base_model, success);
  return BaseModelAdapter::TransformPoint(inv_base_model, pt);
}

// MixtureModelAdapter<Homography> implementation.
inline Homography MixtureModelAdapter<HomographyTraits>::ToBaseModel(
    const MixtureHomography& mixture_model, const float* weights) {
  const int num_models = mixture_model.model_size();

  float params[8] = {0.f, 0.f, 0.f, 0.f, 0.f, 0.f, 0.f, 0.f};
  const Homography& const_homog = mixture_model.model(0);

  // Weighted combination of mixture models.
  switch (mixture_model.dof()) {
    case MixtureHomography::ALL_DOF:
      for (int m = 0; m < num_models; ++m) {
        params[0] += mixture_model.model(m).h_00() * weights[m];
        params[1] += mixture_model.model(m).h_01() * weights[m];
        params[2] += mixture_model.model(m).h_02() * weights[m];
        params[3] += mixture_model.model(m).h_10() * weights[m];
        params[4] += mixture_model.model(m).h_11() * weights[m];
        params[5] += mixture_model.model(m).h_12() * weights[m];
        params[6] += mixture_model.model(m).h_20() * weights[m];
        params[7] += mixture_model.model(m).h_21() * weights[m];
      }
      break;
    case MixtureHomography::TRANSLATION_DOF:
      params[0] = const_homog.h_00();
      params[1] = const_homog.h_01();
      params[3] = const_homog.h_10();
      params[4] = const_homog.h_11();
      params[6] = const_homog.h_20();
      params[7] = const_homog.h_21();

      for (int m = 0; m < num_models; ++m) {
        params[2] += mixture_model.model(m).h_02() * weights[m];
        params[5] += mixture_model.model(m).h_12() * weights[m];
      }
      break;
    case MixtureHomography::SKEW_ROTATION_DOF:
      params[0] = const_homog.h_00();
      params[4] = const_homog.h_11();
      params[6] = const_homog.h_20();
      params[7] = const_homog.h_21();
      for (int m = 0; m < num_models; ++m) {
        params[1] += mixture_model.model(m).h_01() * weights[m];
        params[2] += mixture_model.model(m).h_02() * weights[m];
        params[3] += mixture_model.model(m).h_10() * weights[m];
        params[5] += mixture_model.model(m).h_12() * weights[m];
      }
      break;
    case MixtureHomography::CONST_DOF:
      return const_homog;
    default:
      ABSL_LOG(FATAL) << "Unknown type.";
  }

  return HomographyAdapter::FromFloatPointer(params, false);
}

inline Vector2_f MixtureModelAdapter<HomographyTraits>::TransformPoint(
    const MixtureHomography& model, const float* weights, const Vector2_f& pt) {
  const int num_models = model.model_size();
  const Homography& const_homog = model.model(0);
  Vector3_f result(0, 0, 0);
  const Vector3_f pt3(pt.x(), pt.y(), 1.0f);
  float x;
  float y;
  switch (model.dof()) {
    case MixtureHomography::ALL_DOF:
      for (int i = 0; i < num_models; ++i) {
        result += HomographyAdapter::TransformPoint3(model.model(i),
                                                     pt3 * weights[i]);
      }
      break;
    case MixtureHomography::TRANSLATION_DOF:
      x = const_homog.h_00() * pt.x() + const_homog.h_01() * pt.y();
      y = const_homog.h_10() * pt.x() + const_homog.h_11() * pt.y();
      for (int i = 0; i < num_models; ++i) {
        x += model.model(i).h_02() * weights[i];
        y += model.model(i).h_12() * weights[i];
      }
      result = Vector3_f(
          x, y,
          const_homog.h_20() * pt.x() + const_homog.h_21() * pt.y() + 1.0f);
      break;
    case MixtureHomography::SKEW_ROTATION_DOF:
      x = const_homog.h_00() * pt.x();
      y = const_homog.h_11() * pt.y();
      for (int i = 0; i < num_models; ++i) {
        x += (model.model(i).h_01() * pt.y() + model.model(i).h_02()) *
             weights[i];
        y += (model.model(i).h_10() * pt.x() + model.model(i).h_12()) *
             weights[i];
      }
      result = Vector3_f(
          x, y,
          const_homog.h_20() * pt.x() + const_homog.h_21() * pt.y() + 1.0f);
      break;
    case MixtureHomography::CONST_DOF:
      return HomographyAdapter::TransformPoint(model.model(0), pt);
    default:
      ABSL_LOG(FATAL) << "Unknown type.";
  }

  ABSL_DCHECK_NE(result.z(), 0) << "Degenerate mapping.";
  return Vector2_f(result.x() / result.z(), result.y() / result.z());
}

inline Vector2_f MixtureModelAdapter<HomographyTraits>::TransformPoint(
    const MixtureHomography& model, const MixtureRowWeights& weights,
    const Vector2_f& pt) {
  return TransformPoint(model, weights.RowWeightsClamped(pt.y()), pt);
}

inline Vector2_f MixtureModelAdapter<HomographyTraits>::SolveForPoint(
    const MixtureHomography& model, const float* weights, const Vector2_f& pt) {
  Homography base_model = ToBaseModel(model, weights);
  return HomographyAdapter::TransformPoint(
      HomographyAdapter::Invert(base_model), pt);
}

inline Vector2_f MixtureModelAdapter<HomographyTraits>::SolveForPointChecked(
    const MixtureHomography& model, const float* weights, const Vector2_f& pt,
    bool* success) {
  Homography base_model = ToBaseModel(model, weights);
  Homography inv_base_model =
      HomographyAdapter::InvertChecked(base_model, success);
  return HomographyAdapter::TransformPoint(inv_base_model, pt);
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_MOTION_MODELS_H_
