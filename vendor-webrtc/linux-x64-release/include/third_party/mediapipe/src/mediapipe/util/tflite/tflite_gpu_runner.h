// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_TFLITE_TFLITE_GPU_RUNNER_H_
#define MEDIAPIPE_CALCULATORS_TFLITE_TFLITE_GPU_RUNNER_H_

#include <cstdint>
#include <memory>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/gpu/gl_base.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/delegates/gpu/api.h"
#include "tensorflow/lite/delegates/gpu/common/model.h"
#include "tensorflow/lite/delegates/gpu/common/shape.h"
#include "tensorflow/lite/delegates/gpu/gl/api2.h"
#include "tensorflow/lite/model.h"
#include "tensorflow/lite/model_builder.h"

#if defined(__ANDROID__) || defined(MEDIAPIPE_CHROMIUMOS)
#include "tensorflow/lite/delegates/gpu/cl/api.h"
#endif  // defined(__ANDROID__) || defined(MEDIAPIPE_CHROMIUMOS)

namespace tflite {
namespace gpu {

// Executes GPU based inference using the TFLite GPU delegate api2.
// Currently supports only GPU inputs/outputs.
//
// Typical order of execution:
// 1. Initialize with the flatbuffer model using InitializeWithModel().
// 2. Bind OpenGL SSBO objects as inputs and outputs using
// BindSSBOToInputTensor() and BindSSBOToOutputTensor().
// 3. Build the inference runner with Build() method.
// 4. Invoke() executes the inference, where inputs and outputs are those which
// were specified earlier. Invoke() may be called in the loop.
//
// Note: All of these need to happen inside MediaPipe's RunInGlContext to make
// sure that all steps from inference construction to execution are made using
// same OpenGL context.
class TFLiteGPURunner {
 public:
  explicit TFLiteGPURunner(const InferenceOptions& options)
      : options_(options) {}

  absl::Status InitializeWithModel(const tflite::FlatBufferModel& flatbuffer,
                                   const tflite::OpResolver& op_resolver,
                                   bool allow_quant_ops = false);

  void ForceOpenGL() { opengl_is_forced_ = true; }
  void ForceOpenCL() { opencl_is_forced_ = true; }
  void ForceOpenCLInitFromSerializedModel() {
    opencl_init_from_serialized_model_is_forced_ = true;
  }

  absl::Status BindSSBOToInputTensor(GLuint ssbo_id, int input_id);
  absl::Status BindSSBOToOutputTensor(GLuint ssbo_id, int output_id);

  int inputs_size() const { return input_shapes_.size(); }
  int outputs_size() const { return output_shapes_.size(); }

  absl::StatusOr<int64_t> GetInputElements(int id);
  absl::StatusOr<int64_t> GetOutputElements(int id);

  absl::Status Build();
  absl::Status Invoke();

  std::vector<BHWC> GetInputShapes() { return input_shapes_; }
  std::vector<BHWC> GetOutputShapes() { return output_shapes_; }

  std::vector<std::vector<int>> GetTFLiteInputShapes() {
    return input_shape_from_model_;
  }
  std::vector<std::vector<int>> GetTFLiteOutputShapes() {
    return output_shape_from_model_;
  }

  // Must be invoked after `Build` invocation.
  bool CanGenerateSerializedBinaryCache() { return is_cl_used_; }
  absl::StatusOr<std::vector<uint8_t>> GetSerializedBinaryCache();
  // Must be invoked before `Build` invocation.
  void SetSerializedBinaryCache(std::vector<uint8_t>&& cache);

  // Must be invoked after `Build` invocation.
  bool CanGenerateSerializedModel() { return is_cl_used_; }
  absl::StatusOr<std::vector<uint8_t>> GetSerializedModel();
  // Must be invoked before `Build` invocation.
  void SetSerializedModel(std::vector<uint8_t>&& serialized_model);

 private:
  absl::Status InitializeOpenGL(std::unique_ptr<InferenceBuilder>* builder);
  absl::Status InitializeOpenCL(std::unique_ptr<InferenceBuilder>* builder);

  absl::Status InitializeOpenCLFromSerializedModel(
      std::unique_ptr<InferenceBuilder>* builder);

  InferenceOptions options_;
  std::unique_ptr<gl::InferenceEnvironment> gl_environment_;

#if defined(__ANDROID__) || defined(MEDIAPIPE_CHROMIUMOS)
  std::unique_ptr<cl::InferenceEnvironment> cl_environment_;

  std::vector<uint8_t> serialized_binary_cache_;
  std::vector<uint8_t> serialized_model_;
  bool serialized_model_used_ = false;
#endif  // defined(__ANDROID__) || defined(MEDIAPIPE_CHROMIUMOS)

  // graph_gl_ is maintained temporarily and becomes invalid after runner_ is
  // ready
  std::unique_ptr<GraphFloat32> graph_gl_;
  std::unique_ptr<GraphFloat32> graph_cl_;
  std::unique_ptr<InferenceRunner> runner_;
  bool is_cl_used_ = false;

  // We keep information about input/output shapes, because they are needed
  // after graph_ becomes "converted" into runner_.
  std::vector<BHWC> input_shapes_;
  std::vector<BHWC> output_shapes_;

  // Input/output shapes above belong to the internal graph representation. It
  // is handy in certain situations to have the original tflite model's
  // input/output shapes, which differ conceptually.
  std::vector<std::vector<int>> input_shape_from_model_;
  std::vector<std::vector<int>> output_shape_from_model_;

  bool opencl_is_forced_ = false;
  bool opengl_is_forced_ = false;
  bool opencl_init_from_serialized_model_is_forced_ = false;
};

}  // namespace gpu
}  // namespace tflite

#endif  // MEDIAPIPE_CALCULATORS_TFLITE_TFLITE_GPU_RUNNER_H_
