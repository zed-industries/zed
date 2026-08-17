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
// Implements various tone-models describing tonal change of color intensities
// across frame pairs.

#ifndef MEDIAPIPE_UTIL_TRACKING_TONE_MODELS_H_
#define MEDIAPIPE_UTIL_TRACKING_TONE_MODELS_H_

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <string>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/tone_models.pb.h"

namespace mediapipe {
// Abstract Adapter for tone models.
template <class Model>
class ToneModelAdapter {
 public:
  // Initialized Model from pointer to arguments. If identitiy_parametrization
  // is set, args = {0,0, ... 0} correspond to identity model.
  template <class T>
  static Model FromPointer(const T* args, bool identity_parametrization);

  // Outputs parameters to pointer args.
  template <class T>
  static void ToPointer(const GainBiasModel& model, T* args);

  // Transforms color (RGB space) according to model.
  static Vector3_f TransformColor(const Model& model, const Vector3_f& color);

  // Inverts color mode, if not invertible success is set to zero.
  static Model InvertChecked(const Model& model, bool* success);
  static Model Compose(const Model& lhs, const Model& rhs);

  static int NumParameters();
  static float GetParameter(const Model& model, int idx);

  static float Determinant(const Model& model);
};

// LogDomain LUT:
// Converts from intensity domain to log-domain via Map, inverse mapping via
// UnMap.
// Use as Singleton only, via inline LogDomainLUT()
class LogDomainLUTImpl {
 public:
  LogDomainLUTImpl(const LogDomainLUTImpl&) = delete;
  LogDomainLUTImpl& operator=(const LogDomainLUTImpl&) = delete;

  // Maps intensity between [0, 255] to log-domain by applying log(1 + x).
  // Result will be within [0, log(256)]. Truncation occurs for non-integer
  // inputs. Discritization error is at most 0.5 intensity levels.
  inline float Map(float value) const {
    return log_lut_[std::max(0, std::min<int>(255, value + 0.5f))];
  }

  inline Vector3_f MapVec(Vector3_f vec) const {
    return Vector3_f(Map(vec.x()), Map(vec.y()), Map(vec.z()));
  }

  // Unmaps intensity from log-domain in [0, log(256)] = [0, 2.4] to regular
  // domain, via exp(x) - 1.0. Discritization error of at most 0.01 intensity
  // levels might occur.
  inline float UnMap(float value) const {
    const int bin = value * inv_max_log_value_ * (kExpBins - 2);
    return exp_lut_[std::max(0, std::min(bin, kExpBins - 1))];
  }

  inline Vector3_f UnMapVec(Vector3_f vec) const {
    return Vector3_f(UnMap(vec.x()), UnMap(vec.y()), UnMap(vec.z()));
  }

  float MaxLogDomainValue() const { return max_log_value_; }  // ~ log(256).

 private:
  friend const LogDomainLUTImpl& LogDomainLUT();

  LogDomainLUTImpl();
  std::vector<float> log_lut_;
  std::vector<float> exp_lut_;
  float max_log_value_;
  float inv_max_log_value_;
  enum {
    kExpBins = 2560
  };  // Allocated such that exp(2.4) -
      // exp(2.4 * (1.0 - 1.0 / kExpBins)) < 0.01.
};

inline const LogDomainLUTImpl& LogDomainLUT() {
  static auto p = new LogDomainLUTImpl();
  return *p;
}

inline Vector3_i RoundAndClampColor(const Vector3_f& vec) {
  return Vector3_i(
      std::max(0, std::min(255, static_cast<int>(vec.x() + 0.5f))),
      std::max(0, std::min(255, static_cast<int>(vec.y() + 0.5f))),
      std::max(0, std::min(255, static_cast<int>(vec.z() + 0.5f))));
}

template <>
class ToneModelAdapter<GainBiasModel> {
 public:
  template <class T>
  static GainBiasModel FromPointer(const T* args,
                                   bool identity_parametrization);
  template <class T>
  static void ToPointer(const GainBiasModel& model, T* args);

  inline static Vector3_f TransformColor(const GainBiasModel& model,
                                         const Vector3_f& color);

  inline static GainBiasModel InvertChecked(const GainBiasModel& model,
                                            bool* success);

  inline static GainBiasModel Compose(const GainBiasModel& lhs,
                                      const GainBiasModel& rhs);

  enum { kNumParameters = 6 };
  static int NumParameters() { return kNumParameters; }
  inline static float GetParameter(const GainBiasModel& model, int idx);

