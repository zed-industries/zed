/* Copyright 2022 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_CORE_BASE_OPTIONS_H_
#define MEDIAPIPE_TASKS_CC_CORE_BASE_OPTIONS_H_

#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "absl/memory/memory.h"
#include "mediapipe/tasks/cc/core/mediapipe_builtin_op_resolver.h"
#include "mediapipe/tasks/cc/core/proto/base_options.pb.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"

namespace mediapipe {
namespace tasks {
namespace core {

// Base options for MediaPipe C++ Tasks.
struct BaseOptions {
  // The model asset file contents as a string.
  std::unique_ptr<std::string> model_asset_buffer;

  // The path to the model asset to open and mmap in memory.
  std::string model_asset_path = "";

  // The delegate to run MediaPipe. If the delegate is not set, the default
  // delegate CPU is used. Use `delegate_options` to configure advanced
  // features of the selected delegate."
  enum Delegate {
    CPU = 0,
    GPU = 1,
    // Edge TPU acceleration using NNAPI delegate.
    EDGETPU_NNAPI = 2,
  };

  Delegate delegate = CPU;

  // Options for CPU.
  struct CpuOptions {};

  // Options for GPU.
  struct GpuOptions {
    // Load pre-compiled serialized binary cache to accelerate init process.
    // Only available on Android. Kernel caching will only be enabled if this
    // path is set. NOTE: binary cache usage may be skipped if valid serialized
    // model, specified by "serialized_model_dir", exists.
    std::string cached_kernel_path;

    // A dir to load from and save to a pre-compiled serialized model used to
    // accelerate init process.
    // NOTE: serialized model takes precedence over binary cache
    // specified by "cached_kernel_path", which still can be used if
    // serialized model is invalid or missing.
    std::string serialized_model_dir;

    // Unique token identifying the model. Used in conjunction with
    // "serialized_model_dir". It is the caller's responsibility to ensure
    // there is no clash of the tokens.
    std::string model_token;
  };

  // The file descriptor to a file opened with open(2), with optional additional
  // offset and length information.
  struct FileDescriptorMeta {
    // File descriptor as returned by open(2).
    int fd = -1;

    // Optional length of the mapped memory. If not specified, the actual file
    // size is used at runtime.
    int length = -1;

    // Optional starting offset in the file referred to by the file descriptor
    // `fd`.
    int offset = -1;
  } model_asset_descriptor_meta;

  // A non-default OpResolver to support custom Ops or specify a subset of
  // built-in Ops.
  std::unique_ptr<tflite::OpResolver> op_resolver;

  // Options for the chosen delegate. If not set, the default delegate options
  // is used.
  std::optional<std::variant<CpuOptions, GpuOptions>> delegate_options;

  // Disallows/disables default initialization of MediaPipe graph services. This
  // can be used to disable default OpenCL context creation so that the whole
  // pipeline can run on CPU.
  //
  // Recommendation: do not use unless you have to (for example, default
  // initialization has side effects)
  bool disable_default_service = false;
};

// Converts a BaseOptions to a BaseOptionsProto.
proto::BaseOptions ConvertBaseOptionsToProto(BaseOptions* base_options);

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_BASE_OPTIONS_H_
