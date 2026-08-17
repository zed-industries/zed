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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_API_FACTORY_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_API_FACTORY_H_

#include <memory>

#include "absl/base/macros.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/op_macros.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/configuration_proto_inc.h"
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/proto/base_options_proto_inc.h"
#include "tensorflow_lite_support/cc/task/core/proto/external_file_proto_inc.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace core {

template <typename T>
using EnableIfBaseUntypedTaskApiSubclass = typename std::enable_if<
    std::is_base_of<BaseUntypedTaskApi, T>::value>::type*;

// Template creator for all subclasses of BaseTaskApi
class TaskAPIFactory {
 public:
  TaskAPIFactory() = delete;

  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  ABSL_DEPRECATED(
      "Use CreateFromBaseOptions and configure model input from "
      "tensorflow_lite_support/cc/task/core/proto/base_options.proto")
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromBuffer(
      const char* buffer_data, size_t buffer_size,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>(),
      int num_threads = 1,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings()) {
    auto engine = absl::make_unique<TfLiteEngine>(std::move(resolver));
    TFLITE_RETURN_IF_ERROR(engine->BuildModelFromFlatBuffer(buffer_data, buffer_size,
                                                     compute_settings));
    return CreateFromTfLiteEngine<T>(std::move(engine), num_threads,
                                     compute_settings);
  }

  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  ABSL_DEPRECATED(
      "Use CreateFromBaseOptions and configure model input from "
      "tensorflow_lite_support/cc/task/core/proto/base_options.proto")
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromFile(
      const std::string& file_name,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>(),
      int num_threads = 1,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings()) {
#ifdef _WIN32
    return CreateStatusWithPayload(
        absl::StatusCode::kFailedPrecondition,
        "File loading from memory map on Windows must use "
        "CreateFromBaseOptions",
        tflite::support::TfLiteSupportStatus::kFileReadError);
#else
    auto engine = absl::make_unique<TfLiteEngine>(std::move(resolver));
    TFLITE_RETURN_IF_ERROR(engine->BuildModelFromFile(file_name, compute_settings));
    return CreateFromTfLiteEngine<T>(std::move(engine), num_threads,
                                     compute_settings);
#endif
  }

  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  ABSL_DEPRECATED(
      "Use CreateFromBaseOptions and configure model input from "
      "tensorflow_lite_support/cc/task/core/proto/base_options.proto")
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromFileDescriptor(
      int file_descriptor,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>(),
      int num_threads = 1,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings()) {
#ifdef _WIN32
    return CreateStatusWithPayload(
        absl::StatusCode::kFailedPrecondition,
        "CreateFromFileDescriptor is not supported on Windows.",
        tflite::support::TfLiteSupportStatus::kFileReadError);
#else
    auto engine = absl::make_unique<TfLiteEngine>(std::move(resolver));
    TFLITE_RETURN_IF_ERROR(engine->BuildModelFromFileDescriptor(file_descriptor,
                                                         compute_settings));
    return CreateFromTfLiteEngine<T>(std::move(engine), num_threads,
                                     compute_settings);
#endif
  }

  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  ABSL_DEPRECATED(
      "Use CreateFromBaseOptions and configure model input from "
      "tensorflow_lite_support/cc/task/core/proto/base_options.proto")
  static tflite::support::
      StatusOr<std::unique_ptr<T>> CreateFromExternalFileProto(
          const ExternalFile* external_file,
          std::unique_ptr<tflite::OpResolver> resolver = absl::make_unique<
              tflite::ops::builtin::BuiltinOpResolver>(),
          int num_threads = 1,
          const tflite::proto::ComputeSettings& compute_settings =
              tflite::proto::ComputeSettings()) {
    auto engine = absl::make_unique<TfLiteEngine>(std::move(resolver));
    TFLITE_RETURN_IF_ERROR(engine->BuildModelFromExternalFileProto(external_file,
                                                            compute_settings));
    return CreateFromTfLiteEngine<T>(std::move(engine), num_threads,
                                     compute_settings);
  }

  // Creates a Task API from the provided BaseOptions. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromBaseOptions(
      const BaseOptions* base_options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>()) {
    if (!base_options->has_model_file()) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          "Missing mandatory `model_file` field in `base_options`",
          tflite::support::TfLiteSupportStatus::kInvalidArgumentError);
    }

    int num_threads = base_options->compute_settings()
                          .tflite_settings()
                          .cpu_settings()
                          .num_threads();
    if (num_threads == 0 || num_threads < -1) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          "`num_threads` must be greater than 0 or equal to -1.",
          tflite::support::TfLiteSupportStatus::kInvalidArgumentError);
    }

    auto engine = absl::make_unique<TfLiteEngine>(std::move(resolver));
    tflite::proto::ComputeSettings compute_settings(
        base_options->compute_settings());
    if (compute_settings.has_settings_to_test_locally()) {
      TFLITE_RETURN_IF_ERROR(SetMiniBenchmarkFileNameFromBaseOptions(compute_settings,
                                                              base_options));
    }
    TFLITE_RETURN_IF_ERROR(engine->BuildModelFromExternalFileProto(
        &base_options->model_file(), compute_settings));
    return CreateFromTfLiteEngine<T>(std::move(engine), compute_settings);
  }

 private:
  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromTfLiteEngine(
      std::unique_ptr<TfLiteEngine> engine, int num_threads,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings()) {
    tflite::proto::ComputeSettings settings_copy =
        tflite::proto::ComputeSettings(compute_settings);
    settings_copy.mutable_tflite_settings()
        ->mutable_cpu_settings()
        ->set_num_threads(num_threads);
    return CreateFromTfLiteEngine<T>(std::move(engine), settings_copy);
  }

  template <typename T, EnableIfBaseUntypedTaskApiSubclass<T> = nullptr>
  static tflite::support::StatusOr<std::unique_ptr<T>> CreateFromTfLiteEngine(
      std::unique_ptr<TfLiteEngine> engine,
      const tflite::proto::ComputeSettings& compute_settings =
          tflite::proto::ComputeSettings()) {
    TFLITE_RETURN_IF_ERROR(engine->InitInterpreter(compute_settings));
    return absl::make_unique<T>(std::move(engine));
  }

  static absl::Status SetMiniBenchmarkFileNameFromBaseOptions(
      tflite::proto::ComputeSettings& compute_settings,
      const BaseOptions* base_options);
};

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_API_FACTORY_H_