  static float Determinant(const GainBiasModel& model);

  static GainBiasModel AddIdentity(const GainBiasModel& model);
  static GainBiasModel ScaleParameters(const GainBiasModel& model, float scale);

  static std::string ToString(const GainBiasModel& model);
};

template <>
class ToneModelAdapter<AffineToneModel> {
 public:
  template <class T>
  static AffineToneModel FromPointer(const T* args,
                                     bool identity_parametrization);
  template <class T>
  static void ToPointer(const AffineToneModel& model, T* args) {
    return ToPointerPad(model, false, args);
  }

  // If pad_square is set the 12 DOF 3x4 model is embedded in a 4x4 model
  // (last row = [ 0 0 0 1]) and output to args (row major ordering),
  // otherwise the original 12 DOF 3x4 model is written to args (row major).
  template <class T>
  static void ToPointerPad(const AffineToneModel& model, bool pad_square,
                           T* args);

  inline static Vector3_f TransformColor(const AffineToneModel& model,
                                         const Vector3_f& color);

  inline static AffineToneModel InvertChecked(const AffineToneModel& model,
                                              bool* success);

  inline static AffineToneModel Compose(const AffineToneModel& lhs,
                                        const AffineToneModel& rhs);

  enum { kNumParameters = 12 };
  static int NumParameters() { return kNumParameters; }
  inline static float GetParameter(const AffineToneModel& model, int idx);

  // Used during stabilization: Adds identity model (model + I)
  // and scales each parameter by scale.
  static AffineToneModel AddIdentity(const AffineToneModel& model);
  static AffineToneModel ScaleParameters(const AffineToneModel& model,
                                         float scale);
};

typedef ToneModelAdapter<GainBiasModel> GainBiasModelAdapter;
typedef ToneModelAdapter<AffineToneModel> AffineToneModelAdapter;

// Mixture Models.
template <class BaseModel, class MixtureModel>
struct MixtureToneTraits {
  typedef BaseModel BaseModelType;
  typedef MixtureModel ModelType;
};

typedef MixtureToneTraits<GainBiasModel, MixtureGainBiasModel>
    GainBiasModelTraits;
typedef MixtureToneTraits<AffineToneModel, MixtureAffineToneModel>
    AffineToneModelTraits;

template <class Traits>
class MixtureToneAdapter {
  typedef typename Traits::ModelType MixtureModel;
  typedef typename Traits::BaseModelType BaseModel;
  typedef ToneModelAdapter<BaseModel> BaseModelAdapter;
  enum { kBaseNumParameters = BaseModelAdapter::kNumParameters };

 public:
  template <class T>
  static MixtureModel FromPointer(const T* args, bool identity_parametrization,
                                  int skip, int num_models);

  template <class T>
  static void ToPointer(const MixtureModel& model, T* args);

  static int NumParameters(const MixtureModel& model) {
    return model.model_size() * kBaseNumParameters;
  }

  inline static float GetParameter(const MixtureModel& model, int model_id,
                                   int param_id);

  inline static MixtureModel IdentityModel(int num_mixtures);

  inline static BaseModel ToBaseModel(const MixtureModel& mixture_model,
                                      const float* weights);

  inline static Vector3_f TransformColor(const MixtureModel& model,
                                         const float* weights,
                                         const Vector3_f& pt);

