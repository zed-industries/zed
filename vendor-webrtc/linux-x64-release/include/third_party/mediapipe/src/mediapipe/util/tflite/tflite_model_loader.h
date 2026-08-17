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

#ifndef MEDIAPIPE_UTIL_TFLITE_TFLITE_MODEL_LOADER_H_
#define MEDIAPIPE_UTIL_TFLITE_TFLITE_MODEL_LOADER_H_

#include <functional>
#include <memory>
#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/resources.h"
#include "tensorflow/lite/model_builder.h"

namespace mediapipe {

// Represents a TfLite model as a FlatBuffer.
using TfLiteModelPtr =
    std::unique_ptr<tflite::FlatBufferModel,
                    std::function<void(tflite::FlatBufferModel*)>>;

class TfLiteModelLoader {
 public:
  // Returns a Packet containing a TfLiteModelPtr, pointing to a model loaded
  // from the specified file path. If file at `path` exists and try_mmap is
  // true, tries to load the model as memory mapped file. (This can be
  // significantly faster than loading the tflite file into a buffer first.)
  // If memory mapping is not available or fails, loads the model using
  // `Resources` object. (Which can be customized per graph.)
  static absl::StatusOr<api2::Packet<TfLiteModelPtr>> LoadFromPath(
      const Resources& resources, const std::string& path,
      bool try_mmap = false);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TFLITE_TFLITE_MODEL_LOADER_H_
