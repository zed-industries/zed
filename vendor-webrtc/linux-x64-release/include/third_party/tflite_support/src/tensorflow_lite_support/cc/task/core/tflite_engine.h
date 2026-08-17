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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TFLITE_ENGINE_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TFLITE_ENGINE_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/core/api/error_reporter.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/model.h"
#include "tensorflow/lite/model_builder.h"
#include "tensorflow_lite_support/cc/port/configuration_proto_inc.h"
#include "tensorflow_lite_support/cc/port/tflite_wrapper.h"
#include "tensorflow_lite_support/cc/task/core/error_reporter.h"
#include "tensorflow_lite_support/cc/task/core/external_file_handler.h"
#include "tensorflow_lite_support/cc/task/core/proto/external_file_proto_inc.h"
#include "tensorflow_lite_support/metadata/cc/metadata_extractor.h"

#ifdef ABSL_HAVE_MMAP
#include <sys/mman.h>
#endif

#if _WIN32
typedef void* HANDLE;
#endif

namespace tflite {
namespace task {
namespace core {

// TfLiteEngine encapsulates logic for TFLite model initialization, inference
// and error reporting.
class TfLiteEngine {
 public:
  // Types.
  using InterpreterWrapper = ::tflite::support::TfLiteInterpreterWrapper;
  using Model = ::tflite::FlatBufferModel;
  using Interpreter = ::tflite::Interpreter;
  using ModelDeleter = std::default_delete<Model>;
  using InterpreterDeleter = std::default_delete<Interpreter>;

  // Constructors.
  explicit TfLiteEngine(
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());
  // Model is neither copyable nor movable.
  TfLiteEngine(const TfLiteEngine&) = delete;
  TfLiteEngine& operator=(const TfLiteEngine&) = delete;

  // Accessors.
  static int32_t InputCount(const Interpreter* interpreter) {
    return interpreter->inputs().size();
  }
  static int32_t OutputCount(const Interpreter* interpreter) {
    return interpreter->outputs().size();
  }
  static TfLiteTensor* GetInput(Interpreter* interpreter, int index) {
    return interpreter->tensor(interpreter->inputs()[index]);
  }
  // Same as above, but const.
  static const TfLiteTensor* GetInput(const Interpreter* interpreter,
                                      int index) {
    return interpreter->tensor(interpreter->inputs()[index]);
  }
  static TfLiteTensor* GetOutput(Interpreter* interpreter, int index) {
    return interpreter->tensor(interpreter->outputs()[index]);
  }
  // Same as above, but const.
  static const TfLiteTensor* GetOutput(const Interpreter* interpreter,
                                       int index) {
    return interpreter->tensor(interpreter->outputs()[index]);
  }

  std::vector<TfLiteTensor*> GetInputs();
  std::vector<const TfLiteTensor*> GetOutputs();

  const Model* model() const { return model_.get(); }
  Interpreter* interpreter() { return interpreter_.get(); }
  const Interpreter* interpreter() const { return interpreter_.get(); }
  InterpreterWrapper* interpreter_wrapper() { return &interpreter_; }
  const tflite::metadata::ModelMetadataExtractor* metadata_extractor() const {
    return model_metadata_extractor_.get();
  }

  // Builds the TF Lite FlatBufferModel (model_) from the raw FlatBuffer data
  // whose ownership remains with the caller, and which must outlive the current
  // object. This performs extra verification on the input data using
  // tflite::Verify.
  absl::Status BuildModelFromFlatBuffer(
      const char* buffer_data, size_t buffer_size,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());

  // Builds the TF Lite model from a given file.
  absl::Status BuildModelFromFile(
      const std::string& file_name,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());

#ifdef _WIN32
  // Builds the TF Lite model from a HANDLE to an open file. The caller retains
  // ownership of the handle.
  absl::Status BuildModelFromFileHandle(
      HANDLE file_handle,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());
#endif

  // Builds the TF Lite model from a given file descriptor using mmap(2).
  absl::Status BuildModelFromFileDescriptor(
      int file_descriptor,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());

  // Builds the TFLite model from the provided ExternalFile proto, which must
  // outlive the current object.
  absl::Status BuildModelFromExternalFileProto(
      const ExternalFile* external_file,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());

  // Builds the TFLite model from the provided ExternalFile proto, and take
  // ownership of ExternalFile proto.
  absl::Status BuildModelFromExternalFileProto(
      std::unique_ptr<ExternalFile> external_file);

  // Initializes interpreter with encapsulated model.
  // Note: setting num_threads to -1 has for effect to let TFLite runtime set
  // the value.
  absl::Status InitInterpreter(int num_threads = 1);

  // Initializes interpreter with acceleration configurations.
  absl::Status InitInterpreter(
      const tflite::proto::ComputeSettings& compute_settings);

  // Deprecated. Use the following method, and configure `num_threads` through
  // `compute_settings`, i.e. in `CPUSettings`:
  // absl::Status TfLiteEngine::InitInterpreter(
  //    const tflite::proto::ComputeSettings& compute_settings)
  absl::Status InitInterpreter(
      const tflite::proto::ComputeSettings& compute_settings, int num_threads);

  // Cancels the on-going `Invoke()` call if any and if possible. This method
  // can be called from a different thread than the one where `Invoke()` is
  // running.
  void Cancel() { interpreter_.Cancel(); }

 protected:
  // Custom error reporter capturing and printing to stderr low-level TF Lite
  // error messages.
  ErrorReporter error_reporter_;

 private:
  // Direct wrapper around tflite::TfLiteVerifier which checks the integrity of
  // the FlatBuffer data provided as input.
  class Verifier : public tflite::TfLiteVerifier {
   public:
    bool Verify(const char* data, int length,
                tflite::ErrorReporter* reporter) override;
  };

  // Verifies that the supplied buffer refers to a valid flatbuffer model,
  // and that it uses only operators that are supported by the OpResolver
  // that was passed to the TfLiteEngine constructor, and then builds
  // the model from the buffer and stores it in 'model_'.
  void VerifyAndBuildModelFromBuffer(const char* buffer_data,
                                     size_t buffer_size,
                                     TfLiteVerifier* extra_verifier = nullptr);

  // Gets the buffer from the file handler; verifies and builds the model
  // from the buffer; if successful, sets 'model_metadata_extractor_' to be
  // a TF Lite Metadata extractor for the model; and calculates an appropriate
  // return Status,
  // TODO(b/192726981): Remove `compute_settings` as it's not in use.
  absl::Status InitializeFromModelFileHandler(
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings());

  // ExternalFile and corresponding ExternalFileHandler for models loaded from
  // disk or file descriptor.
  // Make sure ExternalFile proto outlives the model and the interpreter.
  std::unique_ptr<ExternalFile> external_file_;
  std::unique_ptr<ExternalFileHandler> model_file_handler_;

  // TF Lite model and interpreter for actual inference.
  std::unique_ptr<Model, ModelDeleter> model_;

  // Interpreter wrapper built from the model.
  InterpreterWrapper interpreter_;

  // TFLite Metadata extractor built from the model.
  std::unique_ptr<tflite::metadata::ModelMetadataExtractor>
      model_metadata_extractor_;

  // Mechanism used by TF Lite to map Ops referenced in the FlatBuffer model to
  // actual implementation. Defaults to TF Lite BuiltinOpResolver.
  std::unique_ptr<tflite::OpResolver> resolver_;

  // Extra verifier for FlatBuffer input data.
  Verifier verifier_;
};

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TFLITE_ENGINE_H_