  // Mixtures are not invertible via closed form, but invertible for a specific
  // set of weights by computing the underlying BaseModel via ToBaseModel.
  // Resulting BaseModel is applied to pt, if BaseModel is not invertible
  // success is set to false.
  inline static Vector3_f SolveForColorChecked(const MixtureModel& model,
                                               const float* weights,
                                               const Vector3_f& pt,
                                               bool* success);
};

typedef MixtureToneAdapter<GainBiasModelTraits> MixtureGainBiasModelAdapter;
typedef MixtureToneAdapter<AffineToneModelTraits> MixtureAffineToneModelAdapter;

template <class T>
GainBiasModel ToneModelAdapter<GainBiasModel>::FromPointer(const T* args,
                                                           bool identity) {
  ABSL_DCHECK(args);
  GainBiasModel model;
  const float id_shift = identity ? 1.0f : 0.0f;
  model.set_gain_c1(args[0] + id_shift);
  model.set_bias_c1(args[1]);
  model.set_gain_c2(args[2] + id_shift);
  model.set_bias_c2(args[3]);
  model.set_gain_c3(args[4] + id_shift);
  model.set_bias_c3(args[5]);
  return model;
}

template <class T>
void ToneModelAdapter<GainBiasModel>::ToPointer(const GainBiasModel& model,
                                                T* args) {
  args[0] = model.gain_c1();
  args[1] = model.bias_c1();
  args[2] = model.gain_c2();
  args[3] = model.bias_c2();
  args[4] = model.gain_c3();
  args[5] = model.bias_c3();
}

inline Vector3_f ToneModelAdapter<GainBiasModel>::TransformColor(
    const GainBiasModel& model, const Vector3_f& color) {
  return Vector3_f(model.gain_c1() * color.x() + model.bias_c1(),
                   model.gain_c2() * color.y() + model.bias_c2(),
                   model.gain_c3() * color.z() + model.bias_c3());
}

inline float ToneModelAdapter<GainBiasModel>::Determinant(
    const GainBiasModel& model) {
  return model.gain_c1() * model.gain_c2() * model.gain_c3();
}

inline GainBiasModel ToneModelAdapter<GainBiasModel>::InvertChecked(
    const GainBiasModel& model, bool* success) {
  // (g_1  0   0   b_1       ==  (1/g_1  0      0       -b_1/g_1
  //    0  g_2 0   b_2             0    1/g_2   0       -b_2/g_2
  //    0  0   g_3 b_3             0    0     1/g_3     -b_3/g_3
  //    0  0   0   1)^(-1)         0    0       0        1

  // Compute determinant.
  const float det = GainBiasModelAdapter::Determinant(model);
  if (fabs(det) < 1e-10f) {
    *success = false;
    ABSL_LOG(ERROR) << "Model not invertible.";
    return GainBiasModel();
  }

  *success = true;
  GainBiasModel result;
  const float inv_gain_c1 = 1.0f / model.gain_c1();
  const float inv_gain_c2 = 1.0f / model.gain_c2();
  const float inv_gain_c3 = 1.0f / model.gain_c3();
  result.set_gain_c1(inv_gain_c1);
  result.set_bias_c1(inv_gain_c1 * -model.bias_c1());
  result.set_gain_c2(inv_gain_c2);
  result.set_bias_c2(inv_gain_c2 * -model.bias_c2());
  result.set_gain_c3(inv_gain_c3);
  result.set_bias_c3(inv_gain_c3 * -model.bias_c3());
  return result;
}

inline GainBiasModel ToneModelAdapter<GainBiasModel>::Compose(
    const GainBiasModel& lhs, const GainBiasModel& rhs) {
  GainBiasModel result;
  result.set_gain_c1(lhs.gain_c1() * rhs.gain_c1());
  result.set_bias_c1(lhs.gain_c1() * rhs.bias_c1() + lhs.bias_c1());
  result.set_gain_c2(lhs.gain_c2() * rhs.gain_c2());
  result.set_bias_c2(lhs.gain_c2() * rhs.bias_c2() + lhs.bias_c2());
  result.set_gain_c3(lhs.gain_c3() * rhs.gain_c3());
  result.set_bias_c3(lhs.gain_c3() * rhs.bias_c3() + lhs.bias_c3());
  return result;
}

inline float ToneModelAdapter<GainBiasModel>::GetParameter(
    const GainBiasModel& model, int idx) {
  switch (idx) {
    case 0:
      return model.gain_c1();
    case 1:
      return model.bias_c1();
    case 2:
      return model.gain_c2();
    case 3:
      return model.bias_c2();
    case 4:
      return model.gain_c3();
    case 5:
      return model.bias_c3();
    default:
      ABSL_LOG(FATAL) << "Unknown parameter requested.";
  }

  return 0.0f;
}
template <class T>
AffineToneModel ToneModelAdapter<AffineToneModel>::FromPointer(const T* args,
                                                               bool identity) {
  ABSL_DCHECK(args);
  AffineToneModel model;
  const float id_shift = identity ? 1.0f : 0.0f;
  model.set_g_00(args[0] + id_shift);
  model.set_g_01(args[1]);
  model.set_g_02(args[2]);
  model.set_g_03(args[3]);

  model.set_g_10(args[4]);
  model.set_g_11(args[5] + id_shift);
  model.set_g_12(args[6]);
  model.set_g_13(args[7]);

  model.set_g_20(args[8]);
  model.set_g_21(args[9]);
  model.set_g_22(args[10] + id_shift);
  model.set_g_23(args[11]);
  return model;
}

template <class T>
void ToneModelAdapter<AffineToneModel>::ToPointerPad(
    const AffineToneModel& model, bool pad_square, T* args) {
  ABSL_DCHECK(args);
  args[0] = model.g_00();
  args[1] = model.g_01();
  args[2] = model.g_02();
  args[3] = model.g_03();

  args[4] = model.g_10();
  args[5] = model.g_11();
  args[6] = model.g_12();
  args[7] = model.g_13();

  args[8] = model.g_20();
  args[9] = model.g_21();
  args[10] = model.g_22();
  args[11] = model.g_23();

  if (pad_square) {
    args[12] = 0;
    args[13] = 0;
    args[14] = 0;
    args[15] = 1;
  }
}

inline Vector3_f ToneModelAdapter<AffineToneModel>::TransformColor(
    const AffineToneModel& model, const Vector3_f& color) {
  return Vector3_f(model.g_00() * color.x() + model.g_01() * color.y() +
                       model.g_02() * color.z() + model.g_03(),
                   model.g_10() * color.x() + model.g_11() * color.y() +
                       model.g_12() * color.z() + model.g_13(),
                   model.g_20() * color.x() + model.g_21() * color.y() +
                       model.g_22() * color.z() + model.g_23());
}

inline AffineToneModel ToneModelAdapter<AffineToneModel>::InvertChecked(
    const AffineToneModel& model, bool* success) {
  double data[16];
  double inv_data[16];
  ToPointerPad<double>(model, true, data);

  cv::Mat model_mat(4, 4, CV_64F, data);
  cv::Mat inv_model_mat(4, 4, CV_64F, inv_data);

  if (cv::invert(model_mat, inv_model_mat) < 1e-10) {
    ABSL_LOG(ERROR) << "AffineToneModel not invertible, det is zero.";
    *success = false;
    return AffineToneModel();
  }

  *success = true;
  return FromPointer<double>(inv_data, false);
}

inline AffineToneModel ToneModelAdapter<AffineToneModel>::Compose(
    const AffineToneModel& lhs, const AffineToneModel& rhs) {
  AffineToneModel result;
  double lhs_data[16];
  double rhs_data[16];
  double result_data[16];
  ToPointerPad<double>(lhs, true, lhs_data);
  ToPointerPad<double>(rhs, true, rhs_data);

  cv::Mat lhs_mat(4, 4, CV_64F, lhs_data);
  cv::Mat rhs_mat(4, 4, CV_64F, rhs_data);
  cv::Mat result_mat(4, 4, CV_64F, result_data);

  cv::gemm(lhs_mat, rhs_mat, 1.0, cv::Mat(), 0, result_mat);
  return FromPointer<double>(result_data, false);
}

inline float ToneModelAdapter<AffineToneModel>::GetParameter(
    const AffineToneModel& model, int idx) {
  switch (idx) {
    case 0:
      return model.g_00();
    case 1:
      return model.g_01();
    case 2:
      return model.g_02();
    case 3:
      return model.g_03();
    case 4:
      return model.g_10();
    case 5:
      return model.g_11();
    case 6:
      return model.g_12();
    case 7:
      return model.g_13();
    case 8:
      return model.g_20();
    case 9:
      return model.g_21();
    case 10:
      return model.g_22();
    case 11:
      return model.g_23();
    default:
      ABSL_LOG(FATAL) << "Unknown parameter requested.";
  }

  return 0.0f;
}

template <class Traits>
template <class T>
typename Traits::ModelType MixtureToneAdapter<Traits>::FromPointer(
    const T* args, bool identity_parametrization, int skip, int num_models) {
  MixtureModel model;
  const T* arg_ptr = args;
  for (int i = 0; i < num_models; ++i, arg_ptr += kBaseNumParameters + skip) {
    BaseModel base =
        BaseModelAdapter::FromPointer(arg_ptr, identity_parametrization);
    model.add_model()->CopyFrom(base);
  }

  return model;
}

template <class Traits>
template <class T>
void MixtureToneAdapter<Traits>::ToPointer(const MixtureModel& model, T* args) {
  const T* arg_ptr = args;
  const int num_models = model.model_size();
  for (int i = 0; i < num_models; ++i, arg_ptr += kBaseNumParameters) {
    BaseModel base = BaseModelAdapter::ToPointer(model.model(i), arg_ptr);
  }
}

template <class Traits>
inline float MixtureToneAdapter<Traits>::GetParameter(const MixtureModel& model,
                                                      int model_id,
                                                      int param_id) {
  return BaseModelAdapter::GetParameter(model.model(model_id), param_id);
}

template <class Traits>
inline typename Traits::ModelType MixtureToneAdapter<Traits>::IdentityModel(
    int num_mixtures) {
  MixtureModel model;
  for (int i = 0; i < num_mixtures; ++i) {
    model.add_model();
  }
  return model;
}

template <class Traits>
inline typename MixtureToneAdapter<Traits>::BaseModel
MixtureToneAdapter<Traits>::ToBaseModel(const MixtureModel& mixture_model,
                                        const float* weights) {
  const int num_models = mixture_model.model_size();

  float params[kBaseNumParameters];
  memset(params, 0, sizeof(params[0]) * kBaseNumParameters);

  // Weighted combination of mixture models.
  for (int m = 0; m < num_models; ++m) {
    for (int k = 0; k < kBaseNumParameters; ++k) {
      params[k] += BaseModelAdapter::GetParameter(mixture_model.model(m), k) *
                   weights[m];
    }
  }

  return BaseModelAdapter::FromPointer(params, false);
}

template <class Traits>
inline Vector3_f MixtureToneAdapter<Traits>::TransformColor(
    const MixtureModel& model, const float* weights, const Vector3_f& pt) {
  const int num_models = model.model_size();
  Vector3_f result(0, 0, 0);
  for (int i = 0; i < num_models; ++i) {
    result += BaseModelAdapter::TransformColor(model.model(i), pt) * weights[i];
  }

  return result;
}

template <class Traits>
inline Vector3_f MixtureToneAdapter<Traits>::SolveForColorChecked(
    const MixtureModel& model, const float* weights, const Vector3_f& pt,
    bool* success) {
  BaseModel base_model = ToBaseModel(model, weights);
  BaseModel inv_base_model =
      BaseModelAdapter::InvertChecked(base_model, success);
  return BaseModelAdapter::TransformColor(inv_base_model, pt);
}

template <class Model, class Adapter>
class ToneModelMethods {
 public:
  // Maps image input to output by applying model to each pixel's intensity.
  // Set log_domain to true, if model has been estimated in the log-domain.
  // Set normalized_model to true, if model is normalized (expected input
  // intensity range in [0, 1]).
  // Number of output channels (N) should be <= number of input channels (M),
  // in which case the only first N input channels are copied to the first N
  // output channels.
  static void MapImage(const Model& model, bool log_domain,
                       bool normalized_model, const cv::Mat& input,
                       cv::Mat* output);

