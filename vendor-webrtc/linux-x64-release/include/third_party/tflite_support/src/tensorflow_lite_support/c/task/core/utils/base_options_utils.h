/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_CORE_UTILS_BASE_OPTIONS_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_CORE_UTILS_BASE_OPTIONS_UTILS_H_

#include "tensorflow_lite_support/c/task/core/base_options.h"
#include "tensorflow_lite_support/cc/task/core/proto/base_options_proto_inc.h"

// Utils for Creating TfLiteBaseOptions
namespace tflite {
namespace task {
namespace core {

/** Creates a baseline C options struct. */
TfLiteBaseOptions CreateDefaultBaseOptions();

/** Converts a C options struct to TFLiteSettings proto options to be used for
 * configuring the C++ API. */
::tflite::proto::TFLiteSettings TfLiteSettingsProtoFromCOptions(
    const TfLiteComputeSettings* c_options);

}  // namespace core
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_CORE_UTILS_BASE_OPTIONS_UTILS_H_
