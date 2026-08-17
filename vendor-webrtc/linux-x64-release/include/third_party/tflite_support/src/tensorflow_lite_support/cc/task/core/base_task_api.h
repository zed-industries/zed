/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_BASE_TASK_API_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_BASE_TASK_API_H_

#include <utility>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/port/tflite_wrapper.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace core {

class BaseUntypedTaskApi {
 public:
  explicit BaseUntypedTaskApi(std::unique_ptr<TfLiteEngine> engine)
      : engine_{std::move(engine)} {}

  virtual ~BaseUntypedTaskApi() = default;

  const metadata::ModelMetadataExtractor* GetMetadataExtractor() const {
    return engine_->metadata_extractor();
  }

 protected:
  // TODO(b/200258103): It's a short term solution. In the future we will forbid
  // Tasks exposing the underlying TfLiteEngine. Please try not rely on this
  // function.
  //
  // Returns a raw pointer to the underlying TfLiteEngine.
  TfLiteEngine* GetTfLiteEngine() { return engine_.get(); }

 private:
  std::unique_ptr<TfLiteEngine> engine_;
};

template <class OutputType, class... InputTypes>
class BaseTaskApi : public BaseUntypedTaskApi {
 public:
  explicit BaseTaskApi(std::unique_ptr<TfLiteEngine> engine)
      : BaseUntypedTaskApi(std::move(engine)) {}
  // BaseTaskApi is neither copyable nor movable.
  BaseTaskApi(const BaseTaskApi&) = delete;
  BaseTaskApi& operator=(const BaseTaskApi&) = delete;

  int32_t GetInputCount() {
    return GetTfLiteEngine()->interpreter()->inputs().size();
  }

  const TfLiteIntArray* GetInputShape(int index) {
    auto interpreter = GetTfLiteEngine()->interpreter();
    return interpreter->tensor(interpreter->inputs()[index])->dims;
  }

  int32_t GetOutputCount() {
    return GetTfLiteEngine()->interpreter()->outputs().size();
  }

  const TfLiteIntArray* GetOutputShape(int index) {
    auto interpreter = GetTfLiteEngine()->interpreter();
    return interpreter->tensor(interpreter->outputs()[index])->dims;
  }

  // Cancels the current running TFLite invocation on CPU.
  //
  // Usually called on a different thread than the one inference is running on.
  // Calling Cancel() will cause the underlying TFLite interpreter to return an
  // error, which will turn into a `CANCELLED` status and empty results. Calling
  // Cancel() at the other time will not take any effect on the current or
  // following invocation. It is perfectly fine to run inference again on the
  // same instance after a cancelled invocation. If the TFLite inference is
  // partially delegated on CPU, logs a warning message and only cancels the
  // invocation running on CPU. Other invocation which depends on the output of
  // the CPU invocation will not be executed.
  void Cancel() { GetTfLiteEngine()->Cancel(); }

 protected:
  // Subclasses need to populate input_tensors from api_inputs.
  virtual absl::Status Preprocess(
      const std::vector<TfLiteTensor*>& input_tensors,
      InputTypes... api_inputs) = 0;

  // Subclasses need to construct OutputType object from output_tensors.
  // Original inputs are also provided as they may be needed.
  virtual tflite::support::StatusOr<OutputType> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      InputTypes... api_inputs) = 0;

  // Returns (the addresses of) the model's inputs.
  std::vector<TfLiteTensor*> GetInputTensors() {
    return GetTfLiteEngine()->GetInputs();
  }

  // Returns (the addresses of) the model's outputs.
  std::vector<const TfLiteTensor*> GetOutputTensors() {
    return GetTfLiteEngine()->GetOutputs();
  }

  // Performs inference using tflite::support::TfLiteInterpreterWrapper
  // InvokeWithoutFallback().
  tflite::support::StatusOr<OutputType> Infer(InputTypes... args) {
    tflite::task::core::TfLiteEngine::InterpreterWrapper* interpreter_wrapper =
        GetTfLiteEngine()->interpreter_wrapper();
    // Note: AllocateTensors() is already performed by the interpreter wrapper
    // at InitInterpreter time (see TfLiteEngine).
    TFLITE_RETURN_IF_ERROR(Preprocess(GetInputTensors(), args...));
    absl::Status status = interpreter_wrapper->InvokeWithoutFallback();
    if (!status.ok()) {
      return status.GetPayload(tflite::support::kTfLiteSupportPayload)
                     .has_value()
                 ? status
                 : tflite::support::CreateStatusWithPayload(status.code(),
                                                            status.message());
    }
    return Postprocess(GetOutputTensors(), args...);
  }

  // Performs inference using tflite::support::TfLiteInterpreterWrapper
  // InvokeWithFallback() to benefit from automatic fallback from delegation to
  // CPU where applicable.
  tflite::support::StatusOr<OutputType> InferWithFallback(InputTypes... args) {
    tflite::task::core::TfLiteEngine::InterpreterWrapper* interpreter_wrapper =
        GetTfLiteEngine()->interpreter_wrapper();
    // Note: AllocateTensors() is already performed by the interpreter wrapper
    // at InitInterpreter time (see TfLiteEngine).
    TFLITE_RETURN_IF_ERROR(Preprocess(GetInputTensors(), args...));
    auto set_inputs_nop =
        [](tflite::task::core::TfLiteEngine::Interpreter* interpreter)
        -> absl::Status {
      // NOP since inputs are populated at Preprocess() time.
      return absl::OkStatus();
    };
    absl::Status status =
        interpreter_wrapper->InvokeWithFallback(set_inputs_nop);
    if (!status.ok()) {
      return status.GetPayload(tflite::support::kTfLiteSupportPayload)
                     .has_value()
                 ? status
                 : tflite::support::CreateStatusWithPayload(status.code(),
                                                            status.message());
    }
    return Postprocess(GetOutputTensors(), args...);
  }
};

}  // namespace core
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_BASE_TASK_API_H_