  // Fast mapping version of above function via LUT if color model is
  // independent across specified C channels (that is model is a diagonal
  // matrix).
  // Otherwise incorrect results will be obtained.
  template <int C>
  static void MapImageIndependent(const Model& model, bool log_domain,
                                  bool normalized_model, const cv::Mat& input,
                                  cv::Mat* output);
};

typedef ToneModelMethods<GainBiasModel, GainBiasModelAdapter>
    GainBiasModelMethods;

typedef ToneModelMethods<AffineToneModel, AffineToneModelAdapter>
    AffineToneModelMethods;

template <class Model, class Adapter>
template <int C>
void ToneModelMethods<Model, Adapter>::MapImageIndependent(
    const Model& model, bool log_domain, bool normalized_model,
    const cv::Mat& input, cv::Mat* output) {
  ABSL_CHECK(output != nullptr);
  ABSL_CHECK_EQ(input.channels(), C);
  ABSL_CHECK_EQ(output->channels(), C);

  // Input LUT which will be mapped to the output LUT by the tone change model.
  // Needs 3 channels to represent input RGB colors, but since they are assumed
  // independent, we can assign the same value to each channel, which will be
  // transformed by the respective channel's transform.
  cv::Mat lut_input(1, 256, CV_8UC3);
  uint8_t* lut_ptr = lut_input.ptr<uint8_t>(0);
  for (int k = 0; k < 256; ++k, lut_ptr += 3) {
    for (int c = 0; c < 3; ++c) {
      lut_ptr[c] = k;
    }
  }

  // Output LUT. Only needs C channels (as many in the input/output).
  cv::Mat lut(1, 256, CV_8UC(C));
  MapImage(model, log_domain, normalized_model, lut_input, &lut);

  cv::LUT(input, lut, *output);
}

// TODO: Implement for mixtures.
// typedef ToneModelMethods<MixtureGainBiasModel,
//                         MixtureGainBiasModelAdapter>
//    MixtureGainBiasModelMethods;
// typedef ToneModelMethods<MixtureAffineToneModel,
//                         MixtureAffineToneModelAdapter>
//    MixtureAffineToneModelMethods;

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_TONE_MODELS_H_
